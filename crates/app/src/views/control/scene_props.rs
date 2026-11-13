//! CONTROL → sidebar to the right of the scenes: settings of the selected
//! scene.
//!
//! On the far right a narrow bar with tab icons, the content to its left.
//! Every tab shows the scene's name and color at the top with the
//! play/pause button.

use crate::i18n::t;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use dmxcraft_core::{BankScope, Effect, EffectKind, Engine, LoopMode, SceneId, ZoneId};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, Vec2};

use crate::ui::theme;
use crate::ui::widgets::knob::Knob;
use crate::ui::widgets::time_field::time_field;
use crate::ui::widgets::{Dialog, Icon, IconButton, Toggle};
use crate::views::{View, ViewCtx};

/// Width of the tab bar.
const TABS_W: f32 = 40.0;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    Properties,
    #[default]
    Effects,
    Third,
}

#[derive(Default)]
pub struct ScenePropsView {
    tab: Tab,
    /// Selected effect (scene, effect ID).
    selected_fx: Option<(SceneId, u32)>,
    /// Tile selection for adding an effect open.
    adding: bool,
    /// A step (not the effect) was clicked last: delete then removes the step.
    step_focus: bool,
    /// Running import/export dialog.
    fx_dialog: Option<FxDialog>,
    /// Custom color palettes (COLOR FX).
    palettes: super::fx_color::PaletteStore,
    /// Selection window for scenes/banks.
    picker: Option<Picker>,
}

/// Import/export of effects via the system dialog (own thread).
struct FxDialog {
    export: bool,
    scene: SceneId,
    rx: Receiver<Option<PathBuf>>,
}

/// File extension of exported effects (content JSON).
const FX_EXT: &str = "dmxfx";

impl View for ScenePropsView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let full = ui.available_rect_before_wrap();
        let tabs = Rect::from_min_max(pos2(full.right() - TABS_W, full.top()), full.max);
        let body = Rect::from_min_max(full.min, pos2(tabs.left() - theme::SP_S, full.bottom()));
        ui.allocate_rect(full, Sense::hover());

        // Tab bar
        let p = ui.painter();
        p.line_segment(
            [tabs.left_top(), tabs.left_bottom()],
            Stroke::new(1.0, theme::border()),
        );
        let mut tab_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(tabs.shrink2(Vec2::new(3.0, 0.0)))
                .layout(egui::Layout::top_down(egui::Align::Center)),
        );
        // several scenes: properties apply to all, effects only one at a time
        let multi = cx.state.selected_scenes.len() > 1;
        // only shown, not remembered: back to one scene, FX is open again
        let shown = if multi && self.tab == Tab::Effects {
            Tab::Properties
        } else {
            self.tab
        };
        for (tab, icon, tip) in [
            (Tab::Properties, Icon::Sliders, t("Properties")),
            (Tab::Effects, Icon::Fx, t("Effects")),
            (Tab::Third, Icon::Gear, t("Advanced properties")),
        ] {
            let enabled = !(multi && tab == Tab::Effects);
            let r = tab_ui
                .add(
                    IconButton::new(icon)
                        .size(32.0)
                        .flat()
                        .active(shown == tab)
                        .enabled(enabled),
                )
                .on_hover_text(tip);
            let key = format!("scene/tab/{}", tab as u8);
            let mapped = crate::mapping::button(&tab_ui, &r, &key, shown == tab, || {
                (t("Sidebar").to_string(), tip.to_string())
            });
            if (r.clicked() || mapped) && enabled {
                self.tab = tab;
            }
        }

        let mut ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(body)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let ui = &mut ui;
        if let Some(g) = cx.state.selected_group {
            egui::ScrollArea::vertical()
                .id_salt("scene_group_editor")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    super::scene_groups::editor(ui, cx.engine, cx.state, g)
                });
            return;
        }
        let scenes = selected_scenes(cx);
        let Some(&id) = scenes.first() else {
            return;
        };
        if scenes.len() > 1 {
            multi_header(ui, cx.engine, id, scenes.len());
        } else {
            header(ui, cx.engine, id);
        }
        ui.add_space(theme::SP_S);
        let locked = cx.state.locked;
        // The first scene is edited; changed fields then go to all of them
        let before = SceneEdit::of(cx.engine, id);
        picker_window(ui.ctx(), cx.engine, &mut self.picker);
        self.poll_dialog(cx);
        if shown == Tab::Effects {
            self.effects_tab(ui, cx, id, locked);
            return;
        }
        egui::ScrollArea::vertical()
            .id_salt("scene_props")
            .auto_shrink([false, false])
            .show(ui, |ui| match shown {
                Tab::Properties => properties(ui, cx.engine, id, locked),
                Tab::Third => advanced(ui, cx.engine, id, locked, &mut self.picker),
                Tab::Effects => {}
            });
        if let (Some(before), Some(after)) = (before, SceneEdit::of(cx.engine, id)) {
            if before != after {
                apply_to_others(cx.engine, &before, &after, &scenes[1..]);
            }
        }
    }
}

/// Selected scenes that still exist (sorted by ID).
fn selected_scenes(cx: &ViewCtx<'_>) -> Vec<SceneId> {
    cx.state
        .selected_scenes
        .iter()
        .copied()
        .filter(|id| cx.engine.show().scene(*id).is_some())
        .collect()
}

/// What the “Properties” and “Advanced” tabs change on a scene.
#[derive(Clone, PartialEq)]
struct SceneEdit {
    settings: serde_json::Value,
    loop_mode: LoopMode,
    duration: u64,
}

impl SceneEdit {
    fn of(e: &Engine, id: SceneId) -> Option<Self> {
        let s = e.show().scene(id)?;
        Some(Self {
            settings: serde_json::to_value(&s.settings).ok()?,
            loop_mode: s.loop_mode,
            duration: s.duration_ms(),
        })
    }
}

/// Transfer only the fields changed on the first scene to the others.
fn apply_to_others(e: &mut Engine, before: &SceneEdit, after: &SceneEdit, others: &[SceneId]) {
    let changed: Vec<(&String, &serde_json::Value)> = match (&before.settings, &after.settings) {
        (serde_json::Value::Object(b), serde_json::Value::Object(a)) => {
            a.iter().filter(|(k, v)| b.get(*k) != Some(*v)).collect()
        }
        _ => Vec::new(),
    };
    for id in others {
        let Some(s) = e.show_mut().scene_mut(*id) else {
            continue;
        };
        if !changed.is_empty() {
            if let Ok(serde_json::Value::Object(mut map)) = serde_json::to_value(&s.settings) {
                for (k, v) in &changed {
                    map.insert((*k).clone(), (*v).clone());
                }
                if let Ok(st) = serde_json::from_value(serde_json::Value::Object(map)) {
                    s.settings = st;
                }
            }
        }
        if before.loop_mode != after.loop_mode {
            s.loop_mode = after.loop_mode;
        }
        if before.duration != after.duration {
            s.set_duration_ms(after.duration);
        }
    }
}

/// Header for a multi-selection: count in the color of the first scene.
fn multi_header(ui: &mut egui::Ui, e: &Engine, first: SceneId, n: usize) {
    let color = e.show().scene(first).map_or_else(theme::accent, |s| {
        s.color
            .map_or_else(|| theme::zone_color(e.show(), s.zone), theme::rgb)
    });
    ui.horizontal(|ui| {
        ui.set_min_height(34.0);
        ui.label(
            egui::RichText::new(tr!("{n} scenes", n = n))
                .color(color)
                .strong(),
        );
    });
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(line, 0.0, theme::border());
}

/// Name and color of the scene, play/pause on the right, a divider below.
fn header(ui: &mut egui::Ui, e: &mut Engine, id: SceneId) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let name = scene.name.clone();
    let active = e.is_active(id);
    let paused = e.progress(id).is_some_and(|p| p.paused);
    ui.horizontal(|ui| {
        ui.set_min_height(34.0);
        ui.label(egui::RichText::new(&name).color(color).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let icon = if active && !paused {
                Icon::Pause
            } else {
                Icon::Play
            };
            let tip = if active && !paused {
                t("Pause")
            } else {
                t("Play")
            };
            let r = ui
                .add(IconButton::new(icon).size(30.0).active(active))
                .on_hover_text(tip);
            let mapped =
                crate::mapping::button(ui, &r, "scene/play_pause", active && !paused, || {
                    (t("Scene").to_string(), t("Play / pause").to_string())
                });
            if r.clicked() || mapped {
                if active {
                    e.toggle_pause(id);
                } else {
                    e.activate(id);
                }
            }
        });
    });
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(line, 0.0, theme::border());
}

fn properties(ui: &mut egui::Ui, e: &mut Engine, id: SceneId, locked: bool) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let mut st = scene.settings.clone();
    let super_scene = scene.timeline().is_some();
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let mut duration = scene.duration_ms() as u32;
    let duration_before = duration;
    let on = !locked;

    // Slider in the scene color
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let mut dim = f32::from(st.dimmer) / 255.0;
        let dim_text = format!("{:.0} %", dim * 100.0);
        if Knob::new(&mut dim, t("Dimmer"), dim_text)
            .map("scene/knob/dimmer", t("Scene"))
            .color(color)
            .enabled(on)
            .show(ui)
        {
            st.dimmer = (dim * 255.0).round() as u8;
        }
        // Speed logarithmic: 10 % … 100 % (center) … 1000 %
        let mut sp = ((f32::from(st.speed_pct) / 10.0).log10() / 2.0).clamp(0.0, 1.0);
        if Knob::new(&mut sp, "Speed", format!("{} %", st.speed_pct))
            .map("scene/knob/speed", t("Scene"))
            .color(color)
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.speed_pct = (10.0 * 100f32.powf(sp)).round() as u16;
        }
        let mut ph = f32::from(st.phase_deg) / 360.0;
        if Knob::new(&mut ph, t("Phase"), format!("{}°", st.phase_deg))
            .map("scene/knob/phase", t("Scene"))
            .color(color)
            .default_value(0.0)
            .enabled(on)
            .show(ui)
        {
            st.phase_deg = (ph * 360.0).round() as u16;
        }
        let mut sz = f32::from(st.size_pct) / 200.0;
        if Knob::new(&mut sz, "Size", format!("{} %", st.size_pct))
            .map("scene/knob/size", t("Scene"))
            .color(color)
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.size_pct = (sz * 200.0).round() as u8;
        }
    });
    ui.add_space(theme::SP_S);

    // Timeline: start/end, direction, position with elapsed/total time
    let pos = e.scene_position(id);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        for (to, icon, key, tip) in [
            (0.0, Icon::Prev, "scene/seek_start", t("Go to start")),
            (1.0, Icon::Next, "scene/seek_end", t("Go to end")),
        ] {
            let r = ui
                .add(IconButton::new(icon).enabled(pos.is_some()))
                .on_hover_text(tip);
            let mapped = crate::mapping::button(ui, &r, key, false, || {
                (t("Scene").to_string(), tip.to_string())
            });
            if r.clicked() || (mapped && pos.is_some()) {
                e.seek(id, to);
            }
        }
        ui.add_space(theme::SP_XS);
        for (rev, icon, tip) in [
            (true, Icon::PlayBack, t("Play backwards")),
            (false, Icon::Play, t("Play forwards")),
        ] {
            let r = ui
                .add(
                    IconButton::new(icon)
                        .active(st.reverse == rev)
                        .tint(color)
                        .enabled(on),
                )
                .on_hover_text(tip);
            let key = if rev {
                "scene/reverse"
            } else {
                "scene/forward"
            };
            let mapped = crate::mapping::button(ui, &r, key, st.reverse == rev, || {
                (t("Scene").to_string(), tip.to_string())
            });
            if r.clicked() || (mapped && on) {
                st.reverse = rev;
            }
        }
        ui.add_space(theme::SP_XS);
        let mut v = pos.unwrap_or(0.0);
        if bar_slider(ui, "scene_pos", &mut v, pos.is_some(), None, color) {
            e.seek(id, v);
        }
    });
    let total = u64::from(duration);
    let elapsed = (f64::from(pos.unwrap_or(0.0)) * total as f64) as u64;
    // elapsed time on the left (scene color), total duration on the right
    let hours = total >= 3_600_000;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(time_text(elapsed, hours))
                .font(egui::FontId::proportional(theme::FS_BODY + 1.0))
                .strong()
                .color(color),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(time_text(total, hours))
                    .font(egui::FontId::proportional(theme::FS_BODY + 1.0))
                    .color(theme::text()),
            );
        });
    });
    if pos.is_some() {
        ui.ctx().request_repaint();
    }
    ui.add_space(theme::SP_S);

    // Fields right-aligned, label on the left
    let row = |ui: &mut egui::Ui, label: &str, add: &mut dyn FnMut(&mut egui::Ui)| {
        ui.horizontal(|ui| {
            ui.set_min_height(32.0);
            ui.label(egui::RichText::new(label).color(theme::text_dim()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                add(ui)
            });
        });
    };
    row(ui, t("Duration"), &mut |ui| {
        time_field(ui, "duration", &mut duration, on, true, color);
    });
    row(ui, "Fade in", &mut |ui| {
        time_field(ui, "fade_in", &mut st.fade_in_ms, on, false, color);
    });
    row(ui, "Fade out", &mut |ui| {
        time_field(ui, "fade_out", &mut st.fade_out_ms, on, false, color);
    });
    row(ui, t("Fade speed"), &mut |ui| {
        // 0.1 … 4.0 in tenths, default 1.0
        ui.set_max_width(150.0);
        let mut v = f32::from(st.fade_speed.clamp(1, 40) - 1) / 39.0;
        let text = format!("{:.1}", f32::from(st.fade_speed) / 10.0);
        if bar_slider(ui, "fade_speed", &mut v, on, Some(&text), color) {
            st.fade_speed = (v * 39.0).round() as u8 + 1;
        }
    });
    row(ui, t("Phase in"), &mut |ui| {
        time_field(ui, "phase_in", &mut st.phase_in_ms, on, false, color);
    });
    row(ui, t("Phase out"), &mut |ui| {
        time_field(ui, "phase_out", &mut st.phase_out_ms, on, false, color);
    });
    // SUPER SCENE: own duration for “Fade out sound” (0 = program setting)
    if super_scene {
        ui.horizontal(|ui| {
            ui.set_min_height(32.0);
            ui.label(egui::RichText::new(t("Fade out sound")).color(theme::text_dim()))
                .on_hover_text(t(
                    "Duration of “Fade out sound” while this SUPER SCENE is running. 0 = default from the settings.",
                ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                time_field(ui, "audio_fade", &mut st.audio_fade_ms, on, false, color);
                if st.audio_fade_ms == 0 {
                    ui.label(
                        egui::RichText::new(t("Default"))
                            .small()
                            .color(theme::text_faint()),
                    );
                }
            });
        });
    }

    // Apply changes (only if something changed → no “*”)
    if let Some(scene) = e.show().scene(id) {
        let changed = scene.settings != st || duration != duration_before;
        if changed {
            if let Some(scene) = e.show_mut().scene_mut(id) {
                scene.settings = st;
                if duration != duration_before {
                    scene.set_duration_ms(u64::from(duration));
                }
            }
        }
    }
}

/// Time as text like in the time field.
fn time_text(ms: u64, hours: bool) -> String {
    crate::ui::widgets::time_field::format(ms, hours)
}

/// Horizontal bar slider 0..=1, optionally with text in the middle.
pub(super) fn bar_slider(
    ui: &mut egui::Ui,
    id: &str,
    v: &mut f32,
    enabled: bool,
    text: Option<&str>,
    accent: Color32,
) -> bool {
    let w = ui.available_width().max(60.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::hover());
    let sense = if enabled {
        Sense::click_and_drag()
    } else {
        Sense::hover()
    };
    let resp = ui.interact(rect, ui.make_persistent_id(("bar_slider", id)), sense);
    let mut changed = false;
    if resp.dragged() || resp.clicked() {
        if let Some(p) = resp.interact_pointer_pos() {
            let nv = ((p.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            if nv != *v {
                *v = nv;
                changed = true;
            }
        }
    }
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S, theme::bg());
    let fill = Rect::from_min_size(rect.min, Vec2::new(rect.width() * *v, rect.height()));
    let color: Color32 = if enabled {
        accent.gamma_multiply(0.45)
    } else {
        theme::raised()
    };
    p.rect_filled(fill, theme::R_S, color);
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
    );
    if let Some(text) = text {
        p.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            theme::caption(),
            theme::text(),
        );
    }
    changed
}

impl ScenePropsView {
    /// “Effects” tab: for STATIC scenes the tiles to choose from, for FX scenes
    /// the list of effects and the toolbar at the bottom.
    fn effects_tab(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: SceneId, locked: bool) {
        let Some(scene) = cx.engine.show().scene(id) else {
            return;
        };
        let effects = scene.effects.clone();
        if effects.is_empty() || self.adding {
            let mut pick = None;
            // STATIC scene: import/export at the bottom as for FX scenes
            const BAR_H: f32 = 40.0;
            let with_bar = effects.is_empty();
            let area = ui.available_rect_before_wrap();
            let bar_rect = Rect::from_min_max(pos2(area.left(), area.bottom() - BAR_H), area.max);
            let tiles_h = if with_bar {
                (area.height() - BAR_H - theme::SP_XS).max(60.0)
            } else {
                area.height()
            };
            let mut tiles_ui = ui.new_child(egui::UiBuilder::new().max_rect(Rect::from_min_size(
                area.min,
                Vec2::new(area.width(), tiles_h),
            )));
            egui::ScrollArea::vertical()
                .id_salt("fx_tiles")
                .auto_shrink([false, false])
                .show(&mut tiles_ui, |ui| {
                    if self.adding && !effects.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(theme::heading_text(t("Add effect")));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add(IconButton::new(Icon::Close).flat())
                                        .on_hover_text(t("Cancel"))
                                        .clicked()
                                    {
                                        self.adding = false;
                                    }
                                },
                            );
                        });
                        ui.add_space(theme::SP_S);
                    }
                    pick = effect_tiles(ui, !locked, effects.is_empty());
                });
            if with_bar {
                ui.painter().line_segment(
                    [bar_rect.left_top(), bar_rect.right_top()],
                    Stroke::new(1.0, theme::border()),
                );
                let mut bar = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(bar_rect.with_min_y(bar_rect.top() + 1.0))
                        .layout(egui::Layout::right_to_left(egui::Align::Center)),
                );
                let on = !locked && self.fx_dialog.is_none();
                if bar
                    .add(IconButton::new(Icon::Export).size(30.0).flat().enabled(on))
                    .on_hover_text(t("Export scene"))
                    .clicked()
                {
                    self.start_dialog(bar.ctx(), id, true);
                }
                if bar
                    .add(IconButton::new(Icon::Import).size(30.0).flat().enabled(on))
                    .on_hover_text(t("Import effects"))
                    .clicked()
                {
                    self.start_dialog(bar.ctx(), id, false);
                }
            }
            ui.allocate_rect(area, Sense::hover());
            if let Some(kind) = pick {
                cx.state.sync_selection_order(cx.engine.show());
                if let Some(scene) = cx.engine.show_mut().scene_mut(id) {
                    if kind == EffectKind::Steps && scene.steps.is_empty() {
                        scene.steps.push(dmxcraft_core::Step::new(Vec::new()));
                    }
                    let fx = scene.add_effect(kind);
                    self.selected_fx = Some((id, fx));
                    self.step_focus = kind == EffectKind::Steps;
                    if kind == EffectKind::SuperScene {
                        scene.sync_timeline_length();
                    }
                    if !kind.exclusive() {
                        // acts on the selected fixtures (otherwise those of the scene)
                        // order of the selection = beam order
                        let mut ids: Vec<dmxcraft_core::FixtureId> =
                            cx.state.selection_order.clone();
                        if ids.is_empty() {
                            ids = scene
                                .steps
                                .iter()
                                .flat_map(|s| s.values.iter().map(|v| v.fixture))
                                .collect();
                            ids.sort();
                            ids.dedup();
                        }
                        if let Some(e) = scene.effects.iter_mut().find(|e| e.id == fx) {
                            e.fixtures = ids.clone();
                        }
                        cx.state.fx_target = Some((id, fx));
                        cx.state.select_ordered(&ids);
                    }
                }
                if let Some((_, fx)) = cx.state.fx_target.filter(|(s, _)| *s == id) {
                    fit_mapping(cx.engine.show_mut(), id, fx);
                }
                self.adding = false;
            }
            return;
        }

        // List of effects, the toolbar below
        const BAR_H: f32 = 40.0;
        let area = ui.available_rect_before_wrap();
        let bar_rect = Rect::from_min_max(pos2(area.left(), area.bottom() - BAR_H), area.max);
        let list_h = (area.height() - BAR_H - theme::SP_XS).max(60.0);
        // no effect of this scene selected yet: the first one
        let current = self
            .selected_fx
            .filter(|(s, f)| *s == id && effects.iter().any(|e| e.id == *f));
        if current.is_none() {
            let first = &effects[0];
            self.selected_fx = Some((id, first.id));
            self.step_focus = false;
            if first.kind == EffectKind::Steps {
                cx.state.fx_target = None;
            } else {
                cx.state.fx_target = Some((id, first.id));
                cx.state.select_ordered(&first.fixtures);
            }
        }
        let selected = self.selected_fx.filter(|(s, _)| *s == id).map(|(_, f)| f);
        let has_steps = scene.has_steps();
        let mut steps = scene.steps.clone();
        let steps_before = steps.clone();
        let color = scene.color.map_or_else(
            || theme::zone_color(cx.engine.show(), scene.zone),
            theme::rgb,
        );
        let playing_step = cx
            .engine
            .progress(id)
            .filter(|p| !p.finished)
            .map(|p| p.step);
        let mut edit_step = cx.state.edit_step;
        // Preview per COLOR FX: current color of every beam
        let cycles = cx.engine.fx_cycles(id);
        let previews: std::collections::HashMap<u32, FxPreview> = effects
            .iter()
            .filter(|fx| !fx.kind.exclusive())
            .map(|fx| (fx.id, FxPreview::of(cx.engine.show(), fx, cycles)))
            .collect();
        if !previews.is_empty() {
            ui.ctx().request_repaint();
        }
        let mut color_changes: Vec<dmxcraft_core::Effect> = Vec::new();
        let mut move_hold: Option<(u32, [f32; 2])> = None;
        let groups = cx.engine.show().scene_groups.clone();
        let palettes = &mut self.palettes;
        // SUPER SCENE: selected block and its name
        // one block – or one linked group (video + its sound): then the video
        let block = scene
            .timeline()
            .and_then(|tl| single_block(tl, &cx.state.timeline_sel))
            .and_then(|cid| {
                let tl = scene.timeline()?;
                let (_, c) = tl.clip(cid)?;
                let name = match &c.content {
                    dmxcraft_core::timeline::ClipContent::Scene(s) => cx
                        .engine
                        .show()
                        .scene(*s)
                        .map_or_else(|| t("Deleted").to_string(), |s| s.name.clone()),
                    dmxcraft_core::timeline::ClipContent::Audio { path, .. }
                    | dmxcraft_core::timeline::ClipContent::Video { path, .. } => {
                        std::path::Path::new(path)
                            .file_name()
                            .map_or(path.clone(), |n| n.to_string_lossy().into_owned())
                    }
                    dmxcraft_core::timeline::ClipContent::Obs(a) => super::timeline::obs_label(a),
                    dmxcraft_core::timeline::ClipContent::Group { group, .. } => cx
                        .engine
                        .show()
                        .scene_group(*group)
                        .map_or_else(|| t("Deleted").to_string(), |g| g.name.clone()),
                };
                Some((cid, name, tl.beat_ms()))
            });
        // Audio block: trigger display
        let trig_cx = block.as_ref().and_then(|(cid, _, _)| {
            super::audio_trigger_ui::trigger_cx(cx.engine, cx.state, ui.ctx(), id, *cid)
        });
        let mut trig_action = super::audio_trigger_ui::TriggerAction::None;
        let obs_lists = (cx.state.obs.scenes(), cx.state.obs.inputs());
        let mut step_clicked = false;
        let mut toggle = None;
        let mut select = None;
        let mut menu_delete = None;
        ui.allocate_ui(Vec2::new(ui.available_width(), list_h), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("fx_list")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for fx in &effects {
                        let (head, chevron, close) =
                            effect_header(ui, fx, selected == Some(fx.id), !locked);
                        if chevron.clicked() {
                            toggle = Some(fx.id);
                        }
                        if close.is_some_and(|c| c.clicked()) {
                            menu_delete = Some(fx.id);
                        }
                        if head.clicked() {
                            select = Some(fx.id);
                        }
                        if !locked {
                            crate::ui::widgets::menu::context(&head, |ui| {
                                use crate::ui::widgets::menu::{self, MenuItem};
                                menu::begin(ui);
                                menu::title(ui, fx.kind.label());
                                let collapse = if fx.collapsed {
                                    t("Expand")
                                } else {
                                    t("Collapse")
                                };
                                if menu::item(ui, MenuItem::new(collapse)) {
                                    toggle = Some(fx.id);
                                }
                                menu::separator(ui);
                                if menu::item(
                                    ui,
                                    MenuItem::new(t("Delete effect")).icon(Icon::Trash).danger(),
                                ) {
                                    menu_delete = Some(fx.id);
                                }
                            });
                        }
                        if !fx.collapsed {
                            let is_steps = fx.kind == EffectKind::Steps;
                            egui::Frame::new()
                                .fill(theme::raised())
                                .inner_margin(egui::Margin::same(if is_steps { 4 } else { 8 }))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    if is_steps {
                                        if steps_table(
                                            ui,
                                            &mut steps,
                                            &mut edit_step,
                                            playing_step,
                                            color,
                                            !locked,
                                        ) {
                                            step_clicked = true;
                                        }
                                    } else if fx.kind == EffectKind::SuperScene {
                                        let mut e = fx.clone();
                                        if let Some(tl) = e.timeline.as_mut() {
                                            if block_props(
                                                ui,
                                                tl,
                                                block.as_ref(),
                                                color,
                                                !locked,
                                                trig_cx.as_ref(),
                                                &mut trig_action,
                                                &obs_lists,
                                                &groups,
                                            ) {
                                                color_changes.push(e);
                                            }
                                        }
                                    } else if let Some(pv) = previews.get(&fx.id) {
                                        let mut e = fx.clone();
                                        let mut hold = None;
                                        let changed = super::fx_ui::effect_body(
                                            ui,
                                            &mut e,
                                            super::fx_ui::BodyCtx {
                                                hold: &mut hold,
                                                preview: &pv.colors,
                                                positions: &pv.positions,
                                                cycles,
                                                kinds: &pv.kinds,
                                                accent: color,
                                                enabled: !locked,
                                                palettes,
                                            },
                                        );
                                        if let Some(h) = hold {
                                            move_hold = Some((e.id, h));
                                        }
                                        if changed {
                                            color_changes.push(e);
                                        }
                                    } else {
                                        ui.label(
                                            egui::RichText::new(t("Settings coming soon"))
                                                .color(theme::text_faint()),
                                        );
                                    }
                                });
                        }
                        ui.add_space(2.0);
                    }
                });
        });
        match (trig_action, block.as_ref()) {
            (super::audio_trigger_ui::TriggerAction::Pick(tr), Some((cid, _, _))) => {
                cx.state.trigger_pick = Some((id, *cid, tr));
            }
            (super::audio_trigger_ui::TriggerAction::CancelPick, _) => {
                cx.state.trigger_pick = None;
            }
            _ => {}
        }
        if steps != steps_before && !locked {
            if let Some(sc) = cx.engine.show_mut().scene_mut(id) {
                for (dst, src) in sc.steps.iter_mut().zip(&steps) {
                    dst.hold_ms = src.hold_ms;
                    dst.fade_ms = src.fade_ms;
                }
            }
        }
        cx.state.edit_step = edit_step;
        self.palettes.windows(ui.ctx());
        if let Some(e) = self.palettes.error.take() {
            cx.state.error(e);
        }
        if !locked {
            if let Some(sc) = cx.engine.show_mut().scene_mut(id) {
                if cx.state.programmer_mode != crate::state::ProgrammerMode::Live {
                    cx.state.move_hold = move_hold.map(|(fx, h)| (id, fx, h));
                }
                for changed in color_changes {
                    if let Some(fx) = sc.effects.iter_mut().find(|e| e.id == changed.id) {
                        *fx = changed;
                    }
                }
            }
        }
        if step_clicked {
            // a step belongs to the STEPS effect: select it too
            if let Some(fx) = effects.iter().find(|e| e.kind == EffectKind::Steps) {
                select = Some(fx.id);
            }
        }
        if let Some(f) = select {
            self.selected_fx = Some((id, f));
            self.step_focus = step_clicked;
            cx.state.focus = crate::state::Focus::Effect(id, f);
            // Clicking an effect selects its fixtures (FX button of the stage)
            if let Some(fx) = effects
                .iter()
                .find(|e| e.id == f && e.kind != EffectKind::Steps)
            {
                cx.state.fx_target = Some((id, f));
                cx.state.select_ordered(&fx.fixtures);
            } else {
                cx.state.fx_target = None;
            }
        }
        // Delete removes the clicked effect (not the scene)
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        let mut delete = false;
        if let (false, false, crate::state::Focus::Effect(s, f)) = (typing, locked, cx.state.focus)
        {
            if s == id
                && selected == Some(f)
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
            {
                delete = true;
            }
        }
        if let (Some(f), false) = (toggle, locked) {
            if let Some(fx) = cx
                .engine
                .show_mut()
                .scene_mut(id)
                .and_then(|s| s.effects.iter_mut().find(|e| e.id == f))
            {
                fx.collapsed = !fx.collapsed;
            }
        }
        // Context menu “Delete effect”
        if let (Some(f), false) = (menu_delete, locked) {
            if let Some(scene) = cx.engine.show_mut().scene_mut(id) {
                self.selected_fx = remove_effect(scene, f, cx.state).map(|e| (id, e));
                cx.state.focus = crate::state::Focus::Scenes;
            }
            return;
        }

        // Toolbar: fixed bar at the bottom edge, buttons centered
        ui.painter().line_segment(
            [bar_rect.left_top(), bar_rect.right_top()],
            Stroke::new(1.0, theme::border()),
        );
        ui.allocate_rect(bar_rect, Sense::hover());
        let mut bar = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(bar_rect.with_min_y(bar_rect.top() + 1.0))
                .layout(egui::Layout::right_to_left(egui::Align::Center)),
        );
        {
            let ui = &mut bar;
            {
                let on = !locked;
                if ui
                    .add(
                        IconButton::new(Icon::Export)
                            .size(30.0)
                            .flat()
                            .enabled(on && self.fx_dialog.is_none()),
                    )
                    .on_hover_text(if has_steps {
                        t("Export steps")
                    } else {
                        t("Export effects")
                    })
                    .clicked()
                {
                    self.start_dialog(ui.ctx(), id, true);
                }
                if ui
                    .add(
                        IconButton::new(Icon::Import)
                            .size(30.0)
                            .flat()
                            .enabled(on && self.fx_dialog.is_none()),
                    )
                    .on_hover_text(if has_steps {
                        t("Import steps")
                    } else {
                        t("Import effects")
                    })
                    .clicked()
                {
                    self.start_dialog(ui.ctx(), id, false);
                }
                // Step scene: delete affects the selected step as long as there is more
                // than one and a step was clicked last
                let delete_step = has_steps && self.step_focus && steps.len() > 1;
                let clicked_delete = ui
                    .add(
                        IconButton::new(Icon::Trash)
                            .size(30.0)
                            .flat()
                            .enabled(on && (selected.is_some() || delete_step)),
                    )
                    .on_hover_text(if delete_step {
                        t("Delete selected step")
                    } else {
                        t("Delete selected effect")
                    })
                    .clicked();
                if clicked_delete || delete {
                    if delete_step {
                        if let Some(scene) = cx.engine.show_mut().scene_mut(id) {
                            if scene.remove_step(cx.state.edit_step) {
                                cx.state.edit_step =
                                    cx.state.edit_step.min(scene.steps.len().saturating_sub(1));
                                cx.state.edit_reload = true;
                            }
                        }
                    } else if let (Some(f), Some(scene)) =
                        (selected, cx.engine.show_mut().scene_mut(id))
                    {
                        self.selected_fx = remove_effect(scene, f, cx.state).map(|e| (id, e));
                        cx.state.focus = crate::state::Focus::Scenes;
                    }
                }
                let has_super = effects.iter().any(|e| e.kind == EffectKind::SuperScene);
                let (tip, icon) = if has_steps {
                    (t("Add step"), Icon::Plus)
                } else if has_super {
                    (t("Add track"), Icon::Plus)
                } else {
                    (t("Add effect"), Icon::Plus)
                };
                if ui
                    .add(IconButton::new(icon).size(30.0).flat().enabled(on))
                    .on_hover_text(tip)
                    .clicked()
                {
                    if has_super {
                        if let Some(tl) = cx
                            .engine
                            .show_mut()
                            .scene_mut(id)
                            .and_then(|s| s.timeline_mut())
                        {
                            tl.add_track(None);
                        }
                    } else if has_steps {
                        // new step after the edited one (copy of its values)
                        if let Some(scene) = cx.engine.show_mut().scene_mut(id) {
                            cx.state.edit_step = scene.insert_step(cx.state.edit_step);
                            cx.state.edit_reload = true;
                        }
                        self.step_focus = true;
                    } else {
                        self.adding = true;
                    }
                }
            }
        }
    }

    fn start_dialog(&mut self, ctx: &egui::Context, scene: SceneId, export: bool) {
        let (tx, rx) = channel();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let dialog = rfd::FileDialog::new().add_filter(t("DMXCraft effects"), &[FX_EXT]);
            let result = if export {
                dialog
                    .set_title(t("Export effects"))
                    .set_file_name(format!("Effects.{FX_EXT}"))
                    .save_file()
                    .map(|p| p.with_extension(FX_EXT))
            } else {
                dialog.set_title(t("Import effects")).pick_file()
            };
            let _ = tx.send(result);
            ctx.request_repaint();
        });
        self.fx_dialog = Some(FxDialog { export, scene, rx });
    }

    /// Process the result of an import/export dialog.
    fn poll_dialog(&mut self, cx: &mut ViewCtx<'_>) {
        let Some(d) = &self.fx_dialog else {
            return;
        };
        let Ok(result) = d.rx.try_recv() else {
            return;
        };
        let (export, scene) = (d.export, d.scene);
        self.fx_dialog = None;
        let Some(path) = result else {
            return;
        };
        if export {
            // selected effect, otherwise all effects of the scene
            let Some(s) = cx.engine.show().scene(scene) else {
                return;
            };
            let sel = self
                .selected_fx
                .filter(|(sc, _)| *sc == scene)
                .map(|(_, f)| f);
            let effects: Vec<Effect> = match sel {
                Some(f) => s.effects.iter().filter(|e| e.id == f).cloned().collect(),
                None => s.effects.clone(),
            };
            // STEPS takes the steps (values and times) along
            let steps = if effects.is_empty() || effects.iter().any(|e| e.kind == EffectKind::Steps)
            {
                s.steps.clone()
            } else {
                Vec::new()
            };
            let result = serde_json::to_string_pretty(&FxFile { effects, steps })
                .map_err(|e| e.to_string())
                .and_then(|json| std::fs::write(&path, json).map_err(|e| e.to_string()));
            match result {
                Ok(()) => cx
                    .state
                    .info(tr!("Effects exported: {path}", path = path.display())),
                Err(e) => cx.state.error(e),
            }
        } else {
            let read = std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| FxFile::parse(&s));
            match read {
                Ok(file) => {
                    if let Some(s) = cx.engine.show_mut().scene_mut(scene) {
                        match import_fx(s, file) {
                            Ok(msg) => {
                                cx.state.edit_reload = true;
                                cx.state.info(msg);
                            }
                            Err(e) => cx.state.error(e),
                        }
                    }
                }
                Err(e) => cx.state.error(e),
            }
        }
    }
}

/// BLOCK PROPERTIES of a SUPER SCENE: start, duration, fade in/out, tempo
/// and loop of the selected block. `true` on change.
#[allow(clippy::too_many_arguments)]
fn block_props(
    ui: &mut egui::Ui,
    tl: &mut dmxcraft_core::timeline::Timeline,
    block: Option<&(u32, String, f64)>,
    accent: Color32,
    enabled: bool,
    trig: Option<&super::audio_trigger_ui::TriggerCx>,
    trig_action: &mut super::audio_trigger_ui::TriggerAction,
    obs: &(Vec<String>, Vec<String>),
    groups: &[dmxcraft_core::scene_group::SceneGroup],
) -> bool {
    let before = tl.clone();
    let caption = |ui: &mut egui::Ui, s: &str| {
        ui.label(
            egui::RichText::new(s.to_uppercase())
                .font(theme::caption())
                .color(theme::text_dim()),
        );
    };
    caption(ui, t("Block properties"));
    let Some((cid, name, beat)) = block else {
        ui.label(egui::RichText::new(t("No selection")).color(theme::text_faint()));
        return false;
    };
    let Some(c) = tl.clip_mut(*cid) else {
        return false;
    };
    ui.label(egui::RichText::new(name).color(accent).strong());
    ui.add_space(theme::SP_XS);
    let time_row = |ui: &mut egui::Ui, label: &str, salt: &str, v: &mut u32| {
        row(ui, label, |ui| {
            time_field(ui, salt, v, enabled, false, accent);
        });
    };
    let mut start = c.start_ms.min(u64::from(u32::MAX)) as u32;
    let mut dur = c.duration_ms.min(u64::from(u32::MAX)) as u32;
    time_row(ui, t("Start"), "blk_start", &mut start);
    time_row(ui, t("Duration"), "blk_dur", &mut dur);
    time_row(ui, t("Fade in"), "blk_fin", &mut c.fade_in_ms);
    time_row(ui, t("Fade out"), "blk_fout", &mut c.fade_out_ms);
    if c.fade_in_ms > 0 || c.fade_out_ms > 0 {
        row(ui, t("Fade curve"), |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                egui::ComboBox::from_id_salt("blk_fade_curve")
                    .selected_text(t(c.fade_curve.label()))
                    .width(160.0)
                    .show_ui(ui, |ui| {
                        for f in dmxcraft_core::timeline::FadeCurve::ALL {
                            ui.selectable_value(&mut c.fade_curve, f, t(f.label()));
                        }
                    });
            });
        });
    }
    if dmxcraft_core::timeline::Timeline::is_light(c) {
        row(ui, t("Layer"), |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                egui::ComboBox::from_id_salt("blk_layer")
                    .selected_text(t(c.layer.label()))
                    .width(160.0)
                    .show_ui(ui, |ui| {
                        for l in dmxcraft_core::timeline::Layer::ALL {
                            ui.selectable_value(&mut c.layer, l, t(l.label()));
                        }
                    });
            });
        });
    }
    c.start_ms = u64::from(start);
    c.duration_ms = u64::from(dur).max(40);
    row(ui, t("Fit to tempo"), |ui| {
        ui.add(crate::ui::widgets::Toggle::new(&mut c.tempo_sync, "").enabled(enabled));
    });
    row(ui, t("Endless loop"), |ui| {
        ui.add(crate::ui::widgets::Toggle::new(&mut c.looped, "").enabled(enabled));
    });
    if let dmxcraft_core::timeline::ClipContent::Group { group, members } = &mut c.content {
        let mut linked = members.is_none();
        row(ui, t("Linked to the group"), |ui| {
            ui.add(crate::ui::widgets::Toggle::new(&mut linked, "").enabled(enabled))
                .on_hover_text(t(
                    "On: changes to the group apply here too. Off: this block keeps its own copy.",
                ));
        });
        if linked != members.is_none() {
            *members = if linked {
                None
            } else {
                Some(
                    groups
                        .iter()
                        .find(|g| g.id == *group)
                        .map(|g| g.members.clone())
                        .unwrap_or_default(),
                )
            };
        }
    }
    if dmxcraft_core::timeline::Timeline::is_light(c) {
        ui.add_space(theme::SP_S);
        caption(ui, t("Prepare"));
        let mut on = c.prepare.is_some();
        row(ui, t("Pre-position"), |ui| {
            ui.add(crate::ui::widgets::Toggle::new(&mut on, "").enabled(enabled))
                .on_hover_text(t(
                    "Before the block the fixtures already move to its starting position (no light)",
                ));
        });
        if on != c.prepare.is_some() {
            c.prepare = on.then(dmxcraft_core::timeline::Prepare::default);
        }
        if let Some(p) = c.prepare.as_mut() {
            time_row(ui, t("Lead time"), "blk_lead", &mut p.lead_ms);
            p.lead_ms = p.lead_ms.clamp(100, 30_000);
            row(ui, t("Only while dark"), |ui| {
                ui.add(crate::ui::widgets::Toggle::new(&mut p.only_dark, "").enabled(enabled));
            });
            // label above, chips below (they wrap in narrow sidebars)
            ui.label(egui::RichText::new(t("Channels")).color(theme::text_dim()));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(theme::SP_XS, theme::SP_XS);
                for (v, label) in [
                    (&mut p.position, t("Pan/Tilt")),
                    (&mut p.beam, t("Focus/Zoom")),
                    (&mut p.gobo, t("Gobo")),
                    (&mut p.color_wheel, t("Color wheel")),
                ] {
                    if chip(ui, label, *v, accent, enabled).clicked() {
                        *v = !*v;
                    }
                }
            });
        }
    }
    if c.tempo_sync {
        // Round the length to whole beats
        let beats = (c.duration_ms as f64 / beat).round().max(1.0);
        c.duration_ms = (beats * beat).round() as u64;
    }
    if let (Some(tcx), dmxcraft_core::timeline::ClipContent::Audio { triggers, .. }) =
        (trig, &mut c.content)
    {
        super::audio_trigger_ui::triggers_ui(ui, triggers, tcx, accent, enabled, trig_action);
    }
    obs_props(ui, &mut c.content, obs, enabled);
    *tl != before
}

/// Small on/off chip in the color of the scene.
fn chip(
    ui: &mut egui::Ui,
    label: &str,
    on: bool,
    accent: Color32,
    enabled: bool,
) -> egui::Response {
    let galley = ui.painter().layout_no_wrap(
        label.to_string(),
        egui::FontId::proportional(theme::FS_CAPTION),
        theme::text(),
    );
    let size = galley.size() + egui::vec2(12.0, 8.0);
    let sense = if enabled {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(size, sense);
    let (fill, text) = if on {
        (accent.gamma_multiply(0.3), accent)
    } else if resp.hovered() && enabled {
        (theme::raised_hover(), theme::text_dim())
    } else {
        (theme::raised(), theme::text_dim())
    };
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S as f32, fill);
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::proportional(theme::FS_CAPTION),
        text,
    );
    resp
}

/// Video and OBS blocks: choose the target in OBS.
fn obs_props(
    ui: &mut egui::Ui,
    content: &mut dmxcraft_core::timeline::ClipContent,
    (scenes, inputs): &(Vec<String>, Vec<String>),
    enabled: bool,
) {
    use dmxcraft_core::obs::ObsAction;
    use dmxcraft_core::timeline::ClipContent;
    match content {
        ClipContent::Video {
            obs_scene, source, ..
        } => {
            row(ui, t("OBS scene"), |ui| {
                name_pick(
                    ui,
                    "blk_obs_scene",
                    obs_scene,
                    scenes,
                    Some(t("None")),
                    enabled,
                );
            });
            row(ui, t("Media source"), |ui| {
                name_pick(
                    ui,
                    "blk_obs_src",
                    source,
                    inputs,
                    Some(t("Default")),
                    enabled,
                );
            });
        }
        ClipContent::Obs(action) => {
            let mut kind = match action {
                ObsAction::Scene(_) => 0,
                ObsAction::Source { visible: true, .. } => 1,
                ObsAction::Source { visible: false, .. } => 2,
            };
            let before = kind;
            row(ui, t("Action"), |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    egui::ComboBox::from_id_salt("blk_obs_kind")
                        .selected_text(
                            [t("Switch scene"), t("Show source"), t("Hide source")][kind],
                        )
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut kind, 0, t("Switch scene"));
                            ui.selectable_value(&mut kind, 1, t("Show source"));
                            ui.selectable_value(&mut kind, 2, t("Hide source"));
                        });
                });
            });
            if kind != before {
                let scene = match action {
                    ObsAction::Scene(s) | ObsAction::Source { scene: s, .. } => s.clone(),
                };
                let source = match action {
                    ObsAction::Source { source, .. } => source.clone(),
                    ObsAction::Scene(_) => String::new(),
                };
                *action = match kind {
                    0 => ObsAction::Scene(scene),
                    k => ObsAction::Source {
                        scene,
                        source,
                        visible: k == 1,
                    },
                };
            }
            match action {
                ObsAction::Scene(scene) => {
                    row(ui, t("OBS scene"), |ui| {
                        name_pick(ui, "blk_obs_scene", scene, scenes, None, enabled);
                    });
                }
                ObsAction::Source { scene, source, .. } => {
                    row(ui, t("OBS scene"), |ui| {
                        name_pick(ui, "blk_obs_scene", scene, scenes, None, enabled);
                    });
                    row(ui, t("Source"), |ui| {
                        name_pick(ui, "blk_obs_src", source, inputs, None, enabled);
                    });
                }
            }
        }
        _ => {}
    }
}

/// Pick names from OBS's list; without a connection type them in.
fn name_pick(
    ui: &mut egui::Ui,
    salt: &str,
    value: &mut String,
    options: &[String],
    none: Option<&str>,
    enabled: bool,
) {
    ui.add_enabled_ui(enabled, |ui| {
        if options.is_empty() {
            ui.add(
                egui::TextEdit::singleline(value)
                    .desired_width(140.0)
                    .hint_text(none.unwrap_or("")),
            );
            return;
        }
        let shown = if value.is_empty() {
            none.unwrap_or("").to_string()
        } else {
            value.clone()
        };
        egui::ComboBox::from_id_salt(salt)
            .selected_text(shown)
            .width(140.0)
            .show_ui(ui, |ui| {
                if let Some(n) = none {
                    ui.selectable_value(value, String::new(), n);
                }
                for o in options {
                    ui.selectable_value(value, o.clone(), o);
                }
            });
    });
}

/// Put the rectangle of a new mapping around its fixtures.
fn fit_mapping(show: &mut dmxcraft_core::Show, scene: SceneId, fx: u32) {
    let Some(e) = show
        .scene(scene)
        .and_then(|s| s.effects.iter().find(|e| e.id == fx))
        .filter(|e| e.mapping.is_some())
    else {
        return;
    };
    let beams = dmxcraft_core::fx::feature::fixture_beams(show, &e.fixtures);
    if beams.is_empty() {
        return;
    }
    let (mut lo, mut hi) = ([f32::MAX; 2], [f32::MIN; 2]);
    for b in &beams {
        for a in 0..2 {
            lo[a] = lo[a].min(b.pos[a]);
            hi[a] = hi[a].max(b.pos[a]);
        }
    }
    let center = [(lo[0] + hi[0]) / 2.0, (lo[1] + hi[1]) / 2.0];
    let size = [
        ((hi[0] - lo[0]) / 2.0 + 0.05).max(0.06),
        ((hi[1] - lo[1]) / 2.0 + 0.05).max(0.06),
    ];
    if let Some(m) = show
        .scene_mut(scene)
        .and_then(|s| s.effects.iter_mut().find(|e| e.id == fx))
        .and_then(|e| e.mapping.as_mut())
    {
        m.center = center;
        m.size = size;
    }
}

/// Preview of an effect for the sidebar.
struct FxPreview {
    /// Color or level (gray) per beam.
    colors: Vec<[u8; 3]>,
    /// MOVE FX: pan/tilt per beam.
    positions: Vec<[f32; 2]>,
    /// Channel types of the effect fixtures.
    kinds: Vec<dmxcraft_core::ChannelKind>,
}

impl FxPreview {
    fn of(show: &dmxcraft_core::Show, fx: &dmxcraft_core::Effect, cycles: f64) -> Self {
        use dmxcraft_core::fx;
        let mut kinds: Vec<dmxcraft_core::ChannelKind> = Vec::new();
        for f in fx.fixtures.iter().filter_map(|id| show.fixture(*id)) {
            for c in show
                .type_of(f)
                .map(|t| t.channels.as_slice())
                .unwrap_or(&[])
            {
                if !kinds.contains(&c.kind) {
                    kinds.push(c.kind);
                }
            }
        }
        let fbeams = fx::feature::fixture_beams(show, &fx.fixtures);
        let colors = match fx.kind {
            EffectKind::ColorFx => fx
                .color
                .as_ref()
                .map(|c| fx::render(c, &fx::beams(show, &fx.fixtures), cycles))
                .unwrap_or_default(),
            k if k.uses_features() => fx::feature_levels(fx, &fbeams, cycles)
                .into_iter()
                .map(|l| [(l.unwrap_or(0.0).clamp(0.0, 1.0) * 255.0).round() as u8; 3])
                .collect(),
            _ => Vec::new(),
        };
        let positions = fx
            .motion
            .as_ref()
            .map(|m| {
                let n = fbeams.len();
                (0..n).map(|i| m.position(i, n, cycles)).collect()
            })
            .unwrap_or_default();
        Self {
            colors,
            positions,
            kinds,
        }
    }
}

/// Content of a `.dmxfx` file. Older files are a plain list of effects.
#[derive(serde::Serialize, serde::Deserialize)]
struct FxFile {
    effects: Vec<Effect>,
    /// Steps of a STEPS effect.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    steps: Vec<dmxcraft_core::Step>,
}

impl FxFile {
    fn parse(json: &str) -> Result<Self, String> {
        serde_json::from_str::<FxFile>(json)
            .or_else(|_| {
                serde_json::from_str::<Vec<Effect>>(json).map(|effects| FxFile {
                    effects,
                    steps: Vec::new(),
                })
            })
            .map_err(|e| e.to_string())
    }
}

/// Take imported effects into the scene. STEPS only stands alone: in a step
/// scene the steps are appended, a scene without effects becomes a step
/// scene.
fn import_fx(s: &mut dmxcraft_core::Scene, file: FxFile) -> Result<String, String> {
    // exported STATIC scene: take over its values
    if file.effects.is_empty() {
        if !s.effects.is_empty() {
            return Err(t("A static scene can only be imported into a STATIC scene").into());
        }
        if let Some(st) = file.steps.into_iter().next() {
            s.steps = vec![st];
        }
        return Ok(t("Scene imported").into());
    }
    let with_steps = file.effects.iter().any(|e| e.kind == EffectKind::Steps);
    if with_steps {
        if s.has_steps() {
            let n = file.steps.len();
            s.steps.extend(file.steps);
            return Ok(tr!("{n} step(s) imported", n = n));
        }
        if !s.effects.is_empty() {
            return Err(t("STEPS must be the only effect of a scene").into());
        }
        s.add_effect(EffectKind::Steps);
        let n = file.steps.len();
        if !file.steps.is_empty() {
            s.steps = file.steps;
        } else if s.steps.is_empty() {
            s.steps.push(dmxcraft_core::Step::new(Vec::new()));
        }
        return Ok(tr!("{n} step(s) imported", n = n));
    }
    if s.has_steps() {
        return Err(t("A step scene cannot take further effects").into());
    }
    let n = file.effects.len();
    for fx in file.effects {
        // Take over settings and fixtures, only with a new ID
        let id = s.add_effect(fx.kind);
        if let Some(new) = s.effects.iter_mut().find(|e| e.id == id) {
            *new = dmxcraft_core::Effect { id, ..fx };
        }
    }
    Ok(tr!("{n} effect(s) imported", n = n))
}

/// Tiles of the effect kinds in two columns; returns the clicked one.
fn effect_tiles(ui: &mut egui::Ui, enabled: bool, allow_steps: bool) -> Option<EffectKind> {
    let gap = theme::SP_S;
    let w = ((ui.available_width() - gap) / 2.0).min(120.0);
    let h = w * 0.72;
    let mut pick = None;
    for row in EffectKind::ALL.chunks(2) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            let total = w * row.len() as f32 + gap * (row.len() as f32 - 1.0);
            ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
            for kind in row {
                // STEPS and SUPER SCENE only as the only effect of a scene
                let enabled = enabled && (allow_steps || !kind.exclusive());
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());
                let hot = enabled && resp.hovered();
                let p = ui.painter();
                p.rect_filled(
                    rect,
                    theme::R_S,
                    if hot {
                        theme::raised_hover()
                    } else {
                        theme::bg()
                    },
                );
                p.rect_stroke(
                    rect,
                    theme::R_S,
                    Stroke::new(
                        if hot { 2.0 } else { 1.5 },
                        if hot {
                            theme::accent()
                        } else {
                            theme::border()
                        },
                    ),
                    egui::StrokeKind::Inside,
                );
                let galley = p.layout(
                    kind.label().to_string(),
                    theme::heading_font(),
                    if enabled {
                        theme::text()
                    } else {
                        theme::text_faint()
                    },
                    w - 12.0,
                );
                let pos = rect.center() - galley.size() * 0.5;
                p.galley(pos, galley, theme::text());
                if enabled && resp.clicked() {
                    pick = Some(*kind);
                }
            }
        });
        ui.add_space(gap);
    }
    pick
}

/// Table of steps (compact, like a table): number, hold time, fade time. A
/// click on a row selects the step for editing (the faders then show its
/// values). `true` if a row was clicked.
fn steps_table(
    ui: &mut egui::Ui,
    steps: &mut [dmxcraft_core::Step],
    edit_step: &mut usize,
    playing: Option<usize>,
    color: Color32,
    enabled: bool,
) -> bool {
    use crate::ui::widgets::time_field::{time_field_compact, COMPACT_W};
    const ROW_H: f32 = 24.0;
    const GAP: f32 = 1.0;
    let mut clicked = false;
    let full_w = ui.available_width();
    // Columns: number narrow, the times share the rest
    let num_w = (full_w * 0.26).clamp(44.0, 80.0);
    let field_w = ((full_w - num_w - 2.0 * GAP) / 2.0).max(COMPACT_W + 4.0);
    let cols = [num_w, field_w, field_w];
    let left = ui.cursor().min.x;
    let cells = |top: f32| {
        let mut x = left;
        cols.map(|w| {
            let r = Rect::from_min_size(pos2(x, top), Vec2::new(w, ROW_H));
            x += w + GAP;
            r
        })
    };
    ui.spacing_mut().item_spacing.y = GAP;
    // Header row
    let (head, _) = ui.allocate_exact_size(Vec2::new(full_w, ROW_H), Sense::hover());
    for (r, text) in cells(head.top())
        .into_iter()
        .zip([t("Step"), t("Hold time"), t("Fade time")])
    {
        ui.painter().rect_filled(r, 0.0, theme::raised_hover());
        ui.painter().text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            text,
            theme::caption(),
            theme::text(),
        );
    }
    for (i, step) in steps.iter_mut().enumerate() {
        let (row, resp) = ui.allocate_exact_size(Vec2::new(full_w, ROW_H), Sense::click());
        let sel = *edit_step == i;
        let fill = if sel {
            theme::accent_dim()
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::bg()
        };
        let [num, hold, fade] = cells(row.top());
        let p = ui.painter().clone();
        for r in [num, hold, fade] {
            p.rect_filled(r, 0.0, fill);
        }
        if playing == Some(i) {
            // running step: bar in the scene color
            p.rect_filled(
                Rect::from_min_size(num.min, Vec2::new(3.0, num.height())),
                0.0,
                color,
            );
        }
        p.text(
            num.center(),
            egui::Align2::CENTER_CENTER,
            (i + 1).to_string(),
            theme::body(),
            if sel {
                theme::text()
            } else {
                theme::text_dim()
            },
        );
        ui.push_id(("step", i), |ui| {
            for (salt, cell, ms) in [
                ("hold", hold, &mut step.hold_ms),
                ("fade", fade, &mut step.fade_ms),
            ] {
                let r = Rect::from_center_size(cell.center(), Vec2::new(COMPACT_W, ROW_H - 2.0));
                let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(r));
                if time_field_compact(&mut cui, salt, ms, enabled, color) {
                    *edit_step = i;
                    clicked = true;
                }
            }
        });
        if resp.clicked() {
            *edit_step = i;
            clicked = true;
        }
    }
    clicked
}

/// Header row of an effect: collapse arrow, name; response for selection
/// and arrow.
/// Remove an effect from the scene. Returns the effect selected afterwards
/// (the last remaining one).
fn remove_effect(
    scene: &mut dmxcraft_core::Scene,
    f: u32,
    state: &mut crate::state::AppState,
) -> Option<u32> {
    let was_steps = scene
        .effects
        .iter()
        .any(|e| e.id == f && e.kind == EffectKind::Steps);
    // no effects left -> STATIC again
    scene.effects.retain(|e| e.id != f);
    if was_steps {
        // with STEPS all steps and their values disappear too: what remains is an
        // empty STATIC scene
        scene.steps = vec![dmxcraft_core::Step::new(Vec::new())];
        state.edit_step = 0;
        state.edit_reload = true;
    }
    if state.fx_target.is_some_and(|(_, e)| e == f) {
        state.fx_target = None;
    }
    scene.effects.last().map(|e| e.id)
}

/// Header row of an effect: arrow to expand/collapse, name and an X on the
/// right to delete (only if `removable`). Returns (header, arrow, X).
fn effect_header(
    ui: &mut egui::Ui,
    fx: &Effect,
    selected: bool,
    removable: bool,
) -> (egui::Response, egui::Response, Option<egui::Response>) {
    let (rect, head) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::click());
    let arrow_rect = Rect::from_min_size(rect.min, Vec2::new(28.0, rect.height()));
    let chevron = ui.interact(
        arrow_rect,
        ui.make_persistent_id(("fx_chevron", fx.id)),
        Sense::click(),
    );
    let close_rect = Rect::from_min_size(
        pos2(rect.right() - 30.0, rect.top()),
        Vec2::new(30.0, rect.height()),
    );
    let close = removable.then(|| {
        ui.interact(
            close_rect,
            ui.make_persistent_id(("fx_close", fx.id)),
            Sense::click(),
        )
        .on_hover_text(t("Delete effect"))
    });
    let p = ui.painter();
    let fill = if selected {
        theme::accent_dim()
    } else if head.hovered() {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S, fill);
    let icon = if fx.collapsed {
        Icon::ChevronRight
    } else {
        Icon::ChevronDown
    };
    icon.paint(
        p,
        Rect::from_center_size(arrow_rect.center(), Vec2::splat(12.0)),
        theme::text_dim(),
    );
    p.text(
        pos2(arrow_rect.right(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        fx.kind.label(),
        theme::heading_font(),
        if selected {
            theme::text()
        } else {
            theme::text_dim()
        },
    );
    if let Some(c) = &close {
        let hot = c.hovered();
        if hot {
            p.rect_filled(close_rect.shrink(3.0), theme::R_S, theme::raised_hover());
        }
        Icon::Close.paint(
            p,
            Rect::from_center_size(close_rect.center(), Vec2::splat(12.0)),
            if hot {
                theme::DANGER
            } else {
                theme::text_dim()
            },
        );
    }
    (head, chevron, close)
}

/// What the selection window is open for.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PickTarget {
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
fn advanced(
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

fn banks_of(scope: &BankScope) -> std::collections::BTreeSet<u32> {
    match scope {
        BankScope::Banks(list) => list.iter().map(|z| z.0).collect(),
        _ => Default::default(),
    }
}

/// A bank in the selection window: ID, name, color and its scenes.
type PickColumn = (ZoneId, String, Color32, Vec<(SceneId, String, Color32)>);

/// Selection window: all banks as columns with their scenes. Depending on
/// the purpose scenes or banks (header) are selected, several possible.
fn picker_window(ctx: &egui::Context, e: &mut Engine, picker: &mut Option<Picker>) {
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

fn toggle(set: &mut std::collections::BTreeSet<u32>, v: u32) {
    if !set.remove(&v) {
        set.insert(v);
    }
}

/// Tile in the selection window; `true` when clicked.
fn pick_tile(
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
fn row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        ui.label(egui::RichText::new(label).color(theme::text_dim()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add);
    });
}

/// Bank reference as a short selection on the right; for “banks” a button
/// for the selection window below. `true` = open the window.
fn scope_row(ui: &mut egui::Ui, label: &str, id: (&str, u32), scope: &mut BankScope) -> bool {
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

/// The block whose settings the panel shows: the only selected one, or of a
/// selected linked group (video and its sound) the video.
fn single_block(
    tl: &dmxcraft_core::timeline::Timeline,
    sel: &std::collections::BTreeSet<u32>,
) -> Option<u32> {
    use dmxcraft_core::timeline::ClipContent;
    let clips: Vec<&dmxcraft_core::timeline::Clip> = sel
        .iter()
        .filter_map(|id| tl.clip(*id).map(|(_, c)| c))
        .collect();
    match clips.as_slice() {
        [] => None,
        [one] => Some(one.id),
        [first, rest @ ..] => {
            let link = first.link?;
            if rest.iter().any(|c| c.link != Some(link)) {
                return None;
            }
            clips
                .iter()
                .find(|c| matches!(c.content, ClipContent::Video { .. }))
                .or(clips
                    .iter()
                    .find(|c| !matches!(c.content, ClipContent::Audio { .. })))
                .or(clips.first())
                .map(|c| c.id)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn linked_video_and_sound_show_the_video() {
        use dmxcraft_core::timeline::{ClipContent, Timeline};
        use dmxcraft_core::SceneId;
        let mut tl = Timeline::default();
        let content = |kind: &str| -> ClipContent {
            serde_json::from_value(serde_json::json!({ kind: { "path": "v.mp4" } })).unwrap()
        };
        let v = tl.add_clip(0, content("Video"), 0, 4000);
        let a = tl.add_clip(1, content("Audio"), 0, 4000);
        let l = tl.add_clip(2, ClipContent::Scene(SceneId(1)), 0, 1000);
        tl.clip_mut(v).unwrap().link = Some(7);
        tl.clip_mut(a).unwrap().link = Some(7);
        assert_eq!(super::single_block(&tl, &[a, v].into()), Some(v));
        assert_eq!(super::single_block(&tl, &[a].into()), Some(a));
        assert_eq!(super::single_block(&tl, &[a, v, l].into()), None);
    }

    use super::*;
    use dmxcraft_core::{Show, Step};

    #[test]
    fn only_changed_fields_reach_other_scenes() {
        let mut show = Show::new("t", 1);
        let a = show.add_scene("A", None, vec![Step::new(Vec::new())]);
        let b = show.add_scene("B", None, vec![Step::new(Vec::new())]);
        show.scene_mut(b).unwrap().settings.fade_in_ms = 700;
        let mut e = Engine::new(show);
        let before = SceneEdit::of(&e, a).unwrap();
        {
            let s = e.show_mut().scene_mut(a).unwrap();
            s.settings.dimmer = 100;
            s.loop_mode = LoopMode::Once;
        }
        let after = SceneEdit::of(&e, a).unwrap();
        apply_to_others(&mut e, &before, &after, &[b]);
        let sb = e.show().scene(b).unwrap();
        assert_eq!(sb.settings.dimmer, 100);
        assert_eq!(sb.loop_mode, LoopMode::Once);
        assert_eq!(sb.settings.fade_in_ms, 700, "unchanged fields stay");
    }
}
