//! Steuern: Seitenleiste mit Zonen und Szenen-Spalten mit Karten.

use dmxcraft_core::{Engine, SceneId, Step, ZoneId};
use eframe::egui::{self, Color32, CornerRadius, Sense, Stroke, StrokeKind, Vec2};

use crate::app::{SetupTab, UiState};
use crate::theme;
use crate::views::programmer::programmer_values;

const COLUMN_W: f32 = 168.0;
const CARD_H: f32 = 46.0;

/// Welche Zonen-Spalten angezeigt werden.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum PageFilter {
    #[default]
    All,
    Zone(ZoneId),
    Free,
}

enum Action {
    Toggle(SceneId),
    Pause(SceneId),
    Jump(SceneId, i32),
    NewScene(Option<ZoneId>),
    Edit(SceneId),
    Delete(SceneId),
}

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    page_bar(ui, e, st);
    ui.add_space(4.0);

    let mut columns: Vec<(Option<ZoneId>, String)> = e
        .show()
        .zones
        .iter()
        .map(|z| (Some(z.id), z.name.clone()))
        .collect();
    columns.push((None, "Frei".into()));
    columns.retain(|(z, _)| match st.page {
        PageFilter::All => true,
        PageFilter::Zone(id) => *z == Some(id),
        PageFilter::Free => z.is_none(),
    });

    let mut actions = Vec::new();
    egui::ScrollArea::both()
        .id_salt("scene_grid")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                for (zone, name) in &columns {
                    column(ui, e, *zone, name, &mut actions);
                    ui.add_space(6.0);
                }
            });
        });

    for a in actions {
        match a {
            Action::Toggle(id) => e.toggle(id),
            Action::Pause(id) => e.toggle_pause(id),
            Action::Jump(id, d) => e.jump_step(id, d),
            Action::NewScene(zone) => new_scene(e, st, zone),
            Action::Edit(id) => {
                st.edit_scene = Some(id);
                st.mode = crate::app::Mode::Setup;
                st.setup_tab = SetupTab::Scenes;
            }
            Action::Delete(id) => {
                e.show_mut().remove_scene(id);
                e.sync();
                if st.edit_scene == Some(id) {
                    st.edit_scene = None;
                }
            }
        }
    }
}

fn page_bar(ui: &mut egui::Ui, e: &Engine, st: &mut UiState) {
    ui.horizontal_wrapped(|ui| {
        let tab = |ui: &mut egui::Ui, selected: bool, text: &str, color: Color32| {
            let rt = egui::RichText::new(text).color(if selected { Color32::BLACK } else { color });
            let b = egui::Button::new(rt)
                .fill(if selected { color } else { theme::CARD })
                .stroke(Stroke::new(1.0, color.gamma_multiply(0.5)));
            ui.add(b).clicked()
        };
        if tab(ui, st.page == PageFilter::All, "Alle", Color32::WHITE) {
            st.page = PageFilter::All;
        }
        for z in &e.show().zones {
            let c = theme::zone_color(e.show(), Some(z.id));
            let active = e.active_in_zone(z.id).is_some();
            let label = if active {
                format!("{} *", z.name)
            } else {
                z.name.clone()
            };
            if tab(ui, st.page == PageFilter::Zone(z.id), &label, c) {
                st.page = PageFilter::Zone(z.id);
            }
        }
        if tab(ui, st.page == PageFilter::Free, "Frei", theme::TEXT_WEAK) {
            st.page = PageFilter::Free;
        }
    });
}

fn column(
    ui: &mut egui::Ui,
    e: &Engine,
    zone: Option<ZoneId>,
    name: &str,
    actions: &mut Vec<Action>,
) {
    let color = theme::zone_color(e.show(), zone);
    let scenes: Vec<SceneId> = e
        .show()
        .scenes
        .iter()
        .filter(|s| s.zone == zone)
        .map(|s| s.id)
        .collect();
    // Ziel der Transport-Tasten: aktive Szene der Zone bzw. zuletzt gestartete freie Szene
    let target = match zone {
        Some(z) => e.active_in_zone(z),
        None => e
            .active_scenes()
            .into_iter()
            .rev()
            .find(|id| e.show().scene(*id).is_some_and(|s| s.zone.is_none())),
    };

    ui.vertical(|ui| {
        ui.set_width(COLUMN_W);
        // Kopf mit Akzentlinie
        let (rect, _) = ui.allocate_exact_size(Vec2::new(COLUMN_W, 3.0), Sense::hover());
        ui.painter().rect_filled(rect, 1.0, color);
        egui::Frame::new()
            .fill(theme::CARD)
            .corner_radius(CornerRadius::same(4))
            .inner_margin(6)
            .show(ui, |ui| {
                ui.set_width(COLUMN_W - 12.0);
                ui.label(egui::RichText::new(name).strong().color(color));
                ui.horizontal(|ui| {
                    let enabled = target.is_some();
                    let paused = target.and_then(|t| e.progress(t)).is_some_and(|p| p.paused);
                    if ui
                        .add_enabled(
                            enabled,
                            egui::Button::new(if paused { ">" } else { "||" }).small(),
                        )
                        .on_hover_text("Pause / Weiter")
                        .clicked()
                    {
                        actions.push(Action::Pause(target.unwrap_or(SceneId(0))));
                    }
                    if ui
                        .add_enabled(enabled, egui::Button::new("|<").small())
                        .on_hover_text("Vorheriger Schritt")
                        .clicked()
                    {
                        actions.push(Action::Jump(target.unwrap_or(SceneId(0)), -1));
                    }
                    if ui
                        .add_enabled(enabled, egui::Button::new(">|").small())
                        .on_hover_text("Nächster Schritt")
                        .clicked()
                    {
                        actions.push(Action::Jump(target.unwrap_or(SceneId(0)), 1));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .small_button("+")
                            .on_hover_text("Neue Szene aus dem Programmer")
                            .clicked()
                        {
                            actions.push(Action::NewScene(zone));
                        }
                    });
                });
            });
        ui.add_space(4.0);
        for id in scenes {
            scene_card(ui, e, id, color, actions);
            ui.add_space(4.0);
        }
    });
}

fn scene_card(
    ui: &mut egui::Ui,
    e: &Engine,
    id: SceneId,
    color: Color32,
    actions: &mut Vec<Action>,
) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let progress = e.progress(id);
    let active = progress.is_some();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(COLUMN_W, CARD_H), Sense::click());
    let p = ui.painter();
    let fill = if resp.hovered() {
        theme::CARD_HOVER
    } else {
        theme::CARD
    };
    p.rect_filled(rect, 4.0, fill);
    if active {
        p.rect_stroke(rect, 4.0, Stroke::new(1.5, color), StrokeKind::Inside);
    }
    // Fortschrittsleiste rechts
    let bar = egui::Rect::from_min_max(
        egui::pos2(rect.right() - 8.0, rect.top() + 4.0),
        egui::pos2(rect.right() - 4.0, rect.bottom() - 4.0),
    );
    p.rect_filled(bar, 2.0, theme::BG);
    if let Some(pr) = progress {
        let frac =
            ((pr.step as f32 + pr.step_fraction) / pr.step_count.max(1) as f32).clamp(0.0, 1.0);
        let fill_rect = egui::Rect::from_min_max(
            egui::pos2(bar.left(), bar.bottom() - bar.height() * frac.max(0.05)),
            bar.max,
        );
        p.rect_filled(
            fill_rect,
            2.0,
            if pr.paused { theme::TEXT_WEAK } else { color },
        );
    }
    let name_color = if active { color } else { Color32::WHITE };
    let text_rect = rect.shrink2(Vec2::new(8.0, 6.0));
    p.text(
        text_rect.left_top(),
        egui::Align2::LEFT_TOP,
        truncate(&scene.name, 20),
        egui::FontId::proportional(13.5),
        name_color,
    );
    let kind = match (scene.steps.len(), progress) {
        (_, Some(pr)) if pr.step_count > 1 => format!("ABLAUF · {}/{}", pr.step + 1, pr.step_count),
        (n, _) if n > 1 => format!("ABLAUF · {n} Schritte"),
        _ => "STATISCH".to_string(),
    };
    p.text(
        text_rect.left_bottom(),
        egui::Align2::LEFT_BOTTOM,
        kind,
        egui::FontId::proportional(10.0),
        theme::TEXT_WEAK,
    );

    if resp.clicked() {
        actions.push(Action::Toggle(id));
    }
    resp.context_menu(|ui| {
        if ui.button("Bearbeiten").clicked() {
            actions.push(Action::Edit(id));
            ui.close();
        }
        if ui.button("Löschen").clicked() {
            actions.push(Action::Delete(id));
            ui.close();
        }
    });
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}

fn new_scene(e: &mut Engine, st: &mut UiState, zone: Option<ZoneId>) {
    let values = programmer_values(e);
    if values.is_empty() {
        st.error("Programmer ist leer – erst Geräte auswählen und Werte setzen");
        return;
    }
    let n = e.show().scenes.len() + 1;
    let name = format!("Szene {n}");
    let id = e
        .show_mut()
        .add_scene(name.clone(), zone, vec![Step::new(values)]);
    st.append_target = Some(id);
    st.info(format!(
        "„{name}“ angelegt (Rechtsklick → Bearbeiten zum Umbenennen)"
    ));
}
