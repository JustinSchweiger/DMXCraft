//! MAPPINGS and COLOR MAPPINGS: an image (gradient, spiral, fire …) lies as
//! a movable, scalable and rotatable rectangle over the stage plan. Every
//! beam under the rectangle gets the color or level at its position; beams
//! outside stay untouched.

use std::f32::consts::{PI, TAU};

use serde::{Deserialize, Serialize};

use super::{fbm, hash01, sample, Transform};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapPreset {
    #[default]
    Rainbow,
    Spiral,
    Burst,
    Butterfly,
    Plasma,
    Bounce,
    Fire,
    KnightRider,
    Text,
    Sweep,
    Sparkle,
    Tube,
    RandomFill,
    Rain,
    Perlin,
    Explosion,
    Starfield,
    Graph,
    Lines,
}

impl MapPreset {
    pub const ALL: [MapPreset; 19] = [
        MapPreset::Rainbow,
        MapPreset::Spiral,
        MapPreset::Burst,
        MapPreset::Butterfly,
        MapPreset::Plasma,
        MapPreset::Bounce,
        MapPreset::Fire,
        MapPreset::KnightRider,
        MapPreset::Text,
        MapPreset::Sweep,
        MapPreset::Sparkle,
        MapPreset::Tube,
        MapPreset::RandomFill,
        MapPreset::Rain,
        MapPreset::Perlin,
        MapPreset::Explosion,
        MapPreset::Starfield,
        MapPreset::Graph,
        MapPreset::Lines,
    ];

    pub fn label(self) -> &'static str {
        match self {
            MapPreset::Rainbow => "Rainbow",
            MapPreset::Spiral => "Spiral",
            MapPreset::Burst => "Burst",
            MapPreset::Butterfly => "Butterfly",
            MapPreset::Plasma => "Plasma",
            MapPreset::Bounce => "Bounce",
            MapPreset::Fire => "Fire",
            MapPreset::KnightRider => "Knight Rider",
            MapPreset::Text => "Text",
            MapPreset::Sweep => "Sweep",
            MapPreset::Sparkle => "Sparkle",
            MapPreset::Tube => "Tube",
            MapPreset::RandomFill => "Random Fill",
            MapPreset::Rain => "Rain",
            MapPreset::Perlin => "Perlin",
            MapPreset::Explosion => "Explosion",
            MapPreset::Starfield => "Starfield",
            MapPreset::Graph => "Graph",
            MapPreset::Lines => "Lines",
        }
    }

    /// Templates with objects in front of a background (palette: first color =
    /// background, the rest = objects).
    pub fn has_background(self) -> bool {
        matches!(
            self,
            MapPreset::Bounce
                | MapPreset::KnightRider
                | MapPreset::Text
                | MapPreset::Sparkle
                | MapPreset::Rain
                | MapPreset::Explosion
                | MapPreset::Starfield
                | MapPreset::Graph
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MappingFx {
    pub preset: MapPreset,
    pub palette: Vec<[u8; 3]>,
    pub grayscale: bool,
    pub transform: Transform,
    /// Rectangle in the stage plan: center, half width/height, rotation
    /// (degrees).
    pub center: [f32; 2],
    pub size: [f32; 2],
    pub rotation: f32,
    /// Rotations of the image per cycle.
    pub spin: f32,
    pub width: f32,
    pub angle: f32,
    pub blend: f32,
    pub radius: f32,
    pub swirls: f32,
    /// Size of the objects (bounce, Knight Rider, text, explosion …): 0..=100.
    pub object: f32,
    /// Count (sparks, drops, stars, lines, bars, fields).
    pub count: f32,
    pub zoom: f32,
    pub octaves: f32,
    pub amplitude: f32,
    pub text: String,
}

impl Default for MappingFx {
    fn default() -> Self {
        Self::new(MapPreset::Rainbow, true)
    }
}

impl MappingFx {
    pub fn new(preset: MapPreset, color: bool) -> Self {
        Self {
            preset,
            palette: default_palette(preset, color),
            grayscale: false,
            transform: Transform::None,
            center: [0.5, 0.5],
            size: [0.25, 0.25],
            rotation: 0.0,
            spin: 0.0,
            width: 0.0,
            angle: 0.0,
            blend: 100.0,
            radius: 30.0,
            swirls: 1.0,
            object: 30.0,
            count: 5.0,
            zoom: 50.0,
            octaves: 4.0,
            amplitude: 70.0,
            text: "DMXCRAFT".into(),
        }
    }

    /// Switch the template (rectangle, transform and gray levels stay).
    pub fn set_preset(&mut self, preset: MapPreset, color: bool) {
        let keep = (
            self.center,
            self.size,
            self.rotation,
            self.transform,
            self.grayscale,
        );
        *self = Self::new(preset, color);
        (
            self.center,
            self.size,
            self.rotation,
            self.transform,
            self.grayscale,
        ) = keep;
    }

    /// Stage plan point → local position in the rectangle (-1..=1), `None`
    /// outside.
    pub fn local(&self, p: [f32; 2]) -> Option<[f32; 2]> {
        let (s, c) = self.rotation.to_radians().sin_cos();
        let (dx, dy) = (p[0] - self.center[0], p[1] - self.center[1]);
        let (x, y) = (dx * c + dy * s, -dx * s + dy * c);
        let l = [x / self.size[0].max(1e-4), y / self.size[1].max(1e-4)];
        (l[0].abs() <= 1.0 && l[1].abs() <= 1.0).then_some(l)
    }

    /// Local point → stage plan.
    pub fn to_plan(&self, l: [f32; 2]) -> [f32; 2] {
        let (s, c) = self.rotation.to_radians().sin_cos();
        let (x, y) = (l[0] * self.size[0], l[1] * self.size[1]);
        [
            self.center[0] + x * c - y * s,
            self.center[1] + x * s + y * c,
        ]
    }

    /// Color (0..1) at the local position `l`.
    pub fn color_at(&self, l: [f32; 2], cycles: f64) -> [f32; 3] {
        let palette: Vec<[f32; 3]> = if self.palette.is_empty() {
            vec![[1.0; 3]]
        } else {
            self.palette
                .iter()
                .map(|c| c.map(|v| f32::from(v) / 255.0))
                .collect()
        };
        let c = self.pattern(&palette, transform(l, self.transform), cycles);
        if self.grayscale {
            let g = 0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2];
            [g; 3]
        } else {
            c
        }
    }

    fn pattern(&self, pal: &[[f32; 3]], l: [f32; 2], cycles: f64) -> [f32; 3] {
        let phase = cycles.rem_euclid(1.0) as f32;
        let round = cycles.floor() as i64;
        let t = cycles as f32;
        let blend = (self.blend / 100.0).clamp(0.0, 1.0);
        // Rotation of the image over time
        let spin = self.spin * t * TAU;
        let (ss, sc) = spin.sin_cos();
        let (u, v) = (l[0] * sc - l[1] * ss, l[0] * ss + l[1] * sc);
        let r = (u * u + v * v).sqrt();
        let th = v.atan2(u);
        let bg = pal[0];
        let fg = |k: i64| {
            if pal.len() > 1 {
                pal[1 + (k.rem_euclid(pal.len() as i64 - 1)) as usize]
            } else {
                [1.0; 3]
            }
        };
        let obj = (self.object / 100.0).clamp(0.02, 1.0);
        let count = self.count.max(1.0);
        let mixc = |level: f32, c: [f32; 3]| mix(bg, c, level.clamp(0.0, 1.0));
        match self.preset {
            MapPreset::Rainbow => {
                let (s, c) = self.angle.to_radians().sin_cos();
                let x = (u * c + v * s + 1.0) / 2.0;
                sample(pal, x / (1.0 + self.width / 10.0) - phase, blend)
            }
            MapPreset::Spiral => {
                let x = th / TAU * self.swirls.max(1.0) + r * self.radius / 30.0 - phase;
                sample(pal, x, blend)
            }
            MapPreset::Burst => {
                // like COLOR FX: larger color width = wider rings
                sample(pal, r / 2.0 / (1.0 + self.width / 10.0) - phase, blend)
            }
            MapPreset::Butterfly => {
                let petals = self.swirls.max(1.0).round() * 2.0;
                let x = r * (1.0 + 0.5 * (petals * th).sin()) - phase;
                sample(pal, x, blend)
            }
            MapPreset::Plasma => {
                let z = 1.0 + self.zoom / 25.0;
                let a = (z * u * PI + t * TAU).sin();
                let b = (z * v * PI - t * TAU * 0.7).sin();
                let c = (z * (u + v) * PI * 0.7 + t * TAU * 0.5).sin();
                let d = (z * r * PI * 1.3 - t * TAU).sin();
                sample(pal, ((a + b + c + d) / 4.0 + 1.0) / 2.0, blend)
            }
            MapPreset::Bounce => {
                // Balls bounce off the edges
                let mut level: f32 = 0.0;
                let mut col = fg(0);
                for k in 0..count as i64 {
                    let sx = 0.6 + hash01(k as u64, 1) * 0.9;
                    let sy = 0.5 + hash01(k as u64, 2) * 0.9;
                    let ox = hash01(k as u64, 3);
                    let oy = hash01(k as u64, 4);
                    let px = tri(t * sx + ox) * 2.0 - 1.0;
                    let py = tri(t * sy + oy) * 2.0 - 1.0;
                    let d = ((u - px).powi(2) + (v - py).powi(2)).sqrt();
                    let lv = (1.0 - d / (obj * 1.2)).clamp(0.0, 1.0);
                    if lv > level {
                        level = lv;
                        col = fg(k);
                    }
                }
                mixc(level * 1.5, col)
            }
            MapPreset::Fire => {
                let z = 1.0 + self.zoom / 25.0;
                let h = (1.0 - v) / 2.0; // 0 bottom … 1 top
                let n = fbm(u * z * 2.0, v * z * 2.0 + t * 3.0, 4);
                let x = (1.0 - h) * (0.6 + n * self.amplitude / 70.0) - 0.15;
                sample(pal, x.clamp(0.0, 0.999), blend)
            }
            MapPreset::KnightRider => {
                let (s, c) = self.angle.to_radians().sin_cos();
                let x = u * c + v * s;
                let head = tri(phase) * 2.0 - 1.0;
                let level = 1.0 - (x - head).abs() / obj;
                mixc(level * 1.5, fg(round))
            }
            MapPreset::Text => {
                text_level(&self.text, u, v, phase, obj).map_or(bg, |lv| mixc(lv, fg(0)))
            }
            MapPreset::Sweep => {
                let (s, c) = self.angle.to_radians().sin_cos();
                let x = ((u * c + v * s) + 1.0) / 2.0;
                let k = pal.len() as i64;
                if x <= phase {
                    pal[round.rem_euclid(k) as usize]
                } else {
                    pal[(round - 1).rem_euclid(k) as usize]
                }
            }
            MapPreset::Sparkle => {
                let grid = (count * 2.0).max(2.0);
                let cx = ((u + 1.0) / 2.0 * grid).floor();
                let cy = ((v + 1.0) / 2.0 * grid).floor();
                let cell = (cx as u64) * 977 + cy as u64;
                let start = hash01(cell, 5);
                let life = 0.1 + obj * 0.5;
                let age = (phase - start).rem_euclid(1.0) / life;
                let level = if age < 1.0 { 1.0 - age } else { 0.0 };
                mixc(level, fg(cell as i64 + round))
            }
            MapPreset::Tube => {
                // Tunnel: rings run towards the center
                let x = count / (r + 0.15) * 0.1 + phase;
                sample(pal, x, blend)
            }
            MapPreset::RandomFill => {
                let grid = count.max(1.0);
                let cx = ((u + 1.0) / 2.0 * grid).floor().min(grid - 1.0);
                let cy = ((v + 1.0) / 2.0 * grid).floor().min(grid - 1.0);
                let cell = (cx as u64) * 131 + cy as u64;
                let k = pal.len() as i64;
                if hash01(cell, round as u64) < phase {
                    pal[round.rem_euclid(k) as usize]
                } else {
                    pal[(round - 1).rem_euclid(k) as usize]
                }
            }
            MapPreset::Rain => {
                let cols = (count * 2.0).max(2.0);
                let col = ((u + 1.0) / 2.0 * cols).floor();
                let speed = 0.7 + hash01(col as u64, 6) * 0.8;
                let head = ((t * speed + hash01(col as u64, 7)).fract()) * 2.4 - 1.2;
                let tail = obj * 1.5;
                let d = head - v;
                let level = if (0.0..tail).contains(&d) {
                    1.0 - d / tail
                } else {
                    0.0
                };
                mixc(level, fg(col as i64))
            }
            MapPreset::Perlin => {
                let z = 0.5 + self.zoom / 25.0;
                let n = fbm(u * z + t, v * z, self.octaves.clamp(1.0, 8.0) as u32);
                sample(pal, 0.5 + (n - 0.5) * self.amplitude / 50.0, blend)
            }
            MapPreset::Explosion => {
                let rings = count.max(1.0);
                let front = phase * 1.6;
                let mut level: f32 = 0.0;
                for k in 0..rings as usize {
                    let rr = front - k as f32 * 0.25;
                    if rr > 0.0 {
                        let w = obj * 0.4;
                        level = level.max((1.0 - (r - rr).abs() / w) * (1.0 - phase));
                    }
                }
                mixc(level, fg(round))
            }
            MapPreset::Starfield => {
                let mut level: f32 = 0.0;
                for k in 0..(count * 4.0) as u64 {
                    let a = hash01(k, 8) * TAU;
                    let life = (t * (0.6 + hash01(k, 9)) + hash01(k, 10)).fract();
                    let d = life * 1.5;
                    let (px, py) = (a.cos() * d, a.sin() * d);
                    let dist = ((u - px).powi(2) + (v - py).powi(2)).sqrt();
                    level = level.max((1.0 - dist / (0.05 + obj * 0.2)) * life);
                }
                mixc(level * 1.5, fg(0))
            }
            MapPreset::Graph => {
                let bars = count.max(1.0);
                let b = ((u + 1.0) / 2.0 * bars).floor().min(bars - 1.0);
                // The height glides smoothly from value to value
                let a = hash01(b as u64, round as u64);
                let c = hash01(b as u64, round as u64 + 1);
                let h = a + (c - a) * smooth(phase);
                let y = (1.0 - v) / 2.0;
                mixc(if y <= h { 1.0 } else { 0.0 }, fg(b as i64))
            }
            MapPreset::Lines => {
                // Stripes run across the rectangle; dark between them
                let (s, c) = self.angle.to_radians().sin_cos();
                let x = ((u * c + v * s) + 1.0) / 2.0 * count - phase * count;
                let f = x.rem_euclid(1.0);
                let soft = (blend * 0.25).max(0.02);
                let on = ((0.5 - (f - 0.25).abs() * 2.0) / soft).clamp(0.0, 1.0);
                let col = sample(pal, x.floor() / count, 0.0);
                col.map(|v| v * on)
            }
        }
    }
}

fn default_palette(preset: MapPreset, color: bool) -> Vec<[u8; 3]> {
    if !color {
        return if preset.has_background() {
            vec![[0, 0, 0], [255, 255, 255]]
        } else {
            vec![[255, 255, 255], [128, 128, 128], [0, 0, 0]]
        };
    }
    match preset {
        MapPreset::Fire => vec![
            [0, 0, 0],
            [180, 0, 0],
            [255, 80, 0],
            [255, 200, 0],
            [255, 255, 180],
        ],
        p if p.has_background() => vec![[0, 0, 0], [255, 255, 255], [255, 0, 0], [0, 0, 255]],
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

fn transform(l: [f32; 2], t: Transform) -> [f32; 2] {
    match t {
        Transform::None => l,
        Transform::VerticalSymmetry => [l[0].abs() * 2.0 - 1.0, l[1]],
        Transform::HorizontalSymmetry => [l[0], l[1].abs() * 2.0 - 1.0],
        Transform::Center => [l[0].abs() * 2.0 - 1.0, l[1].abs() * 2.0 - 1.0],
        Transform::Reverse => [-l[0], -l[1]],
    }
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [0, 1, 2].map(|i| a[i] + (b[i] - a[i]) * t)
}

/// Triangle 0..1..0 over one period.
fn tri(x: f32) -> f32 {
    1.0 - (2.0 * x.rem_euclid(1.0) - 1.0).abs()
}

fn smooth(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// 5×7 pixel font (own, simple glyphs): 5 bits per row.
fn glyph(c: char) -> [u8; 7] {
    match c.to_ascii_uppercase() {
        'A' => [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'B' => [0x1E, 0x11, 0x11, 0x1E, 0x11, 0x11, 0x1E],
        'C' => [0x0E, 0x11, 0x10, 0x10, 0x10, 0x11, 0x0E],
        'D' => [0x1E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1E],
        'E' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x1F],
        'F' => [0x1F, 0x10, 0x10, 0x1E, 0x10, 0x10, 0x10],
        'G' => [0x0E, 0x11, 0x10, 0x17, 0x11, 0x11, 0x0F],
        'H' => [0x11, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'I' => [0x0E, 0x04, 0x04, 0x04, 0x04, 0x04, 0x0E],
        'J' => [0x07, 0x02, 0x02, 0x02, 0x02, 0x12, 0x0C],
        'K' => [0x11, 0x12, 0x14, 0x18, 0x14, 0x12, 0x11],
        'L' => [0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x1F],
        'M' => [0x11, 0x1B, 0x15, 0x15, 0x11, 0x11, 0x11],
        'N' => [0x11, 0x11, 0x19, 0x15, 0x13, 0x11, 0x11],
        'O' => [0x0E, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'P' => [0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10, 0x10],
        'Q' => [0x0E, 0x11, 0x11, 0x11, 0x15, 0x12, 0x0D],
        'R' => [0x1E, 0x11, 0x11, 0x1E, 0x14, 0x12, 0x11],
        'S' => [0x0F, 0x10, 0x10, 0x0E, 0x01, 0x01, 0x1E],
        'T' => [0x1F, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04],
        'U' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'V' => [0x11, 0x11, 0x11, 0x11, 0x11, 0x0A, 0x04],
        'W' => [0x11, 0x11, 0x11, 0x15, 0x15, 0x15, 0x0A],
        'X' => [0x11, 0x11, 0x0A, 0x04, 0x0A, 0x11, 0x11],
        'Y' => [0x11, 0x11, 0x0A, 0x04, 0x04, 0x04, 0x04],
        'Z' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x10, 0x1F],
        'Ä' => [0x0A, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x11],
        'Ö' => [0x0A, 0x0E, 0x11, 0x11, 0x11, 0x11, 0x0E],
        'Ü' => [0x0A, 0x00, 0x11, 0x11, 0x11, 0x11, 0x0E],
        '0' => [0x0E, 0x11, 0x13, 0x15, 0x19, 0x11, 0x0E],
        '1' => [0x04, 0x0C, 0x04, 0x04, 0x04, 0x04, 0x0E],
        '2' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x08, 0x1F],
        '3' => [0x1F, 0x02, 0x04, 0x02, 0x01, 0x11, 0x0E],
        '4' => [0x02, 0x06, 0x0A, 0x12, 0x1F, 0x02, 0x02],
        '5' => [0x1F, 0x10, 0x1E, 0x01, 0x01, 0x11, 0x0E],
        '6' => [0x06, 0x08, 0x10, 0x1E, 0x11, 0x11, 0x0E],
        '7' => [0x1F, 0x01, 0x02, 0x04, 0x08, 0x08, 0x08],
        '8' => [0x0E, 0x11, 0x11, 0x0E, 0x11, 0x11, 0x0E],
        '9' => [0x0E, 0x11, 0x11, 0x0F, 0x01, 0x02, 0x0C],
        '!' => [0x04, 0x04, 0x04, 0x04, 0x04, 0x00, 0x04],
        '?' => [0x0E, 0x11, 0x01, 0x02, 0x04, 0x00, 0x04],
        '.' => [0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x0C],
        ',' => [0x00, 0x00, 0x00, 0x00, 0x0C, 0x04, 0x08],
        '-' => [0x00, 0x00, 0x00, 0x1F, 0x00, 0x00, 0x00],
        '+' => [0x00, 0x04, 0x04, 0x1F, 0x04, 0x04, 0x00],
        ':' => [0x00, 0x0C, 0x0C, 0x00, 0x0C, 0x0C, 0x00],
        '*' => [0x00, 0x04, 0x15, 0x0E, 0x15, 0x04, 0x00],
        '#' => [0x0A, 0x0A, 0x1F, 0x0A, 0x1F, 0x0A, 0x0A],
        _ => [0; 7],
    }
}

/// Scrolling text: level at the local position, `None` = background.
/// `zoom` (0..1) = share of the text height in the rectangle.
fn text_level(text: &str, u: f32, v: f32, phase: f32, zoom: f32) -> Option<f32> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return None;
    }
    // 7 rows over the height (depending on zoom), columns in the same grid
    let rows = 7.0 / zoom.clamp(0.3, 1.0);
    let cell = 2.0 / rows;
    let row = ((v + 1.0) / cell - (rows - 7.0) / 2.0).floor();
    if !(0.0..7.0).contains(&row) {
        return None;
    }
    let width = chars.len() as f32 * 6.0;
    let visible = 2.0 / cell;
    // in from the right, out to the left
    let col = ((u + 1.0) / cell + phase * (width + visible) - visible).floor();
    if col < 0.0 || col >= width {
        return None;
    }
    let ci = (col / 6.0) as usize;
    let x = (col as usize) % 6;
    if x == 5 {
        return None;
    }
    let bits = glyph(chars[ci])[row as usize];
    ((bits >> (4 - x)) & 1 == 1).then_some(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_inside_and_outside() {
        let m = MappingFx::new(MapPreset::Rainbow, true);
        assert!(m.local([0.5, 0.5]).is_some());
        assert!(m.local([0.9, 0.5]).is_none());
        let p = m.to_plan([0.5, -0.5]);
        let back = m.local(p).unwrap();
        assert!((back[0] - 0.5).abs() < 1e-4);
    }

    #[test]
    fn every_preset_gives_colors() {
        let m = MappingFx::new(MapPreset::Rainbow, true);
        for p in MapPreset::ALL {
            let mut m = m.clone();
            m.set_preset(p, true);
            for (x, y) in [(-0.5, 0.2), (0.0, 0.0), (0.9, -0.9)] {
                let c = m.color_at([x, y], 1.37);
                assert!(c.iter().all(|v| (0.0..=1.0).contains(v)), "{p:?} {c:?}");
            }
        }
    }
}
