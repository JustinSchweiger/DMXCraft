//! Timeline of a SUPER SCENE: tracks with blocks (scenes or audio), as in a
//! video editor. The engine plays the scene blocks; the UI plays the audio.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::audio::AudioGrid;
use crate::mapping::Control;
use crate::show::{ChannelRef, SceneId};

/// Grid of the timeline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridMode {
    #[default]
    Time,
    Beat,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    /// Running number for track and block IDs.
    pub next_id: u32,
    /// A length change repeats the scene (loop) …
    pub loop_resize: bool,
    /// … or stretches/compresses it (speed).
    pub stretch_resize: bool,
    /// Blocks may overlap on a track.
    pub allow_overlap: bool,
    pub auto_scroll: bool,
    pub snap: bool,
    pub grid: GridMode,
    /// The beat grid follows the audio blocks (tempo per piece) instead of
    /// BPM/offset.
    pub follow_audio: bool,
    pub bpm: f32,
    pub offset_ms: u32,
}

impl Default for Timeline {
    fn default() -> Self {
        let mut t = Self {
            tracks: Vec::new(),
            next_id: 1,
            loop_resize: true,
            stretch_resize: false,
            allow_overlap: false,
            auto_scroll: true,
            snap: true,
            grid: GridMode::Time,
            follow_audio: true,
            bpm: 120.0,
            offset_ms: 0,
        };
        for _ in 0..4 {
            t.add_track(None);
        }
        t
    }
}

/// What a track holds: light (scenes) or media (audio, video, OBS).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrackKind {
    #[default]
    Light,
    Media,
}

impl TrackKind {
    /// Track kind a block belongs on.
    pub fn of(content: &ClipContent) -> Self {
        match content {
            ClipContent::Scene(_) => TrackKind::Light,
            _ => TrackKind::Media,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: u32,
    /// Light track (scenes) or media track (audio, video, OBS).
    #[serde(default, skip_serializing_if = "is_light")]
    pub kind: TrackKind,
    /// Muted (eye off).
    #[serde(default)]
    pub muted: bool,
    /// Locked (lock).
    #[serde(default)]
    pub locked: bool,
    /// Sub-tracks (dimmer/volume) visible.
    #[serde(default)]
    pub expanded: bool,
    /// Own name (empty = “Track n”).
    #[serde(default)]
    pub name: String,
    /// Color of the track (header bar).
    #[serde(default)]
    pub color: Option<[u8; 3]>,
    #[serde(default)]
    pub clips: Vec<Clip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClipContent {
    Scene(SceneId),
    /// Audio file; `offset_ms` = start within the file, `length_ms` = length of
    /// the file (0 = unknown).
    Audio {
        path: String,
        #[serde(default)]
        offset_ms: u64,
        #[serde(default)]
        length_ms: u64,
        /// Corrected beat grid of the whole file (older shows; new corrections are
        /// stored per piece in `segment_grids`).
        #[serde(default)]
        grid: Option<AudioGrid>,
        /// Corrected grids per piece (start of the piece in ms, grid).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        segment_grids: Vec<(u64, AudioGrid)>,
        /// Triggers: events of a frequency band trigger targets.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        triggers: Vec<AudioTrigger>,
    },
    /// Video that OBS plays in a media source (`offset_ms`/`length_ms` as for
    /// audio).
    Video {
        path: String,
        #[serde(default)]
        offset_ms: u64,
        #[serde(default)]
        length_ms: u64,
        /// OBS scene to switch to at the start (empty = none).
        #[serde(default, skip_serializing_if = "String::is_empty")]
        obs_scene: String,
        /// Media source in OBS (empty = the one from the OBS settings).
        #[serde(default, skip_serializing_if = "String::is_empty")]
        source: String,
    },
    /// Action in OBS when the block is reached.
    Obs(crate::obs::ObsAction),
}

fn is_light(k: &TrackKind) -> bool {
    *k == TrackKind::Light
}

impl Track {
    /// May this block be put on the track?
    pub fn accepts(&self, content: &ClipContent) -> bool {
        self.kind == TrackKind::of(content)
    }
}

/// Pre-position before a scene block (“move in black”): during the lead time
/// the fixtures of the scene already get the values of its first moment for
/// the chosen channel groups – no intensity, no color mixing.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Prepare {
    pub lead_ms: u32,
    /// Only fixtures without light (in this SUPER SCENE) move.
    pub only_dark: bool,
    /// Pan/tilt.
    pub position: bool,
    /// Focus/zoom.
    pub beam: bool,
    pub gobo: bool,
    pub color_wheel: bool,
}

impl Default for Prepare {
    fn default() -> Self {
        Self {
            lead_ms: 1500,
            only_dark: true,
            position: true,
            beam: true,
            gobo: false,
            color_wheel: false,
        }
    }
}

impl Prepare {
    /// Does the channel kind belong to the chosen groups?
    pub fn includes(&self, k: crate::ChannelKind) -> bool {
        use crate::ChannelKind as K;
        match k {
            K::Pan | K::PanFine | K::Tilt | K::TiltFine => self.position,
            K::Focus | K::Zoom => self.beam,
            K::Gobo => self.gobo,
            K::ColorWheel => self.color_wheel,
            _ => false,
        }
    }
}

impl ClipContent {
    /// Start within the file (audio/video).
    pub fn offset_mut(&mut self) -> Option<&mut u64> {
        match self {
            Self::Audio { offset_ms, .. } | Self::Video { offset_ms, .. } => Some(offset_ms),
            _ => None,
        }
    }

    /// Length of the file (audio/video; 0 = unknown).
    pub fn media_len(&self) -> Option<(u64, u64)> {
        match self {
            Self::Audio {
                offset_ms,
                length_ms,
                ..
            }
            | Self::Video {
                offset_ms,
                length_ms,
                ..
            } => Some((*offset_ms, *length_ms)),
            _ => None,
        }
    }
}

/// Trigger of an audio block: a filtered band of the file; when its level
/// exceeds the threshold the target is triggered (like a key).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioTrigger {
    pub id: u32,
    pub name: String,
    /// Pass band in Hz (`low_hz` ≤ 20: low pass, `high_hz` ≥ 16 000: high
    /// pass).
    pub low_hz: f32,
    pub high_hz: f32,
    /// Threshold and hysteresis (0..=1 of the band level).
    pub threshold: f32,
    pub hysteresis: f32,
    /// Minimum gap between two events.
    pub min_gap_ms: u32,
    /// Minimum duration of an event.
    pub hold_ms: u32,
    /// Lead time: trigger the event this much earlier (inertia of the fixtures).
    pub lead_ms: u32,
    /// Target (as for keyboard/MIDI) and fixed channels if any.
    pub control: Option<Control>,
    pub channels: Vec<ChannelRef>,
    /// Target only during the event (otherwise toggle).
    pub flash: bool,
    pub muted: bool,
}

impl Default for AudioTrigger {
    fn default() -> Self {
        Self::preset(0, TriggerBand::Low)
    }
}

/// Preset bands for new triggers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TriggerBand {
    Low,
    Mid,
    High,
}

impl TriggerBand {
    pub const ALL: [TriggerBand; 3] = [TriggerBand::Low, TriggerBand::Mid, TriggerBand::High];

    pub fn range(self) -> (f32, f32) {
        match self {
            TriggerBand::Low => (0.0, 150.0),
            TriggerBand::Mid => (150.0, 2500.0),
            TriggerBand::High => (2500.0, 20_000.0),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            TriggerBand::Low => "Low",
            TriggerBand::Mid => "Mid",
            TriggerBand::High => "High",
        }
    }
}

impl AudioTrigger {
    pub fn preset(id: u32, band: TriggerBand) -> Self {
        let (low_hz, high_hz) = band.range();
        Self {
            id,
            name: match band {
                TriggerBand::Low => "Kick",
                TriggerBand::Mid => "Snare",
                TriggerBand::High => "Hi-Hat",
            }
            .into(),
            low_hz,
            high_hz,
            threshold: 0.6,
            hysteresis: 0.15,
            min_gap_ms: if band == TriggerBand::High { 60 } else { 120 },
            hold_ms: 80,
            lead_ms: 0,
            control: None,
            channels: Vec::new(),
            flash: true,
            muted: false,
        }
    }

    /// Preset band, if the range matches one.
    pub fn band(&self) -> Option<TriggerBand> {
        TriggerBand::ALL
            .into_iter()
            .find(|b| b.range() == (self.low_hz, self.high_hz))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: u32,
    pub start_ms: u64,
    pub duration_ms: u64,
    #[serde(default)]
    pub fade_in_ms: u32,
    #[serde(default)]
    pub fade_out_ms: u32,
    /// The scene repeats within the block (otherwise the last state stays).
    #[serde(default = "yes")]
    pub looped: bool,
    /// Fit the length to the tempo (whole beats).
    #[serde(default)]
    pub tempo_sync: bool,
    /// Playback speed of the scene (stretching).
    #[serde(default = "one")]
    pub speed: f32,
    /// Dimmer (scene) or volume (audio), 0..=1.
    #[serde(default = "one")]
    pub level: f32,
    pub content: ClipContent,
    /// Pre-position before the block (scene blocks).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepare: Option<Prepare>,
}

fn yes() -> bool {
    true
}

fn one() -> f32 {
    1.0
}

impl Clip {
    pub fn end_ms(&self) -> u64 {
        self.start_ms + self.duration_ms
    }

    pub fn scene(&self) -> Option<SceneId> {
        match self.content {
            ClipContent::Scene(id) => Some(id),
            _ => None,
        }
    }

    /// Level from fade in/out at time `t` (0..=1, without `level`).
    pub fn fade_at(&self, t: u64) -> f32 {
        let since = t.saturating_sub(self.start_ms) as f32;
        let until = self.end_ms().saturating_sub(t) as f32;
        let a = if self.fade_in_ms > 0 {
            (since / self.fade_in_ms as f32).min(1.0)
        } else {
            1.0
        };
        let b = if self.fade_out_ms > 0 {
            (until / self.fade_out_ms as f32).min(1.0)
        } else {
            1.0
        };
        a.min(b)
    }
}

impl Timeline {
    fn next(&mut self) -> u32 {
        let id = self.next_id.max(1);
        self.next_id = id + 1;
        id
    }

    /// Insert a track (before `at`, otherwise at the end); returns its ID.
    pub fn add_track(&mut self, at: Option<usize>) -> u32 {
        let id = self.next();
        let t = Track {
            id,
            kind: TrackKind::Light,
            muted: false,
            locked: false,
            expanded: false,
            name: String::new(),
            color: None,
            clips: Vec::new(),
        };
        match at {
            Some(i) if i <= self.tracks.len() => self.tracks.insert(i, t),
            _ => self.tracks.push(t),
        }
        id
    }

    /// Insert a track of a kind (before `at`, otherwise at the end).
    pub fn add_track_of(&mut self, at: Option<usize>, kind: TrackKind) -> u32 {
        let id = self.add_track(at);
        if let Some(t) = self.tracks.iter_mut().find(|t| t.id == id) {
            t.kind = kind;
        }
        id
    }

    /// Older shows: tracks with only audio/video/OBS blocks become media
    /// tracks.
    pub fn normalize(&mut self) {
        for t in &mut self.tracks {
            if t.kind == TrackKind::Light
                && !t.clips.is_empty()
                && t.clips
                    .iter()
                    .all(|c| TrackKind::of(&c.content) == TrackKind::Media)
            {
                t.kind = TrackKind::Media;
            }
        }
    }

    /// Scene blocks whose pre-position runs at time `t` (lead time before the
    /// block), the next one first.
    pub fn preparing_at(&self, t: u64) -> Vec<&Clip> {
        let mut v: Vec<&Clip> = self
            .tracks
            .iter()
            .filter(|tr| !tr.muted)
            .flat_map(|tr| tr.clips.iter())
            .filter(|c| {
                c.scene().is_some()
                    && c.prepare
                        .is_some_and(|p| t < c.start_ms && c.start_ms - t <= u64::from(p.lead_ms))
            })
            .collect();
        v.sort_by_key(|c| c.start_ms);
        v
    }

    /// End of the last block.
    pub fn length_ms(&self) -> u64 {
        self.tracks
            .iter()
            .flat_map(|t| t.clips.iter())
            .map(Clip::end_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn clip(&self, id: u32) -> Option<(usize, &Clip)> {
        self.tracks
            .iter()
            .enumerate()
            .find_map(|(i, t)| t.clips.iter().find(|c| c.id == id).map(|c| (i, c)))
    }

    pub fn clip_mut(&mut self, id: u32) -> Option<&mut Clip> {
        self.tracks
            .iter_mut()
            .flat_map(|t| t.clips.iter_mut())
            .find(|c| c.id == id)
    }

    /// Create a block; returns its ID.
    /// Insert a scene block: length = one cycle of the scene, fade in/out time
    /// from the scene settings.
    pub fn add_scene_clip(
        &mut self,
        track: usize,
        scene: &crate::show::Scene,
        start_ms: u64,
    ) -> u32 {
        let dur = scene.duration_ms().max(100);
        let id = self.add_clip(track, ClipContent::Scene(scene.id), start_ms, dur);
        if let Some(c) = self.clip_mut(id) {
            let max = u32::try_from(c.duration_ms).unwrap_or(u32::MAX);
            c.fade_in_ms = scene.settings.fade_in_ms.min(max);
            c.fade_out_ms = scene.settings.fade_out_ms.min(max);
        }
        id
    }

    pub fn add_clip(
        &mut self,
        track: usize,
        content: ClipContent,
        start_ms: u64,
        duration_ms: u64,
    ) -> u32 {
        while self.tracks.len() <= track {
            self.add_track(None);
        }
        let id = self.next();
        self.tracks[track].clips.push(Clip {
            id,
            start_ms,
            duration_ms: duration_ms.max(40),
            fade_in_ms: 0,
            fade_out_ms: 0,
            looped: true,
            tempo_sync: false,
            speed: 1.0,
            level: 1.0,
            content,
            prepare: None,
        });
        id
    }

    pub fn remove_clip(&mut self, id: u32) -> Option<Clip> {
        for t in &mut self.tracks {
            if let Some(i) = t.clips.iter().position(|c| c.id == id) {
                return Some(t.clips.remove(i));
            }
        }
        None
    }

    /// Put a block on another track.
    pub fn move_clip_to_track(&mut self, id: u32, track: usize) {
        if track >= self.tracks.len() {
            return;
        }
        if let Some(c) = self.remove_clip(id) {
            self.tracks[track].clips.push(c);
        }
    }

    /// Split a block at `at`; returns the ID of the second part.
    pub fn split_clip(&mut self, id: u32, at: u64) -> Option<u32> {
        let (track, c) = self.clip(id).map(|(t, c)| (t, c.clone()))?;
        if at <= c.start_ms + 40 || at + 40 >= c.end_ms() {
            return None;
        }
        let first = at - c.start_ms;
        let new_id = self.next();
        let mut second = c.clone();
        second.id = new_id;
        second.start_ms = at;
        second.duration_ms = c.duration_ms - first;
        second.fade_in_ms = 0;
        if let Some(offset_ms) = second.content.offset_mut() {
            *offset_ms += first;
        }
        if let Some(a) = self.clip_mut(id) {
            a.duration_ms = first;
            a.fade_out_ms = 0;
        }
        self.tracks[track].clips.push(second);
        Some(new_id)
    }

    /// Would the block (track, range) overlap another one?
    /// Move several blocks together by `dt` ms and `dtrack` tracks.
    /// Not before 0 and not beyond the existing tracks; without allowed overlap
    /// only if no other block is covered.
    /// Returns whether they were moved.
    pub fn move_group(&mut self, ids: &BTreeSet<u32>, dt: i64, dtrack: isize) -> bool {
        let group: Vec<(usize, Clip)> = ids
            .iter()
            .filter_map(|id| self.clip(*id).map(|(t, c)| (t, c.clone())))
            .collect();
        if group.is_empty() {
            return false;
        }
        let min_start = group.iter().map(|(_, c)| c.start_ms).min().unwrap_or(0) as i64;
        let dt = dt.max(-min_start);
        let (lo, hi) = group
            .iter()
            .fold((usize::MAX, 0), |(lo, hi), (t, _)| (lo.min(*t), hi.max(*t)));
        let dtrack =
            if (lo as isize + dtrack) < 0 || hi as isize + dtrack >= self.tracks.len() as isize {
                0
            } else {
                dtrack
            };
        if dt == 0 && dtrack == 0 {
            return false;
        }
        let moved: Vec<(usize, u64, u64, u32)> = group
            .iter()
            .map(|(t, c)| {
                let start = (c.start_ms as i64 + dt) as u64;
                (
                    (*t as isize + dtrack) as usize,
                    start,
                    start + c.duration_ms,
                    c.id,
                )
            })
            .collect();
        let wrong_kind = group
            .iter()
            .zip(&moved)
            .any(|((_, c), (t, ..))| !self.tracks[*t].accepts(&c.content));
        if wrong_kind {
            return false;
        }
        if !self.allow_overlap {
            let blocked = moved.iter().any(|(t, s, e, _)| {
                self.tracks[*t]
                    .clips
                    .iter()
                    .any(|c| !ids.contains(&c.id) && c.start_ms < *e && *s < c.end_ms())
            });
            if blocked {
                return false;
            }
        }
        for (t, start, _, id) in moved {
            if self.clip(id).is_some_and(|(cur, _)| cur != t) {
                self.move_clip_to_track(id, t);
            }
            if let Some(c) = self.clip_mut(id) {
                c.start_ms = start;
            }
        }
        true
    }

    pub fn overlaps(&self, track: usize, start: u64, end: u64, except: u32) -> bool {
        self.tracks.get(track).is_some_and(|t| {
            t.clips
                .iter()
                .any(|c| c.id != except && c.start_ms < end && start < c.end_ms())
        })
    }

    /// Insert blocks (track and time relative to each other as in `clips`): the
    /// group starts on track `track` at the first free position from `at`,
    /// without covering other blocks. Returns the new IDs.
    pub fn paste_group(&mut self, clips: &[(usize, Clip)], track: usize, at: u64) -> Vec<u32> {
        let (Some(first), Some(base)) = (
            clips.iter().map(|(_, c)| c.start_ms).min(),
            clips.iter().map(|(t, _)| *t).min(),
        ) else {
            return Vec::new();
        };
        let parts: Vec<(usize, u64, &Clip)> = clips
            .iter()
            .map(|(t, c)| (track + (t - base), c.start_ms - first, c))
            .collect();
        // push back until nothing collides any more
        let mut shift = at;
        'search: for _ in 0..10_000 {
            for &(ti, rel, c) in &parts {
                let (s, e) = (shift + rel, shift + rel + c.duration_ms);
                if let Some(hit) = self.tracks.get(ti).and_then(|t| {
                    t.clips
                        .iter()
                        .filter(|o| o.start_ms < e && s < o.end_ms())
                        .map(Clip::end_ms)
                        .max()
                }) {
                    shift = hit.saturating_sub(rel).max(shift + 1);
                    continue 'search;
                }
            }
            break;
        }
        parts
            .into_iter()
            .map(|(ti, rel, c)| {
                let start = shift + rel;
                let id = self.add_clip(ti, c.content.clone(), start, c.duration_ms);
                if let Some(n) = self.clip_mut(id) {
                    *n = Clip {
                        id,
                        start_ms: start,
                        ..c.clone()
                    };
                }
                id
            })
            .collect()
    }

    /// Fit the timeline grid to an audio grid: take over the tempo, put the
    /// offset on a downbeat of the block (`start_ms`, start `offset_ms` in the
    /// file).
    pub fn apply_audio_grid(&mut self, start_ms: u64, offset_ms: u64, grid: AudioGrid) {
        self.bpm = grid.bpm;
        self.grid = GridMode::Beat;
        let bar = grid.bar_ms();
        let t = start_ms as f64 + grid.downbeat_ms - offset_ms as f64;
        self.offset_ms = t.rem_euclid(bar).round() as u32;
    }

    /// Length of a beat in ms.
    pub fn beat_ms(&self) -> f64 {
        60_000.0 / f64::from(self.bpm.clamp(20.0, 400.0))
    }

    /// Blocks that contain a scene (for cleaning up when deleting).
    pub fn forget_scene(&mut self, id: SceneId) {
        for t in &mut self.tracks {
            t.clips.retain(|c| c.scene() != Some(id));
        }
    }

    /// Active scene blocks at time `t` in track order.
    pub fn active_at(&self, t: u64) -> impl Iterator<Item = (usize, &Clip)> {
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, tr)| !tr.muted)
            .flat_map(move |(i, tr)| {
                tr.clips
                    .iter()
                    .filter(move |c| c.start_ms <= t && t < c.end_ms())
                    .map(move |c| (i, c))
            })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn group_moves_together_without_covering_others() {
        use crate::show::SceneId;
        let mut tl = Timeline {
            allow_overlap: false,
            ..Default::default()
        };
        let sc = |n| ClipContent::Scene(SceneId(n));
        let a = tl.add_clip(0, sc(1), 1000, 1000);
        let b = tl.add_clip(1, sc(2), 3000, 1000);
        let other = tl.add_clip(0, sc(3), 6000, 1000);
        let ids: BTreeSet<u32> = [a, b].into();
        assert!(tl.move_group(&ids, 500, 0));
        assert_eq!(tl.clip(a).unwrap().1.start_ms, 1500);
        assert_eq!(tl.clip(b).unwrap().1.start_ms, 3500);
        // not before 0
        assert!(tl.move_group(&ids, -5000, 0));
        assert_eq!(tl.clip(a).unwrap().1.start_ms, 0);
        assert_eq!(tl.clip(b).unwrap().1.start_ms, 2000);
        // another block is in the way
        assert!(!tl.move_group(&ids, 5500, 0));
        // Track change together, not beyond the last track
        while tl.tracks.len() < 3 {
            tl.add_track(None);
        }
        assert!(tl.move_group(&ids, 0, 1));
        assert_eq!(tl.clip(a).unwrap().0, 1);
        assert_eq!(tl.clip(b).unwrap().0, 2);
        let last = tl.tracks.len() as isize;
        assert!(!tl.move_group(&ids, 0, last));
        assert_eq!(tl.clip(other).unwrap().0, 0);
    }

    #[test]
    fn media_and_light_tracks_stay_apart() {
        use crate::show::SceneId;
        let mut tl = Timeline::default();
        assert!(!tl.allow_overlap, "no overlap by default");
        let audio = ClipContent::Audio {
            path: "a.mp3".into(),
            offset_ms: 0,
            length_ms: 0,
            grid: None,
            segment_grids: Vec::new(),
            triggers: Vec::new(),
        };
        let a = tl.add_clip(0, audio, 0, 1000);
        let s = tl.add_clip(1, ClipContent::Scene(SceneId(1)), 0, 1000);
        // older show: the track with only audio becomes a media track
        tl.normalize();
        assert_eq!(tl.tracks[0].kind, TrackKind::Media);
        assert_eq!(tl.tracks[1].kind, TrackKind::Light);
        // the scene can't move onto the media track, the audio not off it
        assert!(!tl.move_group(&[s].into(), 2000, -1));
        assert!(!tl.move_group(&[a].into(), 2000, 1));
        assert!(tl.move_group(&[s].into(), 0, 1));
        assert_eq!(tl.clip(s).unwrap().0, 2);
    }

    #[test]
    fn audio_grid_sets_tempo_and_offset() {
        let mut tl = Timeline::default();
        let g = crate::audio::AudioGrid {
            bpm: 120.0,
            downbeat_ms: 300.0,
        };
        // Block from 10 s, file from 1 s: downbeat at 10 s + 0.3 s - 1 s = 9.3 s
        tl.apply_audio_grid(10_000, 1_000, g);
        assert_eq!(tl.bpm, 120.0);
        assert_eq!(tl.grid, GridMode::Beat);
        assert_eq!(f64::from(tl.offset_ms), (9_300.0f64).rem_euclid(2_000.0));
    }

    use super::*;

    #[test]
    fn super_scene_from_scenes_keeps_fades() {
        let mut show = crate::show::Show::default();
        let a = show.add_scene("A", None, vec![crate::show::Step::new(Vec::new())]);
        let b = show.add_scene("B", None, vec![crate::show::Step::new(Vec::new())]);
        show.scene_mut(a).unwrap().settings.fade_in_ms = 400;
        show.scene_mut(a).unwrap().settings.fade_out_ms = 600;
        let s = show.create_super_scene(&[a, b], "S").unwrap();
        assert_eq!(show.scenes[1].id, s);
        let sc = show.scene(s).unwrap();
        let tl = sc.timeline().unwrap();
        let clips = &tl.tracks[0].clips;
        assert_eq!(clips.len(), 2);
        assert_eq!((clips[0].fade_in_ms, clips[0].fade_out_ms), (400, 600));
        assert_eq!(clips[1].start_ms, clips[0].end_ms());
        assert_eq!(sc.duration_ms(), tl.length_ms());
    }

    #[test]
    fn paste_finds_free_place_and_keeps_gaps() {
        let mut t = Timeline::default();
        let a = t.add_clip(1, ClipContent::Scene(SceneId(1)), 0, 1000);
        let b = t.add_clip(2, ClipContent::Scene(SceneId(2)), 1500, 500);
        let group: Vec<(usize, Clip)> = [a, b]
            .iter()
            .map(|id| {
                let (tr, c) = t.clip(*id).unwrap();
                (tr, c.clone())
            })
            .collect();
        // insert three times at the same position on track 3: one after another
        let mut starts = Vec::new();
        for _ in 0..3 {
            let ids = t.paste_group(&group[..1], 3, 200);
            starts.push(t.clip(ids[0]).unwrap().1.start_ms);
        }
        assert_eq!(starts, vec![200, 1200, 2200]);
        // Group across two tracks: spacing and track offset stay
        let ids = t.paste_group(&group, 0, 0);
        let (ta, ca) = t.clip(ids[0]).map(|(x, c)| (x, c.clone())).unwrap();
        let (tb, cb) = t.clip(ids[1]).map(|(x, c)| (x, c.clone())).unwrap();
        assert_eq!((ta, tb), (0, 1));
        assert_eq!(cb.start_ms - ca.start_ms, 1500);
        // Track 1 was occupied at 0..1000 -> the group has to move back far enough
        // that the second block (track 1, +1500) is free: first start 0 works
        assert_eq!(ca.start_ms, 0);
    }

    #[test]
    fn split_and_length() {
        let mut t = Timeline::default();
        let a = t.add_clip(0, ClipContent::Scene(SceneId(1)), 1000, 5000);
        assert_eq!(t.length_ms(), 6000);
        let b = t.split_clip(a, 3000).unwrap();
        assert_eq!(t.clip(a).unwrap().1.duration_ms, 2000);
        assert_eq!(t.clip(b).unwrap().1.start_ms, 3000);
        assert_eq!(t.clip(b).unwrap().1.duration_ms, 3000);
        assert!(t.overlaps(0, 2500, 2600, 0));
        assert!(!t.overlaps(1, 2500, 2600, 0));
    }

    #[test]
    fn fades() {
        let mut t = Timeline::default();
        let a = t.add_clip(0, ClipContent::Scene(SceneId(1)), 0, 1000);
        let c = t.clip_mut(a).unwrap();
        c.fade_in_ms = 200;
        c.fade_out_ms = 200;
        assert!((c.fade_at(100) - 0.5).abs() < 1e-3);
        assert!((c.fade_at(500) - 1.0).abs() < 1e-3);
        assert!((c.fade_at(900) - 0.5).abs() < 1e-3);
    }
}
