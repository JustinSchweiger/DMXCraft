//! Fixtures in 3D space: housing, light beams and light sources from the
//! current DMX values.

use dmxcraft_core::fixture::ChannelKind as K;
use dmxcraft_core::venue::{rotation, Placement};
use dmxcraft_core::{ChannelRef, DmxFrame, Engine, FixtureId, Show};
use glam::{Mat3, Mat4, Quat, Vec3};

/// Basic shape of a housing part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prim {
    Cube,
    Cylinder,
}

#[derive(Clone, Copy, Debug)]
pub struct Part {
    pub prim: Prim,
    pub matrix: Mat4,
    /// linear rgb
    pub color: Vec3,
    pub gloss: f32,
    /// Self-illumination (HDR, lens)
    pub emissive: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Beam {
    pub origin: Vec3,
    pub dir: Vec3,
    /// half the beam angle in degrees
    pub half_angle: f32,
    /// linear rgb · brightness
    pub color: Vec3,
    /// Radius of the lens (width of the beam at the start)
    pub lens: f32,
    /// Gobo pattern (0 = open, 1–6 approximated)
    pub gobo: f32,
    /// Focus channel 0–1 (near → far), if present
    pub focus: Option<f32>,
}

impl Beam {
    /// Sharpness of the light spot at `dist` meters (1 = sharp). Without a focus
    /// channel medium sharpness; otherwise sharp where the focus distance
    /// (1.5–20 m) hits.
    pub fn sharpness(&self, dist: f32) -> f32 {
        match self.focus {
            None => 0.55,
            Some(f) => {
                let d_focus = 1.5 + f * 18.5;
                (-(dist.max(0.1) / d_focus).ln().abs() * 2.2).exp()
            }
        }
    }
}

/// Color of a color wheel (approximated, without manufacturer data): open
/// (white) at the bottom, then eight typical color slots.
pub fn color_wheel(v: f32) -> Vec3 {
    const SLOTS: [[f32; 3]; 8] = [
        [1.0, 0.05, 0.03],
        [1.0, 0.35, 0.02],
        [1.0, 0.85, 0.05],
        [0.1, 1.0, 0.15],
        [0.05, 0.85, 1.0],
        [0.05, 0.15, 1.0],
        [1.0, 0.05, 0.75],
        [0.7, 0.55, 1.0],
    ];
    if v < 0.06 {
        return Vec3::new(1.0, 0.95, 0.9);
    }
    let k = (((v - 0.06) / 0.94) * SLOTS.len() as f32) as usize;
    Vec3::from(SLOTS[k.min(SLOTS.len() - 1)])
}

/// Color of a color wheel channel: from the value ranges of the fixture
/// file, otherwise approximated.
pub fn wheel_color(def: &dmxcraft_core::ChannelDef, v: u8) -> Vec3 {
    use dmxcraft_core::RangeEffect as E;
    match def.range_at(v).map(|r| r.effect) {
        Some(E::Color { rgb }) => linear(rgb),
        Some(E::WheelOpen | E::Open) => Vec3::new(1.0, 0.95, 0.9),
        _ if !def.ranges.is_empty() => Vec3::new(1.0, 0.95, 0.9),
        _ => color_wheel(f32::from(v) / 255.0),
    }
}

/// Gobo pattern of a gobo channel: from the value ranges, otherwise
/// approximated.
pub fn gobo_of(def: &dmxcraft_core::ChannelDef, v: u8) -> f32 {
    use dmxcraft_core::RangeEffect as E;
    match def.range_at(v).map(|r| r.effect) {
        Some(E::Gobo { slot }) => f32::from((slot.max(1) - 1) % 6 + 1),
        Some(_) => 0.0,
        None if !def.ranges.is_empty() => 0.0,
        None => gobo(f32::from(v) / 255.0),
    }
}

/// Gobo number from the channel value (approximated: open at the bottom,
/// then six patterns).
pub fn gobo(v: f32) -> f32 {
    if v < 0.05 {
        0.0
    } else {
        (1.0 + ((v - 0.05) / 0.95 * 6.0).floor()).min(6.0)
    }
}

#[derive(Clone, Debug)]
pub struct RigFixture {
    pub parts: Vec<Part>,
    pub beams: Vec<Beam>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Par,
    Moving,
    Bar,
    Blinder,
    Fog,
    Laser,
}

/// Placement in the room: saved or automatically from the stage plan.
pub fn placement(show: &Show, id: FixtureId) -> Placement {
    show.placement3d(id)
}

pub fn kind(show: &Show, id: FixtureId) -> Kind {
    let Some(f) = show.fixture(id) else {
        return Kind::Par;
    };
    let Some(t) = show.type_of(f) else {
        return Kind::Par;
    };
    let name = format!("{} {} {}", t.manufacturer, t.model, f.name).to_lowercase();
    let has = |k| t.channels.iter().any(|c| c.kind == k);
    let count = |k| t.channels.iter().filter(|c| c.kind == k).count();
    if name.contains("fog") || name.contains("nebel") || name.contains("haze") {
        Kind::Fog
    } else if name.contains("laser") {
        Kind::Laser
    } else if has(K::Pan) || has(K::Tilt) {
        Kind::Moving
    } else if name.contains("blinder") {
        Kind::Blinder
    } else if count(K::Red) > 1 || count(K::White) > 1 || count(K::Uv) > 1 || name.contains("bar") {
        Kind::Bar
    } else {
        Kind::Par
    }
}

fn srgb_to_linear(c: [u8; 3]) -> Vec3 {
    let f = |v: u8| {
        let x = v as f32 / 255.0;
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    };
    Vec3::new(f(c[0]), f(c[1]), f(c[2]))
}

/// Color and brightness of a cell from the channel values (0–1).
fn cell_light(
    get: &dyn Fn(K, usize) -> Option<f32>,
    wheel: Option<Vec3>,
    cell: usize,
    cells: usize,
) -> Vec3 {
    let r = get(K::Red, cell);
    let g = get(K::Green, cell);
    let b = get(K::Blue, cell);
    let w = get(K::White, cell);
    let a = get(K::Amber, cell);
    let uv = get(K::Uv, cell);
    let any_color =
        r.is_some() || g.is_some() || b.is_some() || w.is_some() || a.is_some() || uv.is_some();
    let mut c = if any_color {
        Vec3::new(r.unwrap_or(0.0), g.unwrap_or(0.0), b.unwrap_or(0.0))
            + Vec3::new(1.0, 0.92, 0.82) * w.unwrap_or(0.0)
            + Vec3::new(1.0, 0.5, 0.05) * a.unwrap_or(0.0)
            + Vec3::new(0.32, 0.04, 1.0) * uv.unwrap_or(0.0) * 0.8
    } else if let Some(w) = wheel {
        w
    } else {
        // Halogen/dimmer: warm white
        Vec3::new(1.0, 0.78, 0.5)
    };
    // Dimmer: per cell, otherwise one for all; without a dimmer channel full
    let dim = get(K::Dimmer, cell)
        .or_else(|| (cells > 1).then(|| get(K::Dimmer, 0)).flatten())
        .unwrap_or(1.0);
    if !any_color && wheel.is_none() && get(K::Dimmer, 0).is_none() {
        c = Vec3::ZERO;
    }
    c * dim
}

/// Builds all fixtures from the show and the DMX output. `t` = time (s, for
/// strobe).
pub fn build(
    engine: &Engine,
    frame: &DmxFrame,
    selected: &std::collections::BTreeSet<FixtureId>,
    t: f32,
) -> (Vec<RigFixture>, f32) {
    let show = engine.show();
    let mut out = Vec::new();
    let mut fog_boost: f32 = 0.0;
    for f in &show.fixtures {
        let Some(ty) = show.type_of(f) else {
            continue;
        };
        let kind = kind(show, f.id);
        let p = placement(show, f.id);
        let rot = Mat3::from_cols_array_2d(&rotation(p.rotation));
        let base = Mat4::from_mat3(rot) * Mat4::IDENTITY;
        let base = Mat4::from_translation(Vec3::from(p.position)) * base;
        // Channel values by kind, in order (for cells)
        let value = |i: usize| {
            engine
                .output_value(
                    frame,
                    ChannelRef {
                        fixture: f.id,
                        channel: i as u16,
                    },
                )
                .unwrap_or(0) as f32
                / 255.0
        };
        let by_kind = |k: K| -> Vec<usize> {
            ty.channels
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == k)
                .map(|(i, _)| i)
                .collect()
        };
        let cells = [K::Red, K::Green, K::Blue, K::White, K::Uv, K::Amber]
            .iter()
            .map(|k| by_kind(*k).len())
            .max()
            .unwrap_or(1)
            .max(1);
        let get = |k: K, cell: usize| -> Option<f32> {
            let list = by_kind(k);
            if list.is_empty() {
                return None;
            }
            Some(value(list[cell % list.len()]))
        };
        let raw = |i: usize| (value(i) * 255.0).round() as u8;
        // Shutter/strobe by the value ranges of the fixture file
        let shutter_on = match by_kind(K::Strobe)
            .first()
            .and_then(|i| ty.channels[*i].strobe(raw(*i)))
        {
            None => true,
            Some(hz) if hz <= 0.0 => false,
            Some(hz) => (t * hz).fract() < 0.25,
        };
        // Color wheel, gobo and focus
        let wheel_c = by_kind(K::ColorWheel)
            .first()
            .map(|i| wheel_color(&ty.channels[*i], raw(*i)));
        let focus = by_kind(K::Focus).first().map(|i| value(*i));
        // Beam angle from the fixture data (with a zoom channel between min and
        // max)
        // Wash lights without zoom and gobo often call their zoom “focus”.
        let zoom_ch = ty
            .channels
            .iter()
            .position(|c| c.kind == K::Zoom || c.name.to_lowercase().contains("zoom"))
            .or_else(|| {
                let range = ty
                    .physical
                    .is_some_and(|p| p.beam_angle[0] != p.beam_angle[1]);
                (range && by_kind(K::Gobo).is_empty())
                    .then(|| by_kind(K::Focus).first().copied())
                    .flatten()
            });
        let zoom = zoom_ch.map(value);
        let phys_half = ty.physical.map(|p| {
            let [a, b] = p.beam_angle;
            let z = zoom.unwrap_or(0.0);
            let z = if p.zoom_reversed { 1.0 - z } else { z };
            (a + (b - a) * z) * 0.5
        });
        // used as zoom: no focus effect on the spot
        let focus = if zoom_ch.is_some() && zoom_ch == by_kind(K::Focus).first().copied() {
            None
        } else {
            focus
        };
        let gobo_v = by_kind(K::Gobo)
            .first()
            .map_or(0.0, |i| gobo_of(&ty.channels[*i], raw(*i)));
        let sel = selected.contains(&f.id);
        let body = if sel {
            Vec3::new(0.0, 0.45, 0.5)
        } else {
            Vec3::splat(0.03)
        };
        // metal parts: slightly lighter than the housing
        let trim = if sel { body * 1.4 } else { Vec3::splat(0.07) };
        let mut parts = Vec::new();
        let mut beams = Vec::new();
        let trs = |s: Vec3, r: Quat, tr: Vec3| Mat4::from_scale_rotation_translation(s, r, tr);
        // Brightness from the profile (lumens, otherwise power) relative to a
        // typical fixture of this kind
        let output = ty
            .physical
            .and_then(|p| p.light_output())
            .map_or(1.0, |lm| {
                let typical = match kind {
                    Kind::Moving => 3000.0,
                    Kind::Bar => 800.0,
                    Kind::Blinder => 4000.0,
                    _ => 1500.0,
                };
                (lm / typical).clamp(0.2, 5.0)
            });
        let mut lens_parts: Vec<Part> = Vec::new();
        let mut add_lens =
            |m: Mat4, light: Vec3, origin: Vec3, dir: Vec3, half: f32, lens: f32, power: f32| {
                let on = if shutter_on { light } else { Vec3::ZERO };
                lens_parts.push(Part {
                    prim: Prim::Cylinder,
                    matrix: m,
                    color: on.max(Vec3::splat(0.02)),
                    gloss: 0.9,
                    emissive: on.max_element().min(1.0) * 2.5,
                });
                if on.max_element() > 0.002 {
                    beams.push(Beam {
                        origin,
                        dir,
                        half_angle: half,
                        color: on * power * output,
                        lens,
                        gobo: gobo_v,
                        focus,
                    });
                }
            };
        let down = Vec3::NEG_Y;
        match kind {
            Kind::Blinder => {
                // Housing with two lamps side by side
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(0.46, 0.14, 0.24),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.09, 0.0),
                    ),
                    body,
                );
                bracket(&mut parts, base, 0.5, body);
                let light = cell_light(&get, wheel_c, 0, 1);
                let half = phys_half.unwrap_or(30.0);
                for x in [-0.11f32, 0.11] {
                    cylinder(
                        &mut parts,
                        base * trs(
                            Vec3::new(0.1, 0.03, 0.1),
                            Quat::IDENTITY,
                            Vec3::new(x, -0.175, 0.0),
                        ),
                        trim,
                        0.7,
                    );
                    let o = base.transform_point3(Vec3::new(x, -0.193, 0.0));
                    let d = base.transform_vector3(down).normalize();
                    add_lens(
                        base * trs(
                            Vec3::new(0.085, 0.006, 0.085),
                            Quat::IDENTITY,
                            Vec3::new(x, -0.19, 0.0),
                        ),
                        light * 0.5,
                        o,
                        d,
                        half,
                        0.085,
                        9.0,
                    );
                }
            }
            Kind::Par => {
                let name = ty.model.to_lowercase();
                let cob = name.contains("cob") || name.contains("revue");
                let half = phys_half.unwrap_or(if cob {
                    25.0
                } else if name.contains("wash") {
                    18.0
                } else {
                    13.0
                });
                let (r, len) = (0.11, 0.26);
                // Can: body, rear cap with cooling fins, front ring
                cylinder(
                    &mut parts,
                    base * trs(
                        Vec3::new(r, len, r),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -len * 0.5 - 0.02, 0.0),
                    ),
                    body,
                    0.5,
                );
                cylinder(
                    &mut parts,
                    base * trs(
                        Vec3::new(r * 0.8, 0.04, r * 0.8),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.01, 0.0),
                    ),
                    trim,
                    0.3,
                );
                cylinder(
                    &mut parts,
                    base * trs(
                        Vec3::new(r * 1.08, 0.025, r * 1.08),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -len - 0.02, 0.0),
                    ),
                    trim,
                    0.7,
                );
                // U-bracket with the side knobs
                bracket(&mut parts, base, r * 2.4, body);
                for sx in [-1.0f32, 1.0] {
                    cube(
                        &mut parts,
                        base * trs(
                            Vec3::new(0.012, len * 0.75, 0.04),
                            Quat::IDENTITY,
                            Vec3::new(sx * (r + 0.02), -len * 0.3, 0.0),
                        ),
                        body,
                    );
                    cylinder(
                        &mut parts,
                        base * Mat4::from_translation(Vec3::new(
                            sx * (r + 0.035),
                            -len * 0.55,
                            0.0,
                        )) * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2)
                            * trs(Vec3::new(0.022, 0.02, 0.022), Quat::IDENTITY, Vec3::ZERO),
                        trim,
                        0.6,
                    );
                }
                let light = cell_light(&get, wheel_c, 0, 1);
                let o = base.transform_point3(Vec3::new(0.0, -len - 0.035, 0.0));
                let d = base.transform_vector3(down).normalize();
                let lens_m = base
                    * trs(
                        Vec3::new(r * 0.95, 0.006, r * 0.95),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -len - 0.032, 0.0),
                    );
                add_lens(lens_m, light, o, d, half, r * 0.9, 9.0);
                if !cob {
                    led_dots(&mut parts, lens_m, light, shutter_on, 18);
                }
            }
            Kind::Bar => {
                let width = (cells as f32 * 0.22).clamp(0.5, 1.3);
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(width, 0.07, 0.1),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.045, 0.0),
                    ),
                    body,
                );
                // end caps and mounting brackets
                for sx in [-1.0f32, 1.0] {
                    cube(
                        &mut parts,
                        base * trs(
                            Vec3::new(0.02, 0.08, 0.11),
                            Quat::IDENTITY,
                            Vec3::new(sx * (width * 0.5 + 0.01), -0.045, 0.0),
                        ),
                        trim,
                    );
                    cube(
                        &mut parts,
                        base * trs(
                            Vec3::new(0.04, 0.012, 0.06),
                            Quat::IDENTITY,
                            Vec3::new(sx * width * 0.3, -0.004, 0.0),
                        ),
                        trim,
                    );
                }
                for c in 0..cells {
                    let x = -width * 0.5 + width * (c as f32 + 0.5) / cells as f32;
                    let light = cell_light(&get, wheel_c, c, cells);
                    let o = base.transform_point3(Vec3::new(x, -0.082, 0.0));
                    let d = base.transform_vector3(down).normalize();
                    add_lens(
                        base * trs(
                            Vec3::new(0.03, 0.006, 0.03),
                            Quat::IDENTITY,
                            Vec3::new(x, -0.08, 0.0),
                        ),
                        light,
                        o,
                        d,
                        phys_half.unwrap_or(17.0),
                        0.03,
                        5.0,
                    );
                }
            }
            Kind::Moving => {
                // Pan/tilt (with fine channel), center = 50 %. The DMX direction
                // of moving heads is defined for a standing fixture; hanging
                // (the default here) both axes turn the other way round.
                // Checked against real heads aimed at known points.
                let fine = |coarse: K, fine: K| {
                    let c = by_kind(coarse).first().map(|i| value(*i))?;
                    let f = by_kind(fine).first().map(|i| value(*i)).unwrap_or(0.0);
                    Some(c + f / 255.0)
                };
                let (pan_range, tilt_range) = ty
                    .physical
                    .map_or((540.0, 270.0), |p| (p.pan_range, p.tilt_range));
                let pan = (0.5 - fine(K::Pan, K::PanFine).unwrap_or(0.5)) * pan_range.to_radians();
                let tilt =
                    (0.5 - fine(K::Tilt, K::TiltFine).unwrap_or(0.5)) * tilt_range.to_radians();
                // Size from the profile (height of the fixture), otherwise ~42 cm
                let k = ty
                    .physical
                    .map(|p| p.size_mm[1])
                    .filter(|h| *h > 100.0)
                    .map_or(1.0, |h| (h / 420.0).clamp(0.6, 1.6));
                let wash = by_kind(K::Gobo).is_empty();
                let sc = |v: Vec3| v * k;
                // Base with display, turntable
                cube(
                    &mut parts,
                    base * trs(
                        sc(Vec3::new(0.34, 0.11, 0.26)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, -0.055, 0.0)),
                    ),
                    body,
                );
                cube(
                    &mut parts,
                    base * trs(
                        sc(Vec3::new(0.1, 0.05, 0.004)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.06, -0.055, 0.131)),
                    ),
                    Vec3::new(0.02, 0.05, 0.12),
                );
                cylinder(
                    &mut parts,
                    base * trs(
                        sc(Vec3::new(0.09, 0.03, 0.09)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, -0.125, 0.0)),
                    ),
                    trim,
                    0.5,
                );
                // Yoke: crossbar and two arms
                let yoke = base * Mat4::from_rotation_y(pan);
                cube(
                    &mut parts,
                    yoke * trs(
                        sc(Vec3::new(0.34, 0.035, 0.1)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, -0.157, 0.0)),
                    ),
                    body,
                );
                for sx in [-1.0f32, 1.0] {
                    cube(
                        &mut parts,
                        yoke * trs(
                            sc(Vec3::new(0.04, 0.2, 0.1)),
                            Quat::IDENTITY,
                            sc(Vec3::new(sx * 0.15, -0.27, 0.0)),
                        ),
                        body,
                    );
                    // tilt axle cover
                    cylinder(
                        &mut parts,
                        yoke * Mat4::from_translation(sc(Vec3::new(sx * 0.15, -0.33, 0.0)))
                            * Mat4::from_rotation_z(std::f32::consts::FRAC_PI_2)
                            * trs(
                                sc(Vec3::new(0.045, 0.05, 0.045)),
                                Quat::IDENTITY,
                                Vec3::ZERO,
                            ),
                        trim,
                        0.6,
                    );
                }
                // Head: wash = wide and short, spot = narrower and longer
                let head = yoke
                    * Mat4::from_translation(sc(Vec3::new(0.0, -0.33, 0.0)))
                    * Mat4::from_rotation_x(tilt);
                let (hr, hl) = if wash { (0.125, 0.2) } else { (0.1, 0.28) };
                cylinder(
                    &mut parts,
                    head * trs(sc(Vec3::new(hr, hl, hr)), Quat::IDENTITY, Vec3::ZERO),
                    body,
                    0.5,
                );
                // back cap and front bezel
                cylinder(
                    &mut parts,
                    head * trs(
                        sc(Vec3::new(hr * 0.75, 0.03, hr * 0.75)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, hl * 0.5 + 0.012, 0.0)),
                    ),
                    trim,
                    0.4,
                );
                cylinder(
                    &mut parts,
                    head * trs(
                        sc(Vec3::new(hr * 1.04, 0.02, hr * 1.04)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, -hl * 0.5 - 0.005, 0.0)),
                    ),
                    trim,
                    0.7,
                );
                let light = cell_light(&get, wheel_c, 0, cells.max(1));
                let o = head.transform_point3(sc(Vec3::new(0.0, -hl * 0.5 - 0.02, 0.0)));
                let d = head.transform_vector3(down).normalize();
                let lens_m = head
                    * trs(
                        sc(Vec3::new(hr * 0.92, 0.006, hr * 0.92)),
                        Quat::IDENTITY,
                        sc(Vec3::new(0.0, -hl * 0.5 - 0.016, 0.0)),
                    );
                add_lens(
                    lens_m,
                    light,
                    o,
                    d,
                    phys_half.unwrap_or(if wash { 15.0 } else { 9.0 }),
                    hr * 0.8 * k,
                    12.0,
                );
                if wash {
                    // LED wash: individual lenses in rings (1 + 6 + 12)
                    led_dots(&mut parts, lens_m, light, shutter_on, 19);
                }
            }
            Kind::Fog => {
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(0.25, 0.25, 0.45),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.125, 0.0),
                    ),
                    body,
                );
                // Fog channel (generic/dimmer): increases the haze in the room
                let amount = by_kind(K::Generic)
                    .first()
                    .map(|i| value(*i))
                    .or_else(|| get(K::Dimmer, 0))
                    .unwrap_or(0.0);
                fog_boost = fog_boost.max(amount);
            }
            Kind::Laser => {
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(0.35, 0.1, 0.2),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.05, 0.0),
                    ),
                    body,
                );
                let light = cell_light(&get, wheel_c, 0, 1);
                let light = if light.max_element() < 0.01 {
                    // Laser without a color channel: green when any channel is active
                    let any = (0..ty.channels.len()).any(|i| value(i) > 0.05);
                    if any {
                        Vec3::new(0.05, 1.0, 0.1)
                    } else {
                        Vec3::ZERO
                    }
                } else {
                    light
                };
                for k in 0..5 {
                    let a = (k as f32 - 2.0) * 0.18;
                    let d = base
                        .transform_vector3(
                            Quat::from_rotation_z(a) * Quat::from_rotation_x(0.35) * down,
                        )
                        .normalize();
                    let o = base.transform_point3(Vec3::new(0.0, -0.1, 0.05));
                    add_lens(
                        base * trs(
                            Vec3::new(0.01, 0.004, 0.01),
                            Quat::IDENTITY,
                            Vec3::new(0.0, -0.1, 0.05),
                        ),
                        light,
                        o,
                        d,
                        0.25,
                        0.004,
                        40.0,
                    );
                }
            }
        }
        parts.extend(lens_parts);
        out.push(RigFixture { parts, beams });
    }
    (out, fog_boost)
}

fn cylinder(parts: &mut Vec<Part>, m: Mat4, c: Vec3, gloss: f32) {
    parts.push(Part {
        prim: Prim::Cylinder,
        matrix: m,
        color: c,
        gloss,
        emissive: 0.0,
    });
}

/// Mounting bracket (omega clamp plate) on top of a fixture.
fn bracket(parts: &mut Vec<Part>, base: Mat4, width: f32, c: Vec3) {
    cube(
        parts,
        base * Mat4::from_scale_rotation_translation(
            Vec3::new(width, 0.015, 0.05),
            Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 0.0),
        ),
        c,
    );
}

/// Individual LED lenses on a lens disk (`lens` = matrix of the disk, unit
/// radius): 1 + 6 + 12 … in rings, glowing in the light color.
fn led_dots(parts: &mut Vec<Part>, lens: Mat4, light: Vec3, shutter_on: bool, count: usize) {
    let on = if shutter_on { light } else { Vec3::ZERO };
    let mut spots = vec![(0.0f32, 0.0f32)];
    let mut ring = 1;
    while spots.len() < count {
        let n = ring * 6;
        let r = ring as f32 * 0.31;
        for i in 0..n {
            let a = std::f32::consts::TAU * i as f32 / n as f32;
            spots.push((r * a.cos(), r * a.sin()));
        }
        ring += 1;
    }
    spots.truncate(count);
    for (x, z) in spots {
        parts.push(Part {
            prim: Prim::Cylinder,
            matrix: lens
                * Mat4::from_scale_rotation_translation(
                    Vec3::new(0.12, 1.4, 0.12),
                    Quat::IDENTITY,
                    Vec3::new(x, -0.8, z),
                ),
            color: on.max(Vec3::splat(0.04)),
            gloss: 0.9,
            emissive: on.max_element().min(1.0) * 3.0,
        });
    }
}

fn cube(parts: &mut Vec<Part>, m: Mat4, c: Vec3) {
    parts.push(Part {
        prim: Prim::Cube,
        matrix: m,
        color: c,
        gloss: 0.4,
        emissive: 0.0,
    });
}

/// Linear color from sRGB bytes (for materials).
pub fn linear(c: [u8; 3]) -> Vec3 {
    srgb_to_linear(c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dmxcraft_core::{ChannelRef, Show};

    /// Beam direction of a hanging moving head (rotation 0) for pan/tilt.
    fn beam(pan: u8, tilt: u8) -> Vec3 {
        let mut show = Show::new("t", 1);
        show.fixture_types = dmxcraft_core::library::builtin_fixture_types();
        let id = show
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        show.fixture_mut(id).unwrap().place3d = Some(dmxcraft_core::venue::Placement {
            position: [0.0, 5.0, 0.0],
            rotation: [0.0; 3],
        });
        let mut e = Engine::new(show);
        for (ch, v) in [(0, pan), (2, tilt), (5, 255)] {
            e.set_programmer(
                ChannelRef {
                    fixture: id,
                    channel: ch,
                },
                v,
            );
        }
        let frame = e.render_monitor();
        let (rig, _) = build(&e, &frame, &Default::default(), 0.0);
        rig[0].beams[0].dir
    }

    #[test]
    fn hanging_moving_head_directions() {
        // center: straight down
        let d = beam(128, 128);
        assert!(d.y < -0.99, "{d}");
        // higher tilt: towards +z (front of the hanging fixture)
        let d = beam(128, 200);
        assert!(d.z > 0.5 && d.x.abs() < 0.05, "{d}");
        // pan a quarter turn further (540° range: 255/6): the tilt direction
        // turns clockwise seen from above (+z towards -x)
        let d = beam(128 + 43, 200);
        assert!(d.x < -0.5 && d.z.abs() < 0.1, "{d}");
    }

    #[test]
    fn core_direction_matches_visualizer() {
        for (pan, tilt) in [(128u8, 128u8), (128, 200), (171, 200), (30, 60), (240, 10)] {
            let d = beam(pan, tilt);
            let c = dmxcraft_core::venue::moving_head_direction(
                [0.0; 3],
                f32::from(pan) / 255.0,
                f32::from(tilt) / 255.0,
                540.0,
                270.0,
            );
            assert!(
                d.distance(Vec3::from(c)) < 0.02,
                "{pan}/{tilt}: {d} vs {c:?}"
            );
        }
    }
}
