//! Properties of the audio triggers of an audio block (SUPER SCENE): band,
//! threshold, hysteresis, intervals and target.

use std::collections::HashMap;

use dmxcraft_core::mapping::{Control, SceneTarget};
use dmxcraft_core::timeline::{AudioTrigger, ClipContent, TriggerBand};
use dmxcraft_core::{Engine, SceneId};
use eframe::egui::{self, Color32, Vec2};

use super::fx_color::{label_row, slider_row, toggle_row};
use crate::i18n::t;
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{menu, Icon, IconButton};

/// What the trigger list shows (determined beforehand from show and state).
pub struct TriggerCx {
    /// Scenes as possible targets (without the SUPER SCENE itself).
    pub scenes: Vec<(SceneId, String)>,
    /// Label of the target per trigger.
    pub labels: HashMap<u32, String>,
    /// Number of events per trigger (`None`: being calculated).
    pub counts: HashMap<u32, Option<usize>>,
    /// Trigger whose target is currently being chosen.
    pub picking: Option<u32>,
}

/// Request from the list to the caller.
#[derive(Clone, Copy, PartialEq)]
pub enum TriggerAction {
    None,
    /// Choose the target via Alt + click.
    Pick(u32),
    CancelPick,
}

/// Display data for the audio block `clip` of the SUPER SCENE `scene`.
pub fn trigger_cx(
    e: &Engine,
    st: &AppState,
    ctx: &egui::Context,
    scene: SceneId,
    clip: u32,
) -> Option<TriggerCx> {
    let show = e.show();
    let (_, c) = show.scene(scene)?.timeline()?.clip(clip)?;
    let ClipContent::Audio { path, triggers, .. } = &c.content else {
        return None;
    };
    let mut labels = HashMap::new();
    let mut counts = HashMap::new();
    for tr in triggers {
        let label = match tr.control {
            None => t("No target").to_string(),
            Some(_) if !tr.channels.is_empty() => {
                crate::mapping::channel_name(show, tr.channels[0])
            }
            Some(c) => {
                let (cat, name) = crate::mapping::describe(show, &c);
                if cat.is_empty() {
                    name
                } else {
                    format!("{cat}: {name}")
                }
            }
        };
        labels.insert(tr.id, label);
        counts.insert(tr.id, st.triggers.events(path, tr, ctx).map(|e| e.len()));
    }
    let scenes = show
        .scenes
        .iter()
        .filter(|s| s.id != scene && s.timeline().is_none())
        .map(|s| (s.id, s.name.clone()))
        .collect();
    let picking = st
        .trigger_pick
        .filter(|(s, c, _)| *s == scene && *c == clip)
        .map(|(_, _, t)| t);
    Some(TriggerCx {
        scenes,
        labels,
        counts,
        picking,
    })
}

/// List of triggers; `true` if something changed.
pub fn triggers_ui(
    ui: &mut egui::Ui,
    triggers: &mut Vec<AudioTrigger>,
    tcx: &TriggerCx,
    accent: Color32,
    enabled: bool,
    action: &mut TriggerAction,
) -> bool {
    let before = triggers.clone();
    ui.add_space(theme::SP_S);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(t("Triggers").to_uppercase())
                .font(theme::caption())
                .color(theme::text_dim()),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let resp = ui.add(
                IconButton::new(Icon::Plus)
                    .size(24.0)
                    .flat()
                    .enabled(enabled),
            );
            let resp = resp.on_hover_text(t("Add trigger"));
            menu::dropdown(&resp, |ui| {
                menu::begin(ui);
                for band in TriggerBand::ALL {
                    let label = match band {
                        TriggerBand::Low => t("Low (kick, bass)"),
                        TriggerBand::Mid => t("Mid (snare, vocals)"),
                        TriggerBand::High => t("High (hi-hat, cymbals)"),
                    };
                    if menu::item(ui, menu::MenuItem::new(label).icon(Icon::Waveform)) {
                        let id = triggers.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                        triggers.push(AudioTrigger::preset(id, band));
                    }
                }
            });
        });
    });
    if triggers.is_empty() {
        ui.label(egui::RichText::new(t("No triggers")).color(theme::text_faint()));
    }
    let mut remove = None;
    for tr in triggers.iter_mut() {
        egui::Frame::new()
            .fill(theme::raised())
            .corner_radius(theme::R_S)
            .inner_margin(egui::Margin::same(6))
            .show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                trigger_ui(ui, tr, tcx, accent, enabled, action, &mut remove);
            });
        ui.add_space(theme::SP_XS);
    }
    if let Some(id) = remove {
        triggers.retain(|t| t.id != id);
    }
    *triggers != before
}

fn trigger_ui(
    ui: &mut egui::Ui,
    tr: &mut AudioTrigger,
    tcx: &TriggerCx,
    accent: Color32,
    enabled: bool,
    action: &mut TriggerAction,
    remove: &mut Option<u32>,
) {
    let id = ui.make_persistent_id(("audio_trigger", tr.id));
    // Header: name, count, mute, delete
    ui.horizontal(|ui| {
        ui.add_enabled(
            enabled,
            crate::ui::widgets::dialog::text_field(&mut tr.name).desired_width(90.0),
        );
        let count = match tcx.counts.get(&tr.id).copied().flatten() {
            Some(n) => crate::tr!("{n} events", n = n),
            None => t("calculating …").to_string(),
        };
        ui.label(
            egui::RichText::new(count)
                .font(theme::caption())
                .color(theme::text_faint()),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(
                    IconButton::new(Icon::Trash)
                        .size(22.0)
                        .flat()
                        .enabled(enabled),
                )
                .on_hover_text(t("Delete trigger"))
                .clicked()
            {
                *remove = Some(tr.id);
            }
            let icon = if tr.muted { Icon::EyeOff } else { Icon::Eye };
            if ui
                .add(IconButton::new(icon).size(22.0).flat().enabled(enabled))
                .on_hover_text(t("Enable/disable trigger"))
                .clicked()
            {
                tr.muted = !tr.muted;
            }
        });
    });
    // Band
    ui.horizontal(|ui| {
        ui.set_min_height(28.0);
        ui.label(egui::RichText::new(t("Band")).color(theme::text_dim()));
        let mut band = tr.band();
        let items = [
            (Some(TriggerBand::Low), "Low"),
            (Some(TriggerBand::Mid), "Mid"),
            (Some(TriggerBand::High), "High"),
            (None, t("Custom")),
        ];
        let w = ((ui.available_width() - 4.0) / 4.0).clamp(36.0, 64.0);
        ui.add_enabled_ui(enabled, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if crate::ui::widgets::segmented::Segmented::new(&items)
                .item_width(w)
                .show(ui, &mut band)
            {
                match band {
                    Some(b) => (tr.low_hz, tr.high_hz) = b.range(),
                    // custom band: start values that match no preset
                    None => (tr.low_hz, tr.high_hz) = (40.0, 120.0),
                }
            }
        });
    });
    if tr.band().is_none() {
        label_row(ui, t("Frequency (Hz)"), |ui| {
            ui.add_enabled_ui(enabled, |ui| {
                ui.add_sized(
                    Vec2::new(64.0, 22.0),
                    egui::DragValue::new(&mut tr.low_hz)
                        .range(0.0..=19_000.0)
                        .speed(5.0)
                        .max_decimals(0),
                );
                ui.label(egui::RichText::new("–").color(theme::text_dim()));
                ui.add_sized(
                    Vec2::new(64.0, 22.0),
                    egui::DragValue::new(&mut tr.high_hz)
                        .range(30.0..=20_000.0)
                        .speed(10.0)
                        .max_decimals(0),
                );
            });
        });
        tr.high_hz = tr.high_hz.max(tr.low_hz + 10.0);
    }
    let pct = |ui: &mut egui::Ui, key: &str, label: &str, v: &mut f32, max: f32| {
        let mut p = *v * 100.0;
        slider_row(
            ui,
            id.with(key),
            label,
            &mut p,
            (0.0, max),
            0,
            enabled,
            accent,
        );
        *v = p / 100.0;
    };
    pct(ui, "thr", t("Threshold (%)"), &mut tr.threshold, 100.0);
    pct(ui, "hys", t("Hysteresis (%)"), &mut tr.hysteresis, 50.0);
    tr.threshold = tr.threshold.max(0.01);
    let ms = |ui: &mut egui::Ui, key: &str, label: &str, v: &mut u32, max: f32| {
        let mut f = *v as f32;
        slider_row(
            ui,
            id.with(key),
            label,
            &mut f,
            (0.0, max),
            0,
            enabled,
            accent,
        );
        *v = f.round().max(0.0) as u32;
    };
    ms(ui, "gap", t("Minimum gap (ms)"), &mut tr.min_gap_ms, 1000.0);
    ms(ui, "hold", t("Hold time (ms)"), &mut tr.hold_ms, 1000.0);
    ms(ui, "lead", t("Lead time (ms)"), &mut tr.lead_ms, 500.0);
    toggle_row(ui, t("Only during the event"), &mut tr.flash, enabled);
    // Target
    let label = tcx.labels.get(&tr.id).cloned().unwrap_or_default();
    label_row(ui, t("Target"), |ui| {
        ui.add(
            egui::Label::new(egui::RichText::new(label).color(if tr.control.is_some() {
                accent
            } else {
                theme::text_faint()
            }))
            .truncate(),
        );
    });
    ui.horizontal(|ui| {
        let resp = ui.add_enabled(enabled, egui::Button::new(t("Scene …")));
        menu::dropdown(&resp, |ui| {
            menu::begin(ui);
            egui::ScrollArea::vertical()
                .max_height(320.0)
                .show(ui, |ui| {
                    for (sid, name) in &tcx.scenes {
                        if menu::item(ui, menu::MenuItem::new(name)) {
                            tr.control = Some(Control::Scene {
                                target: SceneTarget::Scene { id: *sid },
                            });
                            tr.channels.clear();
                        }
                    }
                });
        });
        if tcx.picking == Some(tr.id) {
            if ui
                .add_enabled(enabled, egui::Button::new(t("Cancel")))
                .clicked()
            {
                *action = TriggerAction::CancelPick;
            }
        } else if ui
            .add_enabled(enabled, egui::Button::new(t("Pick element")))
            .on_hover_text(t("Then Alt + click a control (as for Touch)"))
            .clicked()
        {
            *action = TriggerAction::Pick(tr.id);
        }
        if tr.control.is_some()
            && ui
                .add_enabled(enabled, egui::Button::new(t("Remove")))
                .clicked()
        {
            tr.control = None;
            tr.channels.clear();
        }
    });
    if tcx.picking == Some(tr.id) {
        ui.label(
            egui::RichText::new(t("Alt + click the target …"))
                .font(theme::caption())
                .color(accent),
        );
    }
}

/// Alt + click while choosing the target: set the trigger's target.
pub fn apply_trigger_pick(
    e: &mut Engine,
    st: &mut AppState,
    (scene, clip, trigger): (SceneId, u32, u32),
    pick: crate::mapping::TouchPick,
) {
    let Some(control) = pick.control else {
        st.info(t("This element can't be a trigger target"));
        return;
    };
    let (control, channels) = if pick.channels.is_empty() {
        (control, Vec::new())
    } else {
        (Control::Channels, pick.channels)
    };
    let tr = e
        .show_mut()
        .scene_mut(scene)
        .and_then(|s| s.timeline_mut())
        .and_then(|tl| tl.clip_mut(clip))
        .and_then(|c| match &mut c.content {
            ClipContent::Audio { triggers, .. } => triggers.iter_mut().find(|t| t.id == trigger),
            _ => None,
        });
    if let Some(tr) = tr {
        tr.control = Some(control);
        tr.channels = channels;
        st.info(t("Target assigned"));
    }
}
