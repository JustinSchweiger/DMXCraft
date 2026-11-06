//! Szenen bearbeiten: Name, Zone, Wiedergabe, Schritte mit Zeiten.

use dmxcraft_core::{Engine, LoopMode, SceneId};
use eframe::egui;

use crate::app::UiState;
use crate::views::programmer::{programmer_values, zone_combo};

enum StepAction {
    Load(usize),
    Overwrite(usize),
    Delete(usize),
    Up(usize),
    Down(usize),
}

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    egui::Panel::left("scene_list")
        .default_size(200.0)
        .show(ui, |ui| {
            ui.strong("Szenen");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for s in &e.show().scenes {
                    let active = e.is_active(s.id);
                    let label = if active {
                        format!("{}  (aktiv)", s.name)
                    } else {
                        s.name.clone()
                    };
                    ui.selectable_value(&mut st.edit_scene, Some(s.id), label);
                }
            });
        });

    let Some(id) = st.edit_scene.filter(|id| e.show().scene(*id).is_some()) else {
        ui.label("Wähle links eine Szene zum Bearbeiten.");
        return;
    };

    scene_header(ui, e, st, id);
    ui.separator();

    let mut action: Option<StepAction> = None;
    let mut changed_times = false;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let Some(scene) = e.show_mut().scene_mut(id) else {
                return;
            };
            let len = scene.steps.len();
            egui::Grid::new("steps")
                .num_columns(5)
                .striped(true)
                .show(ui, |ui| {
                    ui.weak("#");
                    ui.weak("Überblenden (s)");
                    ui.weak("Halten (s)");
                    ui.weak("Kanäle");
                    ui.weak("");
                    ui.end_row();
                    for (i, step) in scene.steps.iter_mut().enumerate() {
                        ui.label(format!("{}", i + 1));
                        changed_times |= seconds(ui, &mut step.fade_ms).changed();
                        changed_times |= seconds(ui, &mut step.hold_ms).changed();
                        ui.label(step.values.len().to_string());
                        ui.horizontal(|ui| {
                            if ui
                                .small_button("laden")
                                .on_hover_text("Werte in den Programmer laden")
                                .clicked()
                            {
                                action = Some(StepAction::Load(i));
                            }
                            if ui
                                .small_button("überschreiben")
                                .on_hover_text("Mit Programmer-Werten überschreiben")
                                .clicked()
                            {
                                action = Some(StepAction::Overwrite(i));
                            }
                            if i > 0 && ui.small_button("hoch").clicked() {
                                action = Some(StepAction::Up(i));
                            }
                            if i + 1 < len && ui.small_button("runter").clicked() {
                                action = Some(StepAction::Down(i));
                            }
                            if ui.small_button("Entf.").clicked() {
                                action = Some(StepAction::Delete(i));
                            }
                        });
                        ui.end_row();
                    }
                });
            if len == 0 {
                ui.weak("Keine Schritte. Im Programmer Werte setzen und „Schritt“ anhängen.");
            }
        });

    if let Some(a) = action {
        apply(e, st, id, a);
    }
    let _ = changed_times;
}

fn scene_header(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, id: SceneId) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let mut name = scene.name.clone();
    let mut zone = scene.zone;
    let mut loop_mode = scene.loop_mode;
    let active = e.is_active(id);

    let mut delete = false;
    let mut toggle = false;
    let mut zone_changed = false;
    ui.horizontal(|ui| {
        ui.label("Name");
        ui.text_edit_singleline(&mut name);
        ui.label("Zone");
        zone_changed = zone_combo(ui, e, &mut zone, "edit_zone");
        ui.label("Ablauf");
        egui::ComboBox::from_id_salt("loop_mode")
            .selected_text(match loop_mode {
                LoopMode::Loop => "Schleife",
                LoopMode::Once => "Einmal",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut loop_mode, LoopMode::Loop, "Schleife");
                ui.selectable_value(&mut loop_mode, LoopMode::Once, "Einmal");
            });
        if ui.button(if active { "Stopp" } else { "Start" }).clicked() {
            toggle = true;
        }
        if ui.button("Szene löschen").clicked() {
            delete = true;
        }
    });

    if let Some(scene) = e.show_mut().scene_mut(id) {
        scene.name = name;
        scene.zone = zone;
        scene.loop_mode = loop_mode;
    }
    if zone_changed && active {
        // Zonen-Exklusivität sofort anwenden
        e.activate(id);
    }
    if toggle {
        e.toggle(id);
    }
    if delete {
        e.show_mut().remove_scene(id);
        e.sync();
        st.edit_scene = None;
        if st.append_target == Some(id) {
            st.append_target = None;
        }
    }
}

fn apply(e: &mut Engine, st: &mut UiState, id: SceneId, a: StepAction) {
    match a {
        StepAction::Load(i) => {
            let values = e
                .show()
                .scene(id)
                .and_then(|s| s.steps.get(i))
                .map(|s| s.values.clone())
                .unwrap_or_default();
            e.clear_programmer();
            for v in &values {
                e.set_programmer(v.channel_ref(), v.value);
            }
            st.selected = values.iter().map(|v| v.fixture).collect();
            st.info(format!("Schritt {} in den Programmer geladen", i + 1));
        }
        StepAction::Overwrite(i) => {
            let values = programmer_values(e);
            if values.is_empty() {
                st.error("Programmer ist leer");
                return;
            }
            if let Some(step) = e.show_mut().scene_mut(id).and_then(|s| s.steps.get_mut(i)) {
                step.values = values;
                st.info(format!("Schritt {} überschrieben", i + 1));
            }
        }
        StepAction::Delete(i) => {
            if let Some(s) = e.show_mut().scene_mut(id) {
                if i < s.steps.len() {
                    s.steps.remove(i);
                }
            }
            e.sync();
        }
        StepAction::Up(i) => {
            if let Some(s) = e.show_mut().scene_mut(id) {
                s.steps.swap(i - 1, i);
            }
        }
        StepAction::Down(i) => {
            if let Some(s) = e.show_mut().scene_mut(id) {
                s.steps.swap(i, i + 1);
            }
        }
    }
}

/// Millisekunden-Wert als Sekunden mit einer Nachkommastelle bearbeiten.
fn seconds(ui: &mut egui::Ui, ms: &mut u32) -> egui::Response {
    let mut s = f64::from(*ms) / 1000.0;
    let r = ui.add(
        egui::DragValue::new(&mut s)
            .range(0.0..=3600.0)
            .speed(0.05)
            .fixed_decimals(2),
    );
    if r.changed() {
        *ms = (s * 1000.0).round() as u32;
    }
    r
}
