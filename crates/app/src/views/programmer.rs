//! Programmer: ausgewählte Geräte live per Fader steuern und das Ergebnis als
//! Szene oder Schritt speichern.

use eframe::egui;
use lightcraft_core::{ChannelKind, ChannelRef, ChannelValue, DmxFrame, Engine, Step};

use crate::app::UiState;

/// Ein Fader in der Oberfläche; steuert gleichnamige Kanäle aller
/// ausgewählten Geräte gemeinsam.
struct FaderGroup {
    name: String,
    kind: ChannelKind,
    channels: Vec<ChannelRef>,
}

fn fader_groups(e: &Engine, st: &UiState) -> Vec<FaderGroup> {
    let mut groups: Vec<FaderGroup> = Vec::new();
    for fid in &st.selected {
        let Some(f) = e.show().fixture(*fid) else {
            continue;
        };
        let Some(ty) = e.show().type_of(f) else {
            continue;
        };
        for (i, def) in ty.channels.iter().enumerate() {
            let ch = ChannelRef {
                fixture: f.id,
                channel: i as u16,
            };
            match groups
                .iter_mut()
                .find(|g| g.kind == def.kind && g.name == def.name)
            {
                Some(g) => g.channels.push(ch),
                None => groups.push(FaderGroup {
                    name: def.name.clone(),
                    kind: def.kind,
                    channels: vec![ch],
                }),
            }
        }
    }
    groups
}

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, frame: &DmxFrame) {
    if st.selected.is_empty() {
        ui.add_space(20.0);
        ui.label("Wähle links im Patch ein oder mehrere Geräte aus.");
        programmer_actions(ui, e, st);
        return;
    }

    let groups = fader_groups(e, st);

    ui.horizontal(|ui| {
        ui.strong(format!("{} Gerät(e) ausgewählt", st.selected.len()));
        // Farbwähler für RGB-Geräte
        let find = |k: ChannelKind| groups.iter().find(|g| g.kind == k);
        if let (Some(r), Some(g), Some(b)) = (
            find(ChannelKind::Red),
            find(ChannelKind::Green),
            find(ChannelKind::Blue),
        ) {
            let cur = |grp: &FaderGroup| e.output_value(frame, grp.channels[0]).unwrap_or(0);
            let mut rgb = [cur(r), cur(g), cur(b)];
            ui.separator();
            ui.label("Farbe");
            if ui.color_edit_button_srgb(&mut rgb).changed() {
                for (grp, v) in [(r, rgb[0]), (g, rgb[1]), (b, rgb[2])] {
                    for ch in &grp.channels {
                        e.set_programmer(*ch, v);
                    }
                }
            }
        }
    });
    ui.add_space(6.0);

    egui::ScrollArea::horizontal()
        .id_salt("faders")
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                for grp in &groups {
                    let first = grp.channels[0];
                    let in_programmer = e.programmer().contains_key(&first);
                    let mut v = e
                        .programmer()
                        .get(&first)
                        .copied()
                        .or_else(|| e.output_value(frame, first))
                        .unwrap_or(0);
                    ui.vertical(|ui| {
                        ui.set_width(64.0);
                        let title = egui::RichText::new(&grp.name).small();
                        ui.label(if in_programmer {
                            title.strong()
                        } else {
                            title.weak()
                        })
                        .on_hover_text(format!(
                            "{} – {} Kanal/Kanäle",
                            grp.kind.label(),
                            grp.channels.len()
                        ));
                        let slider = egui::Slider::new(&mut v, 0..=255)
                            .vertical()
                            .show_value(true);
                        ui.spacing_mut().slider_width = 220.0;
                        if ui.add(slider).changed() {
                            for ch in &grp.channels {
                                e.set_programmer(*ch, v);
                            }
                        }
                        ui.horizontal(|ui| {
                            if ui.small_button("0").clicked() {
                                for ch in &grp.channels {
                                    e.set_programmer(*ch, 0);
                                }
                            }
                            if ui.small_button("Max").clicked() {
                                for ch in &grp.channels {
                                    e.set_programmer(*ch, 255);
                                }
                            }
                        });
                        if in_programmer
                            && ui
                                .small_button("freigeben")
                                .on_hover_text("Kanal aus dem Programmer entfernen")
                                .clicked()
                        {
                            for ch in &grp.channels {
                                e.clear_programmer_channel(*ch);
                            }
                        }
                    });
                }
            });
        });

    ui.separator();
    programmer_actions(ui, e, st);
}

/// Speichern des Programmer-Inhalts als Szene / Schritt.
fn programmer_actions(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    let count = e.programmer().len();
    ui.horizontal(|ui| {
        ui.label(format!("Programmer: {count} Kanal/Kanäle gesetzt"));
        if ui.button("Programmer leeren").clicked() {
            e.clear_programmer();
        }
    });
    if count == 0 {
        return;
    }
    let values = programmer_values(e);

    ui.horizontal(|ui| {
        ui.label("Als neue Szene:");
        ui.text_edit_singleline(&mut st.new_scene_name);
        zone_combo(ui, e, &mut st.new_scene_zone, "new_scene_zone");
        if ui.button("Speichern").clicked() {
            let id = e.show_mut().add_scene(
                st.new_scene_name.trim().to_string(),
                st.new_scene_zone,
                vec![Step::new(values.clone())],
            );
            st.edit_scene = Some(id);
            st.append_target = Some(id);
            st.info(format!("Szene „{}“ angelegt", st.new_scene_name.trim()));
        }
    });

    ui.horizontal(|ui| {
        ui.label("Als Schritt anhängen an:");
        let name = st
            .append_target
            .and_then(|id| e.show().scene(id))
            .map_or_else(|| "– Szene wählen –".into(), |s| s.name.clone());
        egui::ComboBox::from_id_salt("append_target")
            .selected_text(name)
            .show_ui(ui, |ui| {
                for s in &e.show().scenes {
                    ui.selectable_value(&mut st.append_target, Some(s.id), &s.name);
                }
            });
        if ui.button("Anhängen").clicked() {
            match st.append_target.and_then(|id| e.show_mut().scene_mut(id)) {
                Some(scene) => {
                    let mut step = Step::new(values.clone());
                    step.fade_ms = 1000;
                    scene.steps.push(step);
                    let msg = format!(
                        "Schritt {} an „{}“ angehängt",
                        scene.steps.len(),
                        scene.name
                    );
                    st.info(msg);
                }
                None => st.error("Bitte zuerst eine Szene wählen"),
            }
        }
    });
}

pub fn programmer_values(e: &Engine) -> Vec<ChannelValue> {
    let mut v: Vec<ChannelValue> = e
        .programmer()
        .iter()
        .map(|(ch, val)| ChannelValue {
            fixture: ch.fixture,
            channel: ch.channel,
            value: *val,
        })
        .collect();
    v.sort_by_key(|c| (c.fixture, c.channel));
    v
}

pub fn zone_combo(
    ui: &mut egui::Ui,
    e: &Engine,
    zone: &mut Option<lightcraft_core::ZoneId>,
    id: &str,
) -> bool {
    let name = zone
        .and_then(|z| e.show().zones.iter().find(|x| x.id == z))
        .map_or_else(|| "ohne Zone".into(), |z| z.name.clone());
    let before = *zone;
    egui::ComboBox::from_id_salt(id)
        .selected_text(name)
        .show_ui(ui, |ui| {
            ui.selectable_value(zone, None, "ohne Zone");
            for z in &e.show().zones {
                ui.selectable_value(zone, Some(z.id), &z.name);
            }
        });
    before != *zone
}
