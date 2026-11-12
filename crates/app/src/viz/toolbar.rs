//! Toolbar of the 3D visualizer: mode, environment, quality, files and on
//! the right the camera points (click = move there, keys 1–9) next to
//! fullscreen.

use std::time::Instant;

use eframe::egui::{self, pos2, vec2, Align2, Rect, Sense, Stroke};

use super::camera::OrbitCamera;
use super::{DialogKind, Mode, Quality, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::dialog::{select_all, text_field};
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton, Segmented};
use crate::views::ViewCtx;

/// Height of all controls in the bar.
const H: f32 = 28.0;
/// Duration of a camera move.
const FLY_SECS: f32 = 0.45;

impl VizView {
    pub(super) fn toolbar(
        &mut self,
        ui: &mut egui::Ui,
        cx: &mut ViewCtx<'_>,
        locked: bool,
        window: bool,
    ) {
        let frame = egui::Frame::new()
            .fill(theme::surface())
            .stroke(Stroke::new(1.0, theme::border()))
            .corner_radius(theme::R_M)
            .inner_margin(egui::Margin::symmetric(8, 5));
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = theme::SP_S;
                ui.set_min_height(H);
                self.left_part(ui, cx, locked);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = theme::SP_XS;
                    self.fullscreen_button(ui, cx, window);
                    divider(ui);
                    self.camera_points(ui, cx, locked);
                });
            });
        });
    }

    fn left_part(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, locked: bool) {
        let mut mode = self.mode;
        Segmented::new(&[(Mode::Build, t("Build")), (Mode::Live, t("Live"))]).show(ui, &mut mode);
        if !locked {
            self.mode = mode;
        }
        if self.mode == Mode::Build && !locked {
            ui.spacing_mut().item_spacing.x = 2.0;
            if ui
                .add(
                    IconButton::new(Icon::Undo)
                        .size(H)
                        .flat()
                        .enabled(cx.state.can_undo),
                )
                .on_hover_text(t("Undo (Ctrl+Z)"))
                .clicked()
            {
                cx.state.history_request = Some(true);
            }
            if ui
                .add(
                    IconButton::new(Icon::Redo)
                        .size(H)
                        .flat()
                        .enabled(cx.state.can_redo),
                )
                .on_hover_text(t("Redo (Ctrl+Y)"))
                .clicked()
            {
                cx.state.history_request = Some(false);
            }
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            divider(ui);
            self.snap_chip(ui);
        }
        divider(ui);
        let env = &mut cx.engine.show_mut().venue.environment;
        for (label, v, tip) in [
            (t("Ambient"), &mut env.ambient, t("Room light")),
            (t("Beams"), &mut env.beams, t("Visibility of light beams")),
            (t("Fog"), &mut env.fog, t("Haze in the room")),
        ] {
            level(ui, label, v, !locked).on_hover_text(tip);
        }
        divider(ui);
        // Quality as a dropdown in the style of the bar
        let q = cx.state.viz_quality;
        let r = chip(ui, t("Quality"), q.label(), true);
        menu::dropdown(&r, |ui| {
            menu::begin(ui);
            for v in Quality::ALL {
                if menu::item(ui, MenuItem::new(v.label()).checked(v == q)) {
                    cx.state.viz_quality = v;
                    ui.close();
                }
            }
        });
        divider(ui);
        ui.spacing_mut().item_spacing.x = 2.0;
        for (icon, tip, kind, enabled) in [
            (
                Icon::Import,
                t("Import Easy View 2 (as XML) …"),
                DialogKind::ImportEv2,
                !locked,
            ),
            (
                Icon::FileOpen,
                t("Load venue …"),
                DialogKind::LoadVenue,
                !locked,
            ),
            (
                Icon::Save,
                t("Save venue as …"),
                DialogKind::SaveVenue,
                true,
            ),
        ] {
            if ui
                .add(IconButton::new(icon).size(H).flat().enabled(enabled))
                .on_hover_text(tip)
                .clicked()
            {
                self.open_dialog(ui.ctx(), kind);
            }
        }
    }

    /// Snapping when moving and rotating (Ctrl: free).
    fn snap_chip(&mut self, ui: &mut egui::Ui) {
        use super::snap;
        let sn = self.snap;
        let value = match (sn.objects, sn.grid > 0.0) {
            (true, true) => format!("{} · {}", t("Objects"), snap::grid_label(sn.grid)),
            (true, false) => t("Objects").to_string(),
            (false, _) => snap::grid_label(sn.grid),
        };
        let r = chip(ui, t("Snap"), &value, true).on_hover_text(t(
            "Snapping when moving and rotating – hold Ctrl to move freely",
        ));
        menu::dropdown(&r, |ui| {
            menu::begin(ui);
            if menu::item(
                ui,
                MenuItem::new(t("To objects and fixtures")).checked(sn.objects),
            ) {
                self.snap.objects = !sn.objects;
            }
            menu::separator(ui);
            menu::title(ui, t("Grid"));
            for g in snap::GRIDS {
                if menu::item(
                    ui,
                    MenuItem::new(&snap::grid_label(g)).checked(sn.grid == g),
                ) {
                    self.snap.grid = g;
                }
            }
            menu::separator(ui);
            menu::title(ui, t("Rotation"));
            for a in snap::ANGLES {
                if menu::item(
                    ui,
                    MenuItem::new(&snap::angle_label(a)).checked(sn.angle == a),
                ) {
                    self.snap.angle = a;
                }
            }
        });
    }

    fn fullscreen_button(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, window: bool) {
        if window {
            let full = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
            if ui
                .add(
                    IconButton::new(Icon::Fullscreen)
                        .size(H)
                        .flat()
                        .active(full),
                )
                .on_hover_text(t("Full screen"))
                .clicked()
            {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!full));
            }
        } else if ui
            .add(
                IconButton::new(Icon::Fullscreen)
                    .size(H)
                    .flat()
                    .active(cx.state.viz_window),
            )
            .on_hover_text(t("Open in its own window (e.g. second screen)"))
            .clicked()
        {
            cx.state.viz_window = !cx.state.viz_window;
        }
    }

    /// Camera points (drawn right to left): add, points, home view, camera icon.
    fn camera_points(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, locked: bool) {
        let current = self.camera;
        // if the row does not fit, as a dropdown
        let needed: f32 = cx
            .engine
            .show()
            .venue
            .cameras
            .iter()
            .enumerate()
            .map(|(i, p)| point_width(ui, i + 1, &p.name) + theme::SP_XS)
            .sum::<f32>()
            + 3.0 * (H + theme::SP_XS);
        let collapsed = self.cam_rename.is_none() && needed > ui.available_width() - 40.0;
        if !locked
            && ui
                .add(IconButton::new(Icon::Plus).size(H).flat())
                .on_hover_text(t("Save current view as camera point"))
                .clicked()
        {
            if let Some(cam) = current {
                let venue = &mut cx.engine.show_mut().venue;
                let n = (1..)
                    .find(|n| {
                        let name = tr!("View {n}", n = n);
                        !venue.cameras.iter().any(|c| c.name == name)
                    })
                    .unwrap_or(1);
                venue.cameras.push(cam.to_preset(tr!("View {n}", n = n)));
                // rename right away (in the wide bar)
                if !collapsed {
                    self.cam_rename = Some((venue.cameras.len() - 1, tr!("View {n}", n = n)));
                    self.cam_rename_focus = true;
                }
            }
        }
        let presets = cx.engine.show().venue.cameras.clone();
        if collapsed {
            if let Some(i) = self.points_menu(ui, &presets, current, locked) {
                let list = &mut cx.engine.show_mut().venue.cameras;
                if i < list.len() {
                    list.remove(i);
                }
            }
            return;
        }
        let mut remove = None;
        let mut overwrite = None;
        let mut rename_done = None;
        // right to left: the points in reverse so they stand left → right
        for (i, p) in presets.iter().enumerate().rev() {
            if let Some((ri, text)) = self.cam_rename.as_mut().filter(|(ri, _)| *ri == i) {
                let r = ui.add_sized(vec2(120.0, H), text_field(text));
                if std::mem::take(&mut self.cam_rename_focus) {
                    r.request_focus();
                    select_all(ui.ctx(), r.id, text);
                }
                let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
                if esc {
                    self.cam_rename = None;
                } else if r.lost_focus() {
                    rename_done = Some((*ri, text.trim().to_string()));
                }
                continue;
            }
            let target = OrbitCamera::from_preset(p);
            let active = current.is_some_and(|c| c.near(&target));
            let key = if i < 9 {
                tr!("key {n}", n = i + 1)
            } else {
                String::new()
            };
            let r = point_chip(ui, i + 1, &p.name, active).on_hover_text(tr!(
                "Move to camera point “{name}”{key}\nRight-click: overwrite, rename, delete",
                name = p.name,
                key = if key.is_empty() {
                    String::new()
                } else {
                    format!(" ({key})")
                }
            ));
            if r.clicked() {
                self.fly_to(target);
            }
            if !locked {
                if r.double_clicked() {
                    self.cam_rename = Some((i, p.name.clone()));
                    self.cam_rename_focus = true;
                }
                menu::context(&r, |ui| {
                    menu::begin(ui);
                    menu::title(ui, &p.name);
                    if menu::item(
                        ui,
                        MenuItem::new(t("Overwrite with current view")).icon(Icon::Camera),
                    ) {
                        overwrite = Some(i);
                    }
                    if menu::item(ui, MenuItem::new(t("Rename")).icon(Icon::Edit)) {
                        self.cam_rename = Some((i, p.name.clone()));
                        self.cam_rename_focus = true;
                    }
                    menu::separator(ui);
                    if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash).danger()) {
                        remove = Some(i);
                    }
                });
            }
        }
        if ui
            .add(IconButton::new(Icon::ZoomFit).size(H).flat())
            .on_hover_text(t("Default view"))
            .clicked()
        {
            self.fly_to(OrbitCamera::for_room(self.room));
        }

        let venue = &mut cx.engine.show_mut().venue;
        if let Some((i, name)) = rename_done {
            self.cam_rename = None;
            if let (Some(c), false) = (venue.cameras.get_mut(i), name.is_empty()) {
                c.name = name;
            }
        }
        if let (Some(i), Some(cam)) = (overwrite, current) {
            if let Some(c) = venue.cameras.get_mut(i) {
                *c = cam.to_preset(c.name.clone());
            }
        }
        if let Some(i) = remove {
            if i < venue.cameras.len() {
                venue.cameras.remove(i);
            }
            self.cam_rename = None;
        }
    }

    /// Narrow window: camera points as a dropdown (editing in the wide bar).
    fn points_menu(
        &mut self,
        ui: &mut egui::Ui,
        presets: &[dmxcraft_core::venue::CameraPreset],
        current: Option<OrbitCamera>,
        locked: bool,
    ) -> Option<usize> {
        let active = presets
            .iter()
            .position(|p| current.is_some_and(|c| c.near(&OrbitCamera::from_preset(p))));
        let value = active.map_or_else(|| "–".to_string(), |i| presets[i].name.clone());
        let r = chip(ui, t("Camera"), &value, true).on_hover_text(t("Camera points (keys 1–9)"));
        let mut go = None;
        let mut remove = None;
        menu::dropdown(&r, |ui| {
            menu::begin(ui);
            for (i, p) in presets.iter().enumerate() {
                let key = if i < 9 {
                    (i + 1).to_string()
                } else {
                    String::new()
                };
                if menu::item(
                    ui,
                    MenuItem::new(&p.name)
                        .shortcut(&key)
                        .checked(active == Some(i)),
                ) {
                    go = Some(Some(i));
                    ui.close();
                }
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Default view")).icon(Icon::ZoomFit)) {
                go = Some(None);
                ui.close();
            }
            if !locked && !presets.is_empty() {
                menu::sub(ui, MenuItem::new(t("Delete")).icon(Icon::Trash), |ui| {
                    menu::begin(ui);
                    for (i, p) in presets.iter().enumerate() {
                        if menu::item(ui, MenuItem::new(&p.name).danger()) {
                            remove = Some(i);
                            ui.close();
                        }
                    }
                });
            }
        });
        match go {
            Some(None) => self.fly_to(OrbitCamera::for_room(self.room)),
            Some(Some(i)) => {
                if let Some(p) = presets.get(i) {
                    self.fly_to(OrbitCamera::from_preset(p));
                }
            }
            None => {}
        }
        remove
    }

    /// Move smoothly to a camera position.
    pub(super) fn fly_to(&mut self, to: OrbitCamera) {
        if let Some(from) = self.camera {
            self.fly = Some((from, to, Instant::now()));
        } else {
            self.camera = Some(to);
        }
    }

    /// Advance the camera move (once per frame). `true` while it is running.
    pub(super) fn advance_fly(&mut self) -> bool {
        let Some((from, to, start)) = self.fly else {
            return false;
        };
        let k = (start.elapsed().as_secs_f32() / FLY_SECS).min(1.0);
        // accelerate and decelerate smoothly
        let e = k * k * (3.0 - 2.0 * k);
        self.camera = Some(from.lerp(&to, e));
        if k >= 1.0 {
            self.camera = Some(to);
            self.fly = None;
        }
        true
    }

    /// Keys 1–9 recall camera points (when no text field is active).
    pub(super) fn camera_keys(&mut self, ui: &egui::Ui, cx: &ViewCtx<'_>) {
        if ui.ctx().egui_wants_keyboard_input() {
            return;
        }
        let keys = [
            egui::Key::Num1,
            egui::Key::Num2,
            egui::Key::Num3,
            egui::Key::Num4,
            egui::Key::Num5,
            egui::Key::Num6,
            egui::Key::Num7,
            egui::Key::Num8,
            egui::Key::Num9,
        ];
        let hit = ui.input(|i| {
            if i.modifiers.any() {
                return None;
            }
            keys.iter().position(|k| i.key_pressed(*k))
        });
        if let Some(p) = hit.and_then(|n| cx.engine.show().venue.cameras.get(n)) {
            self.fly_to(OrbitCamera::from_preset(p));
        }
    }
}

/// Thin vertical divider.
fn divider(ui: &mut egui::Ui) {
    let (r, _) = ui.allocate_exact_size(vec2(1.0, H - 6.0), Sense::hover());
    ui.painter().rect_filled(r, 0.0, theme::border());
}

/// Value 0–1 as a bar with the label on the left and percent on the right;
/// dragging, clicking or the mouse wheel changes it.
fn level(ui: &mut egui::Ui, label: &str, v: &mut f32, enabled: bool) -> egui::Response {
    let sense = if enabled {
        Sense::click_and_drag()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(116.0, H), sense);
    if enabled {
        if let Some(p) = resp
            .interact_pointer_pos()
            .filter(|_| resp.dragged() || resp.clicked())
        {
            *v = ((p.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            *v = (*v * 100.0).round() / 100.0;
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                *v = (*v + scroll / 2000.0).clamp(0.0, 1.0);
            }
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }
    }
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S, theme::bg());
    let fill = Rect::from_min_size(
        rect.min,
        vec2(rect.width() * v.clamp(0.0, 1.0), rect.height()),
    );
    p.rect_filled(fill, theme::R_S, theme::accent().gamma_multiply(0.28));
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(
            1.0,
            if resp.hovered() || resp.dragged() {
                theme::accent()
            } else {
                theme::border()
            },
        ),
        egui::StrokeKind::Inside,
    );
    p.text(
        pos2(rect.left() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        label.to_uppercase(),
        theme::caption(),
        theme::text_dim(),
    );
    p.text(
        pos2(rect.right() - 8.0, rect.center().y),
        Align2::RIGHT_CENTER,
        format!("{:.0} %", *v * 100.0),
        theme::body(),
        theme::text(),
    );
    resp
}

/// Dropdown in the style of the bar: label, value, arrow.
fn chip(ui: &mut egui::Ui, label: &str, value: &str, enabled: bool) -> egui::Response {
    let label_g =
        ui.painter()
            .layout_no_wrap(label.to_uppercase(), theme::caption(), theme::text_dim());
    let value_g = ui
        .painter()
        .layout_no_wrap(value.to_string(), theme::body(), theme::text());
    let w = 8.0 + label_g.size().x + 8.0 + value_g.size().x + 6.0 + 14.0 + 6.0;
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(w, H), sense);
    let p = ui.painter();
    p.rect_filled(
        rect,
        theme::R_S,
        if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::raised()
        },
    );
    let mut x = rect.left() + 8.0;
    p.galley(
        pos2(x, rect.center().y - label_g.size().y / 2.0),
        label_g.clone(),
        theme::text_dim(),
    );
    x += label_g.size().x + 8.0;
    p.galley(
        pos2(x, rect.center().y - value_g.size().y / 2.0),
        value_g.clone(),
        theme::text(),
    );
    x += value_g.size().x + 6.0;
    Icon::ChevronDown.paint(
        p,
        Rect::from_min_size(pos2(x, rect.center().y - 7.0), vec2(14.0, 14.0)),
        theme::text_dim(),
    );
    resp
}

/// Width of a camera point in the bar.
fn point_width(ui: &egui::Ui, n: usize, name: &str) -> f32 {
    let num = ui
        .painter()
        .layout_no_wrap(n.to_string(), theme::caption(), theme::text_dim());
    let text = ui
        .painter()
        .layout(name.to_string(), theme::body(), theme::text(), 110.0);
    8.0 + num.size().x + 6.0 + text.size().x.min(110.0) + 10.0
}

/// Camera point: number and name; active (camera is there) with accent.
fn point_chip(ui: &mut egui::Ui, n: usize, name: &str, active: bool) -> egui::Response {
    let num = ui
        .painter()
        .layout_no_wrap(n.to_string(), theme::caption(), theme::text_dim());
    // only one line: long names shortened (the tooltip shows the full name)
    let w = point_width(ui, n, name);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, H), Sense::click());
    let p = ui.painter();
    let (fill, stroke) = if active {
        (theme::accent().gamma_multiply(0.22), theme::accent())
    } else if resp.hovered() {
        (theme::raised_hover(), theme::border())
    } else {
        (theme::raised(), theme::raised())
    };
    p.rect_filled(rect, theme::R_S, fill);
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(1.0, stroke),
        egui::StrokeKind::Inside,
    );
    let num_color = if active {
        theme::accent()
    } else {
        theme::text_dim()
    };
    p.text(
        pos2(rect.left() + 8.0, rect.center().y),
        Align2::LEFT_CENTER,
        n.to_string(),
        theme::caption(),
        num_color,
    );
    let clip = Rect::from_min_max(
        pos2(rect.left() + 8.0 + num.size().x + 6.0, rect.top()),
        pos2(rect.right() - 6.0, rect.bottom()),
    );
    ui.painter_at(clip).text(
        pos2(clip.left(), rect.center().y),
        Align2::LEFT_CENTER,
        name,
        theme::body(),
        theme::text(),
    );
    resp
}
