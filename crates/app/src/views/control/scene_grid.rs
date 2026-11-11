//! CONTROL → scene grid: banks as columns, scenes as cards.
//!
//! At the top the bank bar: “All”, one chip per bank and **+** for a new
//! bank. Every scene belongs to a bank. Right-clicking a bank (chip or column
//! header) opens the bank menu (rename, color, duplicate, hide, delete …).
//! Scene cards: a click on the main area starts/stops, a click on the
//! colored strip on the right selects the scene (Ctrl/Shift: several).
//! A started scene is selected automatically (on its own).
//! Delete removes selected scenes. Using the strip you drag scenes into
//! another bank or to another position (selected scenes move together).
//! **+** in the column header creates a scene and starts renaming it right
//! away.

use std::collections::BTreeSet;

use crate::i18n::t;
use crate::mapping;
use dmxcraft_core::mapping::{Control, SceneTarget};
use dmxcraft_core::{Engine, SceneId, Show, Step, ZoneId};
use eframe::egui::{self, Color32, Sense, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::color_picker::color_swatches;
use crate::ui::widgets::dialog::select_all;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::scene_card::{CARD_H, COMPACT_H};
use crate::ui::widgets::{text_field, Icon, IconButton, SceneCard};
use crate::views::control::programmer::programmer_values;
use crate::views::{View, ViewCtx};

const COLUMN_W: f32 = 176.0;

/// Which bank columns are shown.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum PageFilter {
    #[default]
    All,
    Zone(ZoneId),
}

enum Action {
    /// Bank clicked (target of the Delete key).
    FocusBank(ZoneId),
    /// Flash scene pressed (`true`) or released.
    Flash(SceneId, bool),
    Toggle(SceneId),
    Pause(SceneId),
    Jump(SceneId, i32),
    NewScene(ZoneId),
    Delete(SceneId),
    /// Select only this scene (click on the strip).
    Select(SceneId),
    /// Add the scene to the selection/remove it (Ctrl+strip).
    Mark(SceneId),
    /// All scenes from the last clicked one up to here (Shift+strip).
    Range(SceneId),
    /// Start renaming a scene.
    RenameScene(SceneId),
    /// Renaming a scene finished.
    SetSceneName(SceneId, String),
    DuplicateScene(SceneId),
    /// Hide scenes in the overview.
    HideScenes(Vec<SceneId>),
    /// Create a SUPER SCENE from the scenes (one after the other on track 1).
    CreateSuper(Vec<SceneId>),
    /// Drop the dragged scene (with the selection) into bank `.1` before scene
    /// `.2`.
    MoveScenes(SceneId, ZoneId, Option<SceneId>),
    /// Set the color of scenes (`None` = bank color).
    SceneColor(Vec<SceneId>, Option<[u8; 3]>),
    Bank(BankAction),
    /// Renaming in the chip finished.
    RenameBank(ZoneId, String),
    /// Toggle the compact view of the bank.
    Compact(ZoneId),
}

/// Actions from the bank menu.
#[derive(Clone, Copy)]
enum BankAction {
    New,
    Rename(ZoneId),
    Duplicate(ZoneId),
    Hide(ZoneId),
    Show(ZoneId),
    /// Show a hidden scene again.
    ShowScene(SceneId),
    /// Show all hidden scenes of the bank.
    ShowAllScenes(ZoneId),
    SetColor(ZoneId, Option<[u8; 3]>),
    ColorToScenes(ZoneId),
    SelectScenes(ZoneId),
    /// Delete the bank together with its scenes.
    Delete(ZoneId),
}

#[derive(Default)]
pub struct SceneGridView {
    page: PageFilter,
    /// Ongoing rename of a bank (directly in the chip).
    rename: Option<(ZoneId, String)>,
    /// Focus the field in the next frame (only once, otherwise it cannot be left
    /// with a click).
    rename_focus: bool,
    /// Ongoing rename of a scene (directly in the card).
    scene_rename: Option<(SceneId, String)>,
    scene_rename_focus: bool,
    /// Target offset of the bank bar (arrows/mouse wheel), animated.
    bar_scroll: f32,
    /// Scroll the bar to the end while a new bank is being named.
    bar_to_end: bool,
}

impl View for SceneGridView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        // Move scenes without a bank (older shows, import) into a bank
        if cx.engine.show().scenes.iter().any(|s| s.zone.is_none()) {
            let n = cx.engine.show().zones.len() + 1;
            cx.engine
                .show_mut()
                .adopt_free_scenes(tr!("Bank {n}", n = n));
        }
        let mut actions = Vec::new();
        self.bank_bar(ui, cx.engine, cx.state.locked, &mut actions);
        ui.add_space(theme::SP_XS);
        super::scene_groups::strip(ui, cx.engine, cx.state);
        ui.add_space(theme::SP_XS);
        let sel_before = cx.state.selected_scenes.clone();

        // a hidden bank can no longer be selected as a filter
        if let PageFilter::Zone(z) = self.page {
            if cx.engine.show().zone(z).is_none_or(|z| z.hidden) {
                self.page = PageFilter::All;
            }
        }
        // Bank change via keyboard/MIDI; displayed bank for mappings
        if let Some(req) = cx.state.request_bank.take() {
            self.page = match req {
                Some(z) if cx.engine.show().zone(z).is_some_and(|z| !z.hidden) => {
                    PageFilter::Zone(z)
                }
                _ => PageFilter::All,
            };
        }
        cx.state.shown_bank = match self.page {
            PageFilter::All => None,
            PageFilter::Zone(z) => Some(z),
        };
        let columns: Vec<(ZoneId, String)> = cx
            .engine
            .show()
            .zones
            .iter()
            .filter(|z| !z.hidden)
            .filter(|z| match self.page {
                PageFilter::All => true,
                PageFilter::Zone(id) => z.id == id,
            })
            .map(|z| (z.id, z.name.clone()))
            .collect();

        let locked = cx.state.locked;
        egui::ScrollArea::both()
            .id_salt("scene_grid")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.horizontal_top(|ui| {
                    for (zone, name) in &columns {
                        column(
                            ui,
                            cx.engine,
                            *zone,
                            name,
                            locked,
                            &cx.state.selected_scenes,
                            &mut self.scene_rename,
                            &mut self.scene_rename_focus,
                            cx.state.wrap_scenes,
                            &mut actions,
                        );
                        ui.add_space(theme::SP_S);
                    }
                });
            });

        drag_ghost(ui.ctx(), cx.engine.show(), &cx.state.selected_scenes);

        // Keyboard: Delete removes the last clicked bank or the selected scenes,
        // Esc clears the selection
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        if let (false, false, crate::state::Focus::Bank(z)) = (typing, locked, cx.state.focus) {
            if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
                && cx.engine.show().zone(z).is_some()
            {
                actions.push(Action::Bank(BankAction::Delete(z)));
            }
        }
        if !typing
            && !locked
            && cx.state.focus == crate::state::Focus::Scenes
            && !cx.state.selected_scenes.is_empty()
        {
            let (del, esc) = ui.input_mut(|i| {
                (
                    i.consume_key(egui::Modifiers::NONE, egui::Key::Delete),
                    i.consume_key(egui::Modifiers::NONE, egui::Key::Escape),
                )
            });
            if del {
                actions.extend(
                    cx.state
                        .selected_scenes
                        .iter()
                        .map(|id| Action::Delete(*id)),
                );
            }
            if esc {
                cx.state.selected_scenes.clear();
            }
        }
        let renaming = self.rename.is_some();
        let scene_renaming = self.scene_rename.as_ref().map(|(id, _)| *id);
        let clicked_scene = actions
            .iter()
            .any(|a| matches!(a, Action::Select(..) | Action::Mark(..) | Action::Range(..)));
        apply(
            cx.engine,
            cx.state,
            &mut self.rename,
            &mut self.scene_rename,
            actions,
        );
        if !renaming && self.rename.is_some() {
            self.rename_focus = true;
            // the new bank is at the end: scroll the bar there
            let last = cx.engine.show().zones.last().map(|z| z.id);
            self.bar_to_end = self.rename.as_ref().map(|(id, _)| *id) == last;
        }
        if self.rename.is_none() {
            self.bar_to_end = false;
        }
        // a scene clicked: the sidebar shows it instead of the group
        if clicked_scene || cx.state.selected_scenes != sel_before {
            cx.state.selected_group = None;
        }
        let now = self.scene_rename.as_ref().map(|(id, _)| *id);
        if now.is_some() && now != scene_renaming {
            self.scene_rename_focus = true;
        }
    }
}

impl SceneGridView {
    /// Bank bar as its own strip; if it is too long, arrows at the far ends (and
    /// the mouse wheel) scroll it sideways.
    fn bank_bar(&mut self, ui: &mut egui::Ui, e: &Engine, locked: bool, actions: &mut Vec<Action>) {
        const ARROW_W: f32 = 26.0;
        let full = ui.available_rect_before_wrap();
        let bar_h = 40.0;
        let rect = egui::Rect::from_min_size(full.min, Vec2::new(full.width(), bar_h));
        ui.allocate_rect(rect, Sense::hover());
        let p = ui.painter();
        p.rect_filled(rect, theme::R_M, theme::bg());
        p.rect_stroke(
            rect,
            theme::R_M,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );

        // Width of the content from the last frame
        let content_id = ui.make_persistent_id("bank_bar_content");
        let content_w: f32 = ui.ctx().data(|d| d.get_temp(content_id)).unwrap_or(0.0);
        let inner_full = rect.shrink2(Vec2::new(theme::SP_S, 0.0));
        let overflow = content_w > inner_full.width();
        let view = if overflow {
            rect.shrink2(Vec2::new(ARROW_W + 2.0, 0.0))
        } else {
            inner_full
        };
        let max = (content_w - view.width()).max(0.0);

        // Mouse wheel over the bar scrolls sideways
        if overflow && ui.rect_contains_pointer(rect) {
            let d = ui.input(|i| i.smooth_scroll_delta);
            let d = if d.x.abs() > d.y.abs() { d.x } else { d.y };
            self.bar_scroll -= d;
        }
        if overflow {
            let step = view.width() * 0.6;
            let left = egui::Rect::from_min_size(rect.min, Vec2::new(ARROW_W, bar_h));
            let right = egui::Rect::from_min_size(
                rect.right_top() - Vec2::new(ARROW_W, 0.0),
                Vec2::new(ARROW_W, bar_h),
            );
            if arrow(ui, left, Icon::ChevronLeft, self.bar_scroll > 0.5) {
                self.bar_scroll -= step;
            }
            if arrow(ui, right, Icon::ChevronRight, self.bar_scroll < max - 0.5) {
                self.bar_scroll += step;
            }
        }
        if self.bar_to_end {
            self.bar_scroll = max;
        }
        self.bar_scroll = self.bar_scroll.clamp(0.0, max);
        let offset = ui.ctx().animate_value_with_time(
            ui.make_persistent_id("bank_bar_anim"),
            self.bar_scroll,
            0.15,
        );

        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    egui::pos2(view.left() - offset, view.top()),
                    Vec2::new(f32::INFINITY, bar_h),
                ))
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child.set_clip_rect(view.intersect(ui.clip_rect()));
        let show = e.show();
        self.page_bar(&mut child, show, e, locked, actions);
        let used = child.min_rect().width();
        if (used - content_w).abs() > 0.5 {
            // The width changed: adjust arrows/offset in the next frame
            ui.ctx().data_mut(|d| d.insert_temp(content_id, used));
            ui.ctx().request_repaint();
        }
    }

    fn page_bar(
        &mut self,
        ui: &mut egui::Ui,
        show: &Show,
        e: &Engine,
        locked: bool,
        actions: &mut Vec<Action>,
    ) {
        ui.horizontal(|ui| {
            let all = chip(
                ui,
                &mut self.page,
                PageFilter::All,
                t("All"),
                theme::text(),
                false,
            );
            mapping::target(ui, all.rect, Control::ShowBank { zone: None });
            if !locked && show.zones.iter().any(|z| z.hidden) {
                menu::context(&all, |ui| {
                    menu::begin(ui);
                    hidden_banks_menu(ui, show, actions);
                });
            }
            for z in show.zones.iter().filter(|z| !z.hidden) {
                let c = theme::zone_color(show, Some(z.id));
                // Rename directly in the chip
                if let Some((id, text)) = self.rename.as_mut().filter(|(id, _)| *id == z.id) {
                    let r = ui.add(text_field(text).desired_width(120.0));
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
                        self.rename = None;
                    } else if enter || r.lost_focus() {
                        let name = text.trim().to_string();
                        let id = *id;
                        self.rename = None;
                        if !name.is_empty() {
                            actions.push(Action::RenameBank(id, name));
                        }
                    }
                    continue;
                }
                let live = e.active_in_zone(z.id).is_some();
                let resp = chip(ui, &mut self.page, PageFilter::Zone(z.id), &z.name, c, live);
                if mapping::target(ui, resp.rect, Control::ShowBank { zone: Some(z.id) }) {
                    continue;
                }
                if resp.clicked() {
                    actions.push(Action::FocusBank(z.id));
                }
                if !locked {
                    if resp.double_clicked() {
                        self.rename = Some((z.id, z.name.clone()));
                        self.rename_focus = true;
                    }
                    menu::context(&resp, |ui| {
                        if let Some(a) = bank_menu(ui, show, z.id) {
                            if let BankAction::Rename(id) = a {
                                self.rename = Some((id, z.name.clone()));
                                self.rename_focus = true;
                            } else {
                                actions.push(Action::Bank(a));
                            }
                        }
                    });
                }
            }
            if !locked
                && ui
                    .add(IconButton::new(Icon::Plus).flat())
                    .on_hover_text(t("New bank"))
                    .clicked()
            {
                actions.push(Action::Bank(BankAction::New));
            }
        });
    }
}

/// Submenu “Hidden banks” with one entry each to show it again.
fn hidden_banks_menu(ui: &mut egui::Ui, show: &Show, actions: &mut Vec<Action>) {
    let hidden: Vec<_> = show.zones.iter().filter(|z| z.hidden).collect();
    menu::sub(
        ui,
        MenuItem::new(t("Hidden banks"))
            .icon(Icon::LightOn)
            .enabled(!hidden.is_empty()),
        |ui| {
            for z in &hidden {
                let c = theme::zone_color(show, Some(z.id));
                let item = MenuItem::new(&z.name)
                    .icon_fn(move |p, r, _| {
                        p.circle_filled(r.center(), r.width().min(r.height()) * 0.32, c);
                    })
                    .tooltip(t("Unhide"));
                if menu::item(ui, item) {
                    actions.push(Action::Bank(BankAction::Show(z.id)));
                }
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Show all")).icon(Icon::LightOn)) {
                actions.extend(hidden.iter().map(|z| Action::Bank(BankAction::Show(z.id))));
            }
        },
    );
}

/// Bank menu (right-click on chip or column header).
fn bank_menu(ui: &mut egui::Ui, show: &Show, zone: ZoneId) -> Option<BankAction> {
    let mut out = None;
    // Selection outside `act` (which borrows `out` for its lifetime)
    let mut other = None;
    let mut act = |ui: &mut egui::Ui, item: MenuItem<'_>, a: BankAction| {
        if menu::item(ui, item) {
            out = Some(a);
        }
    };
    menu::begin(ui);
    let z = show.zone(zone)?;
    menu::title(ui, &z.name);
    act(
        ui,
        MenuItem::new(t("Rename")).icon(Icon::Edit),
        BankAction::Rename(zone),
    );
    act(
        ui,
        MenuItem::new(t("Duplicate")).icon(Icon::Copy),
        BankAction::Duplicate(zone),
    );
    act(
        ui,
        MenuItem::new(t("Hide")).icon(Icon::LightOff),
        BankAction::Hide(zone),
    );
    // show hidden scenes again one by one or all at once
    let hidden: Vec<(SceneId, String)> = show
        .scenes
        .iter()
        .filter(|s| s.zone == Some(zone) && s.hidden)
        .map(|s| (s.id, s.name.clone()))
        .collect();
    menu::sub(
        ui,
        MenuItem::new(&tr!("Hidden scenes ({n})", n = hidden.len()))
            .icon(Icon::EyeOff)
            .enabled(!hidden.is_empty()),
        |ui| {
            menu::begin(ui);
            for (id, name) in &hidden {
                if menu::item(ui, MenuItem::new(name).icon(Icon::Eye)) {
                    other = Some(BankAction::ShowScene(*id));
                }
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Show all")).icon(Icon::Eye)) {
                other = Some(BankAction::ShowAllScenes(zone));
            }
        },
    );
    menu::separator(ui);

    // Bank color: presets as squares, “Auto” and a custom color (+)
    ui.horizontal(|ui| {
        ui.add_space(theme::SP_S);
        ui.label(
            egui::RichText::new(t("Bank color"))
                .small()
                .color(theme::text_dim()),
        );
    });
    if let Some(c) = color_swatches(
        ui,
        ui.make_persistent_id(("bank_color", zone.0)),
        z.color,
        t("Automatic (by position)"),
    ) {
        other = Some(BankAction::SetColor(zone, c));
    }
    act(
        ui,
        MenuItem::new(t("Apply bank color to all scenes")).icon(Icon::Paste),
        BankAction::ColorToScenes(zone),
    );
    menu::separator(ui);
    act(
        ui,
        MenuItem::new(t("Select all scenes")).icon(Icon::ArrangeGrid),
        BankAction::SelectScenes(zone),
    );
    menu::separator(ui);
    act(
        ui,
        MenuItem::new(t("Delete"))
            .icon(Icon::Trash)
            .danger()
            .tooltip(t("Deletes the bank with all its scenes")),
        BankAction::Delete(zone),
    );
    out.or(other)
}

/// Scroll arrow at the edge of the bank bar.
fn arrow(ui: &mut egui::Ui, rect: egui::Rect, icon: Icon, enabled: bool) -> bool {
    let resp = ui.interact(
        rect,
        ui.make_persistent_id(("bar_arrow", icon == Icon::ChevronLeft)),
        Sense::click(),
    );
    let p = ui.painter();
    if enabled && resp.hovered() {
        p.rect_filled(rect.shrink(3.0), theme::R_S, theme::raised_hover());
    }
    icon.paint(
        p,
        egui::Rect::from_center_size(rect.center(), Vec2::splat(14.0)),
        if enabled {
            theme::text()
        } else {
            theme::text_faint()
        },
    );
    enabled && resp.clicked()
}

/// Bank tab; a dot shows that something is running in the bank.
fn chip(
    ui: &mut egui::Ui,
    page: &mut PageFilter,
    value: PageFilter,
    label: &str,
    color: Color32,
    live: bool,
) -> egui::Response {
    let selected = *page == value;
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), theme::body(), color);
    let w = galley.size().x + 2.0 * theme::SP_M + if live { 12.0 } else { 0.0 };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 30.0), Sense::click());
    let p = ui.painter();
    if selected || resp.hovered() {
        p.rect_filled(
            rect,
            theme::R_S,
            if selected {
                color.gamma_multiply(0.16)
            } else {
                theme::raised_hover()
            },
        );
    }
    // colored underline: thin, bold when selected
    let line_h = if selected { 3.0 } else { 1.5 };
    let line = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 4.0, rect.bottom() - line_h),
        egui::pos2(rect.right() - 4.0, rect.bottom()),
    );
    p.rect_filled(
        line,
        1.0,
        if selected {
            color
        } else {
            color.gamma_multiply(0.45)
        },
    );
    let mut x = rect.left() + theme::SP_M;
    if live {
        p.circle_filled(egui::pos2(x + 3.0, rect.center().y), 3.5, color);
        x += 12.0;
    }
    p.text(
        egui::pos2(x, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::body(),
        if selected { theme::text() } else { color },
    );
    if resp.clicked() {
        *page = value;
    }
    resp
}

#[allow(clippy::too_many_arguments)]
fn column(
    ui: &mut egui::Ui,
    e: &Engine,
    zone: ZoneId,
    name: &str,
    locked: bool,
    marked: &BTreeSet<SceneId>,
    rename: &mut Option<(SceneId, String)>,
    rename_focus: &mut bool,
    wrap: bool,
    actions: &mut Vec<Action>,
) {
    let color = theme::zone_color(e.show(), Some(zone));
    let compact = e.show().zones.iter().any(|z| z.id == zone && z.compact);
    let card_h = if compact { COMPACT_H } else { CARD_H };
    let scenes: Vec<SceneId> = e
        .show()
        .scenes
        .iter()
        .filter(|s| s.zone == Some(zone) && !s.hidden)
        .map(|s| s.id)
        .collect();
    // Target of the transport keys: active scene of the bank
    let target = e.active_in_zone(zone);
    let paused = target.and_then(|t| e.progress(t)).is_some_and(|p| p.paused);

    ui.vertical(|ui| {
        if wrap {
            ui.set_min_width(COLUMN_W);
        } else {
            ui.set_width(COLUMN_W);
        }
        // The click area lies below the buttons (UiBuilder::sense) so “+” and co.
        // keep their clicks and only the rest of the header opens the bank menu.
        let header = ui.scope_builder(egui::UiBuilder::new().sense(Sense::click()), |ui| {
            egui::Frame::new()
                .fill(theme::raised())
                .stroke(Stroke::new(1.0, theme::border()))
                .corner_radius(theme::R_M)
                .inner_margin(theme::SP_S)
                .show(ui, |ui| {
                    ui.set_width(COLUMN_W - 2.0 * theme::SP_S - 2.0);
                    let (bar, _) = ui
                        .allocate_exact_size(Vec2::new(ui.available_width(), 3.0), Sense::hover());
                    ui.painter().rect_filled(bar, 1.5, color);
                    ui.label(egui::RichText::new(name).strong().color(theme::ink(color)));
                    ui.horizontal(|ui| {
                        let on = target.is_some();
                        let sid = target.unwrap_or(SceneId(0));
                        let r = ui
                            .add(
                                IconButton::new(if paused { Icon::Play } else { Icon::Pause })
                                    .enabled(on)
                                    .active(paused),
                            )
                            .on_hover_text(t("Pause / resume"));
                        if !mapping::target(ui, r.rect, Control::ZonePause { zone }) && r.clicked()
                        {
                            actions.push(Action::Pause(sid));
                        }
                        let r = ui
                            .add(IconButton::new(Icon::Prev).enabled(on))
                            .on_hover_text(t("Previous step"));
                        if !mapping::target(ui, r.rect, Control::ZoneStep { zone, delta: -1 })
                            && r.clicked()
                        {
                            actions.push(Action::Jump(sid, -1));
                        }
                        let r = ui
                            .add(IconButton::new(Icon::Next).enabled(on))
                            .on_hover_text(t("Next step"));
                        if !mapping::target(ui, r.rect, Control::ZoneStep { zone, delta: 1 })
                            && r.clicked()
                        {
                            actions.push(Action::Jump(sid, 1));
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(IconButton::new(Icon::Plus))
                                .on_hover_text(t("New scene"))
                                .clicked()
                                && !locked
                            {
                                actions.push(Action::NewScene(zone));
                            }
                            if ui
                                .add(IconButton::new(Icon::ChevronDown).active(compact))
                                .on_hover_text(if compact {
                                    t("Normal view")
                                } else {
                                    t("Compact view (names only)")
                                })
                                .clicked()
                            {
                                actions.push(Action::Compact(zone));
                            }
                        });
                    });
                });
        });
        // Right-click on the column header: bank menu
        if !locked {
            let z = zone;
            let resp = header.response;
            menu::context(&resp, |ui| {
                if let Some(a) = bank_menu(ui, e.show(), z) {
                    actions.push(Action::Bank(a));
                }
            });
        }
        ui.add_space(theme::SP_XS);
        // Wrapping: as many cards stacked as fit down to the bottom edge, then
        // continue in the next column
        // Row height as actually used: card + spacing + egui spacing between the
        // elements (otherwise one card too many fits and it scrolls)
        let gap_y = ui.spacing().item_spacing.y;
        let row_h = card_h + theme::SP_XS + 2.0 * gap_y;
        let per_col = if wrap {
            // n cards need n · row − (row − card), small margin at the bottom
            let room = ui.clip_rect().bottom() - ui.cursor().top() - 4.0;
            (((room + row_h - card_h) / row_h).floor() as usize).max(1)
        } else {
            usize::MAX
        };
        let mut cards: Vec<(SceneId, egui::Pos2)> = Vec::new();
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            for chunk in scenes.chunks(per_col.min(scenes.len().max(1))) {
                ui.vertical(|ui| {
                    ui.set_width(COLUMN_W);
                    for &id in chunk {
                        let top = ui.cursor().top();
                        cards.push((id, egui::pos2(ui.cursor().left(), top + card_h * 0.5)));
                        scene_card(
                            ui,
                            e,
                            id,
                            color,
                            compact,
                            locked,
                            marked,
                            rename,
                            rename_focus,
                            actions,
                        );
                        ui.add_space(theme::SP_XS);
                    }
                });
            }
        });
        if !locked {
            drop_target(ui, zone, color, card_h, &cards, actions);
        }
    });
}

/// Label next to the mouse pointer while a scene is being dragged.
fn drag_ghost(ctx: &egui::Context, show: &Show, selected: &BTreeSet<SceneId>) {
    let Some(id) = egui::DragAndDrop::payload::<SceneId>(ctx) else {
        return;
    };
    let (Some(scene), Some(pos)) = (show.scene(*id), ctx.pointer_interact_pos()) else {
        return;
    };
    ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
    let label = if selected.contains(&*id) && selected.len() > 1 {
        tr!("{n} scenes", n = selected.len())
    } else {
        scene.name.clone()
    };
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(show, scene.zone), theme::rgb);
    egui::Area::new(egui::Id::unique("scene_drag_ghost"))
        .order(egui::Order::Tooltip)
        .fixed_pos(pos + Vec2::new(14.0, 6.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(theme::raised())
                .stroke(Stroke::new(1.0, color))
                .corner_radius(theme::R_S)
                .inner_margin(egui::Margin::symmetric(10, 6))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(egui::RichText::new(label).color(color))
                            .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
}

/// Dropping a dragged scene into this bank column: show the insertion mark
/// and move it on release.
fn drop_target(
    ui: &mut egui::Ui,
    zone: ZoneId,
    color: Color32,
    card_h: f32,
    cards: &[(SceneId, egui::Pos2)],
    actions: &mut Vec<Action>,
) {
    let Some(dragged) = egui::DragAndDrop::payload::<SceneId>(ui.ctx()) else {
        return;
    };
    let col = ui.min_rect();
    // Bank (all of its columns) down to the bottom edge of the visible area
    let area = egui::Rect::from_min_max(
        col.min,
        egui::pos2(
            col.right().max(col.left() + COLUMN_W),
            ui.clip_rect().bottom().max(col.bottom() + card_h),
        ),
    );
    let Some(pos) = ui
        .ctx()
        .pointer_interact_pos()
        .filter(|p| area.contains(*p))
    else {
        return;
    };
    let gap = theme::SP_XS * 0.5;
    // Column under the pointer (when wrapping there are several)
    let mut xs: Vec<f32> = cards.iter().map(|(_, c)| c.x).collect();
    xs.dedup_by(|a, b| (*a - *b).abs() < 1.0);
    let col_x = xs
        .iter()
        .copied()
        .rfind(|x| pos.x >= *x - theme::SP_XS)
        .unwrap_or(col.left());
    let in_col: Vec<&(SceneId, egui::Pos2)> = cards
        .iter()
        .filter(|(_, c)| (c.x - col_x).abs() < 1.0)
        .collect();
    let hit = in_col.iter().find(|(_, c)| pos.y < c.y);
    // before the card hit; below the last card of a column: before the first
    // card of the next column
    let (before, y) = match hit {
        Some((id, c)) => (Some(*id), c.y - card_h * 0.5 - gap),
        None => {
            let last = in_col.last().map(|(id, _)| *id);
            let next = last
                .and_then(|l| cards.iter().position(|(id, _)| *id == l))
                .and_then(|i| cards.get(i + 1))
                .map(|(id, _)| *id);
            let y = in_col
                .last()
                .map_or(col.bottom() + gap, |(_, c)| c.y + card_h * 0.5 + gap);
            (next, y)
        }
    };
    // only the insertion line shows the target position (no frame around the
    // bank)
    let p = ui.painter();
    p.line_segment(
        [egui::pos2(col_x, y), egui::pos2(col_x + COLUMN_W, y)],
        Stroke::new(2.5, color),
    );
    if ui.input(|i| i.pointer.any_released()) {
        actions.push(Action::MoveScenes(*dragged, zone, before));
        egui::DragAndDrop::clear_payload(ui.ctx());
    }
}

#[allow(clippy::too_many_arguments)]
fn scene_card(
    ui: &mut egui::Ui,
    e: &Engine,
    id: SceneId,
    bank_color: Color32,
    compact: bool,
    locked: bool,
    marked: &BTreeSet<SceneId>,
    rename: &mut Option<(SceneId, String)>,
    rename_focus: &mut bool,
    actions: &mut Vec<Action>,
) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let progress = e.progress(id);
    let is_super = scene.timeline().is_some();
    let kind = match (scene.steps.len(), progress) {
        _ if is_super => t("SUPER SCENE").to_string(),
        _ if !scene.effects.is_empty() => t("FX").to_string(),
        (_, Some(p)) if p.step_count > 1 => tr!(
            "SEQUENCE · {step_nr}/{step_count}",
            step_nr = p.step + 1,
            step_count = p.step_count
        ),
        (n, _) if n > 1 => tr!("SEQUENCE · {n} STEPS", n = n),
        _ => t("STATIC").to_string(),
    };
    let is_marked = marked.contains(&id);
    let resp = SceneCard {
        name: &scene.name,
        kind,
        color: scene.color.map_or(bank_color, theme::rgb),
        active: progress.is_some(),
        paused: progress.is_some_and(|p| p.paused),
        progress: progress.map_or(0.0, |p| {
            (p.step as f32 + p.step_fraction) / p.step_count.max(1) as f32
        }),
        width: COLUMN_W,
        selected: is_marked,
        sequence: scene.steps.len() > 1,
        fade: e.fade_state(id),
        flash: scene.settings.flash,
        super_scene: is_super,
        compact,
        missing: is_super && crate::missing_audio::scene_missing(ui.ctx(), id),
    }
    .show(ui);
    if e.fade_state(id).is_some() {
        ui.ctx().request_repaint();
    }
    let element = Control::Scene {
        target: SceneTarget::Scene { id },
    };
    if mapping::target(ui, resp.whole().rect, element) {
        return;
    }
    if !locked {
        resp.select.dnd_set_drag_payload(id);
    }

    // Rename directly in the card (above the name)
    if let Some((_, text)) = rename.as_mut().filter(|(r, _)| *r == id) {
        let field = egui::Rect::from_min_size(
            resp.main_rect.min + Vec2::new(4.0, 4.0),
            Vec2::new(resp.main_rect.width() - 8.0, 22.0),
        );
        let r = ui.put(field, text_field(text).desired_width(field.width()));
        if std::mem::take(rename_focus) {
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
            *rename = None;
        } else if enter || r.lost_focus() {
            let name = text.trim().to_string();
            *rename = None;
            if !name.is_empty() {
                actions.push(Action::SetSceneName(id, name));
            }
        }
        return;
    }

    // Strip: select (Ctrl/Shift: several), main area: start/stop
    if resp.select.clicked() && !locked {
        let mods = ui.input(|i| i.modifiers);
        actions.push(if mods.shift {
            Action::Range(id)
        } else if mods.command {
            Action::Mark(id)
        } else {
            Action::Select(id)
        });
    }
    if scene.settings.flash {
        // Flash: only runs while the card is pressed
        let flag = egui::Id::unique(("flash_down", id.0));
        let was: bool = ui.data(|d| d.get_temp(flag)).unwrap_or(false);
        let down = resp.play.is_pointer_button_down_on();
        if down && !was {
            ui.data_mut(|d| d.insert_temp(flag, true));
            actions.push(Action::Flash(id, true));
            if !locked {
                actions.push(Action::Select(id));
            }
        } else if was && !ui.input(|i| i.pointer.primary_down()) {
            ui.data_mut(|d| d.insert_temp(flag, false));
            actions.push(Action::Flash(id, false));
        }
    } else if resp.play.clicked() {
        actions.push(Action::Toggle(id));
        // a started scene is selected at the same time (on its own); multi-selection
        // is only possible via the strip
        if progress.is_none() && !locked {
            actions.push(Action::Select(id));
        }
    }
    if locked {
        return;
    }
    // The menu acts on all selected scenes if this one belongs to them
    let targets: Vec<SceneId> = if is_marked {
        marked.iter().copied().collect()
    } else {
        vec![id]
    };
    let current = scene.color;
    menu::context(&resp.whole(), |ui| {
        menu::begin(ui);
        let title = if targets.len() > 1 {
            tr!("{n} scenes", n = targets.len())
        } else {
            scene.name.clone()
        };
        menu::title(ui, &title);
        let single = targets.len() == 1;
        if menu::item(
            ui,
            MenuItem::new(t("Rename")).icon(Icon::Edit).enabled(single),
        ) {
            actions.push(Action::RenameScene(id));
        }
        if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
            actions.extend(targets.iter().map(|s| Action::DuplicateScene(*s)));
        }
        if menu::item(
            ui,
            MenuItem::new(t("Add to touch surface")).icon(Icon::Touch),
        ) {
            for s in &targets {
                mapping::touch_pick_control(
                    ui.ctx(),
                    Control::Scene {
                        target: SceneTarget::Scene { id: *s },
                    },
                );
            }
        }
        if menu::item(ui, MenuItem::new(t("Hide")).icon(Icon::EyeOff)) {
            actions.push(Action::HideScenes(targets.clone()));
        }
        if menu::item(
            ui,
            MenuItem::new(t("Create super scene"))
                .icon(Icon::Music)
                .tooltip(t("New super scene with these scenes in sequence")),
        ) {
            actions.push(Action::CreateSuper(targets.clone()));
        }
        menu::separator(ui);
        ui.horizontal(|ui| {
            ui.add_space(theme::SP_S);
            ui.label(
                egui::RichText::new(t("Scene color"))
                    .small()
                    .color(theme::text_dim()),
            );
        });
        if let Some(c) = color_swatches(
            ui,
            ui.make_persistent_id(("scene_color", id.0)),
            current,
            t("Use bank color"),
        ) {
            actions.push(Action::SceneColor(targets.clone(), c));
        }
        menu::separator(ui);
        if menu::item(
            ui,
            MenuItem::new(t("Delete"))
                .icon(Icon::Trash)
                .shortcut(crate::titlebar::keys::delete())
                .danger(),
        ) {
            actions.extend(targets.iter().map(|s| Action::Delete(*s)));
        }
    });
}

fn apply(
    e: &mut Engine,
    st: &mut AppState,
    rename: &mut Option<(ZoneId, String)>,
    scene_rename: &mut Option<(SceneId, String)>,
    actions: Vec<Action>,
) {
    for a in actions {
        match a {
            Action::FocusBank(z) => st.focus = crate::state::Focus::Bank(z),
            Action::Toggle(id) => {
                st.focus = crate::state::Focus::Scenes;
                e.toggle(id);
            }
            Action::Flash(id, true) => e.activate(id),
            Action::Flash(id, false) => e.release(id),
            Action::Pause(id) => e.toggle_pause(id),
            Action::Jump(id, d) => e.jump_step(id, d),
            Action::NewScene(zone) => {
                // Take values from the programmer, otherwise an empty scene
                let values = programmer_values(e);
                let name = tr!("Scene {n}", n = e.show().scenes.len() + 1);
                // new scenes last 5 s
                let mut step = Step::new(values);
                step.hold_ms = 5000;
                let id = e.show_mut().add_scene(name.clone(), Some(zone), vec![step]);
                // select the new scene right away
                st.selected_scenes.clear();
                st.selected_scenes.insert(id);
                st.focus = crate::state::Focus::Scenes;
                *scene_rename = Some((id, name));
            }
            Action::RenameScene(id) => {
                if let Some(s) = e.show().scene(id) {
                    *scene_rename = Some((id, s.name.clone()));
                }
            }
            Action::SetSceneName(id, name) => {
                if let Some(s) = e.show_mut().scene_mut(id) {
                    s.name = name;
                }
            }
            Action::DuplicateScene(id) => {
                let Some(name) = e.show().scene(id).map(|s| s.name.clone()) else {
                    continue;
                };
                e.show_mut()
                    .duplicate_scene(id, tr!("{name} (copy)", name = name));
            }
            Action::HideScenes(ids) => {
                for id in &ids {
                    if let Some(s) = e.show_mut().scene_mut(*id) {
                        s.hidden = true;
                    }
                    st.selected_scenes.remove(id);
                }
                st.info(tr!(
                    "{n} scene(s) hidden – right-click the bank to show them again",
                    n = ids.len()
                ));
            }
            Action::CreateSuper(ids) => {
                // in display order
                let ordered: Vec<SceneId> = e
                    .show()
                    .scenes
                    .iter()
                    .map(|s| s.id)
                    .filter(|i| ids.contains(i))
                    .collect();
                let name = if ordered.len() == 1 {
                    let n = e
                        .show()
                        .scene(ordered[0])
                        .map_or(String::new(), |s| s.name.clone());
                    tr!("{name} (Super)", name = n)
                } else {
                    t("Super scene").to_string()
                };
                if let Some(id) = e.show_mut().create_super_scene(&ordered, name) {
                    st.selected_scenes.clear();
                    st.selected_scenes.insert(id);
                }
            }
            Action::MoveScenes(dragged, zone, before) => {
                // selected scenes move along if the dragged one belongs to them
                let ids: Vec<SceneId> = if st.selected_scenes.contains(&dragged) {
                    e.show()
                        .scenes
                        .iter()
                        .map(|s| s.id)
                        .filter(|id| st.selected_scenes.contains(id))
                        .collect()
                } else {
                    vec![dragged]
                };
                // keep the previous (bank) color instead of inheriting the new bank color
                for id in &ids {
                    let Some(old) = e.show().scene(*id).map(|s| s.zone) else {
                        continue;
                    };
                    if old == Some(zone) {
                        continue;
                    }
                    let c = theme::zone_color(e.show(), old);
                    if let Some(s) = e.show_mut().scene_mut(*id).filter(|s| s.color.is_none()) {
                        s.color = Some([c.r(), c.g(), c.b()]);
                    }
                }
                e.show_mut().move_scenes(&ids, zone, before);
                e.sync();
            }
            Action::SceneColor(ids, c) => {
                for id in ids {
                    if let Some(s) = e.show_mut().scene_mut(id) {
                        s.color = c;
                    }
                }
            }
            Action::Delete(id) => {
                e.show_mut().remove_scene(id);
                e.sync();
                st.selected_scenes.remove(&id);
            }
            Action::Select(id) => {
                st.focus = crate::state::Focus::Scenes;
                st.selected_scenes.clear();
                st.selected_scenes.insert(id);
                st.scene_anchor = Some(id);
            }
            Action::Mark(id) => {
                st.focus = crate::state::Focus::Scenes;
                if !st.selected_scenes.remove(&id) {
                    st.selected_scenes.insert(id);
                }
                st.scene_anchor = Some(id);
            }
            Action::Range(id) => {
                st.focus = crate::state::Focus::Scenes;
                // Order as displayed: banks from left to right, within them the scenes from
                // top to bottom (hidden ones do not count)
                let show = e.show();
                let order: Vec<SceneId> = show
                    .zones
                    .iter()
                    .filter(|z| !z.hidden)
                    .flat_map(|z| {
                        show.scenes
                            .iter()
                            .filter(move |s| s.zone == Some(z.id) && !s.hidden)
                            .map(|s| s.id)
                    })
                    .collect();
                let anchor = st.scene_anchor.filter(|a| order.contains(a)).unwrap_or(id);
                if let (Some(a), Some(b)) = (
                    order.iter().position(|s| *s == anchor),
                    order.iter().position(|s| *s == id),
                ) {
                    let (lo, hi) = (a.min(b), a.max(b));
                    st.selected_scenes = order[lo..=hi].iter().copied().collect();
                }
            }
            Action::Compact(id) => {
                if let Some(z) = e.show_mut().zone_mut(id) {
                    z.compact = !z.compact;
                }
            }
            Action::RenameBank(id, name) => {
                if let Some(z) = e.show_mut().zone_mut(id) {
                    z.name = name;
                }
            }
            Action::Bank(b) => apply_bank(e, st, rename, b),
        }
    }
}

fn apply_bank(
    e: &mut Engine,
    st: &mut AppState,
    rename: &mut Option<(ZoneId, String)>,
    action: BankAction,
) {
    match action {
        BankAction::New => {
            let n = e.show().zones.len() + 1;
            let id = e.show_mut().add_zone(tr!("Bank {n}", n = n));
            // let the user rename it right away
            *rename = Some((id, tr!("Bank {n}", n = n)));
        }
        BankAction::Rename(id) => {
            if let Some(z) = e.show().zone(id) {
                *rename = Some((id, z.name.clone()));
            }
        }
        BankAction::Duplicate(id) => {
            let Some(name) = e.show().zone(id).map(|z| z.name.clone()) else {
                return;
            };
            e.show_mut()
                .duplicate_zone(id, tr!("{name} (copy)", name = name));
        }
        BankAction::Hide(id) => {
            if let Some(z) = e.show_mut().zone_mut(id) {
                z.hidden = true;
                st.info(tr!(
                    "“{name}” hidden – right-click “All” to show it again",
                    name = z.name
                ));
            }
        }
        BankAction::Show(id) => {
            if let Some(z) = e.show_mut().zone_mut(id) {
                z.hidden = false;
            }
        }
        BankAction::ShowScene(id) => {
            if let Some(s) = e.show_mut().scene_mut(id) {
                s.hidden = false;
            }
        }
        BankAction::ShowAllScenes(z) => {
            for s in e.show_mut().scenes.iter_mut().filter(|s| s.zone == Some(z)) {
                s.hidden = false;
            }
        }
        BankAction::SetColor(id, c) => {
            if let Some(z) = e.show_mut().zone_mut(id) {
                z.color = c;
            }
        }
        BankAction::ColorToScenes(id) => {
            let c = theme::zone_color(e.show(), Some(id));
            let rgb = [c.r(), c.g(), c.b()];
            for s in e
                .show_mut()
                .scenes
                .iter_mut()
                .filter(|s| s.zone == Some(id))
            {
                s.color = Some(rgb);
            }
        }
        BankAction::SelectScenes(id) => {
            st.selected_scenes = e
                .show()
                .scenes
                .iter()
                .filter(|s| s.zone == Some(id))
                .map(|s| s.id)
                .collect();
        }
        BankAction::Delete(id) => {
            e.show_mut().remove_zone_with_scenes(id);
            e.sync();
            st.selected_scenes.retain(|s| e.show().scene(*s).is_some());
        }
    }
}
