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
            MoveShape::Circle => "Circle",
            MoveShape::Curve => "Curve",
            MoveShape::Polygon => "Polygon",
            MoveShape::Line => "Line",
            MoveShape::Points => "Points",
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
    /// Rundung je Stützpunkt (0 = spitze Ecke, 1 = ganz rund). Fehlende
    /// Einträge zählen als 0. Gilt für Vieleck, Linie und Punkte.
    pub smooth: Vec<f32>,
    /// Punkte: von Punkt zu Punkt springen statt die Form abzufahren.
    /// Ältere Shows (ohne das Feld) springen wie bisher.
    #[serde(default = "jump_default")]
    pub jump: bool,
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

fn jump_default() -> bool {
    true
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
            smooth: Vec::new(),
            jump: false,
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
        self.smooth.clear();
    }

    /// Rundung des Stützpunkts `i` (0..=1).
    pub fn smooth_at(&self, i: usize) -> f32 {
        self.smooth.get(i).copied().unwrap_or(0.0).clamp(0.0, 1.0)
    }

    pub fn set_smooth(&mut self, i: usize, v: f32) {
        if i >= self.points.len() {
            return;
        }
        if self.smooth.len() < self.points.len() {
            self.smooth.resize(self.points.len(), 0.0);
        }
        self.smooth[i] = v.clamp(0.0, 1.0);
    }

    /// Ist die Bahn geschlossen (letzter Punkt zurück zum ersten)?
    pub fn closed(&self) -> bool {
        matches!(
            self.shape,
            MoveShape::Polygon | MoveShape::Points | MoveShape::Curve
        )
    }

    /// Lassen sich Stützpunkte hinzufügen/löschen/runden?
    pub fn editable(&self) -> bool {
        self.shape != MoveShape::Circle
    }

    /// Mindestanzahl Stützpunkte der Form.
    pub fn min_points(&self) -> usize {
        match self.shape {
            MoveShape::Polygon | MoveShape::Curve => 3,
            _ => 2,
        }
    }

    /// Neuen Stützpunkt an der Stelle `local` einfügen – auf der Kante, die
    /// ihm am nächsten liegt. Gibt den Index des neuen Punkts zurück.
    pub fn insert_point(&mut self, local: [f32; 2]) -> usize {
        let q = [local[0].clamp(-1.0, 1.0), local[1].clamp(-1.0, 1.0)];
        let n = self.points.len();
        if self.smooth.len() < n {
            self.smooth.resize(n, 0.0);
        }
        if n < 2 {
            self.points.push(q);
            self.smooth.push(0.0);
            return self.points.len() - 1;
        }
        let edges = if self.closed() { n } else { n - 1 };
        let best = (0..edges)
            .min_by(|&a, &b| {
                let da = seg_dist(q, self.points[a], self.points[(a + 1) % n]);
                let db = seg_dist(q, self.points[b], self.points[(b + 1) % n]);
                da.total_cmp(&db)
            })
            .unwrap_or(n - 1);
        // offene Bahn: vor dem ersten bzw. hinter dem letzten Punkt anhängen,
        // wenn der Klick deutlich außerhalb liegt
        let at = if self.closed() {
            best + 1
        } else if best == 0 && proj(q, self.points[0], self.points[1]) < 0.0 {
            0
        } else if best == n - 2 && proj(q, self.points[n - 2], self.points[n - 1]) > 1.0 {
            n
        } else {
            best + 1
        };
        self.points.insert(at, q);
        self.smooth.insert(at, 0.0);
        at
    }

    /// Stützpunkt entfernen (nicht unter die Mindestanzahl).
    pub fn remove_point(&mut self, i: usize) -> bool {
        if i >= self.points.len() || self.points.len() <= self.min_points() {
            return false;
        }
        self.points.remove(i);
        if i < self.smooth.len() {
            self.smooth.remove(i);
        }
        true
    }

    /// Bahn mit gerundeten Ecken als dichter Linienzug (lokal). Bei
    /// geschlossenen Bahnen ist der erste Punkt am Ende wiederholt.
    pub fn path(&self) -> Vec<[f32; 2]> {
        let p = &self.points;
        let n = p.len();
        if n < 2 {
            return p.clone();
        }
        let closed = self.closed() && n > 2;
        let mut out = Vec::new();
        for i in 0..n {
            let r = self.smooth_at(i);
            let ends = !closed && (i == 0 || i == n - 1);
            if r <= 0.001 || ends {
                out.push(p[i]);
                continue;
            }
            let a = p[(i + n - 1) % n];
            let b = p[(i + 1) % n];
            let (la, lb) = (dist(p[i], a), dist(p[i], b));
            if la < 1e-5 || lb < 1e-5 {
                out.push(p[i]);
                continue;
            }
            // halbe Kante bei voller Rundung, damit Nachbarn sich treffen
            let d = r * 0.5 * la.min(lb);
            let q1 = lerp(p[i], a, d / la);
            let q2 = lerp(p[i], b, d / lb);
            for k in 0..=12 {
                let u = k as f32 / 12.0;
                out.push(quad(q1, p[i], q2, u));
            }
        }
        if closed {
            out.push(out[0]);
        }
        out
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
            MoveShape::Points if self.jump => p[((t * p.len() as f32) as usize).min(p.len() - 1)],
            MoveShape::Points | MoveShape::Polygon => along(&self.path(), false, t),
            MoveShape::Line => {
                // hin und zurück
                let u = if t < 0.5 { t * 2.0 } else { 2.0 - t * 2.0 };
                along(&self.path(), false, u)
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

fn dist(a: [f32; 2], b: [f32; 2]) -> f32 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

fn lerp(a: [f32; 2], b: [f32; 2], u: f32) -> [f32; 2] {
    [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u]
}

/// Quadratische Bézierkurve q1 → (Kontrollpunkt c) → q2.
fn quad(q1: [f32; 2], c: [f32; 2], q2: [f32; 2], u: f32) -> [f32; 2] {
    let v = 1.0 - u;
    [
        v * v * q1[0] + 2.0 * v * u * c[0] + u * u * q2[0],
        v * v * q1[1] + 2.0 * v * u * c[1] + u * u * q2[1],
    ]
}

/// Lage der Projektion von `q` auf die Gerade a→b (0 = a, 1 = b).
fn proj(q: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let l2 = (dx * dx + dy * dy).max(1e-8);
    ((q[0] - a[0]) * dx + (q[1] - a[1]) * dy) / l2
}

/// Abstand von `q` zur Strecke a–b.
fn seg_dist(q: [f32; 2], a: [f32; 2], b: [f32; 2]) -> f32 {
    dist(q, lerp(a, b, proj(q, a, b).clamp(0.0, 1.0)))
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
        let mut fx = MoveFx::new(MoveShape::Points);
        fx.jump = true;
        assert_eq!(fx.local_at(0.1), [0.0, -1.0]);
        assert_eq!(fx.local_at(0.6), [0.0, 1.0]);
    }

    #[test]
    fn old_points_effects_keep_jumping() {
        let fx: MoveFx =
            serde_json::from_str(r#"{"shape":"points","points":[[0,-1],[0,1]]}"#).unwrap();
        assert!(fx.jump);
        assert!(!MoveFx::new(MoveShape::Points).jump);
    }

    fn square() -> MoveFx {
        let mut fx = MoveFx::new(MoveShape::Points);
        fx.points = vec![[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
        fx
    }

    #[test]
    fn points_travel_the_closed_path() {
        let fx = square();
        // ein Viertel der Strecke: von Ecke 1 zu Ecke 2
        let q = fx.local_at(0.25);
        assert!(
            (q[0] - 1.0).abs() < 1e-4 && (q[1] + 1.0).abs() < 1e-4,
            "{q:?}"
        );
        let q = fx.local_at(0.125);
        assert!(q[0].abs() < 1e-4 && (q[1] + 1.0).abs() < 1e-4, "{q:?}");
    }

    #[test]
    fn smoothing_cuts_corners() {
        let mut fx = square();
        fx.set_smooth(1, 1.0);
        let path = fx.path();
        // die spitze Ecke (1, -1) kommt nicht mehr vor, die Bahn bleibt drinnen
        assert!(!path
            .iter()
            .any(|p| (p[0] - 1.0).abs() < 1e-4 && (p[1] + 1.0).abs() < 1e-4));
        let mid = quad([0.0, -1.0], [1.0, -1.0], [1.0, 0.0], 0.5);
        assert!(path.iter().any(|p| dist(*p, mid) < 1e-4));
        // Start und Ende der Bahn gleich (geschlossen)
        assert_eq!(path.first(), path.last());
    }

    #[test]
    fn insert_and_remove_points() {
        let mut fx = square();
        // Klick nahe der rechten Kante -> zwischen Punkt 1 und 2
        let i = fx.insert_point([0.95, 0.0]);
        assert_eq!(i, 2);
        assert_eq!(fx.points.len(), 5);
        assert_eq!(fx.smooth.len(), 5);
        // Klick nahe der schließenden Kante (links) -> ans Ende
        let i = fx.insert_point([-0.95, 0.2]);
        assert_eq!(i, 5);
        assert!(fx.remove_point(0));
        assert_eq!(fx.points.len(), 5);
        let mut line = MoveFx::new(MoveShape::Line);
        assert!(!line.remove_point(0));
        // offene Linie (0,-1)–(0,1): vor dem Anfang, dazwischen, hinter dem Ende
        assert_eq!(line.insert_point([0.0, -1.0]), 1);
        let mut line = MoveFx::new(MoveShape::Line);
        line.points = vec![[0.0, -0.5], [0.0, 0.5]];
        assert_eq!(line.insert_point([0.0, -0.9]), 0);
        assert_eq!(line.insert_point([0.0, 0.9]), 3);
        assert_eq!(line.insert_point([0.1, 0.0]), 2);
    }
}
