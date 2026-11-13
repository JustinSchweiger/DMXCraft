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

use std::collections::HashMap;
use std::time::Duration;

use crate::dmx::DmxFrame;
use crate::fixture::ChannelKind;
use crate::show::{ChannelRef, FixtureId, FixtureLimits, LoopMode, SceneId, Show};

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

impl Engine {
    pub fn new(show: Show) -> Self {
        Self {
            show,
            playing: Vec::new(),
            next_seq: 1,
            programmer: HashMap::new(),
            preview: Vec::new(),
            preview_step: 0,
            live: HashMap::new(),
            blind_show: None,
            master: 1.0,
            blackout: false,
            speed: 1.0,
            timeline_cursor: 0.0,
            move_hold: None,
            tracking: HashMap::new(),
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
        let mut out = HashMap::new();
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
        let mut out: HashMap<ChannelRef, u8> = HashMap::new();
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
                let mut block: HashMap<ChannelRef, u8> = HashMap::new();
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
                    Self::layer(
                        show,
                        &mut block,
                        values,
                        f64::from(m.fade_at(show, l)),
                        false,
                    );
                }
                Self::layer(show, &mut out, block, k, blend);
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
            Self::layer(show, &mut out, values, k, blend);
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
                if prep.only_dark && Self::lit_in(show, &out, ch.fixture) {
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
        let virt: std::collections::HashSet<crate::FixtureId> = values
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
        let mut lit: HashMap<crate::FixtureId, f64> = HashMap::new();
        for (ch, v) in &values {
            if intensity(*ch) {
                let a = lit.entry(ch.fixture).or_insert(0.0);
                *a = a.max(f64::from(*v) / 255.0);
            }
        }
        // fixtures the lower tracks set (before this block adds its values –
        // otherwise the result would depend on the order of the values)
        let below: std::collections::HashSet<crate::FixtureId> =
            out.keys().map(|c| c.fixture).collect();
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

    /// Does the fixture give light with these values (any intensity > 0)?
    fn lit_in(show: &Show, values: &HashMap<ChannelRef, u8>, fixture: crate::FixtureId) -> bool {
        values
            .iter()
            .any(|(ch, v)| ch.fixture == fixture && *v > 0 && Self::is_intensity(show, *ch))
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
        let mut values: HashMap<ChannelRef, u8> = HashMap::new();
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
        let mut touched: std::collections::HashSet<ChannelRef> = std::collections::HashSet::new();
        // 2) Scenes in start order (LTP: later ones override)
        // Order: priority, then start order (later wins)
        let mut order: Vec<&PlayingScene> = self.playing.iter().collect();
        order.sort_by_key(|p| {
            let prio = show.scene(p.scene).map_or(0, |s| s.settings.priority);
            (prio, p.seq)
        });
        // Only stored channels of a scene take effect; channels not stored stay
        // free for other scenes.
        for p in order {
            let Some(settings) = show.scene(p.scene).map(|s| &s.settings) else {
                continue;
            };
            // Fade in/out of the whole scene (crossfade to what lies below)
            let mut k = 1.0f64;
            if settings.fade_in_ms > 0 {
                k *= (p.age_ms / f64::from(settings.fade_in_ms)).clamp(0.0, 1.0);
            }
            if let Some(r) = p.releasing {
                let out = f64::from(settings.fade_out_ms).max(1.0);
                k *= 1.0 - (r / out).clamp(0.0, 1.0);
            }
            let dim = f64::from(settings.dimmer) / 255.0;
            for (ch, v) in self.scene_values(show, p) {
                let v = if dim < 1.0 && Self::is_intensity(show, ch) {
                    (f64::from(v) * dim).round() as u8
                } else {
                    v
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
            let mut fx = HashMap::new();
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

        // 5) Write into the frame
        for f in &show.fixtures {
            for (ch, v) in values
                .iter()
                .filter(|(ch, _)| ch.fixture == f.id && !ch.is_virtual())
            {
                frame.set(usize::from(f.universe), f.address + ch.channel, *v);
            }
        }
    }

    /// Convenience function: create a new frame.
    /// LIVE TRACKING: aim the fixtures of following trackers at their
    /// points; released ones fade back to the other sources.
    fn update_tracking(&mut self, real_ms: f64) {
        type Wanted = ([f32; 3], Option<TrackLight>, Option<u8>);
        let mut wanted: HashMap<FixtureId, Wanted> = HashMap::new();
        for t in self.show.trackers.iter().filter(|t| t.active) {
            let light = t.light.then(|| TrackLight {
                dimmer: (t.dimmer.clamp(0.0, 1.0) * 255.0).round() as u8,
                color: t.color,
                cells: t.cells,
            });
            let zoom = t
                .own_zoom
                .then(|| (t.zoom.clamp(0.0, 1.0) * 255.0).round() as u8);
            for f in &t.fixtures {
                wanted.entry(*f).or_insert((t.target, light, zoom));
            }
        }
        let step_in = (real_ms / TRACK_IN_MS) as f32;
        let step_out = (real_ms / TRACK_OUT_MS) as f32;
        // where the heads stand now (only needed when one starts to follow)
        let starting = wanted.keys().any(|f| !self.tracking.contains_key(f));
        let frame = starting.then(|| self.render_frame());
        for (f, (target, light, zoom)) in &wanted {
            let prev = self.tracking.get(f).copied();
            let Some((pan, tilt)) =
                self.show
                    .aim_moving_head_near(*f, *target, prev.map(|p| (p.pan, p.tilt)))
            else {
                continue;
            };
            let weight = prev.map_or(0.0, |p| p.weight) + step_in;
            let (wait_ms, light_weight) = match prev {
                Some(p) => {
                    let wait = (p.wait_ms - real_ms as f32).max(0.0);
                    let lw = if wait > 0.0 {
                        p.light_weight
                    } else {
                        p.light_weight + step_in
                    };
                    (wait, lw)
                }
                None => (
                    frame
                        .as_ref()
                        .map_or(0.0, |fr| self.travel_ms(fr, *f, pan, tilt)),
                    0.0,
                ),
            };
            self.tracking.insert(
                *f,
                Track {
                    pan,
                    tilt,
                    weight: weight.min(1.0),
                    light: *light,
                    zoom: *zoom,
                    wait_ms,
                    light_weight: light_weight.min(1.0),
                },
            );
        }
        self.tracking.retain(|f, t| {
            if wanted.contains_key(f) {
                return true;
            }
            t.weight -= step_out;
            t.light_weight = t.light_weight.min(t.weight);
            t.weight > 0.0
        });
    }

    /// Estimated time (ms) a moving head needs from its current output
    /// position in `frame` to pan/tilt `to_pan`/`to_tilt` (programmer values).
    fn travel_ms(&self, frame: &DmxFrame, f: FixtureId, to_pan: u16, to_tilt: u16) -> f32 {
        use crate::fixture::ChannelKind as K;
        let Some(fx) = self.show.fixture(f) else {
            return 0.0;
        };
        let Some(ty) = self.show.type_of(fx) else {
            return 0.0;
        };
        let ph = ty.physical.unwrap_or_default();
        let lim = fx.limits;
        let read = |k: K, kf: K| -> Option<u16> {
            let i = ty.channels.iter().position(|c| c.kind == k)?;
            let c = frame.get(usize::from(fx.universe), fx.address + i as u16)?;
            let fine = ty
                .channels
                .iter()
                .position(|c| c.kind == kf)
                .and_then(|j| frame.get(usize::from(fx.universe), fx.address + j as u16))
                .unwrap_or(0);
            Some(u16::from(c) << 8 | u16::from(fine))
        };
        let (pan_to, tilt_to) = if lim.swap_pan_tilt {
            (to_tilt, to_pan)
        } else {
            (to_pan, to_tilt)
        };
        let deg = |now: Option<u16>, to: u16, min: u8, max: u8, inv: bool, range: f32| {
            now.map_or(range, |n| {
                let to = crate::show::FixtureLimits::map16(to, min, max, inv);
                f32::from(n.abs_diff(to)) / 65535.0 * range
            })
        };
        let pan = deg(
            read(K::Pan, K::PanFine),
            pan_to,
            lim.pan_min,
            lim.pan_max,
            lim.invert_pan,
            ph.pan_range,
        );
        let tilt = deg(
            read(K::Tilt, K::TiltFine),
            tilt_to,
            lim.tilt_min,
            lim.tilt_max,
            lim.invert_tilt,
            ph.tilt_range,
        );
        (pan / PAN_DEG_PER_S).max(tilt / TILT_DEG_PER_S) * 1000.0 + SETTLE_MS
    }

    /// Fixtures that follow a tracker now (or are fading back).
    pub fn tracking(&self) -> &HashMap<FixtureId, Track> {
        &self.tracking
    }

    /// Lay the pan/tilt of following fixtures over the other values.
    fn apply_tracking(
        &self,
        show: &Show,
        values: &mut HashMap<ChannelRef, u8>,
        touched: &mut std::collections::HashSet<ChannelRef>,
    ) {
        use crate::fixture::ChannelKind as K;
        for (f, t) in &self.tracking {
            let Some(ty) = show.fixture(*f).and_then(|x| show.type_of(x)) else {
                continue;
            };
            let find = |k: K| {
                ty.channels
                    .iter()
                    .position(|c| c.kind == k)
                    .map(|i| ChannelRef {
                        fixture: *f,
                        channel: i as u16,
                    })
            };
            // ease in and out
            let w = f64::from(t.weight.clamp(0.0, 1.0));
            let w = w * w * (3.0 - 2.0 * w);
            if let Some(z) = t.zoom {
                for (i, def) in ty.channels.iter().enumerate() {
                    if matches!(def.kind, K::Zoom | K::Focus) {
                        let ch = ChannelRef {
                            fixture: *f,
                            channel: i as u16,
                        };
                        let base = values.get(&ch).copied().unwrap_or(0);
                        values.insert(ch, lerp(base, z, w));
                        touched.insert(ch);
                    }
                }
            }
            if let Some(l) = t.light {
                // the dimmer only comes up once the head has arrived
                let lw = f64::from(t.light_weight.clamp(0.0, 1.0));
                let lw = lw * lw * (3.0 - 2.0 * lw);
                let mut put = |ch: ChannelRef, to: u8, k: f64| {
                    let base = values.get(&ch).copied().unwrap_or(0);
                    values.insert(ch, lerp(base, to, k));
                    touched.insert(ch);
                };
                let white = l.color.iter().copied().min().unwrap_or(0);
                // n-th channel of a color = color cell n
                let mut seen: HashMap<K, u32> = HashMap::new();
                for (i, def) in ty.channels.iter().enumerate() {
                    let ch = ChannelRef {
                        fixture: *f,
                        channel: i as u16,
                    };
                    let mut cell = |k: K| {
                        let n = seen.entry(k).or_insert(0);
                        let on = *n >= 32 || l.cells & (1 << *n) != 0;
                        *n += 1;
                        on
                    };
                    match def.kind {
                        K::Dimmer => put(ch, l.dimmer, lw),
                        K::Red => {
                            let v = if cell(K::Red) { l.color[0] } else { 0 };
                            put(ch, v, w);
                        }
                        K::Green => {
                            let v = if cell(K::Green) { l.color[1] } else { 0 };
                            put(ch, v, w);
                        }
                        K::Blue => {
                            let v = if cell(K::Blue) { l.color[2] } else { 0 };
                            put(ch, v, w);
                        }
                        K::White => {
                            let v = if cell(K::White) { white } else { 0 };
                            put(ch, v, w);
                        }
                        K::Amber | K::Uv => put(ch, 0, w),
                        // shutter open as soon as it follows
                        K::Strobe if w > 0.0 => put(ch, def.open_value(), 1.0),
                        _ => {}
                    }
                }
                if ty.needs_virtual_dimmer() {
                    put(ChannelRef::virtual_dimmer(*f), l.dimmer, lw);
                }
            }
            for (coarse, fine, to) in [(K::Pan, K::PanFine, t.pan), (K::Tilt, K::TiltFine, t.tilt)]
            {
                let Some(c) = find(coarse) else {
                    continue;
                };
                let fine = find(fine);
                let base = u16::from(values.get(&c).copied().unwrap_or(0)) << 8
                    | u16::from(fine.and_then(|x| values.get(&x).copied()).unwrap_or(0));
                let v = (f64::from(base) + (f64::from(to) - f64::from(base)) * w).round() as u16;
                values.insert(c, (v >> 8) as u8);
                touched.insert(c);
                if let Some(fc) = fine {
                    values.insert(fc, (v & 0xFF) as u8);
                    touched.insert(fc);
                }
            }
        }
    }

    pub fn render_frame(&self) -> DmxFrame {
        let mut f = DmxFrame::new(usize::from(self.show.universes));
        self.render(&mut f);
        f
    }

    /// Current output value of a fixture channel (for fader displays).
    pub fn output_value(&self, frame: &DmxFrame, ch: ChannelRef) -> Option<u8> {
        if ch.is_virtual() {
            return None;
        }
        let f = self.show.fixture(ch.fixture)?;
        frame.get(usize::from(f.universe), f.address + ch.channel)
    }
}

fn lerp(from: u8, to: u8, t: f64) -> u8 {
    (f64::from(from) + (f64::from(to) - f64::from(from)) * t).round() as u8
}

#[cfg(test)]
mod tests {
    #[test]
    fn layered_start_keeps_other_scenes() {
        let mut show = Show::default();
        let z = show.zones.first().map(|z| z.id);
        let a = show.add_scene("A", z, vec![crate::show::Step::new(Vec::new())]);
        let b = show.add_scene("B", z, vec![crate::show::Step::new(Vec::new())]);
        let mut e = Engine::new(show);
        e.activate(a);
        e.activate_layered(b);
        assert!(e.is_active(a) && e.is_active(b));
    }

    use super::*;
    use crate::library::builtin_fixture_types;
    use crate::show::{ChannelValue, FixtureId, Step};

    const MS: fn(u64) -> Duration = Duration::from_millis;

    fn cv(fixture: FixtureId, channel: u16, value: u8) -> ChannelValue {
        ChannelValue {
            fixture,
            channel,
            value,
        }
    }

    /// Show with a D+RGB PAR (addr. 1) and an RGB PAR without dimmer (addr. 10).
    fn setup() -> (Show, FixtureId, FixtureId) {
        let mut s = Show::new("T", 2);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        let a = s.add_fixture("A", "generic/drgb-4ch", 0, 1).unwrap();
        let b = s.add_fixture("B", "generic/rgb-3ch", 1, 10).unwrap();
        (s, a, b)
    }

    #[test]
    fn virtual_dimmer_scales_colors_and_keeps_ratio() {
        let (s, a, b) = setup();
        assert!(s.virtual_dimmer(b).is_some());
        assert!(s.virtual_dimmer(a).is_none(), "own dimmer");
        let mut e = Engine::new(s);
        let r = ChannelRef {
            fixture: b,
            channel: 0,
        };
        let g = ChannelRef {
            fixture: b,
            channel: 1,
        };
        e.set_live(r, 200);
        e.set_live(g, 100);
        let out = |e: &Engine| {
            let f = e.render_frame();
            (f.get(1, 10).unwrap(), f.get(1, 11).unwrap())
        };
        assert_eq!(out(&e), (200, 100), "without dimmer value: unchanged");
        e.set_live(ChannelRef::virtual_dimmer(b), 128);
        assert_eq!(out(&e), (100, 50));
        e.set_live(ChannelRef::virtual_dimmer(b), 0);
        assert_eq!(out(&e), (0, 0));
        // stored color values are kept
        assert_eq!(e.live().get(&r), Some(&200));
        e.set_live(ChannelRef::virtual_dimmer(b), 255);
        assert_eq!(out(&e), (200, 100));
        // the virtual channel is not output (no address overflow)
        assert_eq!(
            e.output_value(&e.render_frame(), ChannelRef::virtual_dimmer(b)),
            None
        );
    }

    #[test]
    fn super_scene_plays_clips_on_its_timeline() {
        let (mut s, a, _) = setup();
        let inner = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        tl.add_clip(0, crate::timeline::ClipContent::Scene(inner), 1000, 2000);
        tl.add_clip(1, crate::timeline::ClipContent::Scene(inner), 4000, 1000);
        sc.sync_timeline_length();
        assert_eq!(sc.duration_ms(), 5000);
        let mut e = Engine::new(s);
        e.activate(sup);
        e.tick(Duration::from_millis(500));
        assert_eq!(e.render_frame().get(0, 1), Some(0));
        e.tick(Duration::from_millis(1000));
        assert_eq!(e.render_frame().get(0, 1), Some(200));
        e.seek_timeline(sup, 3500.0);
        assert_eq!(e.render_frame().get(0, 1), Some(0));
        assert!((e.timeline_time(sup).unwrap() - 3500.0).abs() < 1.0);
    }

    #[test]
    fn upper_track_wins_in_super_scene() {
        let (mut s, a, _) = setup();
        let top = s.add_scene("Top", None, vec![Step::new(vec![cv(a, 0, 50)])]);
        let below = s.add_scene("Below", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        tl.add_clip(1, crate::timeline::ClipContent::Scene(below), 0, 2000);
        let id = tl.add_clip(0, crate::timeline::ClipContent::Scene(top), 0, 2000);
        sc.sync_timeline_length();
        let mut e = Engine::new(s.clone());
        e.activate(sup);
        e.tick(Duration::from_millis(500));
        // blending: the brighter value wins
        assert_eq!(e.render_frame().get(0, 1), Some(200));
        // covering: track 1 has priority
        let sc = s.scene_mut(sup).unwrap();
        sc.timeline_mut().unwrap().clip_mut(id).unwrap().layer = crate::timeline::Layer::Cover;
        let mut e = Engine::new(s);
        e.activate(sup);
        e.tick(Duration::from_millis(500));
        assert_eq!(e.render_frame().get(0, 1), Some(50), "track 1 has priority");
    }

    #[test]
    fn prepare_moves_heads_in_black_before_the_block() {
        let (mut s, _, _) = setup();
        let mh = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 1, 100)
            .unwrap();
        let ty = s.type_of(s.fixture(mh).unwrap()).unwrap().clone();
        let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
        let (pan, dim) = (idx(ChannelKind::Pan), idx(ChannelKind::Dimmer));
        let up = s.add_scene(
            "Up",
            None,
            vec![Step::new(vec![cv(mh, pan, 40), cv(mh, dim, 255)])],
        );
        let lit = s.add_scene("Lit", None, vec![Step::new(vec![cv(mh, dim, 100)])]);
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        let c = tl.add_clip(0, crate::timeline::ClipContent::Scene(up), 2000, 1000);
        tl.clip_mut(c).unwrap().prepare = Some(crate::timeline::Prepare::default());
        let l = tl.add_clip(1, crate::timeline::ClipContent::Scene(lit), 0, 300);
        sc.sync_timeline_length();
        let tl = s.scene(sup).unwrap().timeline().unwrap().clone();
        let e = Engine::new(s.clone());
        let at = |t: f64| e.timeline_values(&s, &tl, t, 0);
        let ch = |c: u16| ChannelRef {
            fixture: mh,
            channel: c,
        };
        // before the lead time: nothing
        assert_eq!(at(400.0).get(&ch(pan)), None);
        // during the lead time: pan already there, no light
        let v = at(1000.0);
        assert_eq!(v.get(&ch(pan)), Some(&40));
        assert_eq!(v.get(&ch(dim)), None);
        // only while dark: the lit block keeps the head where it is
        let mut tl2 = tl.clone();
        let lc = tl2.clip_mut(l).unwrap();
        lc.start_ms = 800;
        assert_eq!(e.timeline_values(&s, &tl2, 1000.0, 0).get(&ch(pan)), None);
    }

    #[test]
    fn tracking_aims_heads_at_the_point_and_releases_softly() {
        let mut s = Show::new("T", 1);
        s.fixture_types = builtin_fixture_types();
        let mut heads = Vec::new();
        for (i, x) in [-2.0f32, 2.0].into_iter().enumerate() {
            let id = s
                .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1 + 20 * i as u16)
                .unwrap();
            s.fixture_mut(id).unwrap().place3d = Some(crate::venue::Placement {
                position: [x, 4.0, 0.0],
                rotation: [0.0, -90.0, 0.0],
            });
            heads.push(id);
        }
        let tr = s.add_tracker("A", heads.clone());
        let target = [0.5, 1.0, 2.0];
        s.tracker_mut(tr).unwrap().target = target;
        let mut e = Engine::new(s);
        let pan_tilt = |e: &Engine, id: FixtureId| {
            let ty = e.show().type_of(e.show().fixture(id).unwrap()).unwrap();
            let at = |k: ChannelKind| {
                let i = ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
                let f = e.show().fixture(id).unwrap();
                e.render_frame()
                    .get(usize::from(f.universe), f.address + i)
                    .unwrap()
            };
            (at(ChannelKind::Pan), at(ChannelKind::Tilt))
        };
        let rest = pan_tilt(&e, heads[0]);
        e.show_mut().tracker_mut(tr).unwrap().active = true;
        e.tick(MS(500));
        for id in &heads {
            let want = e.show().aim_moving_head(*id, target).unwrap();
            let lim = e.show().fixture(*id).unwrap().limits;
            let p = FixtureLimits::map16(want.0, lim.pan_min, lim.pan_max, lim.invert_pan);
            let t = FixtureLimits::map16(want.1, lim.tilt_min, lim.tilt_max, lim.invert_tilt);
            assert_eq!(pan_tilt(&e, *id), ((p >> 8) as u8, (t >> 8) as u8));
        }
        // different heads, different angles for the same point
        assert_ne!(pan_tilt(&e, heads[0]), pan_tilt(&e, heads[1]));
        // the target brings its own light
        let dim = |e: &Engine| {
            let f = e.show().fixture(heads[0]).unwrap();
            let ty = e.show().type_of(f).unwrap();
            let i = ty
                .channels
                .iter()
                .position(|c| c.kind == ChannelKind::Dimmer)
                .unwrap() as u16;
            e.render_frame()
                .get(usize::from(f.universe), f.address + i)
                .unwrap()
        };
        // the own dimmer waits until the heads have arrived
        assert_eq!(dim(&e), 0);
        assert!(e.tracking()[&heads[0]].wait_ms > 0.0);
        e.tick(MS(5000));
        e.tick(MS(400));
        assert_eq!(dim(&e), 255);
        e.show_mut().tracker_mut(tr).unwrap().light = false;
        e.tick(MS(10));
        assert_eq!(dim(&e), 0);
        // released: halfway back after half the time, then back to the scenes
        e.show_mut().tracker_mut(tr).unwrap().active = false;
        e.tick(MS(500));
        assert!(!e.tracking().is_empty());
        e.tick(MS(600));
        assert!(e.tracking().is_empty());
        assert_eq!(pan_tilt(&e, heads[0]), rest);
    }

    #[test]
    fn blending_does_not_depend_on_the_order_of_values() {
        let (s, _, b) = setup();
        let vd = ChannelRef::virtual_dimmer(b);
        let red = ChannelRef {
            fixture: b,
            channel: 0,
        };
        // nothing below, the block fades: the virtual dimmer follows the fade
        // every time (each map has its own random order)
        for _ in 0..64 {
            let mut out = HashMap::new();
            let values: HashMap<ChannelRef, u8> = [(vd, 255), (red, 255)].into_iter().collect();
            Engine::layer(&s, &mut out, values, 0.5, true);
            assert_eq!(out[&vd], 128);
            assert_eq!(out[&red], 255);
        }
    }

    #[test]
    fn fading_flash_returns_to_the_color_below() {
        let (mut s, a, b) = setup();
        // a: dimmer + RGB, b: RGB without dimmer
        let blue = s.add_scene(
            "Blue",
            None,
            vec![Step::new(vec![
                cv(a, 0, 255),
                cv(a, 1, 0),
                cv(a, 2, 0),
                cv(a, 3, 255),
                cv(b, 0, 0),
                cv(b, 1, 0),
                cv(b, 2, 255),
            ])],
        );
        // flash: white, then dark (the scene fades out by itself)
        let white = |lvl: u8| {
            vec![
                cv(a, 0, lvl),
                cv(a, 1, 255),
                cv(a, 2, 255),
                cv(a, 3, 255),
                cv(b, 0, lvl),
                cv(b, 1, lvl),
                cv(b, 2, lvl),
            ]
        };
        let flash = s.add_scene(
            "Flash",
            None,
            vec![Step::new(white(255)), Step::new(white(0))],
        );
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        tl.add_clip(1, crate::timeline::ClipContent::Scene(blue), 0, 10_000);
        let top = tl.add_clip(0, crate::timeline::ClipContent::Scene(flash), 0, 10_000);
        tl.clip_mut(top).unwrap().looped = false;
        let mut tl = s.scene(sup).unwrap().timeline().unwrap().clone();
        let e = Engine::new(s.clone());
        let ch = |f: FixtureId, c: u16| ChannelRef {
            fixture: f,
            channel: c,
        };
        let at = |tl: &crate::timeline::Timeline, t: f64| e.timeline_values(&s, tl, t, 0);
        // the flash is white
        let v = at(&tl, 10.0);
        assert_eq!((v[&ch(a, 0)], v[&ch(a, 1)], v[&ch(a, 3)]), (255, 255, 255));
        assert_eq!((v[&ch(b, 0)], v[&ch(b, 2)]), (255, 255));
        // after the flash: blue again, not dark
        let v = at(&tl, 9_000.0);
        assert_eq!(
            (v[&ch(a, 0)], v[&ch(a, 1)], v[&ch(a, 2)], v[&ch(a, 3)]),
            (255, 0, 0, 255)
        );
        assert_eq!((v[&ch(b, 0)], v[&ch(b, 1)], v[&ch(b, 2)]), (0, 0, 255));
        // covering: dark covers the blue
        tl.clip_mut(top).unwrap().layer = crate::timeline::Layer::Cover;
        assert_eq!(at(&tl, 9_000.0)[&ch(a, 0)], 0);
        // the block's own fade out goes back to the blue as well
        let c = tl.clip_mut(top).unwrap();
        c.looped = true;
        c.fade_out_ms = 1000;
        c.duration_ms = 1000;
        let v = at(&tl, 990.0);
        assert!(v[&ch(a, 0)] > 240 && v[&ch(a, 1)] < 20, "{v:?}");
    }

    #[test]
    fn scene_group_block_plays_members_with_offsets() {
        let (mut s, a, _) = setup();
        let one = s.add_scene("One", None, vec![Step::new(vec![cv(a, 0, 100)])]);
        let two = s.add_scene(
            "Two",
            None,
            vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 50)])],
        );
        s.scene_mut(one).unwrap().steps[0].hold_ms = 1000;
        s.scene_mut(two).unwrap().steps[0].hold_ms = 1000;
        let g = s.add_scene_group("G");
        s.scene_group_mut(g).unwrap().members = vec![
            crate::scene_group::GroupMember::new(one, 500),
            crate::scene_group::GroupMember::new(two, 0),
        ];
        assert_eq!(s.scene_group_length(g), 1500);
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        tl.add_group_clip(0, g, 1500, 0);
        let tl = s.scene(sup).unwrap().timeline().unwrap().clone();
        let e = Engine::new(s.clone());
        let ch = |c: u16| ChannelRef {
            fixture: a,
            channel: c,
        };
        // only Two at first, then One wins on channel 0 (first member)
        let v = e.timeline_values(&s, &tl, 200.0, 0);
        assert_eq!(v.get(&ch(0)), Some(&200));
        let v = e.timeline_values(&s, &tl, 700.0, 0);
        assert_eq!(v.get(&ch(0)), Some(&100));
        assert_eq!(v.get(&ch(1)), Some(&50));
        // own length and fade-out of a member
        s.scene_group_mut(g).unwrap().members[1].duration_ms = Some(400);
        s.scene_group_mut(g).unwrap().members[1].fade_out_ms = 200;
        let e = Engine::new(s.clone());
        let v = e.timeline_values(&s, &tl, 300.0, 0);
        assert_eq!(v.get(&ch(1)), Some(&50), "channel 1 is no intensity");
        assert_eq!(e.timeline_values(&s, &tl, 450.0, 0).get(&ch(0)), None);
        // pre-position of a group block: the starting values of its scenes
        let mut s2 = s.clone();
        s2.scene_group_mut(g).unwrap().members[0].offset_ms = 0;
        let mut tl2 = tl.clone();
        let cid = tl2.tracks[0].clips[0].id;
        let c = tl2.clip_mut(cid).unwrap();
        c.start_ms = 3000;
        c.prepare = Some(crate::timeline::Prepare {
            position: false,
            beam: false,
            ..Default::default()
        });
        assert!(
            !tl2.preparing_at(2000).is_empty(),
            "group blocks prepare too"
        );
        // deleting a scene removes it from the group
        s.remove_scene(two);
        assert_eq!(s.scene_group(g).unwrap().members.len(), 1);
    }

    #[test]
    fn finished_super_scene_restarts_on_play() {
        let (mut s, a, _) = setup();
        let inner = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(sup).unwrap();
        sc.add_effect(crate::show::EffectKind::SuperScene);
        sc.loop_mode = crate::show::LoopMode::Once;
        let tl = sc.timeline_mut().unwrap();
        tl.add_clip(0, crate::timeline::ClipContent::Scene(inner), 0, 1000);
        sc.sync_timeline_length();
        let mut e = Engine::new(s);
        e.toggle_timeline(sup, 0.0);
        e.tick(Duration::from_millis(1500));
        assert!(!e.timeline_running(sup), "finished");
        e.toggle_timeline(sup, 0.0);
        assert!(e.timeline_running(sup), "restarts");
        e.tick(Duration::from_millis(300));
        assert!(e.timeline_time(sup).unwrap() < 500.0);
        e.toggle_timeline(sup, 0.0);
        assert!(!e.timeline_running(sup), "pausiert");
    }

    #[test]
    fn end_scene_follows_jump_to() {
        let (mut s, a, _) = setup();
        let next = s.add_scene("B", None, vec![Step::new(vec![cv(a, 0, 50)])]);
        let first = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        s.scene_mut(first).unwrap().settings.jump_to_scenes = vec![next];
        let mut e = Engine::new(s);
        e.activate(first);
        e.end_scene(first);
        assert!(!e.is_active(first));
        assert!(e.is_active(next));
    }

    #[test]
    fn color_fx_drives_rgb_and_moves() {
        let (mut s, a, b) = setup();
        let id = s.add_scene("FX", None, vec![Step::new(vec![cv(a, 0, 255)])]);
        let sc = s.scene_mut(id).unwrap();
        let fx = sc.add_effect(crate::show::EffectKind::ColorFx);
        sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = vec![a, b];
        let mut e = Engine::new(s);
        e.activate(id);
        let rgb = |f: &DmxFrame| (f.get(0, 2), f.get(0, 3), f.get(0, 4), f.get(1, 10));
        let first = rgb(&e.render_frame());
        e.tick(Duration::from_millis(300));
        let later = rgb(&e.render_frame());
        assert_ne!(first, later, "colors run");
        assert_eq!(
            e.render_frame().get(0, 1),
            Some(255),
            "Dimmer from the step"
        );
    }

    #[test]
    fn preview_effects_only_run_when_active() {
        let (mut s, a, b) = setup();
        let id = s.add_scene("FX", None, vec![Step::new(vec![cv(a, 0, 255)])]);
        let sc = s.scene_mut(id).unwrap();
        let fx = sc.add_effect(crate::show::EffectKind::ColorFx);
        sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = vec![a, b];
        let mut e = Engine::new(s);
        let rgb = |e: &Engine| {
            let f = e.render_monitor();
            (f.get(0, 2), f.get(0, 3), f.get(0, 4), f.get(1, 10))
        };
        // edit mode, scene not active: effect shown, but standing still
        e.set_preview(vec![id]);
        let first = rgb(&e);
        e.tick(Duration::from_millis(300));
        assert_eq!(first, rgb(&e), "inactive: effect stands still");
        // active: runs
        e.activate(id);
        e.tick(Duration::from_millis(300));
        let running = rgb(&e);
        assert_ne!(first, running, "active: effect runs");
        // paused: stands still
        e.toggle_pause(id);
        e.tick(Duration::from_millis(300));
        assert_eq!(running, rgb(&e), "paused: effect stands still");
    }

    #[test]
    fn limits_leave_rest_position_alone() {
        use crate::show::FixtureLimits;
        let mut show = Show::new("t", 1);
        show.fixture_types = crate::library::builtin_fixture_types();
        let id = show
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        show.fixture_mut(id).unwrap().limits = FixtureLimits {
            invert_pan: true,
            invert_tilt: true,
            ..FixtureLimits::default()
        };
        let mut e = Engine::new(show);
        // nothing active: base values unchanged (pan/tilt 128, not mirrored to 127)
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(128));
        // set by the programmer: mirrored
        e.set_programmer(
            ChannelRef {
                fixture: id,
                channel: 0,
            },
            0,
        );
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(255));
    }

    #[test]
    fn defaults_are_rendered() {
        let mut s = Show::new("T", 1);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        s.add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
            .unwrap();
        let e = Engine::new(s);
        let f = e.render_frame();
        assert_eq!(f.get(0, 100), Some(128)); // Pan
        assert_eq!(f.get(0, 102), Some(128)); // Tilt
        assert_eq!(f.get(0, 105), Some(0)); // Dimmer
    }

    #[test]
    fn removing_steps_while_playing_does_not_panic() {
        let (mut s, a, _) = setup();
        let id = s.add_scene(
            "S",
            None,
            vec![
                Step::new(vec![cv(a, 0, 255)]),
                Step::new(vec![cv(a, 0, 100)]),
                Step::new(vec![cv(a, 0, 50)]),
            ],
        );
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(Duration::from_millis(2_100));
        e.show_mut().scene_mut(id).unwrap().steps = vec![Step::new(Vec::new())];
        e.tick(Duration::from_millis(20));
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(0));
        assert!(e.progress(id).is_some());
    }

    #[test]
    fn scene_values_land_on_correct_addresses() {
        let (mut s, a, b) = setup();
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![cv(a, 0, 255), cv(a, 1, 200), cv(b, 2, 50)])],
        );
        let mut e = Engine::new(s);
        e.activate(id);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(255));
        assert_eq!(f.get(0, 2), Some(200));
        assert_eq!(f.get(1, 12), Some(50));
    }

    #[test]
    fn latest_scene_wins_for_stored_channels() {
        let (mut s, a, _) = setup();
        let s1 = s.add_scene(
            "1",
            None,
            vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 10)])],
        );
        let s2 = s.add_scene(
            "2",
            None,
            vec![Step::new(vec![cv(a, 0, 100), cv(a, 1, 99)])],
        );
        let mut e = Engine::new(s);
        e.activate(s1);
        e.activate(s2);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(100), "Dimmer LTP");
        assert_eq!(f.get(0, 2), Some(99), "Rot LTP");
        e.activate(s1); // restarted -> now the newest
        assert_eq!(e.render_frame().get(0, 2), Some(10));
    }

    #[test]
    fn unstored_channels_stay_free() {
        // The red scene stores only red, the blue scene only blue -> both visible.
        let (mut s, a, _) = setup();
        let red = s.add_scene("R", None, vec![Step::new(vec![cv(a, 1, 255)])]);
        let blue = s.add_scene("B", None, vec![Step::new(vec![cv(a, 3, 255)])]);
        let mut e = Engine::new(s);
        e.activate(red);
        e.activate(blue);
        let f = e.render_frame();
        assert_eq!(f.get(0, 2), Some(255));
        assert_eq!(f.get(0, 4), Some(255));
        // if the blue scene stores red = 0, it wins (started last)
        e.show_mut().scene_mut(blue).unwrap().steps[0]
            .values
            .push(cv(a, 1, 0));
        assert_eq!(e.render_frame().get(0, 2), Some(0));
    }

    #[test]
    fn zones_are_exclusive() {
        let (mut s, a, _) = setup();
        let z = s.add_zone("Color");
        let s1 = s.add_scene("Red", Some(z), vec![Step::new(vec![cv(a, 1, 255)])]);
        let s2 = s.add_scene("Blue", Some(z), vec![Step::new(vec![cv(a, 3, 255)])]);
        let free = s.add_scene("Free", None, vec![Step::new(vec![cv(a, 0, 255)])]);
        let mut e = Engine::new(s);
        e.activate(free);
        e.activate(s1);
        e.activate(s2);
        assert!(!e.is_active(s1));
        assert!(e.is_active(s2));
        assert!(e.is_active(free));
        e.toggle(s2);
        assert!(!e.is_active(s2));
    }

    #[test]
    fn fades_between_steps_and_loops() {
        let (mut s, a, _) = setup();
        let mut st1 = Step::new(vec![cv(a, 1, 0)]);
        st1.fade_ms = 0;
        st1.hold_ms = 100;
        let mut st2 = Step::new(vec![cv(a, 1, 200)]);
        st2.fade_ms = 1000;
        st2.hold_ms = 0;
        let id = s.add_scene("Fade", None, vec![st1, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.render_frame().get(0, 2), Some(0));
        e.tick(MS(100)); // Step 2 starts
        assert_eq!(e.progress(id).unwrap().step, 1);
        e.tick(MS(500)); // Half of the crossfade
        assert_eq!(e.render_frame().get(0, 2), Some(100));
        e.tick(MS(500)); // End -> loop to step 1 (fade 0)
        assert_eq!(e.progress(id).unwrap().step, 0);
        assert_eq!(e.render_frame().get(0, 2), Some(0));
    }

    #[test]
    fn first_step_fades_intensity_in() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 0, 200), cv(a, 1, 80)]);
        st.fade_ms = 1000;
        let id = s.add_scene("In", None, vec![st]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(MS(250));
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(50), "Dimmer fades in");
        assert_eq!(f.get(0, 2), Some(80), "color jumps");
    }

    #[test]
    fn once_mode_stops_on_last_step() {
        let (mut s, a, _) = setup();
        let id = s.add_scene(
            "Once",
            None,
            vec![Step::new(vec![cv(a, 1, 1)]), Step::new(vec![cv(a, 1, 2)])],
        );
        s.scene_mut(id).unwrap().loop_mode = LoopMode::Once;
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(MS(10_000));
        let p = e.progress(id).unwrap();
        assert!(p.finished);
        assert_eq!(p.step, 1);
        assert_eq!(e.render_frame().get(0, 2), Some(2));
    }

    #[test]
    fn finished_scene_holds_effects_at_their_end() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("Once", None, vec![Step::new(vec![cv(a, 1, 1)])]);
        let sc = s.scene_mut(id).unwrap();
        sc.loop_mode = LoopMode::Once;
        sc.steps[0].hold_ms = 2000;
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(MS(1000));
        assert!((e.fx_cycles(id) - 0.5).abs() < 0.01, "running");
        for _ in 0..10 {
            e.tick(MS(700));
        }
        assert!(e.progress(id).unwrap().finished);
        let c = e.fx_cycles(id);
        assert!(c < 1.0 && c > 0.999, "held just before the wrap: {c}");
    }

    #[test]
    fn move_hold_does_not_stop_a_playing_scene() {
        let (mut s, _, _) = setup();
        let mh = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
            .unwrap();
        let id = s.add_scene("Hold", None, vec![Step::new(Vec::new())]);
        let sc = s.scene_mut(id).unwrap();
        let fx = sc.add_effect(crate::show::EffectKind::MoveFx);
        sc.effects[0].fixtures = vec![mh];
        let mut e = Engine::new(s);
        e.set_move_hold(Some((id, fx, [0.1, 0.1])));
        let scene = e.show().scene(id).unwrap().clone();
        let fx_of = |e: &Engine| {
            let mut out = HashMap::new();
            e.apply_effects(e.show(), &scene, 0.25, &mut out);
            out
        };
        let held = fx_of(&e);
        e.activate(id);
        assert_ne!(fx_of(&e), held, "playing: the hold is ignored");
    }

    #[test]
    fn zero_length_steps_do_not_hang() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 1, 1)]);
        st.hold_ms = 0;
        let mut st2 = st.clone();
        st2.values[0].value = 2;
        let id = s.add_scene("Z", None, vec![st, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(Duration::from_secs(3600)); // must not hang
        assert!(e.is_active(id));
    }

    #[test]
    fn strobe_channel_does_not_fade() {
        let (mut s, _, _) = setup();
        let mh = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
            .unwrap();
        let ty = s.type_of(s.fixture(mh).unwrap()).unwrap();
        let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
        let (dim, strobe) = (idx(ChannelKind::Dimmer), idx(ChannelKind::Strobe));
        let addr = |c: u16| 100 + c;
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![cv(mh, dim, 255), cv(mh, strobe, 255)])],
        );
        s.scene_mut(id).unwrap().settings.fade_in_ms = 1000;
        s.scene_mut(id).unwrap().settings.fade_out_ms = 1000;
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(Duration::from_millis(300));
        let f = e.render_frame();
        assert_eq!(f.get(0, addr(strobe)), Some(255), "open from the start");
        assert!(f.get(0, addr(dim)).unwrap() < 255, "dimmer fades in");
        e.tick(Duration::from_millis(1000));
        e.release(id);
        e.tick(Duration::from_millis(700));
        let f = e.render_frame();
        assert_eq!(f.get(0, addr(strobe)), Some(255), "stays open");
        assert!(f.get(0, addr(dim)).unwrap() < 255, "dimmer fades out");
        e.tick(Duration::from_millis(400));
        assert_eq!(e.render_frame().get(0, addr(dim)), Some(0));
    }

    #[test]
    fn scene_dimmer_fades_and_seek() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        s.scene_mut(id).unwrap().settings.dimmer = 128;
        s.scene_mut(id).unwrap().settings.fade_in_ms = 1000;
        s.scene_mut(id).unwrap().settings.fade_out_ms = 1000;
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.render_frame().get(0, 1), Some(0), "Fade-in starts at 0");
        e.tick(Duration::from_millis(500));
        assert_eq!(
            e.render_frame().get(0, 1),
            Some(50),
            "half faded in, dimmer 50 %"
        );
        e.tick(Duration::from_millis(600));
        assert_eq!(e.render_frame().get(0, 1), Some(100));
        e.release(id);
        assert!(!e.is_active(id));
        e.tick(Duration::from_millis(500));
        assert_eq!(e.render_frame().get(0, 1), Some(50), "fades out");
        e.tick(Duration::from_millis(600));
        assert_eq!(e.render_frame().get(0, 1), Some(0));
        assert!(e.playing.is_empty());

        // Seek within a sequence
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 1, 1)]);
        st.hold_ms = 1000;
        let mut st2 = st.clone();
        st2.values[0].value = 2;
        let id = s.add_scene("Z", None, vec![st, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.seek(id, 0.75);
        assert_eq!(e.render_frame().get(0, 2), Some(2));
        assert!((e.scene_position(id).unwrap() - 0.75).abs() < 0.01);
    }

    #[test]
    fn reverse_plays_steps_backwards() {
        let (mut s, a, _) = setup();
        let steps: Vec<Step> = (1..=3)
            .map(|v| {
                let mut st = Step::new(vec![cv(a, 1, v)]);
                st.hold_ms = 100;
                st
            })
            .collect();
        let id = s.add_scene("R", None, steps);
        s.scene_mut(id).unwrap().settings.reverse = true;
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.render_frame().get(0, 2), Some(3));
        e.tick(Duration::from_millis(110));
        assert_eq!(e.render_frame().get(0, 2), Some(2));
        e.tick(Duration::from_millis(100));
        e.tick(Duration::from_millis(100));
        assert_eq!(e.render_frame().get(0, 2), Some(3), "Loop backwards");
    }

    #[test]
    fn release_modes_and_protection() {
        use crate::show::BankScope;
        let (mut s, a, _) = setup();
        let z1 = s.add_zone("1");
        let z2 = s.add_zone("2");
        let one =
            |s: &mut Show, z, v| s.add_scene("x", Some(z), vec![Step::new(vec![cv(a, 1, v)])]);
        let a1 = one(&mut s, z1, 1);
        let a2 = one(&mut s, z1, 2);
        let b1 = one(&mut s, z2, 3);
        let mut e = Engine::new(s);
        // Default: bank -> only scenes of the same bank
        e.activate(a1);
        e.activate(b1);
        e.activate(a2);
        assert!(!e.is_active(a1) && e.is_active(a2) && e.is_active(b1));
        // All
        e.show_mut().scene_mut(a1).unwrap().settings.release_mode = BankScope::All;
        e.activate(a1);
        assert!(e.is_active(a1) && !e.is_active(a2) && !e.is_active(b1));
        // Protection: b1 cannot be released from outside its bank
        e.show_mut().scene_mut(b1).unwrap().settings.protect = BankScope::OutsideBank;
        e.activate(b1);
        e.activate(a1);
        assert!(e.is_active(b1));
        // Off
        e.show_mut().scene_mut(a2).unwrap().settings.release_mode = BankScope::Off;
        e.activate(a2);
        assert!(e.is_active(a1) && e.is_active(a2));
    }

    #[test]
    fn priority_beats_start_order() {
        let (mut s, a, _) = setup();
        let hi = s.add_scene("hi", None, vec![Step::new(vec![cv(a, 1, 200)])]);
        let lo = s.add_scene("lo", None, vec![Step::new(vec![cv(a, 1, 50)])]);
        s.scene_mut(hi).unwrap().settings.priority = 5;
        let mut e = Engine::new(s);
        e.activate(hi);
        e.activate(lo);
        assert_eq!(e.render_frame().get(0, 2), Some(200));
    }

    #[test]
    fn repeat_then_release_and_jump() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 1, 9)]);
        st.hold_ms = 100;
        let first = s.add_scene("1", None, vec![st]);
        let next = s.add_scene("2", None, vec![Step::new(vec![cv(a, 1, 7)])]);
        {
            let sc = s.scene_mut(first).unwrap();
            sc.loop_mode = LoopMode::Once;
            sc.settings.repeat_count = 2;
            sc.settings.release_at_end = true;
            sc.settings.jump_to_scenes = vec![next];
        }
        let mut e = Engine::new(s);
        e.activate(first);
        e.tick(Duration::from_millis(150));
        assert!(e.is_active(first), "first of two runs");
        e.tick(Duration::from_millis(100));
        assert!(!e.is_active(first));
        assert!(e.is_active(next));
    }

    #[test]
    fn blind_keeps_output_but_monitor_shows_edits() {
        let (mut s, a, _) = setup();
        let red = s.add_scene("Red", None, vec![Step::new(vec![cv(a, 1, 255)])]);
        let mut e = Engine::new(s);
        e.activate(red);
        e.set_blind(true);
        // Edit the scene and set the programmer: the output stays
        e.show_mut().scene_mut(red).unwrap().steps[0].values[0].value = 10;
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 3,
            },
            200,
        );
        let out = e.render_frame();
        assert_eq!((out.get(0, 2), out.get(0, 4)), (Some(255), Some(0)));
        let mon = e.render_monitor();
        assert_eq!((mon.get(0, 2), mon.get(0, 4)), (Some(10), Some(200)));
        e.set_blind(false);
        assert_eq!(e.render_frame().get(0, 2), Some(10));
    }

    #[test]
    fn live_overrides_scenes_and_resets() {
        let (mut s, a, _) = setup();
        let red = s.add_scene(
            "Red",
            None,
            vec![Step::new(vec![cv(a, 1, 255), cv(a, 2, 0), cv(a, 3, 0)])],
        );
        let mut e = Engine::new(s);
        e.activate(red);
        let blue = ChannelRef {
            fixture: a,
            channel: 3,
        };
        e.set_live(blue, 255);
        let f = e.render_frame();
        assert_eq!((f.get(0, 2), f.get(0, 4)), (Some(255), Some(255)));
        e.clear_live();
        assert_eq!(e.render_frame().get(0, 4), Some(0));
    }

    #[test]
    fn preview_outputs_inactive_scene() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("P", None, vec![Step::new(vec![cv(a, 1, 77)])]);
        let mut e = Engine::new(s);
        let ch = ChannelRef {
            fixture: a,
            channel: 1,
        };
        let before = e.render_frame();
        e.set_preview(vec![id]);
        let f = e.render_frame();
        assert_eq!(e.output_value(&f, ch), Some(77));
        assert!(!e.is_active(id));
        e.set_preview(vec![]);
        assert_eq!(e.render_frame(), before);
    }

    #[test]
    fn programmer_overrides_and_master_scales() {
        let (mut s, a, b) = setup();
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 200), cv(b, 0, 200)])],
        );
        let mut e = Engine::new(s);
        e.activate(id);
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 0,
            },
            100,
        );
        e.set_master(0.5);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(50), "Programmer 100 * Master 0.5");
        assert_eq!(
            f.get(0, 2),
            Some(200),
            "Color unchanged on fixture with dimmer"
        );
        assert_eq!(
            f.get(1, 10),
            Some(100),
            "Fixture without dimmer: color scaled"
        );
        e.set_blackout(true);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(0));
        assert_eq!(f.get(1, 10), Some(0));
    }

    #[test]
    fn pause_and_step_jumps() {
        let (mut s, a, _) = setup();
        let steps = (0..3u8)
            .map(|i| {
                let mut st = Step::new(vec![cv(a, 1, i * 10)]);
                st.hold_ms = 1000;
                st
            })
            .collect();
        let z = s.add_zone("Z");
        let id = s.add_scene("S", Some(z), steps);
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.active_in_zone(z), Some(id));
        e.toggle_pause(id);
        e.tick(MS(5000));
        assert_eq!(e.progress(id).unwrap().step, 0, "pausiert");
        e.jump_step(id, 1);
        assert_eq!(e.render_frame().get(0, 2), Some(10));
        e.jump_step(id, -2);
        assert_eq!(
            e.progress(id).unwrap().step,
            2,
            "backwards with wrap-around"
        );
        e.toggle_pause(id);
        e.tick(MS(1000));
        assert_eq!(e.progress(id).unwrap().step, 0);
    }

    #[test]
    fn limits_cap_dimmer_and_position() {
        let mut s = Show::new("T", 1);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        let mh = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        // Channels: 0 pan, 1 pan fine, 2 tilt, 3 tilt fine, 5 dimmer
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![
                cv(mh, 0, 255),
                cv(mh, 1, 255),
                cv(mh, 2, 0),
                cv(mh, 5, 200),
            ])],
        );
        {
            let l = &mut s.fixture_mut(mh).unwrap().limits;
            l.dimmer_max = 128;
            l.pan_min = 64;
            l.pan_max = 191;
            l.invert_tilt = true;
        }
        let mut e = Engine::new(s);
        e.activate(id);
        let f = e.render_frame();
        assert_eq!(f.get(0, 6), Some(100), "Dimmer 200 * 128/255");
        e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 64;
        assert_eq!(
            e.render_frame().get(0, 6),
            Some((64 + 200u32 * 64 / 255) as u8),
            "range 64..128"
        );
        e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 0;
        assert_eq!(f.get(0, 1), Some(191), "pan full -> pan max");
        assert_eq!(f.get(0, 2), Some(255), "pan fine at the upper end");
        assert_eq!(f.get(0, 3), Some(255), "tilt 0 inverted -> full");
        // Swap
        e.show_mut().fixture_mut(mh).unwrap().limits = FixtureLimits {
            swap_pan_tilt: true,
            ..Default::default()
        };
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(0), "pan gets the tilt value");
        assert_eq!(f.get(0, 3), Some(255), "tilt gets the pan value");
    }

    #[test]
    fn map16_ranges() {
        assert_eq!(FixtureLimits::map16(0, 0, 255, false), 0);
        assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, false), u16::MAX);
        assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, true), 0);
        assert_eq!(FixtureLimits::map16(0, 64, 191, false) >> 8, 64);
    }

    #[test]
    fn speed_multiplies_time() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 0, 200)]);
        st.fade_ms = 1000;
        let id = s.add_scene("S", None, vec![st]);
        let mut e = Engine::new(s);
        e.set_speed(2.0);
        e.activate(id);
        e.tick(MS(250));
        assert_eq!(e.render_frame().get(0, 1), Some(100));
    }

    #[test]
    fn sync_drops_removed_scenes_and_fixtures() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 1)])]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 0,
            },
            5,
        );
        e.show_mut().remove_scene(id);
        e.show_mut().remove_fixture(a);
        e.sync();
        assert!(e.active_scenes().is_empty());
        assert!(e.programmer().is_empty());
    }
}
