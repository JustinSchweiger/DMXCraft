//! Tastatur- und MIDI-Zuordnung: Überlagerung über der Oberfläche (jedes
//! zuordenbare Element wird markiert und lässt sich anklicken), Lernen einer
//! Taste bzw. MIDI-Nachricht, Auslösen im Betrieb und LED-Rückmeldung.
//!
//! Bedienelemente melden sich mit [`target`] an. Ist die Überlagerung aktiv,
//! liegt darüber eine Klickfläche – das Element selbst reagiert dann nicht.

pub mod window;

use std::collections::HashMap;

use dmxcraft_core::mapping::{
    ButtonMode, Control, Input, MapKind, Mapping, MidiMessage, SceneTarget,
};
use dmxcraft_core::{Engine, SceneId, Show, ZoneId};
use eframe::egui::{self, Color32, Rect, Sense, Stroke, StrokeKind};

use crate::midi::{MidiEvent, MidiHub};
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

/// Bedienelement anmelden. Gibt `true` zurück, wenn die Überlagerung aktiv
/// ist (dann hat die Überlagerung den Klick, nicht das Element).
pub fn target(ui: &egui::Ui, rect: Rect, control: Control) -> bool {
    let Some(o) = ui.ctx().data(|d| d.get_temp::<Overlay>(overlay_id())) else {
        return false;
    };
    let Some(kind) = o.kind else {
        return false;
    };
    let id = ui
        .scope_id()
        .with(("mapping_target", format!("{control:?}")));
    let resp = ui.interact(rect, id, Sense::click());
    let base = color(kind);
    let label = o.labels.iter().find(|(c, _)| *c == control).map(|l| &l.1);
    let selected = o.selected == Some(control);
    let alpha = if selected {
        0.75
    } else if label.is_some() {
        0.55
    } else if resp.hovered() {
        0.45
    } else {
        0.3
    };
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S, base.gamma_multiply(alpha));
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(if selected { 2.0 } else { 1.0 }, base),
        StrokeKind::Inside,
    );
    if let Some(l) = label {
        let g = p.layout_no_wrap(
            l.clone(),
            egui::FontId::proportional(theme::FS_CAPTION),
            Color32::WHITE,
        );
        let pos = rect.right_top() + egui::vec2(-g.size().x - 4.0, 2.0);
        p.galley(pos, g, Color32::WHITE);
    }
    if resp.clicked() {
        ui.ctx()
            .data_mut(|d| d.insert_temp(pick_id(), Some(control)));
    }
    true
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
    }
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
fn is_on(e: &Engine, st: &AppState, c: &Control) -> bool {
    match c {
        Control::Scene { target } => resolve(e.show(), st, target)
            .iter()
            .any(|id| e.is_active(*id)),
        Control::ShowBank { zone } => st.shown_bank == *zone,
        Control::ZonePause { zone } | Control::ZoneStep { zone, .. } => {
            e.active_in_zone(*zone).is_some()
        }
        Control::Blackout => e.blackout(),
        Control::Master => e.master() > 0.0,
        Control::BankStep { .. } | Control::StopAll => false,
    }
}

/// Eingabe auslösen: `pressed` (Taste/Note an) und Wert 0–127.
fn fire(e: &mut Engine, st: &mut AppState, m: &Mapping, pressed: bool, value: u8) {
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
    }
}

/// Zustand der Zuordnung zwischen den Bildern.
#[derive(Default)]
pub struct Mapper {
    pub midi: MidiHub,
    /// Zuletzt gesendete LED-Zustände je MIDI-Zuordnung.
    sent: HashMap<u32, bool>,
    /// Gedrückte Tasten (für Loslassen bei Flash).
    held: Vec<u32>,
    /// Zustand des Zuordnungsfensters.
    pub win: window::WindowState,
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
            d.remove_temp::<Option<Control>>(pick_id());
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
        if stale {
            let ids: Vec<SceneId> = show.scenes.iter().map(|s| s.id).collect();
            e.show_mut().mappings.retain_scenes(|id| ids.contains(&id));
        }
        // angeklicktes Element: vorhandene Zuordnung wählen oder neue anlegen
        if let (Some(kind), Some(Some(c))) = (
            st.mapping,
            ctx.data(|d| d.get_temp::<Option<Control>>(pick_id())),
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
                        if matches!(c, Control::Master) {
                            m.mode = ButtonMode::Value;
                        }
                    }
                    st.map_sel = Some(id);
                    st.map_learn = true;
                }
            }
        }
        self.keyboard(ctx, e, st);
        let events = self.midi.drain();
        for ev in events {
            self.midi_event(&ev, e, st);
        }
        self.feedback(e, st);
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
                fire(e, st, &m, pressed, if pressed { 127 } else { 0 });
            }
        }
    }

    fn midi_event(&mut self, ev: &MidiEvent, e: &mut Engine, st: &mut AppState) {
        if st.map_learn && st.mapping == Some(MapKind::Midi) {
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
                        if ev.message == MidiMessage::Cc && matches!(m.control, Control::Master) {
                            m.mode = ButtonMode::Value;
                        }
                    }
                }
                st.map_learn = false;
                self.sent.clear();
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
        for m in hits {
            fire(e, st, &m, ev.value > 0, ev.value);
        }
    }

    /// LED-Rückmeldung: geänderte Zustände (bzw. alle nach Geräteverbindung).
    fn feedback(&mut self, e: &Engine, st: &AppState) {
        if std::mem::take(&mut self.midi.new_outputs) {
            self.sent.clear();
        }
        let maps: Vec<Mapping> = e
            .show()
            .mappings
            .midi
            .iter()
            .filter(|m| m.feedback.as_ref().is_some_and(|f| !f.device.is_empty()))
            .cloned()
            .collect();
        for m in maps {
            let (
                Some(fb),
                Some(Input::Midi {
                    message, number, ..
                }),
            ) = (&m.feedback, &m.input)
            else {
                continue;
            };
            let on = is_on(e, st, &m.control);
            if self.sent.get(&m.id) == Some(&on) {
                continue;
            }
            let s = if on { fb.on } else { fb.off };
            let status = match message {
                MidiMessage::Note => 0x90,
                MidiMessage::Cc => 0xb0,
            } | (s.channel.clamp(1, 16) - 1);
            self.midi
                .send(&fb.device, &[status, *number & 0x7f, s.velocity & 0x7f]);
            self.sent.insert(m.id, on);
        }
    }

    /// Rückmeldung beim nächsten Bild komplett neu senden.
    pub fn resend(&mut self) {
        self.sent.clear();
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
        fire(&mut e, &mut st, &m, true, 127);
        fire(&mut e, &mut st, &m, false, 0);
        assert!(e.is_active(a));
        fire(&mut e, &mut st, &m, true, 127);
        assert!(!e.is_active(a));

        let m = mapping(m.control, true);
        fire(&mut e, &mut st, &m, true, 127);
        assert!(e.is_active(a));
        fire(&mut e, &mut st, &m, false, 0);
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
        fire(&mut e, &mut st, &m, true, 127);
        assert_eq!(st.request_bank, Some(Some(z)));
        st.shown_bank = Some(z);
        fire(&mut e, &mut st, &m, true, 127);
        assert_eq!(st.request_bank, Some(None));
    }
}
