//! Target chooser of buttons, faders and knobs.

#[allow(unused_imports)]
use super::*;

/// Target chooser of a button, fader or knob: `Some((control, name))` when
/// one was picked (`control` `None` = remove the target).
pub(crate) fn target_menu(
    ui: &mut Ui,
    show: &Show,
    item: &TouchItem,
) -> Option<(Option<Control>, String)> {
    use dmxcraft_core::mapping::{ui_key, SceneTarget};
    let fader = matches!(item.kind, TouchKind::Fader | TouchKind::Knob);
    let current = if item.control.is_some() {
        short_name(&item_target(show, item))
    } else {
        t("Choose target …").to_string()
    };
    let resp = ui.add(egui::Button::new(current).min_size(vec2(ui.available_width(), 0.0)));
    let mut out = None;
    let ui_button = |key: String| Control::Ui {
        key: ui_key(&key),
        fader: false,
    };
    let ui_fader = |key: String| Control::Ui {
        key: ui_key(&key),
        fader: true,
    };
    menu::dropdown(&resp, |ui| {
        menu::begin(ui);
        let mut pick = |ui: &mut Ui, label: &str, c: Control, name: String| {
            let on = item.control == Some(c);
            if menu::item(ui, MenuItem::new(label).checked(on)) {
                out = Some((Some(c), name));
            }
        };
        if fader {
            pick(ui, t("Master"), Control::Master, t("Master").to_string());
            pick(
                ui,
                t("Sound master"),
                Control::AudioMaster,
                t("Sound master").to_string(),
            );
            menu::sub(ui, MenuItem::new(t("Sound")), |ui| {
                pick(
                    ui,
                    t("Volume"),
                    Control::SoundVolume { deck: None },
                    t("Volume").to_string(),
                );
                for d in 0..show.sound.decks.len() {
                    let name = crate::tr!("Player {d}", d = crate::views::sound::deck_name(d));
                    pick(
                        ui,
                        &name,
                        Control::SoundVolume {
                            deck: Some(d as u8),
                        },
                        name.clone(),
                    );
                }
            });
            if !show.trackers.is_empty() {
                menu::sub(ui, MenuItem::new(t("Live tracking")), |ui| {
                    for tr in &show.trackers {
                        menu::sub(ui, MenuItem::new(&tr.name), |ui| {
                            for (k, l) in [
                                ("x", t("Left / right")),
                                ("z", t("Front / back")),
                                ("y", t("Height")),
                                ("dimmer", t("Dimmer")),
                                ("zoom", t("Zoom")),
                            ] {
                                let c = ui_fader(format!("tracking/{}/{k}", tr.id));
                                pick(ui, l, c, format!("{}: {l}", tr.name));
                            }
                        });
                    }
                });
            }
        } else {
            menu::sub(ui, MenuItem::new(t("Scene")), |ui| {
                pick(
                    ui,
                    t("Selected scene"),
                    Control::Scene {
                        target: SceneTarget::Selected,
                    },
                    t("Selected scene").to_string(),
                );
                menu::separator(ui);
                for z in &show.zones {
                    let scenes: Vec<_> = show
                        .scenes
                        .iter()
                        .filter(|s| s.zone == Some(z.id))
                        .collect();
                    if scenes.is_empty() {
                        continue;
                    }
                    menu::sub(ui, MenuItem::new(&z.name), |ui| {
                        for sc in scenes {
                            pick(
                                ui,
                                &sc.name,
                                Control::Scene {
                                    target: SceneTarget::Scene { id: sc.id },
                                },
                                sc.name.clone(),
                            );
                        }
                    });
                }
            });
            menu::sub(ui, MenuItem::new(t("Bank")), |ui| {
                pick(
                    ui,
                    t("Show all banks"),
                    Control::ShowBank { zone: None },
                    t("All").to_string(),
                );
                pick(
                    ui,
                    t("Next bank"),
                    Control::BankStep { delta: 1 },
                    t("Next bank").to_string(),
                );
                pick(
                    ui,
                    t("Previous bank"),
                    Control::BankStep { delta: -1 },
                    t("Previous bank").to_string(),
                );
                menu::separator(ui);
                for z in &show.zones {
                    menu::sub(ui, MenuItem::new(&z.name), |ui| {
                        pick(
                            ui,
                            t("Show bank"),
                            Control::ShowBank { zone: Some(z.id) },
                            z.name.clone(),
                        );
                        pick(
                            ui,
                            t("Pause"),
                            Control::ZonePause { zone: z.id },
                            format!("{}: {}", z.name, t("Pause")),
                        );
                        pick(
                            ui,
                            t("Next scene"),
                            Control::ZoneStep {
                                zone: z.id,
                                delta: 1,
                            },
                            format!("{} ›", z.name),
                        );
                        pick(
                            ui,
                            t("Previous scene"),
                            Control::ZoneStep {
                                zone: z.id,
                                delta: -1,
                            },
                            format!("‹ {}", z.name),
                        );
                    });
                }
            });
            if !show.groups.is_empty() {
                menu::sub(ui, MenuItem::new(t("Group")), |ui| {
                    pick(
                        ui,
                        t("All"),
                        Control::Group { id: None },
                        t("All").to_string(),
                    );
                    for g in &show.groups {
                        pick(
                            ui,
                            &g.name,
                            Control::Group { id: Some(g.id) },
                            g.name.clone(),
                        );
                    }
                });
            }
            if !show.trackers.is_empty() {
                menu::sub(ui, MenuItem::new(t("Live tracking")), |ui| {
                    for tr in &show.trackers {
                        menu::sub(ui, MenuItem::new(&tr.name), |ui| {
                            for (k, l) in [
                                ("follow", t("Follow")),
                                ("light", t("Own light")),
                                ("own_zoom", t("Own zoom")),
                            ] {
                                let c = ui_button(format!("tracking/{}/{k}", tr.id));
                                pick(ui, l, c, tr.name.clone());
                            }
                        });
                    }
                });
            }
            menu::sub(ui, MenuItem::new(t("Sound")), |ui| {
                for (i, pad) in show.sound.pads.iter().enumerate() {
                    if pad.path.is_empty() {
                        continue;
                    }
                    let name = crate::tr!("Pad {n}: {name}", n = i + 1, name = pad.label());
                    pick(ui, &name, ui_button(format!("sound/pad/{i}")), pad.label());
                }
                pick(
                    ui,
                    t("Stop all pads"),
                    ui_button("sound/pads/stop".into()),
                    t("Stop all pads").to_string(),
                );
                menu::separator(ui);
                for d in 0..show.sound.decks.len() {
                    let deck = crate::views::sound::deck_name(d);
                    for (k, l) in [("play", t("Play / pause")), ("stop", t("Stop"))] {
                        let name = crate::tr!("Player {d}: {what}", d = deck, what = l);
                        pick(
                            ui,
                            &name,
                            ui_button(format!("sound/deck/{d}/{k}")),
                            name.clone(),
                        );
                    }
                }
                pick(
                    ui,
                    t("Crossfade"),
                    ui_button("sound/xfade".into()),
                    t("Crossfade").to_string(),
                );
                pick(ui, t("Mute"), Control::AudioMute, t("Mute").to_string());
                pick(
                    ui,
                    t("Fade out"),
                    Control::AudioFade,
                    t("Fade out").to_string(),
                );
            });
            menu::sub(ui, MenuItem::new(t("Programmer")), |ui| {
                for (m, l) in [
                    ("Edit", "Edit"),
                    ("Blind", t("Blind edit")),
                    ("Live", t("Live")),
                ] {
                    pick(
                        ui,
                        l,
                        ui_button(format!("programmer/mode/{m}")),
                        l.to_string(),
                    );
                }
                pick(
                    ui,
                    t("Reset all beams"),
                    ui_button("programmer/reset_all".into()),
                    t("Reset all beams").to_string(),
                );
            });
            menu::separator(ui);
            pick(
                ui,
                t("Blackout"),
                Control::Blackout,
                t("Blackout").to_string(),
            );
            pick(
                ui,
                t("Stop all"),
                Control::StopAll,
                t("Stop all").to_string(),
            );
        }
        menu::separator(ui);
        ui.add(MenuItem::new(t("Or Alt + click on a control")).enabled(false));
        if item.control.is_some() && menu::item(ui, MenuItem::new(t("Remove target")).danger()) {
            out = Some((None, String::new()));
        }
    });
    out
}

/// Name of a target without its category (“Scene: Intro” → “Intro”).
pub(crate) fn short_name(full: &str) -> String {
    full.split_once(": ")
        .map_or_else(|| full.to_string(), |x| x.1.to_string())
}
