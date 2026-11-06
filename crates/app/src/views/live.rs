//! Live-Ansicht: Szenen-Buttons gruppiert nach Zonen.

use dmxcraft_core::{Engine, SceneId, ZoneId};
use eframe::egui;

use crate::app::UiState;

const BUTTON: egui::Vec2 = egui::vec2(130.0, 56.0);

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    let zones: Vec<(Option<ZoneId>, String)> = e
        .show()
        .zones
        .iter()
        .map(|z| (Some(z.id), z.name.clone()))
        .chain(std::iter::once((
            None,
            "Ohne Zone (frei kombinierbar)".to_string(),
        )))
        .collect();

    let mut toggle: Option<SceneId> = None;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (zone, name) in &zones {
                let scenes: Vec<(SceneId, String)> = e
                    .show()
                    .scenes
                    .iter()
                    .filter(|s| s.zone == *zone)
                    .map(|s| (s.id, s.name.clone()))
                    .collect();
                if scenes.is_empty() && zone.is_none() {
                    continue;
                }
                ui.add_space(6.0);
                ui.strong(name);
                ui.horizontal_wrapped(|ui| {
                    if scenes.is_empty() {
                        ui.weak("Keine Szenen in dieser Zone");
                    }
                    for (id, sname) in scenes {
                        let active = e.is_active(id);
                        ui.vertical(|ui| {
                            let btn = egui::Button::new(egui::RichText::new(&sname).strong())
                                .min_size(BUTTON)
                                .fill(if active {
                                    egui::Color32::from_rgb(40, 120, 70)
                                } else {
                                    ui.visuals().widgets.inactive.weak_bg_fill
                                });
                            if ui.add(btn).clicked() {
                                toggle = Some(id);
                            }
                            let (frac, label) = match e.progress(id) {
                                Some(p) => (
                                    (p.step as f32 + p.step_fraction) / p.step_count as f32,
                                    format!("Schritt {}/{}", p.step + 1, p.step_count),
                                ),
                                None => (0.0, "aus".to_string()),
                            };
                            ui.add(
                                egui::ProgressBar::new(frac)
                                    .desired_width(BUTTON.x)
                                    .desired_height(6.0),
                            );
                            ui.weak(label);
                        });
                    }
                });
            }

            ui.add_space(12.0);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Neue Zone:");
                ui.text_edit_singleline(&mut st.new_zone_name);
                if ui.button("Anlegen").clicked() && !st.new_zone_name.trim().is_empty() {
                    e.show_mut().add_zone(st.new_zone_name.trim().to_string());
                }
            });
            ui.weak(
                "Szenen derselben Zone schließen sich aus. Neue Szenen legst du im \
                 Programmer an.",
            );
        });

    if let Some(id) = toggle {
        e.toggle(id);
    }
}
