//! Effekte, die Kanäle („Features“) direkt bewegen: CHASER FX, VALUE FX,
//! CURVE FX und MAPPINGS. Sie liefern je Beam einen Pegel 0..=1, der auf
//! jedes zugeordnete Feature zwischen dessen Unter- und Obergrenze
//! abgebildet wird.

use std::collections::HashMap;
use std::f32::consts::TAU;

use serde::{Deserialize, Serialize};

use super::hash01;
use crate::fixture::ChannelKind;
use crate::plan;
use crate::show::{ChannelRef, FixtureId, Show};

/// Ein vom Effekt bewegter Kanaltyp mit Wertebereich.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feature {
    pub kind: ChannelKind,
    #[serde(default)]
    pub min: u8,
    #[serde(default = "full")]
    pub max: u8,
}

fn full() -> u8 {
    255
}

impl Feature {
    pub fn new(kind: ChannelKind) -> Self {
        Self {
            kind,
            min: 0,
            max: 255,
        }
    }
}

/// Ein Gerät als Beam für Feature-Effekte (Position im Bühnenplan).
#[derive(Clone, Debug)]
pub struct FixtureBeam {
    pub fixture: FixtureId,
    pub pos: [f32; 2],
}

/// Beams der Geräte in der angegebenen Reihenfolge (je Gerät einer).
pub fn fixture_beams(show: &Show, fixtures: &[FixtureId]) -> Vec<FixtureBeam> {
    fixtures
        .iter()
        .filter_map(|&id| {
            let f = show.fixture(id)?;
            let idx = show.fixtures.iter().position(|x| x.id == id).unwrap_or(0);
            Some(FixtureBeam {
                fixture: id,
                pos: f.plan_pos.unwrap_or_else(|| plan::default_position(idx)),
            })
        })
        .collect()
}

/// Pegel auf die Features schreiben. `relative`: um den bisherigen Wert
/// (Pegel 0.5 = unverändert) statt absolut.
pub fn apply_levels(
    show: &Show,
    beams: &[FixtureBeam],
    levels: &[Option<f32>],
    features: &[Feature],
    relative: bool,
    out: &mut HashMap<ChannelRef, u8>,
) {
    for (b, level) in beams.iter().zip(levels) {
        let Some(level) = level else {
            continue;
        };
        let Some(ty) = show.fixture(b.fixture).and_then(|f| show.type_of(f)) else {
            continue;
        };
        for feat in features {
            let (lo, hi) = (f32::from(feat.min), f32::from(feat.max));
            for (i, c) in ty.channels.iter().enumerate() {
                if c.kind != feat.kind {
                    continue;
                }
                let ch = ChannelRef {
                    fixture: b.fixture,
                    channel: i as u16,
                };
                let v = if relative {
                    let base = f32::from(out.get(&ch).copied().unwrap_or(c.default));
                    base + (level - 0.5) * (hi - lo)
                } else {
                    lo + level.clamp(0.0, 1.0) * (hi - lo)
                };
                out.insert(ch, v.round().clamp(0.0, 255.0) as u8);
            }
        }
    }
}

// ---------------------------------------------------------------- CHASER FX

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChaserPreset {
    /// Ein Lauflicht.
    #[default]
    Single,
    /// Auffüllen und wieder leeren.
    Fill,
    /// Zwei Lauflichter von außen zur Mitte.
    Inward,
    /// Zwei Lauflichter von der Mitte nach außen.
    Outward,
    /// Zufällige Blitze.
    Random,
}

impl ChaserPreset {
    pub const ALL: [ChaserPreset; 5] = [
        ChaserPreset::Single,
        ChaserPreset::Fill,
        ChaserPreset::Inward,
        ChaserPreset::Outward,
        ChaserPreset::Random,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ChaserPreset::Single => "Chaser #1",
            ChaserPreset::Fill => "Chaser #2",
            ChaserPreset::Inward => "Chaser #3",
            ChaserPreset::Outward => "Chaser #4",
            ChaserPreset::Random => "Chaser Random",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ChaserFx {
    pub preset: ChaserPreset,
    pub one_direction: bool,
    pub crossfade: bool,
    /// Breite des Lichts in Beams.
    pub pixels: f32,
    /// Random: Leuchtdauer eines Blitzes in % eines Schritts.
    pub flash: f32,
    /// Random: Startwert des Zufalls.
    pub seed: f32,
    /// Random: Blitze je Durchlauf.
    pub repeats: f32,
}

impl Default for ChaserFx {
    fn default() -> Self {
        Self {
            preset: ChaserPreset::Single,
            one_direction: true,
            crossfade: false,
            pixels: 1.0,
            flash: 100.0,
            seed: 0.0,
            repeats: 1.0,
        }
    }
}

/// Pegel eines Lichtblocks aus `w` Beams an Stelle `x`, der über `span`
/// Stellen läuft (Anteil `t`). Hart: ganze Beams, weich: gleitend.
fn block(x: f32, t: f32, span: f32, w: f32, soft: bool) -> f32 {
    if soft {
        let head = t * span - (w - 1.0);
        let d = x - head - (w - 1.0) / 2.0;
        (w / 2.0 + 0.5 - d.abs()).clamp(0.0, 1.0)
    } else {
        let start = (t * span).floor().min(span - 1.0) - (w - 1.0);
        if x >= start && x < start + w {
            1.0
        } else {
            0.0
        }
    }
}

/// Lage 0..1 eines umlaufenden bzw. pendelnden Laufs.
fn travel(phase: f32, one_direction: bool) -> f32 {
    if one_direction {
        phase
    } else if phase < 0.5 {
        phase * 2.0
    } else {
        2.0 - phase * 2.0
    }
}

pub fn chaser_levels(fx: &ChaserFx, n: usize, cycles: f64) -> Vec<f32> {
    if n == 0 {
        return Vec::new();
    }
    let phase = cycles.rem_euclid(1.0) as f32;
    let w = fx.pixels.max(1.0);
    let nf = n as f32;
    let soft = fx.crossfade;
    match fx.preset {
        ChaserPreset::Single => {
            // ein voller Durchlauf, der Block schiebt sich herein und hinaus
            let span = nf + w - 1.0;
            let t = travel(phase, fx.one_direction);
            (0..n).map(|i| block(i as f32, t, span, w, soft)).collect()
        }
        ChaserPreset::Fill => {
            let t = travel(phase, fx.one_direction);
            (0..n)
                .map(|i| {
                    let x = i as f32;
                    if soft {
                        (t * nf - x).clamp(0.0, 1.0)
                    } else if x < (t * (nf + 1.0)).floor() {
                        1.0
                    } else {
                        0.0
                    }
                })
                .collect()
        }
        ChaserPreset::Inward | ChaserPreset::Outward => {
            let half = n.div_ceil(2);
            let span = half as f32 + w - 1.0;
            let mut t = travel(phase, fx.one_direction);
            if fx.preset == ChaserPreset::Outward {
                t = 1.0 - t;
            }
            (0..n)
                .map(|i| {
                    let m = i.min(n - 1 - i) as f32;
                    block(m, t, span, w, soft)
                })
                .collect()
        }
        ChaserPreset::Random => {
            let slots = fx.repeats.max(1.0) as f64;
            let slot = (cycles * slots).floor() as i64;
            let within = (cycles * slots).fract() as f32;
            let on = within < (fx.flash / 100.0).clamp(0.01, 1.0);
            let mut out = vec![0.0; n];
            if on {
                let seed = fx.seed as u64;
                let count = (w as usize).min(n);
                let mut picked = Vec::new();
                let mut k = 0u64;
                while picked.len() < count && k < 64 {
                    let i = (hash01(slot as u64 ^ (seed << 20), k) * nf) as usize % n;
                    if !picked.contains(&i) {
                        picked.push(i);
                    }
                    k += 1;
                }
                for i in picked {
                    out[i] = 1.0;
                }
            }
            out
        }
    }
}

// ---------------------------------------------------------------- CURVE FX

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurveShape {
    #[default]
    Sine,
    Sine3,
    Tangent,
    Triangle,
    Pulse,
    Square,
    Ramp,
    InverseRamp,
    Random,
    Strobe,
    Custom,
}

impl CurveShape {
    pub const ALL: [CurveShape; 11] = [
        CurveShape::Sine,
        CurveShape::Sine3,
        CurveShape::Tangent,
        CurveShape::Triangle,
        CurveShape::Pulse,
        CurveShape::Square,
        CurveShape::Ramp,
        CurveShape::InverseRamp,
        CurveShape::Random,
        CurveShape::Strobe,
        CurveShape::Custom,
    ];

    pub fn label(self) -> &'static str {
        match self {
            CurveShape::Sine => "Sinus",
            CurveShape::Sine3 => "Sinus³",
            CurveShape::Tangent => "Tangens",
            CurveShape::Triangle => "Dreieck",
            CurveShape::Pulse => "Puls",
            CurveShape::Square => "Rechteck",
            CurveShape::Ramp => "Rampe",
            CurveShape::InverseRamp => "Rampe abwärts",
            CurveShape::Random => "Zufall",
            CurveShape::Strobe => "Strobe",
            CurveShape::Custom => "Eigene",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CurveFx {
    pub shape: CurveShape,
    /// Perioden je Szenendurchlauf.
    pub repeat: f32,
    /// Ausschlag (1.0 = voller Bereich).
    pub size: f32,
    /// Verschiebung der Kurve in der Zeit (0..1 Periode).
    pub phase: f32,
    /// Verschiebung des Pegels (-1..1).
    pub offset: f32,
    /// Versatz von Beam zu Beam (0..1 Periode über alle Beams).
    pub phasing: f32,
    /// Relativ zum bisherigen Kanalwert statt absolut.
    pub relative: bool,
    /// Stützpunkte der eigenen Kurve (x, y in 0..1).
    pub custom: Vec<[f32; 2]>,
}

impl Default for CurveFx {
    fn default() -> Self {
        Self {
            shape: CurveShape::Sine,
            repeat: 1.0,
            size: 1.0,
            phase: 0.0,
            offset: 0.0,
            phasing: 0.0,
            relative: false,
            custom: vec![[0.0, 0.0], [0.25, 1.0], [0.5, 0.2], [1.0, 0.0]],
        }
    }
}

impl CurveFx {
    /// Rohwert der Kurve an Stelle `x` (Periode 0..1), 0..1.
    pub fn shape_value(&self, x: f32, period: i64) -> f32 {
        let x = x.rem_euclid(1.0);
        match self.shape {
            CurveShape::Sine => 0.5 + 0.5 * (TAU * x).sin(),
            CurveShape::Sine3 => 0.5 + 0.5 * (TAU * x).sin().powi(3),
            CurveShape::Tangent => {
                // steiler Anstieg in der Mitte, begrenzt
                let v = (std::f32::consts::PI * (x - 0.5) * 0.9).tan() / 6.3;
                (0.5 + v).clamp(0.0, 1.0)
            }
            CurveShape::Triangle => 1.0 - (2.0 * x - 1.0).abs(),
            CurveShape::Pulse => {
                if x < 0.2 {
                    1.0
                } else {
                    0.0
                }
            }
            CurveShape::Square => {
                if x < 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            CurveShape::Ramp => x,
            CurveShape::InverseRamp => 1.0 - x,
            CurveShape::Random => hash01(period as u64, 99),
            CurveShape::Strobe => {
                if x < 0.05 {
                    1.0
                } else {
                    0.0
                }
            }
            CurveShape::Custom => custom_value(&self.custom, x),
        }
    }

    /// Pegel nach Größe und Offset.
    pub fn level(&self, x: f32, period: i64) -> f32 {
        let c = self.shape_value(x, period);
        ((c - 0.5) * self.size + 0.5 + self.offset / 2.0).clamp(0.0, 1.0)
    }
}

fn custom_value(points: &[[f32; 2]], x: f32) -> f32 {
    if points.is_empty() {
        return 0.0;
    }
    let mut pts = points.to_vec();
    pts.sort_by(|a, b| a[0].total_cmp(&b[0]));
    if x <= pts[0][0] {
        return pts[0][1];
    }
    for w in pts.windows(2) {
        let (a, b) = (w[0], w[1]);
        if x <= b[0] {
            let t = if b[0] - a[0] < 1e-5 {
                0.0
            } else {
                (x - a[0]) / (b[0] - a[0])
            };
            return a[1] + (b[1] - a[1]) * t;
        }
    }
    pts[pts.len() - 1][1]
}

pub fn curve_levels(fx: &CurveFx, n: usize, cycles: f64) -> Vec<f32> {
    let n_f = n.max(1) as f64;
    (0..n)
        .map(|i| {
            let x = cycles * f64::from(fx.repeat.max(0.0)) + f64::from(fx.phase)
                - f64::from(fx.phasing) * i as f64 / n_f;
            fx.level(x.rem_euclid(1.0) as f32, x.floor() as i64)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_chaser_lights_one_beam() {
        let fx = ChaserFx::default();
        let l = chaser_levels(&fx, 5, 0.5);
        assert_eq!(l.iter().filter(|v| **v > 0.5).count(), 1);
    }

    #[test]
    fn fill_ends_full() {
        let fx = ChaserFx {
            preset: ChaserPreset::Fill,
            ..Default::default()
        };
        assert!(chaser_levels(&fx, 4, 0.999).iter().all(|v| *v > 0.5));
        assert!(chaser_levels(&fx, 4, 0.0).iter().all(|v| *v < 0.5));
    }

    #[test]
    fn inward_is_symmetric() {
        let fx = ChaserFx {
            preset: ChaserPreset::Inward,
            ..Default::default()
        };
        let l = chaser_levels(&fx, 6, 0.3);
        for i in 0..3 {
            assert_eq!(l[i], l[5 - i]);
        }
    }

    #[test]
    fn curve_phasing_shifts_beams() {
        let fx = CurveFx {
            phasing: 1.0,
            ..Default::default()
        };
        let l = curve_levels(&fx, 2, 0.25);
        assert!((l[0] - 1.0).abs() < 1e-3);
        assert!(l[1] < 0.01);
    }
}
