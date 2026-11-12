//! LIVE TRACKING (in place of the stage plan): moving heads follow points
//! in the room. Left the trackers with their faders (all mappable to
//! MIDI/keys), right the room from above with the fixtures and the targets
//! to drag.

use crate::i18n::t;

use dmxcraft_core::{Engine, FixtureId};
use eframe::egui::{self, pos2, vec2, Color32, Rect, Sense, Stroke, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{text_field, Icon, IconButton, Toggle};

/// Highest target height (m).
const MAX_HEIGHT: f32 = 3.0;
const LIST_W: f32 = 290.0;

#[derive(Default)]
pub struct TrackingView {
    /// Tracker that a click in the room moves.
    current: Option<u32>,
    rename: Option<(u32, String)>,
}

pub fn color(show: &dmxcraft_core::Show, id: u32) -> Color32 {
    let i = show.trackers.iter().position(|t| t.id == id).unwrap_or(0);
    theme::rgb(theme::ZONE_PALETTE[i % theme::ZONE_PALETTE.len()])
}

impl TrackingView {
    pub fn ui(&mut self, ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
        if self.current.is_none_or(|c| e.show().tracker(c).is_none()) {
            self.current = e.show().trackers.first().map(|t| t.id);
        }
        let area = ui.available_rect_before_wrap();
        let list_rect = Rect::from_min_size(
            area.min,
            vec2(LIST_W.min(area.width() * 0.5), area.height()),
        );
        let room_rect =
            Rect::from_min_max(pos2(list_rect.right() + theme::SP_S, area.top()), area.max);
        let mut list = ui.new_child(egui::UiBuilder::new().max_rect(list_rect));
        egui::ScrollArea::vertical()
            .id_salt("trackers")
            .auto_shrink([false, false])
            .show(&mut list, |ui| self.list(ui, e, st));
        let mut room = ui.new_child(egui::UiBuilder::new().max_rect(room_rect));
        self.room(&mut room, e);
        ui.allocate_rect(area, Sense::hover());
    }

    fn list(&mut self, ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
        let locked = st.locked;
        let ids: Vec<u32> = e.show().trackers.iter().map(|t| t.id).collect();
        let mut delete = None;
        for id in ids {
            let Some(tr) = e.show().tracker(id).cloned() else {
                continue;
            };
            let col = color(e.show(), id);
            let current = self.current == Some(id);
            let frame = egui::Frame::new()
                .fill(if current {
                    theme::raised()
                } else {
                    theme::surface()
                })
                .stroke(Stroke::new(
                    1.0,
                    if current { col } else { theme::border() },
                ))
                .corner_radius(theme::R_S)
                .inner_margin(egui::Margin::same(6));
            let r = frame.show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (dot, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
                    ui.painter().circle_filled(dot.center(), 5.0, col);
                    if let Some((_, text)) = self.rename.as_mut().filter(|(r, _)| *r == id) {
                        let r = ui.add(text_field(text).desired_width(120.0));
                        r.request_focus();
                        if r.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let name = text.trim().to_string();
                            if !name.is_empty() {
                                if let Some(t) = e.show_mut().tracker_mut(id) {
                                    t.name = name;
                                }
                            }
                            self.rename = None;
                        }
                    } else {
                        let r = ui.add(
                            egui::Label::new(egui::RichText::new(&tr.name).strong())
                                .sense(Sense::click()),
                        );
                        if r.clicked() {
                            self.current = Some(id);
                        }
                        if r.double_clicked() && !locked {
                            self.rename = Some((id, tr.name.clone()));
                        }
                    }
                    ui.label(
                        egui::RichText::new(tr!("{n} fixtures", n = tr.fixtures.len()))
                            .font(theme::caption())
                            .color(theme::text_dim()),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let menu_btn = ui.add(IconButton::new(Icon::Menu).size(22.0).flat());
                        menu::dropdown(&menu_btn, |ui| {
                            menu::begin(ui);
                            let some = !st.selected.is_empty();
                            if menu::item(
                                ui,
                                MenuItem::new(t("Use selected fixtures"))
                                    .icon(Icon::Plus)
                                    .enabled(some && !locked),
                            ) {
                                let sel = st.selection_order.clone();
                                if let Some(t) = e.show_mut().tracker_mut(id) {
                                    t.fixtures = sel;
                                }
                            }
                            if menu::item(
                                ui,
                                MenuItem::new(t("Select its fixtures")).icon(Icon::Cursor),
                            ) {
                                st.selected = tr.fixtures.iter().copied().collect();
                                st.selection_order = tr.fixtures.clone();
                            }
                            if menu::item(
                                ui,
                                MenuItem::new(t("Rename")).icon(Icon::Edit).enabled(!locked),
                            ) {
                                self.rename = Some((id, tr.name.clone()));
                            }
                            menu::separator(ui);
                            if menu::item(
                                ui,
                                MenuItem::new(t("Delete"))
                                    .icon(Icon::Trash)
                                    .danger()
                                    .enabled(!locked),
                            ) {
                                delete = Some(id);
                            }
                        });
                        let mut on = tr.active;
                        let key = format!("tracking/{id}/follow");
                        if ui
                            .add(Toggle::new(&mut on, t("Follow")).map(
                                key,
                                t("Tracking").to_string(),
                                tr!("{name}: follow", name = tr.name),
                            ))
                            .changed()
                        {
                            if let Some(t) = e.show_mut().tracker_mut(id) {
                                t.active = on;
                            }
                        }
                    });
                });
                let (xr, zr) = e.show().tracking_area();
                let norm = |v: f32, r: [f32; 2]| ((v - r[0]) / (r[1] - r[0])).clamp(0.0, 1.0);
                let rows = [
                    (t("Left / right"), "x", norm(tr.target[0], xr)),
                    (t("Front / back"), "z", norm(tr.target[2], zr)),
                    (
                        t("Height"),
                        "y",
                        (tr.target[1] / MAX_HEIGHT).clamp(0.0, 1.0),
                    ),
                ];
                for (label, axis, value) in rows {
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            vec2(86.0, 20.0),
                            egui::Label::new(
                                egui::RichText::new(label)
                                    .font(theme::caption())
                                    .color(theme::text_dim()),
                            ),
                        );
                        let mut v = value;
                        let start = ui.cursor().min;
                        let changed = super::scene_props::bar_slider(
                            ui,
                            &format!("track_{id}_{axis}"),
                            &mut v,
                            true,
                            None,
                            col,
                        );
                        let rect =
                            Rect::from_min_size(start, vec2(ui.min_rect().right() - start.x, 22.0));
                        let key = format!("tracking/{id}/{axis}");
                        let mapped = crate::mapping::fader(ui, rect, &key, v, || {
                            (t("Tracking").to_string(), format!("{}: {label}", tr.name))
                        });
                        if changed || mapped.is_some() {
                            let v = mapped.unwrap_or(v);
                            if let Some(t) = e.show_mut().tracker_mut(id) {
                                match axis {
                                    "x" => t.target[0] = xr[0] + v * (xr[1] - xr[0]),
                                    "z" => t.target[2] = zr[0] + v * (zr[1] - zr[0]),
                                    _ => t.target[1] = v * MAX_HEIGHT,
                                }
                            }
                            self.current = Some(id);
                        }
                    });
                }
            });
            if r.response.interact(Sense::click()).clicked() {
                self.current = Some(id);
            }
            ui.add_space(theme::SP_XS);
        }
        if let Some(id) = delete {
            e.show_mut().remove_tracker(id);
        }
        if !locked {
            let some = !st.selected.is_empty();
            let r = ui
                .add_enabled(some, egui::Button::new(t("New tracker from selection")))
                .on_disabled_hover_text(t("Select the moving heads first"));
            if r.clicked() {
                let n = e.show().trackers.len() + 1;
                let id = e
                    .show_mut()
                    .add_tracker(tr!("Target {n}", n = n), st.selection_order.clone());
                self.current = Some(id);
            }
        }
    }

    /// The room from above: fixtures of the trackers and the targets.
    fn room(&mut self, ui: &mut egui::Ui, e: &mut Engine) {
        let area = ui.available_rect_before_wrap();
        let (xr, zr) = e.show().tracking_area();
        let (w, d) = (xr[1] - xr[0], zr[1] - zr[0]);
        let scale = (area.width() / w).min(area.height() / d);
        let room = Rect::from_center_size(area.center(), vec2(w * scale, d * scale));
        // x to the right, z downwards (like the 3D view from above)
        let to_screen = |x: f32, z: f32| {
            pos2(
                room.left() + (x - xr[0]) * scale,
                room.top() + (z - zr[0]) * scale,
            )
        };
        let to_room = |p: egui::Pos2| {
            [
                (xr[0] + (p.x - room.left()) / scale).clamp(xr[0], xr[1]),
                (zr[0] + (p.y - room.top()) / scale).clamp(zr[0], zr[1]),
            ]
        };
        let resp = ui.interact(
            room,
            ui.make_persistent_id("tracking_room"),
            Sense::click_and_drag(),
        );
        let p = ui.painter_at(area);
        p.rect_filled(room, theme::R_S, theme::bg());
        p.rect_stroke(
            room,
            theme::R_S,
            Stroke::new(1.0, theme::border()),
            egui::StrokeKind::Inside,
        );
        let show = e.show();
        // all moving heads faint, those of trackers in their color
        let mut owner: std::collections::HashMap<FixtureId, Color32> = Default::default();
        for tr in &show.trackers {
            for f in &tr.fixtures {
                owner.entry(*f).or_insert(color(show, tr.id));
            }
        }
        for f in &show.fixtures {
            let movable = show.type_of(f).is_some_and(|ty| {
                ty.has_kind(dmxcraft_core::fixture::ChannelKind::Pan)
                    && ty.has_kind(dmxcraft_core::fixture::ChannelKind::Tilt)
            });
            if !movable {
                continue;
            }
            let pos = show.placement3d(f.id).position;
            let c = to_screen(pos[0], pos[2]);
            match owner.get(&f.id) {
                Some(col) => p.circle_filled(c, 5.0, *col),
                None => p.circle_stroke(c, 4.0, Stroke::new(1.0, theme::text_faint())),
            };
        }
        // targets with lines to their fixtures
        for tr in &show.trackers {
            let col = color(show, tr.id);
            let c = to_screen(tr.target[0], tr.target[2]);
            let dim = if tr.active { 1.0 } else { 0.35 };
            for f in &tr.fixtures {
                let pos = show.placement3d(*f).position;
                p.line_segment(
                    [to_screen(pos[0], pos[2]), c],
                    Stroke::new(1.0, col.gamma_multiply(0.35 * dim)),
                );
            }
            let r = 9.0;
            p.circle_stroke(c, r, Stroke::new(2.0, col.gamma_multiply(dim)));
            p.line_segment(
                [c - vec2(r + 4.0, 0.0), c + vec2(r + 4.0, 0.0)],
                Stroke::new(1.5, col.gamma_multiply(dim)),
            );
            p.line_segment(
                [c - vec2(0.0, r + 4.0), c + vec2(0.0, r + 4.0)],
                Stroke::new(1.5, col.gamma_multiply(dim)),
            );
            p.text(
                c + vec2(r + 6.0, -r - 4.0),
                egui::Align2::LEFT_BOTTOM,
                format!("{} · {:.1} m", tr.name, tr.target[1]),
                theme::caption(),
                col.gamma_multiply(dim.max(0.7)),
            );
        }
        if show.trackers.is_empty() {
            p.text(
                room.center(),
                egui::Align2::CENTER_CENTER,
                t("Select moving heads and create a tracker"),
                theme::body(),
                theme::text_faint(),
            );
        }
        // click/drag: the nearest target, otherwise the current one
        if resp.drag_started() || resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let near = show
                    .trackers
                    .iter()
                    .map(|tr| (tr.id, to_screen(tr.target[0], tr.target[2]).distance(pos)))
                    .filter(|(_, d)| *d < 18.0)
                    .min_by(|a, b| a.1.total_cmp(&b.1))
                    .map(|(id, _)| id);
                if near.is_some() {
                    self.current = near;
                }
            }
        }
        if resp.dragged() || resp.clicked() {
            if let (Some(pos), Some(id)) = (resp.interact_pointer_pos(), self.current) {
                let [x, z] = to_room(pos);
                if let Some(t) = e.show_mut().tracker_mut(id) {
                    t.target[0] = x;
                    t.target[2] = z;
                }
            }
        }
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }
    }
}
