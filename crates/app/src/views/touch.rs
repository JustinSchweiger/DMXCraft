//! TOUCH: frei angeordnete Bedienelemente auf Seiten. Im Bearbeiten-Modus
//! lassen sich Elemente anlegen, verschieben, in der Größe ändern,
//! beschriften, einfärben und mit Symbol oder Bild versehen; sonst werden sie
//! bedient.
//!
//! Positionen und Größen sind in Einheiten zu [`UNIT`] Bildpunkten (bei
//! 100 %) gespeichert. Das Raster (Schrittweite, Einrasten, Anzeige) ist
//! einstellbar.
//!
//! Elemente kommen per Alt + Klick auf ein Bedienelement (Szene, Bank,
//! Fader, Gerät …), über das Kontextmenü oder die Leiste „Hinzufügen“.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use dmxcraft_core::mapping::{ButtonMode, Control, Mapping};
use dmxcraft_core::touch::{TouchItem, TouchKind};
use dmxcraft_core::{ChannelKind, ChannelRef, Engine, FixtureId, Show};
use eframe::egui::{
    self, pos2, vec2, Align, Align2, Color32, Layout, Mesh, Pos2, Rect, Sense, Shape, Stroke,
    StrokeKind, Ui,
};

use crate::i18n::t;
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::dialog::{form, select_all};
use crate::ui::widgets::icon_library;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{section_header, text_field, Icon, IconButton, Toggle};
use crate::views::{View, ViewCtx};

/// Bildpunkte einer Einheit bei 100 %.
pub const UNIT: f32 = 72.0;
/// Abstand zwischen Elementen.
const GAP: f32 = 6.0;
/// Breite der Bearbeiten-Leiste rechts.
const PANEL_W: f32 = 300.0;
/// Rasterschritte (Bildpunkte bei 100 %).
const GRID_STEPS: [f32; 6] = [12.0, 18.0, 24.0, 36.0, 48.0, 72.0];

/// Farben für Flächen und Text.
const COLORS: [[u8; 3]; 14] = [
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
    [255, 255, 255],
    [120, 120, 130],
    [20, 20, 24],
];

/// Ziehen im Bearbeiten-Modus.
#[derive(Clone, Copy)]
enum Drag {
    Move { id: u32, grab: egui::Vec2 },
    Resize { id: u32 },
}

/// Wofür ein Bild geladen wird.
#[derive(Clone, Copy)]
enum ImageTarget {
    Page(u32),
    Item(u32, u32),
}

pub struct TouchView {
    /// Ansichtsgröße (1.0 = 100 %).
    zoom: f32,
    drag: Option<Drag>,
    rename_page: Option<(u32, String)>,
    rename_focus: bool,
    /// Zuletzt gesetzte Werte von Fadern ohne lesbaren Zustand.
    values: HashMap<u32, f32>,
    /// Gedrückte Schaltflächen (für Loslassen).
    held: Vec<u32>,
    textures: HashMap<u32, egui::TextureHandle>,
    image_dialog: Option<(ImageTarget, Receiver<Option<PathBuf>>)>,
}

impl Default for TouchView {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            drag: None,
            rename_page: None,
            rename_focus: false,
            values: HashMap::new(),
            held: Vec::new(),
            textures: HashMap::new(),
            image_dialog: None,
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
        self.poll_image(ui.ctx(), cx);
        let editing = cx.state.touch_edit && !cx.state.locked;
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
                    let r =
                        ui.add(egui::Button::new(t("Bearbeiten")).selected(cx.state.touch_edit));
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
                    self.zoom = (self.zoom + 0.1).min(2.5);
                }
                let r = ui
                    .add(
                        egui::Label::new(
                            egui::RichText::new(format!("{:.0} %", self.zoom * 100.0))
                                .color(theme::text_dim()),
                        )
                        .sense(Sense::click()),
                    )
                    .on_hover_text(t("Zurücksetzen"));
                if r.clicked() {
                    self.zoom = 1.0;
                }
                if ui
                    .add(IconButton::new(Icon::ZoomOut).flat())
                    .on_hover_text(t("Verkleinern"))
                    .clicked()
                {
                    self.zoom = (self.zoom - 0.1).max(0.4);
                }
            });
        });
        ui.add_space(theme::SP_S);

        let full = ui.available_rect_before_wrap();
        let (grid_rect, panel_rect) = if editing {
            let split = full.right() - PANEL_W;
            (
                Rect::from_min_max(full.min, pos2(split - theme::SP_M, full.bottom())),
                Some(Rect::from_min_max(pos2(split, full.top()), full.max)),
            )
        } else {
            (full, None)
        };
        ui.allocate_rect(full, Sense::hover());

        match cx.state.touch_page {
            None => self.empty(ui, grid_rect, cx),
            Some(page_id) => {
                let mut grid_ui = ui.new_child(egui::UiBuilder::new().max_rect(grid_rect));
                grid_ui.set_clip_rect(grid_rect);
                egui::ScrollArea::both()
                    .id_salt("touch_grid")
                    .auto_shrink([false, false])
                    .show(&mut grid_ui, |ui| self.grid(ui, cx, page_id, editing));
            }
        }
        if let Some(r) = panel_rect {
            self.panel(ui, r, cx, cx.state.touch_page);
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
            let touch = &mut cx.engine.show_mut().touch;
            touch.pages.retain(|p| p.id != id);
            touch.prune_images();
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
        child.add_space(rect.height() * 0.35);
        child.label(
            egui::RichText::new(t("Noch keine Touch-Oberfläche"))
                .color(theme::text_dim())
                .size(theme::FS_TITLE),
        );
        child.add_space(theme::SP_M);
        if !cx.state.locked && child.button(t("Seite hinzufügen")).clicked() {
            add_page(cx);
            cx.state.touch_edit = true;
        }
    }

    /// Bildpunkte je Einheit (mit Zoom).
    fn unit(&self) -> f32 {
        UNIT * self.zoom
    }

    fn texture(
        &mut self,
        ctx: &egui::Context,
        show: &Show,
        id: u32,
    ) -> Option<egui::TextureHandle> {
        if let Some(t) = self.textures.get(&id) {
            return Some(t.clone());
        }
        let img = show.touch.image(id)?;
        let rgba = image::load_from_memory(&img.data.0).ok()?.to_rgba8();
        let size = [rgba.width() as usize, rgba.height() as usize];
        let tex = ctx.load_texture(
            format!("touch_image_{id}"),
            egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()),
            egui::TextureOptions::LINEAR,
        );
        self.textures.insert(id, tex.clone());
        Some(tex)
    }

    /// Raster mit den Elementen einer Seite.
    fn grid(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>, page_id: u32, editing: bool) {
        let Some(page) = cx.engine.show().touch.page(page_id).cloned() else {
            return;
        };
        let (step_px, snap, show_grid) = {
            let t = &cx.engine.show().touch;
            (t.cell.max(6.0), t.snap, t.show_grid)
        };
        let u = self.unit();
        let step = step_px * self.zoom;
        // Fläche: sichtbarer Bereich oder bis hinter das letzte Element
        let right = page.items.iter().map(|i| i.x + i.w).fold(0.0, f32::max) + 2.0;
        let bottom = page.items.iter().map(|i| i.y + i.h).fold(0.0, f32::max) + 2.0;
        let avail = ui.available_size();
        let size = vec2((right * u).max(avail.x), (bottom * u).max(avail.y));
        let (area, bg) = ui.allocate_exact_size(size, Sense::click());
        let origin = area.min;
        let p = ui.painter().clone();

        // Hintergrund der Seite
        if let Some(c) = page.background_color {
            p.rect_filled(area, 0.0, theme::rgb(c));
        }
        if let Some(tex) = page
            .background
            .and_then(|id| self.texture(ui.ctx(), cx.engine.show(), id))
        {
            paint_cover(&p, ui.clip_rect().intersect(area), &tex, 1.0);
        }
        // Raster
        if show_grid || editing {
            let vis = ui.clip_rect().intersect(area);
            let alpha = if editing { 1.0 } else { 0.5 };
            let first_x = ((vis.left() - origin.x) / step).floor().max(0.0) as i32;
            let first_y = ((vis.top() - origin.y) / step).floor().max(0.0) as i32;
            let nx = (vis.width() / step).ceil() as i32 + 1;
            let ny = (vis.height() / step).ceil() as i32 + 1;
            if step >= 30.0 && show_grid {
                let gap = (step * 0.08).clamp(2.0, GAP);
                for y in first_y..first_y + ny {
                    for x in first_x..first_x + nx {
                        let r = Rect::from_min_size(
                            origin + vec2(x as f32 * step, y as f32 * step),
                            vec2(step - gap, step - gap),
                        );
                        p.rect_filled(r, theme::R_S, theme::raised().gamma_multiply(0.55 * alpha));
                    }
                }
            } else {
                // feines Raster: Punkte an den Schnittpunkten
                for y in first_y..first_y + ny {
                    for x in first_x..first_x + nx {
                        let c = origin + vec2(x as f32 * step, y as f32 * step);
                        p.circle_filled(c, 1.2, theme::border().gamma_multiply(alpha));
                    }
                }
            }
        }
        if editing && bg.clicked() {
            cx.state.touch_sel = None;
        }

        let item_rect = |i: &TouchItem| {
            Rect::from_min_size(
                origin + vec2(i.x * u, i.y * u),
                vec2((i.w * u - GAP).max(8.0), (i.h * u - GAP).max(8.0)),
            )
        };
        let states: HashMap<u64, bool> = crate::mapping::ui_states(ui.ctx());
        let mut changes: Vec<TouchItem> = Vec::new();
        let mut remove = None;
        let mut duplicate = None;
        for item in &page.items {
            let rect = item_rect(item);
            let tex = item
                .image
                .and_then(|id| self.texture(ui.ctx(), cx.engine.show(), id));
            let id = ui.make_persistent_id(("touch_item", item.id));
            let value = self.value(cx.engine, item);
            if editing {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                paint_item(
                    &p,
                    cx.engine,
                    cx.state,
                    &states,
                    item,
                    rect,
                    value,
                    false,
                    tex.as_ref(),
                );
                let sel = cx.state.touch_sel == Some(item.id);
                if sel || resp.hovered() {
                    p.rect_stroke(
                        rect.expand(2.0),
                        theme::R_M,
                        Stroke::new(if sel { 2.0 } else { 1.0 }, theme::accent()),
                        StrokeKind::Outside,
                    );
                }
                // Griff zum Größe ändern
                let handle =
                    Rect::from_min_size(rect.right_bottom() - vec2(16.0, 16.0), vec2(16.0, 16.0));
                let hresp = ui.interact(handle, id.with("resize"), Sense::drag());
                if sel || hresp.hovered() {
                    p.add(Shape::convex_polygon(
                        vec![
                            handle.right_top() + vec2(0.0, 4.0),
                            handle.right_bottom(),
                            handle.left_bottom() + vec2(4.0, 0.0),
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
                    menu::title(ui, &item_title(item));
                    target_entry(ui, cx.engine.show(), item);
                    menu::separator(ui);
                    if menu::item(ui, MenuItem::new(t("Duplizieren")).icon(Icon::Copy)) {
                        duplicate = Some(item.id);
                    }
                    if menu::item(ui, MenuItem::new(t("Löschen")).icon(Icon::Trash)) {
                        remove = Some(item.id);
                    }
                    menu::separator(ui);
                    let mut c = item.color;
                    if swatch_row(ui, &mut c, true) {
                        let mut i = item.clone();
                        i.color = c;
                        changes.push(i);
                        ui.close();
                    }
                });
            } else {
                self.play_item(ui, &p, cx, &states, item, rect, id, tex.as_ref());
            }
        }

        // Ziehen anwenden
        let snap_to = |v: f32| {
            if snap {
                let s = step_px / UNIT;
                (v / s).round() * s
            } else {
                (v * 100.0).round() / 100.0
            }
        };
        if let Some(d) = self.drag {
            let pointer = ui.input(|i| i.pointer.interact_pos());
            let released = ui.input(|i| i.pointer.any_released());
            let preview = match (d, pointer) {
                (Drag::Move { id, grab }, Some(pos)) => {
                    page.items.iter().find(|i| i.id == id).map(|it| {
                        let tl = (pos - grab - origin) / u;
                        let mut n = it.clone();
                        n.x = snap_to(tl.x).max(0.0);
                        n.y = snap_to(tl.y).max(0.0);
                        n
                    })
                }
                (Drag::Resize { id }, Some(pos)) => {
                    page.items.iter().find(|i| i.id == id).map(|it| {
                        let rel = (pos - origin) / u;
                        let min = if snap { step_px / UNIT } else { 0.25 };
                        let mut n = it.clone();
                        n.w = snap_to(rel.x - it.x).max(min);
                        n.h = snap_to(rel.y - it.y).max(min);
                        n
                    })
                }
                _ => None,
            };
            match preview {
                Some(n) => {
                    p.rect_stroke(
                        item_rect(&n),
                        theme::R_M,
                        Stroke::new(2.0, theme::accent()),
                        StrokeKind::Outside,
                    );
                    if released {
                        changes.push(n);
                        self.drag = None;
                    }
                }
                None => self.drag = None,
            }
        }

        // Entf löscht, Pfeiltasten verschieben das gewählte Element
        if editing && !ui.ctx().egui_wants_keyboard_input() {
            if ui.input(|i| i.key_pressed(egui::Key::Delete)) {
                remove = remove.or(cx.state.touch_sel);
            }
            if let Some(sel) = cx
                .state
                .touch_sel
                .and_then(|s| page.items.iter().find(|i| i.id == s))
            {
                let s = if snap { step_px / UNIT } else { 0.05 };
                let (dx, dy) = ui.input(|i| {
                    let mut d = (0.0, 0.0);
                    if i.key_pressed(egui::Key::ArrowLeft) {
                        d.0 -= s;
                    }
                    if i.key_pressed(egui::Key::ArrowRight) {
                        d.0 += s;
                    }
                    if i.key_pressed(egui::Key::ArrowUp) {
                        d.1 -= s;
                    }
                    if i.key_pressed(egui::Key::ArrowDown) {
                        d.1 += s;
                    }
                    d
                });
                if dx != 0.0 || dy != 0.0 {
                    let mut n = sel.clone();
                    n.x = (n.x + dx).max(0.0);
                    n.y = (n.y + dy).max(0.0);
                    changes.push(n);
                }
            }
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
        if remove.is_some() {
            touch.prune_images();
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
        tex: Option<&egui::TextureHandle>,
    ) {
        let value = self.value(cx.engine, item);
        match item.kind {
            TouchKind::Label => {
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, false, tex);
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
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, down, tex);
            }
            TouchKind::Fader | TouchKind::Knob => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let mut v = value;
                if item.kind == TouchKind::Fader {
                    if let Some(pos) = resp
                        .interact_pointer_pos()
                        .filter(|_| resp.dragged() || resp.clicked())
                    {
                        let travel = fader_parts(rect).1;
                        v = (1.0 - (pos.y - travel.top()) / travel.height()).clamp(0.0, 1.0);
                    }
                } else if resp.dragged() {
                    v = (value - resp.drag_delta().y / 160.0).clamp(0.0, 1.0);
                }
                if resp.double_clicked() {
                    v = if value > 0.0 { 0.0 } else { 1.0 };
                }
                if (v - value).abs() > f32::EPSILON {
                    self.values.insert(item.id, v);
                    if let Some(m) = mapping_of(item) {
                        let raw = (v * 127.0).round() as u8;
                        cx.state.touch_fire.push((m, raw > 0, raw, true));
                    }
                }
                let hot = resp.hovered() || resp.dragged();
                paint_item(p, cx.engine, cx.state, states, item, rect, v, hot, tex);
            }
            TouchKind::Color => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let field = content_rect(rect, item);
                if let Some(pos) = resp.interact_pointer_pos().filter(|p| {
                    (resp.dragged() || resp.clicked()) && field.expand(4.0).contains(*p)
                }) {
                    let h = ((pos.x - field.left()) / field.width()).clamp(0.0, 1.0);
                    let s = 1.0 - ((pos.y - field.top()) / field.height()).clamp(0.0, 1.0);
                    let rgb = egui::ecolor::Hsva::new(h, s, 1.0, 1.0).to_srgb();
                    set_color(cx.engine, &item.fixtures, [rgb[0], rgb[1], rgb[2]]);
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, false, tex);
            }
            TouchKind::Xy => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let field = content_rect(rect, item);
                if let Some(pos) = resp.interact_pointer_pos().filter(|p| {
                    (resp.dragged() || resp.clicked()) && field.expand(4.0).contains(*p)
                }) {
                    let x = ((pos.x - field.left()) / field.width()).clamp(0.0, 1.0);
                    let y = ((pos.y - field.top()) / field.height()).clamp(0.0, 1.0);
                    set_position(cx.engine, &item.fixtures, x, y);
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, false, tex);
            }
        }
    }

    /// Bearbeiten-Leiste: Seite, Raster, neue Elemente, Eigenschaften.
    fn panel(&mut self, ui: &mut Ui, rect: Rect, cx: &mut ViewCtx<'_>, page: Option<u32>) {
        ui.painter().line_segment(
            [
                rect.left_top() - vec2(theme::SP_M * 0.5, 0.0),
                rect.left_bottom() - vec2(theme::SP_M * 0.5, 0.0),
            ],
            Stroke::new(1.0, theme::border()),
        );
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        egui::ScrollArea::vertical()
            .id_salt("touch_panel")
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                ui.set_width(rect.width() - theme::SP_S);
                let Some(page) = page else {
                    return;
                };
                // ----- Hinzufügen -----
                section_header(ui, t("Hinzufügen"), |_| {});
                let kinds = [
                    (TouchKind::Button, "square", t("Schaltfläche")),
                    (TouchKind::Fader, "sliders", t("Fader")),
                    (TouchKind::Knob, "circle", t("Drehregler")),
                    (TouchKind::Label, "x", t("Beschriftung")),
                    (TouchKind::Color, "palette", t("Farbrad")),
                    (TouchKind::Xy, "plus", t("XY-Feld")),
                ];
                let w = (ui.available_width() - 2.0 * theme::SP_S) / 3.0;
                for row in kinds.chunks(3) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = theme::SP_S;
                        for (k, icon, label) in row {
                            if add_tile(ui, w, *k, icon, label) {
                                let mut item = TouchItem::new(*k);
                                if *k == TouchKind::Label {
                                    item.label = t("Beschriftung").to_string();
                                }
                                if k.uses_fixtures() {
                                    item.fixtures = cx.state.selection_order.clone();
                                }
                                cx.state.touch_sel =
                                    cx.engine.show_mut().touch.add_item(page, item);
                            }
                        }
                    });
                    ui.add_space(theme::SP_S);
                }

                // ----- Eigenschaften des gewählten Elements -----
                if let Some(sel) = cx.state.touch_sel {
                    ui.add_space(theme::SP_S);
                    self.properties(ui, cx, page, sel);
                    return;
                }

                // ----- Seite -----
                ui.add_space(theme::SP_S);
                section_header(ui, t("Seite"), |_| {});
                let (mut bg, mut img) = cx
                    .engine
                    .show()
                    .touch
                    .page(page)
                    .map_or((None, None), |p| (p.background_color, p.background));
                form(ui, "touch_page", |f| {
                    f.row(t("Hintergrund"), |ui| {
                        swatch_row(ui, &mut bg, true);
                    });
                    f.row(t("Bild"), |ui| {
                        if ui.button(t("Laden …")).clicked() {
                            self.image_dialog =
                                Some((ImageTarget::Page(page), pick_image(ui.ctx())));
                        }
                        if img.is_some() && ui.button(t("Entfernen")).clicked() {
                            img = None;
                        }
                    });
                });
                if let Some(p) = cx.engine.show().touch.page(page) {
                    if p.background_color != bg || p.background != img {
                        let touch = &mut cx.engine.show_mut().touch;
                        if let Some(p) = touch.page_mut(page) {
                            p.background_color = bg;
                            p.background = img;
                        }
                        touch.prune_images();
                    }
                }

                // ----- Raster -----
                ui.add_space(theme::SP_M);
                section_header(ui, t("Raster"), |_| {});
                let tl = cx.engine.show().touch.clone();
                let (mut show_grid, mut snap, mut cell) = (tl.show_grid, tl.snap, tl.cell);
                form(ui, "touch_grid", |f| {
                    f.row(t("Anzeigen"), |ui| {
                        ui.add(Toggle::new(&mut show_grid, ""));
                    });
                    f.row(t("Einrasten"), |ui| {
                        ui.add(Toggle::new(&mut snap, ""));
                    });
                    f.row(t("Schrittweite"), |ui| {
                        egui::ComboBox::from_id_salt("touch_step")
                            .selected_text(format!("{cell:.0} px"))
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for s in GRID_STEPS {
                                    ui.selectable_value(&mut cell, s, format!("{s:.0} px"));
                                }
                            });
                    });
                });
                if (show_grid, snap, cell) != (tl.show_grid, tl.snap, tl.cell) {
                    let touch = &mut cx.engine.show_mut().touch;
                    touch.show_grid = show_grid;
                    touch.snap = snap;
                    touch.cell = cell;
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
        section_header(ui, kind_name(item.kind), |ui| {
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
            if ui
                .add(IconButton::new(Icon::Close).flat())
                .on_hover_text(t("Auswahl aufheben"))
                .clicked()
            {
                cx.state.touch_sel = None;
            }
        });
        let show = cx.engine.show();
        if item.kind != TouchKind::Label {
            let text = if item.kind.uses_fixtures() {
                match item.fixtures.len() {
                    0 => t("keine Geräte").to_string(),
                    1 => show
                        .fixture(item.fixtures[0])
                        .map_or_else(String::new, |f| f.name.clone()),
                    n => crate::tr!("{n} Geräte", n = n),
                }
            } else {
                item_target(show, &item)
            };
            ui.label(egui::RichText::new(text).color(theme::text_dim()));
            ui.add_space(theme::SP_XS);
        }
        let mut pick_img = false;
        form(ui, "touch_props", |f| {
            f.row(t("Beschriftung"), |ui| {
                ui.add(text_field(&mut item.label).desired_width(ui.available_width()));
            });
            f.row(t("Textgröße"), |ui| {
                ui.add(
                    egui::DragValue::new(&mut item.text_size)
                        .range(0.0..=96.0)
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
            f.row(t("Textfarbe"), |ui| {
                swatch_row(ui, &mut item.text_color, true);
            });
            f.row(t("Farbe"), |ui| {
                swatch_row(ui, &mut item.color, true);
            });
            if matches!(item.kind, TouchKind::Button | TouchKind::Label) {
                f.row(t("Symbol"), |ui| {
                    icon_picker(ui, ("touch_icon", sel), &mut item.icon);
                });
                f.row(t("Bild"), |ui| {
                    if ui.button(t("Laden …")).clicked() {
                        pick_img = true;
                    }
                    if item.image.is_some() && ui.button(t("Entfernen")).clicked() {
                        item.image = None;
                    }
                });
            }
            f.row(t("Größe"), |ui| {
                ui.add(
                    egui::DragValue::new(&mut item.w)
                        .range(0.25..=40.0)
                        .speed(0.05)
                        .max_decimals(2),
                );
                ui.label("×");
                ui.add(
                    egui::DragValue::new(&mut item.h)
                        .range(0.25..=40.0)
                        .speed(0.05)
                        .max_decimals(2),
                );
            });
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
                    ui.add(Toggle::new(&mut item.flash, ""));
                });
            }
        });
        if pick_img {
            self.image_dialog = Some((ImageTarget::Item(page, sel), pick_image(ui.ctx())));
        }

        let touch = &mut cx.engine.show_mut().touch;
        if remove {
            if let Some(p) = touch.page_mut(page) {
                p.items.retain(|i| i.id != sel);
            }
            touch.prune_images();
            cx.state.touch_sel = None;
            return;
        }
        if item != before {
            let image_changed = item.image != before.image;
            if let Some(slot) = touch
                .page_mut(page)
                .and_then(|p| p.items.iter_mut().find(|i| i.id == sel))
            {
                *slot = item;
            }
            if image_changed {
                touch.prune_images();
            }
        }
        if duplicate {
            cx.state.touch_sel = touch.duplicate(page, sel);
        }
    }

    /// Ergebnis des Bild-Dialogs übernehmen.
    fn poll_image(&mut self, ctx: &egui::Context, cx: &mut ViewCtx<'_>) {
        let Some((target, rx)) = &self.image_dialog else {
            return;
        };
        let Ok(path) = rx.try_recv() else {
            return;
        };
        let target = *target;
        self.image_dialog = None;
        let Some(path) = path else {
            return;
        };
        match prepare_image(&path) {
            Ok(bytes) => {
                let name = path
                    .file_name()
                    .map_or_else(String::new, |n| n.to_string_lossy().to_string());
                let touch = &mut cx.engine.show_mut().touch;
                let id = touch.add_image(name, bytes);
                match target {
                    ImageTarget::Page(p) => {
                        if let Some(p) = touch.page_mut(p) {
                            p.background = Some(id);
                        }
                    }
                    ImageTarget::Item(p, i) => {
                        if let Some(it) = touch
                            .page_mut(p)
                            .and_then(|p| p.items.iter_mut().find(|x| x.id == i))
                        {
                            it.image = Some(id);
                        }
                    }
                }
                touch.prune_images();
                ctx.request_repaint();
            }
            Err(e) => cx.state.message = Some((e, true)),
        }
    }
}

/// Kachel „Element hinzufügen“: Symbol und Name.
fn add_tile(ui: &mut Ui, w: f32, kind: TouchKind, icon: &str, label: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 64.0), Sense::click());
    let p = ui.painter();
    let fill = if resp.hovered() {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_M, fill);
    p.rect_stroke(
        rect,
        theme::R_M,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );
    let icon_c = pos2(rect.center().x, rect.top() + 24.0);
    let color = if resp.hovered() {
        theme::accent()
    } else {
        theme::text_dim()
    };
    match kind {
        TouchKind::Label => {
            p.text(
                icon_c,
                Align2::CENTER_CENTER,
                "Aa",
                egui::FontId::proportional(18.0),
                color,
            );
        }
        _ => icon_library::paint(p, icon_c, icon, 22.0, color),
    }
    p.text(
        pos2(rect.center().x, rect.bottom() - 13.0),
        Align2::CENTER_CENTER,
        label,
        theme::caption(),
        theme::text(),
    );
    resp.clicked()
}

/// Farbfelder (optional mit „keine“). Gibt `true` zurück, wenn geändert.
fn swatch_row(ui: &mut Ui, value: &mut Option<[u8; 3]>, allow_none: bool) -> bool {
    let mut changed = false;
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(3.0, 3.0);
        const S: f32 = 16.0;
        if allow_none {
            let (r, resp) = ui.allocate_exact_size(vec2(S, S), Sense::click());
            let p = ui.painter();
            p.rect_stroke(
                r,
                3.0,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            p.line_segment(
                [r.left_bottom(), r.right_top()],
                Stroke::new(1.0, theme::DANGER),
            );
            if value.is_none() {
                p.rect_stroke(
                    r.expand(2.0),
                    4.0,
                    Stroke::new(1.5, theme::text()),
                    StrokeKind::Outside,
                );
            }
            if resp.clicked() && value.is_some() {
                *value = None;
                changed = true;
            }
        }
        for c in COLORS {
            let (r, resp) = ui.allocate_exact_size(vec2(S, S), Sense::click());
            let p = ui.painter();
            p.rect_filled(r, 3.0, theme::rgb(c));
            if *value == Some(c) {
                p.rect_stroke(
                    r.expand(2.0),
                    4.0,
                    Stroke::new(1.5, theme::text()),
                    StrokeKind::Outside,
                );
            }
            if resp.clicked() && *value != Some(c) {
                *value = Some(c);
                changed = true;
            }
        }
        // eigene Farbe
        let mut rgb = value.unwrap_or([255, 255, 255]);
        let custom = value.is_some_and(|v| !COLORS.contains(&v));
        let (r, resp) = ui.allocate_exact_size(vec2(S, S), Sense::click());
        {
            let p = ui.painter();
            if custom {
                p.rect_filled(r, 3.0, theme::rgb(rgb));
                p.rect_stroke(
                    r.expand(2.0),
                    4.0,
                    Stroke::new(1.5, theme::text()),
                    StrokeKind::Outside,
                );
            } else {
                p.rect_stroke(
                    r,
                    3.0,
                    Stroke::new(1.0, theme::border()),
                    StrokeKind::Inside,
                );
                p.text(
                    r.center(),
                    Align2::CENTER_CENTER,
                    "+",
                    theme::caption(),
                    theme::text_dim(),
                );
            }
        }
        if resp.clicked() {
            egui::Popup::toggle_id(ui.ctx(), resp.id);
        }
        menu::dropdown(&resp, |ui| {
            ui.set_min_width(212.0);
            if crate::ui::widgets::color_picker::color_picker(ui, resp.id.with("picker"), &mut rgb)
            {
                *value = Some(rgb);
                changed = true;
            }
        });
    });
    changed
}

/// Knopf mit dem gewählten Symbol; Klick öffnet die Symbolbibliothek.
fn icon_picker(ui: &mut Ui, id: impl std::hash::Hash, icon: &mut Option<String>) {
    let (rect, resp) = ui.allocate_exact_size(vec2(40.0, 28.0), Sense::click());
    let p = ui.painter();
    p.rect_filled(
        rect,
        theme::R_S,
        if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::bg()
        },
    );
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );
    match icon {
        Some(name) => icon_library::paint(p, rect.center(), name, 18.0, theme::text()),
        None => {
            p.text(
                rect.center(),
                Align2::CENTER_CENTER,
                "–",
                theme::body(),
                theme::text_faint(),
            );
        }
    }
    let _ = id;
    if resp.clicked() {
        egui::Popup::toggle_id(ui.ctx(), resp.id);
    }
    menu::dropdown(&resp, |ui| {
        ui.set_width(10.0 * 32.0);
        if ui.button(t("Kein Symbol")).clicked() {
            *icon = None;
            ui.close();
        }
        egui::ScrollArea::vertical()
            .max_height(280.0)
            .show(ui, |ui| {
                egui::Grid::new(resp.id.with("grid"))
                    .spacing(vec2(2.0, 2.0))
                    .show(ui, |ui| {
                        for (i, (name, _)) in icon_library::LIBRARY.iter().enumerate() {
                            let (r, cell) =
                                ui.allocate_exact_size(vec2(30.0, 30.0), Sense::click());
                            let on = icon.as_deref() == Some(*name);
                            if on || cell.hovered() {
                                ui.painter().rect_filled(r, 3.0, theme::raised_hover());
                            }
                            icon_library::paint(
                                ui.painter(),
                                r.center(),
                                name,
                                18.0,
                                if on { theme::accent() } else { theme::text() },
                            );
                            if cell.clicked() {
                                *icon = Some((*name).to_string());
                                ui.close();
                            }
                            if i % 10 == 9 {
                                ui.end_row();
                            }
                        }
                    });
            });
    });
}

/// Menüeintrag mit dem Ziel des Elements (zur Fehlersuche).
fn target_entry(ui: &mut Ui, show: &Show, item: &TouchItem) {
    let text = if item.kind.uses_fixtures() {
        let names: Vec<String> = item
            .fixtures
            .iter()
            .filter_map(|f| show.fixture(*f).map(|f| f.name.clone()))
            .collect();
        if names.is_empty() {
            t("keine Geräte").to_string()
        } else {
            names.join(", ")
        }
    } else if item.kind == TouchKind::Label {
        return;
    } else {
        item_target(show, item)
    };
    ui.add(MenuItem::new(&crate::tr!("Ziel: {target}", target = text)).enabled(false));
    if !item.channels.is_empty() {
        let chans: Vec<String> = item
            .channels
            .iter()
            .map(|c| crate::mapping::channel_name(show, *c))
            .collect();
        for c in chans.iter().take(8) {
            ui.add(MenuItem::new(c).enabled(false));
        }
        if chans.len() > 8 {
            ui.add(
                MenuItem::new(&crate::tr!("… und {n} weitere", n = chans.len() - 8)).enabled(false),
            );
        }
    }
}

fn pick_image(ctx: &egui::Context) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Bild laden"))
            .add_filter(t("Bilder"), &["png", "jpg", "jpeg"])
            .pick_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

/// Bild lesen und auf eine sinnvolle Größe bringen (max. 1600 px).
fn prepare_image(path: &std::path::Path) -> Result<Vec<u8>, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let img = image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
    if img.width().max(img.height()) <= 1600 && bytes.len() < 1_500_000 {
        return Ok(bytes);
    }
    let img = img.resize(1600, 1600, image::imageops::FilterType::Triangle);
    let mut out = std::io::Cursor::new(Vec::new());
    if img.color().has_alpha() {
        img.write_to(&mut out, image::ImageFormat::Png)
    } else {
        img.to_rgb8().write_to(&mut out, image::ImageFormat::Jpeg)
    }
    .map_err(|e| e.to_string())?;
    Ok(out.into_inner())
}

/// Bild füllend (beschnitten) zeichnen.
fn paint_cover(p: &egui::Painter, rect: Rect, tex: &egui::TextureHandle, alpha: f32) {
    let [w, h] = tex.size();
    let (iw, ih) = (w as f32, h as f32);
    if iw <= 0.0 || ih <= 0.0 || rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let ra = rect.width() / rect.height();
    let ia = iw / ih;
    let uv = if ia > ra {
        let f = ra / ia;
        Rect::from_min_max(pos2((1.0 - f) * 0.5, 0.0), pos2((1.0 + f) * 0.5, 1.0))
    } else {
        let f = ia / ra;
        Rect::from_min_max(pos2(0.0, (1.0 - f) * 0.5), pos2(1.0, (1.0 + f) * 0.5))
    };
    p.image(tex.id(), rect, uv, Color32::WHITE.gamma_multiply(alpha));
}

fn add_page(cx: &mut ViewCtx<'_>) {
    let touch = &mut cx.engine.show_mut().touch;
    let n = touch.pages.len() + 1;
    let id = touch.add_page(crate::tr!("Seite {n}", n = n));
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
    match (mapping_of(item), item.control) {
        (Some(m), Some(_)) => {
            let (cat, target) = crate::mapping::describe_mapping(show, &m);
            format!("{cat}: {target}")
        }
        _ => t("kein Ziel").to_string(),
    }
}

fn item_title(item: &TouchItem) -> String {
    if item.label.is_empty() {
        kind_name(item.kind).to_string()
    } else {
        item.label.clone()
    }
}

/// Beschriftung: eigene oder aus dem Ziel.
fn display_label(show: &Show, item: &TouchItem) -> String {
    if !item.label.is_empty() {
        return item.label.clone();
    }
    if item.control.is_some() {
        let full = item_target(show, item);
        return full
            .split_once(": ")
            .map_or(full.clone(), |x| x.1.to_string());
    }
    String::new()
}

/// Fläche unter der Beschriftung (Farbrad, XY-Feld).
fn content_rect(rect: Rect, item: &TouchItem) -> Rect {
    let top = if item.label.is_empty() { 8.0 } else { 28.0 };
    Rect::from_min_max(rect.min + vec2(8.0, top), rect.max - vec2(8.0, 8.0))
}

/// Fader: (Schiene mit Griff, Weg des Griffmittelpunkts).
fn fader_parts(rect: Rect) -> (Rect, Rect) {
    const HANDLE_H: f32 = 34.0;
    let track = Rect::from_min_max(
        pos2(rect.left(), rect.top() + 30.0),
        pos2(rect.right(), rect.bottom() - 26.0),
    );
    let travel = Rect::from_min_max(
        pos2(track.left(), track.top() + HANDLE_H * 0.5),
        pos2(track.right(), track.bottom() - HANDLE_H * 0.5),
    );
    (track, travel)
}

/// Lesbare Textfarbe auf einem Hintergrund.
fn text_on(bg: Color32) -> Color32 {
    let l = 0.299 * f32::from(bg.r()) + 0.587 * f32::from(bg.g()) + 0.114 * f32::from(bg.b());
    if l > 150.0 {
        Color32::from_gray(20)
    } else {
        Color32::WHITE
    }
}

/// Element zeichnen. `value`: Fader/Drehregler 0–1; `hot`: gedrückt bzw.
/// unter dem Zeiger.
#[allow(clippy::too_many_arguments)]
fn paint_item(
    p: &egui::Painter,
    e: &Engine,
    st: &AppState,
    states: &HashMap<u64, bool>,
    item: &TouchItem,
    rect: Rect,
    value: f32,
    hot: bool,
    tex: Option<&egui::TextureHandle>,
) {
    let show = e.show();
    let base = item.color.map(theme::rgb);
    let label = display_label(show, item);
    let auto = if item.kind == TouchKind::Label {
        (rect.height() * 0.42).clamp(13.0, 48.0)
    } else {
        (rect.height().min(rect.width()) * 0.16).clamp(11.0, 22.0)
    };
    let size = if item.text_size > 0.0 {
        item.text_size
    } else {
        auto
    };
    let accent = base.unwrap_or(theme::accent());
    match item.kind {
        TouchKind::Label | TouchKind::Button => {
            let on = item.kind == TouchKind::Button
                && mapping_of(item).is_some_and(|m| crate::mapping::is_on(e, st, states, &m));
            let fill = match (item.kind, base) {
                (_, Some(b)) => b,
                (TouchKind::Button, None) => theme::raised(),
                _ => Color32::TRANSPARENT,
            };
            let fill = if hot { fill.gamma_multiply(0.7) } else { fill };
            if fill != Color32::TRANSPARENT {
                p.rect_filled(rect, theme::R_M, fill);
            }
            if let Some(tex) = tex {
                paint_cover(p, rect.shrink(1.0), tex, if hot { 0.7 } else { 1.0 });
            }
            if item.kind == TouchKind::Button {
                p.rect_stroke(
                    rect,
                    theme::R_M,
                    if on {
                        Stroke::new(3.0, Color32::WHITE)
                    } else {
                        Stroke::new(1.0, theme::border())
                    },
                    StrokeKind::Inside,
                );
                if on {
                    // leuchtender Streifen oben
                    let bar = Rect::from_min_size(
                        rect.min + vec2(8.0, 6.0),
                        vec2(rect.width() - 16.0, 3.0),
                    );
                    p.rect_filled(bar, 1.5, Color32::WHITE);
                }
            }
            let text_color = item.text_color.map(theme::rgb).unwrap_or_else(|| {
                if tex.is_some() {
                    Color32::WHITE
                } else if fill == Color32::TRANSPARENT {
                    theme::text()
                } else {
                    text_on(fill)
                }
            });
            // Symbol über der Beschriftung (allein: mittig und groß)
            let mut text_center = rect.center();
            if let Some(icon) = &item.icon {
                let icon_size = if label.is_empty() {
                    (rect.height().min(rect.width()) * 0.5).max(14.0)
                } else {
                    (rect.height().min(rect.width()) * 0.32).max(12.0)
                };
                let c = if label.is_empty() {
                    rect.center()
                } else {
                    rect.center() - vec2(0.0, icon_size * 0.45)
                };
                icon_library::paint(p, c, icon, icon_size, text_color);
                text_center = rect.center() + vec2(0.0, icon_size * 0.5);
            }
            if !label.is_empty() {
                let galley = p.layout(
                    label,
                    egui::FontId::proportional(size),
                    text_color,
                    rect.width() - 12.0,
                );
                let pos = text_center - galley.size() * 0.5;
                if tex.is_some() && item.text_color.is_none() {
                    // Schatten für Lesbarkeit auf Bildern
                    p.galley(
                        pos + vec2(1.0, 1.0),
                        galley.clone(),
                        Color32::from_black_alpha(200),
                    );
                }
                p.galley(pos, galley, text_color);
            }
            if item.kind == TouchKind::Button && item.control.is_none() {
                p.circle_filled(rect.right_top() + vec2(-9.0, 9.0), 3.5, theme::DANGER);
            }
        }
        TouchKind::Fader => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let (track, travel) = fader_parts(rect);
            let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
            p.text(
                pos2(rect.center().x, rect.top() + 15.0),
                Align2::CENTER_CENTER,
                format!("{:.0}", value * 100.0),
                theme::heading_font(),
                text_color,
            );
            // Schlitz mit Pegel in der Elementfarbe
            let slot = Rect::from_center_size(
                pos2(track.center().x, track.center().y),
                vec2(6.0, track.height() - 6.0),
            );
            p.rect_filled(slot, 3.0, theme::bg());
            let hy = travel.bottom() - travel.height() * value;
            let level = Rect::from_min_max(pos2(slot.left(), hy), slot.max);
            p.rect_filled(level, 3.0, accent);
            // Griff wie im Programmer
            let handle = Rect::from_center_size(
                pos2(track.center().x, hy),
                vec2((rect.width() - 16.0).min(40.0), 34.0),
            );
            p.rect_filled(handle, theme::R_S, theme::raised_hover());
            p.rect_stroke(
                handle,
                theme::R_S,
                Stroke::new(
                    1.5,
                    if hot {
                        theme::text()
                    } else {
                        theme::text_dim()
                    },
                ),
                StrokeKind::Inside,
            );
            p.line_segment(
                [
                    pos2(handle.left() + 7.0, handle.center().y),
                    pos2(handle.right() - 7.0, handle.center().y),
                ],
                Stroke::new(2.5, theme::text()),
            );
            if !label.is_empty() {
                let g = p.layout(
                    label,
                    egui::FontId::proportional(size.min(14.0)),
                    text_color,
                    rect.width() - 6.0,
                );
                p.galley(
                    pos2(
                        rect.center().x - g.size().x * 0.5,
                        rect.bottom() - 6.0 - g.size().y,
                    ),
                    g,
                    text_color,
                );
            }
        }
        TouchKind::Knob => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
            let has_label = !label.is_empty();
            let avail_h = rect.height() - if has_label { 22.0 } else { 8.0 };
            let r = (avail_h.min(rect.width() - 12.0) * 0.5).max(12.0);
            let c = pos2(rect.center().x, rect.top() + 4.0 + avail_h * 0.5);
            let start = std::f32::consts::PI * 0.75;
            let sweep = std::f32::consts::PI * 1.5;
            let arc = |from: f32, to: f32, radius: f32| -> Vec<Pos2> {
                (0..=40)
                    .map(|i| c + egui::Vec2::angled(from + (to - from) * i as f32 / 40.0) * radius)
                    .collect()
            };
            let ring = (r * 0.14).clamp(3.0, 7.0);
            p.add(Shape::line(
                arc(start, start + sweep, r - ring),
                Stroke::new(ring, theme::bg()),
            ));
            if value > 0.001 {
                p.add(Shape::line(
                    arc(start, start + sweep * value, r - ring),
                    Stroke::new(ring, accent),
                ));
            }
            let body = r - ring * 2.2;
            p.circle_filled(
                c,
                body,
                if hot {
                    theme::raised_hover()
                } else {
                    theme::raised()
                },
            );
            p.circle_stroke(c, body, Stroke::new(1.0, theme::border()));
            let a = start + sweep * value;
            p.line_segment(
                [
                    c + egui::Vec2::angled(a) * (body * 0.35),
                    c + egui::Vec2::angled(a) * (body * 0.85),
                ],
                Stroke::new(2.5, theme::text()),
            );
            p.text(
                c + vec2(0.0, body * 0.35 + 4.0),
                Align2::CENTER_TOP,
                format!("{:.0}", value * 100.0),
                theme::caption(),
                theme::text_dim(),
            );
            if has_label {
                p.text(
                    pos2(rect.center().x, rect.bottom() - 11.0),
                    Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(size.min(14.0)),
                    text_color,
                );
            }
        }
        TouchKind::Color => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(10.0, 15.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    item.text_color.map(theme::rgb).unwrap_or(theme::text()),
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
            if let Some(rgb) = item.fixtures.iter().find_map(|f| current_color(e, *f)) {
                let hsv = egui::ecolor::Hsva::from_srgb(rgb);
                let m = pos2(
                    field.left() + hsv.h * field.width(),
                    field.top() + (1.0 - hsv.s) * field.height(),
                );
                p.circle_stroke(m, 8.0, Stroke::new(2.0, Color32::BLACK));
                p.circle_stroke(m, 6.0, Stroke::new(2.0, Color32::WHITE));
            }
        }
        TouchKind::Xy => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(10.0, 15.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    item.text_color.map(theme::rgb).unwrap_or(theme::text()),
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
                p.line_segment(
                    [pos2(m.x, field.top()), pos2(m.x, field.bottom())],
                    Stroke::new(1.0, accent.gamma_multiply(0.5)),
                );
                p.line_segment(
                    [pos2(field.left(), m.y), pos2(field.right(), m.y)],
                    Stroke::new(1.0, accent.gamma_multiply(0.5)),
                );
                p.circle_filled(m, 8.0, accent);
                p.circle_stroke(m, 8.0, Stroke::new(2.0, Color32::WHITE));
            }
        }
    }
}

/// Kanäle einer Art eines Geräts.
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
        (None, Some(_)) if !pick.channels.is_empty() => {
            crate::mapping::channel_name(show, pick.channels[0])
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
                touch.add_page(crate::tr!("Seite {n}", n = n))
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
    let name = e
        .show()
        .touch
        .page(page)
        .map_or_else(String::new, |p| p.name.clone());
    if let Some(id) = e.show_mut().touch.add_item(page, item) {
        st.touch_sel = Some(id);
        st.info(crate::tr!(
            "Zur Touch-Seite „{page}“ hinzugefügt",
            page = name
        ));
    }
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
