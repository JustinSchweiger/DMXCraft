//! Snapping in the builder: to a grid and to other objects. When moving, the
//! box around the selection is aligned with the boxes of the other objects
//! and fixtures (edge to edge, center to center, edge to center) – e.g. a
//! table against a wall, a fixture under a truss, several objects in a line.

use glam::{Mat4, Vec3};

use super::{object_box, rig, Pick};
use crate::i18n::t;
use dmxcraft_core::Show;

/// Half size of the box around a fixture (m).
const FIXTURE_HALF: f32 = 0.15;

/// Snapping settings of the builder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Snap {
    /// Grid in m (0 = off).
    pub grid: f32,
    /// Rotation steps in degrees (0 = off).
    pub angle: f32,
    /// Align with other objects and fixtures.
    pub objects: bool,
}

impl Default for Snap {
    fn default() -> Self {
        Self {
            grid: 0.05,
            angle: 5.0,
            objects: true,
        }
    }
}

pub const GRIDS: [f32; 7] = [0.0, 0.01, 0.05, 0.1, 0.25, 0.5, 1.0];
pub const ANGLES: [f32; 6] = [0.0, 1.0, 5.0, 15.0, 45.0, 90.0];

pub fn grid_label(g: f32) -> String {
    if g <= 0.0 {
        t("Off").to_string()
    } else if g >= 1.0 {
        format!("{g} m")
    } else {
        format!("{} cm", (g * 100.0).round())
    }
}

pub fn angle_label(a: f32) -> String {
    if a <= 0.0 {
        t("Off").to_string()
    } else {
        format!("{a}°")
    }
}

/// Axis-aligned box in the room.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    fn of(m: Mat4) -> Self {
        let mut min = Vec3::splat(f32::MAX);
        let mut max = Vec3::splat(f32::MIN);
        for i in 0..8 {
            let p = m.transform_point3(Vec3::new(
                if i & 1 == 0 { -0.5 } else { 0.5 },
                if i & 2 == 0 { -0.5 } else { 0.5 },
                if i & 4 == 0 { -0.5 } else { 0.5 },
            ));
            min = min.min(p);
            max = max.max(p);
        }
        Self { min, max }
    }

    fn union(self, o: Self) -> Self {
        Self {
            min: self.min.min(o.min),
            max: self.max.max(o.max),
        }
    }

    fn moved(self, d: Vec3) -> Self {
        Self {
            min: self.min + d,
            max: self.max + d,
        }
    }

    /// Unit cube → this box (for drawing).
    pub fn matrix(self) -> Mat4 {
        Mat4::from_scale_rotation_translation(
            (self.max - self.min).max(Vec3::splat(0.01)),
            glam::Quat::IDENTITY,
            (self.min + self.max) * 0.5,
        )
    }

    fn features(self, a: usize) -> [f32; 3] {
        [self.min[a], (self.min[a] + self.max[a]) * 0.5, self.max[a]]
    }
}

/// What is moved and what it can snap to (taken when dragging starts).
pub struct SnapStart {
    /// Box around everything that moves.
    pub moving: Aabb,
    /// World position of the main selection.
    pub origin: Vec3,
    /// Boxes of everything else.
    pub others: Vec<Aabb>,
}

impl SnapStart {
    pub fn new(show: &Show, moving: &[Pick], origin: Vec3) -> Option<Self> {
        let moved_obj = |id: u32| {
            moving.iter().any(|p| match p {
                Pick::Object(m) => *m == id || show.venue.ancestors(id).contains(m),
                Pick::Fixture(_) => false,
            })
        };
        let mut mine: Option<Aabb> = None;
        let mut others = Vec::new();
        for (ob, m) in show.venue.flatten() {
            let Some(b) = object_box(show, ob, m) else {
                continue;
            };
            let bb = Aabb::of(b);
            if moved_obj(ob.id) {
                mine = Some(mine.map_or(bb, |x| x.union(bb)));
            } else {
                others.push(bb);
            }
        }
        for f in &show.fixtures {
            let p = Vec3::from(rig::placement(show, f.id).position);
            let bb = Aabb {
                min: p - Vec3::splat(FIXTURE_HALF),
                max: p + Vec3::splat(FIXTURE_HALF),
            };
            if moving.contains(&Pick::Fixture(f.id)) {
                mine = Some(mine.map_or(bb, |x| x.union(bb)));
            } else {
                others.push(bb);
            }
        }
        Some(Self {
            moving: mine?,
            origin,
            others,
        })
    }

    /// Snap a movement `delta` (world) on the given axes: to other objects
    /// within `tolerance` (m), otherwise the main selection to the grid.
    /// Returns the movement and the boxes it snapped to.
    pub fn apply(
        &self,
        delta: Vec3,
        axes: [bool; 3],
        snap: Snap,
        tolerance: f32,
    ) -> (Vec3, Vec<Aabb>) {
        let mut out = delta;
        let mut guides = Vec::new();
        let moved = self.moving.moved(delta);
        for a in (0..3).filter(|a| axes[*a]) {
            let mut best: Option<(f32, usize)> = None;
            if snap.objects {
                let mine = moved.features(a);
                for (i, o) in self.others.iter().enumerate() {
                    for r in o.features(a) {
                        for s in mine {
                            let d = r - s;
                            if d.abs() <= tolerance && best.is_none_or(|(b, _)| d.abs() < b.abs()) {
                                best = Some((d, i));
                            }
                        }
                    }
                }
            }
            if let Some((d, i)) = best {
                out[a] += d;
                guides.push(self.others[i]);
            } else if snap.grid > 0.0 {
                let p = self.origin[a] + delta[a];
                out[a] = (p / snap.grid).round() * snap.grid - self.origin[a];
            }
        }
        (out, guides)
    }
}

/// Snap an angle (degrees) to the steps.
pub fn angle(deg: f32, snap: Snap) -> f32 {
    if snap.angle > 0.0 {
        (deg / snap.angle).round() * snap.angle
    } else {
        deg
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges_meet_and_grid_otherwise() {
        let start = SnapStart {
            moving: Aabb {
                min: Vec3::new(0.0, 0.0, 0.0),
                max: Vec3::new(1.0, 1.0, 1.0),
            },
            origin: Vec3::new(0.5, 0.5, 0.5),
            others: vec![Aabb {
                min: Vec3::new(3.0, 0.0, 0.0),
                max: Vec3::new(4.0, 1.0, 1.0),
            }],
        };
        let snap = Snap::default();
        // right edge 1.93 + 0.0 → close to the other's left edge at 3.0
        let (d, g) = start.apply(Vec3::new(1.93, 0.0, 0.0), [true, false, false], snap, 0.1);
        assert!((d.x - 2.0).abs() < 1e-5);
        assert_eq!(g.len(), 1);
        // far away: grid (5 cm) for the main selection
        let (d, g) = start.apply(Vec3::new(0.73, 0.0, 0.0), [true, false, false], snap, 0.1);
        assert!((start.origin.x + d.x - 1.25).abs() < 1e-5);
        assert!(g.is_empty());
        // without snapping to objects: only the grid
        let off = Snap {
            objects: false,
            ..snap
        };
        let (d, _) = start.apply(Vec3::new(1.93, 0.0, 0.0), [true, false, false], off, 0.1);
        assert!((start.origin.x + d.x - 2.45).abs() < 1e-5);
    }
}
