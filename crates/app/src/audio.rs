//! Audiowiedergabe der SUPER SCENES und Analyse (Wellenform, Bänder, Beat-
//! Raster) für die Zeitleiste.
//!
//! Jedes Frame werden die Audioblöcke laufender SUPER SCENES mit ihrer
//! Zeitleiste abgeglichen: fehlende Spieler werden an der richtigen Stelle
//! gestartet, abweichende nachgestellt, beendete entfernt.

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub use dmxcraft_core::audio::FRAME_MS;
use dmxcraft_core::audio::{Analyzer, AudioAnalysis, VERSION};
use dmxcraft_core::timeline::ClipContent;
use dmxcraft_core::{Engine, SceneId};
use rodio::{Decoder, OutputStream, Sink, Source};

/// Zustand der Analyse einer Datei: läuft, fertig oder nicht lesbar.
#[derive(Clone)]
pub enum WaveState {
    Loading,
    Ready(Arc<AudioAnalysis>),
    Failed(String),
}

/// Ein laufender Spieler: `base` = Stelle in der Datei, an der er begann
/// (nur bei Formaten ohne Sprungmöglichkeit ≠ 0).
struct Player {
    sink: Sink,
    base: Duration,
}

impl Player {
    fn pos(&self) -> Duration {
        self.base + self.sink.get_pos()
    }
}

pub struct AudioPlayer {
    stream: Option<OutputStream>,
    stream_failed: bool,
    /// Laufende Spieler je (Szene, Block).
    sinks: HashMap<(SceneId, u32), Player>,
    waves: Arc<Mutex<HashMap<String, WaveState>>>,
    /// Ton-Master 0–1 (nur DMXCraft, nicht die Systemlautstärke).
    pub master: f32,
    pub mute: bool,
    /// Nach dem Zurücksetzen: MIDI-Fader wird ignoriert, bis er den
    /// aktuellen Wert erreicht (bzw. überquert).
    pub pickup: bool,
    /// Letzter ignorierter Fader-Wert (zum Erkennen des Überquerens).
    pickup_last: Option<f32>,
    /// Laufendes Ausfaden: Startwert, Beginn, Dauer.
    fade: Option<(f32, Instant, Duration)>,
    /// Dauer für „Ton ausfaden“ (aus den Einstellungen).
    pub fade_ms: u32,
    /// Höchstlautstärke 0–1: Ton-Master 100 % entspricht diesem Wert.
    pub limit: f32,
    /// Laufende SUPER SCENES im letzten Frame (zum Erkennen eines Starts).
    running: HashSet<SceneId>,
    /// Abspielkopf wird gezogen: Ton bis zum Loslassen anhalten (setzt die
    /// Zeitleiste in jedem Frame, `update` setzt es zurück).
    pub scrubbing: bool,
}

impl Default for AudioPlayer {
    fn default() -> Self {
        Self {
            stream: None,
            stream_failed: false,
            sinks: HashMap::new(),
            waves: Arc::default(),
            master: 1.0,
            mute: false,
            pickup: false,
            pickup_last: None,
            fade: None,
            fade_ms: 5000,
            limit: 1.0,
            running: HashSet::new(),
            scrubbing: false,
        }
    }
}

impl AudioPlayer {
    /// Wirksame Lautstärke (Master × Höchstlautstärke, stumm).
    pub fn gain(&self) -> f32 {
        if self.mute {
            0.0
        } else {
            self.master * self.limit
        }
    }

    /// Ton-Master setzen (beendet ein laufendes Ausfaden).
    pub fn set_master(&mut self, v: f32) {
        self.master = v.clamp(0.0, 1.0);
        self.fade = None;
    }

    /// Wert eines MIDI-Faders: nach dem Zurücksetzen erst übernehmen, wenn
    /// der Fader den aktuellen Wert erreicht oder überquert hat.
    pub fn fader_input(&mut self, v: f32) {
        if self.pickup {
            let near = (v - self.master).abs() <= 0.03;
            let crossed = self
                .pickup_last
                .is_some_and(|l| (l - self.master).signum() != (v - self.master).signum());
            if !near && !crossed {
                self.pickup_last = Some(v);
                return;
            }
            self.pickup = false;
            self.pickup_last = None;
        }
        self.set_master(v);
    }

    /// Dauer von „Ton ausfaden“: die einer laufenden SUPER SCENE mit Ton,
    /// sonst die aus den Programmeinstellungen.
    pub fn fade_ms_for(&self, e: &Engine) -> u32 {
        e.show()
            .scenes
            .iter()
            .filter(|s| s.settings.audio_fade_ms > 0 && e.is_active(s.id) && has_audio(s))
            .map(|s| s.settings.audio_fade_ms)
            .next()
            .unwrap_or(self.fade_ms)
    }

    /// Gleichmäßig auf 0 ausblenden (Dauer siehe [`Self::fade_ms_for`]).
    pub fn start_fade(&mut self, e: &Engine) {
        let ms = self.fade_ms_for(e);
        self.start_fade_ms(ms);
    }

    fn start_fade_ms(&mut self, ms: u32) {
        if self.master > 0.0 {
            let ms = u64::from(ms.max(100));
            self.fade = Some((self.master, Instant::now(), Duration::from_millis(ms)));
        }
    }

    pub fn fading(&self) -> bool {
        self.fade.is_some()
    }

    /// Ton-Master auf 100 % (z. B. beim Start einer SUPER SCENE).
    fn reset(&mut self) {
        if self.master < 1.0 || self.fade.is_some() {
            self.master = 1.0;
            self.fade = None;
            // ein MIDI-Fader steht vermutlich noch unten
            self.pickup = true;
            self.pickup_last = None;
        }
    }

    /// Ausfaden fortschreiben und Starts von SUPER SCENES erkennen.
    /// Gibt `true` zurück, wenn ein Ausfaden gerade fertig wurde.
    fn tick(&mut self, e: &Engine, reset_on_start: bool) -> bool {
        let mut done = false;
        if let Some((from, start, dur)) = self.fade {
            let k = start.elapsed().as_secs_f32() / dur.as_secs_f32();
            if k >= 1.0 {
                self.master = 0.0;
                self.fade = None;
                done = true;
            } else {
                self.master = from * (1.0 - k);
            }
        }
        let running: HashSet<SceneId> = e
            .show()
            .scenes
            .iter()
            .filter(|s| s.timeline().is_some() && e.timeline_time(s.id).is_some())
            .map(|s| s.id)
            .collect();
        if reset_on_start && running.iter().any(|id| !self.running.contains(id)) {
            self.reset();
        }
        self.running = running;
        done
    }

    /// Analyse einer Datei (startet sie beim ersten Aufruf im Hintergrund;
    /// fertige Analysen kommen aus dem Zwischenspeicher).
    pub fn wave(&self, path: &str, ctx: &eframe::egui::Context) -> WaveState {
        let mut map = self.waves.lock().expect("Wellenformen");
        if let Some(w) = map.get(path) {
            return w.clone();
        }
        map.insert(path.to_string(), WaveState::Loading);
        let waves = self.waves.clone();
        let path = path.to_string();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let state = match analysis(&path) {
                Ok(w) => WaveState::Ready(Arc::new(w)),
                Err(e) => WaveState::Failed(e),
            };
            waves.lock().expect("Wellenformen").insert(path, state);
            ctx.request_repaint();
        });
        WaveState::Loading
    }

    /// Fertige Analyse einer Datei (ohne eine neue zu starten).
    pub fn ready(&self, path: &str) -> Option<Arc<AudioAnalysis>> {
        match self.waves.lock().expect("Wellenformen").get(path) {
            Some(WaveState::Ready(a)) => Some(a.clone()),
            _ => None,
        }
    }

    /// Wiedergabe an die laufenden SUPER SCENES anpassen.
    pub fn update(&mut self, e: &mut Engine, settings: &crate::settings::Settings) {
        let scrubbing = std::mem::take(&mut self.scrubbing);
        self.fade_ms = settings.audio_fade_ms;
        self.limit = f32::from(settings.audio_limit_pct.clamp(1, 100)) / 100.0;
        if self.tick(e, settings.audio_reset_on_start) && settings.audio_fade_stop {
            // ausgeblendet: SUPER SCENES mit Ton stoppen
            let with_audio: Vec<SceneId> = e
                .show()
                .scenes
                .iter()
                .filter(|s| has_audio(s))
                .map(|s| s.id)
                .filter(|id| e.is_active(*id))
                .collect();
            for id in with_audio {
                e.release(id);
            }
        }
        let gain = self.gain();
        let mut wanted: HashMap<(SceneId, u32), (String, Duration, f32, f32)> = HashMap::new();
        for scene in &e.show().scenes {
            let Some(tl) = scene.timeline() else {
                continue;
            };
            let Some(t) = e.timeline_time(scene.id) else {
                continue;
            };
            let rate = if e.timeline_running(scene.id) && !scrubbing {
                e.speed() * f32::from(scene.settings.speed_pct) / 100.0
            } else {
                0.0
            };
            let now = t as u64;
            for (_, clip) in tl.active_at(now) {
                if let ClipContent::Audio {
                    path, offset_ms, ..
                } = &clip.content
                {
                    let pos = offset_ms + (now - clip.start_ms);
                    let vol = clip.level * clip.fade_at(now) * gain;
                    wanted.insert(
                        (scene.id, clip.id),
                        (path.clone(), Duration::from_millis(pos), vol, rate),
                    );
                }
            }
        }
        self.sinks.retain(|k, _| wanted.contains_key(k));
        if wanted.is_empty() {
            return;
        }
        if self.stream.is_none() && !self.stream_failed {
            match rodio::OutputStreamBuilder::open_default_stream() {
                Ok(mut s) => {
                    s.log_on_drop(false);
                    self.stream = Some(s);
                }
                Err(_) => self.stream_failed = true,
            }
        }
        let Some(stream) = &self.stream else {
            return;
        };
        for (key, (path, pos, vol, rate)) in wanted {
            let vol = vol.clamp(0.0, 1.0);
            // langsamer/schneller abspielen wie ein Plattenspieler (Tonhöhe
            // ändert sich mit), damit der Ton synchron zur Szene bleibt
            let running = rate > 0.01;
            if let std::collections::hash_map::Entry::Vacant(slot) = self.sinks.entry(key) {
                let Some(p) = start_player(stream, &path, pos) else {
                    continue;
                };
                slot.insert(p);
            }
            let Some(p) = self.sinks.get(&key) else {
                continue;
            };
            p.sink.set_volume(vol);
            if running {
                p.sink.set_speed(rate.clamp(0.05, 8.0));
            }
            if !running {
                p.sink.pause();
                continue;
            }
            // weicht die Wiedergabe ab (Lineal, Sprung, Fortsetzen nach
            // Pause): springen; geht das nicht, an der Stelle neu starten
            let drift = p.pos().as_secs_f64() - pos.as_secs_f64();
            if p.sink.is_paused() || drift.abs() > 0.15 {
                let seeked = p.base.is_zero() && p.sink.try_seek(pos).is_ok();
                if !seeked {
                    self.sinks.remove(&key);
                    let Some(n) = start_player(stream, &path, pos) else {
                        continue;
                    };
                    n.sink.set_volume(vol);
                    n.sink.set_speed(rate.clamp(0.05, 8.0));
                    self.sinks.insert(key, n);
                    continue;
                }
            }
            p.sink.play();
        }
    }
}

/// SUPER SCENE mit Audioblöcken?
fn has_audio(s: &dmxcraft_core::Scene) -> bool {
    s.timeline().is_some_and(|tl| {
        tl.tracks
            .iter()
            .flat_map(|t| &t.clips)
            .any(|c| matches!(c.content, ClipContent::Audio { .. }))
    })
}

/// Datei öffnen; aus einer `File` (nicht `BufReader`) erzeugt, damit der
/// Decoder springen kann (Länge bekannt, „seekable“).
pub(crate) fn open_decoder(path: &str) -> Option<Decoder<BufReader<File>>> {
    Decoder::try_from(File::open(path).ok()?).ok()
}

/// Spieler an Stelle `pos` starten.
fn start_player(stream: &OutputStream, path: &str, pos: Duration) -> Option<Player> {
    let sink = Sink::connect_new(stream.mixer());
    sink.append(open_decoder(path)?);
    if pos.is_zero() || sink.try_seek(pos).is_ok() {
        return Some(Player {
            sink,
            base: Duration::ZERO,
        });
    }
    // Format ohne Sprungmöglichkeit: bis zur Stelle vorspulen
    let sink = Sink::connect_new(stream.mixer());
    sink.append(open_decoder(path)?.skip_duration(pos));
    Some(Player { sink, base: pos })
}

/// Analyse aus dem Zwischenspeicher oder neu erstellen (und speichern).
fn analysis(path: &str) -> Result<AudioAnalysis, String> {
    let cache = cache_file(path);
    if let Some(a) = cache
        .as_ref()
        .and_then(|c| std::fs::read(c).ok())
        .and_then(|b| serde_json::from_slice::<AudioAnalysis>(&b).ok())
        .filter(|a| a.version == VERSION)
    {
        return Ok(a);
    }
    let a = analyze_file(path)?;
    if let Some(c) = cache {
        if let Some(dir) = c.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(json) = serde_json::to_vec(&a) {
            let _ = std::fs::write(c, json);
        }
    }
    Ok(a)
}

/// Datei dekodieren (auf Mono gemischt) und analysieren.
fn analyze_file(path: &str) -> Result<AudioAnalysis, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let dec = Decoder::try_from(file).map_err(|e| e.to_string())?;
    let channels = usize::from(dec.channels()).max(1);
    let mut an = Analyzer::new(dec.sample_rate());
    let (mut sum, mut n) = (0.0f32, 0usize);
    for s in dec {
        sum += s;
        n += 1;
        if n == channels {
            an.push(sum / channels as f32);
            sum = 0.0;
            n = 0;
        }
    }
    Ok(an.finish())
}

/// Zwischenspeicher je Datei (Pfad, Größe und Änderungszeit im Namen).
fn cache_file(path: &str) -> Option<std::path::PathBuf> {
    let meta = std::fs::metadata(path).ok()?;
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map_or(0, |d| d.as_secs());
    let key = format!("{path}|{}|{mtime}", meta.len());
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in key.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    let dir = crate::settings::Settings::path()?
        .parent()?
        .join("audio-cache");
    Some(dir.join(format!("{h:016x}.json")))
}

/// Länge einer Audiodatei in ms (für neue Blöcke).
pub fn duration_ms(path: &str) -> Option<u64> {
    let dec = open_decoder(path)?;
    if let Some(d) = dec.total_duration() {
        return Some(d.as_millis() as u64);
    }
    // ohne Längenangabe: Samples zählen
    let rate = u64::from(dec.sample_rate()).max(1);
    let channels = u64::from(dec.channels()).max(1);
    let n = dec.count() as u64;
    Some(n / channels * 1000 / rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Nach dem Zurücksetzen wirkt der MIDI-Fader erst ganz oben.
    #[test]
    fn fader_pickup_after_reset() {
        let mut a = AudioPlayer::default();
        a.fader_input(0.0);
        assert_eq!(a.master, 0.0);
        a.reset();
        assert_eq!(a.master, 1.0);
        assert!(a.pickup);
        // Fader steht noch unten / wird hochgeschoben: ignoriert
        a.fader_input(0.0);
        a.fader_input(0.6);
        assert_eq!(a.master, 1.0);
        // oben angekommen: übernimmt wieder
        a.fader_input(1.0);
        assert!(!a.pickup);
        a.fader_input(0.4);
        assert!((a.master - 0.4).abs() < 1e-6);
        // ohne Änderung kein Fangmodus
        a.set_master(1.0);
        a.reset();
        assert!(!a.pickup);
    }

    #[test]
    fn mute_and_fade() {
        let mut a = AudioPlayer {
            mute: true,
            ..Default::default()
        };
        assert_eq!(a.gain(), 0.0);
        a.mute = false;
        a.limit = 0.3;
        assert!((a.gain() - 0.3).abs() < 1e-6, "Höchstlautstärke begrenzt");
        a.start_fade_ms(100);
        assert!(a.fading());
        a.set_master(0.5);
        assert!(!a.fading(), "Eingriff beendet das Ausfaden");
    }

    /// Kleine WAV-Datei (Mono, 8 kHz, 16 Bit): 0,5 s Ton, 0,5 s Stille.
    fn wav(path: &std::path::Path) {
        let rate = 8000u32;
        let n = rate as usize;
        let mut data = Vec::with_capacity(n * 2);
        for i in 0..n {
            let v: i16 = if i < n / 2 {
                ((i as f32 * 0.3).sin() * 20_000.0) as i16
            } else {
                0
            };
            data.extend_from_slice(&v.to_le_bytes());
        }
        let mut f = Vec::new();
        f.extend_from_slice(b"RIFF");
        f.extend_from_slice(&(36 + data.len() as u32).to_le_bytes());
        f.extend_from_slice(b"WAVEfmt ");
        f.extend_from_slice(&16u32.to_le_bytes());
        f.extend_from_slice(&1u16.to_le_bytes());
        f.extend_from_slice(&1u16.to_le_bytes());
        f.extend_from_slice(&rate.to_le_bytes());
        f.extend_from_slice(&(rate * 2).to_le_bytes());
        f.extend_from_slice(&2u16.to_le_bytes());
        f.extend_from_slice(&16u16.to_le_bytes());
        f.extend_from_slice(b"data");
        f.extend_from_slice(&(data.len() as u32).to_le_bytes());
        f.extend_from_slice(&data);
        std::fs::write(path, f).unwrap();
    }

    /// MP3 (falls ffmpeg da ist): der Decoder muss springen können, sonst
    /// folgt der Ton dem Lineal nicht.
    #[test]
    fn mp3_decoder_can_seek() {
        let w = std::env::temp_dir().join("dmxcraft_seek.wav");
        let mp3 = std::env::temp_dir().join("dmxcraft_seek.mp3");
        wav(&w);
        let ok = std::process::Command::new("ffmpeg")
            .args(["-y", "-loglevel", "quiet", "-i"])
            .arg(&w)
            .arg(&mp3)
            .status()
            .is_ok_and(|s| s.success());
        if !ok {
            return;
        }
        let mut dec = open_decoder(&mp3.to_string_lossy()).expect("Decoder");
        assert!(dec.try_seek(Duration::from_millis(600)).is_ok());
        // zurückspringen geht nur mit „seekable“
        assert!(dec.try_seek(Duration::from_millis(100)).is_ok());
        let _ = std::fs::remove_file(w);
        let _ = std::fs::remove_file(mp3);
    }

    #[test]
    fn waveform_and_duration() {
        let path = std::env::temp_dir().join("dmxcraft_test_tone.wav");
        wav(&path);
        let p = path.to_string_lossy();
        assert_eq!(duration_ms(&p), Some(1000));
        let w = analyze_file(&p).unwrap();
        assert!((95..=105).contains(&w.peaks.len()), "{}", w.peaks.len());
        assert!(w.peaks[10] > 100);
        assert!(w.peaks[80] < 3);
        assert_eq!(w.duration_ms, 1000);
        let _ = std::fs::remove_file(path);
    }
}
