//! Measuring in the 3D view (build mode, MEASURE in the toolbar): click two
//! points – corners, edge midpoints and face centers of objects, fixtures,
//! or any point on a surface – and see the distance and its parts along
//! x/y/z. Typing another value moves the object or fixture at one end so the
//! distance becomes that value (along the line, or along one axis).

use eframe::egui::{self, vec2, Color32, Pos2, Rect, Stroke};
use glam::{Mat4, Vec3};

use super::{move_all, object_box, project, rig, sel_frame, Pick, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::Segmented;
use crate::views::ViewCtx;
use dmxcraft_core::Show;

/// Snap distance to a point on screen (px).
const SNAP_PX: f32 = 12.0;

/// End of a measurement: the point and what it belongs to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct End {
    pub point: Vec3,
    pub pick: Option<Pick>,
}

#[derive(Default)]
pub struct Measure {
    pub on: bool,
    pub a: Option<End>,
    pub b: Option<End>,
    /// Point under the pointer (snapped).
    pub hover: Option<End>,
    /// Which end moves when a value is typed (false = B, true = A).
    pub move_a: bool,
}

/// Points of the venue to snap to: corners, edge midpoints, face centers and
/// centers of objects, positions of fixtures.
fn snap_points(show: &Show) -> Vec<End> {
    let mut out = Vec::new();
    for (ob, m) in show.venue.flatten() {
        if ob.hidden {
            continue;
        }
        let Some(b) = object_box(show, ob, m) else {
            continue;
        };
        let pick = Some(Pick::Object(ob.id));
        for x in [-0.5f32, 0.0, 0.5] {
            for y in [-0.5f32, 0.0, 0.5] {
                for z in [-0.5f32, 0.0, 0.5] {
                    // corners, edge midpoints, face centers, center
                    out.push(End {
                        point: b.transform_point3(Vec3::new(x, y, z)),
                        pick,
                    });
                }
            }
        }
    }
    for f in &show.fixtures {
        out.push(End {
            point: Vec3::from(rig::placement(show, f.id).position),
            pick: Some(Pick::Fixture(f.id)),
        });
    }
    out
}

impl Measure {
    /// Point under the pointer: the nearest snap point on screen, otherwise
    /// the surface the ray hits (and what it belongs to).
    pub fn hover_at(
        &mut self,
        show: &Show,
        vp: Mat4,
        rect: Rect,
        ptr: Option<Pos2>,
        ray: Option<(Vec3, Vec3)>,
    ) {
        self.hover = None;
        let (Some(p), Some((o, d))) = (ptr, ray) else {
            return;
        };
        let near = snap_points(show)
            .into_iter()
            .filter_map(|e| Some((e, project(vp, rect, e.point)?.distance(p))))
            .filter(|(_, dist)| *dist < SNAP_PX)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(e, _)| e);
        self.hover = near.or_else(|| {
            let point = super::surface_hit(show, o, d)?;
            Some(End {
                point,
                pick: VizView::pick(show, o, d),
            })
        });
    }

    /// A click: first end, second end, then start again.
    pub fn click(&mut self) {
        let Some(h) = self.hover else {
            return;
        };
        if self.a.is_none() || self.b.is_some() {
            self.a = Some(h);
            self.b = None;
        } else {
            self.b = Some(h);
        }
    }

    pub fn paint(&self, painter: &egui::Painter, vp: Mat4, rect: Rect) {
        if !self.on {
            return;
        }
        let dot = |e: &End, col: Color32| {
            if let Some(c) = project(vp, rect, e.point) {
                painter.circle_filled(c, 4.5, col);
                painter.circle_stroke(c, 4.5, Stroke::new(1.0, theme::bg()));
            }
        };
        if let Some(h) = &self.hover {
            if let Some(c) = project(vp, rect, h.point) {
                painter.circle_stroke(c, 7.0, Stroke::new(1.5, theme::accent()));
            }
        }
        let a = self.a;
        let b = self.b.or(if self.a.is_some() { self.hover } else { None });
        if let (Some(a), Some(b)) = (a, b) {
            // parts along x (red), z (blue), y (green) as a path
            let p0 = a.point;
            let p1 = Vec3::new(b.point.x, p0.y, p0.z);
            let p2 = Vec3::new(b.point.x, p0.y, b.point.z);
            for (s, e, col) in [
                (p0, p1, Color32::from_rgb(220, 80, 80)),
                (p1, p2, Color32::from_rgb(80, 130, 230)),
                (p2, b.point, Color32::from_rgb(80, 200, 110)),
            ] {
                if let (Some(s), Some(e)) = (project(vp, rect, s), project(vp, rect, e)) {
                    painter.line_segment([s, e], Stroke::new(1.0, col.gamma_multiply(0.7)));
                }
            }
            if let (Some(s), Some(e)) = (project(vp, rect, a.point), project(vp, rect, b.point)) {
                painter.line_segment([s, e], Stroke::new(2.0, theme::accent()));
                let d = b.point - a.point;
                let label = format!("{:.3} m", d.length());
                let mid = s + (e - s) * 0.5;
                let galley = painter.layout_no_wrap(label, theme::body(), theme::text());
                let r = Rect::from_center_size(mid, galley.size() + vec2(10.0, 4.0));
                painter.rect_filled(r, theme::R_S, theme::surface());
                painter.rect_stroke(
                    r,
                    theme::R_S,
                    Stroke::new(1.0, theme::accent()),
                    egui::StrokeKind::Inside,
                );
                painter.galley(r.min + vec2(5.0, 2.0), galley, theme::text());
            }
            dot(&a, theme::accent());
            dot(&b, theme::accent());
        } else if let Some(a) = &a {
            dot(a, theme::accent());
        }
        if self.a.is_none() {
            painter.text(
                rect.center_top() + vec2(0.0, 12.0),
                egui::Align2::CENTER_TOP,
                t("Click the first point"),
                theme::body(),
                theme::accent(),
            );
        } else if self.b.is_none() {
            painter.text(
                rect.center_top() + vec2(0.0, 12.0),
                egui::Align2::CENTER_TOP,
                t("Click the second point"),
                theme::body(),
                theme::accent(),
            );
        }
    }

    /// Values of the measurement to read and change (over the view).
    pub fn panel(&mut self, ui: &mut egui::Ui, rect: Rect, cx: &mut ViewCtx<'_>) {
        let (Some(a), Some(b)) = (self.a, self.b) else {
            return;
        };
        let d = b.point - a.point;
        let mut changed: Option<Vec3> = None;
        egui::Area::new(egui::Id::unique("viz_measure"))
            .fixed_pos(rect.left_bottom() + vec2(theme::SP_S, -theme::SP_S - 130.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(theme::surface())
                    .stroke(Stroke::new(1.0, theme::border()))
                    .corner_radius(theme::R_M)
                    .inner_margin(egui::Margin::same(8))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(t("Measurement"))
                                .font(theme::caption())
                                .color(theme::text_dim()),
                        );
                        egui::Grid::new("viz_measure_grid")
                            .num_columns(2)
                            .spacing(vec2(theme::SP_S, 4.0))
                            .show(ui, |ui| {
                                let mut len = d.length();
                                ui.label(t("Distance"));
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut len)
                                            .speed(0.005)
                                            .range(0.0..=200.0)
                                            .fixed_decimals(3)
                                            .suffix(" m"),
                                    )
                                    .changed()
                                {
                                    let dir = d.normalize_or(Vec3::Y);
                                    changed = Some(dir * len);
                                }
                                ui.end_row();
                                for (i, name) in ["X", "Y", "Z"].into_iter().enumerate() {
                                    let mut v = d[i];
                                    ui.label(name);
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut v)
                                                .speed(0.005)
                                                .fixed_decimals(3)
                                                .suffix(" m"),
                                        )
                                        .changed()
                                    {
                                        let mut n = d;
                                        n[i] = v;
                                        changed = Some(n);
                                    }
                                    ui.end_row();
                                }
                            });
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(t("Changes move"))
                                    .font(theme::caption())
                                    .color(theme::text_dim()),
                            );
                            Segmented::new(&[(false, t("Second point")), (true, t("First point"))])
                                .show(ui, &mut self.move_a);
                        });
                    });
            });
        let Some(want) = changed else {
            return;
        };
        // the moving end goes by the difference
        let (end, delta) = if self.move_a {
            (a, -(want - d))
        } else {
            (b, want - d)
        };
        if a.pick.is_some() && a.pick == b.pick {
            cx.state.error(t(
                "Both points belong to the same object – measure to another one",
            ));
            return;
        }
        let Some(pick) = end.pick else {
            cx.state
                .error(t("This point belongs to nothing that can be moved"));
            return;
        };
        let Some(frame) = sel_frame(cx.engine.show(), pick) else {
            cx.state.error(t("Locked – unlock it to move it"));
            return;
        };
        move_all(cx, &[(pick, frame)], delta);
        let moved = End {
            point: end.point + delta,
            pick: end.pick,
        };
        if self.move_a {
            self.a = Some(moved);
        } else {
            self.b = Some(moved);
        }
    }
}
