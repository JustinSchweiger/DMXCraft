//! STEUERN → Bühnenplan: Gruppen und Geräteauswahl (Anordnen unter EINRICHTEN).
//!
//! Über dem Plan stehen die Gruppen als Pillen. Klick wählt genau die Gruppe
//! (nochmal Klick hebt die Auswahl auf), Strg/Umschalt+Klick fügt sie hinzu
//! bzw. nimmt sie heraus. Doppelklick benennt um, Rechtsklick öffnet das
//! Gruppenmenü, **+** legt eine Gruppe aus der aktuellen Auswahl an.

use crate::i18n::t;
use std::collections::BTreeSet;

use dmxcraft_core::{Engine, FixtureId, GroupId};
use eframe::egui::{self, Color32, Sense, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::dialog::select_all;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{text_field, CanvasMode, Icon, IconButton, StageCanvas, StageInteraction};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct StageView {
    interaction: StageInteraction,
    /// Laufende Umbenennung einer Gruppe.
    rename: Option<(GroupId, String)>,
    rename_focus: bool,
}

enum GroupAction {
    Select(BTreeSet<FixtureId>),
    New,
    Rename(GroupId),
    SetName(GroupId, String),
    AddSelection(GroupId),
    Delete(GroupId),
}

impl View for StageView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let mut actions = Vec::new();
        self.group_bar(ui, cx.engine, cx.state, &mut actions);
        self.apply(cx.engine, cx.state, actions);
        ui.add_space(theme::SP_S);
        let show = cx.engine.show();
        let canvas = StageCanvas {
            show,
            frame: cx.frame,
            mode: CanvasMode::Select,
            snap: None,
            show_labels: false,
            show_grid: false,
            auto_fit: true,
        };
        let mut action = None;
        let clip = cx.state.layout_clipboard.is_some();
        let locked = cx.state.locked;
        let area = ui.available_rect_before_wrap();
        let blind = cx.state.programmer_mode == crate::state::ProgrammerMode::Blind;
        // Im Auswahlmodus entstehen keine Verschiebungen.
        let _ = canvas.show(
            ui,
            &mut self.interaction,
            &mut cx.state.selected,
            &mut |ui, target, sel| {
                if locked {
                    ui.label(t("Oberfläche ist gesperrt"));
                    return;
                }
                if let Some(a) = fixture_menu::menu(ui, show, sel, target, MenuPlace::Stage, clip) {
                    action = Some(a);
                }
            },
        );
        if blind {
            // Hinweis: Bühne zeigt die Bearbeitung, nicht den Ausgang
            ui.painter().text(
                area.right_bottom() - Vec2::new(14.0, 10.0),
                egui::Align2::RIGHT_BOTTOM,
                t("BLIND"),
                egui::FontId::new(22.0, egui::FontFamily::Name(theme::HEADING_FAMILY.into())),
                theme::DANGER,
            );
        }
        if let Some(a) = action {
            fixture_menu::apply(cx.engine, cx.state, a);
        }
    }
}

impl StageView {
    fn group_bar(
        &mut self,
        ui: &mut egui::Ui,
        e: &Engine,
        st: &AppState,
        actions: &mut Vec<GroupAction>,
    ) {
        let show = e.show();
        let locked = st.locked;
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            ui.label(theme::heading_text(t("Gruppen")));
            ui.label(egui::RichText::new("|").color(theme::text_faint()));
            let all: BTreeSet<FixtureId> = show.fixtures.iter().map(|f| f.id).collect();
            if let Some(a) = group_pill(ui, t("Alle"), &all, &st.selected) {
                actions.push(a);
            }
            for g in &show.groups {
                let set: BTreeSet<FixtureId> = g.fixtures.iter().copied().collect();
                if let Some((_, text)) = self.rename.as_mut().filter(|(id, _)| *id == g.id) {
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
                        self.rename = None;
                    } else if enter || r.lost_focus() {
                        let name = text.trim().to_string();
                        self.rename = None;
                        if !name.is_empty() {
                            actions.push(GroupAction::SetName(g.id, name));
                        }
                    }
                    continue;
                }
                let pill = PillResult::show(ui, &g.name, &set, &st.selected);
                if let Some(a) = pill.action {
                    actions.push(a);
                }
                if locked {
                    continue;
                }
                if pill.resp.double_clicked() {
                    actions.push(GroupAction::Rename(g.id));
                }
                let some = !st.selected.is_empty();
                let id = g.id;
                menu::context(&pill.resp, |ui| {
                    menu::begin(ui);
                    menu::title(ui, &g.name);
                    if menu::item(ui, MenuItem::new(t("Umbenennen")).icon(Icon::Edit)) {
                        actions.push(GroupAction::Rename(id));
                    }
                    if menu::item(
                        ui,
                        MenuItem::new(t("Ausgewählte Geräte hinzufügen"))
                            .icon(Icon::Plus)
                            .enabled(some),
                    ) {
                        actions.push(GroupAction::AddSelection(id));
                    }
                    menu::separator(ui);
                    if menu::item(ui, MenuItem::new(t("Löschen")).icon(Icon::Trash).danger()) {
                        actions.push(GroupAction::Delete(id));
                    }
                });
            }
            if !locked {
                let some = !st.selected.is_empty();
                let r = ui
                    .add_enabled(some, IconButton::new(Icon::Plus).flat())
                    .on_hover_text(t("Neue Gruppe aus Auswahl"))
                    .on_disabled_hover_text(t("Erst Geräte auswählen"));
                if r.clicked() {
                    actions.push(GroupAction::New);
                }
            }
            if show.groups.is_empty() {
                ui.label(
                    egui::RichText::new(t("Geräte auswählen und mit + eine Gruppe anlegen"))
                        .color(theme::text_faint()),
                );
            }
        });
    }

    fn apply(&mut self, e: &mut Engine, st: &mut AppState, actions: Vec<GroupAction>) {
        for a in actions {
            match a {
                GroupAction::Select(set) => st.selected = set,
                GroupAction::New => {
                    let name = tr!("Gruppe {n}", n = e.show().groups.len() + 1);
                    let ids = st.selected.iter().copied().collect();
                    let id = e.show_mut().add_group(name.clone(), ids);
                    self.rename = Some((id, name));
                    self.rename_focus = true;
                }
                GroupAction::Rename(id) => {
                    if let Some(g) = e.show().groups.iter().find(|g| g.id == id) {
                        self.rename = Some((id, g.name.clone()));
                        self.rename_focus = true;
                    }
                }
                GroupAction::SetName(id, name) => {
                    if let Some(g) = e.show_mut().groups.iter_mut().find(|g| g.id == id) {
                        g.name = name;
                    }
                }
                GroupAction::AddSelection(id) => {
                    if let Some(g) = e.show_mut().groups.iter_mut().find(|g| g.id == id) {
                        for f in &st.selected {
                            if !g.fixtures.contains(f) {
                                g.fixtures.push(*f);
                            }
                        }
                    }
                }
                GroupAction::Delete(id) => e.show_mut().remove_group(id),
            }
        }
    }
}

fn group_pill(
    ui: &mut egui::Ui,
    label: &str,
    set: &BTreeSet<FixtureId>,
    selected: &BTreeSet<FixtureId>,
) -> Option<GroupAction> {
    PillResult::show(ui, label, set, selected).action
}

struct PillResult {
    resp: egui::Response,
    action: Option<GroupAction>,
}

impl PillResult {
    /// Gruppen-Pille: Name und Anzahl der Geräte. Ganz ausgewählt = gefüllt,
    /// teilweise ausgewählt = farbiger Rand.
    fn show(
        ui: &mut egui::Ui,
        label: &str,
        set: &BTreeSet<FixtureId>,
        selected: &BTreeSet<FixtureId>,
    ) -> Self {
        let hit = set.iter().filter(|f| selected.contains(f)).count();
        let full = !set.is_empty() && hit == set.len();
        let exact = full && selected.len() == set.len();
        let partial = hit > 0 && !full;

        let font = theme::body();
        let count = set.len().to_string();
        let name_w = ui
            .painter()
            .layout_no_wrap(label.to_string(), font.clone(), Color32::WHITE)
            .size()
            .x;
        let count_w = ui
            .painter()
            .layout_no_wrap(count.clone(), theme::caption(), Color32::WHITE)
            .size()
            .x
            .max(10.0)
            + 10.0;
        let h = 28.0;
        let w = theme::SP_M + name_w + theme::SP_S + count_w + 4.0;
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());
        let p = ui.painter();
        let r = h * 0.5;
        let accent = theme::accent();
        let fill = if full {
            theme::accent_dim()
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::raised()
        };
        p.rect_filled(rect, r, fill);
        let stroke = if full {
            Stroke::new(1.0, accent)
        } else if partial {
            Stroke::new(1.0, accent.gamma_multiply(0.6))
        } else {
            Stroke::new(1.0, theme::border())
        };
        p.rect_stroke(rect, r, stroke, StrokeKind::Inside);
        p.text(
            egui::pos2(rect.left() + theme::SP_M, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            font,
            if full || partial {
                theme::text()
            } else {
                theme::text_dim()
            },
        );
        // Anzahl als kleines Abzeichen rechts
        let badge = egui::Rect::from_center_size(
            egui::pos2(rect.right() - 4.0 - count_w * 0.5, rect.center().y),
            Vec2::new(count_w, 18.0),
        );
        p.rect_filled(
            badge,
            9.0,
            if full {
                accent.gamma_multiply(0.35)
            } else {
                theme::bg()
            },
        );
        p.text(
            badge.center(),
            egui::Align2::CENTER_CENTER,
            if partial {
                format!("{hit}/{count}")
            } else {
                count
            },
            theme::caption(),
            if full {
                theme::text()
            } else {
                theme::text_faint()
            },
        );

        let mut action = None;
        if resp.clicked() {
            let mods = ui.input(|i| i.modifiers);
            action = Some(GroupAction::Select(if mods.command || mods.shift {
                // hinzufügen bzw. herausnehmen
                if full {
                    selected.difference(set).copied().collect()
                } else {
                    selected.union(set).copied().collect()
                }
            } else if exact {
                BTreeSet::new()
            } else {
                set.clone()
            }));
        }
        Self { resp, action }
    }
}
