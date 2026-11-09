//! Wiedergabe des Reiters SOUND: Player mit Playlisten, Überblenden und
//! Jingle-Pads. Die Einstellungen liegen in der Show
//! ([`dmxcraft_core::sound::SoundBoard`]), der Laufzustand hier.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::time::{Duration, Instant};

use dmxcraft_core::sound::SoundBoard;
use rodio::{Decoder, OutputStream, Sink, Source};

use crate::audio::{start_player, Player};

/// Zustand eines Players für die Anzeige.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeckState {
    /// Kein Titel geladen.
    Empty,
    /// Titel geladen, steht.
    Ready,
    Playing,
    Paused,
    /// Läuft und blendet gerade aus.
    Fading,
    /// Am Ende angekommen.
    Ended,
}

#[derive(Default)]
struct DeckRt {
    /// Geladener Titel (Index in der Playlist) und dessen Pfad.
    current: Option<(usize, String)>,
    player: Option<Player>,
    paused: bool,
    /// Stelle beim Stehen (ms).
    pos_ms: u64,
    cue_ms: u64,
    /// Laufendes Ausfaden: Beginn, Dauer.
    fade: Option<(Instant, Duration)>,
    ended: bool,
    /// Gespielte Titel (abgeblendet in der Playlist).
    played: HashSet<usize>,
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
    /// Überblendregler 0 (nur A) … 1 (nur B); Mitte = beide voll.
    pub xfade: f32,
    /// Laufendes Überblenden: Startwert, Ziel, Beginn, Dauer, Quell-Player.
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

    // ---- Abfragen für die Anzeige ----

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

    /// Geladener Titel.
    pub fn current(&self, d: usize) -> Option<usize> {
        self.decks
            .get(d)
            .and_then(|r| r.current.as_ref().map(|c| c.0))
    }

    /// Aktuelle Stelle (ms).
    pub fn pos_ms(&self, d: usize) -> u64 {
        self.decks.get(d).map_or(0, |r| match &r.player {
            Some(p) => p.pos().as_millis() as u64,
            None => r.pos_ms,
        })
    }

    pub fn cue_ms(&self, d: usize) -> u64 {
        self.decks.get(d).map_or(0, |r| r.cue_ms)
    }

    pub fn played(&self, d: usize, i: usize) -> bool {
        self.decks.get(d).is_some_and(|r| r.played.contains(&i))
    }

    /// Restzeit des Ausfadens (s), falls es läuft.
    pub fn fade_left(&self, d: usize) -> Option<f32> {
        let (start, dur) = self.decks.get(d)?.fade?;
        Some(dur.saturating_sub(start.elapsed()).as_secs_f32())
    }

    pub fn xfading(&self) -> bool {
        self.xfade_run.is_some()
    }

    /// Fortschritt eines laufenden Pads (0–1), `None` wenn es steht.
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

    // ---- Bedienung ----

    /// Titel `i` laden (steht danach am Anfang).
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

    /// Titel `i` laden und abspielen.
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
        let player = start_player(stream, &path, Duration::from_millis(from_ms));
        let rt = self.deck(d);
        rt.player = player;
        rt.paused = false;
        rt.ended = false;
        rt.fade = None;
    }

    /// Play/Pause. Ohne geladenen Titel startet der erste noch nicht
    /// gespielte der Playlist.
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

    /// Springen (Klick in die Wellenform).
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

    /// CUE: läuft der Titel, zurück zur Cue-Marke und anhalten; sonst die
    /// Cue-Marke an die aktuelle Stelle setzen.
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

    /// Ausfaden über `fade_ms` des Players, danach Stopp.
    pub fn fade_out(&mut self, board: &SoundBoard, d: usize) {
        let ms = board.decks.get(d).map_or(3000, |dk| dk.fade_ms);
        let rt = self.deck(d);
        if rt.player.is_some() && !rt.paused {
            rt.fade = Some((Instant::now(), Duration::from_millis(u64::from(ms.max(1)))));
        }
    }

    /// Ziel von „Überblenden“: der Player, der gerade nicht läuft; laufen
    /// beide oder keiner, nach der Stellung des Reglers.
    pub fn xfade_target(&self) -> usize {
        let on = |d| matches!(self.state(d), DeckState::Playing | DeckState::Fading);
        match (on(0), on(1)) {
            (true, false) => 1,
            (false, true) => 0,
            _ if self.xfade <= 0.5 => 1,
            _ => 0,
        }
    }

    /// Überblenden zum anderen Player: startet ihn bei Bedarf und stoppt den
    /// bisherigen am Ende.
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

    /// Pad drücken: läuft es, wird es gestoppt, sonst gestartet.
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

    /// Alles anhalten (z. B. beim Schließen der Show).
    pub fn stop_all(&mut self) {
        self.pads.clear();
        self.decks.clear();
        self.xfade_run = None;
    }

    /// Nächster Titel: nach dem geladenen, sonst der erste nicht gespielte.
    fn next_index(&self, board: &SoundBoard, d: usize) -> Option<usize> {
        let list = &board.decks.get(d)?.playlist;
        if list.is_empty() {
            return None;
        }
        let rt = self.decks.get(d);
        match rt.and_then(|r| r.current.as_ref()) {
            Some((i, _)) if i + 1 < list.len() => Some(i + 1),
            Some(_) => None,
            None => (0..list.len())
                .find(|i| !rt.is_some_and(|r| r.played.contains(i)))
                .or(Some(0)),
        }
    }

    /// Jedes Frame: Lautstärken setzen, Enden erkennen, automatisch weiter.
    /// `gain` = Ton-Master von DMXCraft.
    pub fn update(&mut self, board: &SoundBoard, gain: f32) -> bool {
        let master = board.master.clamp(0.0, 1.0) * gain;
        // Überblenden fortschreiben
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
            let i = self.decks[d].current.as_ref().map(|c| c.0);
            if let Some(i) = i {
                self.decks[d].played.insert(i);
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
