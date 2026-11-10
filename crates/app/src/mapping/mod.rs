//! Keyboard and MIDI mapping: overlay above the interface (every mappable
//! element is highlighted and can be clicked), learning a key or MIDI
//! message, triggering during operation and LED feedback.
//!
//! Controls register with [`target`]. While the overlay is active a click
//! area lies on top – the element itself does not react then.

pub mod window;

use std::collections::{HashMap, HashSet};

use dmxcraft_core::mapping::{
    ButtonMode, Control, Input, MapKind, Mapping, MidiMessage, SceneTarget,
};
use dmxcraft_core::{Engine, SceneId, Show, ZoneId};
use eframe::egui::{self, Color32, Rect, Response, Sense, Stroke, StrokeKind};

use crate::midi::{MidiEvent, MidiHub};
use crate::midi_device::{self, MidiDevice};
use crate::state::AppState;
use crate::ui::theme;

/// Color of the overlay: keyboard red, MIDI violet.
pub fn color(kind: MapKind) -> Color32 {
    match kind {
        MapKind::Keyboard => Color32::from_rgb(214, 64, 82),
        MapKind::Midi => Color32::from_rgb(134, 92, 214),
    }
}

/// What the overlay shows in this frame (set by the app frame).
#[derive(Clone, Default)]
struct Overlay {
    kind: Option<MapKind>,
    selected: Option<Control>,
    /// Mapped elements with a short form of the input.
    labels: Vec<(Control, String)>,
}

fn overlay_id() -> egui::Id {
    egui::Id::unique("mapping_overlay")
}

fn pick_id() -> egui::Id {
    egui::Id::unique("mapping_pick")
}

/// Clicked element with display name (only for [`Control::Ui`]) and the
/// channels it currently controls (programmer faders).
#[derive(Clone, Debug)]
struct Pick {
    element: Control,
    name: Option<(String, String)>,
    channels: Vec<dmxcraft_core::ChannelRef>,
}

fn pick(ctx: &egui::Context, element: Control, name: Option<(String, String)>) {
    pick_full(
        ctx,
        Pick {
            element,
            name,
            channels: Vec::new(),
        },
    );
}

fn pick_full(ctx: &egui::Context, p: Pick) {
    ctx.data_mut(|d| d.insert_temp(pick_id(), Some(p)));
}

/// Triggered inputs for interface controls (for the next frame).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiFire {
    /// Key pressed (button: like a click).
    pub press: bool,
    /// Value 0–1 (knob via CC or “value in %”).
    pub value: Option<f32>,
}

fn fires_id() -> egui::Id {
    egui::Id::unique("mapping_ui_fires")
}

fn states_id() -> egui::Id {
    egui::Id::unique("mapping_ui_states")
}

/// Collect the triggered input of an element.
fn take_fire(ctx: &egui::Context, key: u64) -> Option<UiFire> {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<HashMap<u64, UiFire>>(fires_id())
            .remove(&key)
    })
}

/// Reported states of the controls (for touch buttons).
pub fn ui_states(ctx: &egui::Context) -> HashMap<u64, bool> {
    ctx.data(|d| d.get_temp(states_id())).unwrap_or_default()
}

/// Remember the state of an element for LED feedback.
fn report(ctx: &egui::Context, key: u64, on: bool) {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<HashMap<u64, bool>>(states_id())
            .insert(key, on);
    });
}

/// Evaluate the mapped key of a control even when it is not shown at the
/// moment (e.g. sound pads from another tab).
pub fn poll(ctx: &egui::Context, key: &str, on: bool) -> bool {
    let key = dmxcraft_core::mapping::ui_key(key);
    report(ctx, key, on);
    take_fire(ctx, key).is_some_and(|f| f.press || f.value.is_some_and(|v| v > 0.0))
}

/// Make an interface button mappable. `key` is a fixed name such as
/// “programmer/mode/live”, `on` the state for the LED, `name` returns
/// (category, target) for the list. Returns `true` if a mapped key was
/// pressed (then treat it like a click).
pub fn button(
    ui: &egui::Ui,
    resp: &Response,
    key: &str,
    on: bool,
    name: impl FnOnce() -> (String, String),
) -> bool {
    button_rect(ui, resp.rect, key, on, name)
}

/// Like [`button`] for an area without its own response (e.g. fixtures in
/// the plan).
pub fn button_rect(
    ui: &egui::Ui,
    rect: Rect,
    key: &str,
    on: bool,
    name: impl FnOnce() -> (String, String),
) -> bool {
    let key = dmxcraft_core::mapping::ui_key(key);
    let control = Control::Ui { key, fader: false };
    report(ui.ctx(), key, on);
    // Inputs also work while the mapping window is open (to try them out)
    target_with(ui, rect, control, name);
    take_fire(ui.ctx(), key).is_some_and(|f| f.press || f.value.is_some_and(|v| v > 0.0))
}

/// Make an interface slider mappable. `value` is the current value 0–1.
/// Returns the new value when an input came (knob: value, key: toggle
/// between 0 and 1).
pub fn fader(
    ui: &egui::Ui,
    rect: Rect,
    key: &str,
    value: f32,
    name: impl FnOnce() -> (String, String),
) -> Option<f32> {
    fader_channels(ui, rect, key, value, &[], name)
}

/// Like [`fader`] for a fader that currently controls specific channels
/// (programmer). A new mapping then controls exactly these channels – even if
/// the fixtures are no longer selected later. In the window you can choose
/// “Selected fixtures” instead (follows the selection).
pub fn fader_channels(
    ui: &egui::Ui,
    rect: Rect,
    key: &str,
    value: f32,
    channels: &[dmxcraft_core::ChannelRef],
    name: impl FnOnce() -> (String, String),
) -> Option<f32> {
    let key = dmxcraft_core::mapping::ui_key(key);
    // Element = fader with exactly these channels (key ^ channel list)
    let element = if channels.is_empty() {
        key
    } else {
        key ^ dmxcraft_core::mapping::channels_key(channels)
    };
    report(ui.ctx(), key, value > 0.0);
    // Inputs also work while the mapping window is open (to try them out)
    target_shape(
        ui,
        rect,
        0.0,
        Control::Ui {
            key: element,
            fader: true,
        },
        name,
        channels,
    );
    let f = take_fire(ui.ctx(), key)?;
    match f.value {
        Some(v) => Some(v),
        None if f.press => Some(if value > 0.5 { 0.0 } else { 1.0 }),
        None => None,
    }
}

/// Register a control. Returns `true` if the overlay is active (then the
/// overlay gets the click, not the element).
pub fn target(ui: &egui::Ui, rect: Rect, control: Control) -> bool {
    target_with(ui, rect, control, || (String::new(), String::new()))
}

/// Like [`target`] for a rotated area (degrees around the center), e.g. a
/// rotated fixture in the stage plan.
pub fn target_rotated(ui: &egui::Ui, rect: Rect, rotation: f32, control: Control) -> bool {
    target_shape(
        ui,
        rect,
        rotation,
        control,
        || (String::new(), String::new()),
        &[],
    )
}

/// Like [`target`], with a display name for interface controls.
fn target_with(
    ui: &egui::Ui,
    rect: Rect,
    control: Control,
    name: impl FnOnce() -> (String, String),
) -> bool {
    target_shape(ui, rect, 0.0, control, name, &[])
}

/// Corners of a rectangle rotated around its center.
fn rotated_quad(rect: Rect, rotation: f32) -> [egui::Pos2; 4] {
    let c = rect.center();
    let (sin, cos) = rotation.to_radians().sin_cos();
    [
        rect.left_top(),
        rect.right_top(),
        rect.right_bottom(),
        rect.left_bottom(),
    ]
    .map(|q| {
        let d = q - c;
        c + egui::vec2(d.x * cos - d.y * sin, d.x * sin + d.y * cos)
    })
}

/// Does a point lie inside the rotated rectangle?
fn in_rotated(rect: Rect, rotation: f32, p: egui::Pos2) -> bool {
    let c = rect.center();
    let (sin, cos) = (-rotation).to_radians().sin_cos();
    let d = p - c;
    let local = c + egui::vec2(d.x * cos - d.y * sin, d.x * sin + d.y * cos);
    rect.contains(local)
}

fn target_shape(
    ui: &egui::Ui,
    rect: Rect,
    rotation: f32,
    control: Control,
    name: impl FnOnce() -> (String, String),
    channels: &[dmxcraft_core::ChannelRef],
) -> bool {
    let Some(o) = ui.ctx().data(|d| d.get_temp::<Overlay>(overlay_id())) else {
        return false;
    };
    let Some(kind) = o.kind else {
        // Alt + click: add the element to the touch surface
        let mut chans = channels.to_vec();
        alt_click(ui, rect, rotation, control, |p| {
            p.name = matches!(control, Control::Ui { .. }).then(name);
            p.channels = std::mem::take(&mut chans);
        });
        return false;
    };
    let rotated = rotation.rem_euclid(360.0) != 0.0;
    let quad = rotated_quad(rect, rotation);
    let hit_rect = if rotated {
        Rect::from_points(&quad)
    } else {
        rect
    };
    let id = ui
        .scope_id()
        .with(("mapping_target", format!("{control:?}")));
    let resp = ui.interact(hit_rect, id, Sense::click());
    let inside = !rotated
        || resp
            .hover_pos()
            .or(resp.interact_pointer_pos())
            .is_some_and(|p| in_rotated(rect, rotation, p));
    let base = color(kind);
    let label = o.labels.iter().find(|(c, _)| *c == control).map(|l| &l.1);
    let selected = o.selected == Some(control);
    let alpha = if selected {
        0.75
    } else if label.is_some() {
        0.55
    } else if resp.hovered() && inside {
        0.45
    } else {
        0.3
    };
    let p = ui.painter();
    let stroke = Stroke::new(if selected { 2.0 } else { 1.0 }, base);
    if rotated {
        p.add(egui::Shape::convex_polygon(
            quad.to_vec(),
            base.gamma_multiply(alpha),
            stroke,
        ));
    } else {
        p.rect_filled(rect, theme::R_S, base.gamma_multiply(alpha));
        p.rect_stroke(rect, theme::R_S, stroke, StrokeKind::Inside);
    }
    if let Some(l) = label {
        let g = p.layout_no_wrap(
            l.clone(),
            egui::FontId::proportional(theme::FS_CAPTION),
            Color32::WHITE,
        );
        // rotated: centered (otherwise the corner lies next to the shape)
        let pos = if rotated {
            rect.center() - g.size() * 0.5
        } else {
            rect.right_top() + egui::vec2(-g.size().x - 4.0, 2.0)
        };
        p.galley(pos, g, Color32::WHITE);
    }
    if resp.clicked() && inside {
        let name = matches!(control, Control::Ui { .. }).then(name);
        pick_full(
            ui.ctx(),
            Pick {
                element: control,
                name,
                channels: channels.to_vec(),
            },
        );
    }
    true
}

/// Element for the touch surface (Alt + click or context menu).
#[derive(Clone, Debug, Default)]
pub struct TouchPick {
    pub kind: dmxcraft_core::touch::TouchKind,
    pub control: Option<Control>,
    pub name: Option<(String, String)>,
    pub channels: Vec<dmxcraft_core::ChannelRef>,
    pub fixtures: Vec<dmxcraft_core::FixtureId>,
}

fn touch_pick_id() -> egui::Id {
    egui::Id::unique("touch_pick")
}

/// Queue an element for the touch surface (processed by the app frame).
pub fn touch_pick(ctx: &egui::Context, p: TouchPick) {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<Vec<TouchPick>>(touch_pick_id())
            .push(p);
    });
}

/// Collect queued elements.
pub fn take_touch_picks(ctx: &egui::Context) -> Vec<TouchPick> {
    ctx.data_mut(|d| d.remove_temp::<Vec<TouchPick>>(touch_pick_id()))
        .unwrap_or_default()
}

/// Queue a control for the touch surface via a context menu.
pub fn touch_pick_control(ctx: &egui::Context, control: Control) {
    touch_pick(
        ctx,
        TouchPick {
            control: Some(control),
            ..Default::default()
        },
    );
}

/// While Alt is pressed a click area lies above the element (highlights it)
/// and a click queues it for the touch surface.
fn alt_click(
    ui: &egui::Ui,
    rect: Rect,
    rotation: f32,
    control: Control,
    fill: impl FnOnce(&mut TouchPick),
) {
    if !ui.input(|i| i.modifiers.alt) {
        return;
    }
    let quad = rotated_quad(rect, rotation);
    let hit = Rect::from_points(&quad);
    let id = ui.scope_id().with(("touch_alt", format!("{control:?}")));
    let resp = ui
        .interact(hit, id, Sense::click())
        .on_hover_text(t_touch_hint());
    if resp.hovered() {
        ui.painter().add(egui::Shape::closed_line(
            quad.to_vec(),
            Stroke::new(2.0, theme::accent()),
        ));
    }
    if resp.clicked() {
        let fader = matches!(
            control,
            Control::Master
                | Control::AudioMaster
                | Control::Channel { .. }
                | Control::Channels
                | Control::Ui { fader: true, .. }
        );
        let mut p = TouchPick {
            kind: if fader {
                dmxcraft_core::touch::TouchKind::Fader
            } else {
                dmxcraft_core::touch::TouchKind::Button
            },
            control: Some(control),
            ..Default::default()
        };
        fill(&mut p);
        if !p.channels.is_empty() {
            p.kind = dmxcraft_core::touch::TouchKind::Fader;
        }
        touch_pick(ui.ctx(), p);
    }
}

fn t_touch_hint() -> &'static str {
    crate::i18n::t("Add to touch surface")
}

/// Color field or XY field of the programmer: Alt + click creates a color
/// wheel or XY pad for the selected fixtures on the touch surface.
pub fn touch_source(
    ui: &egui::Ui,
    rect: Rect,
    kind: dmxcraft_core::touch::TouchKind,
    fixtures: &[dmxcraft_core::FixtureId],
) {
    if !ui.input(|i| i.modifiers.alt) || fixtures.is_empty() {
        return;
    }
    let id = ui.scope_id().with(("touch_src", kind as u8));
    let resp = ui
        .interact(rect, id, Sense::click())
        .on_hover_text(t_touch_hint());
    if resp.hovered() {
        ui.painter().rect_stroke(
            rect,
            theme::R_S,
            Stroke::new(2.0, theme::accent()),
            StrokeKind::Inside,
        );
    }
    if resp.clicked() {
        touch_pick(
            ui.ctx(),
            TouchPick {
                kind,
                fixtures: fixtures.to_vec(),
                ..Default::default()
            },
        );
    }
}

/// Display name of a mapping: (category, target).
pub fn describe_mapping(show: &Show, m: &Mapping) -> (String, String) {
    match (&m.control, &m.name) {
        // fixed channels: current fixture names (renaming takes effect at once)
        (Control::Channels, _) if !m.channels.is_empty() => {
            let fixtures: std::collections::BTreeSet<_> =
                m.channels.iter().map(|c| c.fixture).collect();
            let label = channel_name(show, m.channels[0]);
            let label = if fixtures.len() > 1 {
                let chan = label.split_once(": ").map_or(label.as_str(), |x| x.1);
                crate::tr!("{n} devices: {chan}", n = fixtures.len(), chan = chan)
            } else {
                label
            };
            (crate::i18n::t("Programmer").to_string(), label)
        }
        (Control::Ui { .. } | Control::Channels, Some(n)) => n.clone(),
        (c, _) => describe(show, c),
    }
}

/// Name of the control for lists: (category, target).
pub fn describe(show: &Show, c: &Control) -> (String, String) {
    use crate::i18n::t;
    let zone = |z: ZoneId| {
        show.zone(z)
            .map_or_else(|| t("(deleted)").to_string(), |z| z.name.clone())
    };
    match c {
        Control::Scene { target } => {
            let what = match target {
                SceneTarget::Scene { id } => match show.scene(*id) {
                    Some(s) => {
                        let bank = s.zone.map(zone).unwrap_or_default();
                        format!("{}: {}", bank.to_uppercase(), s.name)
                    }
                    None => t("(deleted)").to_string(),
                },
                SceneTarget::Selected => t("Selected scene").to_string(),
                SceneTarget::Index { zone: z, index } => {
                    crate::tr!(
                        "{bank}: no. {n}",
                        bank = zone(*z).to_uppercase(),
                        n = index + 1
                    )
                }
                SceneTarget::IndexInShownBank { index } => {
                    crate::tr!("Shown bank: no. {n}", n = index + 1)
                }
            };
            (t("Scene").to_string(), what)
        }
        Control::ShowBank { zone: z } => (
            t("Bank").to_string(),
            z.map_or_else(|| t("All").to_string(), zone),
        ),
        Control::BankStep { delta } => (
            t("Bank").to_string(),
            if *delta < 0 {
                t("Previous bank")
            } else {
                t("Next bank")
            }
            .to_string(),
        ),
        Control::ZonePause { zone: z } => (
            t("Bank").to_string(),
            crate::tr!("{bank}: pause", bank = zone(*z)),
        ),
        Control::ZoneStep { zone: z, delta } => (
            t("Bank").to_string(),
            if *delta < 0 {
                crate::tr!("{bank}: step back", bank = zone(*z))
            } else {
                crate::tr!("{bank}: step forward", bank = zone(*z))
            },
        ),
        Control::Blackout => (t("Global").to_string(), t("Blackout").to_string()),
        Control::Master => (t("Global").to_string(), t("Master").to_string()),
        Control::AudioMaster => (t("Audio").to_string(), t("Sound master").to_string()),
        Control::AudioMute => (t("Audio").to_string(), t("Mute sound").to_string()),
        Control::AudioFade => (t("Audio").to_string(), t("Fade out sound").to_string()),
        Control::StopAll => (t("Global").to_string(), t("Stop all scenes").to_string()),
        Control::Ui { .. } => (t("Interface").to_string(), t("UI element").to_string()),
        Control::Group { id } => (
            t("Group").to_string(),
            match id {
                None => t("All").to_string(),
                Some(g) => show
                    .groups
                    .iter()
                    .find(|x| x.id == *g)
                    .map_or_else(|| t("(deleted)").to_string(), |x| x.name.clone()),
            },
        ),
        Control::Fixture { id } => (
            t("Device").to_string(),
            show.fixture(*id)
                .map_or_else(|| t("(deleted)").to_string(), |f| f.name.clone()),
        ),
        Control::Channel { channel } => (t("Device").to_string(), channel_name(show, *channel)),
        Control::Channels => (t("Programmer").to_string(), t("Fixed channels").to_string()),
    }
}

/// Fixtures of a group (`None` = all).
fn group_set(
    show: &Show,
    id: Option<dmxcraft_core::GroupId>,
) -> std::collections::BTreeSet<dmxcraft_core::FixtureId> {
    match id {
        None => show.fixtures.iter().map(|f| f.id).collect(),
        Some(g) => show
            .groups
            .iter()
            .find(|x| x.id == g)
            .map(|x| x.fixtures.iter().copied().collect())
            .unwrap_or_default(),
    }
}

/// “Fixture: channel name”.
pub fn channel_name(show: &Show, ch: dmxcraft_core::ChannelRef) -> String {
    let Some(f) = show.fixture(ch.fixture) else {
        return t_deleted();
    };
    let name = if ch.is_virtual() {
        crate::i18n::t("Dimmer (virtual)").to_string()
    } else {
        show.type_of(f)
            .and_then(|ty| ty.channels.get(usize::from(ch.channel)))
            .map_or_else(|| format!("{}", ch.channel + 1), |c| c.name.clone())
    };
    format!("{}: {}", f.name, name)
}

fn t_deleted() -> String {
    crate::i18n::t("(deleted)").to_string()
}

/// Scenes of a bank in the displayed order (without hidden ones).
fn zone_scenes(show: &Show, z: ZoneId) -> Vec<SceneId> {
    show.scenes
        .iter()
        .filter(|s| s.zone == Some(z) && !s.hidden)
        .map(|s| s.id)
        .collect()
}

/// Scenes a target currently points to.
pub fn resolve(show: &Show, st: &AppState, t: &SceneTarget) -> Vec<SceneId> {
    match *t {
        SceneTarget::Scene { id } => show.scene(id).map(|_| id).into_iter().collect(),
        SceneTarget::Selected => st.selected_scenes.iter().copied().collect(),
        SceneTarget::Index { zone, index } => zone_scenes(show, zone)
            .get(index)
            .copied()
            .into_iter()
            .collect(),
        SceneTarget::IndexInShownBank { index } => {
            let z = st
                .shown_bank
                .or_else(|| show.zones.iter().find(|z| !z.hidden).map(|z| z.id));
            z.and_then(|z| zone_scenes(show, z).get(index).copied())
                .into_iter()
                .collect()
        }
    }
}

/// “On” state of an element (for LED feedback).
pub(crate) fn is_on(e: &Engine, st: &AppState, states: &HashMap<u64, bool>, m: &Mapping) -> bool {
    match &m.control {
        Control::Channels => m
            .channels
            .iter()
            .any(|c| e.live().get(c).is_some_and(|v| *v > 0)),
        Control::Scene { target } => resolve(e.show(), st, target)
            .iter()
            .any(|id| e.is_active(*id)),
        Control::ShowBank { zone } => st.shown_bank == *zone,
        Control::ZonePause { zone } | Control::ZoneStep { zone, .. } => {
            e.active_in_zone(*zone).is_some()
        }
        Control::Blackout => e.blackout(),
        Control::Master => e.master() > 0.0,
        Control::AudioMaster => st.audio.gain() > 0.0,
        Control::AudioMute => st.audio.mute,
        Control::AudioFade => st.audio.fading(),
        Control::Ui { key, .. } => states.get(key).copied().unwrap_or(false),
        Control::Fixture { id } => st.selected.contains(id),
        Control::Group { id } => {
            let set = group_set(e.show(), *id);
            !set.is_empty() && st.selected == set
        }
        Control::Channel { channel } => e.live().get(channel).is_some_and(|v| *v > 0),
        Control::BankStep { .. } | Control::StopAll => false,
    }
}

/// Trigger an input: `pressed` (key/note on) and value 0–127.
fn fire(
    e: &mut Engine,
    st: &mut AppState,
    ui: &mut HashMap<u64, UiFire>,
    m: &Mapping,
    pressed: bool,
    value: u8,
    cc: bool,
) {
    let on = match m.mode {
        ButtonMode::Toggle => pressed,
        ButtonMode::Value => value > 0,
    };
    match &m.control {
        Control::Scene { target } => {
            let ids = resolve(e.show(), st, target);
            for id in ids {
                match (m.mode, m.flash) {
                    (ButtonMode::Value, _) | (ButtonMode::Toggle, true) => {
                        if on && !e.is_active(id) {
                            e.activate(id);
                        } else if !on && e.is_active(id) {
                            e.release(id);
                        }
                    }
                    (ButtonMode::Toggle, false) => {
                        if pressed {
                            e.toggle(id);
                        }
                    }
                }
            }
        }
        Control::ShowBank { zone } => {
            if pressed {
                st.request_bank = Some(*zone);
            }
        }
        Control::BankStep { delta } => {
            if pressed {
                let zones: Vec<Option<ZoneId>> = std::iter::once(None)
                    .chain(
                        e.show()
                            .zones
                            .iter()
                            .filter(|z| !z.hidden)
                            .map(|z| Some(z.id)),
                    )
                    .collect();
                let i = zones.iter().position(|z| *z == st.shown_bank).unwrap_or(0) as i32;
                let n = zones.len() as i32;
                st.request_bank = Some(zones[(i + delta).rem_euclid(n) as usize]);
            }
        }
        Control::ZonePause { zone } => {
            if pressed {
                if let Some(id) = e.active_in_zone(*zone) {
                    e.toggle_pause(id);
                }
            }
        }
        Control::ZoneStep { zone, delta } => {
            if pressed {
                if let Some(id) = e.active_in_zone(*zone) {
                    e.jump_step(id, *delta);
                }
            }
        }
        Control::Blackout => match (m.mode, m.flash) {
            (ButtonMode::Toggle, false) => {
                if pressed {
                    let b = e.blackout();
                    e.set_blackout(!b);
                }
            }
            _ => e.set_blackout(on),
        },
        Control::Master => match m.mode {
            ButtonMode::Value => e.set_master(f32::from(value) / 127.0),
            ButtonMode::Toggle => {
                if pressed {
                    let v = if e.master() > 0.5 { 0.0 } else { 1.0 };
                    e.set_master(v);
                }
            }
        },
        Control::StopAll => {
            if pressed {
                e.release_all();
            }
        }
        Control::AudioMaster => match m.mode {
            ButtonMode::Value => st.audio.set_master(f32::from(value) / 127.0),
            ButtonMode::Toggle => {
                if pressed {
                    let v = if st.audio.master > 0.5 { 0.0 } else { 1.0 };
                    st.audio.set_master(v);
                }
            }
        },
        Control::AudioMute => match (m.mode, m.flash) {
            (ButtonMode::Toggle, false) => {
                if pressed {
                    st.audio.mute = !st.audio.mute;
                }
            }
            _ => st.audio.mute = on,
        },
        Control::AudioFade => {
            if pressed {
                st.audio.start_fade(e);
            }
        }
        Control::Group { id } => {
            if pressed {
                let set = group_set(e.show(), *id);
                st.selected = if st.selected == set {
                    Default::default()
                } else {
                    set
                };
                st.selection_order = st.selected.iter().copied().collect();
            }
        }
        Control::Fixture { id } => {
            if pressed && !st.selected.remove(id) {
                st.selected.insert(*id);
            }
        }
        Control::Channels => {
            let any_on = m
                .channels
                .iter()
                .any(|c| e.live().get(c).is_some_and(|v| *v > 0));
            for ch in &m.channels {
                let ch = *ch;
                if cc || m.mode == ButtonMode::Value {
                    e.set_live(ch, ((u32::from(value) * 255 + 63) / 127) as u8);
                } else if m.flash {
                    if pressed {
                        e.set_live(ch, 255);
                    } else {
                        e.clear_live_channel(ch);
                    }
                } else if pressed {
                    if any_on {
                        e.clear_live_channel(ch);
                    } else {
                        e.set_live(ch, 255);
                    }
                }
            }
        }
        Control::Channel { channel } => {
            let ch = *channel;
            if cc || m.mode == ButtonMode::Value {
                e.set_live(ch, ((u32::from(value) * 255 + 63) / 127) as u8);
            } else if m.flash {
                if pressed {
                    e.set_live(ch, 255);
                } else {
                    e.clear_live_channel(ch);
                }
            } else if pressed {
                if e.live().get(&ch).is_some_and(|v| *v > 0) {
                    e.clear_live_channel(ch);
                } else {
                    e.set_live(ch, 255);
                }
            }
        }
        Control::Ui { key, fader } => {
            let f = ui.entry(*key).or_default();
            if *fader && (cc || m.mode == ButtonMode::Value) {
                f.value = Some(f32::from(value) / 127.0);
            } else if pressed {
                f.press = true;
            }
        }
    }
}

/// LED feedback for a newly learned key: output of the configured device
/// (or the output with the same name), “on” in the scene color.
fn default_feedback(
    devices: &[MidiDevice],
    outputs: &[String],
    input: &str,
    color: Option<[u8; 3]>,
) -> Option<dmxcraft_core::mapping::Feedback> {
    let (out, model) = match devices.iter().find(|d| d.input == input) {
        Some(d) if !d.output.is_empty() => (d.output.clone(), d.model),
        Some(_) => return None,
        None => {
            let o = midi_device::matching_output(input, outputs)?;
            (o, midi_device::DeviceModel::detect(input))
        }
    };
    let mut fb = dmxcraft_core::mapping::Feedback {
        device: out,
        ..Default::default()
    };
    if let (Some(p), Some(c)) = (model.palette(), color) {
        fb.on.velocity = midi_device::nearest(p, c);
    }
    Some(fb)
}

/// State of the mapping between frames.
#[derive(Default)]
pub struct Mapper {
    pub midi: MidiHub,
    /// Last sent LED states per MIDI mapping.
    sent: HashMap<u32, Sent>,
    /// Pressed keys (for releasing with flash).
    held: Vec<u32>,
    /// State of the mapping window.
    pub win: window::WindowState,
    /// Configured controllers (from the settings).
    pub devices: Vec<MidiDevice>,
    /// Outputs whose LEDs have been cleared since connecting.
    cleared: HashSet<String>,
    /// Triggered inputs for controls (go to the next frame).
    ui_fires: HashMap<u64, UiFire>,
}

impl Mapper {
    /// Every frame before the views: provide the overlay.
    pub fn begin_frame(&mut self, ctx: &egui::Context, show: &Show, st: &AppState) {
        let o = match st.mapping {
            Some(kind) => {
                let list = show.mappings.list(kind);
                Overlay {
                    kind: Some(kind),
                    selected: st
                        .map_sel
                        .and_then(|id| list.iter().find(|m| m.id == id))
                        .map(|m| m.element()),
                    labels: list
                        .iter()
                        .filter_map(|m| {
                            let short = match m.input.as_ref()? {
                                Input::Key { .. } => m.input.as_ref()?.label(),
                                Input::Midi {
                                    message, number, ..
                                } => match message {
                                    MidiMessage::Note => dmxcraft_core::mapping::note_name(*number),
                                    MidiMessage::Cc => format!("CC{number}"),
                                },
                            };
                            Some((m.element(), short))
                        })
                        .collect(),
                }
            }
            None => Overlay::default(),
        };
        ctx.data_mut(|d| {
            d.insert_temp(overlay_id(), o);
            d.remove_temp::<Option<Pick>>(pick_id());
        });
    }

    /// Every frame after the views: process clicks of the overlay, keyboard and
    /// MIDI, send feedback.
    pub fn end_frame(&mut self, ctx: &egui::Context, e: &mut Engine, st: &mut AppState) {
        self.midi.scan();
        // Remove mappings of deleted scenes
        let show = e.show();
        let stale = show.mappings.keyboard.iter().chain(&show.mappings.midi).any(|m| {
            matches!(m.element(), Control::Scene { target: SceneTarget::Scene { id } } if show.scene(id).is_none())
        });
        let gone_fixture = |c: Control| match c {
            Control::Fixture { id }
            | Control::Channel {
                channel: dmxcraft_core::ChannelRef { fixture: id, .. },
            } => show.fixture(id).is_none(),
            _ => false,
        };
        let stale_fixtures = show
            .mappings
            .keyboard
            .iter()
            .chain(&show.mappings.midi)
            .any(|m| gone_fixture(m.element()) || gone_fixture(m.control));
        if stale {
            let ids: Vec<SceneId> = show.scenes.iter().map(|s| s.id).collect();
            e.show_mut().mappings.retain_scenes(|id| ids.contains(&id));
        }
        if stale_fixtures {
            let ids: Vec<dmxcraft_core::FixtureId> =
                e.show().fixtures.iter().map(|f| f.id).collect();
            let alive = |c: Control| match c {
                Control::Fixture { id }
                | Control::Channel {
                    channel: dmxcraft_core::ChannelRef { fixture: id, .. },
                } => ids.contains(&id),
                _ => true,
            };
            let maps = &mut e.show_mut().mappings;
            for list in [&mut maps.keyboard, &mut maps.midi] {
                list.retain(|m| alive(m.element()) && alive(m.control));
            }
        }
        // clicked element: select the existing mapping or create a new one
        if let (
            Some(kind),
            Some(Some(Pick {
                element: c,
                name,
                channels,
            })),
        ) = (
            st.mapping,
            ctx.data(|d| d.get_temp::<Option<Pick>>(pick_id())),
        ) {
            let existing = e
                .show()
                .mappings
                .list(kind)
                .iter()
                .find(|m| m.element() == c)
                .map(|m| m.id);
            match existing {
                Some(id) => {
                    st.map_sel = Some(id);
                    st.map_learn = false;
                }
                None => {
                    let flash = match c {
                        Control::Scene {
                            target: SceneTarget::Scene { id },
                        } => e.show().scene(id).is_some_and(|s| s.settings.flash),
                        _ => false,
                    };
                    let maps = &mut e.show_mut().mappings;
                    let id = maps.add(kind, c);
                    if let Some(m) = maps.find_mut(kind, id) {
                        m.flash = flash;
                        m.name = name;
                        // Programmer fader: fixed to the channels currently controlled
                        if !channels.is_empty() {
                            m.element = Some(c);
                            m.control = Control::Channels;
                            m.channels = channels;
                        }
                        if matches!(
                            c,
                            Control::Master
                                | Control::AudioMaster
                                | Control::Channel { .. }
                                | Control::Ui { fader: true, .. }
                        ) && kind == MapKind::Midi
                        {
                            m.mode = ButtonMode::Value;
                        }
                    }
                    st.map_sel = Some(id);
                    st.map_learn = true;
                }
            }
        }
        self.clear_leds();
        self.keyboard(ctx, e, st);
        let events = self.midi.drain();
        for ev in events {
            self.midi_event(&ev, e, st);
        }
        // Touch surface: buttons and faders trigger like keys
        for (m, pressed, value, cc) in std::mem::take(&mut st.touch_fire) {
            fire(e, st, &mut self.ui_fires, &m, pressed, value, cc);
        }
        // Clear touch elements of deleted scenes/fixtures
        let show = e.show();
        let stale_touch = show.touch.pages.iter().flat_map(|p| &p.items).any(|i| {
            i.fixtures.iter().any(|f| show.fixture(*f).is_none())
                || match i.control {
                    Some(Control::Scene {
                        target: SceneTarget::Scene { id },
                    }) => show.scene(id).is_none(),
                    Some(Control::Fixture { id }) => show.fixture(id).is_none(),
                    Some(Control::Channel { channel }) => show.fixture(channel.fixture).is_none(),
                    _ => false,
                }
        });
        if stale_touch {
            let scenes: Vec<SceneId> = show.scenes.iter().map(|s| s.id).collect();
            let fixtures: Vec<dmxcraft_core::FixtureId> =
                show.fixtures.iter().map(|f| f.id).collect();
            e.show_mut()
                .touch
                .retain(|s| scenes.contains(&s), |f| fixtures.contains(&f));
        }
        let fires = std::mem::take(&mut self.ui_fires);
        if !fires.is_empty() {
            ctx.request_repaint();
        }
        ctx.data_mut(|d| d.insert_temp(fires_id(), fires));
        let states: HashMap<u64, bool> = ctx.data(|d| d.get_temp(states_id())).unwrap_or_default();
        self.feedback(e, st, &states);
    }

    /// Clear the LEDs of newly connected outputs (setting per device).
    fn clear_leds(&mut self) {
        let connected = &self.midi.output_names;
        self.cleared.retain(|o| connected.contains(o));
        let todo: Vec<(String, midi_device::DeviceModel)> = self
            .devices
            .iter()
            .filter(|d| {
                d.clear_on_connect
                    && !d.output.is_empty()
                    && connected.contains(&d.output)
                    && !self.cleared.contains(&d.output)
            })
            .map(|d| (d.output.clone(), d.model))
            .collect();
        for (out, model) in todo {
            self.clear_now(&out, model);
        }
    }

    /// Switch off all LEDs of an output, then send the states again.
    pub fn clear_now(&mut self, output: &str, model: midi_device::DeviceModel) {
        for msg in model.clear_messages() {
            self.midi.send(output, &msg);
        }
        self.cleared.insert(output.to_string());
        self.sent.clear();
    }

    fn keyboard(&mut self, ctx: &egui::Context, e: &mut Engine, st: &mut AppState) {
        let typing = ctx.egui_wants_keyboard_input();
        let events: Vec<(egui::Key, bool, egui::Modifiers)> = ctx.input(|i| {
            i.events
                .iter()
                .filter_map(|ev| match ev {
                    egui::Event::Key {
                        key,
                        pressed,
                        repeat: false,
                        modifiers,
                        ..
                    } => Some((*key, *pressed, *modifiers)),
                    _ => None,
                })
                .collect()
        });
        for (key, pressed, mods) in events {
            // Learning: the next key gets mapped (Esc cancels)
            if st.map_learn && st.mapping == Some(MapKind::Keyboard) {
                if !pressed {
                    continue;
                }
                if key != egui::Key::Escape {
                    let input = Input::Key {
                        key: key.name().to_string(),
                        ctrl: mods.command,
                        shift: mods.shift,
                        alt: mods.alt,
                    };
                    if let Some(id) = st.map_sel {
                        if let Some(m) = e.show_mut().mappings.find_mut(MapKind::Keyboard, id) {
                            m.input = Some(input);
                        }
                    }
                }
                st.map_learn = false;
                continue;
            }
            if typing {
                continue;
            }
            let name = key.name();
            let hits: Vec<Mapping> = e
                .show()
                .mappings
                .keyboard
                .iter()
                .filter(|m| match &m.input {
                    Some(Input::Key {
                        key,
                        ctrl,
                        shift,
                        alt,
                    }) => {
                        key == name
                            && (*ctrl == mods.command || !pressed)
                            && (*shift == mods.shift || !pressed)
                            && (*alt == mods.alt || !pressed)
                    }
                    _ => false,
                })
                .cloned()
                .collect();
            for m in hits {
                if pressed {
                    self.held.push(m.id);
                } else if !self.held.contains(&m.id) {
                    continue;
                } else {
                    self.held.retain(|h| *h != m.id);
                }
                let v = if pressed { 127 } else { 0 };
                fire(e, st, &mut self.ui_fires, &m, pressed, v, false);
            }
        }
    }

    fn midi_event(&mut self, ev: &MidiEvent, e: &mut Engine, st: &mut AppState) {
        if st.map_learn && st.mapping == Some(MapKind::Midi) {
            // Suggest the scene color as the “on” color
            let color_hint = st.map_sel.and_then(|id| {
                let show = e.show();
                let m = show.mappings.midi.iter().find(|m| m.id == id)?;
                match m.element() {
                    Control::Scene {
                        target: SceneTarget::Scene { id },
                    } => {
                        let s = show.scene(id)?;
                        Some(s.color.unwrap_or_else(|| {
                            let c = theme::zone_color(show, s.zone);
                            [c.r(), c.g(), c.b()]
                        }))
                    }
                    _ => None,
                }
            });
            // only learn “press” or knob movement
            if ev.value > 0 || ev.message == MidiMessage::Cc {
                if let Some(id) = st.map_sel {
                    if let Some(m) = e.show_mut().mappings.find_mut(MapKind::Midi, id) {
                        m.input = Some(Input::Midi {
                            device: ev.device.clone(),
                            channel: ev.channel,
                            message: ev.message,
                            number: ev.number,
                        });
                        if ev.message == MidiMessage::Cc
                            && matches!(
                                m.control,
                                Control::Master
                                    | Control::AudioMaster
                                    | Control::Channel { .. }
                                    | Control::Ui { fader: true, .. }
                            )
                        {
                            m.mode = ButtonMode::Value;
                        }
                        // Suggest LED feedback: output of the device
                        if m.feedback.is_none() && ev.message == MidiMessage::Note {
                            m.feedback = default_feedback(
                                &self.devices,
                                &self.midi.output_names,
                                &ev.device,
                                color_hint,
                            );
                        }
                    }
                }
                st.map_learn = false;
            }
            return;
        }
        let hits: Vec<Mapping> = e
            .show()
            .mappings
            .midi
            .iter()
            .filter(|m| {
                m.input
                    .as_ref()
                    .is_some_and(|i| i.matches_midi(&ev.device, ev.channel, ev.message, ev.number))
            })
            .cloned()
            .collect();
        let cc = ev.message == MidiMessage::Cc;
        for m in hits {
            // Sound master via fader: only take over once the fader reaches the value
            // (after the reset when a SUPER SCENE starts)
            if m.control == Control::AudioMaster && (cc || m.mode == ButtonMode::Value) {
                st.audio.fader_input(f32::from(ev.value) / 127.0);
                continue;
            }
            fire(e, st, &mut self.ui_fires, &m, ev.value > 0, ev.value, cc);
        }
    }

    /// LED feedback: send changed states. LEDs of deleted or reassigned mappings
    /// are switched off. In the mapping window the selected mapping shows the
    /// state currently being edited (preview).
    fn feedback(&mut self, e: &Engine, st: &AppState, states: &HashMap<u64, bool>) {
        if std::mem::take(&mut self.midi.new_outputs) {
            self.sent.clear();
        }
        let preview = match (st.mapping, st.map_sel) {
            (Some(MapKind::Midi), Some(id)) => Some((id, self.win.edit_on)),
            _ => None,
        };
        let mut want: HashMap<u32, Sent> = HashMap::new();
        for m in &e.show().mappings.midi {
            let (
                Some(fb),
                Some(Input::Midi {
                    message, number, ..
                }),
            ) = (&m.feedback, &m.input)
            else {
                continue;
            };
            if fb.device.is_empty() {
                continue;
            }
            let on = match preview {
                Some((id, p)) if id == m.id => p,
                _ => is_on(e, st, states, m),
            };
            let s = if on { fb.on } else { fb.off };
            want.insert(
                m.id,
                Sent {
                    device: fb.device.clone(),
                    cc: *message == MidiMessage::Cc,
                    number: *number & 0x7f,
                    channel: s.channel.clamp(1, 16),
                    velocity: s.velocity & 0x7f,
                },
            );
        }
        // switch off keys that are no longer used
        let gone: Vec<(u32, Sent)> = self
            .sent
            .iter()
            .filter(|(id, s)| want.get(id).is_none_or(|w| !w.same_key(s)))
            .map(|(id, s)| (*id, s.clone()))
            .collect();
        for (id, s) in gone {
            self.sent.remove(&id);
            if !want.values().any(|w| w.same_key(&s)) {
                self.midi.send(&s.device, &s.blank());
            }
        }
        for (id, w) in want {
            if self.sent.get(&id) != Some(&w) {
                self.midi.send(&w.device, &w.message());
                self.sent.insert(id, w);
            }
        }
    }
}

/// Last feedback sent to a key.
#[derive(Clone, Debug, PartialEq)]
struct Sent {
    device: String,
    cc: bool,
    number: u8,
    channel: u8,
    velocity: u8,
}

impl Sent {
    fn same_key(&self, o: &Sent) -> bool {
        self.device == o.device && self.cc == o.cc && self.number == o.number
    }
    fn message(&self) -> [u8; 3] {
        let base = if self.cc { 0xb0 } else { 0x90 };
        [base | (self.channel - 1), self.number, self.velocity]
    }
    /// Key off (channel 1, value 0).
    fn blank(&self) -> [u8; 3] {
        [if self.cc { 0xb0 } else { 0x90 }, self.number, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dmxcraft_core::Step;

    fn setup() -> (Engine, AppState, SceneId, SceneId, ZoneId) {
        let mut show = Show::new("T", 8);
        let z = show.add_zone("A");
        let a = show.add_scene("a", Some(z), vec![Step::new(Vec::new())]);
        let b = show.add_scene("b", Some(z), vec![Step::new(Vec::new())]);
        (Engine::new(show), AppState::new(String::new()), a, b, z)
    }

    fn mapping(control: Control, flash: bool) -> Mapping {
        Mapping {
            id: 1,
            control,
            element: None,
            input: None,
            mode: ButtonMode::Toggle,
            flash,
            feedback: None,
            name: None,
            channels: Vec::new(),
        }
    }

    #[test]
    fn toggle_and_flash() {
        let (mut e, mut st, a, _, _) = setup();
        let m = mapping(
            Control::Scene {
                target: SceneTarget::Scene { id: a },
            },
            false,
        );
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, false, 0, false);
        assert!(e.is_active(a));
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert!(!e.is_active(a));

        let m = mapping(m.control, true);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert!(e.is_active(a));
        fire(&mut e, &mut st, &mut HashMap::new(), &m, false, 0, false);
        assert!(!e.is_active(a));
    }

    #[test]
    fn index_targets() {
        let (e, mut st, a, b, z) = setup();
        let idx = SceneTarget::Index { zone: z, index: 1 };
        assert_eq!(resolve(e.show(), &st, &idx), vec![b]);
        let shown = SceneTarget::IndexInShownBank { index: 0 };
        assert_eq!(resolve(e.show(), &st, &shown), vec![a]);
        st.selected_scenes = [b].into();
        assert_eq!(resolve(e.show(), &st, &SceneTarget::Selected), vec![b]);
    }

    #[test]
    fn bank_step_wraps() {
        let (mut e, mut st, _, _, z) = setup();
        let m = mapping(Control::BankStep { delta: 1 }, false);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert_eq!(st.request_bank, Some(Some(z)));
        st.shown_bank = Some(z);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert_eq!(st.request_bank, Some(None));
    }

    #[test]
    fn ui_controls_get_value_or_press() {
        let (mut e, mut st, _, _, _) = setup();
        let mut fires = HashMap::new();
        let fader = mapping(
            Control::Ui {
                key: 7,
                fader: true,
            },
            false,
        );
        fire(&mut e, &mut st, &mut fires, &fader, true, 127, true);
        assert_eq!(fires[&7].value, Some(1.0));
        let button = mapping(
            Control::Ui {
                key: 8,
                fader: false,
            },
            false,
        );
        fire(&mut e, &mut st, &mut fires, &button, true, 127, false);
        assert!(fires[&8].press);
        fire(&mut e, &mut st, &mut fires, &button, false, 0, false);
        assert!(fires[&8].press);
    }

    #[test]
    fn channel_mapping_sets_live_value() {
        let (mut e, mut st, _, _, _) = setup();
        let ch = dmxcraft_core::ChannelRef {
            fixture: dmxcraft_core::FixtureId(1),
            channel: 0,
        };
        let m = mapping(Control::Channel { channel: ch }, false);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, true);
        assert_eq!(e.live().get(&ch), Some(&255));
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 64, true);
        assert_eq!(e.live().get(&ch), Some(&129));
        let m = mapping(Control::Channel { channel: ch }, false);
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert!(e.live().get(&ch).is_none());
    }

    #[test]
    fn feedback_messages() {
        let s = Sent {
            device: "APC".into(),
            cc: false,
            number: 56,
            channel: 7,
            velocity: 21,
        };
        assert_eq!(s.message(), [0x96, 56, 21]);
        assert_eq!(s.blank(), [0x90, 56, 0]);
    }

    #[test]
    fn fixed_channels_work_without_selection() {
        let (mut e, mut st, _, _, _) = setup();
        let chans = vec![
            dmxcraft_core::ChannelRef {
                fixture: dmxcraft_core::FixtureId(1),
                channel: 0,
            },
            dmxcraft_core::ChannelRef::virtual_dimmer(dmxcraft_core::FixtureId(2)),
        ];
        let mut m = mapping(Control::Channels, false);
        m.channels = chans.clone();
        assert!(st.selected.is_empty());
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, true);
        assert!(chans.iter().all(|c| e.live().get(c) == Some(&255)));
        // Key: on/off for all together
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert!(chans.iter().all(|c| e.live().get(c).is_none()));
    }
}
