//! Tastatur- und MIDI-Zuordnungen: Tasten bzw. MIDI-Noten/-Regler lösen
//! Bedienelemente aus (Szene starten, Bank anzeigen, Blackout …). Gespeichert
//! in der Show; als Voreinstellung im- und exportierbar.

use serde::{Deserialize, Serialize};

use crate::show::{ChannelRef, GroupId, SceneId, ZoneId};
use crate::FixtureId;

/// Alle Zuordnungen einer Show.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mappings {
    pub keyboard: Vec<Mapping>,
    pub midi: Vec<Mapping>,
    /// Nächste freie ID.
    pub next_id: u32,
}

impl Mappings {
    pub fn is_empty(&self) -> bool {
        self.keyboard.is_empty() && self.midi.is_empty()
    }

    pub fn list(&self, kind: MapKind) -> &Vec<Mapping> {
        match kind {
            MapKind::Keyboard => &self.keyboard,
            MapKind::Midi => &self.midi,
        }
    }

    pub fn list_mut(&mut self, kind: MapKind) -> &mut Vec<Mapping> {
        match kind {
            MapKind::Keyboard => &mut self.keyboard,
            MapKind::Midi => &mut self.midi,
        }
    }

    /// Neue Zuordnung (ohne Eingabe – wird beim Lernen gesetzt).
    pub fn add(&mut self, kind: MapKind, control: Control) -> u32 {
        self.next_id = self.next_id.max(1);
        let id = self.next_id;
        self.next_id += 1;
        self.list_mut(kind).push(Mapping {
            id,
            control,
            element: None,
            input: None,
            mode: ButtonMode::Toggle,
            flash: false,
            feedback: None,
            name: None,
            channels: Vec::new(),
        });
        id
    }

    pub fn find_mut(&mut self, kind: MapKind, id: u32) -> Option<&mut Mapping> {
        self.list_mut(kind).iter_mut().find(|m| m.id == id)
    }

    /// Zuordnungen, deren Szenen nicht mehr existieren, entfernen.
    pub fn retain_scenes(&mut self, exists: impl Fn(SceneId) -> bool) {
        for list in [&mut self.keyboard, &mut self.midi] {
            list.retain(|m| match m.element() {
                Control::Scene {
                    target: SceneTarget::Scene { id },
                } => exists(id),
                _ => true,
            });
        }
    }
}

/// Tastatur oder MIDI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapKind {
    Keyboard,
    Midi,
}

/// Eine Zuordnung Eingabe → Bedienelement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    pub id: u32,
    pub control: Control,
    /// Angeklicktes Element, wenn das Ziel davon abweicht (z. B. Szene →
    /// „Index in der Bank“). Daran erkennt die Überlagerung die Zuordnung.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<Control>,
    /// `None`: noch nicht gelernt.
    #[serde(default)]
    pub input: Option<Input>,
    #[serde(default)]
    pub mode: ButtonMode,
    /// Nur solange gedrückt (Szene läuft, bis die Taste losgelassen wird).
    #[serde(default)]
    pub flash: bool,
    /// MIDI-Rückmeldung (LEDs am Controller).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Feedback>,
    /// Anzeigename (Kategorie, Ziel) bei Bedienelementen der Oberfläche
    /// ([`Control::Ui`]); andere Ziele werden aus der Show beschriftet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<(String, String)>,
    /// Feste Kanäle (Programmer-Fader, zugeordnet mit einer Geräteauswahl):
    /// Ziel bei [`Control::Channels`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<ChannelRef>,
}

impl Mapping {
    /// Element in der Oberfläche, zu dem die Zuordnung gehört.
    pub fn element(&self) -> Control {
        self.element.unwrap_or(self.control)
    }
}

/// Was ausgelöst wird.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Control {
    /// Szene starten/stoppen.
    Scene {
        target: SceneTarget,
    },
    /// Bank anzeigen (`None` = alle).
    ShowBank {
        zone: Option<ZoneId>,
    },
    /// Nächste/vorige Bank anzeigen.
    BankStep {
        delta: i32,
    },
    /// Laufende Szene einer Bank pausieren.
    ZonePause {
        zone: ZoneId,
    },
    /// Laufende Szene einer Bank einen Schritt weiter/zurück.
    ZoneStep {
        zone: ZoneId,
        delta: i32,
    },
    Blackout,
    /// Gruppe auswählen (`None` = alle Geräte); nochmal: Auswahl leeren.
    Group {
        id: Option<GroupId>,
    },
    /// Gerät im Plan: Auswahl umschalten.
    Fixture {
        id: FixtureId,
    },
    /// Einzelner Kanal eines Geräts (Live-Wert), unabhängig von der Auswahl.
    Channel {
        channel: ChannelRef,
    },
    /// Feste Kanäle der Zuordnung ([`Mapping::channels`]) als Live-Werte –
    /// wirkt unabhängig von der Auswahl.
    Channels,
    /// Bedienelement der Oberfläche (Knopf, Regler, Farbfeld …), erkannt
    /// über einen festen Schlüssel ([`ui_key`]). Das Element reagiert selbst,
    /// solange es angezeigt wird.
    Ui {
        key: u64,
        /// Regler (Wert) statt Knopf (Drücken).
        fader: bool,
    },
    /// Gesamthelligkeit (Wert der Eingabe).
    Master,
    /// Alle Szenen stoppen.
    StopAll,
}

/// Welche Szene: diese, die ausgewählte, oder nach Position in einer Bank.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SceneTarget {
    Scene {
        id: SceneId,
    },
    /// Die in der Oberfläche ausgewählte(n) Szene(n).
    Selected,
    /// n-te Szene (ab 0) einer Bank.
    Index {
        zone: ZoneId,
        index: usize,
    },
    /// n-te Szene der gerade angezeigten Bank.
    IndexInShownBank {
        index: usize,
    },
}

/// Taste oder MIDI-Nachricht.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Input {
    Key {
        /// Name der Taste (egui `Key::name`, z. B. „A“, „F1“, „Space“).
        key: String,
        #[serde(default)]
        ctrl: bool,
        #[serde(default)]
        shift: bool,
        #[serde(default)]
        alt: bool,
    },
    Midi {
        /// Gerätename (leer = jedes Gerät).
        #[serde(default)]
        device: String,
        /// Kanal 1–16.
        channel: u8,
        #[serde(default)]
        message: MidiMessage,
        /// Noten- bzw. Reglernummer 0–127.
        number: u8,
    },
}

/// Art der MIDI-Nachricht.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MidiMessage {
    #[default]
    Note,
    Cc,
}

/// Wie die Eingabe wirkt.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonMode {
    /// Drücken schaltet um.
    #[default]
    Toggle,
    /// Wert der Eingabe (Anschlag/Regler): über 0 an, 0 aus; Master: Helligkeit.
    Value,
}

/// LED-Rückmeldung: Nachricht je Zustand an ein MIDI-Ausgabegerät.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub device: String,
    pub off: FeedbackState,
    pub on: FeedbackState,
}

/// Kanal (1–16) und Anschlagstärke (0–127) für einen Zustand. Bei manchen
/// Controllern wählt der Kanal das Leuchtverhalten und die Stärke die Farbe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FeedbackState {
    pub channel: u8,
    pub velocity: u8,
}

impl Default for Feedback {
    fn default() -> Self {
        Self {
            device: String::new(),
            off: FeedbackState {
                channel: 1,
                velocity: 0,
            },
            on: FeedbackState {
                channel: 7,
                velocity: 21,
            },
        }
    }
}

/// Fester Schlüssel eines Bedienelements aus einem Namen wie
/// „programmer/fader/dimmer/1“ (FNV-1a, unabhängig von Version und Plattform).
pub fn ui_key(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// Prüfsumme einer Kanalliste (für den Schlüssel „Fader mit diesen Geräten“).
pub fn channels_key(channels: &[ChannelRef]) -> u64 {
    let mut s = String::new();
    for c in channels {
        s.push_str(&format!("{}.{};", c.fixture.0, c.channel));
    }
    ui_key(&s)
}

/// Notenname wie in vielen DAWs und Daslight-Listen: 60 = C3, 0 = C-2.
pub fn note_name(n: u8) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    format!("{}{}", NAMES[usize::from(n % 12)], i32::from(n / 12) - 2)
}

impl Input {
    /// Kurzform für Listen: „Strg+A“, „1/G#3 (Gerät)“, „1/CC 7“.
    pub fn label(&self) -> String {
        match self {
            Input::Key {
                key,
                ctrl,
                shift,
                alt,
            } => {
                let mut s = String::new();
                if *ctrl {
                    s.push_str("Ctrl+");
                }
                if *shift {
                    s.push_str("Shift+");
                }
                if *alt {
                    s.push_str("Alt+");
                }
                s.push_str(key);
                s
            }
            Input::Midi {
                device,
                channel,
                message,
                number,
            } => {
                let n = match message {
                    MidiMessage::Note => note_name(*number),
                    MidiMessage::Cc => format!("CC {number}"),
                };
                if device.is_empty() {
                    format!("{channel}/{n}")
                } else {
                    format!("{channel}/{n} ({device})")
                }
            }
        }
    }

    /// Passt eine eingehende MIDI-Nachricht (Gerät, Kanal, Art, Nummer)?
    pub fn matches_midi(&self, dev: &str, ch: u8, msg: MidiMessage, num: u8) -> bool {
        match self {
            Input::Midi {
                device,
                channel,
                message,
                number,
            } => {
                (device.is_empty() || device == dev)
                    && *channel == ch
                    && *message == msg
                    && *number == num
            }
            Input::Key { .. } => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_keys_are_stable() {
        assert_eq!(ui_key(""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(ui_key("a"), 0xaf63_dc4c_8601_ec8c);
        assert_ne!(ui_key("go"), ui_key("og"));
    }

    #[test]
    fn note_names() {
        assert_eq!(note_name(60), "C3");
        assert_eq!(note_name(0), "C-2");
        assert_eq!(note_name(56), "G#2");
    }

    #[test]
    fn add_find_and_json() {
        let mut m = Mappings::default();
        let id = m.add(
            MapKind::Midi,
            Control::Scene {
                target: SceneTarget::Scene { id: SceneId(3) },
            },
        );
        let k = m.add(MapKind::Keyboard, Control::Blackout);
        assert_ne!(id, k);
        m.find_mut(MapKind::Midi, id).unwrap().input = Some(Input::Midi {
            device: "APC".into(),
            channel: 1,
            message: MidiMessage::Note,
            number: 56,
        });
        let input = m.midi[0].input.as_ref().unwrap();
        assert!(input.matches_midi("APC", 1, MidiMessage::Note, 56));
        assert!(!input.matches_midi("APC", 2, MidiMessage::Note, 56));
        assert_eq!(input.label(), "1/G#2 (APC)");
        let j = serde_json::to_string(&m).unwrap();
        let back: Mappings = serde_json::from_str(&j).unwrap();
        assert_eq!(back, m);
        m.retain_scenes(|s| s != SceneId(3));
        assert!(m.midi.is_empty());
    }
}
