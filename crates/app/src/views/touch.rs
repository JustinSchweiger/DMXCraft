//! TOUCH: freely arranged controls on pages. In edit mode elements can be
//! created, moved, resized, labeled, colored and given an icon or image;
//! otherwise they are operated.
//!
//! Positions and sizes are stored in units of [`UNIT`] pixels (at 100 %).
//! The grid (step size, snapping, display) is adjustable.
//!
//! Elements are added via Alt + click on a control (scene, bank, fader,
//! fixture …), via the context menu or the “Add” bar.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use dmxcraft_core::mapping::{ButtonMode, Control, Mapping};
use dmxcraft_core::touch::{ButtonStyle, TouchItem, TouchKind, LONG_PRESS_MS};
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
use crate::views::control::tracking::RoomMap;
use crate::views::{View, ViewCtx};

/// Pixels of one unit at 100 %.
pub const UNIT: f32 = 72.0;
/// Spacing between elements.
const GAP: f32 = 6.0;
/// Margin around the page (so nothing is cut off at the top/left).
const PAD: f32 = 10.0;
/// Width of the edit bar on the right.
const PANEL_W: f32 = 300.0;
/// LIVE TRACKING: how close (pixels) a touch must be to grab a target.
const TARGET_GRAB: f32 = 28.0;
/// Grid steps (pixels at 100 %).
const GRID_STEPS: [f32; 6] = [12.0, 18.0, 24.0, 36.0, 48.0, 72.0];

/// Colors for surfaces and text.
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

/// Dragging in edit mode.
#[derive(Clone, Copy)]
enum Drag {
    Move { id: u32, grab: egui::Vec2 },
    Resize { id: u32 },
}

/// What an image is loaded for.
#[derive(Clone, Copy)]
enum ImageTarget {
    Page(u32),
    Item(u32, u32),
}

pub struct TouchView {
    /// View size (1.0 = 100 %).
    zoom: f32,
    drag: Option<Drag>,
    /// Selection rectangle: start point.
    marquee: Option<Pos2>,
    rename_page: Option<(u32, String)>,
    rename_focus: bool,
    /// Pressed buttons (for releasing).
    held: Vec<u32>,
    textures: HashMap<u32, egui::TextureHandle>,
    image_dialog: Option<(ImageTarget, Receiver<Option<PathBuf>>)>,
    /// LIVE TRACKING: the target being dragged (element, tracker).
    track_drag: Option<(u32, u32)>,
    /// Page tab being dragged to a new position.
    page_drag: Option<u32>,
    /// Long-press buttons being held: element → start time (s).
    holding: HashMap<u32, f64>,
    /// Element type being dragged from the edit bar onto the page.
    add_drag: Option<TouchKind>,
    /// Last drawn grid: visible area, origin, pixels per unit, snap step
    /// (units).
    grid_geo: Option<(Rect, Pos2, f32, Option<f32>)>,
}

impl Default for TouchView {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            drag: None,
            marquee: None,
            rename_page: None,
            rename_focus: false,
            held: Vec::new(),
            textures: HashMap::new(),
            image_dialog: None,
            track_drag: None,
            page_drag: None,
            holding: HashMap::new(),
            add_drag: None,
            grid_geo: None,
        }
    }
}

/// Select only this element.
fn select_only(st: &mut AppState, id: u32) {
    st.touch_sel = Some(id);
    st.touch_multi = vec![id];
}

/// Add the element to the selection or remove it (Shift/Ctrl + click).
fn toggle_selected(st: &mut AppState, id: u32) {
    if let Some(pos) = st.touch_multi.iter().position(|i| *i == id) {
        st.touch_multi.remove(pos);
    } else {
        st.touch_multi.push(id);
    }
    st.touch_sel = if st.touch_multi.contains(&id) {
        Some(id)
    } else {
        st.touch_multi.first().copied()
    };
}

/// Mapping an element triggers with (like a key).
pub(crate) fn mapping_of(item: &TouchItem) -> Option<Mapping> {
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

        // ----- Header: pages, zoom, edit -----
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            self.page_tabs(ui, cx, &pages, editing);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if !cx.state.locked {
                    let r = ui.add(egui::Button::new(t("Edit")).selected(cx.state.touch_edit));
                    if r.clicked() {
                        cx.state.touch_edit = !cx.state.touch_edit;
                        cx.state.touch_sel = None;
                    }
                }
                ui.add_space(theme::SP_S);
                if ui
                    .add(IconButton::new(Icon::ZoomIn).flat())
                    .on_hover_text(t("Zoom in"))
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
                    .on_hover_text(t("Reset"));
                if r.clicked() {
                    self.zoom = 1.0;
                }
                if ui
                    .add(IconButton::new(Icon::ZoomOut).flat())
                    .on_hover_text(t("Zoom out"))
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
        self.drop_new(ui, cx, editing);
    }
}

impl TouchView {
    /// Element dragged from the edit bar: preview on the page, created where
    /// it is released.
    fn drop_new(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>, editing: bool) {
        let Some(kind) = self.add_drag else {
            return;
        };
        let (pos, released) = ui.input(|i| (i.pointer.hover_pos(), !i.pointer.primary_down()));
        let place = match (self.grid_geo, pos, cx.state.touch_page) {
            (Some((vis, origin, u, snap)), Some(pos), Some(page)) if vis.contains(pos) => {
                let (w, h) = kind.default_size();
                // pointer in the middle of the new element
                let mut x = (pos.x - origin.x) / u - w / 2.0;
                let mut y = (pos.y - origin.y) / u - h / 2.0;
                if let Some(s) = snap {
                    x = (x / s).round() * s;
                    y = (y / s).round() * s;
                }
                let (x, y) = (x.max(0.0), y.max(0.0));
                let rect = Rect::from_min_size(origin + vec2(x * u, y * u), vec2(w * u, h * u));
                Some((page, x, y, rect))
            }
            _ => None,
        };
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        let layer = egui::LayerId::new(egui::Order::Tooltip, egui::Id::unique("touch_add_drag"));
        let p = ui.ctx().layer_painter(layer);
        match place {
            Some((_, _, _, rect)) => {
                p.rect_filled(rect, theme::R_M, theme::accent().gamma_multiply(0.25));
                p.rect_stroke(
                    rect,
                    theme::R_M,
                    Stroke::new(1.5, theme::accent()),
                    StrokeKind::Inside,
                );
                p.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    kind_name(kind),
                    theme::body(),
                    theme::text(),
                );
            }
            None => {
                if let Some(pos) = pos {
                    p.text(
                        pos + vec2(12.0, 12.0),
                        Align2::LEFT_TOP,
                        kind_name(kind),
                        theme::body(),
                        theme::text(),
                    );
                }
            }
        }
        if !released {
            return;
        }
        self.add_drag = None;
        if let (true, Some((page, x, y, _))) = (editing, place) {
            let mut item = TouchItem::new(kind);
            if kind == TouchKind::Label {
                item.label = t("Label").to_string();
            }
            if kind.uses_fixtures() {
                item.fixtures = cx.state.selection_order.clone();
            }
            let id = cx.engine.show_mut().touch.add_item_at(page, item, x, y);
            cx.state.touch_sel = id;
            cx.state.touch_multi = id.into_iter().collect();
        }
    }

    fn page_tabs(
        &mut self,
        ui: &mut Ui,
        cx: &mut ViewCtx<'_>,
        pages: &[(u32, String)],
        editing: bool,
    ) {
        let mut delete = None;
        let mut moved: Option<(u32, usize)> = None;
        let mut tabs: Vec<(u32, Rect)> = Vec::new();
        for (index, (id, name)) in pages.iter().enumerate() {
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
            tabs.push((*id, r.rect));
            if editing {
                // drag a tab to reorder the pages
                let d = r.interact(Sense::drag());
                if d.drag_started() {
                    self.page_drag = Some(*id);
                }
                if self.page_drag == Some(*id) {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                    ui.painter().rect_stroke(
                        r.rect,
                        theme::R_S,
                        Stroke::new(1.5, theme::accent()),
                        StrokeKind::Outside,
                    );
                }
            }
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
                    if menu::item(ui, MenuItem::new(t("Rename")).icon(Icon::Edit)) {
                        self.rename_page = Some((*id, name.clone()));
                        self.rename_focus = true;
                    }
                    if index > 0 && menu::item(ui, MenuItem::new(t("Move left"))) {
                        moved = Some((*id, index - 1));
                    }
                    if index + 1 < pages.len() && menu::item(ui, MenuItem::new(t("Move right"))) {
                        moved = Some((*id, index + 1));
                    }
                    if menu::item(ui, MenuItem::new(t("Delete page")).icon(Icon::Trash)) {
                        delete = Some(*id);
                    }
                });
            }
        }
        if let Some(dragged) = self.page_drag {
            let (pos, down) = ui.input(|i| (i.pointer.interact_pos(), i.pointer.primary_down()));
            if !down || !editing {
                self.page_drag = None;
            } else if let Some(pos) = pos {
                // new place: behind every other tab whose middle lies left of
                // the pointer (stable with tabs of different widths)
                let to = tabs
                    .iter()
                    .filter(|(id, r)| *id != dragged && r.center().x < pos.x)
                    .count();
                if tabs.iter().position(|(id, _)| *id == dragged) != Some(to) {
                    moved = Some((dragged, to));
                }
            }
        }
        if let Some((id, to)) = moved {
            cx.engine.show_mut().touch.move_page(id, to);
        }
        if editing
            && ui
                .add(IconButton::new(Icon::Plus).flat())
                .on_hover_text(t("Add page"))
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

    /// No page yet.
    fn empty(&mut self, ui: &mut Ui, rect: Rect, cx: &mut ViewCtx<'_>) {
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Center)),
        );
        child.add_space(rect.height() * 0.35);
        child.label(
            egui::RichText::new(t("No touch surface yet"))
                .color(theme::text_dim())
                .size(theme::FS_TITLE),
        );
        child.add_space(theme::SP_M);
        if !cx.state.locked && child.button(t("Add page")).clicked() {
            add_page(cx);
            cx.state.touch_edit = true;
        }
    }

    /// Pixels per unit (with zoom).
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

    /// Grid with the elements of a page.
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
        // Area: visible region or up to beyond the last element
        let right = page.items.iter().map(|i| i.x + i.w).fold(0.0, f32::max) + 2.0;
        let bottom = page.items.iter().map(|i| i.y + i.h).fold(0.0, f32::max) + 2.0;
        let avail = ui.available_size();
        let size = vec2(
            (right * u + 2.0 * PAD).max(avail.x),
            (bottom * u + 2.0 * PAD).max(avail.y),
        );
        let (area, bg) = ui.allocate_exact_size(
            size,
            if editing {
                Sense::click_and_drag()
            } else {
                Sense::click()
            },
        );
        let origin = area.min + vec2(PAD, PAD);
        self.grid_geo = Some((
            ui.clip_rect().intersect(area),
            origin,
            u,
            snap.then_some(step_px / UNIT),
        ));
        // Clean up the selection: only elements of this page, main selection
        // included
        {
            let st = &mut *cx.state;
            st.touch_multi
                .retain(|id| page.items.iter().any(|i| i.id == *id));
            match st.touch_sel {
                Some(s) if !st.touch_multi.contains(&s) => st.touch_multi = vec![s],
                None => st.touch_multi.clear(),
                _ => {}
            }
        }
        let mods = ui.input(|i| i.modifiers);
        let additive = mods.shift || mods.command;
        let p = ui.painter().clone();

        // Background of the page
        if let Some(c) = page.background_color {
            p.rect_filled(area, 0.0, theme::rgb(c));
        }
        if let Some(tex) = page
            .background
            .and_then(|id| self.texture(ui.ctx(), cx.engine.show(), id))
        {
            paint_cover(&p, ui.clip_rect().intersect(area), &tex, 1.0);
        }
        // Grid
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
                // fine grid: dots at the intersections
                for y in first_y..first_y + ny {
                    for x in first_x..first_x + nx {
                        let c = origin + vec2(x as f32 * step, y as f32 * step);
                        p.circle_filled(c, 1.2, theme::border().gamma_multiply(alpha));
                    }
                }
            }
        }
        if editing && bg.clicked() && !additive {
            cx.state.touch_sel = None;
            cx.state.touch_multi.clear();
        }
        if editing && bg.drag_started() {
            self.marquee = bg.interact_pointer_pos();
        }

        let item_rect = |i: &TouchItem| {
            Rect::from_min_size(
                origin + vec2(i.x * u, i.y * u),
                vec2((i.w * u - GAP).max(8.0), (i.h * u - GAP).max(8.0)),
            )
        };
        let states: HashMap<u64, bool> = crate::mapping::ui_states(ui.ctx());
        let mut changes: Vec<TouchItem> = Vec::new();
        let mut remove: Vec<u32> = Vec::new();
        let mut duplicate: Vec<u32> = Vec::new();
        let multi = cx.state.touch_multi.clone();
        for item in &page.items {
            let rect = item_rect(item);
            let tex = item
                .image
                .and_then(|id| self.texture(ui.ctx(), cx.engine.show(), id));
            let id = ui.make_persistent_id(("touch_item", item.id));
            let value = item_value(cx.engine, cx.state, item);
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
                let sel = multi.contains(&item.id);
                if sel || resp.hovered() {
                    p.rect_stroke(
                        rect.expand(2.0),
                        theme::R_M,
                        Stroke::new(if sel { 2.0 } else { 1.0 }, theme::accent()),
                        StrokeKind::Outside,
                    );
                }
                // Handle for resizing
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
                    select_only(cx.state, item.id);
                } else if resp.drag_started() {
                    let grab = resp
                        .interact_pointer_pos()
                        .map_or(vec2(0.0, 0.0), |p| p - rect.min);
                    self.drag = Some(Drag::Move { id: item.id, grab });
                    // in a multi-selection: move all together
                    if !sel {
                        select_only(cx.state, item.id);
                    }
                }
                if resp.clicked() {
                    if additive {
                        toggle_selected(cx.state, item.id);
                    } else {
                        select_only(cx.state, item.id);
                    }
                }
                if resp.secondary_clicked() && !sel {
                    select_only(cx.state, item.id);
                }
                // The context menu acts on the whole selection if the element belongs to it
                let group: Vec<u32> = if sel && multi.len() > 1 {
                    multi.clone()
                } else {
                    vec![item.id]
                };
                menu::context(&resp, |ui| {
                    menu::begin(ui);
                    if group.len() > 1 {
                        menu::title(ui, &tr!("{n} items", n = group.len()));
                    } else {
                        menu::title(ui, &item_title(item));
                        target_entry(ui, cx.engine.show(), item);
                    }
                    menu::separator(ui);
                    if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
                        duplicate = group.clone();
                    }
                    if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash)) {
                        remove = group.clone();
                    }
                    menu::separator(ui);
                    let mut c = item.color;
                    if swatch_row(ui, &mut c, true) {
                        for i in page.items.iter().filter(|i| group.contains(&i.id)) {
                            let mut i = i.clone();
                            i.color = c;
                            changes.push(i);
                        }
                        ui.close();
                    }
                });
            } else {
                self.play_item(ui, &p, cx, &states, item, rect, id, tex.as_ref());
            }
        }

        // Apply the drag
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
            let preview: Option<Vec<TouchItem>> = match (d, pointer) {
                (Drag::Move { id, grab }, Some(pos)) => {
                    page.items.iter().find(|i| i.id == id).map(|it| {
                        let tl = (pos - grab - origin) / u;
                        let group: Vec<&TouchItem> = page
                            .items
                            .iter()
                            .filter(|i| i.id == id || cx.state.touch_multi.contains(&i.id))
                            .collect();
                        // Offset of the dragged element, the group stays within the area
                        let min_x = group.iter().map(|i| i.x).fold(f32::MAX, f32::min);
                        let min_y = group.iter().map(|i| i.y).fold(f32::MAX, f32::min);
                        let dx = (snap_to(tl.x) - it.x).max(-min_x);
                        let dy = (snap_to(tl.y) - it.y).max(-min_y);
                        group
                            .into_iter()
                            .map(|i| {
                                let mut n = i.clone();
                                n.x = (n.x + dx).max(0.0);
                                n.y = (n.y + dy).max(0.0);
                                n
                            })
                            .collect()
                    })
                }
                (Drag::Resize { id }, Some(pos)) => {
                    page.items.iter().find(|i| i.id == id).map(|it| {
                        let rel = (pos - origin) / u;
                        let min = if snap { step_px / UNIT } else { 0.25 };
                        let mut n = it.clone();
                        n.w = snap_to(rel.x - it.x).max(min);
                        n.h = snap_to(rel.y - it.y).max(min);
                        vec![n]
                    })
                }
                _ => None,
            };
            match preview {
                Some(ns) => {
                    for n in &ns {
                        p.rect_stroke(
                            item_rect(n),
                            theme::R_M,
                            Stroke::new(2.0, theme::accent()),
                            StrokeKind::Outside,
                        );
                    }
                    if released {
                        changes.extend(ns);
                        self.drag = None;
                    }
                }
                None => self.drag = None,
            }
        }

        // Selection rectangle
        if let Some(start) = self.marquee {
            let pointer = ui.input(|i| i.pointer.interact_pos());
            let released = ui.input(|i| i.pointer.any_released());
            if !editing || pointer.is_none() {
                self.marquee = None;
            } else if let Some(pos) = pointer {
                let r = Rect::from_two_pos(start, pos);
                p.rect_filled(r, 0.0, theme::accent().gamma_multiply(0.12));
                p.rect_stroke(
                    r,
                    0.0,
                    Stroke::new(1.0, theme::accent()),
                    StrokeKind::Inside,
                );
                let hits: Vec<u32> = page
                    .items
                    .iter()
                    .filter(|i| item_rect(i).intersects(r))
                    .map(|i| i.id)
                    .collect();
                for i in page.items.iter().filter(|i| hits.contains(&i.id)) {
                    p.rect_stroke(
                        item_rect(i).expand(2.0),
                        theme::R_M,
                        Stroke::new(1.0, theme::accent()),
                        StrokeKind::Outside,
                    );
                }
                if released {
                    self.marquee = None;
                    let st = &mut *cx.state;
                    if !additive {
                        st.touch_multi.clear();
                    }
                    for id in hits {
                        if !st.touch_multi.contains(&id) {
                            st.touch_multi.push(id);
                        }
                    }
                    st.touch_sel = st.touch_multi.first().copied();
                }
            }
        }

        // Delete deletes, arrow keys move the selected element
        if editing && !ui.ctx().egui_wants_keyboard_input() {
            if remove.is_empty()
                && ui.input(|i| {
                    i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace)
                })
            {
                remove = cx.state.touch_multi.clone();
            }
            // Ctrl+A: select everything on the page
            if ui.input(|i| i.modifiers.command && i.key_pressed(egui::Key::A)) {
                cx.state.touch_multi = page.items.iter().map(|i| i.id).collect();
                cx.state.touch_sel = cx.state.touch_multi.first().copied();
            }
            let sel_items: Vec<&TouchItem> = page
                .items
                .iter()
                .filter(|i| cx.state.touch_multi.contains(&i.id))
                .collect();
            if !sel_items.is_empty() {
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
                    let min_x = sel_items.iter().map(|i| i.x).fold(f32::MAX, f32::min);
                    let min_y = sel_items.iter().map(|i| i.y).fold(f32::MAX, f32::min);
                    let (dx, dy) = (dx.max(-min_x), dy.max(-min_y));
                    for i in sel_items {
                        let mut n = i.clone();
                        n.x += dx;
                        n.y += dy;
                        changes.push(n);
                    }
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
            if !remove.is_empty() {
                pg.items.retain(|i| !remove.contains(&i.id));
                cx.state.touch_multi.retain(|i| !remove.contains(i));
                cx.state.touch_sel = cx.state.touch_multi.first().copied();
            }
        }
        if !remove.is_empty() {
            touch.prune_images();
        }
        if !duplicate.is_empty() {
            // Copies become the new selection
            let copies: Vec<u32> = duplicate
                .iter()
                .filter_map(|id| touch.duplicate(page_id, *id))
                .collect();
            cx.state.touch_sel = copies.first().copied();
            cx.state.touch_multi = copies;
        }
    }

    /// Operate an element.
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
        let value = item_value(cx.engine, cx.state, item);
        match item.kind {
            TouchKind::Label => {
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, false, tex);
            }
            TouchKind::Button => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let pressed = resp.is_pointer_button_down_on();
                let was = self.held.contains(&item.id);
                // long press: only counts once held long enough
                let mut hold = None;
                let down = if item.long_press {
                    let now = ui.input(|i| i.time);
                    if pressed {
                        let start = *self.holding.entry(item.id).or_insert(now);
                        let k = ((now - start) * 1000.0 / LONG_PRESS_MS as f64) as f32;
                        if k < 1.0 {
                            hold = Some(k);
                            ui.ctx().request_repaint();
                        }
                        k >= 1.0 || was
                    } else {
                        self.holding.remove(&item.id);
                        false
                    }
                } else {
                    pressed
                };
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
                    p, cx.engine, cx.state, states, item, rect, 0.0, pressed, tex,
                );
                if let Some(k) = hold {
                    // progress of the long press along the bottom edge
                    let bar = Rect::from_min_size(
                        pos2(rect.left() + 6.0, rect.bottom() - 7.0),
                        vec2((rect.width() - 12.0) * k, 3.0),
                    );
                    p.rect_filled(bar, 1.5, Color32::WHITE);
                }
            }
            TouchKind::Fader | TouchKind::Knob => {
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let mut v = value;
                if item.kind == TouchKind::Fader {
                    if let Some(pos) = resp
                        .interact_pointer_pos()
                        .filter(|_| resp.dragged() || resp.clicked())
                    {
                        let travel = fader_parts(rect, item.horizontal).1;
                        v = if item.horizontal {
                            ((pos.x - travel.left()) / travel.width()).clamp(0.0, 1.0)
                        } else {
                            (1.0 - (pos.y - travel.top()) / travel.height()).clamp(0.0, 1.0)
                        };
                    }
                } else if resp.dragged() {
                    v = (value - resp.drag_delta().y / 160.0).clamp(0.0, 1.0);
                }
                if resp.double_clicked() {
                    v = if value > 0.0 { 0.0 } else { 1.0 };
                }
                if (v - value).abs() > f32::EPSILON {
                    cx.state.touch_values.insert(item.id, v);
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
            TouchKind::Tracking => {
                // drag a target to move it, tap it to switch following
                let resp = ui.interact(rect, id, Sense::click_and_drag());
                let map = RoomMap::new(cx.engine.show(), content_rect(rect, item));
                if resp.drag_started() {
                    let origin = ui.input(|i| i.pointer.press_origin());
                    self.track_drag = origin
                        .and_then(|o| map.target_at(cx.engine.show(), o, TARGET_GRAB))
                        .map(|tr| (item.id, tr));
                }
                let dragging = self.track_drag.filter(|(i, _)| *i == item.id);
                if let (Some((_, tr)), Some(pos)) = (dragging, resp.interact_pointer_pos()) {
                    if resp.dragged() {
                        let [x, z] = map.to_room(pos);
                        if let Some(t) = cx.engine.show_mut().tracker_mut(tr) {
                            t.target[0] = x;
                            t.target[2] = z;
                        }
                    }
                }
                if resp.drag_stopped() && dragging.is_some() {
                    self.track_drag = None;
                }
                if resp.clicked() {
                    let hit = resp
                        .interact_pointer_pos()
                        .and_then(|pos| map.target_at(cx.engine.show(), pos, TARGET_GRAB));
                    if let Some(t) = hit.and_then(|tr| cx.engine.show_mut().tracker_mut(tr)) {
                        t.active = !t.active;
                    }
                }
                paint_item(p, cx.engine, cx.state, states, item, rect, 0.0, false, tex);
            }
        }
    }

    /// Edit bar: page, grid, new elements, properties.
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
                // ----- Add -----
                section_header(ui, t("Add"), |_| {});
                let kinds = [
                    (TouchKind::Button, "square", t("Button")),
                    (TouchKind::Fader, "sliders", t("Fader")),
                    (TouchKind::Knob, "circle", t("Knob")),
                    (TouchKind::Label, "x", t("Label")),
                    (TouchKind::Color, "palette", t("Color wheel")),
                    (TouchKind::Xy, "plus", t("XY pad")),
                    (TouchKind::Tracking, "person", t("Live tracking")),
                ];
                let w = (ui.available_width() - 2.0 * theme::SP_S) / 3.0;
                for row in kinds.chunks(3) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = theme::SP_S;
                        for (k, icon, label) in row {
                            let tile = add_tile(ui, w, *k, icon, label);
                            if tile.drag_started() {
                                self.add_drag = Some(*k);
                            }
                            if tile.clicked() {
                                let mut item = TouchItem::new(*k);
                                if *k == TouchKind::Label {
                                    item.label = t("Label").to_string();
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

                // ----- Multi-selection -----
                if cx.state.touch_multi.len() > 1 {
                    ui.add_space(theme::SP_S);
                    self.multi_panel(ui, cx, page);
                    return;
                }

                // ----- Properties of the selected element -----
                if let Some(sel) = cx.state.touch_sel {
                    ui.add_space(theme::SP_S);
                    self.properties(ui, cx, page, sel);
                    return;
                }

                // ----- Page -----
                ui.add_space(theme::SP_S);
                section_header(ui, t("Side"), |_| {});
                let (mut bg, mut img) = cx
                    .engine
                    .show()
                    .touch
                    .page(page)
                    .map_or((None, None), |p| (p.background_color, p.background));
                form(ui, "touch_page", |f| {
                    f.row(t("Background"), |ui| {
                        swatch_row(ui, &mut bg, true);
                    });
                    f.row(t("Image"), |ui| {
                        if ui.button(t("Load …")).clicked() {
                            self.image_dialog =
                                Some((ImageTarget::Page(page), pick_image(ui.ctx())));
                        }
                        if img.is_some() && ui.button(t("Remove")).clicked() {
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

                // ----- Grid -----
                ui.add_space(theme::SP_M);
                section_header(ui, t("Grid"), |_| {});
                let tl = cx.engine.show().touch.clone();
                let (mut show_grid, mut snap, mut cell) = (tl.show_grid, tl.snap, tl.cell);
                form(ui, "touch_grid", |f| {
                    f.row(t("Visible"), |ui| {
                        ui.add(Toggle::new(&mut show_grid, ""));
                    });
                    f.row(t("Snap"), |ui| {
                        ui.add(Toggle::new(&mut snap, ""));
                    });
                    f.row(t("Step size"), |ui| {
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

    /// Properties of the selected element.
    /// Several elements selected: count, color for all, duplicate, delete.
    fn multi_panel(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>, page: u32) {
        let ids = cx.state.touch_multi.clone();
        section_header(ui, &tr!("{n} items selected", n = ids.len()), |_| {});
        ui.label(
            egui::RichText::new(t(
                "Drag moves all of them, so do the arrow keys; Delete removes them. Shift/Ctrl + click adds or removes.",
            ))
            .small()
            .color(theme::text_dim()),
        );
        ui.add_space(theme::SP_S);
        let first_color = cx
            .engine
            .show()
            .touch
            .page(page)
            .and_then(|p| p.items.iter().find(|i| ids.contains(&i.id)))
            .and_then(|i| i.color);
        let mut c = first_color;
        form(ui, "touch_multi", |f| {
            f.row(t("Color"), |ui| {
                if swatch_row(ui, &mut c, true) {
                    if let Some(p) = cx.engine.show_mut().touch.page_mut(page) {
                        for i in p.items.iter_mut().filter(|i| ids.contains(&i.id)) {
                            i.color = c;
                        }
                    }
                }
            });
        });
        ui.add_space(theme::SP_S);
        ui.horizontal(|ui| {
            if ui.button(t("Duplicate")).clicked() {
                let touch = &mut cx.engine.show_mut().touch;
                let copies: Vec<u32> = ids
                    .iter()
                    .filter_map(|id| touch.duplicate(page, *id))
                    .collect();
                cx.state.touch_sel = copies.first().copied();
                cx.state.touch_multi = copies;
            }
            if ui.button(t("Delete")).clicked() {
                let touch = &mut cx.engine.show_mut().touch;
                if let Some(p) = touch.page_mut(page) {
                    p.items.retain(|i| !ids.contains(&i.id));
                }
                touch.prune_images();
                cx.state.touch_sel = None;
                cx.state.touch_multi.clear();
            }
        });
    }

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
        // older shows: automatic name and separate icon become the label text
        if item.auto_label || item.icon.is_some() {
            item.label = display_label(cx.engine.show(), &item);
            item.icon = None;
            item.auto_label = false;
        }
        let mut remove = false;
        let mut duplicate = false;
        section_header(ui, kind_name(item.kind), |ui| {
            if ui
                .add(IconButton::new(Icon::Trash).flat().danger())
                .on_hover_text(t("Delete"))
                .clicked()
            {
                remove = true;
            }
            if ui
                .add(IconButton::new(Icon::Copy).flat())
                .on_hover_text(t("Duplicate"))
                .clicked()
            {
                duplicate = true;
            }
            if ui
                .add(IconButton::new(Icon::Close).flat())
                .on_hover_text(t("Clear selection"))
                .clicked()
            {
                cx.state.touch_sel = None;
            }
        });
        let show = cx.engine.show();
        if !matches!(item.kind, TouchKind::Label | TouchKind::Tracking) {
            let text = if item.kind.uses_fixtures() {
                match item.fixtures.len() {
                    0 => t("no devices").to_string(),
                    1 => show
                        .fixture(item.fixtures[0])
                        .map_or_else(String::new, |f| f.name.clone()),
                    n => crate::tr!("{n} fixtures", n = n),
                }
            } else {
                item_target(show, &item)
            };
            if matches!(
                item.kind,
                TouchKind::Button | TouchKind::Fader | TouchKind::Knob
            ) {
                if let Some((control, name)) = target_menu(ui, show, &item) {
                    // the label follows the target unless it was changed
                    let old = item.control.map(|_| short_name(&item_target(show, &item)));
                    if item.label.is_empty() || old.as_deref() == Some(item.label.as_str()) {
                        item.label = name;
                    }
                    if let Some(Control::Scene {
                        target: dmxcraft_core::mapping::SceneTarget::Scene { id },
                    }) = control
                    {
                        if let Some(sc) = show.scene(id) {
                            item.color = item.color.or(sc.color);
                        }
                    }
                    item.control = control;
                    item.channels.clear();
                }
            } else {
                ui.label(egui::RichText::new(text).color(theme::text_dim()));
            }
            ui.add_space(theme::SP_XS);
        }
        let mut pick_img = false;
        form(ui, "touch_props", |f| {
            f.row(t("Label"), |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut item.label)
                        .desired_rows(1)
                        .desired_width(ui.available_width())
                        .hint_text(t("Text – icons as :name:")),
                );
            });
            f.row(t("Text size"), |ui| {
                ui.add(
                    egui::DragValue::new(&mut item.text_size)
                        .range(0.0..=96.0)
                        .speed(0.5)
                        .custom_formatter(|v, _| {
                            if v <= 0.0 {
                                t("automatic").to_string()
                            } else {
                                format!("{v:.0}")
                            }
                        }),
                );
            });
            f.row(t("Text colour"), |ui| {
                swatch_row(ui, &mut item.text_color, true);
            });
            f.row(t("Color"), |ui| {
                swatch_row(ui, &mut item.color, true);
            });
            if matches!(item.kind, TouchKind::Button | TouchKind::Label) {
                f.row(t("Icon"), |ui| {
                    // inserts the icon into the text (move and scale it there)
                    let mut pick = None;
                    icon_picker(ui, ("touch_icon", sel), &mut pick);
                    if let Some(name) = pick {
                        if !item.label.is_empty() && !item.label.ends_with(char::is_whitespace) {
                            item.label.push(' ');
                        }
                        item.label.push_str(&format!(":{name}:"));
                    }
                });
                f.row(t("Image"), |ui| {
                    if ui.button(t("Load …")).clicked() {
                        pick_img = true;
                    }
                    if item.image.is_some() && ui.button(t("Remove")).clicked() {
                        item.image = None;
                    }
                });
            }
            f.row(t("Size"), |ui| {
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
                            egui::Button::new(t("Use selection")),
                        )
                        .clicked()
                    {
                        item.fixtures = cx.state.selection_order.clone();
                    }
                });
            }
            if item.kind == TouchKind::Button {
                f.row(t("Style"), |ui| {
                    egui::ComboBox::from_id_salt(("touch_style", sel))
                        .selected_text(style_name(item.style))
                        .show_ui(ui, |ui| {
                            for s in ButtonStyle::ALL {
                                ui.selectable_value(&mut item.style, s, style_name(s));
                            }
                        });
                });
                f.row(t("Behaviour"), |ui| {
                    egui::ComboBox::from_id_salt(("touch_behaviour", sel))
                        .selected_text(if item.flash {
                            t("While held")
                        } else {
                            t("Toggle")
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut item.flash, false, t("Toggle"));
                            ui.selectable_value(&mut item.flash, true, t("While held"));
                        });
                });
                f.row(t("Long press"), |ui| {
                    ui.add(Toggle::new(&mut item.long_press, ""))
                        .on_hover_text(t("Only triggers when held – against accidental touches"));
                });
            }
            if item.kind == TouchKind::Fader {
                let mut horizontal = item.horizontal;
                f.row(t("Horizontal"), |ui| {
                    ui.add(Toggle::new(&mut horizontal, ""));
                });
                if horizontal != item.horizontal {
                    // turn it on its side: width and height swap
                    item.horizontal = horizontal;
                    std::mem::swap(&mut item.w, &mut item.h);
                }
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

    /// Take over the result of the image dialog.
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

/// “Add element” tile: icon and name.
fn add_tile(ui: &mut Ui, w: f32, kind: TouchKind, icon: &str, label: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 64.0), Sense::click_and_drag());
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
    resp.on_hover_text(t("Click to add, or drag onto the page"))
}

/// Color swatches (optionally with “none”). Returns `true` if changed.
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
        // own color
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

/// Button with the chosen icon; click opens the icon library.
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
        if ui.button(t("No icon")).clicked() {
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

/// Menu entry with the element's target (for debugging).
fn target_entry(ui: &mut Ui, show: &Show, item: &TouchItem) {
    let text = if item.kind.uses_fixtures() {
        let names: Vec<String> = item
            .fixtures
            .iter()
            .filter_map(|f| show.fixture(*f).map(|f| f.name.clone()))
            .collect();
        if names.is_empty() {
            t("no devices").to_string()
        } else {
            names.join(", ")
        }
    } else if matches!(item.kind, TouchKind::Label | TouchKind::Tracking) {
        return;
    } else {
        item_target(show, item)
    };
    ui.add(MenuItem::new(&crate::tr!("Target: {target}", target = text)).enabled(false));
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
                MenuItem::new(&crate::tr!("… and {n} more", n = chans.len() - 8)).enabled(false),
            );
        }
    }
}

fn pick_image(ctx: &egui::Context) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Load image"))
            .add_filter(t("Images"), &["png", "jpg", "jpeg"])
            .pick_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

/// Read an image and bring it to a sensible size (max. 1600 px).
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

/// Draw an image filling the area (cropped).
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
    let id = touch.add_page(crate::tr!("Page {n}", n = n));
    cx.state.touch_page = Some(id);
}

fn style_name(s: ButtonStyle) -> &'static str {
    match s {
        ButtonStyle::Normal => t("Normal"),
        ButtonStyle::Outline => t("Outline"),
        ButtonStyle::Switch => t("Switch"),
        ButtonStyle::Led => t("Status light"),
        ButtonStyle::Round => t("Round"),
    }
}

fn kind_name(k: TouchKind) -> &'static str {
    match k {
        TouchKind::Label => t("Label"),
        TouchKind::Button => t("Button"),
        TouchKind::Fader => t("Fader"),
        TouchKind::Knob => t("Knob"),
        TouchKind::Color => t("Color wheel"),
        TouchKind::Xy => t("XY pad"),
        TouchKind::Tracking => t("Live tracking"),
    }
}

/// Target of an element as text.
fn item_target(show: &Show, item: &TouchItem) -> String {
    match (mapping_of(item), item.control) {
        (Some(m), Some(_)) => {
            let (cat, target) = crate::mapping::describe_mapping(show, &m);
            format!("{cat}: {target}")
        }
        _ => t("no target").to_string(),
    }
}

fn item_title(item: &TouchItem) -> String {
    if item.label.is_empty() {
        kind_name(item.kind).to_string()
    } else {
        item.label.clone()
    }
}

/// Label: own or from the target.
pub(crate) fn display_label(show: &Show, item: &TouchItem) -> String {
    // older shows: empty label = name of the target
    let text = if item.label.is_empty() && item.auto_label && item.control.is_some() {
        let full = item_target(show, item);
        full.split_once(": ")
            .map_or(full.clone(), |x| x.1.to_string())
    } else {
        item.label.clone()
    };
    // older shows: separate icon above the text
    match &item.icon {
        Some(icon) if text.is_empty() => format!(":{icon}:"),
        Some(icon) => format!(":{icon}:\n{text}"),
        None => text,
    }
}

/// Target chooser of a button, fader or knob: `Some((control, name))` when
/// one was picked (`control` `None` = remove the target).
fn target_menu(ui: &mut Ui, show: &Show, item: &TouchItem) -> Option<(Option<Control>, String)> {
    use dmxcraft_core::mapping::{ui_key, SceneTarget};
    let fader = matches!(item.kind, TouchKind::Fader | TouchKind::Knob);
    let current = if item.control.is_some() {
        short_name(&item_target(show, item))
    } else {
        t("Choose target …").to_string()
    };
    let resp = ui.add(egui::Button::new(current).min_size(vec2(ui.available_width(), 0.0)));
    let mut out = None;
    let ui_button = |key: String| Control::Ui {
        key: ui_key(&key),
        fader: false,
    };
    let ui_fader = |key: String| Control::Ui {
        key: ui_key(&key),
        fader: true,
    };
    menu::dropdown(&resp, |ui| {
        menu::begin(ui);
        let mut pick = |ui: &mut Ui, label: &str, c: Control, name: String| {
            let on = item.control == Some(c);
            if menu::item(ui, MenuItem::new(label).checked(on)) {
                out = Some((Some(c), name));
            }
        };
        if fader {
            pick(ui, t("Master"), Control::Master, t("Master").to_string());
            pick(
                ui,
                t("Sound master"),
                Control::AudioMaster,
                t("Sound master").to_string(),
            );
            menu::sub(ui, MenuItem::new(t("Sound")), |ui| {
                pick(
                    ui,
                    t("Volume"),
                    Control::SoundVolume { deck: None },
                    t("Volume").to_string(),
                );
                for d in 0..show.sound.decks.len() {
                    let name = crate::tr!("Player {d}", d = crate::views::sound::deck_name(d));
                    pick(
                        ui,
                        &name,
                        Control::SoundVolume {
                            deck: Some(d as u8),
                        },
                        name.clone(),
                    );
                }
            });
            if !show.trackers.is_empty() {
                menu::sub(ui, MenuItem::new(t("Live tracking")), |ui| {
                    for tr in &show.trackers {
                        menu::sub(ui, MenuItem::new(&tr.name), |ui| {
                            for (k, l) in [
                                ("x", t("Left / right")),
                                ("z", t("Front / back")),
                                ("y", t("Height")),
                                ("dimmer", t("Dimmer")),
                                ("zoom", t("Zoom")),
                            ] {
                                let c = ui_fader(format!("tracking/{}/{k}", tr.id));
                                pick(ui, l, c, format!("{}: {l}", tr.name));
                            }
                        });
                    }
                });
            }
        } else {
            menu::sub(ui, MenuItem::new(t("Scene")), |ui| {
                pick(
                    ui,
                    t("Selected scene"),
                    Control::Scene {
                        target: SceneTarget::Selected,
                    },
                    t("Selected scene").to_string(),
                );
                menu::separator(ui);
                for z in &show.zones {
                    let scenes: Vec<_> = show
                        .scenes
                        .iter()
                        .filter(|s| s.zone == Some(z.id))
                        .collect();
                    if scenes.is_empty() {
                        continue;
                    }
                    menu::sub(ui, MenuItem::new(&z.name), |ui| {
                        for sc in scenes {
                            pick(
                                ui,
                                &sc.name,
                                Control::Scene {
                                    target: SceneTarget::Scene { id: sc.id },
                                },
                                sc.name.clone(),
                            );
                        }
                    });
                }
            });
            menu::sub(ui, MenuItem::new(t("Bank")), |ui| {
                pick(
                    ui,
                    t("Show all banks"),
                    Control::ShowBank { zone: None },
                    t("All").to_string(),
                );
                pick(
                    ui,
                    t("Next bank"),
                    Control::BankStep { delta: 1 },
                    t("Next bank").to_string(),
                );
                pick(
                    ui,
                    t("Previous bank"),
                    Control::BankStep { delta: -1 },
                    t("Previous bank").to_string(),
                );
                menu::separator(ui);
                for z in &show.zones {
                    menu::sub(ui, MenuItem::new(&z.name), |ui| {
                        pick(
                            ui,
                            t("Show bank"),
                            Control::ShowBank { zone: Some(z.id) },
                            z.name.clone(),
                        );
                        pick(
                            ui,
                            t("Pause"),
                            Control::ZonePause { zone: z.id },
                            format!("{}: {}", z.name, t("Pause")),
                        );
                        pick(
                            ui,
                            t("Next scene"),
                            Control::ZoneStep {
                                zone: z.id,
                                delta: 1,
                            },
                            format!("{} ›", z.name),
                        );
                        pick(
                            ui,
                            t("Previous scene"),
                            Control::ZoneStep {
                                zone: z.id,
                                delta: -1,
                            },
                            format!("‹ {}", z.name),
                        );
                    });
                }
            });
            if !show.groups.is_empty() {
                menu::sub(ui, MenuItem::new(t("Group")), |ui| {
                    pick(
                        ui,
                        t("All"),
                        Control::Group { id: None },
                        t("All").to_string(),
                    );
                    for g in &show.groups {
                        pick(
                            ui,
                            &g.name,
                            Control::Group { id: Some(g.id) },
                            g.name.clone(),
                        );
                    }
                });
            }
            if !show.trackers.is_empty() {
                menu::sub(ui, MenuItem::new(t("Live tracking")), |ui| {
                    for tr in &show.trackers {
                        menu::sub(ui, MenuItem::new(&tr.name), |ui| {
                            for (k, l) in [
                                ("follow", t("Follow")),
                                ("light", t("Own light")),
                                ("own_zoom", t("Own zoom")),
                            ] {
                                let c = ui_button(format!("tracking/{}/{k}", tr.id));
                                pick(ui, l, c, tr.name.clone());
                            }
                        });
                    }
                });
            }
            menu::sub(ui, MenuItem::new(t("Sound")), |ui| {
                for (i, pad) in show.sound.pads.iter().enumerate() {
                    if pad.path.is_empty() {
                        continue;
                    }
                    let name = crate::tr!("Pad {n}: {name}", n = i + 1, name = pad.label());
                    pick(ui, &name, ui_button(format!("sound/pad/{i}")), pad.label());
                }
                pick(
                    ui,
                    t("Stop all pads"),
                    ui_button("sound/pads/stop".into()),
                    t("Stop all pads").to_string(),
                );
                menu::separator(ui);
                for d in 0..show.sound.decks.len() {
                    let deck = crate::views::sound::deck_name(d);
                    for (k, l) in [("play", t("Play / pause")), ("stop", t("Stop"))] {
                        let name = crate::tr!("Player {d}: {what}", d = deck, what = l);
                        pick(
                            ui,
                            &name,
                            ui_button(format!("sound/deck/{d}/{k}")),
                            name.clone(),
                        );
                    }
                }
                pick(
                    ui,
                    t("Crossfade"),
                    ui_button("sound/xfade".into()),
                    t("Crossfade").to_string(),
                );
                pick(ui, t("Mute"), Control::AudioMute, t("Mute").to_string());
                pick(
                    ui,
                    t("Fade out"),
                    Control::AudioFade,
                    t("Fade out").to_string(),
                );
            });
            menu::sub(ui, MenuItem::new(t("Programmer")), |ui| {
                for (m, l) in [
                    ("Edit", "Edit"),
                    ("Blind", t("Blind edit")),
                    ("Live", t("Live")),
                ] {
                    pick(
                        ui,
                        l,
                        ui_button(format!("programmer/mode/{m}")),
                        l.to_string(),
                    );
                }
                pick(
                    ui,
                    t("Reset all beams"),
                    ui_button("programmer/reset_all".into()),
                    t("Reset all beams").to_string(),
                );
            });
            menu::separator(ui);
            pick(
                ui,
                t("Blackout"),
                Control::Blackout,
                t("Blackout").to_string(),
            );
            pick(
                ui,
                t("Stop all"),
                Control::StopAll,
                t("Stop all").to_string(),
            );
        }
        menu::separator(ui);
        ui.add(MenuItem::new(t("Or Alt + click on a control")).enabled(false));
        if item.control.is_some() && menu::item(ui, MenuItem::new(t("Remove target")).danger()) {
            out = Some((None, String::new()));
        }
    });
    out
}

/// Name of a target without its category (“Scene: Intro” → “Intro”).
fn short_name(full: &str) -> String {
    full.split_once(": ")
        .map_or_else(|| full.to_string(), |x| x.1.to_string())
}

/// Label with the icon tokens replaced by their glyphs (web remote: the
/// icon font is a fallback of the text font there).
pub(crate) fn label_with_glyphs(label: &str) -> String {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    label_tokens(label)
        .into_iter()
        .map(|part| match part {
            LabelPart::Text(t) => t.to_string(),
            LabelPart::Icon(name) => {
                icon_library::glyph(name).map_or_else(|| format!(":{name}:"), |c| c.to_string())
            }
        })
        .collect()
}

/// Does the label consist of icons only (then they are drawn large)?
fn icons_only(label: &str) -> bool {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    let parts = label_tokens(label);
    parts.iter().any(|p| matches!(p, LabelPart::Icon(_)))
        && parts.iter().all(|p| {
            matches!(p, LabelPart::Icon(_))
                || matches!(p, LabelPart::Text(t) if t.trim().is_empty())
        })
}

/// Text with icons (`:name:`), centered and wrapped.
fn label_galley(
    p: &egui::Painter,
    label: &str,
    size: f32,
    color: Color32,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    use egui::text::{LayoutJob, TextFormat};
    let text_fmt = TextFormat::simple(egui::FontId::proportional(size), color);
    let icon_fmt = TextFormat::simple(
        egui::FontId::new(
            size * 1.15,
            egui::FontFamily::Name(crate::ui::widgets::icons::ICON_FAMILY.into()),
        ),
        color,
    );
    let mut job = LayoutJob::default();
    for part in label_tokens(label) {
        match part {
            LabelPart::Text(t) => job.append(t, 0.0, text_fmt.clone()),
            LabelPart::Icon(name) => match icon_library::glyph(name) {
                Some(c) => job.append(&c.to_string(), 0.0, icon_fmt.clone()),
                None => job.append(&format!(":{name}:"), 0.0, text_fmt.clone()),
            },
        }
    }
    job.wrap.max_width = width.max(10.0);
    job.halign = Align::Center;
    p.layout_job(job)
}

/// Area below the label (color wheel, XY pad).
fn content_rect(rect: Rect, item: &TouchItem) -> Rect {
    let top = if item.label.is_empty() { 8.0 } else { 28.0 };
    Rect::from_min_max(rect.min + vec2(8.0, top), rect.max - vec2(8.0, 8.0))
}

/// Fader: (track with handle, travel of the handle center).
fn fader_parts(rect: Rect, horizontal: bool) -> (Rect, Rect) {
    const HANDLE_H: f32 = 34.0;
    if horizontal {
        // value on the left, track to the right of it
        let track = Rect::from_min_max(
            pos2(rect.left() + H_VALUE_W, rect.top()),
            pos2(rect.right() - 8.0, rect.bottom()),
        );
        let travel = Rect::from_min_max(
            pos2(track.left() + HANDLE_H * 0.5, track.top()),
            pos2(track.right() - HANDLE_H * 0.5, track.bottom()),
        );
        return (track, travel);
    }
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

/// Width of the value at the left of a horizontal fader.
const H_VALUE_W: f32 = 52.0;

/// Horizontal fader: value on the left, label above the track.
#[allow(clippy::too_many_arguments)]
fn paint_h_fader(
    p: &egui::Painter,
    item: &TouchItem,
    rect: Rect,
    value: f32,
    hot: bool,
    accent: Color32,
    label: &str,
    size: f32,
) {
    p.rect_filled(rect, theme::R_M, theme::raised());
    let (track, travel) = fader_parts(rect, true);
    let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
    p.text(
        pos2(rect.left() + H_VALUE_W * 0.5, rect.center().y),
        Align2::CENTER_CENTER,
        format!("{:.0}", value * 100.0),
        theme::heading_font(),
        text_color,
    );
    // label above the track (if there is room for it)
    let mut cy = track.center().y;
    if !label.is_empty() && rect.height() >= 56.0 {
        let g = p.layout(
            label.to_string(),
            egui::FontId::proportional(size.min(14.0)),
            text_color,
            track.width(),
        );
        p.galley(pos2(track.left(), rect.top() + 5.0), g.clone(), text_color);
        cy = (rect.top() + 5.0 + g.size().y + rect.bottom()) * 0.5;
    }
    // Slot with level in the element color
    let slot = Rect::from_center_size(pos2(track.center().x, cy), vec2(track.width() - 6.0, 6.0));
    p.rect_filled(slot, 3.0, theme::bg());
    let hx = travel.left() + travel.width() * value;
    let level = Rect::from_min_max(slot.min, pos2(hx, slot.bottom()));
    p.rect_filled(level, 3.0, accent);
    let handle = Rect::from_center_size(
        pos2(hx, cy),
        vec2(34.0, (rect.height() - 16.0).clamp(16.0, 40.0)),
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
            pos2(handle.center().x, handle.top() + 7.0),
            pos2(handle.center().x, handle.bottom() - 7.0),
        ],
        Stroke::new(2.5, theme::text()),
    );
}

/// Readable text color on a background.
fn text_on(bg: Color32) -> Color32 {
    let l = 0.299 * f32::from(bg.r()) + 0.587 * f32::from(bg.g()) + 0.114 * f32::from(bg.b());
    if l > 150.0 {
        Color32::from_gray(20)
    } else {
        Color32::WHITE
    }
}

/// Draw an element. `value`: fader/knob 0–1; `hot`: pressed or under the
/// pointer.
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
    // a fixed text size grows and shrinks with the view zoom
    let zoom = rect.width() / (item.w * UNIT).max(1.0);
    let size = if item.text_size > 0.0 {
        item.text_size * zoom
    } else if icons_only(&label) {
        (rect.height().min(rect.width()) * 0.42).max(14.0)
    } else {
        auto
    };
    let accent = base.unwrap_or(theme::accent());
    match item.kind {
        TouchKind::Label | TouchKind::Button => {
            let on = item.kind == TouchKind::Button
                && mapping_of(item).is_some_and(|m| crate::mapping::is_on(e, st, states, &m));
            let style = if item.kind == TouchKind::Button {
                item.style
            } else {
                ButtonStyle::Normal
            };
            let fill = match (item.kind, base) {
                (TouchKind::Button, _) if style == ButtonStyle::Outline => {
                    if on {
                        accent
                    } else {
                        Color32::TRANSPARENT
                    }
                }
                (_, Some(b)) => b,
                (TouchKind::Button, None) => theme::raised(),
                _ => Color32::TRANSPARENT,
            };
            let fill = if hot && fill != Color32::TRANSPARENT {
                fill.gamma_multiply(0.7)
            } else {
                fill
            };
            // round: a circle in the middle of the element
            let round = style == ButtonStyle::Round;
            let shape = if round {
                Rect::from_center_size(
                    rect.center(),
                    egui::Vec2::splat(rect.width().min(rect.height())),
                )
            } else {
                rect
            };
            let radius = if round {
                shape.width() / 2.0
            } else {
                f32::from(theme::R_M)
            };
            if fill != Color32::TRANSPARENT {
                p.rect_filled(shape, radius, fill);
            } else if hot {
                p.rect_filled(shape, radius, theme::raised());
            }
            if let Some(tex) = tex {
                paint_cover(p, rect.shrink(1.0), tex, if hot { 0.7 } else { 1.0 });
            }
            if item.kind == TouchKind::Button && style != ButtonStyle::Normal {
                paint_style(p, item, shape, radius, style, on, accent);
            } else if item.kind == TouchKind::Button {
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
                    // glowing strip at the top
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
            // Switch: the text sits next to / above the switch
            let rect = if item.kind == TouchKind::Button && style == ButtonStyle::Switch {
                switch_parts(rect).0
            } else {
                rect
            };
            if !label.is_empty() {
                let p = &p.with_clip_rect(p.clip_rect().intersect(rect));
                let galley = label_galley(p, &label, size, text_color, rect.width() - 12.0);
                // halign center: the galley is laid out around x = 0
                let pos = pos2(rect.center().x, rect.center().y - galley.size().y * 0.5);
                if tex.is_some() && item.text_color.is_none() {
                    // Shadow for readability on images
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
        TouchKind::Fader if item.horizontal => {
            paint_h_fader(p, item, rect, value, hot, accent, &label, size);
        }
        TouchKind::Fader => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let (track, travel) = fader_parts(rect, false);
            let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
            p.text(
                pos2(rect.center().x, rect.top() + 15.0),
                Align2::CENTER_CENTER,
                format!("{:.0}", value * 100.0),
                theme::heading_font(),
                text_color,
            );
            // Slot with level in the element color
            let slot = Rect::from_center_size(
                pos2(track.center().x, track.center().y),
                vec2(6.0, track.height() - 6.0),
            );
            p.rect_filled(slot, 3.0, theme::bg());
            let hy = travel.bottom() - travel.height() * value;
            let level = Rect::from_min_max(pos2(slot.left(), hy), slot.max);
            p.rect_filled(level, 3.0, accent);
            // Handle like in the programmer
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
        TouchKind::Tracking => {
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
            RoomMap::new(show, field).paint(
                &p.with_clip_rect(field.expand(2.0)),
                show,
                None,
                false,
            );
        }
    }
}

/// Switch style: (area for icon and text, switch track).
fn switch_parts(rect: Rect) -> (Rect, Rect) {
    let size = vec2(
        (rect.height() * 0.5).clamp(26.0, 46.0),
        (rect.height() * 0.28).clamp(14.0, 24.0),
    );
    if rect.width() >= rect.height() * 1.4 {
        // wide: switch on the right, text to its left
        let track = Rect::from_center_size(
            pos2(rect.right() - 10.0 - size.x / 2.0, rect.center().y),
            size,
        );
        (rect.with_max_x(track.left() - 4.0), track)
    } else {
        // narrow: switch at the bottom, text above
        let track = Rect::from_center_size(
            pos2(rect.center().x, rect.bottom() - 8.0 - size.y / 2.0),
            size,
        );
        (rect.with_max_y(track.top() - 2.0), track)
    }
}

/// Frame and state display of the button styles other than “Normal”.
fn paint_style(
    p: &egui::Painter,
    item: &TouchItem,
    shape: Rect,
    radius: f32,
    style: ButtonStyle,
    on: bool,
    accent: Color32,
) {
    match style {
        ButtonStyle::Normal => {}
        ButtonStyle::Outline => {
            p.rect_stroke(shape, radius, Stroke::new(2.0, accent), StrokeKind::Inside);
        }
        ButtonStyle::Round => {
            p.rect_stroke(
                shape,
                radius,
                if on {
                    Stroke::new(3.0, Color32::WHITE)
                } else {
                    Stroke::new(1.0, theme::border())
                },
                StrokeKind::Inside,
            );
        }
        ButtonStyle::Led => {
            p.rect_stroke(
                shape,
                radius,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let c = shape.left_top() + vec2(12.0, 12.0);
            if on {
                p.circle_filled(c, 8.0, accent.gamma_multiply(0.35));
                p.circle_filled(c, 5.0, led_color(item, accent));
            } else {
                p.circle_filled(c, 5.0, theme::bg());
            }
            p.circle_stroke(c, 5.0, Stroke::new(1.0, theme::border()));
        }
        ButtonStyle::Switch => {
            p.rect_stroke(
                shape,
                radius,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let track = switch_parts(shape).1;
            let r = track.height() / 2.0;
            p.rect_filled(
                track,
                r,
                if on {
                    led_color(item, accent)
                } else {
                    theme::bg()
                },
            );
            p.rect_stroke(
                track,
                r,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let knob = if on {
                track.right() - r
            } else {
                track.left() + r
            };
            p.circle_filled(pos2(knob, track.center().y), r - 3.0, Color32::WHITE);
        }
    }
}

/// Color of the on state: the element color, on a colored surface the accent.
fn led_color(item: &TouchItem, accent: Color32) -> Color32 {
    if item.color.is_some() {
        theme::OK
    } else {
        accent
    }
}

/// Current value 0–1 of a fader/knob (readable state, otherwise the last
/// set value – shared by touch mode and remote).
pub(crate) fn item_value(e: &Engine, st: &AppState, item: &TouchItem) -> f32 {
    let live = |c: &ChannelRef| {
        e.live()
            .get(c)
            .map_or(if c.is_virtual() { 1.0 } else { 0.0 }, |v| {
                f32::from(*v) / 255.0
            })
    };
    match item.control {
        Some(Control::Master) => e.master(),
        Some(Control::AudioMaster) => st.audio.master,
        Some(Control::SoundVolume { deck }) => crate::mapping::sound_volume(e, deck),
        Some(Control::Channels) => item.channels.first().map_or(0.0, live),
        Some(Control::Channel { channel }) => live(&channel),
        _ => st.touch_values.get(&item.id).copied().unwrap_or(0.0),
    }
}

/// Channels of one kind of a fixture.
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

/// Set a color as live values (red/green/blue of all cells).
pub(crate) fn set_color(e: &mut Engine, fixtures: &[FixtureId], rgb: [u8; 3]) {
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

pub(crate) fn current_color(e: &Engine, f: FixtureId) -> Option<[u8; 3]> {
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

/// Set pan/tilt as live values (0–1, fine channels 0).
pub(crate) fn set_position(e: &mut Engine, fixtures: &[FixtureId], x: f32, y: f32) {
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

pub(crate) fn current_position(e: &Engine, f: FixtureId) -> Option<(f32, f32)> {
    let get = |k| {
        channels_of(e.show(), f, k)
            .first()
            .and_then(|c| e.live().get(c).copied())
    };
    let x = get(ChannelKind::Pan)?;
    let y = get(ChannelKind::Tilt).unwrap_or(128);
    Some((f32::from(x) / 255.0, f32::from(y) / 255.0))
}

/// Take over the queued element (Alt + click): give it to the selected
/// element as target (editing touch) or create it on the current page.
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
    // Programmer fader: fixed to the channels currently controlled
    let (control, channels) = if pick.channels.is_empty() {
        (pick.control, Vec::new())
    } else {
        (Some(Control::Channels), pick.channels.clone())
    };
    let touch = &mut e.show_mut().touch;
    // the selected element in edit mode gets the target
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
                    if item.label.is_empty() {
                        item.label = short_name(&label);
                    }
                }
                st.info(t("Target assigned"));
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
                touch.add_page(crate::tr!("Page {n}", n = n))
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
    } else if matches!(control, Some(Control::Ui { .. })) {
        // interface controls have no name of their own in the show
        item.label = label.clone();
    } else {
        item.label = short_name(&label);
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
        st.info(crate::tr!("Added to touch page “{page}”", page = name));
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

        // the selected element in edit mode gets the target
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
