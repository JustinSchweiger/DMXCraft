//! Effects of FX scenes. Currently: COLOR FX – color gradients across the
//! selected fixtures (“beams”), computed from a color palette and the effect
//! time. Most templates run along the **beam order** (= the order in which
//! the fixtures were selected); plasma and Perlin use the position in the
//! stage plan.
//!
//! Time is measured in **cycles** (`cycles`, 1.0 = one scene duration); so
//! duration, speed and phase of the scene work as usual.

use crate::fasthash::FastMap as HashMap;
use std::f32::consts::TAU;

use serde::{Deserialize, Serialize};

pub mod feature;
pub mod mapping;
pub mod motion;

use crate::fixture::ChannelKind;
use crate::plan;
use crate::show::{ChannelRef, FixtureId, Show};

/// Template of a COLOR FX.
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

    /// Matching start palette of the template.
    pub fn default_palette(self) -> Vec<[u8; 3]> {
        match self {
            ColorPreset::KnightRider => vec![[0, 0, 0], [255, 0, 0]],
            ColorPreset::Sparkle => vec![[0, 0, 0], [255, 255, 255]],
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

/// Transformation of the beam positions before the calculation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transform {
    #[default]
    None,
    /// Mirrored left and right (from the center outwards).
    VerticalSymmetry,
    /// Mirrored top and bottom.
    HorizontalSymmetry,
    /// Both axes mirrored.
    Center,
    /// Running direction reversed.
    Reverse,
}

impl Transform {
    pub fn label(self) -> &'static str {
        match self {
            Transform::None => "None",
            Transform::VerticalSymmetry => "Vertical symmetry",
            Transform::HorizontalSymmetry => "Horizontal symmetry",
            Transform::Center => "From the center",
            Transform::Reverse => "Reverse",
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

/// Settings of a COLOR FX. Not every template uses every field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ColorFx {
    pub preset: ColorPreset,
    pub palette: Vec<[u8; 3]>,
    pub grayscale: bool,
    pub transform: Transform,
    /// Color width (rainbow, burst): 0..=100.
    pub width: f32,
    /// Angle of the running direction in degrees (rainbow, Perlin): 0..=360.
    pub angle: f32,
    /// Softness of the transitions between the palette colors: 0..=100.
    pub blend: f32,
    /// Knight Rider: width of the bar in beams.
    pub size: f32,
    pub one_direction: bool,
    /// Knight Rider: soft edges.
    pub crossfade: bool,
    /// Knight Rider: the bar gets brighter towards the middle.
    pub fade_outward: bool,
    /// Knight Rider: afterglow in % of the beams.
    pub trail: f32,
    /// Plasma: size and shape per axis, speed per axis.
    pub plasma_size: [f32; 2],
    pub plasma_param: [f32; 2],
    pub plasma_speed: [f32; 2],
    pub plasma_speed_param: [f32; 2],
    /// Sweep: every second round in the opposite direction.
    pub alternate: bool,
    /// Sparkle: sparks per cycle, lifetime (0..=1 cycle), width.
    pub sparkle_count: f32,
    pub sparkle_life: f32,
    pub sparkle_width: f32,
    /// Random fill: beams per dot.
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

    /// VALUE FX: same templates, palette of gray values.
    pub fn values() -> Self {
        let mut fx = Self::new(ColorPreset::Rainbow);
        fx.palette = vec![[255, 255, 255], [128, 128, 128], [0, 0, 0]];
        fx
    }

    /// Choose another template: palette and settings of the template.
    pub fn set_preset(&mut self, preset: ColorPreset) {
        let grayscale = self.grayscale;
        let transform = self.transform;
        *self = Self::new(preset);
        self.grayscale = grayscale;
        self.transform = transform;
    }
}

/// A single colorable light point: a fixture or a segment of a multi-cell
/// fixture (e.g. a cell of an LED bar).
#[derive(Clone, Debug)]
pub struct Beam {
    pub fixture: FixtureId,
    /// Position in the stage plan (0..=1).
    pub pos: [f32; 2],
    pub red: Option<u16>,
    pub green: Option<u16>,
    pub blue: Option<u16>,
    /// Without color channels: brightness via dimmer or white.
    pub intensity: Option<u16>,
}

/// Beams of the fixtures in the given order.
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
        // Segments side by side along the fixture's rotation
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

/// Colors of all beams at time `cycles`.
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
    // Position in the beam order: cyclic (i/n) or from start to end
    let cyc = order_coords(n, fx.transform, true);
    let line = order_coords(n, fx.transform, false);
    let colors: Vec<[f32; 3]> = match fx.preset {
        ColorPreset::Rainbow => {
            // Color width 0: every palette color once across all beams.
            // The angle rotates the running direction (180° = reversed, 90° = all
            // equal).
            let span = color_span(fx.width);
            // rounded so 180° reverses the order exactly
            let dir = (fx.angle.to_radians().cos() * 1e4).round() / 1e4;
            let proj: Vec<f32> = cyc.iter().map(|u| u * dir).collect();
            let min = proj.iter().copied().fold(f32::INFINITY, f32::min);
            proj.iter()
                .map(|p| sample(&palette, (p - min) * span - phase + 1e-5, blend))
                .collect()
        }
        ColorPreset::Burst => {
            // from the center outwards; color width as for the rainbow
            let span = color_span(fx.width);
            let mid = (n as f32 - 1.0) / 2.0;
            (0..n)
                .map(|i| {
                    let d = (i as f32 - mid).abs() / n as f32;
                    sample(&palette, d * span - phase, blend)
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
                    // Step so that all are filled at phase 1
                    if u * (n as f32 - 1.0) / n as f32 + 1.0 / n as f32 <= phase + 1e-6 {
                        cur
                    } else {
                        prev
                    }
                })
                .collect()
        }
        ColorPreset::Sparkle => {
            // Sparks on beam numbers, in the chosen order
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

/// Output an effect of a scene (overrides existing values).
pub fn apply_effect(
    show: &Show,
    fx: &crate::show::Effect,
    cycles: f64,
    out: &mut HashMap<ChannelRef, u8>,
) {
    apply_effect_hold(show, fx, cycles, None, out);
}

/// As [`apply_effect`]; `hold` keeps the beams of a MOVE FX on a point.
pub fn apply_effect_hold(
    show: &Show,
    fx: &crate::show::Effect,
    cycles: f64,
    hold: Option<[f32; 2]>,
    out: &mut HashMap<ChannelRef, u8>,
) {
    use crate::show::EffectKind;
    match fx.kind {
        EffectKind::ColorFx => {
            if let Some(c) = &fx.color {
                apply(show, c, &fx.fixtures, cycles, out);
            }
        }
        EffectKind::ColorMappings => {
            if let Some(m) = &fx.mapping {
                let beams = beams(show, &fx.fixtures);
                let colors: Vec<Option<[u8; 3]>> = beams
                    .iter()
                    .map(|b| m.local(b.pos).map(|l| to_rgb(m.color_at(l, cycles))))
                    .collect();
                write_colors(&beams, &colors, out);
            }
        }
        EffectKind::MoveFx => {
            if let Some(m) = &fx.motion {
                let beams = feature::fixture_beams(show, &fx.fixtures);
                motion::apply(show, m, &beams, cycles, hold, out);
            }
        }
        k if k.uses_features() => {
            let beams = feature::fixture_beams(show, &fx.fixtures);
            let levels = feature_levels(fx, &beams, cycles);
            let relative = fx.curve.as_ref().is_some_and(|c| c.relative);
            feature::apply_levels(show, &beams, &levels, &fx.features, relative, out);
        }
        _ => {}
    }
}

/// Level (0..1) per beam of a feature effect; `None` = untouched.
pub fn feature_levels(
    fx: &crate::show::Effect,
    beams: &[feature::FixtureBeam],
    cycles: f64,
) -> Vec<Option<f32>> {
    use crate::show::EffectKind;
    let n = beams.len();
    let all = |v: Vec<f32>| v.into_iter().map(Some).collect();
    match fx.kind {
        EffectKind::ChaserFx => fx
            .chaser
            .as_ref()
            .map_or_else(Vec::new, |c| all(feature::chaser_levels(c, n, cycles))),
        EffectKind::CurveFx => fx
            .curve
            .as_ref()
            .map_or_else(Vec::new, |c| all(feature::curve_levels(c, n, cycles))),
        EffectKind::ValueFx => {
            let Some(c) = &fx.color else {
                return Vec::new();
            };
            // same templates as COLOR FX, level = brightness
            let pseudo: Vec<Beam> = beams
                .iter()
                .map(|b| Beam {
                    fixture: b.fixture,
                    pos: b.pos,
                    red: None,
                    green: None,
                    blue: None,
                    intensity: Some(0),
                })
                .collect();
            render(c, &pseudo, cycles)
                .into_iter()
                .map(|c| Some(luma(c) / 255.0))
                .collect()
        }
        EffectKind::Mappings => {
            let Some(m) = &fx.mapping else {
                return Vec::new();
            };
            beams
                .iter()
                .map(|b| {
                    m.local(b.pos).map(|l| {
                        let c = m.color_at(l, cycles);
                        0.299 * c[0] + 0.587 * c[1] + 0.114 * c[2]
                    })
                })
                .collect()
        }
        _ => Vec::new(),
    }
}

fn luma(c: [u8; 3]) -> f32 {
    0.299 * f32::from(c[0]) + 0.587 * f32::from(c[1]) + 0.114 * f32::from(c[2])
}

fn to_rgb(c: [f32; 3]) -> [u8; 3] {
    c.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)
}

/// Write colors to the beams (as brightness without color channels).
fn write_colors(beams: &[Beam], colors: &[Option<[u8; 3]>], out: &mut HashMap<ChannelRef, u8>) {
    for (b, c) in beams.iter().zip(colors) {
        let Some(c) = c else {
            continue;
        };
        let ch = |channel: u16| ChannelRef {
            fixture: b.fixture,
            channel,
        };
        if let Some(i) = b.intensity {
            out.insert(ch(i), luma(*c).round() as u8);
            continue;
        }
        for (slot, v) in [(b.red, c[0]), (b.green, c[1]), (b.blue, c[2])] {
            if let Some(i) = slot {
                out.insert(ch(i), v);
            }
        }
    }
}

/// Write the output values of the effect into `out` (overrides existing
/// ones).
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

/// Positions stretched to 0..=1 (both axes together, the aspect ratio is
/// kept), then transformed.
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
                // smaller axis centered
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

/// Position of every beam in the order, transformed. `cyclic`: i/n (for
/// wrapping gradients, the last beam connects to the first), otherwise
/// 0..=1 from the first to the last beam.
/// Share of the palette that runs across all beams: color width 0 = whole
/// palette, larger = wider color areas.
fn color_span(width: f32) -> f32 {
    1.0 / (1.0 + width.max(0.0) / 10.0)
}

fn order_coords(n: usize, t: Transform, cyclic: bool) -> Vec<f32> {
    let lin = |i: usize| {
        if cyclic {
            i as f32 / n as f32
        } else {
            i as f32 / (n.max(2) - 1) as f32
        }
    };
    // mirrored: same position for i and n-1-i
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

/// Rank of every value (0 = smallest); ties keep the order.
fn ranks(v: &[f32]) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|a, b| v[*a].total_cmp(&v[*b]).then(a.cmp(b)));
    let mut out = vec![0; v.len()];
    for (rank, i) in idx.into_iter().enumerate() {
        out[i] = rank;
    }
    out
}

/// Color at position `x` (cyclic); `blend` 0 = hard edges, 1 = blended
/// throughout.
pub(crate) fn sample(palette: &[[f32; 3]], x: f32, blend: f32) -> [f32; 3] {
    let k = palette.len();
    if k == 1 {
        return palette[0];
    }
    let x = x.rem_euclid(1.0) * k as f32;
    let i = x.floor() as usize % k;
    let f = x.fract();
    // Blending only in the rear part of every section
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
    // The head runs back and forth or in one direction only
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
            // Bar
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
            // Afterglow behind the head
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

/// Pseudo-random 0..1 from two values (fixed so the effect behaves
/// reproducibly).
pub(crate) fn hash01(a: u64, b: u64) -> f32 {
    let mut x = a.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ b.wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
    x ^= x >> 33;
    x = x.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    x ^= x >> 33;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

/// Value noise with smooth interpolation.
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

/// Several octaves of noise, result 0..1.
pub(crate) fn fbm(x: f32, y: f32, octaves: u32) -> f32 {
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
        // a full cycle gives the same image again
        assert_eq!(a, render(&fx, &b, 1.0));
    }

    #[test]
    fn rainbow_follows_beam_order() {
        let mut fx = ColorFx::new(ColorPreset::Rainbow);
        fx.palette = (0..9).map(|i| [i * 20, 0, 0]).collect();
        // Position in the plan does not matter: the order counts
        let mut b = line(9);
        b.reverse();
        let now = render(&fx, &b, 0.0);
        for (i, c) in now.iter().enumerate() {
            assert_eq!(*c, fx.palette[i], "Beam {i}");
        }
        // one beam further: the color moves from beam i to beam i+1
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
        let c = render(&fx, &line(4), 1.5); // Round 1: green fills over red
        assert_eq!(c[0], [0, 255, 0]);
        assert_eq!(c[3], [255, 0, 0]);
    }

    #[test]
    fn rainbow_angle_reverses_and_flattens() {
        let mut fx = ColorFx::new(ColorPreset::Rainbow);
        fx.palette = (0..6).map(|i| [i * 40, 0, 0]).collect();
        fx.blend = 0.0;
        let b = line(6);
        let fwd = render(&fx, &b, 0.0);
        fx.angle = 180.0;
        let mut back = render(&fx, &b, 0.0);
        back.reverse();
        // reversed order, same colors
        assert_eq!(back, fwd);
        fx.angle = 90.0;
        let flat = render(&fx, &b, 0.0);
        assert!(flat.iter().all(|c| *c == flat[0]));
    }

    #[test]
    fn burst_width_widens_rings() {
        let mut fx = ColorFx::new(ColorPreset::Burst);
        fx.palette = (0..8).map(|i| [i * 30, 0, 0]).collect();
        fx.width = 0.0;
        let b = line(7);
        let spread = |c: &[[u8; 3]]| i32::from(c[0][0]) - i32::from(c[3][0]);
        let narrow = spread(&render(&fx, &b, 0.0)).abs();
        fx.width = 50.0;
        let wide = spread(&render(&fx, &b, 0.0)).abs();
        assert!(wide < narrow, "{wide} < {narrow}");
        // symmetrical to the center
        let c = render(&fx, &b, 0.3);
        assert_eq!(c[0], c[6]);
    }

    #[test]
    fn all_presets_render_every_beam() {
        for p in ColorPreset::ALL {
            let fx = ColorFx::new(p);
            assert_eq!(render(&fx, &line(5), 2.7).len(), 5, "{p:?}");
        }
    }
}
