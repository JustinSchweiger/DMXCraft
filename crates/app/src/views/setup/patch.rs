//! Patch: Geräte hinzufügen, umpatchen, entfernen, auswählen.

use dmxcraft_core::{Engine, FixtureId};
use eframe::egui;

use crate::state::AppState as UiState;
use crate::views::{View, ViewCtx};

/// EINRICHTEN → Patch.
#[derive(Default)]
pub struct PatchView;

impl View for PatchView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        patch_ui(ui, cx.engine, cx.state);
    }
}

fn patch_ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    ui.heading("Patch");

    // ---------- Hinzufügen ----------
    egui::CollapsingHeader::new("Geräte hinzufügen")
        .default_open(true)
        .show(ui, |ui| {
            let types: Vec<(String, String, u16)> = e
                .show()
                .fixture_types
                .iter()
                .map(|t| (t.key.clone(), t.display_name(), t.channel_count()))
                .collect();
            let selected_name = types
                .iter()
                .find(|(k, _, _)| *k == st.patch.type_key)
                .map(|(_, n, _)| n.clone())
                .unwrap_or_else(|| "– wählen –".into());
            egui::ComboBox::from_id_salt("patch_type")
                .width(300.0)
                .selected_text(selected_name)
                .show_ui(ui, |ui| {
                    for (key, name, _) in &types {
                        ui.selectable_value(&mut st.patch.type_key, key.clone(), name);
                    }
                });
            if ui
                .button("Gerätebibliothek öffnen")
                .on_hover_text("Über 600 Geräte aus der Open Fixture Library")
                .clicked()
            {
                st.library_open = true;
            }

            let universes = e.show().universes;
            egui::Grid::new("patch_form").num_columns(2).show(ui, |ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut st.patch.name);
                ui.end_row();
                ui.label("Anzahl");
                ui.add(egui::DragValue::new(&mut st.patch.count).range(1..=64));
                ui.end_row();
                ui.label("Universum");
                ui.add(egui::DragValue::new(&mut st.patch.universe).range(1..=universes));
                ui.end_row();
                ui.label("Startadresse");
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut st.patch.address).range(1..=512));
                    if ui.small_button("nächste freie").clicked() {
                        let ch = types
                            .iter()
                            .find(|(k, _, _)| *k == st.patch.type_key)
                            .map_or(1, |t| t.2);
                        match e.show().next_free_address(st.patch.universe - 1, ch) {
                            Some(a) => st.patch.address = a,
                            None => st.error("Kein Platz mehr in diesem Universum"),
                        }
                    }
                });
                ui.end_row();
            });

            if ui.button("Hinzufügen").clicked() {
                add_fixtures(e, st, &types);
            }
        });

    ui.separator();

    // ---------- Liste ----------
    ui.horizontal(|ui| {
        ui.strong(format!("{} Geräte", e.show().fixtures.len()));
        if ui.small_button("alle").clicked() {
            st.selected = e.show().fixtures.iter().map(|f| f.id).collect();
        }
        if ui.small_button("keine").clicked() {
            st.selected.clear();
        }
    });

    let mut remove: Option<FixtureId> = None;
    let mut repatch: Option<(FixtureId, u8, u16)> = None;
    let universes = e.show().universes;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("patch_list")
                .num_columns(5)
                .striped(true)
                .show(ui, |ui| {
                    ui.weak("");
                    ui.weak("Name");
                    ui.weak("Univ.");
                    ui.weak("Adr.");
                    ui.weak("");
                    ui.end_row();
                    for f in &e.show().fixtures {
                        let mut sel = st.selected.contains(&f.id);
                        if ui.checkbox(&mut sel, "").changed() {
                            if sel {
                                st.selected.insert(f.id);
                            } else {
                                st.selected.remove(&f.id);
                            }
                        }
                        let ty_name = e
                            .show()
                            .type_of(f)
                            .map_or_else(|| "? unbekannter Typ".into(), |t| t.display_name());
                        ui.label(&f.name).on_hover_text(ty_name);
                        let mut u = f.universe + 1;
                        let mut a = f.address;
                        let ru = ui.add(egui::DragValue::new(&mut u).range(1..=universes));
                        let ra = ui.add(egui::DragValue::new(&mut a).range(1..=512));
                        if ru.changed() || ra.changed() {
                            repatch = Some((f.id, u - 1, a));
                        }
                        if ui
                            .small_button("Entf.")
                            .on_hover_text("Entfernen")
                            .clicked()
                        {
                            remove = Some(f.id);
                        }
                        ui.end_row();
                    }
                });
        });

    if let Some((id, u, a)) = repatch {
        if let Err(err) = e.show_mut().repatch_fixture(id, u, a) {
            st.error(err.to_string());
        }
    }
    if let Some(id) = remove {
        e.show_mut().remove_fixture(id);
        st.selected.remove(&id);
        e.sync();
    }
}

fn add_fixtures(e: &mut Engine, st: &mut UiState, types: &[(String, String, u16)]) {
    let Some(&(_, _, channels)) = types.iter().find(|(k, _, _)| *k == st.patch.type_key) else {
        st.error("Bitte einen Gerätetyp wählen");
        return;
    };
    let universe = st.patch.universe.saturating_sub(1);
    let count = st.patch.count.max(1);
    let mut address = st.patch.address;
    let mut added = 0;
    for i in 0..count {
        let name = if count == 1 {
            st.patch.name.clone()
        } else {
            format!("{} {}", st.patch.name, i + 1)
        };
        match e
            .show_mut()
            .add_fixture(name, &st.patch.type_key.clone(), universe, address)
        {
            Ok(_) => {
                added += 1;
                address = address.saturating_add(channels);
            }
            Err(err) => {
                st.error(format!("Gerät {}: {err}", i + 1));
                break;
            }
        }
    }
    if added > 0 {
        st.patch.address = address.min(512);
        if added == count {
            st.info(format!("{added} Gerät(e) hinzugefügt"));
        }
    }
}
