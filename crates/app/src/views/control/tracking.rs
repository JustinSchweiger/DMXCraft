//! LIVE TRACKING (in place of the stage plan): moving heads follow points
//! in the room. Left the targets with their faders (all mappable to
//! MIDI/keys), right the room from above with the venue, the fixtures and
//! the targets to drag.

use crate::i18n::t;

use dmxcraft_core::tracking::Tracker;
use dmxcraft_core::{Engine, FixtureId};
use eframe::egui::{self, pos2, vec2, Color32, Pos2, Rect, Sense, Stroke, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::color_picker::color_swatches;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{text_field, Icon, IconButton, Toggle};

/// Highest target height (m).
const MAX_HEIGHT: f32 = 3.0;
const LIST_W: f32 = 310.0;
const LABEL_W: f32 = 78.0;

#[derive(Default)]
pub struct TrackingView {
    /// Target that a click in the room moves.
    current: Option<u32>,
    rename: Option<(u32, String)>,
}

pub fn color(show: &dmxcraft_core::Show, id: u32) -> Color32 {
    let i = show.trackers.iter().position(|t| t.id == id).unwrap_or(0);
    theme::rgb(theme::ZONE_PALETTE[i % theme::ZONE_PALETTE.len()])
}

/// A labelled bar slider that can be mapped; `Some(value)` when it changed.
fn bar_row(
    ui: &mut egui::Ui,
    label: &str,
    key: &str,
    target: &str,
    value: f32,
    col: Color32,
) -> Option<f32> {
    let mut out = None;
    ui.horizontal(|ui| {
        ui.add_sized(
            vec2(LABEL_W, 20.0),
            egui::Label::new(
                egui::RichText::new(label)
                    .font(theme::caption())
                    .color(theme::text_dim()),
            ),
        );
        let mut v = value;
        let start = ui.cursor().min;
        let changed = super::scene_props::bar_slider(ui, key, &mut v, true, None, col);
        let rect = Rect::from_min_size(start, vec2(ui.min_rect().right() - start.x, 22.0));
        let mapped = crate::mapping::fader(ui, rect, key, v, || {
            (t("Tracking").to_string(), target.to_string())
        });
        if let Some(m) = mapped {
            out = Some(m);
        } else if changed {
            out = Some(v);
        }
    });
    out
}

fn edit(e: &mut Engine, id: u32, f: impl FnOnce(&mut Tracker)) {
    if let Some(t) = e.show_mut().tracker_mut(id) {
        f(t);
    }
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
                self.header(ui, e, st, &tr, col, &mut delete);
                self.controls(ui, e, &tr, col);
            });
            // a click on the card (not on its controls) picks the target;
            // checked by position so the controls keep their own clicks
            let clicked_card = ui.input(|i| {
                i.pointer.primary_clicked()
                    && i.pointer
                        .interact_pos()
                        .is_some_and(|p| r.response.rect.contains(p))
            });
            if clicked_card {
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
                    .add_tracker(crate::tr!("Target {n}", n = n), st.selection_order.clone());
                self.current = Some(id);
            }
        }
    }

    /// Position faders and the own light of a target.
    fn controls(&mut self, ui: &mut egui::Ui, e: &mut Engine, tr: &Tracker, col: Color32) {
        let id = tr.id;
        let name = &tr.name;
        let (xr, zr) = e.show().tracking_area();
        let norm = |v: f32, r: [f32; 2]| ((v - r[0]) / (r[1] - r[0])).clamp(0.0, 1.0);
        let label = |l: &str| format!("{name}: {l}");
        if let Some(v) = bar_row(
            ui,
            t("Left / right"),
            &format!("tracking/{id}/x"),
            &label(t("Left / right")),
            norm(tr.target[0], xr),
            col,
        ) {
            edit(e, id, |t| t.target[0] = xr[0] + v * (xr[1] - xr[0]));
            self.current = Some(id);
        }
        if let Some(v) = bar_row(
            ui,
            t("Front / back"),
            &format!("tracking/{id}/z"),
            &label(t("Front / back")),
            norm(tr.target[2], zr),
            col,
        ) {
            edit(e, id, |t| t.target[2] = zr[0] + v * (zr[1] - zr[0]));
            self.current = Some(id);
        }
        if let Some(v) = bar_row(
            ui,
            t("Height"),
            &format!("tracking/{id}/y"),
            &label(t("Height")),
            (tr.target[1] / MAX_HEIGHT).clamp(0.0, 1.0),
            col,
        ) {
            edit(e, id, |t| t.target[1] = v * MAX_HEIGHT);
        }
        ui.add_space(theme::SP_XS);
        // own light: dimmer, color
        ui.horizontal(|ui| {
            let mut light = tr.light;
            if ui
                .add(Toggle::new(&mut light, t("Own light")).map(
                    format!("tracking/{id}/light"),
                    t("Tracking").to_string(),
                    label(t("Own light")),
                ))
                .on_hover_text(t(
                    "Dimmer and color from the target instead of the scenes – the dimmer comes up once the heads have arrived",
                ))
                .changed()
            {
                edit(e, id, |t| t.light = light);
            }
            if tr.light {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let (rect, resp) = ui.allocate_exact_size(vec2(36.0, 20.0), Sense::click());
                    ui.painter()
                        .rect_filled(rect, theme::R_S, theme::rgb(tr.color));
                    ui.painter().rect_stroke(
                        rect,
                        theme::R_S,
                        Stroke::new(1.0, theme::border()),
                        egui::StrokeKind::Outside,
                    );
                    let resp = resp.on_hover_text(t("Color"));
                    menu::dropdown(&resp, |ui| {
                        if let Some(pick) = color_swatches(
                            ui,
                            ui.make_persistent_id(("track_color", id)),
                            Some(tr.color),
                            t("White"),
                        ) {
                            let rgb = pick.unwrap_or([255, 255, 255]);
                            edit(e, id, |t| t.color = rgb);
                        }
                    });
                });
            }
        });
        if tr.light {
            if let Some(v) = bar_row(
                ui,
                t("Dimmer"),
                &format!("tracking/{id}/dimmer"),
                &label(t("Dimmer")),
                tr.dimmer,
                col,
            ) {
                edit(e, id, |t| t.dimmer = v);
            }
            // color cells (rings): which ones get the color
            let cells = e.show().tracker_cells(id).min(32);
            if cells > 1 {
                ui.horizontal(|ui| {
                    ui.add_sized(
                        vec2(LABEL_W, 20.0),
                        egui::Label::new(
                            egui::RichText::new(t("Color on"))
                                .font(theme::caption())
                                .color(theme::text_dim()),
                        ),
                    )
                    .on_hover_text(t(
                        "Color cells (rings) that get the color – the others stay dark",
                    ));
                    for n in 0..cells {
                        let on = tr.cells & (1 << n) != 0;
                        let r = ui.add(egui::Button::new((n + 1).to_string()).selected(on));
                        let mapped = crate::mapping::button(
                            ui,
                            &r,
                            &format!("tracking/{id}/cell{}", n + 1),
                            on,
                            || {
                                (
                                    t("Tracking").to_string(),
                                    crate::tr!(
                                        "{name}: cell {n}",
                                        name = tr.name.clone(),
                                        n = n + 1
                                    ),
                                )
                            },
                        );
                        if r.clicked() || mapped {
                            // only the used cells count; never all off
                            let used = if cells >= 32 {
                                u32::MAX
                            } else {
                                (1u32 << cells) - 1
                            };
                            let mask = (tr.cells & used) ^ (1 << n);
                            if mask != 0 {
                                edit(e, id, |t| {
                                    t.cells = if mask == used { u32::MAX } else { mask }
                                });
                            }
                        }
                    }
                });
            }
        }
        // zoom: independent of the own light
        ui.horizontal(|ui| {
            let mut own = tr.own_zoom;
            if ui
                .add(Toggle::new(&mut own, t("Own zoom")).map(
                    format!("tracking/{id}/own_zoom"),
                    t("Tracking").to_string(),
                    label(t("Own zoom")),
                ))
                .on_hover_text(t("Zoom from the target instead of the scenes"))
                .changed()
            {
                edit(e, id, |t| t.own_zoom = own);
            }
        });
        if tr.own_zoom {
            if let Some(v) = bar_row(
                ui,
                t("Zoom"),
                &format!("tracking/{id}/zoom"),
                &label(t("Zoom")),
                tr.zoom,
                col,
            ) {
                edit(e, id, |t| t.zoom = v);
            }
        }
    }

    /// Name, fixture count, Follow switch and menu of a target.
    fn header(
        &mut self,
        ui: &mut egui::Ui,
        e: &mut Engine,
        st: &mut AppState,
        tr: &Tracker,
        col: Color32,
        delete: &mut Option<u32>,
    ) {
        let id = tr.id;
        let locked = st.locked;
        ui.horizontal(|ui| {
            let (dot, _) = ui.allocate_exact_size(Vec2::splat(12.0), Sense::hover());
            ui.painter().circle_filled(dot.center(), 5.0, col);
            if let Some((_, text)) = self.rename.as_mut().filter(|(r, _)| *r == id) {
                let r = ui.add(text_field(text).desired_width(120.0));
                r.request_focus();
                if r.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let name = text.trim().to_string();
                    if !name.is_empty() {
                        edit(e, id, |t| t.name = name);
                    }
                    self.rename = None;
                }
            } else {
                let r = ui.add(
                    egui::Label::new(egui::RichText::new(&tr.name).strong()).sense(Sense::click()),
                );
                if r.clicked() {
                    self.current = Some(id);
                }
                if r.double_clicked() && !locked {
                    self.rename = Some((id, tr.name.clone()));
                }
            }
            ui.label(
                egui::RichText::new(crate::tr!("{n} fixtures", n = tr.fixtures.len()))
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
                        edit(e, id, |t| t.fixtures = sel);
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
                        *delete = Some(id);
                    }
                });
                let mut on = tr.active;
                if ui
                    .add(Toggle::new(&mut on, t("Follow")).map(
                        format!("tracking/{id}/follow"),
                        t("Tracking").to_string(),
                        crate::tr!("{name}: follow", name = tr.name),
                    ))
                    .changed()
                {
                    edit(e, id, |t| t.active = on);
                }
            });
        });
    }

    /// The room from above: venue, moving heads, beams and targets.
    fn room(&mut self, ui: &mut egui::Ui, e: &mut Engine) {
        let area = ui.available_rect_before_wrap().shrink(2.0);
        let map = RoomMap::new(e.show(), area);
        let room = map.room;
        let resp = ui.interact(
            room,
            ui.make_persistent_id("tracking_room"),
            Sense::click_and_drag(),
        );
        let p = ui.painter_at(area.expand(2.0));
        map.paint(&p, e.show(), self.current, true);
        let show = e.show();
        if show.trackers.is_empty() {
            p.text(
                room.center(),
                egui::Align2::CENTER_CENTER,
                t("Select moving heads and create a tracker"),
                theme::body(),
                theme::text_dim(),
            );
        }
        // click/drag: the nearest target, otherwise the current one
        if resp.drag_started() || resp.clicked() {
            if let Some(near) = resp
                .interact_pointer_pos()
                .and_then(|pos| map.target_at(show, pos, 20.0))
            {
                self.current = Some(near);
            }
        }
        if resp.dragged() || resp.clicked() {
            if let (Some(pos), Some(id)) = (resp.interact_pointer_pos(), self.current) {
                let [x, z] = map.to_room(pos);
                edit(e, id, |t| {
                    t.target[0] = x;
                    t.target[2] = z;
                });
            }
        }
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
            // mouse wheel: height of the target under the cursor, otherwise
            // of the current one (one notch = 10 cm)
            let wheel = ui.input(|i| i.smooth_scroll_delta.y);
            if wheel != 0.0 {
                let under = resp
                    .hover_pos()
                    .and_then(|pos| map.target_at(e.show(), pos, 20.0));
                if let Some(id) = under.or(self.current) {
                    edit(e, id, |t| {
                        t.target[1] = (t.target[1] + wheel * 0.00125).clamp(0.0, MAX_HEIGHT);
                    });
                }
                ui.input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
            }
        }
    }
}

/// The room from above, fitted into a rectangle (shared by the tracking view
/// and the touch surface).
pub struct RoomMap {
    pub room: Rect,
    scale: f32,
    turn: bool,
    xr: [f32; 2],
    zr: [f32; 2],
}

impl RoomMap {
    pub fn new(show: &dmxcraft_core::Show, area: Rect) -> Self {
        let (xr, zr) = show.tracking_area();
        let (w, d) = ((xr[1] - xr[0]).max(0.1), (zr[1] - zr[0]).max(0.1));
        // Long side of the room along the long side of the area: turned by
        // 90° (stage side to the right) when that gives a larger plan
        let upright = (area.width() / w).min(area.height() / d);
        let turned = (area.width() / d).min(area.height() / w);
        let turn = turned > upright * 1.05;
        let scale = upright.max(turned) * 0.97;
        let size = if turn {
            vec2(d * scale, w * scale)
        } else {
            vec2(w * scale, d * scale)
        };
        Self {
            room: Rect::from_center_size(area.center(), size),
            scale,
            turn,
            xr,
            zr,
        }
    }

    /// upright: x to the right, z downwards (like the 3D view from above);
    /// turned: z to the right, x upwards
    pub fn to_screen(&self, x: f32, z: f32) -> Pos2 {
        let (room, s) = (self.room, self.scale);
        if self.turn {
            pos2(
                room.left() + (z - self.zr[0]) * s,
                room.bottom() - (x - self.xr[0]) * s,
            )
        } else {
            pos2(
                room.left() + (x - self.xr[0]) * s,
                room.top() + (z - self.zr[0]) * s,
            )
        }
    }

    pub fn to_room(&self, p: Pos2) -> [f32; 2] {
        let (room, s) = (self.room, self.scale);
        let (x, z) = if self.turn {
            (
                self.xr[0] + (room.bottom() - p.y) / s,
                self.zr[0] + (p.x - room.left()) / s,
            )
        } else {
            (
                self.xr[0] + (p.x - room.left()) / s,
                self.zr[0] + (p.y - room.top()) / s,
            )
        };
        [
            x.clamp(self.xr[0], self.xr[1]),
            z.clamp(self.zr[0], self.zr[1]),
        ]
    }

    /// Target nearest to `pos` (within `radius` pixels).
    pub fn target_at(&self, show: &dmxcraft_core::Show, pos: Pos2, radius: f32) -> Option<u32> {
        show.trackers
            .iter()
            .map(|tr| {
                (
                    tr.id,
                    self.to_screen(tr.target[0], tr.target[2]).distance(pos),
                )
            })
            .filter(|(_, d)| *d < radius)
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(id, _)| id)
    }

    /// Venue, beams, moving heads and targets; `labels`: names and heights
    /// next to the targets.
    pub fn paint(
        &self,
        p: &egui::Painter,
        show: &dmxcraft_core::Show,
        current: Option<u32>,
        labels: bool,
    ) {
        let room = self.room;
        let to_screen = |x: f32, z: f32| self.to_screen(x, z);
        p.rect_filled(room, theme::R_S, theme::bg());
        // venue from above: low objects darker, high ones lighter; ceilings
        // and anything above the heads are left out
        let head_y = show
            .fixtures
            .iter()
            .map(|f| show.placement3d(f.id).position[1])
            .fold(0.0f32, f32::max)
            .max(2.5);
        let mut shapes = crate::viz::floor_plan(show);
        shapes.retain(|s| s.bottom < head_y - 0.3 || s.truss);
        shapes.sort_by(|a, b| a.top.total_cmp(&b.top));
        for s in &shapes {
            let pts: Vec<Pos2> = s.outline.iter().map(|q| to_screen(q[0], q[1])).collect();
            if s.truss {
                p.add(egui::Shape::closed_line(
                    pts,
                    Stroke::new(1.0, theme::text_faint()),
                ));
                continue;
            }
            let k = (s.top / head_y).clamp(0.0, 1.0);
            let fill = theme::raised().gamma_multiply(0.35 + 0.65 * k);
            p.add(egui::Shape::convex_polygon(
                pts,
                fill,
                Stroke::new(1.0, theme::border()),
            ));
        }
        p.rect_stroke(
            room,
            theme::R_S,
            Stroke::new(1.0, theme::border()),
            egui::StrokeKind::Inside,
        );

        // beams of following heads
        for tr in show.trackers.iter().filter(|t| t.active) {
            let col = color(show, tr.id);
            let c = to_screen(tr.target[0], tr.target[2]);
            for f in &tr.fixtures {
                let pos = show.placement3d(*f).position;
                let a = to_screen(pos[0], pos[2]);
                if a.distance(c) < 1.0 {
                    continue;
                }
                let spread = (c - a).rot90().normalized() * 6.0;
                p.add(egui::Shape::convex_polygon(
                    vec![a, c + spread, c - spread],
                    col.gamma_multiply(0.18),
                    Stroke::NONE,
                ));
            }
        }
        // moving heads: those of a target in its color, the others faint
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
                Some(col) => {
                    p.circle_filled(c, 5.5, *col);
                    p.circle_stroke(c, 5.5, Stroke::new(1.0, theme::bg()));
                }
                None => {
                    p.circle_stroke(c, 4.5, Stroke::new(1.0, theme::text_dim()));
                }
            }
        }
        // targets
        for tr in &show.trackers {
            let c = to_screen(tr.target[0], tr.target[2]);
            let strong = tr.active || current == Some(tr.id);
            let col = color(show, tr.id);
            let col = if strong { col } else { col.gamma_multiply(0.5) };
            if !tr.active {
                for f in &tr.fixtures {
                    let pos = show.placement3d(*f).position;
                    p.line_segment(
                        [to_screen(pos[0], pos[2]), c],
                        Stroke::new(1.0, col.gamma_multiply(0.3)),
                    );
                }
            }
            let r = 10.0;
            if tr.active {
                p.circle_filled(c, r, col.gamma_multiply(0.25));
            }
            p.circle_stroke(c, r, Stroke::new(2.0, col));
            for v in [vec2(1.0, 0.0), vec2(0.0, 1.0)] {
                p.line_segment([c - v * (r + 5.0), c - v * 4.0], Stroke::new(1.5, col));
                p.line_segment([c + v * 4.0, c + v * (r + 5.0)], Stroke::new(1.5, col));
            }
            let text = if labels {
                format!("{} · {:.1} m", tr.name, tr.target[1])
            } else {
                tr.name.clone()
            };
            p.text(
                c + vec2(r + 7.0, -r - 3.0),
                egui::Align2::LEFT_BOTTOM,
                text,
                theme::caption(),
                col,
            );
        }
    }
}
