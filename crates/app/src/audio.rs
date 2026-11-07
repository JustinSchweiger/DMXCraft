//! Audiowiedergabe der SUPER SCENES und Wellenformen für die Zeitleiste.
//!
//! Jedes Frame werden die Audioblöcke laufender SUPER SCENES mit ihrer
//! Zeitleiste abgeglichen: fehlende Spieler werden an der richtigen Stelle
//! gestartet, abweichende nachgestellt, beendete entfernt.

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dmxcraft_core::timeline::ClipContent;
use dmxcraft_core::{Engine, SceneId};
use rodio::{Decoder, OutputStream, Sink, Source};

/// Spitzenwerte einer Datei für die Anzeige.
pub struct Waveform {
    /// Höchster Ausschlag je 10 ms (0..=1).
    pub peaks: Vec<f32>,
}

/// Zustand einer Wellenform: wird gelesen, fertig oder nicht lesbar.
#[derive(Clone)]
pub enum WaveState {
    Loading,
    Ready(Arc<Waveform>),
    Failed(String),
}

#[derive(Default)]
pub struct AudioPlayer {
    stream: Option<OutputStream>,
    stream_failed: bool,
    /// Laufende Spieler je (Szene, Block).
    sinks: HashMap<(SceneId, u32), Sink>,
    waves: Arc<Mutex<HashMap<String, WaveState>>>,
}

impl AudioPlayer {
    /// Wellenform einer Datei (startet das Lesen beim ersten Aufruf).
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
            let state = match read_wave(&path) {
                Ok(w) => WaveState::Ready(Arc::new(w)),
                Err(e) => WaveState::Failed(e),
            };
            waves.lock().expect("Wellenformen").insert(path, state);
            ctx.request_repaint();
        });
        WaveState::Loading
    }

    /// Wiedergabe an die laufenden SUPER SCENES anpassen.
    pub fn update(&mut self, e: &Engine) {
        let mut wanted: HashMap<(SceneId, u32), (String, Duration, f32, bool)> = HashMap::new();
        for scene in &e.show().scenes {
            let Some(tl) = scene.timeline() else {
                continue;
            };
            let Some(t) = e.timeline_time(scene.id) else {
                continue;
            };
            let running = e.timeline_running(scene.id);
            let now = t as u64;
            for (_, clip) in tl.active_at(now) {
                if let ClipContent::Audio {
                    path, offset_ms, ..
                } = &clip.content
                {
                    let pos = offset_ms + (now - clip.start_ms);
                    let vol = clip.level * clip.fade_at(now) * e.master();
                    wanted.insert(
                        (scene.id, clip.id),
                        (path.clone(), Duration::from_millis(pos), vol, running),
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
        for (key, (path, pos, vol, running)) in wanted {
            let sink = match self.sinks.get(&key) {
                Some(s) => s,
                None => {
                    let Ok(file) = File::open(&path) else {
                        continue;
                    };
                    let Ok(dec) = Decoder::try_from(BufReader::new(file)) else {
                        continue;
                    };
                    let sink = Sink::connect_new(stream.mixer());
                    sink.append(dec);
                    let _ = sink.try_seek(pos);
                    self.sinks.entry(key).or_insert(sink)
                }
            };
            sink.set_volume(vol.clamp(0.0, 1.0));
            if running {
                if sink.is_paused() {
                    let _ = sink.try_seek(pos);
                    sink.play();
                }
                // weicht die Wiedergabe ab, nachstellen
                let drift = sink.get_pos().as_secs_f64() - pos.as_secs_f64();
                if drift.abs() > 0.15 {
                    let _ = sink.try_seek(pos);
                }
            } else {
                sink.pause();
            }
        }
    }
}

/// Datei ganz lesen und Spitzenwerte je 10 ms sammeln.
fn read_wave(path: &str) -> Result<Waveform, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let dec = Decoder::try_from(BufReader::new(file)).map_err(|e| e.to_string())?;
    let rate = dec.sample_rate() as usize;
    let channels = usize::from(dec.channels()).max(1);
    let per_peak = (rate * channels / 100).max(1);
    let mut peaks = Vec::new();
    let mut max = 0.0f32;
    let mut n = 0usize;
    for s in dec {
        max = max.max(s.abs());
        n += 1;
        if n == per_peak {
            peaks.push(max.min(1.0));
            max = 0.0;
            n = 0;
        }
    }
    if n > 0 {
        peaks.push(max.min(1.0));
    }
    Ok(Waveform { peaks })
}

/// Länge einer Audiodatei in ms (für neue Blöcke).
pub fn duration_ms(path: &str) -> Option<u64> {
    let file = File::open(path).ok()?;
    let dec = Decoder::try_from(BufReader::new(file)).ok()?;
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

    #[test]
    fn waveform_and_duration() {
        let path = std::env::temp_dir().join("dmxcraft_test_tone.wav");
        wav(&path);
        let p = path.to_string_lossy();
        assert_eq!(duration_ms(&p), Some(1000));
        let w = read_wave(&p).unwrap();
        assert!((95..=105).contains(&w.peaks.len()), "{}", w.peaks.len());
        assert!(w.peaks[10] > 0.4);
        assert!(w.peaks[80] < 0.01);
        let _ = std::fs::remove_file(path);
    }
}
