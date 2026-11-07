//! Effekte von FX-Szenen. Derzeit: COLOR FX – Farbverläufe über die
//! ausgewählten Geräte („Beams“), berechnet aus einer Farbpalette und der
//! Effektzeit. Die meisten Vorlagen laufen entlang der **Beam-Reihenfolge**
//! (= Reihenfolge, in der die Geräte ausgewählt wurden); Plasma und Perlin
//! nutzen die Lage im Bühnenplan.
//!
//! Die Zeit wird in **Durchläufen** gemessen (`cycles`, 1.0 = eine
//! Szenendauer); so wirken Dauer, Speed und Phase der Szene wie gewohnt.

use std::collections::HashMap;
use std::f32::consts::TAU;

use serde::{Deserialize, Serialize};

use crate::fixture::ChannelKind;
use crate::plan;
use crate::show::{ChannelRef, FixtureId, Show};

/// Vorlage eines COLOR FX.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColorPreset {
    #[default]
    Rainbow,
    Burst,
    Plasma,
    KnightRider,
    Sweep,
    Sparkle,
    RandomFill,
    Perlin,
}

impl ColorPreset {
    pub const ALL: [ColorPreset; 8] = [
        ColorPreset::Rainbow,
        ColorPreset::Burst,
        ColorPreset::Plasma,
        ColorPreset::KnightRider,
        ColorPreset::Sweep,
        ColorPreset::Sparkle,
        ColorPreset::RandomFill,
        ColorPreset::Perlin,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ColorPreset::Rainbow => "Rainbow",
            ColorPreset::Burst => "Burst",
            ColorPreset::Plasma => "Plasma",
            ColorPreset::KnightRider => "Knight Rider",
            ColorPreset::Sweep => "Sweep",
            ColorPreset::Sparkle => "Sparkle",
            ColorPreset::RandomFill => "Random Fill",
            ColorPreset::Perlin => "Perlin",
        }
    }

    /// Passende Startpalette der Vorlage.
    pub fn default_palette(self) -> Vec<[u8; 3]> {
        match self {
            ColorPreset::KnightRider => vec![[0, 0, 0], [255, 0, 0]],
            ColorPreset::Sparkle => vec![[0, 0, 0], [255, 255, 255]],
            ColorPreset::Sweep | ColorPreset::RandomFill => {
                vec![[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 0]]
            }
            _ => vec![
                [255, 0, 0],
                [255, 160, 0],
                [255, 255, 0],
                [0, 255, 0],
                [0, 255, 255],
                [0, 0, 255],
                [180, 0, 255],
                [255, 0, 160],
            ],
        }
    }
}

/// Umformung der Beam-Positionen vor der Berechnung.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transform {
    #[default]
    None,
    /// Links und rechts gespiegelt (von der Mitte nach außen).
    VerticalSymmetry,
    /// Oben und unten gespiegelt.
    HorizontalSymmetry,
    /// Beide Achsen gespiegelt.
    Center,
    /// Laufrichtung umgekehrt.
    Reverse,
}

impl Transform {
    pub fn label(self) -> &'static str {
        match self {
            Transform::None => "Keine",
            Transform::VerticalSymmetry => "Vertikale Symmetrie",
            Transform::HorizontalSymmetry => "Horizontale Symmetrie",
            Transform::Center => "Von der Mitte",
            Transform::Reverse => "Umkehren",
        }
    }

    pub const ALL: [Transform; 5] = [
        Transform::None,
        Transform::VerticalSymmetry,
        Transform::HorizontalSymmetry,
        Transform::Center,
        Transform::Reverse,
    ];
}

/// Einstellungen eines COLOR FX. Nicht jede Vorlage nutzt jedes Feld.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorFx {
    pub preset: ColorPreset,
    pub palette: Vec<[u8; 3]>,
    pub grayscale: bool,
    pub transform: Transform,
    /// Farbenbreite (Rainbow, Burst): 0..=100.
    pub width: f32,
    /// Winkel der Laufrichtung in Grad (Rainbow, Perlin): 0..=360.
    pub angle: f32,
    /// Weichheit der Übergänge zwischen den Palettenfarben: 0..=100.
    pub blend: f32,
    /// Knight Rider: Breite des Balkens in Beams.
    pub size: f32,
    pub one_direction: bool,
    /// Knight Rider: weiche Kanten.
    pub crossfade: bool,
    /// Knight Rider: Balken wird zur Mitte hin heller.
    pub fade_outward: bool,
    /// Knight Rider: Nachleuchten in % der Beams.
    pub trail: f32,
    /// Plasma: Größe und Form je Achse, Geschwindigkeit je Achse.
    pub plasma_size: [f32; 2],
    pub plasma_param: [f32; 2],
    pub plasma_speed: [f32; 2],
    pub plasma_speed_param: [f32; 2],
    /// Sweep: jede zweite Runde in Gegenrichtung.
    pub alternate: bool,
    /// Sparkle: Funken je Durchlauf, Lebensdauer (0..=1 Durchlauf), Breite.
    pub sparkle_count: f32,
    pub sparkle_life: f32,
    pub sparkle_width: f32,
    /// Random Fill: Beams je Punkt.
    pub dot_width: f32,
    /// Perlin.
    pub octaves: f32,
    pub zoom: f32,
    pub direction: f32,
    pub tempo: f32,
    pub amplitude: f32,
}

impl Default for ColorFx {
    fn default() -> Self {
        Self::new(ColorPreset::Rainbow)
    }
}

impl ColorFx {
    pub fn new(preset: ColorPreset) -> Self {
        Self {
            preset,
            palette: preset.default_palette(),
            grayscale: false,
            transform: Transform::None,
            width: if preset == ColorPreset::Burst {
                50.0
            } else {
                0.0
            },
            angle: 0.0,
            blend: 100.0,
            size: 1.0,
            one_direction: false,
            crossfade: true,
            fade_outward: false,
            trail: 50.0,
            plasma_size: [1.0, 1.0],
            plasma_param: [2.0, 2.0],
            plasma_speed: [-1.0, 1.0],
            plasma_speed_param: [2.0, -1.0],
            alternate: false,
            sparkle_count: 5.0,
            sparkle_life: 0.0,
            sparkle_width: 1.0,
            dot_width: 1.0,
            octaves: 4.0,
            zoom: 75.0,
            direction: 2.0,
            tempo: 1.0,
            amplitude: 70.0,
        }
    }

    /// Andere Vorlage wählen: Palette und Einstellungen der Vorlage.
    pub fn set_preset(&mut self, preset: ColorPreset) {
        let grayscale = self.grayscale;
        let transform = self.transform;
        *self = Self::new(preset);
        self.grayscale = grayscale;
        self.transform = transform;
    }
}

/// Ein einzeln färbbarer Lichtpunkt: ein Gerät oder ein Segment eines
/// Mehrfach-Geräts (z. B. eine Zelle einer LED-Leiste).
#[derive(Clone, Debug)]
pub struct Beam {
    pub fixture: FixtureId,
    /// Position im Bühnenplan (0..=1).
    pub pos: [f32; 2],
    pub red: Option<u16>,
    pub green: Option<u16>,
    pub blue: Option<u16>,
    /// Ohne Farbkanäle: Helligkeit über Dimmer bzw. Weiß.
    pub intensity: Option<u16>,
}

/// Beams der Geräte in der angegebenen Reihenfolge.
pub fn beams(show: &Show, fixtures: &[FixtureId]) -> Vec<Beam> {
    let mut out = Vec::new();
    for &id in fixtures {
        let Some(f) = show.fixture(id) else {
            continue;
        };
        let Some(ty) = show.type_of(f) else {
            continue;
        };
        let idx = show.fixtures.iter().position(|x| x.id == id).unwrap_or(0);
        let pos = f.plan_pos.unwrap_or_else(|| plan::default_position(idx));
        let of = |k: ChannelKind| -> Vec<u16> {
            ty.channels
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == k)
                .map(|(i, _)| i as u16)
                .collect()
        };
        let (r, g, b) = (
            of(ChannelKind::Red),
            of(ChannelKind::Green),
            of(ChannelKind::Blue),
        );
        let n = r.len().max(g.len()).max(b.len());
        if n == 0 {
            let intensity = of(ChannelKind::Dimmer)
                .first()
                .or(of(ChannelKind::White).first())
                .copied();
            if intensity.is_some() {
                out.push(Beam {
                    fixture: id,
                    pos,
                    red: None,
                    green: None,
                    blue: None,
                    intensity,
                });
            }
            continue;
        }
        // Segmente nebeneinander entlang der Gerätedrehung
        let rot = f.plan_rotation.unwrap_or(0.0).to_radians();
        let step = 0.012 * f.plan_size.unwrap_or(1.0);
        for k in 0..n {
            let off = (k as f32 - (n as f32 - 1.0) / 2.0) * step;
            out.push(Beam {
                fixture: id,
                pos: [pos[0] + off * rot.cos(), pos[1] + off * rot.sin()],
                red: r.get(k).copied(),
                green: g.get(k).copied(),
                blue: b.get(k).copied(),
                intensity: None,
            });
        }
    }
    out
}

/// Farben aller Beams zum Zeitpunkt `cycles`.
pub fn render(fx: &ColorFx, beams: &[Beam], cycles: f64) -> Vec<[u8; 3]> {
    let n = beams.len();
    if n == 0 {
        return Vec::new();
    }
    let palette: Vec<[f32; 3]> = if fx.palette.is_empty() {
        vec![[1.0; 3]]
    } else {
        fx.palette
            .iter()
            .map(|c| c.map(|v| f32::from(v) / 255.0))
            .collect()
    };
    let pos = normalized(beams, fx.transform);
    let phase = cycles.rem_euclid(1.0) as f32;
    let round = cycles.floor() as i64;
    let blend = (fx.blend / 100.0).clamp(0.0, 1.0);
    // Lage in der Beam-Reihenfolge: zyklisch (i/n) bzw. von Anfang bis Ende
    let cyc = order_coords(n, fx.transform, true);
    let line = order_coords(n, fx.transform, false);
    let colors: Vec<[f32; 3]> = match fx.preset {
        ColorPreset::Rainbow => {
            // Farbenbreite 0: jede Palettenfarbe einmal über alle Beams
            let span = 1.0 / (1.0 + fx.width / 10.0);
            let offset = fx.angle / 360.0;
            cyc.iter()
                .map(|u| sample(&palette, u * span + offset - phase, blend))
                .collect()
        }
        ColorPreset::Burst => {
            let rings = 0.25 + fx.width / 100.0 * 1.75;
            let mid = (n as f32 - 1.0) / 2.0;
            let half = (n as f32 / 2.0).max(1.0);
            (0..n)
                .map(|i| {
                    let d = (i as f32 - mid).abs() / half;
                    sample(&palette, d * rings - phase, blend)
                })
                .collect()
        }
        ColorPreset::Plasma => {
            let t = cycles as f32;
            pos.iter()
                .map(|p| {
                    let a = (TAU * (p[0] * fx.plasma_size[0] + t * fx.plasma_speed[0])).sin();
                    let b = (TAU * (p[1] * fx.plasma_size[1] + t * fx.plasma_speed[1])).sin();
                    let c = (TAU
                        * ((p[0] * fx.plasma_param[0] + p[1] * fx.plasma_param[1]) / 2.0
                            + t * (fx.plasma_speed_param[0] + fx.plasma_speed_param[1]) / 2.0))
                        .sin();
                    sample(&palette, ((a + b + c) / 3.0 + 1.0) / 2.0, blend)
                })
                .collect()
        }
        ColorPreset::KnightRider => knight_rider(fx, &palette, &line, phase, round),
        ColorPreset::Sweep => {
            let u = &line;
            let k = palette.len() as i64;
            let reverse = fx.alternate && round.rem_euclid(2) == 1;
            let cur = palette[round.rem_euclid(k) as usize];
            let prev = palette[(round - 1).rem_euclid(k) as usize];
            u.iter()
                .map(|&u| {
                    let u = if reverse { 1.0 - u } else { u };
                    // Schritt so, dass bei Phase 1 alle gefüllt sind
                    if u * (n as f32 - 1.0) / n as f32 + 1.0 / n as f32 <= phase + 1e-6 {
                        cur
                    } else {
                        prev
                    }
                })
                .collect()
        }
        ColorPreset::Sparkle => {
            // Funken auf Beam-Nummern, in der gewählten Reihenfolge
            let base = sparkle(fx, &palette, n, cycles);
            ranks(&line).iter().map(|&r| base[r]).collect()
        }
        ColorPreset::RandomFill => {
            let u = &line;
            let k = palette.len() as i64;
            let cur = palette[round.rem_euclid(k) as usize];
            let prev = palette[(round - 1).rem_euclid(k) as usize];
            let w = fx.dot_width.max(1.0);
            let order = ranks(u);
            order
                .iter()
                .map(|&rank| {
                    let group = (rank as f32 / w).floor() as u64;
                    if hash01(group, round as u64) < phase {
                        cur
                    } else {
                        prev
                    }
                })
                .collect()
        }
        ColorPreset::Perlin => {
            let zoom = 0.5 + fx.zoom / 100.0 * 4.0;
            let dir = fx.direction.rem_euclid(8.0).round() * 45.0;
            let (s, c) = dir.to_radians().sin_cos();
            let t = cycles as f32 * fx.tempo;
            let amp = fx.amplitude / 50.0;
            pos.iter()
                .map(|p| {
                    let x = p[0] * zoom + c * t;
                    let y = p[1] * zoom + s * t;
                    let v = fbm(x, y, fx.octaves.clamp(1.0, 8.0) as u32);
                    sample(&palette, 0.5 + (v - 0.5) * amp, blend)
                })
                .collect()
        }
    };
    colors
        .into_iter()
        .map(|c| {
            let c = if fx.grayscale {
                let l = 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
                [l; 3]
            } else {
                c
            };
            c.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
        })
        .collect()
}

/// Ausgabewerte des Effekts in `out` schreiben (überschreibt vorhandene).
pub fn apply(
    show: &Show,
    fx: &ColorFx,
    fixtures: &[FixtureId],
    cycles: f64,
    out: &mut HashMap<ChannelRef, u8>,
) {
    let beams = beams(show, fixtures);
    let colors = render(fx, &beams, cycles);
    for (b, c) in beams.iter().zip(colors) {
        let ch = |channel: u16| ChannelRef {
            fixture: b.fixture,
            channel,
        };
        if let Some(i) = b.intensity {
            let l = (0.299 * f32::from(c[0]) + 0.587 * f32::from(c[1]) + 0.114 * f32::from(c[2]))
                .round() as u8;
            out.insert(ch(i), l);
            continue;
        }
        for (slot, v) in [(b.red, c[0]), (b.green, c[1]), (b.blue, c[2])] {
            if let Some(i) = slot {
                out.insert(ch(i), v);
            }
        }
    }
}

/// Positionen auf 0..=1 gestreckt (beide Achsen gemeinsam, Seitenverhältnis
/// bleibt), dann umgeformt.
fn normalized(beams: &[Beam], t: Transform) -> Vec<[f32; 2]> {
    let (mut min, mut max) = ([f32::MAX; 2], [f32::MIN; 2]);
    for b in beams {
        for a in 0..2 {
            min[a] = min[a].min(b.pos[a]);
            max[a] = max[a].max(b.pos[a]);
        }
    }
    let ext = (max[0] - min[0]).max(max[1] - min[1]).max(1e-4);
    beams
        .iter()
        .map(|b| {
            let mut p = [0.0; 2];
            for a in 0..2 {
                // kleinere Achse mittig
                let off = (ext - (max[a] - min[a])) / 2.0;
                p[a] = (b.pos[a] - min[a] + off) / ext;
            }
            match t {
                Transform::None => p,
                Transform::VerticalSymmetry => [1.0 - (2.0 * p[0] - 1.0).abs(), p[1]],
                Transform::HorizontalSymmetry => [p[0], 1.0 - (2.0 * p[1] - 1.0).abs()],
                Transform::Center => [
                    1.0 - (2.0 * p[0] - 1.0).abs(),
                    1.0 - (2.0 * p[1] - 1.0).abs(),
                ],
                Transform::Reverse => [1.0 - p[0], 1.0 - p[1]],
            }
        })
        .collect()
}

/// Lage jedes Beams in der Reihenfolge, umgeformt. `cyclic`: i/n (für
/// umlaufende Verläufe, der letzte Beam schließt an den ersten an), sonst
/// 0..=1 vom ersten bis zum letzten Beam.
fn order_coords(n: usize, t: Transform, cyclic: bool) -> Vec<f32> {
    let lin = |i: usize| {
        if cyclic {
            i as f32 / n as f32
        } else {
            i as f32 / (n.max(2) - 1) as f32
        }
    };
    // gespiegelt: gleiche Lage für i und n-1-i
    let half = n.div_ceil(2).max(1);
    let mirror = |i: usize| {
        let m = i.min(n - 1 - i);
        if cyclic {
            m as f32 / half as f32
        } else {
            m as f32 / (half.max(2) - 1) as f32
        }
    };
    (0..n)
        .map(|i| match t {
            Transform::None => lin(i),
            Transform::Reverse => lin(n - 1 - i),
            Transform::VerticalSymmetry | Transform::HorizontalSymmetry => mirror(i),
            Transform::Center => {
                let m = mirror(i);
                if cyclic {
                    (1.0 - m - 1.0 / half as f32).max(0.0)
                } else {
                    1.0 - m
                }
            }
        })
        .collect()
}

/// Rang jedes Werts (0 = kleinster); bei Gleichstand gilt die Reihenfolge.
fn ranks(v: &[f32]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|a, b| v[*a].total_cmp(&v[*b]).then(a.cmp(b)));
    let mut out = vec![0; v.len()];
    for (rank, i) in idx.into_iter().enumerate() {
        out[i] = rank;
    }
    out
}

/// Farbe an Stelle `x` (zyklisch); `blend` 0 = harte Kanten, 1 = durchgehend
/// überblendet.
fn sample(palette: &[[f32; 3]], x: f32, blend: f32) -> [f32; 3] {
    let k = palette.len();
    if k == 1 {
        return palette[0];
    }
    let x = x.rem_euclid(1.0) * k as f32;
    let i = x.floor() as usize % k;
    let f = x.fract();
    // Überblendung nur im hinteren Teil jedes Abschnitts
    let edge = 1.0 - blend;
    let t = if blend <= 0.0 || f < edge {
        0.0
    } else {
        (f - edge) / blend.max(1e-4)
    };
    mix(palette[i], palette[(i + 1) % k], t)
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

fn knight_rider(
    fx: &ColorFx,
    palette: &[[f32; 3]],
    u: &[f32],
    phase: f32,
    round: i64,
) -> Vec<[f32; 3]> {
    let n = u.len();
    let (bg, fgs) = if palette.len() >= 2 {
        (palette[0], &palette[1..])
    } else {
        ([0.0; 3], palette)
    };
    let fg = fgs[round.rem_euclid(fgs.len() as i64) as usize];
    let order = ranks(u);
    let last = (n as f32 - 1.0).max(1.0);
    // Kopf läuft hin und zurück bzw. nur in eine Richtung
    let (head, forward) = if fx.one_direction || n == 1 {
        (phase * last, true)
    } else if phase < 0.5 {
        (phase * 2.0 * last, true)
    } else {
        ((1.0 - (phase - 0.5) * 2.0) * last, false)
    };
    let half = fx.size.max(1.0) / 2.0;
    let trail = fx.trail / 100.0 * n as f32;
    order
        .iter()
        .map(|&r| {
            let x = r as f32;
            let d = x - head;
            // Balken
            let mut level = if d.abs() <= half - 0.5 {
                1.0
            } else if fx.crossfade {
                (1.0 - (d.abs() - (half - 0.5))).clamp(0.0, 1.0)
            } else {
                0.0
            };
            if fx.fade_outward && level > 0.0 {
                level *= 1.0 - (d.abs() / (half + 0.5)).min(1.0) * 0.7;
            }
            // Nachleuchten hinter dem Kopf
            let behind = if forward { -d } else { d };
            if trail > 0.0 && behind > half - 0.5 {
                let t = 1.0 - (behind - (half - 0.5)) / trail;
                level = level.max(t.clamp(0.0, 1.0) * 0.8);
            }
            mix(bg, fg, level)
        })
        .collect()
}

fn sparkle(fx: &ColorFx, palette: &[[f32; 3]], n: usize, cycles: f64) -> Vec<[f32; 3]> {
    let (bg, fgs) = if palette.len() >= 2 {
        (palette[0], &palette[1..])
    } else {
        ([0.0; 3], palette)
    };
    let count = fx.sparkle_count.max(1.0) as f64;
    let life = (0.15 + f64::from(fx.sparkle_life) * 0.85) / count * 4.0;
    let width = fx.sparkle_width.max(1.0) as usize;
    let mut out = vec![bg; n];
    let slot_now = (cycles * count).floor() as i64;
    let back = (life * count).ceil() as i64 + 1;
    for slot in (slot_now - back)..=slot_now {
        let start = slot as f64 / count;
        let age = (cycles - start) / life;
        if !(0.0..1.0).contains(&age) {
            continue;
        }
        let level = (1.0 - age) as f32;
        let seed = slot as u64;
        let at = (hash01(seed, 7) * n as f32) as usize % n;
        let color = fgs[(hash01(seed, 13) * fgs.len() as f32) as usize % fgs.len()];
        for k in 0..width {
            let i = (at + k) % n;
            out[i] = mix(out[i], color, level);
        }
    }
    out
}

/// Pseudozufall 0..1 aus zwei Werten (fest, damit sich der Effekt
/// wiederholbar verhält).
fn hash01(a: u64, b: u64) -> f32 {
    let mut x = a.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ b.wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    x ^= x >> 33;
    x = x.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    x ^= x >> 33;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

/// Wertrauschen mit glatter Interpolation.
fn noise(x: f32, y: f32) -> f32 {
    let (xi, yi) = (x.floor(), y.floor());
    let (xf, yf) = (x - xi, y - yi);
    let h = |dx: f32, dy: f32| hash01((xi + dx) as i64 as u64, ((yi + dy) as i64 as u64) << 1);
    let s = |t: f32| t * t * (3.0 - 2.0 * t);
    let (sx, sy) = (s(xf), s(yf));
    let a = h(0.0, 0.0) + (h(1.0, 0.0) - h(0.0, 0.0)) * sx;
    let b = h(0.0, 1.0) + (h(1.0, 1.0) - h(0.0, 1.0)) * sx;
    a + (b - a) * sy
}

/// Mehrere Oktaven Rauschen, Ergebnis 0..1.
fn fbm(x: f32, y: f32, octaves: u32) -> f32 {
    let (mut sum, mut amp, mut freq, mut norm) = (0.0, 1.0, 1.0, 0.0);
    for _ in 0..octaves {
        sum += noise(x * freq, y * freq) * amp;
        norm += amp;
        amp *= 0.5;
        freq *= 2.0;
    }
    sum / norm
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(n: usize) -> Vec<Beam> {
        (0..n)
            .map(|i| Beam {
                fixture: FixtureId(i as u32 + 1),
                pos: [i as f32 / 10.0, 0.5],
                red: Some(0),
                green: Some(1),
                blue: Some(2),
                intensity: None,
            })
            .collect()
    }

    #[test]
    fn rainbow_moves_over_time() {
        let fx = ColorFx::new(ColorPreset::Rainbow);
        let b = line(8);
        let a = render(&fx, &b, 0.0);
        let c = render(&fx, &b, 0.25);
        assert_eq!(a.len(), 8);
        assert_ne!(a, c);
        // ein voller Durchlauf ergibt wieder dasselbe Bild
        assert_eq!(a, render(&fx, &b, 1.0));
    }

    #[test]
    fn rainbow_follows_beam_order() {
        let mut fx = ColorFx::new(ColorPreset::Rainbow);
        fx.palette = (0..9).map(|i| [i * 20, 0, 0]).collect();
        // Lage im Plan egal: Reihenfolge zählt
        let mut b = line(9);
        b.reverse();
        let now = render(&fx, &b, 0.0);
        for (i, c) in now.iter().enumerate() {
            assert_eq!(*c, fx.palette[i], "Beam {i}");
        }
        // einen Beam weiter: Farbe wandert von Beam i zu Beam i+1
        let next = render(&fx, &b, 1.0 / 9.0);
        for i in 1..9 {
            assert_eq!(next[i], now[i - 1]);
        }
    }

    #[test]
    fn hard_edges_use_palette_colors_only() {
        let mut fx = ColorFx::new(ColorPreset::Rainbow);
        fx.palette = vec![[255, 0, 0], [0, 0, 255]];
        fx.blend = 0.0;
        for c in render(&fx, &line(10), 0.3) {
            assert!(c == [255, 0, 0] || c == [0, 0, 255], "{c:?}");
        }
    }

    #[test]
    fn grayscale_has_equal_channels() {
        let mut fx = ColorFx::new(ColorPreset::Plasma);
        fx.grayscale = true;
        for c in render(&fx, &line(6), 0.4) {
            assert!(c[0] == c[1] && c[1] == c[2]);
        }
    }

    #[test]
    fn knight_rider_lights_one_bar() {
        let mut fx = ColorFx::new(ColorPreset::KnightRider);
        fx.trail = 0.0;
        fx.crossfade = false;
        let c = render(&fx, &line(9), 0.0);
        assert_eq!(c[0], [255, 0, 0]);
        assert!(c[1..].iter().all(|c| *c == [0, 0, 0]));
    }

    #[test]
    fn sweep_fills_with_next_color() {
        let mut fx = ColorFx::new(ColorPreset::Sweep);
        fx.palette = vec![[255, 0, 0], [0, 255, 0]];
        let c = render(&fx, &line(4), 1.5); // Runde 1: grün füllt über rot
        assert_eq!(c[0], [0, 255, 0]);
        assert_eq!(c[3], [255, 0, 0]);
    }

    #[test]
    fn all_presets_render_every_beam() {
        for p in ColorPreset::ALL {
            let fx = ColorFx::new(p);
            assert_eq!(render(&fx, &line(5), 2.7).len(), 5, "{p:?}");
        }
    }
}
