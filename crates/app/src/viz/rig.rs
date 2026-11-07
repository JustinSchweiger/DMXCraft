//! Geräte im 3D-Raum: Gehäuse, Lichtstrahlen und Lichtquellen aus den
//! aktuellen DMX-Werten.

use dmxcraft_core::fixture::ChannelKind as K;
use dmxcraft_core::venue::{rotation, Placement};
use dmxcraft_core::{ChannelRef, DmxFrame, Engine, FixtureId, Show};
use glam::{Mat3, Mat4, Quat, Vec3};

/// Grundform eines Gehäuseteils.
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
    /// Selbstleuchten (HDR, Linse)
    pub emissive: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Beam {
    pub origin: Vec3,
    pub dir: Vec3,
    /// halber Öffnungswinkel in Grad
    pub half_angle: f32,
    /// linear rgb · Helligkeit
    pub color: Vec3,
    /// Radius der Linse (Breite des Strahls am Anfang)
    pub lens: f32,
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

/// Lage im Raum: gespeicherte oder automatisch aus dem Bühnenplan.
pub fn placement(show: &Show, id: FixtureId) -> Placement {
    if let Some(p) = show.fixture(id).and_then(|f| f.place3d) {
        return p;
    }
    let [w, h, d] = show.venue.size;
    let [px, py] = show.plan_position(id);
    Placement {
        position: [
            (px - 0.5) * w * 0.8,
            (h - 0.35).max(1.5),
            (0.5 - py) * d * 0.8,
        ],
        rotation: [0.0; 3],
    }
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

/// Farbe und Helligkeit einer Zelle aus den Kanalwerten (0–1).
fn cell_light(get: &dyn Fn(K, usize) -> Option<f32>, cell: usize, cells: usize) -> Vec3 {
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
    } else {
        // Halogen/Dimmer: warmweiß
        Vec3::new(1.0, 0.78, 0.5)
    };
    // Dimmer: je Zelle, sonst einer für alle; ohne Dimmerkanal voll
    let dim = get(K::Dimmer, cell)
        .or_else(|| (cells > 1).then(|| get(K::Dimmer, 0)).flatten())
        .unwrap_or(1.0);
    if !any_color && get(K::Dimmer, 0).is_none() {
        c = Vec3::ZERO;
    }
    c * dim
}

/// Baut alle Geräte aus Show und DMX-Ausgang. `t` = Zeit (s, für Strobe).
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
        // Kanalwerte nach Art, in Reihenfolge (für Zellen)
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
        // Strobe: oberhalb der „offen“-Werte blitzen
        let strobe = by_kind(K::Strobe).first().map(|i| value(*i)).unwrap_or(0.0);
        let shutter_on = if strobe > 0.08 {
            let hz = 1.0 + (strobe - 0.08) / 0.92 * 19.0;
            (t * hz).fract() < 0.25
        } else {
            true
        };
        // Abstrahlwinkel aus den Gerätedaten (mit Zoomkanal zwischen min und max)
        let zoom = ty
            .channels
            .iter()
            .position(|c| c.name.to_lowercase().contains("zoom"))
            .map(value);
        let phys_half = ty.physical.map(|p| {
            let [a, b] = p.beam_angle;
            (a + (b - a) * zoom.unwrap_or(0.0)) * 0.5
        });
        let sel = selected.contains(&f.id);
        let body = if sel {
            Vec3::new(0.0, 0.45, 0.5)
        } else {
            Vec3::splat(0.03)
        };
        let mut parts = Vec::new();
        let mut beams = Vec::new();
        let trs = |s: Vec3, r: Quat, tr: Vec3| Mat4::from_scale_rotation_translation(s, r, tr);
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
                        color: on * power,
                        lens,
                    });
                }
            };
        let down = Vec3::NEG_Y;
        match kind {
            Kind::Par | Kind::Blinder => {
                let (r, len, half) = if kind == Kind::Blinder {
                    (0.1, 0.12, 30.0)
                } else {
                    (0.11, 0.28, 13.0)
                };
                let name = ty.model.to_lowercase();
                let half = phys_half.unwrap_or(if name.contains("cob") || name.contains("revue") {
                    25.0
                } else if name.contains("wash") {
                    18.0
                } else {
                    half
                });
                parts.push(Part {
                    prim: Prim::Cylinder,
                    matrix: base
                        * trs(
                            Vec3::new(r, len, r),
                            Quat::IDENTITY,
                            Vec3::new(0.0, -len * 0.5, 0.0),
                        ),
                    color: body,
                    gloss: 0.5,
                    emissive: 0.0,
                });
                // Bügel
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(r * 2.3, 0.02, 0.03),
                        Quat::IDENTITY,
                        Vec3::new(0.0, 0.01, 0.0),
                    ),
                    body,
                );
                let light = cell_light(&get, 0, 1);
                let o = base.transform_point3(Vec3::new(0.0, -len - 0.003, 0.0));
                let d = base.transform_vector3(down).normalize();
                add_lens(
                    base * trs(
                        Vec3::new(r * 0.9, 0.006, r * 0.9),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -len, 0.0),
                    ),
                    light,
                    o,
                    d,
                    half,
                    r * 0.9,
                    9.0,
                );
            }
            Kind::Bar => {
                let width = (cells as f32 * 0.22).clamp(0.5, 1.3);
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(width, 0.07, 0.1),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.035, 0.0),
                    ),
                    body,
                );
                for c in 0..cells {
                    let x = -width * 0.5 + width * (c as f32 + 0.5) / cells as f32;
                    let light = cell_light(&get, c, cells);
                    let o = base.transform_point3(Vec3::new(x, -0.072, 0.0));
                    let d = base.transform_vector3(down).normalize();
                    add_lens(
                        base * trs(
                            Vec3::new(0.04, 0.006, 0.04),
                            Quat::IDENTITY,
                            Vec3::new(x, -0.07, 0.0),
                        ),
                        light,
                        o,
                        d,
                        phys_half.unwrap_or(17.0),
                        0.04,
                        5.0,
                    );
                }
            }
            Kind::Moving => {
                // Pan/Tilt (mit Feinkanal), Mitte = 50 %
                let fine = |coarse: K, fine: K| {
                    let c = by_kind(coarse).first().map(|i| value(*i))?;
                    let f = by_kind(fine).first().map(|i| value(*i)).unwrap_or(0.0);
                    Some(c + f / 255.0)
                };
                let (pan_range, tilt_range) = ty
                    .physical
                    .map_or((540.0, 270.0), |p| (p.pan_range, p.tilt_range));
                let pan = (fine(K::Pan, K::PanFine).unwrap_or(0.5) - 0.5) * pan_range.to_radians();
                let tilt =
                    (fine(K::Tilt, K::TiltFine).unwrap_or(0.5) - 0.5) * tilt_range.to_radians();
                cube(
                    &mut parts,
                    base * trs(
                        Vec3::new(0.32, 0.1, 0.24),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.05, 0.0),
                    ),
                    body,
                );
                let yoke = base * Mat4::from_rotation_y(pan);
                for sx in [-1.0f32, 1.0] {
                    cube(
                        &mut parts,
                        yoke * trs(
                            Vec3::new(0.03, 0.24, 0.08),
                            Quat::IDENTITY,
                            Vec3::new(sx * 0.15, -0.21, 0.0),
                        ),
                        body,
                    );
                }
                let head = yoke
                    * Mat4::from_translation(Vec3::new(0.0, -0.3, 0.0))
                    * Mat4::from_rotation_x(tilt);
                parts.push(Part {
                    prim: Prim::Cylinder,
                    matrix: head * trs(Vec3::new(0.12, 0.26, 0.12), Quat::IDENTITY, Vec3::ZERO),
                    color: body,
                    gloss: 0.5,
                    emissive: 0.0,
                });
                let light = cell_light(&get, 0, cells.max(1));
                let o = head.transform_point3(Vec3::new(0.0, -0.135, 0.0));
                let d = head.transform_vector3(down).normalize();
                add_lens(
                    head * trs(
                        Vec3::new(0.1, 0.006, 0.1),
                        Quat::IDENTITY,
                        Vec3::new(0.0, -0.13, 0.0),
                    ),
                    light,
                    o,
                    d,
                    phys_half.unwrap_or(9.0),
                    0.08,
                    12.0,
                );
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
                // Nebelkanal (generisch/Dimmer): erhöht den Dunst im Raum
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
                let light = cell_light(&get, 0, 1);
                let light = if light.max_element() < 0.01 {
                    // Laser ohne Farbkanal: grün, wenn irgendein Kanal aktiv ist
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

fn cube(parts: &mut Vec<Part>, m: Mat4, c: Vec3) {
    parts.push(Part {
        prim: Prim::Cube,
        matrix: m,
        color: c,
        gloss: 0.4,
        emissive: 0.0,
    });
}

/// Linearfarbe aus sRGB-Bytes (für Materialien).
pub fn linear(c: [u8; 3]) -> Vec3 {
    srgb_to_linear(c)
}
