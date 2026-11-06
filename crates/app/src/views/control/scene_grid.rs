//! STEUERN → Szenen-Raster: Zonen als Spalten, Szenen als Karten.

use dmxcraft_core::{Engine, SceneId, Step, ZoneId};
use eframe::egui::{self, Color32, Sense, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, SceneCard};
use crate::views::control::programmer::programmer_values;
use crate::views::{View, ViewCtx};

const COLUMN_W: f32 = 176.0;

/// Welche Zonen-Spalten angezeigt werden.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum PageFilter {
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

#[derive(Default)]
pub struct SceneGridView {
    page: PageFilter,
}

impl View for SceneGridView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        ui.horizontal(|ui| {
            self.page_bar(ui, cx.engine);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                playback_controls(ui, cx.engine);
            });
        });
        ui.add_space(theme::SP_XS);

        let mut columns: Vec<(Option<ZoneId>, String)> = cx
            .engine
            .show()
            .zones
            .iter()
            .map(|z| (Some(z.id), z.name.clone()))
            .collect();
        columns.push((None, "Frei".into()));
        columns.retain(|(z, _)| match self.page {
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
                        column(ui, cx.engine, *zone, name, &mut actions);
                        ui.add_space(theme::SP_S);
                    }
                });
            });
        apply(cx.engine, cx.state, actions);
    }
}

impl SceneGridView {
    fn page_bar(&mut self, ui: &mut egui::Ui, e: &Engine) {
        ui.horizontal(|ui| {
            chip(
                ui,
                &mut self.page,
                PageFilter::All,
                "Alle",
                theme::TEXT,
                false,
            );
            for z in &e.show().zones {
                let c = theme::zone_color(e.show(), Some(z.id));
                let live = e.active_in_zone(z.id).is_some();
                chip(ui, &mut self.page, PageFilter::Zone(z.id), &z.name, c, live);
            }
            chip(
                ui,
                &mut self.page,
                PageFilter::Free,
                "Frei",
                theme::TEXT_DIM,
                false,
            );
        });
    }
}

/// Master, Speed, Blackout und Alle aus (rechts neben den Zonen-Chips).
fn playback_controls(ui: &mut egui::Ui, e: &mut Engine) {
    let bo = e.blackout();
    let bo_btn = egui::Button::new(egui::RichText::new("Blackout").strong().color(if bo {
        Color32::WHITE
    } else {
        theme::TEXT
    }))
    .fill(if bo { theme::DANGER } else { theme::RAISED })
    .min_size(Vec2::new(88.0, 28.0));
    if ui.add(bo_btn).on_hover_text("Taste B").clicked() {
        e.set_blackout(!bo);
    }
    if ui
        .button("Alle aus")
        .on_hover_text("Alle Szenen stoppen")
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
        .on_hover_text("Wiedergabegeschwindigkeit")
        .changed()
    {
        e.set_speed(speed);
    }
    ui.label(egui::RichText::new("Speed").color(theme::TEXT_DIM));
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
    ui.label(egui::RichText::new("Master").color(theme::TEXT_DIM));
}

/// Filter-Chip; ein Punkt zeigt an, dass in der Zone etwas läuft.
fn chip(
    ui: &mut egui::Ui,
    page: &mut PageFilter,
    value: PageFilter,
    label: &str,
    color: Color32,
    live: bool,
) {
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
            theme::RAISED_HOVER
        } else {
            theme::RAISED
        },
    );
    p.rect_stroke(
        rect,
        r,
        Stroke::new(1.0, if selected { color } else { theme::BORDER }),
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
        if selected { theme::TEXT } else { color },
    );
    if resp.clicked() {
        *page = value;
    }
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
    let paused = target.and_then(|t| e.progress(t)).is_some_and(|p| p.paused);

    ui.vertical(|ui| {
        ui.set_width(COLUMN_W);
        egui::Frame::new()
            .fill(theme::RAISED)
            .stroke(Stroke::new(1.0, theme::BORDER))
            .corner_radius(theme::R_M)
            .inner_margin(theme::SP_S)
            .show(ui, |ui| {
                ui.set_width(COLUMN_W - 2.0 * theme::SP_S - 2.0);
                let (bar, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 3.0), Sense::hover());
                ui.painter().rect_filled(bar, 1.5, color);
                ui.label(egui::RichText::new(name).strong().color(color));
                ui.horizontal(|ui| {
                    let on = target.is_some();
                    let t = target.unwrap_or(SceneId(0));
                    if ui
                        .add(
                            IconButton::new(if paused { Icon::Play } else { Icon::Pause })
                                .enabled(on)
                                .active(paused),
                        )
                        .on_hover_text("Pause / Weiter")
                        .clicked()
                    {
                        actions.push(Action::Pause(t));
                    }
                    if ui
                        .add(IconButton::new(Icon::Prev).enabled(on))
                        .on_hover_text("Vorheriger Schritt")
                        .clicked()
                    {
                        actions.push(Action::Jump(t, -1));
                    }
                    if ui
                        .add(IconButton::new(Icon::Next).enabled(on))
                        .on_hover_text("Nächster Schritt")
                        .clicked()
                    {
                        actions.push(Action::Jump(t, 1));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(IconButton::new(Icon::Plus))
                            .on_hover_text("Neue Szene aus dem Programmer")
                            .clicked()
                        {
                            actions.push(Action::NewScene(zone));
                        }
                    });
                });
            });
        ui.add_space(theme::SP_XS);
        for id in scenes {
            scene_card(ui, e, id, color, actions);
            ui.add_space(theme::SP_XS);
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
    let kind = match (scene.steps.len(), progress) {
        (_, Some(p)) if p.step_count > 1 => format!("ABLAUF · {}/{}", p.step + 1, p.step_count),
        (n, _) if n > 1 => format!("ABLAUF · {n} SCHRITTE"),
        _ => "STATISCH".to_string(),
    };
    let card = SceneCard {
        name: &scene.name,
        kind,
        color,
        active: progress.is_some(),
        paused: progress.is_some_and(|p| p.paused),
        progress: progress.map_or(0.0, |p| {
            (p.step as f32 + p.step_fraction) / p.step_count.max(1) as f32
        }),
        width: COLUMN_W,
    };
    let resp = ui.add(card);
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

fn apply(e: &mut Engine, st: &mut AppState, actions: Vec<Action>) {
    for a in actions {
        match a {
            Action::Toggle(id) => e.toggle(id),
            Action::Pause(id) => e.toggle_pause(id),
            Action::Jump(id, d) => e.jump_step(id, d),
            Action::NewScene(zone) => {
                let values = programmer_values(e);
                if values.is_empty() {
                    st.error("Programmer ist leer – erst Geräte auswählen und Werte setzen");
                    continue;
                }
                let name = format!("Szene {}", e.show().scenes.len() + 1);
                let id = e
                    .show_mut()
                    .add_scene(name.clone(), zone, vec![Step::new(values)]);
                st.append_target = Some(id);
                st.info(format!(
                    "„{name}“ angelegt (Rechtsklick → Bearbeiten zum Umbenennen)"
                ));
            }
            Action::Edit(id) => st.open_scene_editor(id),
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
