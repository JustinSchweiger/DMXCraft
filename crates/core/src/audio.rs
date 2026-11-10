//! Analysis of audio files for the SUPER SCENES: frequency bands (low, mid,
//! high), tempo, beat grid with downbeats and energy curve.
//!
//! Without I/O: the UI decodes the file and pushes the samples (mono) into
//! an [`Analyzer`]. The analysis assumes a fixed tempo across the whole
//! piece (typical for DJ music); tempo and downbeat can be corrected per
//! block.

use std::f64::consts::TAU;

use serde::{Deserialize, Serialize};

/// Length of an analysis frame in ms.
pub const FRAME_MS: f64 = 10.0;
/// Frames per energy value (100 ms).
pub const ENERGY_FRAMES: usize = 10;
/// Version of the analysis (the cache is regenerated when it changes).
pub const VERSION: u32 = 2;

/// Cutoff frequencies of the bands in Hz.
const LOW_HZ: f64 = 150.0;
const HIGH_HZ: f64 = 2500.0;
/// Tempo search range in BPM.
const BPM_MIN: f64 = 70.0;
const BPM_MAX: f64 = 180.0;

/// Beat grid of an audio file: tempo and position of a downbeat (beat 1 of a
/// bar) in ms from the start of the file, normalized to the first bar.
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

    /// Put the downbeat into the first bar (0..bar length).
    fn normalized(mut self) -> Self {
        self.downbeat_ms = self.downbeat_ms.rem_euclid(self.bar_ms());
        self
    }

    /// Beats in the range `from..to` (ms in the file): (time, downbeat?).
    pub fn beats(&self, from: f64, to: f64) -> impl Iterator<Item = (f64, bool)> + '_ {
        let beat = self.beat_ms();
        let first = ((from - self.downbeat_ms) / beat).ceil() as i64;
        (first..)
            .map(move |k| (self.downbeat_ms + k as f64 * beat, k.rem_euclid(4) == 0))
            .take_while(move |(t, _)| *t < to)
    }

    /// Double the tempo (the downbeat stays).
    pub fn doubled(self) -> Self {
        Self {
            bpm: (self.bpm * 2.0).min(400.0),
            ..self
        }
        .normalized()
    }

    /// Halve the tempo (the downbeat stays).
    pub fn halved(self) -> Self {
        Self {
            bpm: (self.bpm / 2.0).max(20.0),
            ..self
        }
        .normalized()
    }

    /// Shift the downbeat by `beats` beats.
    pub fn shifted(self, beats: i32) -> Self {
        Self {
            downbeat_ms: self.downbeat_ms + f64::from(beats) * self.beat_ms(),
            ..self
        }
        .normalized()
    }
}

/// Result of the analysis of a file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioAnalysis {
    pub version: u32,
    pub duration_ms: u64,
    /// Highest peak per frame (0..=255).
    pub peaks: Vec<u8>,
    /// Level of the bands per frame (0..=255, each band normalized to the piece).
    pub low: Vec<u8>,
    pub mid: Vec<u8>,
    pub high: Vec<u8>,
    /// Smoothed overall level per 100 ms (0..=255).
    pub energy: Vec<u8>,
    /// Detected beat grid (`None` without a recognizable rhythm); with several
    /// pieces that of the longest.
    pub grid: Option<AudioGrid>,
    /// Pieces of the file (songs cut together with their own tempo).
    #[serde(default)]
    pub segments: Vec<AudioSegment>,
}

/// A piece within the file with its own beat grid.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioSegment {
    pub start_ms: f64,
    pub end_ms: f64,
    pub grid: Option<AudioGrid>,
}

impl AudioAnalysis {
    /// Pieces with a valid grid: correction per piece (key = start in ms),
    /// otherwise the old correction for the whole file (`single`, only with one
    /// piece), otherwise the detected grid. Older analyses without pieces count
    /// as one piece.
    pub fn effective_segments(
        &self,
        single: Option<AudioGrid>,
        overrides: &[(u64, AudioGrid)],
    ) -> Vec<AudioSegment> {
        let mut segs = if self.segments.is_empty() {
            vec![AudioSegment {
                start_ms: 0.0,
                end_ms: self.duration_ms as f64,
                grid: self.grid,
            }]
        } else {
            self.segments.clone()
        };
        let one = segs.len() == 1;
        for sg in &mut segs {
            let key = sg.start_ms.round() as u64;
            if let Some((_, g)) = overrides.iter().find(|(k, _)| *k == key) {
                sg.grid = Some(*g);
            } else if let (true, Some(g)) = (one, single) {
                sg.grid = Some(g);
            }
        }
        segs
    }

    /// Piece at position `ms` (index), if there are pieces.
    pub fn segment_at(&self, ms: f64) -> Option<usize> {
        if self.segments.is_empty() {
            return None;
        }
        let i = self.segments.partition_point(|s| s.start_ms <= ms);
        Some(i.saturating_sub(1))
    }
}

impl AudioAnalysis {
    /// Values (0..=1) of a band in the range `from..to` (ms): maximum.
    pub fn band_max(band: &[u8], from: f64, to: f64) -> f32 {
        let i = (from / FRAME_MS).max(0.0) as usize;
        let j = ((to / FRAME_MS).ceil() as usize).max(i + 1).min(band.len());
        let max = band
            .get(i..j)
            .map_or(0, |s| s.iter().copied().max().unwrap_or(0));
        f32::from(max) / 255.0
    }
}

/// Biquad filter (RBJ cookbook).
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

/// Takes samples (mono) and collects frame values.
pub struct Analyzer {
    rate: f64,
    /// Samples per frame (not an integer).
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
            // two stages each for steeper slopes
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
        // Frame boundaries exactly every 10 ms (also at e.g. 44.1 kHz)
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
        let segments = detect_segments(&low, &mid, &high);
        let grid = segments
            .iter()
            .filter(|s| s.grid.is_some())
            .max_by(|a, b| (a.end_ms - a.start_ms).total_cmp(&(b.end_ms - b.start_ms)))
            .and_then(|s| s.grid);
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
            segments,
        }
    }
}

/// Envelope of a freely filtered band (for triggers): `low_hz` ≤ 20 = low
/// pass only, `high_hz` ≥ 16 kHz = high pass only, otherwise band pass.
pub struct Envelope {
    frame_len: f64,
    stages: Vec<Biquad>,
    sum: f64,
    n: usize,
    samples: u64,
    frames: Vec<f32>,
}

impl Envelope {
    pub fn new(sample_rate: u32, low_hz: f32, high_hz: f32) -> Self {
        let rate = f64::from(sample_rate.max(1000));
        let (lo, hi) = (f64::from(low_hz), f64::from(high_hz));
        let mut stages = Vec::new();
        if lo > 20.0 {
            for _ in 0..2 {
                stages.push(Biquad::new(FilterKind::High, lo, 0.707, rate));
            }
        }
        if hi < 16_000.0 {
            for _ in 0..2 {
                stages.push(Biquad::new(FilterKind::Low, hi.max(lo + 10.0), 0.707, rate));
            }
        }
        Self {
            frame_len: rate * FRAME_MS / 1000.0,
            stages,
            sum: 0.0,
            n: 0,
            samples: 0,
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, s: f32) {
        let mut x = f64::from(s);
        for st in &mut self.stages {
            x = st.run(x);
        }
        self.sum += x * x;
        self.n += 1;
        self.samples += 1;
        if self.samples as f64 >= (self.frames.len() as f64 + 1.0) * self.frame_len {
            self.flush();
        }
    }

    fn flush(&mut self) {
        if self.n > 0 {
            self.frames.push((self.sum / self.n as f64).sqrt() as f32);
        }
        self.sum = 0.0;
        self.n = 0;
    }

    /// Level per 10 ms (0..=255, normalized to the piece).
    pub fn finish(mut self) -> Vec<u8> {
        self.flush();
        let top = percentile(&self.frames, 0.995).max(1e-6);
        self.frames
            .iter()
            .map(|x| ((x / top).min(1.0) * 255.0).round() as u8)
            .collect()
    }
}

/// Events (start, end in ms) of an envelope: on when the level reaches the
/// threshold (at the earliest `min_gap_ms` after the last start), off when it
/// falls below threshold − hysteresis; at least `hold_ms` long.
pub fn trigger_events(
    env: &[u8],
    threshold: f32,
    hysteresis: f32,
    min_gap_ms: f64,
    hold_ms: f64,
) -> Vec<(f64, f64)> {
    let on = (threshold.clamp(0.01, 1.0) * 255.0).round() as u8;
    let off =
        (((threshold - hysteresis.max(0.0)).clamp(0.0, 1.0) * 255.0).round() as u8).clamp(1, on);
    let mut out: Vec<(f64, f64)> = Vec::new();
    let mut start: Option<f64> = None;
    let mut last_start = f64::NEG_INFINITY;
    for (i, v) in env.iter().enumerate() {
        let t = i as f64 * FRAME_MS;
        match start {
            None => {
                if *v >= on && t - last_start >= min_gap_ms {
                    start = Some(t);
                    last_start = t;
                }
            }
            Some(s) => {
                if *v < off {
                    out.push((s, t.max(s + hold_ms)));
                    start = None;
                }
            }
        }
    }
    if let Some(s) = start {
        let end = env.len() as f64 * FRAME_MS;
        out.push((s, end.max(s + hold_ms)));
    }
    // The hold time must not reach into the next event
    for i in 1..out.len() {
        let next = out[i].0;
        out[i - 1].1 = out[i - 1].1.min(next - 1.0);
    }
    out
}

/// Is an event active at time `t` (ms)? (`events` sorted)
pub fn event_at(events: &[(f64, f64)], t: f64) -> Option<usize> {
    let i = events.partition_point(|e| e.0 <= t);
    (i > 0 && t < events[i - 1].1).then(|| i - 1)
}

/// Analyze the whole sample sequence at once.
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

/// Average over ±`r` frames.
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

/// Onset strength of a band: rise of the (logarithmic) level, without the
/// moving average.
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

/// Fourier coefficient of the onset curve at `bpm` (magnitude, angle).
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
            // compensate the rounding error of the rotating phasor
            let m = (pr * pr + pi * pi).sqrt();
            (pr, pi) = (pr / m, pi / m);
        }
    }
    ((re * re + im * im).sqrt(), im.atan2(re))
}

fn tempo_score(o: &[f32], bpm: f64) -> f64 {
    tempo_coef(o, bpm).0 + 0.5 * tempo_coef(o, bpm * 2.0).0
}

/// Length of the window for the local tempo and step size (frames).
const SEG_WIN: usize = 1000;
const SEG_HOP: usize = 200;
/// Shortest piece (frames).
const SEG_MIN: usize = 1200;

/// Roughly estimate the tempo of a section (0.5 BPM steps) with strength.
fn local_tempo(o: &[f32]) -> Option<f64> {
    let total: f64 = o.iter().map(|x| f64::from(*x)).sum();
    if total < 1e-3 {
        return None;
    }
    let mut best = (0.0, 0.0);
    let mut b = BPM_MIN;
    while b <= BPM_MAX {
        let sc = tempo_score(o, b);
        if sc > best.1 {
            best = (b, sc);
        }
        b += 0.5;
    }
    (tempo_coef(o, best.0).0 / total >= 0.08).then_some(best.0)
}

/// Same tempo (also double/half, ±2 %)?
fn same_tempo(a: f64, b: f64) -> bool {
    [1.0, 2.0, 0.5]
        .iter()
        .any(|k| (a * k / b - 1.0).abs() < 0.02)
}

/// Split the file into pieces: at tempo changes (boundary at the strongest
/// sound change nearby) and at pauses; each piece gets its own grid.
fn detect_segments(low: &[f32], mid: &[f32], high: &[f32]) -> Vec<AudioSegment> {
    let n = low.len();
    let whole = |a: usize, b: usize| AudioSegment {
        start_ms: a as f64 * FRAME_MS,
        end_ms: b as f64 * FRAME_MS,
        grid: detect_grid(&low[a..b], &mid[a..b], &high[a..b]).map(|g| AudioGrid {
            downbeat_ms: g.downbeat_ms + a as f64 * FRAME_MS,
            ..g
        }),
    };
    if n < SEG_MIN * 2 {
        return vec![whole(0, n)];
    }
    let (ol, om, oh) = (onsets(low), onsets(mid), onsets(high));
    let o: Vec<f32> = (0..n).map(|i| ol[i] + 0.6 * om[i] + 0.6 * oh[i]).collect();
    // local tempo per window
    let mut local: Vec<(usize, Option<f64>)> = Vec::new();
    let mut a = 0;
    while a + SEG_WIN <= n {
        local.push((a + SEG_WIN / 2, local_tempo(&o[a..a + SEG_WIN])));
        a += SEG_HOP;
    }
    // Sound change: difference of the band levels 4 s before/after
    let level: Vec<[f32; 3]> = (0..n).map(|i| [low[i], mid[i], high[i]]).collect();
    let mut prefix = vec![[0.0f64; 3]; n + 1];
    for i in 0..n {
        for b in 0..3 {
            prefix[i + 1][b] = prefix[i][b] + f64::from(level[i][b]);
        }
    }
    let mean = |a: usize, b: usize| -> [f64; 3] {
        let len = (b - a).max(1) as f64;
        [0, 1, 2].map(|k| (prefix[b][k] - prefix[a][k]) / len)
    };
    let novelty = |i: usize| -> f64 {
        let w = 400.min(i).min(n - i);
        if w < 50 {
            return 0.0;
        }
        let (x, y) = (mean(i - w, i), mean(i, i + w));
        (0..3).map(|k| (x[k] - y[k]).abs()).sum()
    };
    let mut cuts: Vec<usize> = Vec::new();
    // Tempo change: two windows in a row with a different tempo than before
    let mut cur: Option<f64> = None;
    let mut k = 0;
    while k < local.len() {
        let (center, bpm) = local[k];
        match (cur, bpm) {
            (None, Some(b)) => cur = Some(b),
            (Some(c), Some(b)) if !same_tempo(c, b) => {
                let stable = local
                    .get(k + 1)
                    .and_then(|l| l.1)
                    .is_some_and(|b2| same_tempo(b, b2));
                if stable {
                    // Boundary: strongest change between the window centers
                    let from = local[k.saturating_sub(1)].0.saturating_sub(SEG_WIN / 2);
                    let to = (center + SEG_WIN / 2).min(n - 1);
                    let cut = (from..=to)
                        .step_by(5)
                        .max_by(|x, y| novelty(*x).total_cmp(&novelty(*y)))
                        .unwrap_or(center);
                    cuts.push(cut);
                    cur = Some(b);
                }
            }
            _ => {}
        }
        k += 1;
    }
    // Pauses (≥ 0.3 s almost silent) also split
    let mut quiet = 0;
    for (i, l) in level.iter().enumerate() {
        if l[0] + l[1] + l[2] < 0.03 {
            quiet += 1;
        } else {
            if quiet >= 30 && i > quiet {
                cuts.push(i);
            }
            quiet = 0;
        }
    }
    cuts.sort_unstable();
    // avoid pieces that are too short
    let mut bounds = vec![0];
    for c in cuts {
        if c >= bounds.last().copied().unwrap_or(0) + SEG_MIN && c + SEG_MIN <= n {
            bounds.push(c);
        }
    }
    bounds.push(n);
    let mut segs: Vec<AudioSegment> = bounds.windows(2).map(|w| whole(w[0], w[1])).collect();
    // Songs usually start on a downbeat: put the boundary on the next downbeat
    // of the new piece (at most half a bar away)
    for i in 1..segs.len() {
        let Some(g) = segs[i].grid else {
            continue;
        };
        let start = segs[i].start_ms;
        let bar = g.bar_ms();
        let near = g.downbeat_ms + ((start - g.downbeat_ms) / bar).round() * bar;
        if (near - start).abs() <= bar / 2.0
            && near > segs[i - 1].start_ms + 1000.0
            && near < segs[i].end_ms - 1000.0
        {
            segs[i].start_ms = near;
            segs[i - 1].end_ms = near;
        }
    }
    segs
}

/// Estimate tempo, beat phase and downbeat.
fn detect_grid(low: &[f32], mid: &[f32], high: &[f32]) -> Option<AudioGrid> {
    let n = low.len();
    // at least 4 s
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
    // coarse in 0.1 BPM steps, then fine
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
    // Is the rhythm clear enough? (share of the energy on the beat)
    let mag = tempo_coef(&o, bpm).0;
    if mag / total < 0.05 {
        return None;
    }
    let period = 60_000.0 / bpm / FRAME_MS; // Frames per beat
                                            // Position of the beats: kicks (low) and snares (mid) are on the beat,
                                            // hi-hats and basslines often in between – hence low/mid weighted more,
                                            // high subtracted.
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
    let phase = phase.0; // first beat (frame)

    // Downbeat: the beat with the strongest onsets and level rises
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

/// Overall level smoothed (±1 s) in 100 ms steps, normalized to the piece.
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

    /// Synthetic bar: kick (60 Hz, decaying) on every beat, additionally a
    /// cymbal (noise) on beat 1, hi-hat on the offbeats.
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
            // Hi-hat on the offbeat
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
            let g = a.grid.expect("grid");
            assert!(
                (f64::from(g.bpm) - bpm).abs() < 0.3,
                "{} instead of {bpm}",
                g.bpm
            );
            let bar = g.bar_ms();
            let want = start.rem_euclid(bar);
            let mut d = (g.downbeat_ms - want).abs();
            d = d.min(bar - d);
            assert!(
                d < 25.0,
                "Downbeat {} instead of {want} ({bpm} BPM)",
                g.downbeat_ms
            );
        }
    }

    #[test]
    fn bassline_and_intro_keep_the_grid() {
        // 16 s intro with hi-hats only, then kick; plus a bassline on the offbeats
        // (as in house) that also excites the low band
        let bpm = 124.0;
        let mut s = track(bpm, 60.0, 0.0);
        let beat = 60.0 / bpm;
        let rate = f64::from(RATE);
        let intro = (16.0 * rate) as usize;
        let hats = track(bpm, 16.0, 0.0);
        for (i, v) in s.iter_mut().enumerate().take(intro) {
            // in the intro only the high part (roughly: difference)
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
        let g = analyze(&s, RATE).grid.expect("grid");
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
        // simulate 44.1 kHz: every sample 4 times
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
        // Kick at 0 ms: low strong; hi-hat at 250 ms: high strong, low weak
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
    fn trigger_events_use_threshold_hysteresis_and_gap() {
        // Level: 0, 200 (on), 150 (stays on because of hysteresis), 50 (off) …
        let env = [0u8, 200, 150, 50, 0, 210, 0, 220, 0, 0, 0, 0, 230, 0];
        let ev = trigger_events(&env, 0.6, 0.2, 0.0, 0.0);
        assert_eq!(ev[0], (10.0, 30.0));
        assert_eq!(ev.len(), 4);
        // Minimum gap 60 ms: events at 50 and 120 ms are dropped
        let ev = trigger_events(&env, 0.6, 0.2, 60.0, 0.0);
        let starts: Vec<f64> = ev.iter().map(|e| e.0).collect();
        assert_eq!(starts, vec![10.0, 70.0]);
        // Hold time extended, but ends before the next event
        let ev = trigger_events(&env, 0.6, 0.2, 0.0, 100.0);
        assert_eq!(ev[0], (10.0, 49.0));
        assert_eq!(event_at(&ev, 20.0), Some(0));
        assert_eq!(event_at(&ev, 49.5), None);
    }

    #[test]
    fn envelope_filters_band() {
        // 60 Hz and 5 kHz mixed: the low pass only sees the bass
        let n = RATE as usize * 2;
        let s: Vec<f32> = (0..n)
            .map(|i| {
                let t = i as f64 / f64::from(RATE);
                let bass = if t < 1.0 { (TAU * 60.0 * t).sin() } else { 0.0 };
                let hat = if t >= 1.0 {
                    (TAU * 5000.0 * t).sin()
                } else {
                    0.0
                };
                (0.5 * (bass + hat)) as f32
            })
            .collect();
        let run = |lo: f32, hi: f32| {
            let mut e = Envelope::new(RATE, lo, hi);
            for x in &s {
                e.push(*x);
            }
            e.finish()
        };
        let low = run(0.0, 150.0);
        let high = run(2500.0, 20_000.0);
        assert!(low[50] > 200 && low[150] < 20, "{} {}", low[50], low[150]);
        assert!(
            high[150] > 200 && high[50] < 20,
            "{} {}",
            high[150],
            high[50]
        );
    }

    #[test]
    fn mixed_file_splits_into_tracks() {
        // three songs one after another (126, 140, 100 BPM), 40 s each
        let mut s = track(126.0, 40.0, 300.0);
        s.extend(track(140.0, 40.0, 0.0));
        s.extend(track(100.0, 40.0, 150.0));
        let a = analyze(&s, RATE);
        assert_eq!(a.segments.len(), 3, "{:?}", a.segments);
        for (seg, (bpm, start)) in
            a.segments
                .iter()
                .zip([(126.0, 0.0), (140.0, 40_000.0), (100.0, 80_000.0)])
        {
            assert!(
                (seg.start_ms - start).abs() < 1500.0,
                "{} instead of {start}",
                seg.start_ms
            );
            let g = seg.grid.expect("grid");
            assert!(
                (f64::from(g.bpm) - bpm).abs() < 0.3,
                "{} instead of {bpm}",
                g.bpm
            );
        }
        // The downbeat of the second song lies on its start (40 s)
        let g = a.segments[1].grid.unwrap();
        let bar = g.bar_ms();
        let mut d = (g.downbeat_ms - 40_000.0).rem_euclid(bar);
        d = d.min(bar - d);
        assert!(d < 30.0, "Downbeat {}", g.downbeat_ms);
        assert_eq!(a.segment_at(50_000.0), Some(1));
        // Boundaries lie on the first beat of the songs
        assert!(
            (a.segments[2].start_ms - 80_150.0).abs() < 30.0,
            "{}",
            a.segments[2].start_ms
        );
    }

    #[test]
    fn one_song_stays_one_segment() {
        let a = analyze(&track(128.0, 120.0, 0.0), RATE);
        assert_eq!(a.segments.len(), 1, "{:?}", a.segments);
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
