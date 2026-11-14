//! Lighting engine: plays scenes and computes DMX frames from them.
//!
//! Merge rules:
//! * All channels: LTP – the most recently started scene wins, but only for
//!   channels it has stored (including stored 0 values). Channels not stored
//!   stay free for other scenes.
//! * The programmer (edit values) overrides the scenes.
//! * Live values override everything; they do not belong to the show and are
//!   not saved.
//! * Master/blackout scale dimmer channels; fixtures without a dimmer channel
//!   are dimmed via their color channels.

use crate::fasthash::{FastMap as HashMap, FastSet};
use std::time::Duration;

use crate::dmx::DmxFrame;
use crate::fixture::ChannelKind;
use crate::show::{ChannelRef, FixtureId, FixtureLimits, LoopMode, SceneId, Show};

mod tracking;

/// Upper limit for step changes per tick (protection against endless loops
/// in scenes with very short steps).
const MAX_STEP_ADVANCES_PER_TICK: usize = 1000;

#[derive(Clone, Debug)]
struct PlayingScene {
    scene: SceneId,
    /// Start order for LTP.
    seq: u64,
    step: usize,
    elapsed_ms: f64,
    /// Still in the first pass of step 0 (there is no predecessor).
    first_entry: bool,
    finished: bool,
    paused: bool,
    /// Time since the start (for fading in).
    age_ms: f64,
    /// Time since stopping, while the scene fades out.
    releasing: Option<f64>,
    /// Completed cycles (for “play X times”).
    cycles: u32,
    /// Effect time in ms (runs with speed, stands still when paused).
    fx_ms: f64,
}

/// Fade in/out state of a scene (level 0..=1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FadeState {
    In(f32),
    Out(f32),
}

/// Progress of a running scene for the UI.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneProgress {
    pub step: usize,
    pub step_count: usize,
    /// 0.0..=1.0 within the current step.
    pub step_fraction: f32,
    pub finished: bool,
    pub paused: bool,
}

pub struct Engine {
    show: Show,
    playing: Vec<PlayingScene>,
    next_seq: u64,
    programmer: HashMap<ChannelRef, u8>,
    /// Preview scenes (edit mode): output even when they are not running –
    /// their first step lies above the running scenes.
    preview: Vec<SceneId>,
    /// Displayed step of the preview (step scenes in edit mode).
    preview_step: usize,
    /// Live values (live mode): override everything, not saved.
    live: HashMap<ChannelRef, u8>,
    /// Blind mode: the output uses this frozen state without edit values.
    blind_show: Option<Show>,
    master: f32,
    blackout: bool,
    speed: f32,
    /// Preview position in timelines (SUPER SCENE, ms).
    timeline_cursor: f64,
    /// MOVE FX being edited: its beams stay on this point (scene, effect,
    /// pan/tilt 0..1).
    move_hold: Option<(SceneId, u32, [f32; 2])>,
    /// LIVE TRACKING per fixture: pan/tilt (16 bit) and how far it covers
    /// the other sources (fades in on start, out on release).
    tracking: HashMap<FixtureId, Track>,
}

/// Pan/tilt of a following fixture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Track {
    pub pan: u16,
    pub tilt: u16,
    pub weight: f32,
    /// Own light of the target (dimmer, color) – `None`: from the other
    /// sources.
    pub light: Option<TrackLight>,
    /// Own zoom of the target – `None`: from the other sources.
    pub zoom: Option<u8>,
    /// Time (ms) until the head has arrived; the own dimmer waits for it.
    pub wait_ms: f32,
    /// How far the own dimmer is up (0–1).
    pub light_weight: f32,
    /// On release: how far the intensity is pulled down (0–1) so the head
    /// does not move back lit.
    pub dark: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrackLight {
    pub dimmer: u8,
    pub color: [u8; 3],
    /// Color cells that get the color (bit n = cell n), the others stay dark.
    pub cells: u32,
}

/// LIVE TRACKING: fade in on start …
const TRACK_IN_MS: f64 = 300.0;
/// … and back to the scenes on release.
const TRACK_OUT_MS: f64 = 1000.0;
/// Assumed speed of a moving head (degrees per second) to estimate when it
/// has arrived – the own dimmer only comes up then.
const PAN_DEG_PER_S: f32 = 180.0;
const TILT_DEG_PER_S: f32 = 150.0;
/// Extra settling time after the estimated move (ms).
const SETTLE_MS: f32 = 250.0;
/// On release the light fades out before the head moves back …
const RELEASE_DARK_MS: f64 = 400.0;
/// … and fades back in once it has arrived.
const RELEASE_LIGHT_MS: f64 = 600.0;

impl Engine {
    pub fn new(show: Show) -> Self {
        Self {
            show,
            playing: Vec::new(),
            next_seq: 1,
            programmer: HashMap::default(),
            preview: Vec::new(),
            preview_step: 0,
            live: HashMap::default(),
            blind_show: None,
            master: 1.0,
            blackout: false,
            speed: 1.0,
            timeline_cursor: 0.0,
            move_hold: None,
            tracking: HashMap::default(),
        }
    }

    // ---------- Show ----------

    pub fn show(&self) -> &Show {
        &self.show
    }

    /// For editing the show. Call [`Engine::sync`] afterwards if needed.
    pub fn show_mut(&mut self) -> &mut Show {
        &mut self.show
    }

    /// Replaces the whole show (e.g. after loading) and stops everything.
    pub fn replace_show(&mut self, show: Show) {
        self.show = show;
        self.playing.clear();
        self.programmer.clear();
        self.live.clear();
        self.blind_show = None;
    }

    /// Removes playback/programmer state that is no longer valid after show
    /// changes.
    pub fn sync(&mut self) {
        let show = &self.show;
        self.playing
            .retain(|p| show.scene(p.scene).is_some_and(|s| !s.steps.is_empty()));
        for p in &mut self.playing {
            let len = show.scene(p.scene).map_or(1, |s| s.steps.len());
            if p.step >= len {
                p.step = 0;
                p.elapsed_ms = 0.0;
            }
        }
        self.programmer
            .retain(|ch, _| show.channel_def(*ch).is_some());
        self.live.retain(|ch, _| show.channel_def(*ch).is_some());
    }

    // ---------- Scenes ----------

    /// Is the scene running (scenes fading out no longer count)?
    pub fn is_active(&self, id: SceneId) -> bool {
        self.playing
            .iter()
            .any(|p| p.scene == id && p.releasing.is_none())
    }

    /// Does the output change by itself (running or fading scenes, previewed
    /// effects, LIVE TRACKING)? Otherwise it stays as it is until something
    /// is changed.
    pub fn is_moving(&self) -> bool {
        let preview_fx = self
            .preview
            .iter()
            .any(|id| self.show.scene(*id).is_some_and(|s| !s.effects.is_empty()));
        !self.playing.is_empty() || preview_fx || !self.tracking.is_empty()
    }

    pub fn active_scenes(&self) -> Vec<SceneId> {
        self.playing
            .iter()
            .filter(|p| p.releasing.is_none())
            .map(|p| p.scene)
            .collect()
    }

    fn fade_out_ms(show: &Show, id: SceneId) -> u32 {
        show.scene(id).map_or(0, |s| s.settings.fade_out_ms)
    }

    /// Stop a scene – with its fade-out time, otherwise immediately.
    fn stop(show: &Show, p: &mut PlayingScene) -> bool {
        if Self::fade_out_ms(show, p.scene) == 0 {
            return false; // remove
        }
        if p.releasing.is_none() {
            p.releasing = Some(0.0);
        }
        true
    }

    /// Starts a scene (from the beginning). Other scenes of the same zone stop.
    pub fn activate(&mut self, id: SceneId) {
        self.start(id, true);
    }

    /// Starts a scene additionally without releasing others (e.g. from an audio
    /// trigger while the SUPER SCENE of the same bank is running).
    pub fn activate_layered(&mut self, id: SceneId) {
        self.start(id, false);
    }

    fn start(&mut self, id: SceneId, exclusive: bool) {
        let Some(scene) = self.show.scene(id) else {
            return;
        };
        if scene.steps.is_empty() {
            return;
        }
        let (scene_reverse, scene_len) = (scene.settings.reverse, scene.steps.len());
        let zone = scene.zone;
        let mode = scene.settings.release_mode.clone();
        let show = &self.show;
        // the same scene restarts; other scenes are released according to the
        // release mode unless they are protected from it
        self.playing.retain_mut(|p| {
            if p.scene == id {
                return false;
            }
            if p.releasing.is_some() {
                return true;
            }
            let Some(other) = show.scene(p.scene) else {
                return false;
            };
            let target = exclusive && mode.matches(zone, other.zone);
            let protected = other.settings.protect.matches(other.zone, zone);
            !target || protected || Self::stop(show, p)
        });
        self.playing.push(PlayingScene {
            scene: id,
            seq: self.next_seq,
            step: if scene_reverse { scene_len - 1 } else { 0 },
            elapsed_ms: 0.0,
            first_entry: true,
            finished: false,
            paused: false,
            age_ms: 0.0,
            releasing: None,
            cycles: 0,
            fx_ms: 0.0,
        });
        self.next_seq += 1;
    }

    /// Stops a scene (with its fade-out time).
    pub fn release(&mut self, id: SceneId) {
        let show = &self.show;
        self.playing
            .retain_mut(|p| p.scene != id || Self::stop(show, p));
    }

    /// Jumps to a position in a running scene (0 = start, 1 = end).
    pub fn seek(&mut self, id: SceneId, fraction: f32) {
        let Some(scene) = self.show.scene(id) else {
            return;
        };
        let total = scene.duration_ms();
        let Some(p) = self
            .playing
            .iter_mut()
            .find(|p| p.scene == id && p.releasing.is_none())
        else {
            return;
        };
        let mut target = (f64::from(fraction.clamp(0.0, 1.0)) * total as f64)
            .min(total as f64 - 1.0)
            .max(0.0);
        p.finished = false;
        p.first_entry = target == 0.0;
        if scene.timeline().is_none() {
            p.fx_ms = target + f64::from(p.cycles) * total as f64;
        }
        for (i, st) in scene.steps.iter().enumerate() {
            let d = st.duration_ms() as f64;
            if target < d || i + 1 == scene.steps.len() {
                p.step = i;
                p.elapsed_ms = target.min(d);
                return;
            }
            target -= d;
        }
    }

    /// Is the scene currently fading in or out?
    pub fn fade_state(&self, id: SceneId) -> Option<FadeState> {
        let p = self.playing.iter().rev().find(|p| p.scene == id)?;
        let st = &self.show.scene(id)?.settings;
        if let Some(r) = p.releasing {
            let out = f64::from(st.fade_out_ms).max(1.0);
            return Some(FadeState::Out((1.0 - r / out).clamp(0.0, 1.0) as f32));
        }
        (st.fade_in_ms > 0 && p.age_ms < f64::from(st.fade_in_ms))
            .then(|| FadeState::In((p.age_ms / f64::from(st.fade_in_ms)) as f32))
    }

    /// Progress across the whole scene (0..=1), if it is running.
    pub fn scene_position(&self, id: SceneId) -> Option<f32> {
        let p = self
            .playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())?;
        let scene = self.show.scene(id)?;
        let total = scene.duration_ms().max(1) as f64;
        let before: u64 = scene.steps[..p.step.min(scene.steps.len())]
            .iter()
            .map(|s| s.duration_ms())
            .sum();
        Some(((before as f64 + p.elapsed_ms) / total).clamp(0.0, 1.0) as f32)
    }

    pub fn toggle(&mut self, id: SceneId) {
        if self.is_active(id) {
            self.release(id);
        } else {
            self.activate(id);
        }
    }

    pub fn release_all(&mut self) {
        self.playing.clear();
    }

    pub fn progress(&self, id: SceneId) -> Option<SceneProgress> {
        let p = self
            .playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())?;
        let scene = self.show.scene(id)?;
        let step = scene
            .steps
            .get(p.step.min(scene.steps.len().saturating_sub(1)))?;
        let dur = step.duration_ms().max(1) as f64;
        Some(SceneProgress {
            step: p.step,
            step_count: scene.steps.len(),
            step_fraction: (p.elapsed_ms / dur).clamp(0.0, 1.0) as f32,
            finished: p.finished,
            paused: p.paused,
        })
    }

    /// Pauses or resumes a running scene.
    pub fn toggle_pause(&mut self, id: SceneId) {
        if let Some(p) = self.playing.iter_mut().find(|p| p.scene == id) {
            p.paused = !p.paused;
        }
    }

    /// Jumps to the next (`delta = 1`) or previous (`-1`) step.
    /// The new step fades in with its fade time.
    pub fn jump_step(&mut self, id: SceneId, delta: i32) {
        let Some(len) = self.show.scene(id).map(|s| s.steps.len()) else {
            return;
        };
        if len == 0 {
            return;
        }
        if let Some(p) = self.playing.iter_mut().find(|p| p.scene == id) {
            let n = len as i64;
            p.step = (p.step as i64 + i64::from(delta)).rem_euclid(n) as usize;
            p.elapsed_ms = 0.0;
            p.first_entry = false;
            p.finished = false;
        }
    }

    /// Active scene of a zone (at most one is active per zone).
    pub fn active_in_zone(&self, zone: crate::show::ZoneId) -> Option<SceneId> {
        self.playing
            .iter()
            .rev()
            .map(|p| p.scene)
            .find(|id| self.show.scene(*id).is_some_and(|s| s.zone == Some(zone)))
    }

    // ---------- Programmer ----------

    pub fn set_programmer(&mut self, ch: ChannelRef, value: u8) {
        self.programmer.insert(ch, value);
    }

    pub fn clear_programmer_channel(&mut self, ch: ChannelRef) {
        self.programmer.remove(&ch);
    }

    // ---------- Blind ----------

    /// Blind mode on/off: when switching on, the output is frozen at the current
    /// state, edits only take effect after switching off.
    pub fn set_blind(&mut self, on: bool) {
        if on && self.blind_show.is_none() {
            self.blind_show = Some(self.show.clone());
        } else if !on {
            self.blind_show = None;
        }
    }

    pub fn is_blind(&self) -> bool {
        self.blind_show.is_some()
    }

    // ---------- Live values ----------

    pub fn set_live(&mut self, ch: ChannelRef, value: u8) {
        self.live.insert(ch, value);
    }

    pub fn clear_live_channel(&mut self, ch: ChannelRef) {
        self.live.remove(&ch);
    }

    pub fn clear_live(&mut self) {
        self.live.clear();
    }

    pub fn live(&self) -> &HashMap<ChannelRef, u8> {
        &self.live
    }

    /// Replace all live values (e.g. undo).
    pub fn replace_live(&mut self, live: HashMap<ChannelRef, u8>) {
        self.live = live;
    }

    /// Set preview scenes (empty = no preview).
    pub fn set_preview(&mut self, scenes: Vec<SceneId>) {
        self.preview = scenes;
    }

    /// Which step of the preview scenes is shown (otherwise the first).
    pub fn set_preview_step(&mut self, step: usize) {
        self.preview_step = step;
    }

    pub fn preview(&self) -> &[SceneId] {
        &self.preview
    }

    pub fn clear_programmer(&mut self) {
        self.programmer.clear();
    }

    pub fn programmer(&self) -> &HashMap<ChannelRef, u8> {
        &self.programmer
    }

    // ---------- Master / speed ----------

    pub fn master(&self) -> f32 {
        self.master
    }

    pub fn set_master(&mut self, v: f32) {
        self.master = v.clamp(0.0, 1.0);
    }

    pub fn blackout(&self) -> bool {
        self.blackout
    }

    pub fn set_blackout(&mut self, on: bool) {
        self.blackout = on;
    }

    /// Playback speed (1.0 = normal).
    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, v: f32) {
        self.speed = v.clamp(0.05, 20.0);
    }

    // ---------- Time ----------

    /// Advance time.
    pub fn tick(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0 * f64::from(self.speed);
        let real_ms = dt.as_secs_f64() * 1000.0;
        self.update_tracking(real_ms);
        // remove scenes that have faded out
        let show = &self.show;
        self.playing.retain_mut(|p| match &mut p.releasing {
            Some(r) => {
                *r += real_ms;
                *r < f64::from(Self::fade_out_ms(show, p.scene))
            }
            None => true,
        });
        let mut ended = Vec::new();
        for p in &mut self.playing {
            let Some(scene) = self.show.scene(p.scene) else {
                continue;
            };
            p.age_ms += real_ms;
            // a finished scene holds its effects at their end
            let held = p.finished && scene.timeline().is_none();
            if !p.paused && !held {
                p.fx_ms += dt_ms * f64::from(scene.settings.speed_pct) / 100.0;
            }
            let len = scene.steps.len();
            if len == 0 {
                continue;
            }
            // Steps can be deleted during playback
            if p.step >= len {
                p.step = len - 1;
                p.elapsed_ms = 0.0;
            }
            if p.finished || p.paused {
                continue;
            }
            p.elapsed_ms += dt_ms * f64::from(scene.settings.speed_pct) / 100.0;
            let mut guard = 0;
            loop {
                let dur = scene.steps[p.step].duration_ms().max(1) as f64;
                if p.elapsed_ms < dur || guard >= MAX_STEP_ADVANCES_PER_TICK {
                    break;
                }
                guard += 1;
                let reverse = scene.settings.reverse;
                let has_next = if reverse {
                    p.step > 0
                } else {
                    p.step + 1 < len
                };
                if has_next {
                    p.elapsed_ms -= dur;
                    if reverse {
                        p.step -= 1;
                    } else {
                        p.step += 1;
                    }
                    p.first_entry = false;
                } else if scene.loop_mode == LoopMode::Loop
                    || p.cycles + 1 < u32::from(scene.settings.repeat_count.max(1))
                {
                    if scene.loop_mode != LoopMode::Loop {
                        p.cycles += 1;
                    }
                    p.elapsed_ms -= dur;
                    p.step = if reverse { len - 1 } else { 0 };
                    p.first_entry = false;
                } else {
                    p.elapsed_ms = dur;
                    p.finished = true;
                    // effects stop just before the wrap to their start, so a
                    // one-shot movement stays at its end point
                    if scene.timeline().is_none() {
                        let total = scene.duration_ms().max(100) as f64;
                        p.fx_ms = total * (f64::from(p.cycles) + 1.0) - 0.5;
                    }
                    ended.push(p.scene);
                    break;
                }
            }
        }
        // End after X cycles: release and/or jump to a scene
        for id in ended {
            let Some(st) = self.show.scene(id).map(|s| s.settings.clone()) else {
                continue;
            };
            if st.release_at_end {
                self.release(id);
            }
            self.jump_from(id, &st.jump_to_scenes);
        }
    }

    /// End a scene early (e.g. “Fade out sound”): release it and switch to the
    /// scenes from “Jump to” as at the regular end.
    pub fn end_scene(&mut self, id: SceneId) {
        let jumps = self
            .show
            .scene(id)
            .map(|s| s.settings.jump_to_scenes.clone())
            .unwrap_or_default();
        self.release(id);
        self.jump_from(id, &jumps);
    }

    fn jump_from(&mut self, id: SceneId, targets: &[SceneId]) {
        for next in targets.iter().filter(|n| **n != id) {
            self.activate(*next);
        }
    }

    // ---------- Rendering ----------

    /// Intensity channel for the scene dimmer: dimmer, for fixtures without a
    /// dimmer channel the color channels.
    /// Scale the color channels of fixtures with a virtual dimmer by its value
    /// (the ratio of the colors is kept, 255 = unchanged).
    fn apply_virtual_dimmers(show: &Show, values: &mut HashMap<ChannelRef, u8>) {
        for f in &show.fixtures {
            let Some(dim) = values.get(&ChannelRef::virtual_dimmer(f.id)).copied() else {
                continue;
            };
            if dim == 255 {
                continue;
            }
            let Some(ty) = show.type_of(f) else {
                continue;
            };
            for (i, def) in ty.channels.iter().enumerate() {
                if !def.kind.is_color_intensity() {
                    continue;
                }
                let ch = ChannelRef {
                    fixture: f.id,
                    channel: i as u16,
                };
                if let Some(v) = values.get_mut(&ch) {
                    *v = ((u32::from(*v) * u32::from(dim) + 127) / 255) as u8;
                }
            }
        }
    }

    fn is_intensity(show: &Show, ch: ChannelRef) -> bool {
        let Some(def) = show.channel_def(ch) else {
            return false;
        };
        if def.kind.is_htp() {
            return true;
        }
        let has_dimmer = show
            .fixture(ch.fixture)
            .and_then(|f| show.type_of(f))
            .is_some_and(|t| t.channels.iter().any(|c| c.kind.is_htp()));
        !has_dimmer && def.kind.is_color_intensity()
    }

    /// Crossfade of one channel: switch channels (strobe/shutter, wheels,
    /// macros) do not fade but take the new value as soon as it fades in
    /// at all, so a fading scene keeps its shutter open to the end.
    fn mix(show: &Show, ch: ChannelRef, from: u8, to: u8, k: f64) -> u8 {
        if show.channel_def(ch).is_some_and(|d| d.kind.snaps()) {
            if k > 0.0 {
                to
            } else {
                from
            }
        } else {
            lerp(from, to, k)
        }
    }

    fn is_htp(show: &Show, ch: ChannelRef) -> bool {
        show.channel_def(ch).is_some_and(|d| d.kind.is_htp())
    }

    /// Current (crossfaded) values of a running scene.
    fn scene_values(&self, show: &Show, p: &PlayingScene) -> HashMap<ChannelRef, u8> {
        self.scene_values_d(show, p, 0)
    }

    /// Values of a scene; `depth` limits nested SUPER SCENES.
    fn scene_values_d(&self, show: &Show, p: &PlayingScene, depth: u8) -> HashMap<ChannelRef, u8> {
        let mut out = HashMap::default();
        let Some(scene) = show.scene(p.scene) else {
            return out;
        };
        if let Some(tl) = scene.timeline() {
            let len = tl.length_ms().max(1) as f64;
            return self.timeline_values(show, tl, p.fx_ms.rem_euclid(len), depth);
        }
        let len = scene.steps.len();
        let Some(step) = scene.steps.get(p.step.min(len.saturating_sub(1))) else {
            return out;
        };
        let current = step.value_map();
        // The scene's fade share scales the step crossfade
        let speed = f64::from(scene.settings.fade_speed.clamp(1, 40)) / 10.0;
        let fade = f64::from(step.fade_ms) / speed;
        let t = if fade < 1.0 {
            1.0
        } else {
            (p.elapsed_ms / fade).clamp(0.0, 1.0)
        };
        let previous = if p.first_entry {
            None
        } else {
            // Predecessor in playback direction
            let prev = if scene.settings.reverse {
                (p.step + 1) % len
            } else {
                (p.step + len - 1) % len
            };
            Some(scene.steps[prev].value_map())
        };

        for (&ch, &to) in &current {
            let from = match previous.as_ref().and_then(|m| m.get(&ch)) {
                Some(&v) => v,
                // Without a predecessor: intensity fades up from 0, the rest jumps.
                None if Self::is_htp(show, ch) => 0,
                None => to,
            };
            out.insert(ch, Self::mix(show, ch, from, to, t));
        }
        // Intensities missing in the new step fade out.
        if let Some(prev) = previous {
            for (ch, from) in prev {
                if !current.contains_key(&ch) && Self::is_htp(show, ch) {
                    out.insert(ch, lerp(from, 0, t));
                }
            }
        }
        self.apply_effects(show, scene, self.fx_cycles_of(scene, p.fx_ms), &mut out);
        out
    }

    /// Values of a timeline at time `t` (ms): blocks of upper tracks override
    /// lower ones (track 1 wins); fade in/out and dimmer of the blocks affect
    /// the brightness.
    fn timeline_values(
        &self,
        show: &Show,
        tl: &crate::timeline::Timeline,
        t: f64,
        depth: u8,
    ) -> HashMap<ChannelRef, u8> {
        let mut out: HashMap<ChannelRef, u8> = HashMap::default();
        if depth > 3 {
            return out;
        }
        let now = t.max(0.0) as u64;
        // Lower tracks first, so the upper ones (track 1 = highest priority)
        // are applied last and win.
        let clips: Vec<_> = tl.active_at(now).collect();
        for (_, clip) in clips.into_iter().rev() {
            let k = f64::from(clip.fade_at(now) * clip.level_at(now));
            let mut local = (t - clip.start_ms as f64) * f64::from(clip.speed.max(0.01));
            let blend = clip.layer == crate::timeline::Layer::Blend;
            // SCENE GROUP: each scene from its offset on, the first one wins
            if let Some(members) = show.group_members(&clip.content) {
                let len = crate::scene_group::members_length(show, members).max(1) as f64;
                local = if clip.looped {
                    local.rem_euclid(len)
                } else {
                    local.min(len - 1.0)
                };
                let mut block: HashMap<ChannelRef, u8> = HashMap::default();
                let mut floor: HashMap<crate::FixtureId, f64> = HashMap::default();
                for m in members.iter().rev() {
                    let Some(scene) = show.scene(m.scene) else {
                        continue;
                    };
                    let l = local - m.offset_ms as f64;
                    let len = m.length(show) as f64;
                    if l < 0.0 || l >= len {
                        continue;
                    }
                    // longer than the scene: it repeats
                    let dur = scene.duration_ms().max(1) as f64;
                    let p = Self::virtual_play(scene, l.rem_euclid(dur));
                    let values = self.scene_values_d(show, &p, depth + 1);
                    let fade = f64::from(m.fade_at(show, l));
                    for (f, v) in Self::step_lit(show, scene, &p) {
                        let a = floor.entry(f).or_insert(0.0);
                        *a = a.max(v * fade);
                    }
                    Self::layer(show, &mut block, values, fade, false, None);
                }
                Self::layer(show, &mut out, block, k, blend, Some(&floor));
                continue;
            }
            let Some(id) = clip.scene() else {
                continue;
            };
            let Some(scene) = show.scene(id) else {
                continue;
            };
            let dur = scene.duration_ms().max(1) as f64;
            local = if clip.looped {
                local.rem_euclid(dur)
            } else {
                local.min(dur - 1.0)
            };
            let p = Self::virtual_play(scene, local);
            let values = self.scene_values_d(show, &p, depth + 1);
            // how far the block lights a fixture by its scene (step values,
            // not the effects): a pulsing dimmer effect must not pull color
            // and position back and forth to the tracks below
            let floor = Self::step_lit(show, scene, &p);
            Self::layer(show, &mut out, values, k, blend, Some(&floor));
        }
        // Pre-position (“move in black”) before upcoming blocks: only channels
        // no running block sets, only the chosen channel groups
        for clip in tl.preparing_at(now) {
            let Some(prep) = clip.prepare else {
                continue;
            };
            // the scene of the block, or the scenes of a group (first wins)
            let scenes: Vec<SceneId> = match show.group_members(&clip.content) {
                Some(m) => m.iter().map(|m| m.scene).collect(),
                None => clip.scene().into_iter().collect(),
            };
            let mut values: Vec<(ChannelRef, u8)> = Vec::new();
            for scene in scenes.iter().filter_map(|id| show.scene(*id)) {
                let p = Self::virtual_play(scene, 0.0);
                values.extend(self.scene_values_d(show, &p, depth + 1));
            }
            // fixtures lit by the running blocks (decided once, before
            // anything is pre-positioned)
            let lit = if prep.only_dark {
                let fixtures = values.iter().map(|(c, _)| c.fixture).collect();
                Self::lit_fixtures(show, &out, &fixtures)
            } else {
                Default::default()
            };
            for (ch, v) in values {
                if out.contains_key(&ch) {
                    continue;
                }
                let Some(def) = show.channel_def(ch) else {
                    continue;
                };
                if !prep.includes(def.kind) {
                    continue;
                }
                if lit.contains(&ch.fixture) {
                    continue;
                }
                out.insert(ch, v);
            }
        }
        out
    }

    /// Lay the values of a block over what the lower tracks give, with the
    /// block's fade `k` (0–1). The block's fade always crosses to what lies
    /// below, never to black.
    ///
    /// * cover: the block's channels replace the lower ones.
    /// * blend: brightness adds up (highest wins) and the other channels of
    ///   a fixture (color, position …) go over as far as the block lights the
    ///   fixture – a fading flash returns to the color below.
    fn layer(
        show: &Show,
        out: &mut HashMap<ChannelRef, u8>,
        values: HashMap<ChannelRef, u8>,
        k: f64,
        blend: bool,
        floor: Option<&HashMap<crate::FixtureId, f64>>,
    ) {
        let k = k.clamp(0.0, 1.0);
        if !blend {
            for (ch, v) in values {
                let v = if k >= 1.0 {
                    v
                } else {
                    match out.get(&ch) {
                        Some(&base) => Self::mix(show, ch, base, v, k),
                        None if Self::is_intensity(show, ch) => (f64::from(v) * k).round() as u8,
                        None => v,
                    }
                };
                out.insert(ch, v);
            }
            return;
        }
        // with a virtual dimmer it gives the brightness, the colors are color
        let virt: FastSet<crate::FixtureId> = values
            .keys()
            .filter(|c| c.is_virtual())
            .map(|c| c.fixture)
            .collect();
        let intensity = |ch: ChannelRef| {
            if ch.is_virtual() {
                true
            } else if virt.contains(&ch.fixture) {
                Self::is_htp(show, ch)
            } else {
                Self::is_intensity(show, ch)
            }
        };
        // how far the block lights each fixture (fixtures it gives no
        // brightness – e.g. only a position – are taken over fully)
        let mut lit: HashMap<crate::FixtureId, f64> = HashMap::default();
        for (ch, v) in &values {
            if intensity(*ch) {
                let a = lit.entry(ch.fixture).or_insert(0.0);
                *a = a.max(f64::from(*v) / 255.0);
            }
        }
        // at least as far as the scene's own step lights it
        if let Some(floor) = floor {
            for (f, l) in floor {
                if let Some(a) = lit.get_mut(f) {
                    *a = a.max(*l);
                }
            }
        }
        // fixtures the lower tracks set (before this block adds its values –
        // otherwise the result would depend on the order of the values)
        let below: FastSet<crate::FixtureId> = out.keys().map(|c| c.fixture).collect();
        for (ch, v) in values {
            let v = if intensity(ch) {
                let own = (f64::from(v) * k).round() as u8;
                // below without a virtual dimmer value: its colors are full
                let base = out
                    .get(&ch)
                    .copied()
                    .or_else(|| (ch.is_virtual() && below.contains(&ch.fixture)).then_some(255));
                base.map_or(own, |base| base.max(own))
            } else {
                let a = lit.get(&ch.fixture).copied().unwrap_or(1.0) * k;
                out.get(&ch)
                    .map_or(v, |&base| Self::mix(show, ch, base, v, a))
            };
            out.insert(ch, v);
        }
    }

    /// How far the current step of a scene (without effects) lights each
    /// fixture: its highest intensity value (0–1).
    fn step_lit(
        show: &Show,
        scene: &crate::show::Scene,
        p: &PlayingScene,
    ) -> HashMap<crate::FixtureId, f64> {
        let mut out: HashMap<crate::FixtureId, f64> = HashMap::default();
        let Some(step) = scene.steps.get(p.step) else {
            return out;
        };
        for (ch, v) in step.value_map() {
            if ch.is_virtual() || Self::is_intensity(show, ch) {
                let a = out.entry(ch.fixture).or_insert(0.0);
                *a = a.max(f64::from(v) / 255.0);
            }
        }
        out
    }

    /// Does the fixture give light with these values (any intensity > 0)?
    /// Which of `fixtures` give light in `values` (one pass over the values).
    fn lit_fixtures(
        show: &Show,
        values: &HashMap<ChannelRef, u8>,
        fixtures: &FastSet<crate::FixtureId>,
    ) -> FastSet<crate::FixtureId> {
        let mut lit = FastSet::default();
        for (ch, v) in values {
            if *v > 0
                && fixtures.contains(&ch.fixture)
                && !lit.contains(&ch.fixture)
                && Self::is_intensity(show, *ch)
            {
                lit.insert(ch.fixture);
            }
        }
        lit
    }

    /// Playback state of a scene at position `local` (ms) of its sequence.
    fn virtual_play(scene: &crate::show::Scene, local: f64) -> PlayingScene {
        let mut rest = local.max(0.0);
        let mut step = 0;
        for (i, s) in scene.steps.iter().enumerate() {
            let d = s.duration_ms() as f64;
            if rest < d || i + 1 == scene.steps.len() {
                step = i;
                break;
            }
            rest -= d;
        }
        PlayingScene {
            scene: scene.id,
            seq: 0,
            step,
            elapsed_ms: rest,
            first_entry: step == 0,
            finished: false,
            paused: false,
            age_ms: local,
            releasing: None,
            cycles: 0,
            fx_ms: local,
        }
    }

    /// Run time (ms) of a running SUPER SCENE in its timeline.
    pub fn timeline_time(&self, id: SceneId) -> Option<f64> {
        let len = self.show.scene(id)?.timeline()?.length_ms().max(1) as f64;
        self.playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())
            .map(|p| p.fx_ms.rem_euclid(len))
    }

    /// Is the SUPER SCENE currently running (not paused)?
    pub fn timeline_running(&self, id: SceneId) -> bool {
        self.playing
            .iter()
            .any(|p| p.scene == id && p.releasing.is_none() && !p.paused && !p.finished)
    }

    /// Play/pause of a SUPER SCENE: if it is running or paused it is paused or
    /// resumed; if it has ended or is stopped it starts again – from `cursor`,
    /// after the end or with the cursor at the end from the beginning again.
    pub fn toggle_timeline(&mut self, id: SceneId, cursor: f64) {
        let paused = self.progress(id).is_some_and(|p| p.paused);
        if self.timeline_running(id) || paused {
            self.toggle_pause(id);
            return;
        }
        let len = self
            .show
            .scene(id)
            .and_then(|s| s.timeline())
            .map_or(0, |tl| tl.length_ms()) as f64;
        let finished = self.is_active(id);
        let from = if finished || cursor >= len - 50.0 {
            0.0
        } else {
            cursor
        };
        self.activate(id);
        self.seek_timeline(id, from);
    }

    /// Set the playback of a SUPER SCENE to `ms`.
    pub fn seek_timeline(&mut self, id: SceneId, ms: f64) {
        for p in self.playing.iter_mut().filter(|p| p.scene == id) {
            p.fx_ms = ms.max(0.0);
            p.elapsed_ms = ms.max(0.0);
            p.step = 0;
            p.finished = false;
        }
    }

    /// Position of the timeline shown by the preview while the scene is not
    /// running.
    pub fn set_timeline_cursor(&mut self, ms: f64) {
        self.timeline_cursor = ms;
    }

    /// Effect time in cycles (one scene duration = 1.0) including phase.
    fn fx_cycles_of(&self, scene: &crate::show::Scene, ms: f64) -> f64 {
        let dur = scene.duration_ms().max(100) as f64;
        ms / dur + f64::from(scene.settings.phase_deg) / 360.0
    }

    /// Effect time of a scene: its own while it is active (stands still when
    /// paused). Effects of a scene that is not active do not run – the preview
    /// in edit mode shows them at their start.
    pub fn fx_cycles(&self, id: SceneId) -> f64 {
        let Some(scene) = self.show.scene(id) else {
            return 0.0;
        };
        let ms = self
            .playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())
            .map_or(0.0, |p| p.fx_ms);
        self.fx_cycles_of(scene, ms)
    }

    /// Put the scene's effects over the step values.
    fn apply_effects(
        &self,
        show: &Show,
        scene: &crate::show::Scene,
        cycles: f64,
        out: &mut HashMap<ChannelRef, u8>,
    ) {
        // the editor hold is a preview: a playing scene keeps moving
        let playing = self
            .playing
            .iter()
            .any(|p| p.scene == scene.id && p.releasing.is_none());
        for fx in &scene.effects {
            let hold = self
                .move_hold
                .filter(|(s, f, _)| !playing && *s == scene.id && *f == fx.id)
                .map(|h| h.2);
            crate::fx::apply_effect_hold(show, fx, cycles, hold, out);
        }
    }

    /// Hold the beams of a MOVE FX on a point (preview while editing a
    /// control point of a scene that is not playing); `None` lets it run
    /// again.
    pub fn set_move_hold(&mut self, hold: Option<(SceneId, u32, [f32; 2])>) {
        self.move_hold = hold;
    }

    /// Computes the current DMX frame for the output. In blind mode without edit
    /// values and with the show state frozen when switching.
    pub fn render(&self, frame: &mut DmxFrame) {
        match &self.blind_show {
            Some(frozen) => self.render_show(frozen, false, frame),
            None => self.render_show(&self.show, true, frame),
        }
    }

    /// Frame with all edits (preview for stage and faders, also in blind mode).
    pub fn render_monitor(&self) -> DmxFrame {
        let mut f = DmxFrame::new(usize::from(self.show.universes));
        self.render_show(&self.show, true, &mut f);
        f
    }

    fn render_show(&self, show: &Show, edit: bool, frame: &mut DmxFrame) {
        frame.resize(usize::from(show.universes));
        frame.clear();

        // 1) Defaults
        let mut values: HashMap<ChannelRef, u8> = HashMap::default();
        for f in &show.fixtures {
            if let Some(ty) = show.type_of(f) {
                for (i, def) in ty.channels.iter().enumerate() {
                    let ch = ChannelRef {
                        fixture: f.id,
                        channel: i as u16,
                    };
                    values.insert(ch, def.default);
                }
                if ty.needs_virtual_dimmer() {
                    values.insert(ChannelRef::virtual_dimmer(f.id), 255);
                }
            }
        }

        // Channels set by a source (scene, preview, programmer, effect, live) –
        // fixture limits only apply to these, not to base values
        let mut touched: FastSet<ChannelRef> = FastSet::default();
        // 2) Scenes in start order (LTP: later ones override)
        // Order: priority, then start order (later wins)
        let mut order: Vec<&PlayingScene> = self.playing.iter().collect();
        order.sort_by_key(|p| {
            let prio = show.scene(p.scene).map_or(0, |s| s.settings.priority);
            (prio, p.seq)
        });
        // Only stored channels of a scene take effect; channels not stored stay
        // free for other scenes.
        let scenes: Vec<(&PlayingScene, HashMap<ChannelRef, u8>)> = order
            .into_iter()
            .map(|p| (p, self.scene_values(show, p)))
            .collect();
        for (i, (p, scene_values)) in scenes.iter().enumerate() {
            let Some(settings) = show.scene(p.scene).map(|s| &s.settings) else {
                continue;
            };
            // Fade in/out of the whole scene (crossfade to what lies below)
            let mut k_in = 1.0f64;
            if settings.fade_in_ms > 0 {
                k_in = (p.age_ms / f64::from(settings.fade_in_ms)).clamp(0.0, 1.0);
            }
            let mut k_out = 1.0f64;
            if let Some(r) = p.releasing {
                let out = f64::from(settings.fade_out_ms).max(1.0);
                k_out = 1.0 - (r / out).clamp(0.0, 1.0);
            }
            // Channels a later scene that stays takes over: the releasing scene
            // holds them, the new one fades from its values (like a morph –
            // equal values do not dip, different ones glide over)
            let taken: FastSet<ChannelRef> = if p.releasing.is_some() {
                scenes[i + 1..]
                    .iter()
                    .filter(|(q, _)| q.releasing.is_none())
                    .flat_map(|(_, v)| v.keys().copied())
                    .collect()
            } else {
                FastSet::default()
            };
            // Fixtures dark below: position, color, beam … stand at once,
            // only the brightness fades (no swinging in or out)
            // (decided before this scene writes, independent of the order)
            // (one pass over the scene's channels: lit fixtures among them)
            let scene_fixtures: FastSet<crate::FixtureId> =
                scene_values.keys().map(|c| c.fixture).collect();
            let lit = Self::lit_fixtures(show, &values, &scene_fixtures);
            let dark_below: FastSet<crate::FixtureId> =
                scene_fixtures.difference(&lit).copied().collect();
            let dim = f64::from(settings.dimmer) / 255.0;
            for (&ch, &v) in scene_values {
                let v = if dim < 1.0 && Self::is_intensity(show, ch) {
                    (f64::from(v) * dim).round() as u8
                } else {
                    v
                };
                let k = if taken.contains(&ch) {
                    k_in
                } else {
                    k_in * k_out
                };
                let k = if k > 0.0
                    && k < 1.0
                    && !ch.is_virtual()
                    && !Self::is_intensity(show, ch)
                    && dark_below.contains(&ch.fixture)
                {
                    1.0
                } else {
                    k
                };
                touched.insert(ch);
                if let Some(slot) = values.get_mut(&ch) {
                    *slot = if k >= 1.0 {
                        v
                    } else {
                        Self::mix(show, ch, *slot, v, k)
                    };
                }
            }
        }

        // 3) Preview of scenes not running (first step, LTP)
        let preview: &[SceneId] = if edit { &self.preview } else { &[] };
        for id in preview {
            if self.is_active(*id) {
                continue;
            }
            // SUPER SCENE: state of the timeline at the preview position
            if let Some(tl) = show.scene(*id).and_then(|s| s.timeline()) {
                for (ch, v) in self.timeline_values(show, tl, self.timeline_cursor, 0) {
                    touched.insert(ch);
                    if let Some(slot) = values.get_mut(&ch) {
                        *slot = v;
                    }
                }
                continue;
            }
            let Some(step) = show
                .scene(*id)
                .and_then(|s| s.steps.get(self.preview_step).or(s.steps.first()))
            else {
                continue;
            };
            for cv in &step.values {
                let ch = ChannelRef {
                    fixture: cv.fixture,
                    channel: cv.channel,
                };
                touched.insert(ch);
                if let Some(slot) = values.get_mut(&ch) {
                    *slot = cv.value;
                }
            }
        }

        // 4) Programmer
        for (ch, v) in self.programmer.iter().filter(|_| edit) {
            touched.insert(*ch);
            if let Some(slot) = values.get_mut(ch) {
                *slot = *v;
            }
        }
        // 4b) Effects of the edited scenes lie above their base values
        for id in preview {
            let Some(scene) = show.scene(*id).filter(|s| !s.effects.is_empty()) else {
                continue;
            };
            let mut fx = HashMap::default();
            self.apply_effects(show, scene, self.fx_cycles(*id), &mut fx);
            for (ch, v) in fx {
                touched.insert(ch);
                if let Some(slot) = values.get_mut(&ch) {
                    *slot = v;
                }
            }
        }

        // 5) Live values
        for (ch, v) in &self.live {
            touched.insert(*ch);
            if let Some(slot) = values.get_mut(ch) {
                *slot = *v;
            }
        }

        // 5a) LIVE TRACKING: pan/tilt of following fixtures
        self.apply_tracking(show, &mut values, &mut touched);

        // 5b) Virtual dimmers: turn down the color channels proportionally. The
        // stored values stay unchanged, only the output is scaled.
        Self::apply_virtual_dimmers(show, &mut values);

        // 6) Master / blackout
        let master = if self.blackout { 0.0 } else { self.master };
        if master < 1.0 {
            for f in &show.fixtures {
                let Some(ty) = show.type_of(f) else {
                    continue;
                };
                let has_dimmer = ty.channels.iter().any(|c| c.kind.is_htp());
                for (i, def) in ty.channels.iter().enumerate() {
                    let scale = if has_dimmer {
                        def.kind.is_htp()
                    } else {
                        def.kind.is_color_intensity()
                    };
                    if scale {
                        let ch = ChannelRef {
                            fixture: f.id,
                            channel: i as u16,
                        };
                        if let Some(v) = values.get_mut(&ch) {
                            *v = (f32::from(*v) * master).round() as u8;
                        }
                    }
                }
            }
        }

        // 4b) Fixture limits
        for f in &show.fixtures {
            let lim = f.limits;
            if lim.is_default() {
                continue;
            }
            let Some(ty) = show.type_of(f) else {
                continue;
            };
            let ch = |i: usize| ChannelRef {
                fixture: f.id,
                channel: i as u16,
            };
            if lim.affects_dimmer() {
                let has_dimmer = ty.channels.iter().any(|c| c.kind.is_htp());
                for (i, def) in ty.channels.iter().enumerate() {
                    let scale = if has_dimmer {
                        def.kind.is_htp()
                    } else {
                        def.kind.is_color_intensity()
                    };
                    if scale && touched.contains(&ch(i)) {
                        if let Some(v) = values.get_mut(&ch(i)) {
                            *v = lim.map_dimmer(*v);
                        }
                    }
                }
            }
            if lim.affects_position() {
                let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k);
                let (pan, pan_f, tilt, tilt_f) = (
                    idx(ChannelKind::Pan),
                    idx(ChannelKind::PanFine),
                    idx(ChannelKind::Tilt),
                    idx(ChannelKind::TiltFine),
                );
                let read = |coarse: Option<usize>, fine: Option<usize>| -> Option<u16> {
                    let c = values.get(&ch(coarse?)).copied()?;
                    let fv = fine.and_then(|i| values.get(&ch(i)).copied()).unwrap_or(0);
                    Some(u16::from(c) << 8 | u16::from(fv))
                };
                // only what a source has set (the rest position stays unchanged)
                let set = |c: Option<usize>, f: Option<usize>| {
                    c.is_some_and(|i| touched.contains(&ch(i)))
                        || f.is_some_and(|i| touched.contains(&ch(i)))
                };
                if !set(pan, pan_f) && !set(tilt, tilt_f) {
                    continue;
                }
                let (mut p16, mut t16) = (read(pan, pan_f), read(tilt, tilt_f));
                if lim.swap_pan_tilt {
                    std::mem::swap(&mut p16, &mut t16);
                }
                let mut write = |coarse: Option<usize>, fine: Option<usize>, v: u16| {
                    if let Some(slot) = coarse.and_then(|i| values.get_mut(&ch(i))) {
                        *slot = (v >> 8) as u8;
                    }
                    if let Some(slot) = fine.and_then(|i| values.get_mut(&ch(i))) {
                        *slot = (v & 0xFF) as u8;
                    }
                };
                if let Some(v) = p16 {
                    write(
                        pan,
                        pan_f,
                        FixtureLimits::map16(v, lim.pan_min, lim.pan_max, lim.invert_pan),
                    );
                }
                if let Some(v) = t16 {
                    write(
                        tilt,
                        tilt_f,
                        FixtureLimits::map16(v, lim.tilt_min, lim.tilt_max, lim.invert_tilt),
                    );
                }
            }
        }

        // 5) Write into the frame (one pass over the values)
        for (ch, v) in values.iter().filter(|(ch, _)| !ch.is_virtual()) {
            if let Some(f) = show.fixture(ch.fixture) {
                frame.set(usize::from(f.universe), f.address + ch.channel, *v);
            }
        }
    }
}

fn lerp(from: u8, to: u8, t: f64) -> u8 {
    (f64::from(from) + (f64::from(to) - f64::from(from)) * t).round() as u8
}

#[cfg(test)]
mod tests;
