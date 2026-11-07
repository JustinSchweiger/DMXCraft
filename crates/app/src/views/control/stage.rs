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
        // Gruppenleiste links, rechts der FX-Knopf
        let full = ui.available_rect_before_wrap();
        const FX_W: f32 = 40.0;
        let mut bar = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(full.with_max_x(full.right() - FX_W - theme::SP_S))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        self.group_bar(&mut bar, cx.engine, cx.state, &mut actions);
        let bar_h = bar.min_rect().height().max(28.0);
        let fx_rect = egui::Rect::from_min_size(
            egui::pos2(full.right() - 28.0, full.top()),
            Vec2::new(28.0, 28.0),
        );
        fx_button(ui, fx_rect, cx.engine, cx.state);
        ui.allocate_exact_size(Vec2::new(full.width(), bar_h), Sense::hover());
        self.apply(cx.engine, cx.state, actions);
        ui.add_space(theme::SP_S);
        let show = cx.engine.show();
        let order = cx.state.selection_order.clone();
        // angeklicktes Mapping: Bild unter den Geräten, Rahmen darüber
        let mapping = mapping_target(cx.engine, cx.state);
        let cycles = mapping
            .as_ref()
            .map_or(0.0, |(sc, _, _)| cx.engine.fx_cycles(*sc));
        let underlay = mapping.as_ref().map(|(_, _, m)| {
            let m = m.clone();
            move |p: &egui::Painter, cam: crate::ui::widgets::stage_canvas::Camera| {
                paint_mapping(p, cam, &m, cycles);
            }
        });
        let canvas = StageCanvas {
            show,
            frame: cx.frame,
            mode: CanvasMode::Select,
            snap: None,
            show_grid: false,
            auto_fit: true,
            order: &order,
            underlay: underlay
                .as_ref()
                .map(|f| f as &dyn Fn(&egui::Painter, crate::ui::widgets::stage_canvas::Camera)),
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
        if let (Some((scene, fx, mut m)), Some((rect, cam))) = (mapping, self.interaction.view) {
            if !locked && mapping_handles(ui, rect, cam, &mut m) {
                if let Some(f) = cx
                    .engine
                    .show_mut()
                    .scene_mut(scene)
                    .and_then(|s| s.effects.iter_mut().find(|f| f.id == fx))
                {
                    f.mapping = Some(m);
                }
            }
            ui.ctx().request_repaint();
        }
    }
}

/// Angeklicktes MAPPINGS/COLOR MAPPINGS (Szene, Effekt, Einstellungen).
fn mapping_target(
    e: &Engine,
    st: &AppState,
) -> Option<(
    dmxcraft_core::SceneId,
    u32,
    dmxcraft_core::fx::mapping::MappingFx,
)> {
    let (scene, fx) = st.fx_target?;
    let f = e.show().scene(scene)?.effects.iter().find(|f| f.id == fx)?;
    Some((scene, fx, f.mapping.clone()?))
}

/// Bild des Mappings als weich eingefärbtes Gitter im Rechteck.
fn paint_mapping(
    p: &egui::Painter,
    cam: crate::ui::widgets::stage_canvas::Camera,
    m: &dmxcraft_core::fx::mapping::MappingFx,
    cycles: f64,
) {
    const N: u32 = 28;
    let mut mesh = egui::epaint::Mesh::default();
    for yi in 0..=N {
        for xi in 0..=N {
            let l = [
                xi as f32 / N as f32 * 2.0 - 1.0,
                yi as f32 / N as f32 * 2.0 - 1.0,
            ];
            let c = m.color_at(l, cycles);
            let col = Color32::from_rgba_unmultiplied(
                (c[0] * 255.0) as u8,
                (c[1] * 255.0) as u8,
                (c[2] * 255.0) as u8,
                200,
            );
            mesh.colored_vertex(cam.to_screen(m.to_plan(l)), col);
        }
    }
    for yi in 0..N {
        for xi in 0..N {
            let i = yi * (N + 1) + xi;
            mesh.add_triangle(i, i + 1, i + N + 1);
            mesh.add_triangle(i + 1, i + N + 2, i + N + 1);
        }
    }
    p.add(egui::Shape::mesh(mesh));
}

/// Rahmen des Mappings auf der Bühne: Fläche ziehen verschiebt, Ecken
/// skalieren, der Griff oben dreht. `true`, wenn sich etwas geändert hat.
fn mapping_handles(
    ui: &mut egui::Ui,
    area: egui::Rect,
    cam: crate::ui::widgets::stage_canvas::Camera,
    m: &mut dmxcraft_core::fx::mapping::MappingFx,
) -> bool {
    let before = m.clone();
    let id = egui::Id::unique("mapping_handles");
    let p = ui.painter_at(area);
    let corners =
        [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]].map(|c| cam.to_screen(m.to_plan(c)));
    for i in 0..4 {
        p.add(egui::Shape::dashed_line(
            &[corners[i], corners[(i + 1) % 4]],
            Stroke::new(1.5, Color32::WHITE),
            5.0,
            4.0,
        ));
    }
    // Fläche: verschieben (als Polygon-Hülle reicht das umschließende Rechteck nicht
    // bei Drehung – Treffer per lokaler Lage)
    let body = egui::Rect::from_points(&corners);
    let r = ui.interact(body, id.with("body"), Sense::drag());
    let inside = r
        .interact_pointer_pos()
        .or(r.hover_pos())
        .is_some_and(|q| m.local(cam.to_world(q)).is_some());
    if r.dragged() && inside {
        let d = r.drag_delta() / cam.scale();
        m.center = [m.center[0] + d.x, m.center[1] + d.y];
    }
    if inside && (r.hovered() || r.dragged()) {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    // Ecken: Größe
    for (i, c) in corners.iter().enumerate() {
        let r = ui.interact(
            egui::Rect::from_center_size(*c, Vec2::splat(14.0)),
            id.with(("corner", i)),
            Sense::drag(),
        );
        if r.dragged() {
            if let Some(q) = r.interact_pointer_pos() {
                let w = cam.to_world(q);
                let (s, co) = m.rotation.to_radians().sin_cos();
                let (dx, dy) = (w[0] - m.center[0], w[1] - m.center[1]);
                m.size = [
                    (dx * co + dy * s).abs().max(0.01),
                    (-dx * s + dy * co).abs().max(0.01),
                ];
            }
        }
        let hot = r.hovered() || r.dragged();
        p.rect_filled(
            egui::Rect::from_center_size(*c, Vec2::splat(8.0)),
            1.0,
            if hot { theme::accent() } else { Color32::WHITE },
        );
    }
    // Drehgriff über der oberen Kante
    let top = cam.to_screen(m.to_plan([0.0, -1.0]));
    let dir = (top - cam.to_screen(m.center)).normalized();
    // außerhalb der Fläche: Griff nach innen setzen
    let knob = if area.shrink(8.0).contains(top + dir * 22.0) {
        top + dir * 22.0
    } else {
        top - dir * 22.0
    };
    p.line_segment([top, knob], Stroke::new(1.0, Color32::WHITE));
    let r = ui.interact(
        egui::Rect::from_center_size(knob, Vec2::splat(16.0)),
        id.with("rotate"),
        Sense::drag(),
    );
    if r.dragged() {
        if let Some(q) = r.interact_pointer_pos() {
            let w = cam.to_world(q);
            let a = (w[1] - m.center[1]).atan2(w[0] - m.center[0]).to_degrees() + 90.0;
            m.rotation = (a / 5.0).round() * 5.0;
        }
    }
    let hot = r.hovered() || r.dragged();
    p.circle_filled(
        knob,
        5.0,
        if hot { theme::accent() } else { Color32::WHITE },
    );
    *m != before
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

/// FX-Knopf: übernimmt die aktuelle Geräteauswahl in den zuletzt
/// angeklickten Effekt. Grau, solange die Auswahl den Geräten des Effekts
/// entspricht; rot, wenn sie davon abweicht.
fn fx_button(ui: &mut egui::Ui, rect: egui::Rect, e: &mut Engine, st: &mut AppState) {
    let target = st.fx_target.filter(|(scene, fx)| {
        e.show()
            .scene(*scene)
            .is_some_and(|s| s.effects.iter().any(|f| f.id == *fx))
    });
    let current: Option<Vec<FixtureId>> = target.and_then(|(scene, fx)| {
        e.show()
            .scene(scene)?
            .effects
            .iter()
            .find(|f| f.id == fx)
            .map(|f| f.fixtures.clone())
    });
    // abweichende Geräte oder andere Reihenfolge
    let pending = !st.locked
        && !st.selected.is_empty()
        && current.as_ref().is_some_and(|c| *c != st.selection_order);
    // rot hinterlegt, sobald sich die Auswahl vom Effekt unterscheidet
    let btn = IconButton::new(Icon::Fx)
        .size(28.0)
        .enabled(pending)
        .active(pending)
        .tint(theme::DANGER);
    let r = ui
        .place(rect, btn)
        .on_hover_text(t("Effekt mit der aktuellen Auswahl aktualisieren"))
        .on_disabled_hover_text(t("Effekt anklicken, dann Geräte ändern"));
    if r.clicked() && pending {
        if let Some((scene, fx)) = target {
            let ids = st.selection_order.clone();
            if let Some(f) = e
                .show_mut()
                .scene_mut(scene)
                .and_then(|s| s.effects.iter_mut().find(|f| f.id == fx))
            {
                f.fixtures = ids;
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
