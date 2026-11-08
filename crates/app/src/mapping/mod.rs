//! Tastatur- und MIDI-Zuordnung: Überlagerung über der Oberfläche (jedes
//! zuordenbare Element wird markiert und lässt sich anklicken), Lernen einer
//! Taste bzw. MIDI-Nachricht, Auslösen im Betrieb und LED-Rückmeldung.
//!
//! Bedienelemente melden sich mit [`target`] an. Ist die Überlagerung aktiv,
//! liegt darüber eine Klickfläche – das Element selbst reagiert dann nicht.

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

/// Farbe der Überlagerung: Tastatur rot, MIDI violett.
pub fn color(kind: MapKind) -> Color32 {
    match kind {
        MapKind::Keyboard => Color32::from_rgb(214, 64, 82),
        MapKind::Midi => Color32::from_rgb(134, 92, 214),
    }
}

/// Was die Überlagerung in diesem Bild zeigt (vom App-Rahmen gesetzt).
#[derive(Clone, Default)]
struct Overlay {
    kind: Option<MapKind>,
    selected: Option<Control>,
    /// Zugeordnete Elemente mit Kurzform der Eingabe.
    labels: Vec<(Control, String)>,
}

fn overlay_id() -> egui::Id {
    egui::Id::unique("mapping_overlay")
}

fn pick_id() -> egui::Id {
    egui::Id::unique("mapping_pick")
}

/// Angeklicktes Element mit Anzeigename (nur bei [`Control::Ui`]) und den
/// Kanälen, die es gerade steuert (Programmer-Fader).
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

/// Ausgelöste Eingaben für Bedienelemente der Oberfläche (für das nächste Bild).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiFire {
    /// Taste gedrückt (Knopf: wie ein Klick).
    pub press: bool,
    /// Wert 0–1 (Regler über CC bzw. „Wert in %“).
    pub value: Option<f32>,
}

fn fires_id() -> egui::Id {
    egui::Id::unique("mapping_ui_fires")
}

fn states_id() -> egui::Id {
    egui::Id::unique("mapping_ui_states")
}

/// Ausgelöste Eingabe eines Elements abholen.
fn take_fire(ctx: &egui::Context, key: u64) -> Option<UiFire> {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<HashMap<u64, UiFire>>(fires_id())
            .remove(&key)
    })
}

/// Zustand eines Elements für die LED-Rückmeldung merken.
fn report(ctx: &egui::Context, key: u64, on: bool) {
    ctx.data_mut(|d| {
        d.get_temp_mut_or_default::<HashMap<u64, bool>>(states_id())
            .insert(key, on);
    });
}

/// Knopf der Oberfläche zuordenbar machen. `key` ist ein fester Name wie
/// „programmer/mode/live“, `on` der Zustand für die LED, `name` liefert
/// (Kategorie, Ziel) für die Liste. Gibt `true` zurück, wenn eine
/// zugeordnete Taste gedrückt wurde (dann wie ein Klick behandeln).
pub fn button(
    ui: &egui::Ui,
    resp: &Response,
    key: &str,
    on: bool,
    name: impl FnOnce() -> (String, String),
) -> bool {
    button_rect(ui, resp.rect, key, on, name)
}

/// Wie [`button`] für eine Fläche ohne eigene Antwort (z. B. Geräte im Plan).
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
    // Eingaben wirken auch bei offenem Zuordnungsfenster (zum Ausprobieren)
    target_with(ui, rect, control, name);
    take_fire(ui.ctx(), key).is_some_and(|f| f.press || f.value.is_some_and(|v| v > 0.0))
}

/// Regler der Oberfläche zuordenbar machen. `value` ist der aktuelle Wert
/// 0–1. Gibt den neuen Wert zurück, wenn eine Eingabe kam (Regler: Wert,
/// Taste: zwischen 0 und 1 umschalten).
pub fn fader(
    ui: &egui::Ui,
    rect: Rect,
    key: &str,
    value: f32,
    name: impl FnOnce() -> (String, String),
) -> Option<f32> {
    fader_channels(ui, rect, key, value, &[], name)
}

/// Wie [`fader`] für einen Fader, der gerade bestimmte Kanäle steuert
/// (Programmer). Eine neue Zuordnung steuert dann fest diese Kanäle – auch
/// wenn die Geräte später nicht mehr ausgewählt sind. Im Fenster lässt sich
/// stattdessen „Ausgewählte Geräte“ wählen (folgt der Auswahl).
pub fn fader_channels(
    ui: &egui::Ui,
    rect: Rect,
    key: &str,
    value: f32,
    channels: &[dmxcraft_core::ChannelRef],
    name: impl FnOnce() -> (String, String),
) -> Option<f32> {
    let key = dmxcraft_core::mapping::ui_key(key);
    // Element = Fader mit genau diesen Kanälen (Schlüssel ^ Kanalliste)
    let element = if channels.is_empty() {
        key
    } else {
        key ^ dmxcraft_core::mapping::channels_key(channels)
    };
    report(ui.ctx(), key, value > 0.0);
    // Eingaben wirken auch bei offenem Zuordnungsfenster (zum Ausprobieren)
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

/// Bedienelement anmelden. Gibt `true` zurück, wenn die Überlagerung aktiv
/// ist (dann hat die Überlagerung den Klick, nicht das Element).
pub fn target(ui: &egui::Ui, rect: Rect, control: Control) -> bool {
    target_with(ui, rect, control, || (String::new(), String::new()))
}

/// Wie [`target`] für eine gedrehte Fläche (Grad um die Mitte), z. B. ein
/// gedrehtes Gerät im Bühnenplan.
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

/// Wie [`target`], mit Anzeigename für Bedienelemente der Oberfläche.
fn target_with(
    ui: &egui::Ui,
    rect: Rect,
    control: Control,
    name: impl FnOnce() -> (String, String),
) -> bool {
    target_shape(ui, rect, 0.0, control, name, &[])
}

/// Ecken eines um seine Mitte gedrehten Rechtecks.
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

/// Liegt ein Punkt im gedrehten Rechteck?
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
        // gedreht: mittig (die Ecke liegt sonst neben der Form)
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

/// Anzeigename einer Zuordnung: (Kategorie, Ziel).
pub fn describe_mapping(show: &Show, m: &Mapping) -> (String, String) {
    match (&m.control, &m.name) {
        // feste Kanäle: aktuelle Gerätenamen (Umbenennen wirkt sofort)
        (Control::Channels, _) if !m.channels.is_empty() => {
            let fixtures: std::collections::BTreeSet<_> =
                m.channels.iter().map(|c| c.fixture).collect();
            let label = channel_name(show, m.channels[0]);
            let label = if fixtures.len() > 1 {
                let chan = label.split_once(": ").map_or(label.as_str(), |x| x.1);
                crate::tr!("{n} Geräte: {chan}", n = fixtures.len(), chan = chan)
            } else {
                label
            };
            (crate::i18n::t("Programmer").to_string(), label)
        }
        (Control::Ui { .. } | Control::Channels, Some(n)) => n.clone(),
        (c, _) => describe(show, c),
    }
}

/// Name des Bedienelements für Listen: (Kategorie, Ziel).
pub fn describe(show: &Show, c: &Control) -> (String, String) {
    use crate::i18n::t;
    let zone = |z: ZoneId| {
        show.zone(z)
            .map_or_else(|| t("(gelöscht)").to_string(), |z| z.name.clone())
    };
    match c {
        Control::Scene { target } => {
            let what = match target {
                SceneTarget::Scene { id } => match show.scene(*id) {
                    Some(s) => {
                        let bank = s.zone.map(zone).unwrap_or_default();
                        format!("{}: {}", bank.to_uppercase(), s.name)
                    }
                    None => t("(gelöscht)").to_string(),
                },
                SceneTarget::Selected => t("Ausgewählte Szene").to_string(),
                SceneTarget::Index { zone: z, index } => {
                    crate::tr!(
                        "{bank}: Nr. {n}",
                        bank = zone(*z).to_uppercase(),
                        n = index + 1
                    )
                }
                SceneTarget::IndexInShownBank { index } => {
                    crate::tr!("Angezeigte Bank: Nr. {n}", n = index + 1)
                }
            };
            (t("Szene").to_string(), what)
        }
        Control::ShowBank { zone: z } => (
            t("Bank").to_string(),
            z.map_or_else(|| t("Alle").to_string(), zone),
        ),
        Control::BankStep { delta } => (
            t("Bank").to_string(),
            if *delta < 0 {
                t("Vorige Bank")
            } else {
                t("Nächste Bank")
            }
            .to_string(),
        ),
        Control::ZonePause { zone: z } => (
            t("Bank").to_string(),
            crate::tr!("{bank}: Pause", bank = zone(*z)),
        ),
        Control::ZoneStep { zone: z, delta } => (
            t("Bank").to_string(),
            if *delta < 0 {
                crate::tr!("{bank}: Schritt zurück", bank = zone(*z))
            } else {
                crate::tr!("{bank}: Schritt vor", bank = zone(*z))
            },
        ),
        Control::Blackout => (t("Global").to_string(), t("Blackout").to_string()),
        Control::Master => (t("Global").to_string(), t("Master").to_string()),
        Control::StopAll => (
            t("Global").to_string(),
            t("Alle Szenen stoppen").to_string(),
        ),
        Control::Ui { .. } => (t("Oberfläche").to_string(), t("Bedienelement").to_string()),
        Control::Fixture { id } => (
            t("Gerät").to_string(),
            show.fixture(*id)
                .map_or_else(|| t("(gelöscht)").to_string(), |f| f.name.clone()),
        ),
        Control::Channel { channel } => (t("Gerät").to_string(), channel_name(show, *channel)),
        Control::Channels => (t("Programmer").to_string(), t("Feste Kanäle").to_string()),
    }
}

/// „Gerät: Kanalname“.
pub fn channel_name(show: &Show, ch: dmxcraft_core::ChannelRef) -> String {
    let Some(f) = show.fixture(ch.fixture) else {
        return t_deleted();
    };
    let name = if ch.is_virtual() {
        crate::i18n::t("Dimmer (virtuell)").to_string()
    } else {
        show.type_of(f)
            .and_then(|ty| ty.channels.get(usize::from(ch.channel)))
            .map_or_else(|| format!("{}", ch.channel + 1), |c| c.name.clone())
    };
    format!("{}: {}", f.name, name)
}

fn t_deleted() -> String {
    crate::i18n::t("(gelöscht)").to_string()
}

/// Szenen einer Bank in der angezeigten Reihenfolge (ohne ausgeblendete).
fn zone_scenes(show: &Show, z: ZoneId) -> Vec<SceneId> {
    show.scenes
        .iter()
        .filter(|s| s.zone == Some(z) && !s.hidden)
        .map(|s| s.id)
        .collect()
}

/// Szenen, auf die ein Ziel gerade zeigt.
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

/// Zustand „an“ eines Elements (für die LED-Rückmeldung).
fn is_on(e: &Engine, st: &AppState, states: &HashMap<u64, bool>, m: &Mapping) -> bool {
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
        Control::Ui { key, .. } => states.get(key).copied().unwrap_or(false),
        Control::Fixture { id } => st.selected.contains(id),
        Control::Channel { channel } => e.live().get(channel).is_some_and(|v| *v > 0),
        Control::BankStep { .. } | Control::StopAll => false,
    }
}

/// Eingabe auslösen: `pressed` (Taste/Note an) und Wert 0–127.
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

/// LED-Rückmeldung für eine neu gelernte Taste: Ausgang des eingerichteten
/// Geräts (oder gleichnamiger Ausgang), „An“ in der Szenenfarbe.
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

/// Zustand der Zuordnung zwischen den Bildern.
#[derive(Default)]
pub struct Mapper {
    pub midi: MidiHub,
    /// Zuletzt gesendete LED-Zustände je MIDI-Zuordnung.
    sent: HashMap<u32, Sent>,
    /// Gedrückte Tasten (für Loslassen bei Flash).
    held: Vec<u32>,
    /// Zustand des Zuordnungsfensters.
    pub win: window::WindowState,
    /// Eingerichtete Controller (aus den Einstellungen).
    pub devices: Vec<MidiDevice>,
    /// Ausgänge, deren LEDs seit dem Verbinden gelöscht wurden.
    cleared: HashSet<String>,
    /// Ausgelöste Eingaben für Bedienelemente (gehen ans nächste Bild).
    ui_fires: HashMap<u64, UiFire>,
}

impl Mapper {
    /// Je Bild vor den Ansichten: Überlagerung bereitstellen.
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

    /// Je Bild nach den Ansichten: Klicks der Überlagerung, Tastatur und
    /// MIDI verarbeiten, Rückmeldung senden.
    pub fn end_frame(&mut self, ctx: &egui::Context, e: &mut Engine, st: &mut AppState) {
        self.midi.scan();
        // Zuordnungen gelöschter Szenen entfernen
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
        // angeklicktes Element: vorhandene Zuordnung wählen oder neue anlegen
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
                        // Programmer-Fader: fest die gerade gesteuerten Kanäle
                        if !channels.is_empty() {
                            m.element = Some(c);
                            m.control = Control::Channels;
                            m.channels = channels;
                        }
                        if matches!(
                            c,
                            Control::Master
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
        let fires = std::mem::take(&mut self.ui_fires);
        if !fires.is_empty() {
            ctx.request_repaint();
        }
        ctx.data_mut(|d| d.insert_temp(fires_id(), fires));
        let states: HashMap<u64, bool> = ctx.data(|d| d.get_temp(states_id())).unwrap_or_default();
        self.feedback(e, st, &states);
    }

    /// LEDs neu verbundener Ausgänge löschen (Einstellung je Gerät).
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

    /// Alle LEDs eines Ausgangs ausschalten, danach die Zustände neu senden.
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
            // Lernen: nächste Taste wird zugeordnet (Esc bricht ab)
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
            // Szenenfarbe als „An“-Farbe vorschlagen
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
            // nur „Drücken“ bzw. Reglerbewegung lernen
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
                                    | Control::Channel { .. }
                                    | Control::Ui { fader: true, .. }
                            )
                        {
                            m.mode = ButtonMode::Value;
                        }
                        // LED-Rückmeldung vorschlagen: Ausgang des Geräts
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
            fire(e, st, &mut self.ui_fires, &m, ev.value > 0, ev.value, cc);
        }
    }

    /// LED-Rückmeldung: geänderte Zustände senden. LEDs gelöschter oder
    /// umgelegter Zuordnungen werden ausgeschaltet. Im Zuordnungsfenster zeigt
    /// die gewählte Zuordnung den gerade bearbeiteten Zustand (Vorschau).
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
        // nicht mehr verwendete Tasten ausschalten
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

/// Zuletzt an eine Taste gesendete Rückmeldung.
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
    /// Taste aus (Kanal 1, Wert 0).
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
        // Taste: an/aus für alle zusammen
        fire(&mut e, &mut st, &mut HashMap::new(), &m, true, 127, false);
        assert!(chans.iter().all(|c| e.live().get(c).is_none()));
    }
}
