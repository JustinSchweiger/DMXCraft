//! Analyse von Audiodateien für die SUPER SCENES: Frequenzbänder (Low, Mid,
//! High), Tempo, Beat-Raster mit Downbeats und Energieverlauf.
//!
//! Ohne I/O: die Oberfläche dekodiert die Datei und schiebt die Samples
//! (Mono) in einen [`Analyzer`]. Die Analyse nimmt ein festes Tempo über das
//! ganze Stück an (typisch für DJ-Musik); Tempo und Downbeat lassen sich je
//! Block korrigieren.

use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

/// Länge eines Analyse-Rahmens in ms.
pub const FRAME_MS: f64 = 10.0;
/// Rahmen je Energie-Wert (100 ms).
pub const ENERGY_FRAMES: usize = 10;
/// Version der Analyse (Zwischenspeicher wird bei Änderung neu erzeugt).
pub const VERSION: u32 = 1;

/// Grenzfrequenzen der Bänder in Hz.
const LOW_HZ: f64 = 150.0;
const HIGH_HZ: f64 = 2500.0;
/// Tempo-Suchbereich in BPM.
const BPM_MIN: f64 = 70.0;
const BPM_MAX: f64 = 180.0;

/// Beat-Raster einer Audiodatei: Tempo und Lage eines Downbeats (Schlag 1
/// eines Takts) in ms ab Dateianfang, normiert auf den ersten Takt.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioGrid {
    pub bpm: f32,
    pub downbeat_ms: f64,
}

impl AudioGrid {
    pub fn beat_ms(&self) -> f64 {
        60_000.0 / f64::from(self.bpm.clamp(20.0, 400.0))
    }

    pub fn bar_ms(&self) -> f64 {
        self.beat_ms() * 4.0
    }

    /// Downbeat in den ersten Takt (0..Taktlänge) legen.
    fn normalized(mut self) -> Self {
        self.downbeat_ms = self.downbeat_ms.rem_euclid(self.bar_ms());
        self
    }

    /// Schläge im Bereich `from..to` (ms in der Datei): (Zeit, Downbeat?).
    pub fn beats(&self, from: f64, to: f64) -> impl Iterator<Item = (f64, bool)> + '_ {
        let beat = self.beat_ms();
        let first = ((from - self.downbeat_ms) / beat).ceil() as i64;
        (first..)
            .map(move |k| (self.downbeat_ms + k as f64 * beat, k.rem_euclid(4) == 0))
            .take_while(move |(t, _)| *t < to)
    }

    /// Tempo verdoppeln (Downbeat bleibt).
    pub fn doubled(self) -> Self {
        Self {
            bpm: (self.bpm * 2.0).min(400.0),
            ..self
        }
        .normalized()
    }

    /// Tempo halbieren (Downbeat bleibt).
    pub fn halved(self) -> Self {
        Self {
            bpm: (self.bpm / 2.0).max(20.0),
            ..self
        }
        .normalized()
    }

    /// Downbeat um `beats` Schläge verschieben.
    pub fn shifted(self, beats: i32) -> Self {
        Self {
            downbeat_ms: self.downbeat_ms + f64::from(beats) * self.beat_ms(),
            ..self
        }
        .normalized()
    }
}

/// Ergebnis der Analyse einer Datei.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioAnalysis {
    pub version: u32,
    pub duration_ms: u64,
    /// Höchster Ausschlag je Rahmen (0..=255).
    pub peaks: Vec<u8>,
    /// Pegel der Bänder je Rahmen (0..=255, je Band auf das Stück normiert).
    pub low: Vec<u8>,
    pub mid: Vec<u8>,
    pub high: Vec<u8>,
    /// Geglätteter Gesamtpegel je 100 ms (0..=255).
    pub energy: Vec<u8>,
    /// Erkanntes Beat-Raster (`None` ohne erkennbaren Rhythmus).
    pub grid: Option<AudioGrid>,
}

impl AudioAnalysis {
    /// Werte (0..=1) eines Bands im Bereich `from..to` (ms): Maximum.
    pub fn band_max(band: &[u8], from: f64, to: f64) -> f32 {
        let i = (from / FRAME_MS).max(0.0) as usize;
        let j = ((to / FRAME_MS).ceil() as usize).max(i + 1).min(band.len());
        let max = band
            .get(i..j)
            .map_or(0, |s| s.iter().copied().max().unwrap_or(0));
        f32::from(max) / 255.0
    }
}

/// Biquad-Filter (RBJ-Kochbuch).
#[derive(Clone, Copy)]
struct Biquad {
    b: [f64; 3],
    a: [f64; 2],
    x: [f64; 2],
    y: [f64; 2],
}

enum FilterKind {
    Low,
    Band,
    High,
}

impl Biquad {
    fn new(kind: FilterKind, hz: f64, q: f64, rate: f64) -> Self {
        let w = TAU * (hz / rate).min(0.49);
        let (s, c) = w.sin_cos();
        let alpha = s / (2.0 * q);
        let (b, a0, a) = match kind {
            FilterKind::Low => (
                [(1.0 - c) / 2.0, 1.0 - c, (1.0 - c) / 2.0],
                1.0 + alpha,
                [-2.0 * c, 1.0 - alpha],
            ),
            FilterKind::High => (
                [(1.0 + c) / 2.0, -(1.0 + c), (1.0 + c) / 2.0],
                1.0 + alpha,
                [-2.0 * c, 1.0 - alpha],
            ),
            FilterKind::Band => ([alpha, 0.0, -alpha], 1.0 + alpha, [-2.0 * c, 1.0 - alpha]),
        };
        Self {
            b: b.map(|v| v / a0),
            a: a.map(|v| v / a0),
            x: [0.0; 2],
            y: [0.0; 2],
        }
    }

    fn run(&mut self, x: f64) -> f64 {
        let y = self.b[0] * x + self.b[1] * self.x[0] + self.b[2] * self.x[1]
            - self.a[0] * self.y[0]
            - self.a[1] * self.y[1];
        self.x = [x, self.x[0]];
        self.y = [y, self.y[0]];
        y
    }
}

/// Nimmt Samples (Mono) entgegen und sammelt Rahmenwerte.
pub struct Analyzer {
    rate: f64,
    /// Samples je Rahmen (nicht ganzzahlig).
    frame_len: f64,
    filters: [[Biquad; 2]; 3],
    sums: [f64; 4],
    peak: f32,
    n: usize,
    samples: u64,
    frames: [Vec<f32>; 4],
    peaks: Vec<f32>,
}

impl Analyzer {
    pub fn new(sample_rate: u32) -> Self {
        let rate = f64::from(sample_rate.max(1000));
        let lp = || Biquad::new(FilterKind::Low, LOW_HZ, 0.707, rate);
        let hp = || Biquad::new(FilterKind::High, HIGH_HZ, 0.707, rate);
        let center = (LOW_HZ * HIGH_HZ).sqrt();
        let bp = || Biquad::new(FilterKind::Band, center, 0.5, rate);
        Self {
            rate,
            frame_len: rate * FRAME_MS / 1000.0,
            // je zwei Stufen für steilere Flanken
            filters: [[lp(), lp()], [bp(), bp()], [hp(), hp()]],
            sums: [0.0; 4],
            peak: 0.0,
            n: 0,
            samples: 0,
            frames: Default::default(),
            peaks: Vec::new(),
        }
    }

    pub fn push(&mut self, s: f32) {
        let x = f64::from(s);
        for (b, f) in self.filters.iter_mut().enumerate() {
            let y0 = f[0].run(x);
            let y = f[1].run(y0);
            self.sums[b] += y * y;
        }
        self.sums[3] += x * x;
        self.peak = self.peak.max(s.abs());
        self.n += 1;
        self.samples += 1;
        // Rahmengrenzen exakt alle 10 ms (auch bei z. B. 44,1 kHz)
        let frames = self.peaks.len() as f64 + 1.0;
        if self.samples as f64 >= frames * self.frame_len {
            self.flush();
        }
    }

    fn flush(&mut self) {
        if self.n == 0 {
            return;
        }
        for (b, v) in self.frames.iter_mut().enumerate() {
            v.push((self.sums[b] / self.n as f64).sqrt() as f32);
        }
        self.peaks.push(self.peak.min(1.0));
        self.sums = [0.0; 4];
        self.peak = 0.0;
        self.n = 0;
    }

    pub fn finish(mut self) -> AudioAnalysis {
        self.flush();
        let [low, mid, high, all] = std::mem::take(&mut self.frames);
        let duration_ms = (self.samples as f64 * 1000.0 / self.rate) as u64;
        let norm = |v: &[f32]| -> Vec<f32> {
            let top = percentile(v, 0.995).max(1e-6);
            v.iter().map(|x| (x / top).min(1.0)).collect()
        };
        let (low, mid, high) = (norm(&low), norm(&mid), norm(&high));
        let grid = detect_grid(&low, &mid, &high);
        let energy = energy_curve(&norm(&all));
        let q = |v: &[f32]| v.iter().map(|x| (x * 255.0).round() as u8).collect();
        AudioAnalysis {
            version: VERSION,
            duration_ms,
            peaks: q(&self.peaks),
            low: q(&low),
            mid: q(&mid),
            high: q(&high),
            energy: q(&energy),
            grid,
        }
    }
}

/// Ganze Sample-Folge auf einmal analysieren.
pub fn analyze(samples: &[f32], sample_rate: u32) -> AudioAnalysis {
    let mut a = Analyzer::new(sample_rate);
    for s in samples {
        a.push(*s);
    }
    a.finish()
}

fn percentile(v: &[f32], p: f64) -> f32 {
    if v.is_empty() {
        return 0.0;
    }
    let mut s = v.to_vec();
    s.sort_by(f32::total_cmp);
    s[((s.len() - 1) as f64 * p) as usize]
}

/// Mittelwert über ±`r` Rahmen.
fn smooth(v: &[f32], r: usize) -> Vec<f32> {
    let mut prefix = Vec::with_capacity(v.len() + 1);
    prefix.push(0.0f64);
    for x in v {
        prefix.push(prefix.last().unwrap() + f64::from(*x));
    }
    (0..v.len())
        .map(|i| {
            let a = i.saturating_sub(r);
            let b = (i + r + 1).min(v.len());
            ((prefix[b] - prefix[a]) / (b - a) as f64) as f32
        })
        .collect()
}

/// Einsatzstärke eines Bands: Anstieg des (logarithmischen) Pegels, ohne
/// den gleitenden Mittelwert.
fn onsets(v: &[f32]) -> Vec<f32> {
    let l: Vec<f32> = v.iter().map(|x| (1.0 + 100.0 * x).ln()).collect();
    let d: Vec<f32> = (0..l.len())
        .map(|i| {
            if i == 0 {
                0.0
            } else {
                (l[i] - l[i - 1]).max(0.0)
            }
        })
        .collect();
    let mean = smooth(&d, 25);
    d.iter().zip(mean).map(|(a, m)| (a - m).max(0.0)).collect()
}

/// Fourier-Koeffizient der Einsatzkurve bei `bpm` (Betrag, Winkel).
fn tempo_coef(o: &[f32], bpm: f64) -> (f64, f64) {
    let w = TAU * bpm / 60.0 * FRAME_MS / 1000.0;
    let (sw, cw) = (-w).sin_cos();
    let (mut re, mut im) = (0.0f64, 0.0f64);
    let (mut pr, mut pi) = (1.0f64, 0.0f64);
    for (n, x) in o.iter().enumerate() {
        let x = f64::from(*x);
        re += x * pr;
        im += x * pi;
        (pr, pi) = (pr * cw - pi * sw, pr * sw + pi * cw);
        if n % 1024 == 1023 {
            // Rundungsfehler des Drehzeigers ausgleichen
            let m = (pr * pr + pi * pi).sqrt();
            (pr, pi) = (pr / m, pi / m);
        }
    }
    ((re * re + im * im).sqrt(), im.atan2(re))
}

fn tempo_score(o: &[f32], bpm: f64) -> f64 {
    tempo_coef(o, bpm).0 + 0.5 * tempo_coef(o, bpm * 2.0).0
}

/// Tempo, Beat-Phase und Downbeat schätzen.
fn detect_grid(low: &[f32], mid: &[f32], high: &[f32]) -> Option<AudioGrid> {
    let n = low.len();
    // mindestens 4 s
    if n < 400 {
        return None;
    }
    let (ol, om, oh) = (onsets(low), onsets(mid), onsets(high));
    let o: Vec<f32> = (0..n)
        .map(|i| 1.0 * ol[i] + 0.6 * om[i] + 0.6 * oh[i])
        .collect();
    let total: f64 = o.iter().map(|x| f64::from(*x)).sum();
    if total < 1e-3 {
        return None;
    }
    // grob in 0,1-BPM-Schritten, dann fein
    let mut best = (BPM_MIN, 0.0);
    let mut b = BPM_MIN;
    while b <= BPM_MAX {
        let s = tempo_score(&o, b);
        if s > best.1 {
            best = (b, s);
        }
        b += 0.1;
    }
    let mut fine = best;
    let mut b = best.0 - 0.1;
    while b <= best.0 + 0.1 {
        let s = tempo_score(&o, b);
        if s > fine.1 {
            fine = (b, s);
        }
        b += 0.005;
    }
    let bpm = fine.0;
    // Rhythmus deutlich genug? (Anteil der Energie auf dem Beat)
    let mag = tempo_coef(&o, bpm).0;
    if mag / total < 0.05 {
        return None;
    }
    let period = 60_000.0 / bpm / FRAME_MS; // Rahmen je Schlag
                                            // Lage der Schläge: Kicks (Low) und Snares (Mid) liegen auf dem Beat,
                                            // Hi-Hats und Basslines oft dazwischen – daher Low/Mid stärker, High
                                            // abgezogen.
    let beat_on: Vec<f32> = (0..n).map(|i| ol[i] + 0.5 * om[i] - 0.3 * oh[i]).collect();
    let kernel = |v: &[f32], f: f64| -> f64 {
        let lo = (f - 1.5).ceil().max(0.0) as usize;
        let hi = ((f + 1.5).floor() as usize).min(n - 1);
        (lo..=hi)
            .map(|i| f64::from(v[i]) * (1.0 - (i as f64 - f).abs() / 1.5).max(0.0))
            .sum()
    };
    let phase_score = |ph: f64| -> f64 {
        let mut sum = 0.0;
        let mut f = ph;
        while f < n as f64 {
            sum += kernel(&beat_on, f);
            f += period;
        }
        sum
    };
    let mut phase = (0.0, f64::NEG_INFINITY);
    let mut ph = 0.0;
    while ph < period {
        let sc = phase_score(ph);
        if sc > phase.1 {
            phase = (ph, sc);
        }
        ph += 0.25;
    }
    let phase = phase.0; // erster Schlag (Rahmen)

    // Downbeat: Schlag mit den stärksten Einsätzen und Pegelanstiegen
    let at = |v: &[f32], f: f64| -> f32 {
        let i = f.round() as i64;
        (i - 1..=i + 1)
            .filter_map(|k| usize::try_from(k).ok().and_then(|k| v.get(k)))
            .copied()
            .fold(0.0, f32::max)
    };
    let level: Vec<f32> = (0..n).map(|i| low[i] + mid[i] + high[i]).collect();
    let mean = |a: f64, b: f64| -> f32 {
        let a = a.max(0.0) as usize;
        let b = (b.max(0.0) as usize).min(n);
        if b <= a {
            return 0.0;
        }
        level[a..b].iter().sum::<f32>() / (b - a) as f32
    };
    let mut score = [0.0f32; 4];
    let mut count = [0u32; 4];
    let mut k = 0usize;
    loop {
        let f = phase + k as f64 * period;
        if f >= n as f64 {
            break;
        }
        let j = k % 4;
        let bar = period * 4.0;
        let rise = (mean(f, f + bar) - mean(f - bar, f)).max(0.0);
        score[j] += at(&ol, f) + 0.5 * at(&oh, f) + 2.0 * rise;
        count[j] += 1;
        k += 1;
    }
    let j = (0..4)
        .max_by(|a, b| {
            let sa = score[*a] / count[*a].max(1) as f32;
            let sb = score[*b] / count[*b].max(1) as f32;
            sa.total_cmp(&sb)
        })
        .unwrap_or(0);
    let downbeat = (phase + j as f64 * period) * FRAME_MS;
    Some(
        AudioGrid {
            bpm: ((bpm * 100.0).round() / 100.0) as f32,
            downbeat_ms: downbeat,
        }
        .normalized(),
    )
}

/// Gesamtpegel geglättet (±1 s) in 100-ms-Schritten, auf das Stück normiert.
fn energy_curve(all: &[f32]) -> Vec<f32> {
    let s = smooth(all, 100);
    let v: Vec<f32> = s.chunks(ENERGY_FRAMES).map(|c| c[0]).collect();
    let top = v.iter().copied().fold(0.0, f32::max).max(1e-6);
    v.iter().map(|x| x / top).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 11_025;

    /// Synthetischer Takt: Kick (60 Hz, abklingend) auf jedem Schlag, auf
    /// Schlag 1 zusätzlich ein Becken (Rauschen), Hi-Hat auf den Offbeats.
    fn track(bpm: f64, secs: f64, start_ms: f64) -> Vec<f32> {
        let n = (secs * f64::from(RATE)) as usize;
        let beat = 60.0 / bpm;
        let mut out = vec![0.0f32; n];
        let mut seed = 1u32;
        let mut noise = move || {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1u32 << 24) as f32 * 2.0 - 1.0
        };
        let mut k = 0;
        loop {
            let t0 = start_ms / 1000.0 + k as f64 * beat;
            if t0 >= secs {
                break;
            }
            let i0 = (t0 * f64::from(RATE)) as usize;
            for i in 0..(f64::from(RATE) * 0.2) as usize {
                let t = i as f64 / f64::from(RATE);
                let env = (-t * 25.0).exp() as f32;
                let Some(o) = out.get_mut(i0 + i) else { break };
                *o += 0.8 * env * (TAU * 60.0 * t).sin() as f32;
                if k % 4 == 0 {
                    *o += 0.3 * (-t * 8.0).exp() as f32 * noise();
                }
            }
            // Hi-Hat auf dem Offbeat
            let ih = ((t0 + beat / 2.0) * f64::from(RATE)) as usize;
            for i in 0..(f64::from(RATE) * 0.03) as usize {
                if let Some(o) = out.get_mut(ih + i) {
                    *o += 0.15 * noise();
                }
            }
            k += 1;
        }
        out
    }

    #[test]
    fn finds_tempo_and_downbeat() {
        for (bpm, start) in [(128.0, 250.0), (174.0, 900.0), (95.0, 0.0)] {
            let a = analyze(&track(bpm, 30.0, start), RATE);
            let g = a.grid.expect("Raster");
            assert!(
                (f64::from(g.bpm) - bpm).abs() < 0.3,
                "{} statt {bpm}",
                g.bpm
            );
            let bar = g.bar_ms();
            let want = start.rem_euclid(bar);
            let mut d = (g.downbeat_ms - want).abs();
            d = d.min(bar - d);
            assert!(
                d < 25.0,
                "Downbeat {} statt {want} ({bpm} BPM)",
                g.downbeat_ms
            );
        }
    }

    #[test]
    fn bassline_and_intro_keep_the_grid() {
        // 16 s Intro nur mit Hi-Hats, dann Kick; dazu eine Bassline auf den
        // Offbeats (wie bei House), die das Low-Band ebenfalls anregt
        let bpm = 124.0;
        let mut s = track(bpm, 60.0, 0.0);
        let beat = 60.0 / bpm;
        let rate = f64::from(RATE);
        let intro = (16.0 * rate) as usize;
        let hats = track(bpm, 16.0, 0.0);
        for (i, v) in s.iter_mut().enumerate().take(intro) {
            // im Intro nur der hohe Anteil (grob: Differenz)
            let prev = if i > 0 { hats[i - 1] } else { 0.0 };
            *v = (hats[i] - prev) * 0.5;
        }
        for (i, v) in s.iter_mut().enumerate() {
            let t = i as f64 / rate;
            let in_beat = (t / beat).fract();
            if (0.5..0.9).contains(&in_beat) {
                *v += 0.3 * (TAU * 55.0 * t).sin() as f32;
            }
        }
        let g = analyze(&s, RATE).grid.expect("Raster");
        assert!((f64::from(g.bpm) - bpm).abs() < 0.2, "{}", g.bpm);
        let bar = g.bar_ms();
        let mut d = g.downbeat_ms.rem_euclid(bar);
        d = d.min(bar - d);
        assert!(d < 25.0, "Downbeat {}", g.downbeat_ms);
    }

    #[test]
    #[ignore]
    fn speed_six_minutes() {
        let s = track(128.0, 360.0, 0.0);
        let t = std::time::Instant::now();
        let mut a = Analyzer::new(44_100);
        // 44,1 kHz simulieren: jedes Sample 4-mal
        for x in &s {
            for _ in 0..4 {
                a.push(*x);
            }
        }
        let r = a.finish();
        eprintln!("{:?} {:?}", t.elapsed(), r.grid);
    }

    #[test]
    fn bands_separate_kick_and_hats() {
        let a = analyze(&track(120.0, 8.0, 0.0), RATE);
        assert_eq!(a.low.len(), a.high.len());
        assert!((7990..=8010).contains(&a.duration_ms), "{}", a.duration_ms);
        // Kick bei 0 ms: Low stark; Hi-Hat bei 250 ms: High stark, Low schwach
        let low_kick = AudioAnalysis::band_max(&a.low, 500.0, 560.0);
        let low_hat = AudioAnalysis::band_max(&a.low, 750.0, 770.0);
        let high_hat = AudioAnalysis::band_max(&a.high, 750.0, 770.0);
        assert!(low_kick > 0.5, "{low_kick}");
        assert!(low_hat < 0.3, "{low_hat}");
        assert!(high_hat > 0.2, "{high_hat}");
    }

    #[test]
    fn silence_has_no_grid() {
        let a = analyze(&vec![0.0; RATE as usize * 6], RATE);
        assert!(a.grid.is_none());
        assert!(a.peaks.iter().all(|p| *p == 0));
    }

    #[test]
    fn grid_helpers() {
        let g = AudioGrid {
            bpm: 120.0,
            downbeat_ms: 100.0,
        };
        let beats: Vec<_> = g.beats(0.0, 2100.0).collect();
        assert_eq!(beats.len(), 4);
        assert_eq!(beats[0], (100.0, true));
        assert!(!beats[1].1);
        assert_eq!(g.shifted(1).downbeat_ms, 600.0);
        assert_eq!(g.shifted(-1).downbeat_ms, 1600.0);
        assert_eq!(g.doubled().bpm, 240.0);
        assert_eq!(g.halved().bpm, 60.0);
    }
}
