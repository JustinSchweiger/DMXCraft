//! Face handles in the builder: a small square on each side of the selected
//! object. Dragging it moves only that side – the object gets longer or
//! shorter in that direction while the opposite side stays where it is
//! (like pulling out a face). Boxes, planes, cylinders and trusses change
//! their size, models their scale.

use eframe::egui::{self, Pos2, Rect, Stroke};
use glam::{Mat4, Vec3};

use super::{object_box, project, SelFrame};
use crate::ui::theme;
use dmxcraft_core::venue::{Object, Shape};
use dmxcraft_core::Show;

/// Shortest length a side can be pulled to (m).
const MIN_LEN: f32 = 0.02;
const SIZE: f32 = 9.0;

/// A side of the selected object.
pub struct Face {
    /// Local axis (0 = x, 1 = y, 2 = z) and side (+1/−1).
    pub axis: usize,
    pub sign: f32,
    /// On screen.
    pub pos: Pos2,
    /// Screen movement per meter along the outward normal.
    pub per_meter: egui::Vec2,
    /// Outward normal (world, unit length) and length of the object along it.
    pub normal: Vec3,
    pub length: f32,
}

/// Dragging a side: the object as it was, its placement, the side.
pub struct FaceDrag {
    pub id: u32,
    pub axis: usize,
    pub sign: f32,
    pub start: Pos2,
    pub per_meter: egui::Vec2,
    pub normal: Vec3,
    pub length: f32,
    pub object: Object,
    pub frame: SelFrame,
}

/// Axes along which a shape can be pulled.
fn axes(shape: &Shape) -> &'static [usize] {
    match shape {
        Shape::Group => &[],
        Shape::Plane { .. } => &[0, 2],
        Shape::Truss { .. } => &[0],
        _ => &[0, 1, 2],
    }
}

/// The sides of an object (none when locked or a group).
pub fn faces(show: &Show, id: u32, vp: Mat4, rect: Rect) -> Vec<Face> {
    let Some((ob, m)) = show.venue.flatten().into_iter().find(|(o, _)| o.id == id) else {
        return Vec::new();
    };
    if ob.locked {
        return Vec::new();
    }
    let Some(b) = object_box(show, ob, m) else {
        return Vec::new();
    };
    let center = b.transform_point3(Vec3::ZERO);
    let mut out = Vec::new();
    for &axis in axes(&ob.shape) {
        let col = b.col(axis).truncate();
        let length = col.length();
        if length < 1e-4 {
            continue;
        }
        let normal = col / length;
        for sign in [-1.0f32, 1.0] {
            let at = center + normal * sign * length * 0.5;
            let (Some(pos), Some(ahead)) = (
                project(vp, rect, at),
                project(vp, rect, at + normal * sign * 0.1),
            ) else {
                continue;
            };
            out.push(Face {
                axis,
                sign,
                pos,
                per_meter: (ahead - pos) * 10.0,
                normal: normal * sign,
                length,
            });
        }
    }
    out
}

/// Side under the pointer.
pub fn hit(faces: &[Face], p: Pos2) -> Option<usize> {
    faces
        .iter()
        .enumerate()
        .map(|(i, f)| (i, f.pos.distance(p)))
        .filter(|(_, d)| *d < SIZE)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(i, _)| i)
}

pub fn paint(
    painter: &egui::Painter,
    faces: &[Face],
    hovered: Option<usize>,
    active: Option<(usize, f32)>,
) {
    for (i, f) in faces.iter().enumerate() {
        let on = hovered == Some(i) || active == Some((f.axis, f.sign));
        let r =
            Rect::from_center_size(f.pos, egui::Vec2::splat(if on { SIZE + 3.0 } else { SIZE }));
        painter.rect_filled(
            r,
            2.0,
            if on {
                theme::accent()
            } else {
                theme::surface()
            },
        );
        painter.rect_stroke(
            r,
            2.0,
            Stroke::new(1.5, theme::accent()),
            egui::StrokeKind::Inside,
        );
    }
}

impl FaceDrag {
    pub fn new(id: u32, f: &Face, start: Pos2, object: Object, frame: SelFrame) -> Self {
        Self {
            id,
            axis: f.axis,
            sign: f.sign,
            start,
            per_meter: f.per_meter,
            normal: f.normal,
            length: f.length,
            object,
            frame,
        }
    }

    /// The object with the side moved to the pointer `p` (`grid` m, 0 = free).
    pub fn apply(&self, p: Pos2, grid: f32) -> Object {
        let pm = self.per_meter;
        let along = (p - self.start).dot(pm) / pm.length_sq().max(1e-3);
        let mut len = (self.length + along).max(MIN_LEN);
        if grid > 0.0 {
            len = ((len / grid).round() * grid).max(MIN_LEN);
        }
        let k = len / self.length;
        let mut o = self.object.clone();
        let a = self.axis;
        match &mut o.shape {
            Shape::Box { size } => size[a] *= k,
            Shape::Plane { size } => size[if a == 0 { 0 } else { 1 }] *= k,
            Shape::Cylinder { radius, height } => {
                if a == 1 {
                    *height *= k;
                } else {
                    *radius *= k;
                }
            }
            Shape::Truss { length, .. } => *length *= k,
            Shape::Model { .. } => o.transform.scale[a] *= k,
            Shape::Group => {}
        }
        // the opposite side stays: the center moves by half the change
        let world = self.frame.world + self.normal * (len - self.length) * 0.5;
        let local = self.frame.parent.inverse().transform_point3(world);
        o.transform.position = local.to_array();
        o
    }
}
