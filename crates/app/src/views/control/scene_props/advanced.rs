//! Advanced scene settings: bank/scene pickers and scopes.

#[allow(unused_imports)]
use super::*;

/// What the selection window is open for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PickTarget {
    /// “Jump to”: scenes.
    Jump,
    /// Release mode “banks”.
    Release,
    /// Release protection “banks”.
    Protect,
}

/// Open selection window (scenes or banks, multi-selection).
pub(super) struct Picker {
    scene: SceneId,
    target: PickTarget,
    chosen: std::collections::BTreeSet<u32>,
}

/// “Advanced properties” tab: priority, repeat, end, release, release
/// protection and flash.
pub(crate) fn advanced(
    ui: &mut egui::Ui,
    e: &mut Engine,
    id: SceneId,
    locked: bool,
    picker: &mut Option<Picker>,
) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let mut st = scene.settings.clone();
    let mut loop_mode = scene.loop_mode;
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let names: std::collections::HashMap<u32, String> = e
        .show()
        .scenes
        .iter()
        .map(|s| (s.id.0, s.name.clone()))
        .collect();
    let on = !locked;
    let once = loop_mode == LoopMode::Once;
    let mut open = None;

    ui.add_enabled_ui(on, |ui| {
        row(ui, t("Scene priority"), |ui| {
            ui.add(egui::DragValue::new(&mut st.priority).range(-10..=10))
                .on_hover_text(t("Higher priority wins where scenes overlap"));
        });
        row(ui, t("Repeats"), |ui| {
            if once {
                ui.add(
                    egui::DragValue::new(&mut st.repeat_count)
                        .range(1..=999)
                        .suffix("×"),
                );
            }
            for (mode, icon, tip) in [
                (LoopMode::Once, Icon::RepeatOnce, t("Play X times")),
                (LoopMode::Loop, Icon::Repeat, t("Loop forever")),
            ] {
                let r = ui
                    .add(IconButton::new(icon).active(loop_mode == mode).tint(color))
                    .on_hover_text(tip);
                let key = format!("scene/loop/{mode:?}");
                let mapped = crate::mapping::button(ui, &r, &key, loop_mode == mode, || {
                    (t("Scene").to_string(), tip.to_string())
                });
                if r.clicked() || mapped {
                    loop_mode = mode;
                }
            }
        });
        ui.add_enabled_ui(once, |ui| {
            row(ui, t("Jump to"), |ui| {
                let text = match st.jump_to_scenes.as_slice() {
                    [] => t("None").to_string(),
                    [one] => names.get(&one.0).cloned().unwrap_or_default(),
                    many => tr!("{n} scenes", n = many.len()),
                };
                if ui.button(text).clicked() {
                    open = Some(PickTarget::Jump);
                }
            });
            row(ui, t("Release at end"), |ui| {
                ui.add(Toggle::new(&mut st.release_at_end, "").map(
                    "scene/release_at_end",
                    t("Scene"),
                    t("Release at end"),
                ));
            });
        });
        ui.add_space(theme::SP_S);
        if scope_row(
            ui,
            t("Release mode"),
            ("release_mode", id.0),
            &mut st.release_mode,
        ) {
            open = Some(PickTarget::Release);
        }
        if scope_row(
            ui,
            t("Protect from release"),
            ("protect", id.0),
            &mut st.protect,
        ) {
            open = Some(PickTarget::Protect);
        }
        ui.add_space(theme::SP_S);
        row(ui, t("Flash mode"), |ui| {
            ui.add(Toggle::new(&mut st.flash, "").map("scene/flash", t("Scene"), t("Flash mode")))
                .on_hover_text(t("Only plays while the scene is held down"));
        });
    });

    if let Some(target) = open {
        let chosen = match target {
            PickTarget::Jump => st.jump_to_scenes.iter().map(|s| s.0).collect(),
            PickTarget::Release => banks_of(&st.release_mode),
            PickTarget::Protect => banks_of(&st.protect),
        };
        *picker = Some(Picker {
            scene: id,
            target,
            chosen,
        });
    }

    if let Some(scene) = e.show().scene(id) {
        if scene.settings != st || scene.loop_mode != loop_mode {
            if let Some(scene) = e.show_mut().scene_mut(id) {
                scene.settings = st;
                scene.loop_mode = loop_mode;
            }
        }
    }
}

pub(crate) fn banks_of(scope: &BankScope) -> std::collections::BTreeSet<u32> {
    match scope {
        BankScope::Banks(list) => list.iter().map(|z| z.0).collect(),
        _ => Default::default(),
    }
}

/// A bank in the selection window: ID, name, color and its scenes.
pub(crate) type PickColumn = (ZoneId, String, Color32, Vec<(SceneId, String, Color32)>);

/// Selection window: all banks as columns with their scenes. Depending on
/// the purpose scenes or banks (header) are selected, several possible.
pub(crate) fn picker_window(ctx: &egui::Context, e: &mut Engine, picker: &mut Option<Picker>) {
    let Some(p) = picker.as_mut() else {
        return;
    };
    let pick_banks = p.target != PickTarget::Jump;
    let title = if pick_banks {
        t("Select banks")
    } else {
        t("Select scenes")
    };
    let show = e.show();
    let columns: Vec<PickColumn> = show
        .zones
        .iter()
        .map(|z| {
            let zc = theme::zone_color(show, Some(z.id));
            let scenes = show
                .scenes
                .iter()
                .filter(|s| s.zone == Some(z.id) && s.id != p.scene)
                .map(|s| (s.id, s.name.clone(), s.color.map_or(zc, theme::rgb)))
                .collect();
            (z.id, z.name.clone(), zc, scenes)
        })
        .collect();
    let mut open = true;
    let res = Dialog::new(title)
        .id("scene_bank_picker")
        .resizable([760.0, 460.0])
        .cancel(t("Cancel"))
        .primary(t("Apply"))
        .show(ctx, &mut open, |ui, _| {
            ui.label(
                egui::RichText::new(if pick_banks {
                    t("Click banks to select or deselect them.")
                } else {
                    t("Click scenes to select or deselect them.")
                })
                .color(theme::text_dim()),
            );
            ui.add_space(theme::SP_S);
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        for (zid, zname, zc, scenes) in &columns {
                            ui.vertical(|ui| {
                                ui.set_width(150.0);
                                let on = pick_banks && p.chosen.contains(&zid.0);
                                if pick_tile(ui, zname, *zc, on, pick_banks, true) {
                                    toggle(&mut p.chosen, zid.0);
                                }
                                for (sid, name, c) in scenes {
                                    let on = !pick_banks && p.chosen.contains(&sid.0);
                                    if pick_tile(ui, name, *c, on, !pick_banks, false) {
                                        toggle(&mut p.chosen, sid.0);
                                    }
                                }
                            });
                        }
                    });
                });
        });
    if res.clicked(1) {
        let (scene, target, chosen) = (p.scene, p.target, p.chosen.clone());
        if let Some(s) = e.show_mut().scene_mut(scene) {
            match target {
                PickTarget::Jump => {
                    s.settings.jump_to_scenes = chosen.into_iter().map(SceneId).collect();
                }
                PickTarget::Release => {
                    s.settings.release_mode =
                        BankScope::Banks(chosen.into_iter().map(ZoneId).collect());
                }
                PickTarget::Protect => {
                    s.settings.protect = BankScope::Banks(chosen.into_iter().map(ZoneId).collect());
                }
            }
        }
        open = false;
    }
    if !open {
        *picker = None;
    }
}

pub(crate) fn toggle(set: &mut std::collections::BTreeSet<u32>, v: u32) {
    if !set.remove(&v) {
        set.insert(v);
    }
}

/// Tile in the selection window; `true` when clicked.
pub(crate) fn pick_tile(
    ui: &mut egui::Ui,
    name: &str,
    color: Color32,
    on: bool,
    enabled: bool,
    head: bool,
) -> bool {
    let h = if head { 30.0 } else { 34.0 };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(150.0, h), Sense::click());
    let p = ui.painter();
    let fill = if on {
        color.gamma_multiply(0.25)
    } else if enabled && resp.hovered() {
        theme::raised_hover()
    } else if head {
        theme::bg()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S, fill);
    let stroke = if on {
        Stroke::new(1.5, color)
    } else {
        Stroke::new(1.0, theme::border())
    };
    p.rect_stroke(rect, theme::R_S, stroke, egui::StrokeKind::Inside);
    if head {
        p.line_segment(
            [
                rect.left_top() + Vec2::new(6.0, 2.0),
                rect.right_top() + Vec2::new(-6.0, 2.0),
            ],
            Stroke::new(2.0, color),
        );
    }
    p.text(
        pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        if head {
            theme::heading_font()
        } else {
            theme::body()
        },
        if enabled || head {
            color
        } else {
            color.gamma_multiply(0.5)
        },
    );
    ui.add_space(4.0);
    enabled && resp.clicked()
}

/// Row: label on the left, control right-aligned.
pub(crate) fn row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        ui.label(egui::RichText::new(label).color(theme::text_dim()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add);
    });
}

/// Bank reference as a short selection on the right; for “banks” a button
/// for the selection window below. `true` = open the window.
pub(crate) fn scope_row(
    ui: &mut egui::Ui,
    label: &str,
    id: (&str, u32),
    scope: &mut BankScope,
) -> bool {
    let names = [t("Off"), t("All"), t("Bank"), t("Outside"), t("Banks")];
    let index = match scope {
        BankScope::Off => 0,
        BankScope::All => 1,
        BankScope::Bank => 2,
        BankScope::OutsideBank => 3,
        BankScope::Banks(_) => 4,
    };
    let mut picked = index;
    let mut open = false;
    row(ui, label, |ui| {
        egui::ComboBox::from_id_salt(id)
            .selected_text(names[index])
            .width(110.0)
            .show_ui(ui, |ui| {
                for (i, n) in names.iter().enumerate() {
                    ui.selectable_value(&mut picked, i, *n);
                }
            });
    });
    if picked != index {
        *scope = match picked {
            0 => BankScope::Off,
            1 => BankScope::All,
            2 => BankScope::Bank,
            3 => BankScope::OutsideBank,
            _ => {
                open = true;
                BankScope::Banks(Vec::new())
            }
        };
    }
    if let BankScope::Banks(list) = scope {
        row(ui, "", |ui| {
            let text = match list.len() {
                0 => t("No bank").to_string(),
                1 => t("1 bank").to_string(),
                n => tr!("{n} banks", n = n),
            };
            if ui.button(text).on_hover_text(t("Select banks")).clicked() {
                open = true;
            }
        });
    }
    open
}
