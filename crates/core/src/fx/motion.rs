//! MOVE FX: Pan/Tilt fahren eine Form ab (Kreis, Kurve, Vieleck, Linie,
//! Punkte). Die Form liegt in einem Feld 0..=1 (x = Pan, y = Tilt) und wird
//! über Mittelpunkt, Größe und Drehung platziert.

use std::collections::HashMap;
use std::f32::consts::TAU;

use serde::{Deserialize, Serialize};

use super::feature::FixtureBeam;
use crate::fixture::ChannelKind;
use crate::show::{ChannelRef, Show};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoveShape {
    #[default]
    Circle,
    Curve,
    Polygon,
    Line,
    Points,
}

impl MoveShape {
    pub const ALL: [MoveShape; 5] = [
        MoveShape::Circle,
        MoveShape::Curve,
        MoveShape::Polygon,
        MoveShape::Line,
        MoveShape::Points,
    ];

    pub fn label(self) -> &'static str {
        match self {
            MoveShape::Circle => "Kreis",
            MoveShape::Curve => "Kurve",
            MoveShape::Polygon => "Vieleck",
            MoveShape::Line => "Linie",
            MoveShape::Points => "Punkte",
        }
    }

    /// Stützpunkte einer neuen Form (lokal, -1..=1).
    pub fn default_points(self) -> Vec<[f32; 2]> {
        match self {
            MoveShape::Circle => Vec::new(),
            MoveShape::Curve => vec![[-1.0, 1.0], [-0.5, -1.0], [0.4, 1.0], [1.0, -1.0]],
            MoveShape::Polygon => vec![[0.0, -1.0], [1.0, 0.0], [0.0, 1.0], [-1.0, 0.0]],
            MoveShape::Line | MoveShape::Points => vec![[0.0, -1.0], [0.0, 1.0]],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MoveFx {
    pub shape: MoveShape,
    /// Stützpunkte im lokalen Feld (-1..=1).
    pub points: Vec<[f32; 2]>,
    /// Mittelpunkt in Pan/Tilt (0..=1).
    pub center: [f32; 2],
    /// Halbe Breite/Höhe des Rahmens (0..=0.5).
    pub size: [f32; 2],
    /// Drehung in Grad.
    pub rotation: f32,
    /// Versatz von Beam zu Beam (0..1 Durchlauf über alle Beams).
    pub phasing: f32,
    /// Jeder zweite Beam gespiegelt.
    pub symmetry: bool,
    /// Relativ zur Position aus den Schritten statt absolut.
    pub relative: bool,
}

impl Default for MoveFx {
    fn default() -> Self {
        Self::new(MoveShape::Circle)
    }
}

impl MoveFx {
    pub fn new(shape: MoveShape) -> Self {
        Self {
            shape,
            points: shape.default_points(),
            center: [0.5, 0.5],
            size: [0.25, 0.25],
            rotation: 0.0,
            phasing: 0.0,
            symmetry: false,
            relative: false,
        }
    }

    pub fn set_shape(&mut self, shape: MoveShape) {
        self.shape = shape;
        self.points = shape.default_points();
    }

    /// Lokaler Punkt (-1..=1) → Pan/Tilt-Feld (0..=1).
    pub fn to_field(&self, p: [f32; 2]) -> [f32; 2] {
        let (s, c) = self.rotation.to_radians().sin_cos();
        let (x, y) = (p[0] * self.size[0], p[1] * self.size[1]);
        [
            self.center[0] + x * c - y * s,
            self.center[1] + x * s + y * c,
        ]
    }

    /// Pan/Tilt-Feld → lokaler Punkt.
    pub fn to_local(&self, q: [f32; 2]) -> [f32; 2] {
        let (s, c) = self.rotation.to_radians().sin_cos();
        let (dx, dy) = (q[0] - self.center[0], q[1] - self.center[1]);
        let (x, y) = (dx * c + dy * s, -dx * s + dy * c);
        [x / self.size[0].max(1e-4), y / self.size[1].max(1e-4)]
    }

    /// Lokaler Punkt der Form an Stelle `t` (0..1 Durchlauf).
    pub fn local_at(&self, t: f32) -> [f32; 2] {
        let t = t.rem_euclid(1.0);
        let p = &self.points;
        match self.shape {
            MoveShape::Circle => [(TAU * t).cos(), (TAU * t).sin()],
            _ if p.is_empty() => [0.0, 0.0],
            MoveShape::Points => p[((t * p.len() as f32) as usize).min(p.len() - 1)],
            MoveShape::Polygon => along(p, true, t),
            MoveShape::Line => {
                // hin und zurück
                let u = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
                along(p, false, u)
            }
            MoveShape::Curve => {
                let n = p.len();
                let f = t * n as f32;
                let i = f.floor() as usize % n;
                let u = f.fract();
                let g = |k: isize| p[(i as isize + k).rem_euclid(n as isize) as usize];
                catmull(g(-1), g(0), g(1), g(2), u)
            }
        }
    }

    /// Pan/Tilt (0..=1) des Beams `i` von `n`.
    pub fn position(&self, i: usize, n: usize, cycles: f64) -> [f32; 2] {
        let t = cycles as f32 - self.phasing * i as f32 / n.max(1) as f32;
        let mut l = self.local_at(t);
        if self.symmetry && i % 2 == 1 {
            l[0] = -l[0];
        }
        let q = self.to_field(l);
        [q[0].clamp(0.0, 1.0), q[1].clamp(0.0, 1.0)]
    }
}

/// Punkt auf dem Linienzug bei Anteil `t` der Gesamtlänge.
fn along(p: &[[f32; 2]], closed: bool, t: f32) -> [f32; 2] {
    let mut segs: Vec<([f32; 2], [f32; 2])> = p.windows(2).map(|w| (w[0], w[1])).collect();
    if closed && p.len() > 2 {
        segs.push((p[p.len() - 1], p[0]));
    }
    if segs.is_empty() {
        return p[0];
    }
    let len = |a: [f32; 2], b: [f32; 2]| ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt();
    let total: f32 = segs.iter().map(|(a, b)| len(*a, *b)).sum::<f32>().max(1e-5);
    let mut d = t.clamp(0.0, 1.0) * total;
    for (a, b) in &segs {
        let l = len(*a, *b);
        if d <= l || l < 1e-6 {
            let u = if l < 1e-6 { 0.0 } else { d / l };
            return [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u];
        }
        d -= l;
    }
    segs[segs.len() - 1].1
}

fn catmull(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], p3: [f32; 2], t: f32) -> [f32; 2] {
    let t2 = t * t;
    let t3 = t2 * t;
    let f = |a: f32, b: f32, c: f32, d: f32| {
        0.5 * (2.0 * b
            + (-a + c) * t
            + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
            + (-a + 3.0 * b - 3.0 * c + d) * t3)
    };
    [f(p0[0], p1[0], p2[0], p3[0]), f(p0[1], p1[1], p2[1], p3[1])]
}

/// Pan/Tilt (samt Feinkanälen) der Beams schreiben.
pub fn apply(
    show: &Show,
    fx: &MoveFx,
    beams: &[FixtureBeam],
    cycles: f64,
    out: &mut HashMap<ChannelRef, u8>,
) {
    let n = beams.len();
    for (i, b) in beams.iter().enumerate() {
        let Some(ty) = show.fixture(b.fixture).and_then(|f| show.type_of(f)) else {
            continue;
        };
        let q = fx.position(i, n, cycles);
        let find = |k: ChannelKind| {
            ty.channels
                .iter()
                .position(|c| c.kind == k)
                .map(|c| ChannelRef {
                    fixture: b.fixture,
                    channel: c as u16,
                })
        };
        for (axis, coarse, fine) in [
            (0, ChannelKind::Pan, ChannelKind::PanFine),
            (1, ChannelKind::Tilt, ChannelKind::TiltFine),
        ] {
            let Some(ch) = find(coarse) else {
                continue;
            };
            let mut v = q[axis];
            if fx.relative {
                let base = f32::from(out.get(&ch).copied().unwrap_or(128)) / 255.0;
                v = (base + v - fx.center[axis]).clamp(0.0, 1.0);
            }
            let wide = (v * 65535.0).round() as u32;
            out.insert(ch, (wide >> 8) as u8);
            if let Some(f) = find(fine) {
                out.insert(f, (wide & 0xFF) as u8);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_goes_round() {
        let fx = MoveFx::default();
        let a = fx.position(0, 1, 0.0);
        let b = fx.position(0, 1, 0.5);
        assert!((a[0] - 0.75).abs() < 1e-4 && (b[0] - 0.25).abs() < 1e-4);
    }

    #[test]
    fn local_roundtrip() {
        let fx = MoveFx {
            rotation: 30.0,
            size: [0.2, 0.1],
            ..Default::default()
        };
        let p = [0.3, -0.7];
        let back = fx.to_local(fx.to_field(p));
        assert!((back[0] - p[0]).abs() < 1e-4 && (back[1] - p[1]).abs() < 1e-4);
    }

    #[test]
    fn points_jump() {
        let fx = MoveFx::new(MoveShape::Points);
        assert_eq!(fx.local_at(0.1), [0.0, -1.0]);
        assert_eq!(fx.local_at(0.6), [0.0, 1.0]);
    }
}
