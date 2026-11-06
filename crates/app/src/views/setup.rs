//! Einrichten: Patch, Szenen bearbeiten, Zonen & Gruppen, DMX-Monitor.

use dmxcraft_core::{DmxFrame, Engine, GroupId, ZoneId};
use eframe::egui;

use crate::app::{SetupTab, UiState};
use crate::theme;
use crate::views;

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, frame: &DmxFrame) {
    ui.horizontal(|ui| {
        for (t, label) in [
            (SetupTab::Patch, "Patch"),
            (SetupTab::Scenes, "Szenen"),
            (SetupTab::Zones, "Zonen & Gruppen"),
            (SetupTab::Monitor, "DMX-Monitor"),
        ] {
            ui.selectable_value(&mut st.setup_tab, t, label);
        }
    });
    ui.separator();
    match st.setup_tab {
        SetupTab::Patch => views::patch::ui(ui, e, st),
        SetupTab::Scenes => views::scenes::ui(ui, e, st),
        SetupTab::Zones => zones_and_groups(ui, e, st),
        SetupTab::Monitor => views::monitor::ui(ui, e, st, frame),
    }
}

fn zones_and_groups(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    let mut remove_zone: Option<ZoneId> = None;
    let mut remove_group: Option<GroupId> = None;
    let mut move_zone: Option<(usize, usize)> = None;

    ui.columns(2, |cols| {
        let ui = &mut cols[0];
        ui.heading("Zonen");
        ui.weak("Jede Zone ist eine Spalte unter STEUERN. Szenen derselben Zone schließen sich aus.");
        ui.add_space(6.0);
        let n = e.show().zones.len();
        let zone_ids: Vec<ZoneId> = e.show().zones.iter().map(|z| z.id).collect();
        for (i, id) in zone_ids.iter().enumerate() {
            let fallback = theme::ZONE_PALETTE[i % theme::ZONE_PALETTE.len()];
            let scenes = e.show().scenes.iter().filter(|s| s.zone == Some(*id)).count();
            let Some(z) = e.show_mut().zones.iter_mut().find(|z| z.id == *id) else {
                continue;
            };
            ui.horizontal(|ui| {
                let mut c = z.color.unwrap_or(fallback);
                if ui.color_edit_button_srgb(&mut c).changed() {
                    z.color = Some(c);
                }
                ui.add(egui::TextEdit::singleline(&mut z.name).desired_width(180.0));
                ui.weak(format!("{scenes} Szenen"));
                if i > 0 && ui.small_button("hoch").clicked() {
                    move_zone = Some((i, i - 1));
                }
                if i + 1 < n && ui.small_button("runter").clicked() {
                    move_zone = Some((i, i + 1));
                }
                if ui
                    .small_button("Entf.")
                    .on_hover_text("Zone löschen – Szenen werden „frei“")
                    .clicked()
                {
                    remove_zone = Some(*id);
                }
            });
        }
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut st.new_zone_name);
            if ui.button("Zone anlegen").clicked() && !st.new_zone_name.trim().is_empty() {
                e.show_mut().add_zone(st.new_zone_name.trim().to_string());
            }
        });

        let ui = &mut cols[1];
        ui.heading("Gruppen");
        ui.weak("Gruppen wählen mehrere Geräte mit einem Klick aus (unter STEUERN über dem Bühnenplan).");
        ui.add_space(6.0);
        let names: Vec<(dmxcraft_core::FixtureId, String)> = e
            .show()
            .fixtures
            .iter()
            .map(|f| (f.id, f.name.clone()))
            .collect();
        for g in &mut e.show_mut().groups {
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut g.name).desired_width(160.0));
                let members: Vec<String> = g
                    .fixtures
                    .iter()
                    .filter_map(|id| names.iter().find(|(i, _)| i == id).map(|(_, n)| n.clone()))
                    .collect();
                ui.weak(format!("{} Geräte", members.len()))
                    .on_hover_text(members.join("\n"));
                if ui
                    .small_button("= Auswahl")
                    .on_hover_text("Gruppe auf die aktuelle Auswahl setzen")
                    .clicked()
                {
                    g.fixtures = st.selected.iter().copied().collect();
                }
                if ui.small_button("Entf.").clicked() {
                    remove_group = Some(g.id);
                }
            });
        }
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut st.new_group_name)
                    .hint_text("Name")
                    .desired_width(160.0),
            );
            if ui
                .add_enabled(!st.selected.is_empty(), egui::Button::new("Gruppe aus Auswahl"))
                .clicked()
            {
                let name = if st.new_group_name.trim().is_empty() {
                    format!("Gruppe {}", e.show().groups.len() + 1)
                } else {
                    st.new_group_name.trim().to_string()
                };
                e.show_mut()
                    .add_group(name, st.selected.iter().copied().collect());
                st.new_group_name.clear();
            }
        });
    });

    if let Some((a, b)) = move_zone {
        e.show_mut().zones.swap(a, b);
    }
    if let Some(z) = remove_zone {
        e.show_mut().remove_zone(z);
    }
    if let Some(g) = remove_group {
        e.show_mut().remove_group(g);
    }
}
