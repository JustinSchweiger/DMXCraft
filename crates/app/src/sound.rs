//! Playback of the SOUND tab: players with playlists, crossfade and jingle
//! pads. The settings live in the show
//! ([`dmxcraft_core::sound::SoundBoard`]), the runtime state here.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::time::{Duration, Instant};

use dmxcraft_core::sound::SoundBoard;
use rodio::{Decoder, OutputStream, Sink, Source};

use crate::audio::{start_player, Player};

/// State of a player for display.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckState {
    /// No track loaded.
    Empty,
    /// Track loaded, stopped.
    Ready,
    Playing,
    Paused,
    /// Playing and currently fading out.
    Fading,
    /// Reached the end.
    Ended,
}

#[derive(Default)]
struct DeckRt {
    /// Loaded track (index in the playlist) and its path.
    current: Option<(usize, String)>,
    player: Option<Player>,
    paused: bool,
    /// Position while stopped (ms).
    pos_ms: u64,
    cue_ms: u64,
    /// Running fade-out: start time, duration.
    fade: Option<(Instant, Duration)>,
    ended: bool,
    /// Played tracks (paths; dimmed in the playlist).
    played: HashSet<String>,
}

struct PadRt {
    sink: Sink,
    started: Instant,
}

pub struct SoundPlayer {
    stream: Option<OutputStream>,
    stream_failed: bool,
    decks: Vec<DeckRt>,
    pads: HashMap<usize, PadRt>,
    /// Crossfader 0 (A only) … 1 (B only); center = both full.
    pub xfade: f32,
    /// Running crossfade: start value, target, start time, duration, source player.
    xfade_run: Option<(f32, f32, Instant, Duration, usize)>,
}

impl Default for SoundPlayer {
    fn default() -> Self {
        Self {
            stream: None,
            stream_failed: false,
            decks: Vec::new(),
            pads: HashMap::new(),
            xfade: 0.5,
            xfade_run: None,
        }
    }
}

impl SoundPlayer {
    fn deck(&mut self, d: usize) -> &mut DeckRt {
        if self.decks.len() <= d {
            self.decks.resize_with(d + 1, DeckRt::default);
        }
        &mut self.decks[d]
    }

    fn stream(&mut self) -> Option<&OutputStream> {
        if self.stream.is_none() && !self.stream_failed {
            match rodio::OutputStreamBuilder::open_default_stream() {
                Ok(mut s) => {
                    s.log_on_drop(false);
                    self.stream = Some(s);
                }
                Err(_) => self.stream_failed = true,
            }
        }
        self.stream.as_ref()
    }

    // ---- Queries for display ----

    pub fn state(&self, d: usize) -> DeckState {
        let Some(rt) = self.decks.get(d) else {
            return DeckState::Empty;
        };
        if rt.current.is_none() {
            DeckState::Empty
        } else if rt.ended {
            DeckState::Ended
        } else if rt.player.is_none() {
            DeckState::Ready
        } else if rt.paused {
            DeckState::Paused
        } else if rt.fade.is_some() {
            DeckState::Fading
        } else {
            DeckState::Playing
        }
    }

    /// Loaded track.
    pub fn current(&self, d: usize) -> Option<usize> {
        self.decks
            .get(d)
            .and_then(|r| r.current.as_ref().map(|c| c.0))
            .filter(|i| *i != usize::MAX)
    }

    /// Path of the loaded track.
    pub fn current_path(&self, d: usize) -> Option<String> {
        self.decks
            .get(d)
            .and_then(|r| r.current.as_ref().map(|c| c.1.clone()))
    }

    /// After changes to the playlists (move, remove) determine the index of the
    /// loaded track again from its path (`usize::MAX` = no longer in its own
    /// list).
    pub fn sync(&mut self, board: &SoundBoard) {
        for (d, rt) in self.decks.iter_mut().enumerate() {
            let Some((i, path)) = &mut rt.current else {
                continue;
            };
            let list = board
                .decks
                .get(d)
                .map(|dk| dk.playlist.as_slice())
                .unwrap_or(&[]);
            if list.get(*i).map(|t| &t.path) != Some(path) {
                *i = list
                    .iter()
                    .position(|t| &t.path == path)
                    .unwrap_or(usize::MAX);
            }
        }
    }

    /// Current position (ms).
    pub fn pos_ms(&self, d: usize) -> u64 {
        self.decks.get(d).map_or(0, |r| match &r.player {
            Some(p) => p.pos().as_millis() as u64,
            None => r.pos_ms,
        })
    }

    pub fn cue_ms(&self, d: usize) -> u64 {
        self.decks.get(d).map_or(0, |r| r.cue_ms)
    }

    pub fn played(&self, d: usize, path: &str) -> bool {
        self.decks.get(d).is_some_and(|r| r.played.contains(path))
    }

    /// Remaining time of the fade-out (s), if it is running.
    pub fn fade_left(&self, d: usize) -> Option<f32> {
        let (start, dur) = self.decks.get(d)?.fade?;
        Some(dur.saturating_sub(start.elapsed()).as_secs_f32())
    }

    pub fn xfading(&self) -> bool {
        self.xfade_run.is_some()
    }

    /// Progress of a running pad (0–1), `None` when it is stopped.
    pub fn pad_progress(&self, i: usize, len_ms: u64) -> Option<f32> {
        let p = self.pads.get(&i)?;
        if len_ms == 0 {
            return Some(0.0);
        }
        let t = p.started.elapsed().as_millis() as f32 / len_ms as f32;
        Some(t.fract())
    }

    pub fn any_running(&self) -> bool {
        !self.pads.is_empty() || self.decks.iter().any(|r| r.player.is_some() && !r.paused)
    }

    // ---- Controls ----

    /// Load track `i` (it then sits at the start).
    pub fn load(&mut self, board: &SoundBoard, d: usize, i: usize) {
        let Some(t) = board.decks.get(d).and_then(|dk| dk.playlist.get(i)) else {
            return;
        };
        let path = t.path.clone();
        let rt = self.deck(d);
        rt.player = None;
        rt.current = Some((i, path));
        rt.paused = false;
        rt.pos_ms = 0;
        rt.cue_ms = 0;
        rt.fade = None;
        rt.ended = false;
    }

    /// Load track `i` and play it.
    pub fn play_track(&mut self, board: &SoundBoard, d: usize, i: usize) {
        self.load(board, d, i);
        self.start(d, 0);
    }

    fn start(&mut self, d: usize, from_ms: u64) {
        let Some((_, path)) = self.deck(d).current.clone() else {
            return;
        };
        let Some(stream) = self.stream() else {
            return;
        };
        let player = start_player(stream, &path, Duration::from_millis(from_ms), 1.0);
        if let Some(p) = &player {
            p.sink.play();
        }
        let rt = self.deck(d);
        rt.player = player;
        rt.paused = false;
        rt.ended = false;
        rt.fade = None;
    }

    /// Play/pause. Without a loaded track, the first track of the playlist not
    /// played yet starts.
    pub fn play_pause(&mut self, board: &SoundBoard, d: usize) {
        if self.deck(d).current.is_none() || self.deck(d).ended {
            let next = self.next_index(board, d);
            if let Some(i) = next {
                self.play_track(board, d, i);
            }
            return;
        }
        let rt = self.deck(d);
        match &rt.player {
            Some(p) => {
                if rt.paused {
                    p.sink.play();
                } else {
                    p.sink.pause();
                }
                rt.paused = !rt.paused;
            }
            None => {
                let from = rt.pos_ms;
                self.start(d, from);
            }
        }
    }

    pub fn stop(&mut self, d: usize) {
        let rt = self.deck(d);
        rt.player = None;
        rt.paused = false;
        rt.pos_ms = 0;
        rt.fade = None;
        rt.ended = false;
    }

    pub fn rewind(&mut self, d: usize) {
        let rt = self.deck(d);
        rt.ended = false;
        match &mut rt.player {
            Some(p) => {
                if p.sink.try_seek(Duration::ZERO).is_ok() {
                    p.base = Duration::ZERO;
                } else {
                    let paused = rt.paused;
                    self.start(d, 0);
                    if paused {
                        self.pause(d);
                    }
                }
            }
            None => rt.pos_ms = 0,
        }
    }

    fn pause(&mut self, d: usize) {
        let rt = self.deck(d);
        if let Some(p) = &rt.player {
            p.sink.pause();
            rt.paused = true;
        }
    }

    /// Seek (click into the waveform).
    pub fn seek(&mut self, d: usize, ms: u64) {
        let rt = self.deck(d);
        rt.ended = false;
        match &mut rt.player {
            Some(p) => {
                if p.sink.try_seek(Duration::from_millis(ms)).is_ok() {
                    p.base = Duration::ZERO;
                } else {
                    let paused = rt.paused;
                    self.start(d, ms);
                    if paused {
                        self.pause(d);
                    }
                }
            }
            None => rt.pos_ms = ms,
        }
    }

    /// CUE: if the track is playing, go back to the cue mark and stop; otherwise
    /// set the cue mark to the current position.
    pub fn cue(&mut self, d: usize) {
        let playing = self.state(d) == DeckState::Playing || self.state(d) == DeckState::Fading;
        if playing {
            let cue = self.deck(d).cue_ms;
            self.seek(d, cue);
            self.pause(d);
        } else {
            let pos = self.pos_ms(d);
            self.deck(d).cue_ms = pos;
        }
    }

    /// Fade out over the player's `fade_ms`, then stop.
    pub fn fade_out(&mut self, board: &SoundBoard, d: usize) {
        let ms = board.decks.get(d).map_or(3000, |dk| dk.fade_ms);
        let rt = self.deck(d);
        if rt.player.is_some() && !rt.paused {
            rt.fade = Some((Instant::now(), Duration::from_millis(u64::from(ms.max(1)))));
        }
    }

    /// Target of “Crossfade”: the player that is not playing; if both or none
    /// are playing, by the position of the crossfader.
    pub fn xfade_target(&self) -> usize {
        let on = |d| matches!(self.state(d), DeckState::Playing | DeckState::Fading);
        match (on(0), on(1)) {
            (true, false) => 1,
            (false, true) => 0,
            _ if self.xfade <= 0.5 => 1,
            _ => 0,
        }
    }

    /// Crossfade to the other player: starts it if needed and stops the
    /// previous one at the end.
    pub fn crossfade(&mut self, board: &SoundBoard) {
        let to = self.xfade_target();
        let (from, target) = if to == 1 { (0, 1.0) } else { (1, 0.0) };
        if matches!(
            self.state(to),
            DeckState::Empty | DeckState::Ready | DeckState::Ended | DeckState::Paused
        ) {
            self.play_pause(board, to);
        }
        let dur = Duration::from_millis(u64::from(board.crossfade_ms.max(1)));
        self.xfade_run = Some((self.xfade, target, Instant::now(), dur, from));
    }

    /// Press a pad: if it is playing it stops, otherwise it starts.
    pub fn pad(&mut self, board: &SoundBoard, i: usize) {
        if self.pads.remove(&i).is_some() {
            return;
        }
        let Some(pad) = board.pads.get(i) else {
            return;
        };
        if pad.path.is_empty() {
            return;
        }
        if pad.exclusive {
            self.pads.clear();
        }
        let (path, repeat) = (pad.path.clone(), pad.repeat);
        let Some(stream) = self.stream() else {
            return;
        };
        let Some(dec) = File::open(&path)
            .ok()
            .and_then(|f| Decoder::try_from(f).ok())
        else {
            return;
        };
        let sink = Sink::connect_new(stream.mixer());
        if repeat {
            sink.append(dec.buffered().repeat_infinite());
        } else {
            sink.append(dec);
        }
        self.pads.insert(
            i,
            PadRt {
                sink,
                started: Instant::now(),
            },
        );
    }

    pub fn stop_pads(&mut self) {
        self.pads.clear();
    }

    /// Stop everything (e.g. when the show is closed).
    pub fn stop_all(&mut self) {
        self.pads.clear();
        self.decks.clear();
        self.xfade_run = None;
    }

    /// Next track: after the loaded one, otherwise the first one not played.
    fn next_index(&self, board: &SoundBoard, d: usize) -> Option<usize> {
        let list = &board.decks.get(d)?.playlist;
        if list.is_empty() {
            return None;
        }
        let rt = self.decks.get(d);
        match rt.and_then(|r| r.current.as_ref()) {
            Some((i, _)) if *i != usize::MAX && i + 1 < list.len() => Some(i + 1),
            Some((i, _)) if *i != usize::MAX => None,
            _ => (0..list.len())
                .find(|i| !rt.is_some_and(|r| r.played.contains(&list[*i].path)))
                .or(Some(0)),
        }
    }

    /// Every frame: set volumes, detect track ends, auto-advance.
    /// `gain` = sound master of DMXCraft.
    pub fn update(&mut self, board: &SoundBoard, gain: f32) -> bool {
        self.sync(board);
        let master = board.master.clamp(0.0, 1.0) * gain;
        // Advance the crossfade
        if let Some((a, b, start, dur, from)) = self.xfade_run {
            let t = (start.elapsed().as_secs_f32() / dur.as_secs_f32()).min(1.0);
            self.xfade = a + (b - a) * t;
            if t >= 1.0 {
                self.xfade_run = None;
                self.stop(from);
                self.xfade = 0.5;
            }
        }
        let x = self.xfade.clamp(0.0, 1.0);
        let xgain = [(2.0 * (1.0 - x)).min(1.0), (2.0 * x).min(1.0)];
        let mut ended = Vec::new();
        for (d, rt) in self.decks.iter_mut().enumerate() {
            let Some(p) = &rt.player else {
                continue;
            };
            let mut fade = 1.0;
            if let Some((start, dur)) = rt.fade {
                let t = start.elapsed().as_secs_f32() / dur.as_secs_f32();
                if t >= 1.0 {
                    ended.push((d, true));
                    continue;
                }
                fade = 1.0 - t;
            }
            let vol = board.decks.get(d).map_or(1.0, |dk| dk.volume);
            let xg = xgain.get(d).copied().unwrap_or(1.0);
            p.sink
                .set_volume((vol * fade * xg * master).clamp(0.0, 1.0));
            if p.sink.empty() {
                ended.push((d, false));
            }
        }
        for (d, faded) in ended {
            if let Some((_, path)) = self.decks[d].current.clone() {
                self.decks[d].played.insert(path);
            }
            let deck = board.decks.get(d).cloned().unwrap_or_default();
            if !faded && deck.repeat {
                self.start(d, 0);
                continue;
            }
            let next = self.next_index(board, d);
            match next {
                Some(n) if deck.auto_next && !faded => self.play_track(board, d, n),
                Some(n) if deck.auto_next => self.load(board, d, n),
                _ => {
                    let rt = &mut self.decks[d];
                    rt.player = None;
                    rt.fade = None;
                    rt.ended = !faded;
                    rt.pos_ms = 0;
                }
            }
        }
        // Pads
        self.pads.retain(|i, p| {
            let vol = board.pads.get(*i).map_or(1.0, |pd| pd.volume);
            p.sink.set_volume((vol * master).clamp(0.0, 1.0));
            !p.sink.empty()
        });
        self.any_running() || self.xfade_run.is_some()
    }
}
