//! Audio playback of SUPER SCENES and analysis (waveform, bands, beat grid)
//! for the timeline.
//!
//! Every frame the audio blocks of running SUPER SCENES are matched against
//! their timeline: missing players are started at the right position,
//! drifting ones are corrected, finished ones are removed.

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

/// State of the analysis of a file: running, done or unreadable.
#[derive(Clone)]
pub enum WaveState {
    Loading,
    Ready(Arc<AudioAnalysis>),
    Failed(String),
}

/// A running player: `base` = position in the file where it started
/// (≠ 0 only for formats that cannot seek).
pub(crate) struct Player {
    pub(crate) sink: Sink,
    pub(crate) base: Duration,
}

impl Player {
    pub(crate) fn pos(&self) -> Duration {
        self.base + self.sink.get_pos()
    }
}

pub struct AudioPlayer {
    stream: Option<OutputStream>,
    stream_failed: bool,
    /// Running players per (scene, block).
    sinks: HashMap<(SceneId, u32), Player>,
    waves: Arc<Mutex<HashMap<String, WaveState>>>,
    /// Sound master 0–1 (DMXCraft only, not the system volume).
    pub master: f32,
    pub mute: bool,
    /// After a reset: the MIDI fader is ignored until it reaches (or crosses)
    /// the current value.
    pub pickup: bool,
    /// Last ignored fader value (to detect crossing).
    pickup_last: Option<f32>,
    /// Running fade-out: start value, start time, duration.
    fade: Option<(f32, Instant, Duration)>,
    /// Duration of “Fade out sound” (from the settings).
    pub fade_ms: u32,
    /// Maximum volume 0–1: sound master 100 % corresponds to this value.
    pub limit: f32,
    /// Running SUPER SCENES in the last frame (to detect a start).
    running: HashSet<SceneId>,
    /// Playhead is being dragged: hold the sound until release (set by the
    /// timeline every frame, reset by `update`).
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
    /// Effective volume (master × maximum volume, mute).
    pub fn gain(&self) -> f32 {
        if self.mute {
            0.0
        } else {
            self.master * self.limit
        }
    }

    /// Set the sound master (ends a running fade-out).
    pub fn set_master(&mut self, v: f32) {
        self.master = v.clamp(0.0, 1.0);
        self.fade = None;
    }

    /// Value of a MIDI fader: after a reset only take it over once the fader
    /// has reached or crossed the current value.
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

    /// Duration of “Fade out sound”: that of a running SUPER SCENE with audio,
    /// otherwise the one from the program settings.
    pub fn fade_ms_for(&self, e: &Engine) -> u32 {
        e.show()
            .scenes
            .iter()
            .filter(|s| s.settings.audio_fade_ms > 0 && e.is_active(s.id) && has_audio(s))
            .map(|s| s.settings.audio_fade_ms)
            .next()
            .unwrap_or(self.fade_ms)
    }

    /// Fade out evenly to 0 (duration see [`Self::fade_ms_for`]).
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

    /// Sound master to 100 % (e.g. when a SUPER SCENE starts).
    fn reset(&mut self) {
        if self.master < 1.0 || self.fade.is_some() {
            self.master = 1.0;
            self.fade = None;
            // a MIDI fader is probably still at the bottom
            self.pickup = true;
            self.pickup_last = None;
        }
    }

    /// Advance the fade-out and detect starts of SUPER SCENES.
    /// Returns `true` when a fade-out has just finished.
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

    /// Analysis of a file (starts it in the background on the first call;
    /// finished analyses come from the cache).
    pub fn wave(&self, path: &str, ctx: &eframe::egui::Context) -> WaveState {
        let mut map = self.waves.lock().expect("waveforms");
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
            waves.lock().expect("waveforms").insert(path, state);
            ctx.request_repaint();
        });
        WaveState::Loading
    }

    /// Finished analysis of a file (without starting a new one).
    pub fn ready(&self, path: &str) -> Option<Arc<AudioAnalysis>> {
        match self.waves.lock().expect("waveforms").get(path) {
            Some(WaveState::Ready(a)) => Some(a.clone()),
            _ => None,
        }
    }

    /// Adapt playback to the running SUPER SCENES.
    pub fn update(&mut self, e: &mut Engine, settings: &crate::settings::Settings) {
        let scrubbing = std::mem::take(&mut self.scrubbing);
        self.fade_ms = settings.audio_fade_ms;
        self.limit = f32::from(settings.audio_limit_pct.clamp(1, 100)) / 100.0;
        if self.tick(e, settings.audio_reset_on_start) && settings.audio_fade_stop {
            // faded out: stop SUPER SCENES with audio (incl. “Jump to”)
            let with_audio: Vec<SceneId> = e
                .show()
                .scenes
                .iter()
                .filter(|s| has_audio(s))
                .map(|s| s.id)
                .filter(|id| e.is_active(*id))
                .collect();
            for id in with_audio {
                e.end_scene(id);
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
            // play slower/faster like a turntable (the pitch changes too) so the
            // sound stays in sync with the scene
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
            // playback drifts (ruler, jump, resume after pause): seek; if that is
            // not possible, restart at the position
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

/// SUPER SCENE with audio blocks?
fn has_audio(s: &dmxcraft_core::Scene) -> bool {
    s.timeline().is_some_and(|tl| {
        tl.tracks
            .iter()
            .flat_map(|t| &t.clips)
            .any(|c| matches!(c.content, ClipContent::Audio { .. }))
    })
}

/// Open the file; created from a `File` (not a `BufReader`) so the decoder
/// can seek (length known, “seekable”).
pub(crate) fn open_decoder(path: &str) -> Option<Decoder<BufReader<File>>> {
    Decoder::try_from(File::open(path).ok()?).ok()
}

/// Does the file have a sound track DMXCraft can play (also videos)?
pub fn has_sound(path: &str) -> bool {
    open_decoder(path).is_some()
}

/// Start a player at position `pos`.
pub(crate) fn start_player(stream: &OutputStream, path: &str, pos: Duration) -> Option<Player> {
    let sink = Sink::connect_new(stream.mixer());
    sink.append(open_decoder(path)?);
    if pos.is_zero() || sink.try_seek(pos).is_ok() {
        return Some(Player {
            sink,
            base: Duration::ZERO,
        });
    }
    // Format cannot seek: skip ahead to the position
    let sink = Sink::connect_new(stream.mixer());
    sink.append(open_decoder(path)?.skip_duration(pos));
    Some(Player { sink, base: pos })
}

/// Analysis from the cache or created anew (and stored).
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

/// Decode the file (mixed to mono) and analyze it.
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

/// Cache per file (path, size and modification time in the name).
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

/// Length of an audio file in ms (for new blocks).
pub fn duration_ms(path: &str) -> Option<u64> {
    let dec = open_decoder(path)?;
    if let Some(d) = dec.total_duration() {
        return Some(d.as_millis() as u64);
    }
    // no length given: count samples
    let rate = u64::from(dec.sample_rate()).max(1);
    let channels = u64::from(dec.channels()).max(1);
    let n = dec.count() as u64;
    Some(n / channels * 1000 / rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// After a reset the MIDI fader only takes effect at the very top.
    #[test]
    fn fader_pickup_after_reset() {
        let mut a = AudioPlayer::default();
        a.fader_input(0.0);
        assert_eq!(a.master, 0.0);
        a.reset();
        assert_eq!(a.master, 1.0);
        assert!(a.pickup);
        // Fader is still at the bottom / being pushed up: ignored
        a.fader_input(0.0);
        a.fader_input(0.6);
        assert_eq!(a.master, 1.0);
        // arrived at the top: takes over again
        a.fader_input(1.0);
        assert!(!a.pickup);
        a.fader_input(0.4);
        assert!((a.master - 0.4).abs() < 1e-6);
        // no change, no catch mode
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
        assert!((a.gain() - 0.3).abs() < 1e-6, "Maximum volume limited");
        a.start_fade_ms(100);
        assert!(a.fading());
        a.set_master(0.5);
        assert!(!a.fading(), "Intervention ends the fade-out");
    }

    /// Small WAV file (mono, 8 kHz, 16 bit): 0.5 s tone, 0.5 s silence.
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

    /// MP3 (if ffmpeg is present): the decoder must be able to seek, otherwise
    /// the sound does not follow the ruler.
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
        // seeking backwards only works when “seekable”
        assert!(dec.try_seek(Duration::from_millis(100)).is_ok());
        let _ = std::fs::remove_file(w);
        let _ = std::fs::remove_file(mp3);
    }

    /// Video files: the sound track of an MP4 (AAC) is read directly.
    #[test]
    fn sound_from_mp4_video() {
        let mp4 = std::env::temp_dir().join("dmxcraft_video.mp4");
        let silent = std::env::temp_dir().join("dmxcraft_video_silent.mp4");
        let made = std::process::Command::new("ffmpeg")
            .args(["-y", "-loglevel", "quiet", "-f", "lavfi", "-i"])
            .arg("testsrc=duration=1:size=64x48:rate=10")
            .args(["-f", "lavfi", "-i", "sine=frequency=440:duration=1"])
            .args(["-c:v", "mpeg4", "-c:a", "aac", "-shortest"])
            .arg(&mp4)
            .status()
            .is_ok_and(|s| s.success())
            && std::process::Command::new("ffmpeg")
                .args(["-y", "-loglevel", "quiet", "-f", "lavfi", "-i"])
                .arg("testsrc=duration=1:size=64x48:rate=10")
                .args(["-c:v", "mpeg4"])
                .arg(&silent)
                .status()
                .is_ok_and(|s| s.success());
        if !made {
            return;
        }
        assert!(has_sound(&mp4.to_string_lossy()));
        assert!(!has_sound(&silent.to_string_lossy()));
        let w = analyze_file(&mp4.to_string_lossy()).unwrap();
        assert!((900..=1100).contains(&w.duration_ms), "{}", w.duration_ms);
        assert!(w.peaks.iter().any(|p| *p > 10));
        let _ = std::fs::remove_file(mp4);
        let _ = std::fs::remove_file(silent);
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
