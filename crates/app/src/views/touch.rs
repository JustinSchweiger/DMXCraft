//! TOUCH: frei angeordnete Bedienelemente auf Seiten (Raster). Im
//! Bearbeiten-Modus lassen sich Elemente anlegen, verschieben, in der Größe
//! ändern, beschriften und einfärben; sonst werden sie bedient.
//!
//! Elemente kommen per Alt + Klick auf ein Bedienelement (Szene, Bank,
//! Fader, Gerät …) oder über die Liste „Bedienelemente hinzufügen“.

use std::collections::HashMap;

use dmxcraft_core::mapping::{ButtonMode, Control, Mapping};
use dmxcraft_core::touch::{TouchItem, TouchKind, TouchPage};
use dmxcraft_core::{ChannelKind, ChannelRef, Engine, FixtureId, Show};
use eframe::egui::{
    self, pos2, vec2, Align, Align2, Color32, Layout, Mesh, Pos2, Rect, Sense, Shape, Stroke,
    StrokeKind, Ui,
};

use crate::i18n::t;
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::dialog::{form, select_all};
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{section_header, text_field, Icon, IconButton, Toggle};
use crate::views::{View, ViewCtx};

/// Abstand zwischen Elementen.
const GAP: f32 = 6.0;
/// Breite der Bearbeiten-Leiste rechts.
const PANEL_W: f32 = 280.0;

/// Farben zum Einfärben von Schaltflächen.
const COLORS: [[u8; 3]; 12] = [
    [239, 68, 68],
    [249, 115, 22],
    [234, 179, 8],
    [132, 204, 22],
    [34, 197, 94],
    [20, 184, 166],
    [56, 189, 248],
    [59, 130, 246],
    [139, 92, 246],
    [217, 70, 239],
    [244, 114, 182],
    [229, 231, 235],
];

/// Ziehen im Bearbeiten-Modus.
#[derive(Clone, Copy)]
enum Drag {
    Move { id: u32, grab: egui::Vec2 },
    Resize { id: u32 },
}

pub struct TouchView {
    /// Kantenlänge einer Rasterzelle.
    cell: f32,
    drag: Option<Drag>,
    rename_page: Option<(u32, String)>,
    rename_focus: bool,
    /// Zuletzt gesetzte Werte von Fadern ohne lesbaren Zustand.
    values: HashMap<u32, f32>,
    /// Gedrückte Schaltflächen (für Loslassen).
    held: Vec<u32>,
}

impl Default for TouchView {
    fn default() -> Self {
        Self {
            cell: 72.0,
            drag: None,
            rename_page: None,
            rename_focus: false,
            values: HashMap::new(),
            held: Vec::new(),
        }
    }
}

/// Zuordnung, mit der ein Element auslöst (wie eine Taste).
fn mapping_of(item: &TouchItem) -> Option<Mapping> {
    let control = item.control?;
    let fader = matches!(item.kind, TouchKind::Fader | TouchKind::Knob);
    Some(Mapping {
        id: 0,
        control,
        element: None,
        input: None,
        mode: if fader {
            ButtonMode::Value
        } else {
            ButtonMode::Toggle
        },
        flash: item.flash,
        feedback: None,
        name: None,
        channels: item.channels.clone(),
    })
}

impl View for TouchView {
    fn ui(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>) {
        let editing = cx.state.touch_edit && !cx.state.locked;
        // gültige Seite
        let pages: Vec<(u32, String)> = cx
            .engine
            .show()
            .touch
            .pages
            .iter()
            .map(|p| (p.id, p.name.clone()))
            .collect();
        if !pages.iter().any(|p| Some(p.0) == cx.state.touch_page) {
            cx.state.touch_page = pages.first().map(|p| p.0);
        }

        // ----- Kopfzeile: Seiten, Zoom, Bearbeiten -----
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            self.page_tabs(ui, cx, &pages, editing);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if !cx.state.locked {
                    let r = ui
                        .add(egui::Button::new(t("Bearbeiten")).selected(cx.state.touch_edit))
                        .on_hover_text(t("Touch-Oberfläche bearbeiten"));
                    if r.clicked() {
                        cx.state.touch_edit = !cx.state.touch_edit;
                        cx.state.touch_sel = None;
                    }
                }
                ui.add_space(theme::SP_S);
                if ui
                    .add(IconButton::new(Icon::ZoomIn).flat())
                    .on_hover_text(t("Vergrößern"))
                    .clicked()
                {
                    self.cell = (self.cell + 8.0).min(160.0);
                }
                if ui
                    .add(IconButton::new(Icon::ZoomOut).flat())
                    .on_hover_text(t("Verkleinern"))
                    .clicked()
                {
                    self.cell = (self.cell - 8.0).max(40.0);
                }
            });
        });
        ui.add_space(theme::SP_S);

        let full = ui.available_rect_before_wrap();
        let (grid_rect, panel_rect) = if editing {
            let split = full.right() - PANEL_W;
            (
                Rect::from_min_max(full.min, pos2(split - theme::SP_S, full.bottom())),
                Some(Rect::from_min_max(pos2(split, full.top()), full.max)),
            )
        } else {
            (full, None)
        };
        ui.allocate_rect(full, Sense::hover());

        let Some(page_id) = cx.state.touch_page else {
            self.empty(ui, grid_rect, cx);
            if let Some(r) = panel_rect {
                self.panel(ui, r, cx, None);
            }
            return;
        };

        let mut grid_ui = ui.new_child(egui::UiBuilder::new().max_rect(grid_rect));
        egui::ScrollArea::both()
            .id_salt("touch_grid")
            .auto_shrink([false, false])
            .show(&mut grid_ui, |ui| self.grid(ui, cx, page_id, editing));
        if let Some(r) = panel_rect {
            self.panel(ui, r, cx, Some(page_id));
        }
    }
}

impl TouchView {
    fn page_tabs(
        &mut self,
        ui: &mut Ui,
        cx: &mut ViewCtx<'_>,
        pages: &[(u32, String)],
        editing: bool,
    ) {
        let mut delete = None;
        for (id, name) in pages {
            if let Some((_, text)) = self.rename_page.as_mut().filter(|(p, _)| p == id) {
                let r = ui.add(text_field(text).desired_width(140.0));
                if std::mem::take(&mut self.rename_focus) {
                    r.request_focus();
                    select_all(ui.ctx(), r.id, text);
                }
                let (enter, esc) = ui.input(|i| {
                    (
                        i.key_pressed(egui::Key::Enter),
                        i.key_pressed(egui::Key::Escape),
                    )
                });
                if esc {
                    self.rename_page = None;
                } else if enter || r.lost_focus() {
                    let name = text.trim().to_string();
                    let id = *id;
                    self.rename_page = None;
                    if !name.is_empty() {
                        if let Some(p) = cx.engine.show_mut().touch.page_mut(id) {
                            p.name = name;
                        }
                    }
                }
                continue;
            }
            let selected = cx.state.touch_page == Some(*id);
            let r = ui.add(egui::Button::new(name.as_str()).selected(selected));
            if r.clicked() {
                cx.state.touch_page = Some(*id);
                cx.state.touch_sel = None;
            }
            if editing {
                if r.double_clicked() {
                    self.rename_page = Some((*id, name.clone()));
                    self.rename_focus = true;
                }
                menu::context(&r, |ui| {
                    menu::begin(ui);
                    if menu::item(ui, MenuItem::new(t("Umbenennen")).icon(Icon::Edit)) {
                        self.rename_page = Some((*id, name.clone()));
                        self.rename_focus = true;
                    }
                    if menu::item(ui, MenuItem::new(t("Seite löschen")).icon(Icon::Trash)) {
                        delete = Some(*id);
                    }
                });
            }
        }
        if editing
            && ui
                .add(IconButton::new(Icon::Plus).flat())
                .on_hover_text(t("Seite hinzufügen"))
                .clicked()
        {
            add_page(cx);
        }
        if let Some(id) = delete {
            cx.engine.show_mut().touch.pages.retain(|p| p.id != id);
            if cx.state.touch_page == Some(id) {
                cx.state.touch_page = None;
            }
        }
    }

    /// Noch keine Seite.
    fn empty(&mut self, ui: &mut Ui, rect: Rect, cx: &mut ViewCtx<'_>) {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Center)),
        );
        child.add_space(rect.height() * 0.3);
        child.label(
            egui::RichText::new(t("Noch keine Touch-Oberfläche"))
                .color(theme::text_dim())
                .size(theme::FS_TITLE),
        );
        child.add_space(theme::SP_S);
        child.label(
            egui::RichText::new(t(
                "Alt + Klick auf eine Szene, einen Fader oder ein anderes Bedienelement fügt es hier hinzu.",
            ))
            .color(theme::text_faint()),
        );
        child.add_space(theme::SP_M);
        if !cx.state.locked && child.button(t("Seite hinzufügen")).clicked() {
            add_page(cx);
            cx.state.touch_edit = true;
        }
    }

    /// Raster mit den Elementen einer Seite.
    fn grid(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>, page_id: u32, editing: bool) {
        let Some(page) = cx.engine.show().touch.page(page_id).cloned() else {
            return;
        };
        let c = self.cell;
        // Fläche: mindestens sichtbarer Bereich, sonst bis zum letzten Element
        let cols = page
            .items
            .iter()
            .map(|i| i.x + i.w)
            .max()
            .unwrap_or(0)
            .max(1)
            + 2;
        let rows = page
            .items
            .iter()
            .map(|i| i.y + i.h)
            .max()
            .unwrap_or(0)
            .max(1)
            + 2;
        let avail = ui.available_size();
        let size = vec2(
            (cols as f32 * c).max(avail.x),
            (rows as f32 * c).max(avail.y),
        );
        let (area, bg) = ui.allocate_exact_size(size, Sense::click());
        let origin = area.min;
        let p = ui.painter().clone();
        // Rasterzellen
        let vis = ui.clip_rect();
        let (x0, y0) = (
            ((vis.left() - origin.x) / c).floor().max(0.0) as i32,
            ((vis.top() - origin.y) / c).floor().max(0.0) as i32,
        );
        let (x1, y1) = (
            ((vis.right() - origin.x) / c).ceil() as i32,
            ((vis.bottom() - origin.y) / c).ceil() as i32,
        );
        let cell_fill = if editing {
            theme::raised()
        } else {
            theme::raised().gamma_multiply(0.45)
        };
        for y in y0..y1 {
            for x in x0..x1 {
                let r = Rect::from_min_size(
                    origin + vec2(x as f32 * c, y as f32 * c),
                    vec2(c - GAP, c - GAP),
                );
                p.rect_filled(r, theme::R_S, cell_fill);
            }
        }
        if editing && bg.clicked() {
            cx.state.touch_sel = None;
        }

        let item_rect = |i: &TouchItem| {
            Rect::from_min_size(
                origin + vec2(i.x as f32 * c, i.y as f32 * c),
                vec2(i.w as f32 * c - GAP, i.h as f32 * c - GAP),
            )
        };
        let states: HashMap<u64, bool> = crate::mapping::ui_states(ui.ctx());
        let mut changes: Vec<TouchItem> = Vec::new();
        let mut remove = None;
        let mut duplicate = None;
        for item in &page.items {
            let rect = item_rect(item);
            let id = ui.make_persistent_id(("touch_item", item.id));
            if editing {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                paint_item(
                    &p,
                    cx.engine,
                    cx.state,
                    &states,
                    item,
                    rect,
                    self.value(cx.engine, item),
                );
                let sel = cx.state.touch_sel == Some(item.id);
                p.rect_stroke(
                    rect,
                    theme::R_S,
                    Stroke::new(
                        if sel { 2.0 } else { 1.0 },
                        if sel {
                            theme::accent()
                        } else {
                            theme::border()
                        },
                    ),
                    StrokeKind::Inside,
                );
                // Griff zum Größe ändern
                let handle =
                    Rect::from_min_size(rect.right_bottom() - vec2(14.0, 14.0), vec2(14.0, 14.0));
                let hresp = ui.interact(handle, id.with("resize"), Sense::drag());
                if sel || hresp.hovered() {
                    p.add(Shape::convex_polygon(
                        vec![
                            handle.right_top(),
                            handle.right_bottom(),
                            handle.left_bottom(),
                        ],
                        theme::accent(),
                        Stroke::NONE,
                    ));
                }
                if hresp.hovered() || hresp.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeNwSe);
                }
                if hresp.drag_started() {
                    self.drag = Some(Drag::Resize { id: item.id });
                    cx.state.touch_sel = Some(item.id);
                } else if resp.drag_started() {
                    let grab = resp
                        .interact_pointer_pos()
                        .map_or(vec2(0.0, 0.0), |p| p - rect.min);
                    self.drag = Some(Drag::Move { id: item.id, grab });
                    cx.state.touch_sel = Some(item.id);
                }
                if resp.clicked() {
                    cx.state.touch_sel = Some(item.id);
                }
                menu::context(&resp, |ui| {
                    menu::begin(ui);
                    menu::title(ui, &item_title(cx.engine.show(), item));
                    if menu::item(ui, MenuItem::new(t("Duplizieren")).icon(Icon::Copy)) {
                        duplicate = Some(item.id);
                    }
                    if menu::item(ui, MenuItem::new(t("Löschen")).icon(Icon::Trash)) {
                        remove = Some(item.id);
                    }
                    menu::separator(ui);
                    ui.horizontal_wrapped(|ui| {
                        ui.set_max_width(200.0);
                        for col in COLORS {
                            if ui
                                .add(crate::ui::widgets::Swatch(theme::rgb(col)))
                                .clicked()
                            {
                                let mut i = item.clone();
                                i.color = Some(col);
                                changes.push(i);
                                ui.close();
                            }
                        }
                    });
                });
            } else {
                self.play_item(ui, &p, cx, &states, item, rect, id);
            }
        }

        // Ziehen anwenden (in Zellen)
        if let Some(d) = self.drag {
            let pointer = ui.input(|i| i.pointer.interact_pos());
            let released = ui.input(|i| i.pointer.any_released());
            let (id, preview) = match (d, pointer) {
                (Drag::Move { id, grab }, Some(pos)) => {
                    let Some(it) = page.items.iter().find(|i| i.id == id) else {
                        self.drag = None;
                        return;
                    };
                    let tl = pos - grab - origin;
                    let mut n = it.clone();
                    n.x = ((tl.x / c).round() as i32).max(0);
                    n.y = ((tl.y / c).round() as i32).max(0);
                    (id, n)
                }
                (Drag::Resize { id }, Some(pos)) => {
                    let Some(it) = page.items.iter().find(|i| i.id == id) else {
                        self.drag = None;
                        return;
                    };
                    let rel = pos - origin;
                    let mut n = it.clone();
                    n.w = ((rel.x / c).ceil() as i32 - it.x).max(1);
                    n.h = ((rel.y / c).ceil() as i32 - it.y).max(1);
                    (id, n)
                }
                _ => {
                    self.drag = None;
                    return;
                }
            };
            let free = page.is_free(preview.x, preview.y, preview.w, preview.h, Some(id));
            let r = item_rect(&preview);
            p.rect_stroke(
                r,
                theme::R_S,
                Stroke::new(2.0, if free { theme::accent() } else { theme::DANGER }),
                StrokeKind::Inside,
            );
            if released {
                if free {
                    changes.push(preview);
                }
                self.drag = None;
            }
        }

        // Entf löscht das gewählte Element
        if editing
            && !ui.ctx().egui_wants_keyboard_input()
            && ui.input(|i| i.key_pressed(egui::Key::Delete))
        {
            remove = remove.or(cx.state.touch_sel);
        }
        let touch = &mut cx.engine.show_mut().touch;
        if let Some(pg) = touch.page_mut(page_id) {
            for n in changes {
                if let Some(slot) = pg.items.iter_mut().find(|i| i.id == n.id) {
                    *slot = n;
                }
            }
            if let Some(id) = remove {
                pg.items.retain(|i| i.id != id);
                if cx.state.touch_sel == Some(id) {
                    cx.state.touch_sel = None;
                }
            }
        }
        if let Some(id) = duplicate {
            cx.state.touch_sel = touch.duplicate(page_id, id);
        }
    }

    /// Aktueller Wert 0–1 eines Faders/Drehreglers.
    fn value(&self, e: &Engine, item: &TouchItem) -> f32 {
        let live = |c: &ChannelRef| {
            e.live()
                .get(c)
                .map_or(if c.is_virtual() { 1.0 } else { 0.0 }, |v| {
                    f32::from(*v) / 255.0
                })
        };
        match item.control {
            Some(Control::Master) => e.master(),
            Some(Control::Channels) => item.channels.first().map_or(0.0, live),
            Some(Control::Channel { channel }) => live(&channel),
            _ => self.values.get(&item.id).copied().unwrap_or(0.0),
        }
    }

    /// Element bedienen.
    #[allow(clippy::too_many_arguments)]
    fn play_item(
        &mut self,
        ui: &mut Ui,
        p: &egui::Painter,
        cx: &mut ViewCtx<'_>,
        states: &HashMap<u64, bool>,
        item: &TouchItem,
        rect: Rect,
        id: egui::Id,
    ) {
        match item.kind {
            TouchKind::Label => {
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0);
            }
            TouchKind::Button => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let down = resp.is_pointer_button_down_on();
                let was = self.held.contains(&item.id);
                if let Some(m) = mapping_of(item) {
                    if down && !was {
                        self.held.push(item.id);
                        cx.state.touch_fire.push((m, true, 127, false));
                    } else if !down && was {
                        self.held.retain(|h| *h != item.id);
                        cx.state.touch_fire.push((m, false, 0, false));
                    }
                }
                paint_item(
                    p,
                    cx.engine,
                    cx.state,
                    states,
                    item,
                    rect,
                    if down { 1.0 } else { 0.0 },
                );
            }
            TouchKind::Fader | TouchKind::Knob => {
                let value = self.value(cx.engine, item);
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let mut v = value;
                if let Some(pos) = resp
                    .interact_pointer_pos()
                    .filter(|_| resp.dragged() || resp.clicked())
                {
                    if item.kind == TouchKind::Fader {
                        let track = fader_track(rect);
                        v = (1.0 - (pos.y - track.top()) / track.height()).clamp(0.0, 1.0);
                    } else {
                        v = (value - resp.drag_delta().y / 150.0).clamp(0.0, 1.0);
                    }
                }
                if (v - value).abs() > f32::EPSILON {
                    self.values.insert(item.id, v);
                    if let Some(m) = mapping_of(item) {
                        let raw = (v * 127.0).round() as u8;
                        cx.state.touch_fire.push((m, raw > 0, raw, true));
                    }
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, v);
            }
            TouchKind::Color => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let field = content_rect(rect, item);
                if let Some(pos) = resp
                    .interact_pointer_pos()
                    .filter(|p| (resp.dragged() || resp.clicked()) && field.contains(*p))
                {
                    let h = ((pos.x - field.left()) / field.width()).clamp(0.0, 1.0);
                    let s = 1.0 - ((pos.y - field.top()) / field.height()).clamp(0.0, 1.0);
                    let rgb = egui::ecolor::Hsva::new(h, s, 1.0, 1.0).to_srgb();
                    set_color(cx.engine, &item.fixtures, [rgb[0], rgb[1], rgb[2]]);
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0);
            }
            TouchKind::Xy => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let field = content_rect(rect, item);
                if let Some(pos) = resp
                    .interact_pointer_pos()
                    .filter(|p| (resp.dragged() || resp.clicked()) && field.contains(*p))
                {
                    let x = ((pos.x - field.left()) / field.width()).clamp(0.0, 1.0);
                    let y = ((pos.y - field.top()) / field.height()).clamp(0.0, 1.0);
                    set_position(cx.engine, &item.fixtures, x, y);
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0);
            }
        }
    }

    /// Bearbeiten-Leiste: Seite, neue Elemente, Eigenschaften.
    fn panel(&mut self, ui: &mut Ui, rect: Rect, cx: &mut ViewCtx<'_>, page: Option<u32>) {
        ui.painter().line_segment(
            [
                rect.left_top() - vec2(theme::SP_S * 0.5, 0.0),
                rect.left_bottom() - vec2(theme::SP_S * 0.5, 0.0),
            ],
            Stroke::new(1.0, theme::border()),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.shrink2(vec2(theme::SP_S, 0.0)))
                .layout(Layout::top_down(Align::Min)),
        );
        egui::ScrollArea::vertical()
            .id_salt("touch_panel")
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                if ui
                    .add_sized([ui.available_width(), 32.0], egui::Button::new(t("Seite hinzufügen")))
                    .clicked()
                {
                    add_page(cx);
                }
                let Some(page) = page else {
                    return;
                };
                ui.add_space(theme::SP_M);
                section_header(ui, t("Bedienelemente hinzufügen"), |_| {});
                let kinds = [
                    (TouchKind::Label, t("Beschriftung")),
                    (TouchKind::Button, t("Schaltfläche")),
                    (TouchKind::Fader, t("Fader")),
                    (TouchKind::Knob, t("Drehregler")),
                    (TouchKind::Color, t("Farbrad")),
                    (TouchKind::Xy, t("XY-Feld")),
                ];
                egui::Grid::new("touch_add_kinds")
                    .num_columns(2)
                    .spacing(vec2(theme::SP_S, theme::SP_S))
                    .show(ui, |ui| {
                        let w = (ui.available_width() - theme::SP_S) / 2.0;
                        for (i, (k, label)) in kinds.into_iter().enumerate() {
                            if ui
                                .add(egui::Button::new(label).min_size(vec2(w, 40.0)))
                                .clicked()
                            {
                                let mut item = TouchItem::new(k);
                                item.label = match k {
                                    TouchKind::Label => t("Beschriftung").to_string(),
                                    _ => String::new(),
                                };
                                if k.uses_fixtures() {
                                    item.fixtures = cx.state.selection_order.clone();
                                }
                                cx.state.touch_sel = cx.engine.show_mut().touch.add_item(page, item);
                            }
                            if i % 2 == 1 {
                                ui.end_row();
                            }
                        }
                    });
                ui.add_space(theme::SP_S);
                ui.label(
                    egui::RichText::new(t(
                        "Tipp: Alt + Klick auf eine Szene, einen Fader, ein Gerät oder einen anderen Knopf legt ihn hier an – ist ein Element gewählt, bekommt es dieses Ziel.",
                    ))
                    .small()
                    .color(theme::text_faint()),
                );
                if let Some(sel) = cx.state.touch_sel {
                    ui.add_space(theme::SP_M);
                    self.properties(ui, cx, page, sel);
                }
            });
    }

    /// Eigenschaften des gewählten Elements.
    fn properties(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>, page: u32, sel: u32) {
        let Some(mut item) = cx
            .engine
            .show()
            .touch
            .page(page)
            .and_then(|p| p.items.iter().find(|i| i.id == sel))
            .cloned()
        else {
            cx.state.touch_sel = None;
            return;
        };
        let before = item.clone();
        let mut remove = false;
        let mut duplicate = false;
        section_header(ui, t("Eigenschaften"), |ui| {
            if ui
                .add(IconButton::new(Icon::Trash).flat().danger())
                .on_hover_text(t("Löschen"))
                .clicked()
            {
                remove = true;
            }
            if ui
                .add(IconButton::new(Icon::Copy).flat())
                .on_hover_text(t("Duplizieren"))
                .clicked()
            {
                duplicate = true;
            }
        });
        let show = cx.engine.show();
        form(ui, "touch_props", |f| {
            f.row(t("Art"), |ui| {
                ui.label(kind_name(item.kind));
            });
            f.row(t("Beschriftung"), |ui| {
                ui.add(text_field(&mut item.label).desired_width(ui.available_width()));
            });
            f.row(t("Textgröße"), |ui| {
                ui.add(
                    egui::DragValue::new(&mut item.text_size)
                        .range(0.0..=72.0)
                        .speed(0.5)
                        .custom_formatter(|v, _| {
                            if v <= 0.0 {
                                t("automatisch").to_string()
                            } else {
                                format!("{v:.0}")
                            }
                        }),
                );
            });
            f.row(t("Größe"), |ui| {
                ui.add(egui::DragValue::new(&mut item.w).range(1..=32).prefix("B "));
                ui.add(egui::DragValue::new(&mut item.h).range(1..=32).prefix("H "));
            });
            if item.kind != TouchKind::Label {
                f.row(t("Ziel"), |ui| {
                    let text = if item.kind.uses_fixtures() {
                        match item.fixtures.len() {
                            0 => t("keine Geräte").to_string(),
                            1 => show
                                .fixture(item.fixtures[0])
                                .map_or_else(String::new, |f| f.name.clone()),
                            n => tr!("{n} Geräte", n = n),
                        }
                    } else {
                        item_target(show, &item)
                    };
                    ui.label(egui::RichText::new(text).color(theme::text_dim()));
                });
            }
            if item.kind.uses_fixtures() {
                f.row("", |ui| {
                    if ui
                        .add_enabled(
                            !cx.state.selected.is_empty(),
                            egui::Button::new(t("Auswahl übernehmen")),
                        )
                        .clicked()
                    {
                        item.fixtures = cx.state.selection_order.clone();
                    }
                });
            }
            if item.kind == TouchKind::Button {
                f.row(t("Flash-Taste"), |ui| {
                    ui.add(Toggle::new(&mut item.flash, ""))
                        .on_hover_text(t("Nur aktiv, solange die Taste gedrückt ist"));
                });
            }
        });
        ui.add_space(theme::SP_S);
        ui.label(egui::RichText::new(t("Farbe")).color(theme::text_dim()));
        ui.horizontal_wrapped(|ui| {
            if ui
                .add(
                    IconButton::new(Icon::Close)
                        .flat()
                        .active(item.color.is_none()),
                )
                .on_hover_text(t("Keine Farbe"))
                .clicked()
            {
                item.color = None;
            }
            for col in COLORS {
                let r = ui.add(crate::ui::widgets::Swatch(theme::rgb(col)));
                if item.color == Some(col) {
                    ui.painter().rect_stroke(
                        r.rect.expand(2.0),
                        theme::R_S,
                        Stroke::new(2.0, theme::text()),
                        StrokeKind::Outside,
                    );
                }
                if r.clicked() {
                    item.color = Some(col);
                }
            }
        });

        let touch = &mut cx.engine.show_mut().touch;
        if remove {
            if let Some(p) = touch.page_mut(page) {
                p.items.retain(|i| i.id != sel);
            }
            cx.state.touch_sel = None;
            return;
        }
        if item != before {
            if let Some(p) = touch.page_mut(page) {
                // Größe nur übernehmen, wenn der Platz frei ist
                if !p.is_free(item.x, item.y, item.w, item.h, Some(item.id)) {
                    item.w = before.w;
                    item.h = before.h;
                }
                if let Some(slot) = p.items.iter_mut().find(|i| i.id == sel) {
                    *slot = item;
                }
            }
        }
        if duplicate {
            cx.state.touch_sel = touch.duplicate(page, sel);
        }
    }
}

fn add_page(cx: &mut ViewCtx<'_>) {
    let touch = &mut cx.engine.show_mut().touch;
    let n = touch.pages.len() + 1;
    let id = touch.add_page(tr!("Seite {n}", n = n));
    cx.state.touch_page = Some(id);
}

fn kind_name(k: TouchKind) -> &'static str {
    match k {
        TouchKind::Label => t("Beschriftung"),
        TouchKind::Button => t("Schaltfläche"),
        TouchKind::Fader => t("Fader"),
        TouchKind::Knob => t("Drehregler"),
        TouchKind::Color => t("Farbrad"),
        TouchKind::Xy => t("XY-Feld"),
    }
}

/// Ziel eines Elements als Text.
fn item_target(show: &Show, item: &TouchItem) -> String {
    match item.control {
        None => t("keins – Alt + Klick auf ein Bedienelement").to_string(),
        Some(c) => {
            let m = mapping_of(item).unwrap_or_else(|| Mapping {
                id: 0,
                control: c,
                element: None,
                input: None,
                mode: ButtonMode::Toggle,
                flash: false,
                feedback: None,
                name: None,
                channels: item.channels.clone(),
            });
            let (cat, target) = crate::mapping::describe_mapping(show, &m);
            format!("{cat}: {target}")
        }
    }
}

fn item_title(show: &Show, item: &TouchItem) -> String {
    if item.label.is_empty() {
        kind_name(item.kind).to_string()
    } else {
        let _ = show;
        item.label.clone()
    }
}

/// Beschriftung: eigene oder aus dem Ziel.
fn display_label(show: &Show, item: &TouchItem) -> String {
    if !item.label.is_empty() {
        return item.label.clone();
    }
    match item.control {
        Some(_) => {
            let full = item_target(show, item);
            full.split_once(": ")
                .map_or(full.clone(), |x| x.1.to_string())
        }
        None => String::new(),
    }
}

/// Fläche unter der Beschriftung (Farbrad, XY-Feld).
fn content_rect(rect: Rect, item: &TouchItem) -> Rect {
    let top = if item.label.is_empty() { 6.0 } else { 24.0 };
    Rect::from_min_max(rect.min + vec2(6.0, top), rect.max - vec2(6.0, 6.0))
}

fn fader_track(rect: Rect) -> Rect {
    Rect::from_min_max(
        pos2(rect.center().x - 10.0, rect.top() + 28.0),
        pos2(rect.center().x + 10.0, rect.bottom() - 10.0),
    )
}

/// Element zeichnen. `value`: Fader/Drehregler 0–1, Schaltfläche 1 = gedrückt.
fn paint_item(
    p: &egui::Painter,
    e: &Engine,
    st: &AppState,
    states: &HashMap<u64, bool>,
    item: &TouchItem,
    rect: Rect,
    value: f32,
) {
    let show = e.show();
    let base = item.color.map(theme::rgb);
    let label = display_label(show, item);
    let auto = (rect.height().min(rect.width()) * 0.16).clamp(11.0, 22.0);
    let size = if item.text_size > 0.0 {
        item.text_size
    } else {
        auto
    };
    let text_color = |bg: Color32| {
        let l = 0.299 * f32::from(bg.r()) + 0.587 * f32::from(bg.g()) + 0.114 * f32::from(bg.b());
        if l > 150.0 {
            Color32::from_gray(20)
        } else {
            Color32::WHITE
        }
    };
    match item.kind {
        TouchKind::Label => {
            if let Some(b) = base {
                p.rect_filled(rect, theme::R_S, b.gamma_multiply(0.25));
            }
            p.text(
                rect.center(),
                Align2::CENTER_CENTER,
                label,
                egui::FontId::proportional(size.max(14.0)),
                base.unwrap_or(theme::text()),
            );
        }
        TouchKind::Button => {
            let on = mapping_of(item).is_some_and(|m| crate::mapping::is_on(e, st, states, &m));
            let fill = base.unwrap_or(theme::raised());
            let fill = if value > 0.5 {
                fill.gamma_multiply(0.7)
            } else {
                fill
            };
            p.rect_filled(rect, theme::R_M, fill);
            if on {
                p.rect_stroke(
                    rect,
                    theme::R_M,
                    Stroke::new(3.0, Color32::WHITE),
                    StrokeKind::Inside,
                );
            }
            let galley = p.layout(
                label,
                egui::FontId::proportional(size),
                text_color(fill),
                rect.width() - 12.0,
            );
            p.galley(rect.center() - galley.size() * 0.5, galley, Color32::WHITE);
            if item.control.is_none() {
                p.circle_filled(rect.right_bottom() - vec2(9.0, 9.0), 3.0, theme::DANGER);
            }
        }
        TouchKind::Fader => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let track = fader_track(rect);
            p.rect_filled(track, 4.0, theme::bg());
            let fill_col = base.unwrap_or(theme::accent());
            let filled = Rect::from_min_max(
                pos2(track.left(), track.bottom() - track.height() * value),
                track.max,
            );
            p.rect_filled(filled, 4.0, fill_col);
            p.text(
                pos2(rect.center().x, rect.top() + 14.0),
                Align2::CENTER_CENTER,
                format!("{:.0} %", value * 100.0),
                theme::caption(),
                theme::text_dim(),
            );
            if !label.is_empty() {
                let g = p.layout(
                    label,
                    egui::FontId::proportional(11.0),
                    theme::text(),
                    rect.width() - 4.0,
                );
                let pos = pos2(
                    rect.center().x - g.size().x * 0.5,
                    track.top() - g.size().y - 2.0,
                );
                let _ = pos;
                p.galley(
                    pos2(
                        rect.center().x - g.size().x * 0.5,
                        rect.bottom() - g.size().y - 2.0,
                    ),
                    g,
                    Color32::WHITE,
                );
            }
        }
        TouchKind::Knob => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let c = rect.center() - vec2(0.0, 6.0);
            let r = (rect.width().min(rect.height()) * 0.32).max(10.0);
            let start = std::f32::consts::PI * 0.75;
            let sweep = std::f32::consts::PI * 1.5;
            let arc = |to: f32| -> Vec<Pos2> {
                (0..=32)
                    .map(|i| c + egui::Vec2::angled(start + to * i as f32 / 32.0) * r)
                    .collect()
            };
            p.add(Shape::line(arc(sweep), Stroke::new(5.0, theme::bg())));
            if value > 0.001 {
                p.add(Shape::line(
                    arc(sweep * value),
                    Stroke::new(5.0, base.unwrap_or(theme::accent())),
                ));
            }
            p.text(
                c,
                Align2::CENTER_CENTER,
                format!("{:.0}", value * 100.0),
                theme::caption(),
                theme::text(),
            );
            if !label.is_empty() {
                p.text(
                    pos2(rect.center().x, rect.bottom() - 10.0),
                    Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(11.0),
                    theme::text(),
                );
            }
        }
        TouchKind::Color => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(8.0, 12.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    theme::text(),
                );
            }
            let field = content_rect(rect, item);
            let mut mesh = Mesh::default();
            const N: u32 = 24;
            for yi in 0..=1u32 {
                for xi in 0..=N {
                    let h = xi as f32 / N as f32;
                    let s = 1.0 - yi as f32;
                    mesh.colored_vertex(
                        pos2(
                            field.left() + field.width() * h,
                            field.top() + field.height() * yi as f32,
                        ),
                        egui::ecolor::Hsva::new(h, s, 1.0, 1.0).into(),
                    );
                }
            }
            for xi in 0..N {
                mesh.add_triangle(xi, xi + 1, xi + N + 1);
                mesh.add_triangle(xi + 1, xi + N + 2, xi + N + 1);
            }
            p.add(Shape::mesh(mesh));
            // aktuelle Farbe des ersten Geräts
            if let Some(rgb) = item.fixtures.iter().find_map(|f| current_color(e, *f)) {
                let hsv = egui::ecolor::Hsva::from_srgb(rgb);
                let m = pos2(
                    field.left() + hsv.h * field.width(),
                    field.top() + (1.0 - hsv.s) * field.height(),
                );
                p.circle_stroke(m, 7.0, Stroke::new(2.0, Color32::BLACK));
                p.circle_stroke(m, 5.0, Stroke::new(2.0, Color32::WHITE));
            }
        }
        TouchKind::Xy => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(8.0, 12.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    theme::text(),
                );
            }
            let field = content_rect(rect, item);
            p.rect_filled(field, theme::R_S, theme::bg());
            for i in 1..4 {
                let f = i as f32 / 4.0;
                let s = Stroke::new(1.0, theme::raised());
                p.line_segment(
                    [
                        pos2(field.left() + field.width() * f, field.top()),
                        pos2(field.left() + field.width() * f, field.bottom()),
                    ],
                    s,
                );
                p.line_segment(
                    [
                        pos2(field.left(), field.top() + field.height() * f),
                        pos2(field.right(), field.top() + field.height() * f),
                    ],
                    s,
                );
            }
            if let Some((x, y)) = item.fixtures.iter().find_map(|f| current_position(e, *f)) {
                let m = pos2(
                    field.left() + x * field.width(),
                    field.top() + y * field.height(),
                );
                p.circle_filled(m, 7.0, base.unwrap_or(theme::accent()));
            }
        }
    }
}

/// Kanal einer Art eines Geräts.
fn channels_of(show: &Show, f: FixtureId, kind: ChannelKind) -> Vec<ChannelRef> {
    show.fixture(f)
        .and_then(|fx| show.type_of(fx))
        .map(|ty| {
            ty.channels
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == kind)
                .map(|(i, _)| ChannelRef {
                    fixture: f,
                    channel: i as u16,
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Farbe als Live-Werte setzen (Rot/Grün/Blau aller Zellen).
fn set_color(e: &mut Engine, fixtures: &[FixtureId], rgb: [u8; 3]) {
    for f in fixtures {
        for (kind, v) in [
            (ChannelKind::Red, rgb[0]),
            (ChannelKind::Green, rgb[1]),
            (ChannelKind::Blue, rgb[2]),
        ] {
            for ch in channels_of(e.show(), *f, kind) {
                e.set_live(ch, v);
            }
        }
    }
}

fn current_color(e: &Engine, f: FixtureId) -> Option<[u8; 3]> {
    let get = |k| {
        channels_of(e.show(), f, k)
            .first()
            .and_then(|c| e.live().get(c).copied())
    };
    Some([
        get(ChannelKind::Red)?,
        get(ChannelKind::Green)?,
        get(ChannelKind::Blue)?,
    ])
}

/// Pan/Tilt als Live-Werte setzen (0–1, Feinkanäle 0).
fn set_position(e: &mut Engine, fixtures: &[FixtureId], x: f32, y: f32) {
    for f in fixtures {
        for (kind, v) in [
            (ChannelKind::Pan, (x * 255.0).round() as u8),
            (ChannelKind::Tilt, (y * 255.0).round() as u8),
            (ChannelKind::PanFine, 0),
            (ChannelKind::TiltFine, 0),
        ] {
            for ch in channels_of(e.show(), *f, kind) {
                e.set_live(ch, v);
            }
        }
    }
}

fn current_position(e: &Engine, f: FixtureId) -> Option<(f32, f32)> {
    let get = |k| {
        channels_of(e.show(), f, k)
            .first()
            .and_then(|c| e.live().get(c).copied())
    };
    let x = get(ChannelKind::Pan)?;
    let y = get(ChannelKind::Tilt).unwrap_or(128);
    Some((f32::from(x) / 255.0, f32::from(y) / 255.0))
}

/// Vorgemerktes Element (Alt + Klick) übernehmen: dem gewählten Element als
/// Ziel geben (Touch bearbeiten) oder auf der aktuellen Seite anlegen.
pub fn apply_pick(e: &mut Engine, st: &mut AppState, pick: crate::mapping::TouchPick) {
    use crate::state::Mode;
    let show = e.show();
    let label = match (&pick.name, pick.control) {
        (Some(n), _) => n.1.clone(),
        (None, Some(c)) if !pick.channels.is_empty() || matches!(c, Control::Channels) => {
            crate::mapping::channel_name(
                show,
                pick.channels
                    .first()
                    .copied()
                    .unwrap_or(ChannelRef::virtual_dimmer(FixtureId(0))),
            )
        }
        (None, Some(c)) => crate::mapping::describe(show, &c).1,
        (None, None) => String::new(),
    };
    // Programmer-Fader: fest die gerade gesteuerten Kanäle
    let (control, channels) = if pick.channels.is_empty() {
        (pick.control, Vec::new())
    } else {
        (Some(Control::Channels), pick.channels.clone())
    };
    let touch = &mut e.show_mut().touch;
    // gewähltes Element im Bearbeiten-Modus bekommt das Ziel
    if st.mode == Mode::Touch && st.touch_edit {
        if let (Some(page), Some(sel)) = (st.touch_page, st.touch_sel) {
            if let Some(item) = touch
                .page_mut(page)
                .and_then(|p| p.items.iter_mut().find(|i| i.id == sel))
            {
                if item.kind.uses_fixtures() {
                    item.fixtures = pick.fixtures.clone();
                } else if item.kind != TouchKind::Label {
                    item.control = control;
                    item.channels = channels;
                }
                st.info(t("Ziel übernommen"));
                return;
            }
        }
    }
    let page = match st.touch_page.filter(|p| touch.page(*p).is_some()) {
        Some(p) => p,
        None => match touch.pages.first() {
            Some(p) => p.id,
            None => {
                let n = touch.pages.len() + 1;
                touch.add_page(tr!("Seite {n}", n = n))
            }
        },
    };
    st.touch_page = Some(page);
    let mut item = TouchItem::new(pick.kind);
    item.control = control;
    item.channels = channels;
    item.fixtures = pick.fixtures;
    if pick.kind.uses_fixtures() {
        item.label = kind_name(pick.kind).to_string();
    } else if pick.kind == TouchKind::Fader {
        item.label = label
            .split_once(": ")
            .map_or(label.clone(), |x| x.1.to_string());
    }
    if let Some(Control::Scene {
        target: dmxcraft_core::mapping::SceneTarget::Scene { id },
    }) = item.control
    {
        if let Some(s) = e.show().scene(id) {
            item.color = s.color;
            item.flash = s.settings.flash;
            item.label = s.name.clone();
        }
    }
    let name = touch_page_name(e.show(), page);
    if let Some(id) = e.show_mut().touch.add_item(page, item) {
        st.touch_sel = Some(id);
        st.info(tr!("Zur Touch-Seite „{page}“ hinzugefügt", page = name));
    }
}

fn touch_page_name(show: &Show, page: u32) -> String {
    show.touch
        .page(page)
        .map_or_else(String::new, |p: &TouchPage| p.name.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::TouchPick;
    use dmxcraft_core::mapping::SceneTarget;
    use dmxcraft_core::Step;

    #[test]
    fn pick_adds_scene_button_and_assigns_to_selection() {
        let mut show = Show::new("T", 8);
        let z = show.add_zone("A");
        let a = show.add_scene("Intro", Some(z), vec![Step::new(Vec::new())]);
        let mut e = Engine::new(show);
        let mut st = AppState::new(String::new());
        let scene = Control::Scene {
            target: SceneTarget::Scene { id: a },
        };
        apply_pick(
            &mut e,
            &mut st,
            TouchPick {
                control: Some(scene),
                ..Default::default()
            },
        );
        let page = &e.show().touch.pages[0];
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].label, "Intro");
        assert_eq!(page.items[0].control, Some(scene));

        // gewähltes Element im Bearbeiten-Modus bekommt das Ziel
        st.mode = crate::state::Mode::Touch;
        st.touch_edit = true;
        apply_pick(
            &mut e,
            &mut st,
            TouchPick {
                control: Some(Control::Blackout),
                ..Default::default()
            },
        );
        let page = &e.show().touch.pages[0];
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].control, Some(Control::Blackout));
    }

    #[test]
    fn programmer_fader_becomes_fixed_channels() {
        let mut e = Engine::new(Show::new("T", 8));
        let mut st = AppState::new(String::new());
        let ch = ChannelRef {
            fixture: FixtureId(3),
            channel: 1,
        };
        apply_pick(
            &mut e,
            &mut st,
            TouchPick {
                kind: TouchKind::Fader,
                control: Some(Control::Ui {
                    key: 1,
                    fader: true,
                }),
                name: Some(("Programmer".into(), "Dimmer".into())),
                channels: vec![ch],
                ..Default::default()
            },
        );
        let item = &e.show().touch.pages[0].items[0];
        assert_eq!(item.control, Some(Control::Channels));
        assert_eq!(item.channels, vec![ch]);
        assert_eq!(item.kind, TouchKind::Fader);
    }
}
