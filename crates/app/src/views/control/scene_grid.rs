//! STEUERN → Szenen-Raster: Bänke als Spalten, Szenen als Karten.
//!
//! Oben die Bankleiste: „Alle“, eine Chip je Bank und **+** für eine neue
//! Bank. Jede Szene gehört zu einer Bank. Rechtsklick auf eine Bank (Chip oder Spaltenkopf) öffnet das
//! Bank-Menü (Umbenennen, Farbe, Duplizieren, Ausblenden, Löschen …).
//! Szenenkarten: Klick auf den Hauptbereich startet/stoppt, Klick auf den
//! farbigen Streifen rechts wählt die Szene aus (Strg/Umschalt: mehrere).
//! Entf löscht ausgewählte Szenen. **+** im Spaltenkopf legt eine Szene an und startet
//! gleich das Umbenennen.

use std::collections::BTreeSet;

use crate::i18n::t;
use dmxcraft_core::{Engine, SceneId, Show, Step, ZoneId};
use eframe::egui::{self, Color32, Sense, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::color_picker::color_swatches;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{text_field, Icon, IconButton, SceneCard};
use crate::views::control::programmer::programmer_values;
use crate::views::{View, ViewCtx};

const COLUMN_W: f32 = 176.0;

/// Welche Bank-Spalten angezeigt werden.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum PageFilter {
    #[default]
    All,
    Zone(ZoneId),
}

enum Action {
    Toggle(SceneId),
    Pause(SceneId),
    Jump(SceneId, i32),
    NewScene(ZoneId),
    Delete(SceneId),
    /// Nur diese Szene auswählen (Klick auf den Streifen).
    Select(SceneId),
    /// Szene zur Auswahl hinzufügen/entfernen (Strg/Umschalt+Streifen).
    Mark(SceneId),
    /// Umbenennen einer Szene starten.
    RenameScene(SceneId),
    /// Umbenennung einer Szene abgeschlossen.
    SetSceneName(SceneId, String),
    DuplicateScene(SceneId),
    /// Farbe für Szenen setzen (`None` = Bankfarbe).
    SceneColor(Vec<SceneId>, Option<[u8; 3]>),
    Bank(BankAction),
    /// Umbenennung im Chip abgeschlossen.
    RenameBank(ZoneId, String),
    OpenBanksWindow,
}

/// Aktionen aus dem Bank-Menü.
#[derive(Clone, Copy)]
enum BankAction {
    New,
    Rename(ZoneId),
    Duplicate(ZoneId),
    Hide(ZoneId),
    Show(ZoneId),
    SetColor(ZoneId, Option<[u8; 3]>),
    ColorToScenes(ZoneId),
    SelectScenes(ZoneId),
    /// Bank samt Szenen löschen.
    Delete(ZoneId),
}

#[derive(Default)]
pub struct SceneGridView {
    page: PageFilter,
    /// Laufende Umbenennung einer Bank (direkt im Chip).
    rename: Option<(ZoneId, String)>,
    /// Feld beim nächsten Frame fokussieren (nur einmal, sonst lässt es
    /// sich nicht per Klick verlassen).
    rename_focus: bool,
    /// Laufende Umbenennung einer Szene (direkt in der Karte).
    scene_rename: Option<(SceneId, String)>,
    scene_rename_focus: bool,
}

impl View for SceneGridView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        // Szenen ohne Bank (ältere Shows, Import) in eine Bank übernehmen
        if cx.engine.show().scenes.iter().any(|s| s.zone.is_none()) {
            let n = cx.engine.show().zones.len() + 1;
            cx.engine
                .show_mut()
                .adopt_free_scenes(tr!("Bank {n}", n = n));
        }
        let mut actions = Vec::new();
        ui.horizontal(|ui| {
            self.page_bar(
                ui,
                cx.engine.show(),
                cx.engine,
                cx.state.locked,
                &mut actions,
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                playback_controls(ui, cx.engine);
            });
        });
        ui.add_space(theme::SP_XS);

        // ausgeblendete Bank als Filter ist nicht mehr wählbar
        if let PageFilter::Zone(z) = self.page {
            if cx.engine.show().zone(z).is_none_or(|z| z.hidden) {
                self.page = PageFilter::All;
            }
        }
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
                            &mut actions,
                        );
                        ui.add_space(theme::SP_S);
                    }
                });
            });

        // Tastatur: Entf löscht markierte Szenen, Esc hebt die Markierung auf
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        if !typing && !locked && !cx.state.selected_scenes.is_empty() {
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
        apply(
            cx.engine,
            cx.state,
            &mut self.rename,
            &mut self.scene_rename,
            actions,
        );
        if !renaming && self.rename.is_some() {
            self.rename_focus = true;
        }
        let now = self.scene_rename.as_ref().map(|(id, _)| *id);
        if now.is_some() && now != scene_renaming {
            self.scene_rename_focus = true;
        }
    }
}

impl SceneGridView {
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
                t("Alle"),
                theme::text(),
                false,
            );
            if !locked && show.zones.iter().any(|z| z.hidden) {
                menu::context(&all, |ui| {
                    menu::begin(ui);
                    hidden_banks_menu(ui, show, actions);
                });
            }
            for z in show.zones.iter().filter(|z| !z.hidden) {
                let c = theme::zone_color(show, Some(z.id));
                // Umbenennen direkt im Chip
                if let Some((id, text)) = self.rename.as_mut().filter(|(id, _)| *id == z.id) {
                    let r = ui.add(text_field(text).desired_width(120.0));
                    if std::mem::take(&mut self.rename_focus) {
                        r.request_focus();
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
                    .on_hover_text(t("Neue Bank"))
                    .clicked()
            {
                actions.push(Action::Bank(BankAction::New));
            }
            if !locked
                && ui
                    .add(IconButton::new(Icon::Edit).flat())
                    .on_hover_text(t("Bänke & Gruppen bearbeiten"))
                    .clicked()
            {
                actions.push(Action::OpenBanksWindow);
            }
        });
    }
}

/// Untermenü „Ausgeblendete Bänke“ mit je einem Eintrag zum Einblenden.
fn hidden_banks_menu(ui: &mut egui::Ui, show: &Show, actions: &mut Vec<Action>) {
    let hidden: Vec<_> = show.zones.iter().filter(|z| z.hidden).collect();
    menu::sub(
        ui,
        MenuItem::new(t("Ausgeblendete Bänke"))
            .icon(Icon::LightOn)
            .enabled(!hidden.is_empty()),
        |ui| {
            for z in hidden {
                if menu::item(ui, MenuItem::new(&z.name).tooltip(t("Einblenden"))) {
                    actions.push(Action::Bank(BankAction::Show(z.id)));
                }
            }
        },
    );
}

/// Bank-Menü (Rechtsklick auf Chip oder Spaltenkopf).
fn bank_menu(ui: &mut egui::Ui, show: &Show, zone: ZoneId) -> Option<BankAction> {
    let mut out = None;
    // Auswahl außerhalb von `act` (das `out` dauerhaft ausleiht)
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
        MenuItem::new(t("Umbenennen")).icon(Icon::Edit),
        BankAction::Rename(zone),
    );
    act(
        ui,
        MenuItem::new(t("Duplizieren")).icon(Icon::Copy),
        BankAction::Duplicate(zone),
    );
    act(
        ui,
        MenuItem::new(t("Ausblenden")).icon(Icon::LightOff),
        BankAction::Hide(zone),
    );
    menu::separator(ui);

    // Bankfarbe: Vorgaben als Quadrate, „Auto“ und eigene Farbe (+)
    ui.horizontal(|ui| {
        ui.add_space(theme::SP_S);
        ui.label(
            egui::RichText::new(t("Bankfarbe"))
                .small()
                .color(theme::text_dim()),
        );
    });
    if let Some(c) = color_swatches(
        ui,
        ui.make_persistent_id(("bank_color", zone.0)),
        z.color,
        t("Automatisch (nach Position)"),
    ) {
        other = Some(BankAction::SetColor(zone, c));
    }
    act(
        ui,
        MenuItem::new(t("Bankfarbe allen Szenen zuordnen")).icon(Icon::Paste),
        BankAction::ColorToScenes(zone),
    );
    menu::separator(ui);
    act(
        ui,
        MenuItem::new(t("Alle Szenen auswählen")).icon(Icon::ArrangeGrid),
        BankAction::SelectScenes(zone),
    );
    menu::separator(ui);
    act(
        ui,
        MenuItem::new(t("Löschen"))
            .icon(Icon::Trash)
            .danger()
            .tooltip(t("Löscht die Bank mit allen Szenen")),
        BankAction::Delete(zone),
    );
    out.or(other)
}

/// Master, Speed, Blackout und Alle aus (rechts neben den Zonen-Chips).
fn playback_controls(ui: &mut egui::Ui, e: &mut Engine) {
    let bo = e.blackout();
    let bo_btn = egui::Button::new(egui::RichText::new(t("Blackout")).strong().color(if bo {
        Color32::WHITE
    } else {
        theme::text()
    }))
    .fill(if bo { theme::DANGER } else { theme::raised() })
    .min_size(Vec2::new(88.0, 28.0));
    if ui.add(bo_btn).on_hover_text(t("Taste B")).clicked() {
        e.set_blackout(!bo);
    }
    if ui
        .button(t("Alle aus"))
        .on_hover_text(t("Alle Szenen stoppen"))
        .clicked()
    {
        e.release_all();
    }
    ui.spacing_mut().slider_width = 110.0;
    let mut speed = e.speed();
    if ui
        .add(
            egui::Slider::new(&mut speed, 0.1..=4.0)
                .logarithmic(true)
                .fixed_decimals(2)
                .suffix("x"),
        )
        .on_hover_text(t("Wiedergabegeschwindigkeit"))
        .changed()
    {
        e.set_speed(speed);
    }
    ui.label(egui::RichText::new(t("Speed")).color(theme::text_dim()));
    let mut master = e.master() * 100.0;
    if ui
        .add(
            egui::Slider::new(&mut master, 0.0..=100.0)
                .integer()
                .suffix("%"),
        )
        .changed()
    {
        e.set_master(master / 100.0);
    }
    ui.label(egui::RichText::new(t("Master")).color(theme::text_dim()));
}

/// Filter-Chip; ein Punkt zeigt an, dass in der Bank etwas läuft.
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
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 28.0), Sense::click());
    let p = ui.painter();
    let r = 14.0;
    p.rect_filled(
        rect,
        r,
        if selected {
            color.gamma_multiply(0.22)
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::raised()
        },
    );
    p.rect_stroke(
        rect,
        r,
        Stroke::new(1.0, if selected { color } else { theme::border() }),
        StrokeKind::Inside,
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
    actions: &mut Vec<Action>,
) {
    let color = theme::zone_color(e.show(), Some(zone));
    let scenes: Vec<SceneId> = e
        .show()
        .scenes
        .iter()
        .filter(|s| s.zone == Some(zone))
        .map(|s| s.id)
        .collect();
    // Ziel der Transport-Tasten: aktive Szene der Bank
    let target = e.active_in_zone(zone);
    let paused = target.and_then(|t| e.progress(t)).is_some_and(|p| p.paused);

    ui.vertical(|ui| {
        ui.set_width(COLUMN_W);
        // Klick-Fläche liegt unter den Knöpfen (UiBuilder::sense), damit „+“ & Co.
        // ihre Klicks behalten und nur der Rest des Kopfs das Bank-Menü öffnet.
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
                    ui.label(egui::RichText::new(name).strong().color(color));
                    ui.horizontal(|ui| {
                        let on = target.is_some();
                        let sid = target.unwrap_or(SceneId(0));
                        if ui
                            .add(
                                IconButton::new(if paused { Icon::Play } else { Icon::Pause })
                                    .enabled(on)
                                    .active(paused),
                            )
                            .on_hover_text(t("Pause / Weiter"))
                            .clicked()
                        {
                            actions.push(Action::Pause(sid));
                        }
                        if ui
                            .add(IconButton::new(Icon::Prev).enabled(on))
                            .on_hover_text(t("Vorheriger Schritt"))
                            .clicked()
                        {
                            actions.push(Action::Jump(sid, -1));
                        }
                        if ui
                            .add(IconButton::new(Icon::Next).enabled(on))
                            .on_hover_text(t("Nächster Schritt"))
                            .clicked()
                        {
                            actions.push(Action::Jump(sid, 1));
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(IconButton::new(Icon::Plus))
                                .on_hover_text(t("Neue Szene"))
                                .clicked()
                                && !locked
                            {
                                actions.push(Action::NewScene(zone));
                            }
                        });
                    });
                });
        });
        // Rechtsklick auf den Spaltenkopf: Bank-Menü
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
        for id in scenes {
            scene_card(
                ui,
                e,
                id,
                color,
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

#[allow(clippy::too_many_arguments)]
fn scene_card(
    ui: &mut egui::Ui,
    e: &Engine,
    id: SceneId,
    bank_color: Color32,
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
    let kind = match (scene.steps.len(), progress) {
        (_, Some(p)) if p.step_count > 1 => tr!(
            "ABLAUF · {step_nr}/{step_count}",
            step_nr = p.step + 1,
            step_count = p.step_count
        ),
        (n, _) if n > 1 => tr!("ABLAUF · {n} SCHRITTE", n = n),
        _ => t("STATISCH").to_string(),
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
    }
    .show(ui);

    // Umbenennen direkt in der Karte (über dem Namen)
    if let Some((_, text)) = rename.as_mut().filter(|(r, _)| *r == id) {
        let field = egui::Rect::from_min_size(
            resp.main_rect.min + Vec2::new(4.0, 4.0),
            Vec2::new(resp.main_rect.width() - 8.0, 22.0),
        );
        let r = ui.put(field, text_field(text).desired_width(field.width()));
        if std::mem::take(rename_focus) {
            r.request_focus();
            // ganzen Namen markieren, damit man sofort tippen kann
            if let Some(mut st) = egui::TextEdit::load_state(ui.ctx(), r.id) {
                let n = text.chars().count();
                st.cursor.set_char_range(Some(egui::text::CCursorRange::two(
                    egui::text::CCursor::new(0),
                    egui::text::CCursor::new(n),
                )));
                st.store(ui.ctx(), r.id);
            }
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

    // Streifen: auswählen (Strg/Umschalt: mehrere), Hauptbereich: starten/stoppen
    if resp.select.clicked() && !locked {
        let mods = ui.input(|i| i.modifiers);
        actions.push(if mods.command || mods.shift {
            Action::Mark(id)
        } else {
            Action::Select(id)
        });
    }
    if resp.play.clicked() {
        actions.push(Action::Toggle(id));
    }
    if locked {
        return;
    }
    // Menü wirkt auf alle markierten Szenen, wenn diese dazugehört
    let targets: Vec<SceneId> = if is_marked {
        marked.iter().copied().collect()
    } else {
        vec![id]
    };
    let current = scene.color;
    menu::context(&resp.whole(), |ui| {
        menu::begin(ui);
        let title = if targets.len() > 1 {
            tr!("{n} Szenen", n = targets.len())
        } else {
            scene.name.clone()
        };
        menu::title(ui, &title);
        let single = targets.len() == 1;
        if menu::item(
            ui,
            MenuItem::new(t("Umbenennen"))
                .icon(Icon::Edit)
                .enabled(single),
        ) {
            actions.push(Action::RenameScene(id));
        }
        if menu::item(ui, MenuItem::new(t("Duplizieren")).icon(Icon::Copy)) {
            actions.extend(targets.iter().map(|s| Action::DuplicateScene(*s)));
        }
        menu::item(
            ui,
            MenuItem::new(t("SuperSzene erstellen"))
                .icon(Icon::Music)
                .enabled(false)
                .tooltip(t("Kommt in einer späteren Version")),
        );
        menu::separator(ui);
        ui.horizontal(|ui| {
            ui.add_space(theme::SP_S);
            ui.label(
                egui::RichText::new(t("Szenenfarbe"))
                    .small()
                    .color(theme::text_dim()),
            );
        });
        if let Some(c) = color_swatches(
            ui,
            ui.make_persistent_id(("scene_color", id.0)),
            current,
            t("Bankfarbe übernehmen"),
        ) {
            actions.push(Action::SceneColor(targets.clone(), c));
        }
        menu::separator(ui);
        if menu::item(
            ui,
            MenuItem::new(t("Löschen"))
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
            Action::Toggle(id) => e.toggle(id),
            Action::Pause(id) => e.toggle_pause(id),
            Action::Jump(id, d) => e.jump_step(id, d),
            Action::NewScene(zone) => {
                // Werte aus dem Programmer übernehmen, sonst leere Szene
                let values = programmer_values(e);
                let name = tr!("Szene {n}", n = e.show().scenes.len() + 1);
                let id = e
                    .show_mut()
                    .add_scene(name.clone(), Some(zone), vec![Step::new(values)]);
                st.append_target = Some(id);
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
                    .duplicate_scene(id, tr!("{name} (Kopie)", name = name));
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
                st.selected_scenes.clear();
                st.selected_scenes.insert(id);
            }
            Action::Mark(id) => {
                if !st.selected_scenes.remove(&id) {
                    st.selected_scenes.insert(id);
                }
            }
            Action::RenameBank(id, name) => {
                if let Some(z) = e.show_mut().zone_mut(id) {
                    z.name = name;
                }
            }
            Action::Bank(b) => apply_bank(e, st, rename, b),
            Action::OpenBanksWindow => st.zones_open = true,
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
            // gleich umbenennen lassen
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
                .duplicate_zone(id, tr!("{name} (Kopie)", name = name));
        }
        BankAction::Hide(id) => {
            if let Some(z) = e.show_mut().zone_mut(id) {
                z.hidden = true;
                st.info(tr!(
                    "„{name}“ ausgeblendet – Rechtsklick auf „Alle“ blendet sie wieder ein",
                    name = z.name
                ));
            }
        }
        BankAction::Show(id) => {
            if let Some(z) = e.show_mut().zone_mut(id) {
                z.hidden = false;
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
