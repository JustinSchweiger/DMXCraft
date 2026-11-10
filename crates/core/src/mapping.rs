//! Keyboard and MIDI mappings: keys or MIDI notes/controllers trigger
//! controls (start a scene, show a bank, blackout …). Stored in the show;
//! can be imported and exported as presets.

use serde::{Deserialize, Serialize};

use crate::show::{ChannelRef, GroupId, SceneId, ZoneId};
use crate::FixtureId;

/// All mappings of a show.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Mappings {
    pub keyboard: Vec<Mapping>,
    pub midi: Vec<Mapping>,
    /// Next free ID.
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

    /// New mapping (without input – set while learning).
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

    /// Remove mappings whose scenes no longer exist.
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

/// Keyboard or MIDI.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MapKind {
    Keyboard,
    Midi,
}

/// A mapping input → control.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mapping {
    pub id: u32,
    pub control: Control,
    /// Clicked element if the target differs from it (e.g. scene → “index in
    /// the bank”). The overlay recognizes the mapping by it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub element: Option<Control>,
    /// `None`: not learned yet.
    #[serde(default)]
    pub input: Option<Input>,
    #[serde(default)]
    pub mode: ButtonMode,
    /// Only while pressed (the scene runs until the key is released).
    #[serde(default)]
    pub flash: bool,
    /// MIDI feedback (LEDs on the controller).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Feedback>,
    /// Display name (category, target) for interface controls
    /// ([`Control::Ui`]); other targets are labeled from the show.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<(String, String)>,
    /// Fixed channels (programmer fader, mapped with a fixture selection):
    /// target for [`Control::Channels`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<ChannelRef>,
}

impl Mapping {
    /// Element in the UI the mapping belongs to.
    pub fn element(&self) -> Control {
        self.element.unwrap_or(self.control)
    }
}

/// What is triggered.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Control {
    /// Start/stop a scene.
    Scene {
        target: SceneTarget,
    },
    /// Show a bank (`None` = all).
    ShowBank {
        zone: Option<ZoneId>,
    },
    /// Show the next/previous bank.
    BankStep {
        delta: i32,
    },
    /// Pause the running scene of a bank.
    ZonePause {
        zone: ZoneId,
    },
    /// Running scene of a bank one step forward/back.
    ZoneStep {
        zone: ZoneId,
        delta: i32,
    },
    Blackout,
    /// Select a group (`None` = all fixtures); again: clear the selection.
    Group {
        id: Option<GroupId>,
    },
    /// Fixture in the plan: toggle the selection.
    Fixture {
        id: FixtureId,
    },
    /// Single channel of a fixture (live value), independent of the selection.
    Channel {
        channel: ChannelRef,
    },
    /// Fixed channels of the mapping ([`Mapping::channels`]) as live values –
    /// works independently of the selection.
    Channels,
    /// Interface control (button, slider, color field …), identified by a fixed
    /// key ([`ui_key`]). The element reacts itself while it is shown.
    Ui {
        key: u64,
        /// Slider (value) instead of button (press).
        fader: bool,
    },
    /// Overall brightness (value of the input).
    Master,
    /// Stop all scenes.
    StopAll,
    /// Sound master: volume of the audio blocks (value of the input).
    AudioMaster,
    /// Mute the sound.
    AudioMute,
    /// Fade out the sound evenly (duration in the settings).
    AudioFade,
    /// Volume of a player in the SOUND tab (`None` = master of the tab).
    SoundVolume {
        deck: Option<u8>,
    },
}

/// Which scene: this one, the selected one, or by position in a bank.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SceneTarget {
    Scene {
        id: SceneId,
    },
    /// The scene(s) selected in the UI.
    Selected,
    /// n-th scene (from 0) of a bank.
    Index {
        zone: ZoneId,
        index: usize,
    },
    /// n-th scene of the bank currently shown.
    IndexInShownBank {
        index: usize,
    },
}

/// Key or MIDI message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Input {
    Key {
        /// Name of the key (egui `Key::name`, e.g. “A”, “F1”, “Space”).
        key: String,
        #[serde(default)]
        ctrl: bool,
        #[serde(default)]
        shift: bool,
        #[serde(default)]
        alt: bool,
    },
    Midi {
        /// Device name (empty = any device).
        #[serde(default)]
        device: String,
        /// Channel 1–16.
        channel: u8,
        #[serde(default)]
        message: MidiMessage,
        /// Note or controller number 0–127.
        number: u8,
    },
}

/// Kind of the MIDI message.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MidiMessage {
    #[default]
    Note,
    Cc,
}

/// How the input works.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ButtonMode {
    /// Pressing toggles.
    #[default]
    Toggle,
    /// Value of the input (velocity/controller): above 0 on, 0 off; master:
    /// brightness.
    Value,
}

/// LED feedback: message per state to a MIDI output device.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Feedback {
    pub device: String,
    pub off: FeedbackState,
    pub on: FeedbackState,
}

/// Channel (1–16) and velocity (0–127) for a state. On some controllers the
/// channel chooses the light behavior and the velocity the color.
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

/// Fixed key of a control from a name such as “programmer/fader/dimmer/1”
/// (FNV-1a, independent of version and platform).
pub fn ui_key(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// Checksum of a channel list (for the key “fader with these fixtures”).
pub fn channels_key(channels: &[ChannelRef]) -> u64 {
    let mut s = String::new();
    for c in channels {
        s.push_str(&format!("{}.{};", c.fixture.0, c.channel));
    }
    ui_key(&s)
}

/// Note name as in many DAWs and Daslight lists: 60 = C3, 0 = C-2.
pub fn note_name(n: u8) -> String {
    const NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    format!("{}{}", NAMES[usize::from(n % 12)], i32::from(n / 12) - 2)
}

impl Input {
    /// Short form for lists: “Ctrl+A”, “1/G#3 (device)”, “1/CC 7”.
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

    /// Does an incoming MIDI message match (device, channel, kind, number)?
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
