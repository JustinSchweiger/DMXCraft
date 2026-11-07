//! Eigene Geräteprofile (Profile Builder).
//!
//! Ein [`Profile`] beschreibt ein Gerät so, wie es der Fixture-Editor der
//! Open Fixture Library tut: allgemeine Angaben, physische Daten, Kanäle mit
//! ihren Funktionen (DMX-Bereichen) und Modi als Kanalreihenfolgen.
//!
//! * Gespeichert wird es als eigene Datei (`.dmxfixture`, JSON, abwärts-
//!   kompatibel über `#[serde(default)]`).
//! * [`Profile::to_ofl`] erzeugt eine gültige OFL-Gerätedatei (zum Teilen
//!   oder Einreichen bei der OFL), [`Profile::from_ofl`] liest eine ein.
//! * [`Profile::fixture_types`] liefert die Gerätetypen für Patch und Show –
//!   über denselben Konverter wie die eingebettete Bibliothek, damit die
//!   Kanalarten überall gleich erkannt werden.

use std::collections::{BTreeMap, HashSet};

use dmxcraft_core::{ChannelKind, FixtureType};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::convert::{convert_fixture, slug, OflError};

/// Dateiendung eigener Profile.
pub const PROFILE_EXT: &str = "dmxfixture";
/// Präfix der Gerätetyp-Schlüssel eigener Profile.
pub const KEY_PREFIX: &str = "user/";

const OFL_SCHEMA: &str =
    "https://raw.githubusercontent.com/OpenLightingProject/open-fixture-library/master/schemas/fixture.json";

/// Gerätekategorien der OFL (feste Liste).
pub const CATEGORIES: [&str; 17] = [
    "Barrel Scanner",
    "Blinder",
    "Color Changer",
    "Dimmer",
    "Effect",
    "Fan",
    "Flower",
    "Hazer",
    "Laser",
    "Matrix",
    "Moving Head",
    "Pixel Bar",
    "Scanner",
    "Smoke",
    "Stand",
    "Strobe",
    "Other",
];

/// Farben für `ColorIntensity` (OFL-Bezeichnungen).
pub const INTENSITY_COLORS: [&str; 13] = [
    "Red",
    "Green",
    "Blue",
    "White",
    "Warm White",
    "Cold White",
    "Amber",
    "UV",
    "Cyan",
    "Magenta",
    "Yellow",
    "Lime",
    "Indigo",
];

/// DMX-Anschlüsse (OFL-Bezeichnungen).
pub const DMX_CONNECTORS: [&str; 7] = [
    "3-pin",
    "5-pin",
    "3-pin and 5-pin",
    "3-pin XLR IP65",
    "5-pin XLR IP65",
    "3.5mm stereo jack",
    "RJ45",
];

/// Dateiname aus einem Gerätenamen (Kleinbuchstaben, Bindestriche).
pub fn file_slug(s: &str) -> String {
    let k = slug(s);
    if k.is_empty() {
        "geraet".into()
    } else {
        k
    }
}

/// Ein komplettes Geräteprofil.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub format_version: u32,
    /// Fester Schlüssel (Dateiname, Gerätetyp-Schlüssel). Wird beim ersten
    /// Speichern vergeben und danach nicht mehr geändert – auch nicht beim
    /// Umbenennen, damit gepatchte Geräte ihr Profil wiederfinden.
    pub key: String,
    pub manufacturer: String,
    pub name: String,
    pub short_name: String,
    pub categories: Vec<String>,
    pub author: String,
    pub comment: String,
    pub manual_url: String,
    pub product_url: String,
    /// Datum JJJJ-MM-TT.
    pub created: String,
    pub modified: String,
    pub physical: Physical,
    pub channels: Vec<ProfileChannel>,
    pub modes: Vec<ProfileMode>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            format_version: 1,
            key: String::new(),
            manufacturer: String::new(),
            name: String::new(),
            short_name: String::new(),
            categories: Vec::new(),
            author: String::new(),
            comment: String::new(),
            manual_url: String::new(),
            product_url: String::new(),
            created: String::new(),
            modified: String::new(),
            physical: Physical::default(),
            channels: Vec::new(),
            modes: Vec::new(),
        }
    }
}

/// Physische Daten. 0 bzw. leer = unbekannt (wird nicht exportiert).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Physical {
    /// Breite, Höhe, Tiefe in mm.
    pub dimensions: [f32; 3],
    /// kg
    pub weight: f32,
    /// W
    pub power: f32,
    pub dmx_connector: String,
    /// z. B. „LED“, „Halogen“.
    pub bulb: String,
    /// K
    pub color_temperature: f32,
    /// lm
    pub lumens: f32,
    pub lens: String,
    /// Abstrahlwinkel in Grad (min, max); gleich bei fester Optik.
    pub beam_angle: [f32; 2],
}

/// Ein Kanal mit seinen Funktionen.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfileChannel {
    pub name: String,
    /// Anzahl feiner Kanäle (0 = 8 Bit, 1 = 16 Bit, 2 = 24 Bit).
    pub fine: u8,
    /// Standardwert (grobes Byte).
    pub default: u8,
    pub capabilities: Vec<Capability>,
}

impl Default for ProfileChannel {
    fn default() -> Self {
        Self {
            name: String::new(),
            fine: 0,
            default: 0,
            capabilities: vec![Capability::default()],
        }
    }
}

impl ProfileChannel {
    pub fn new(name: impl Into<String>, cap: Capability) -> Self {
        Self {
            name: name.into(),
            fine: 0,
            default: 0,
            capabilities: vec![cap],
        }
    }

    /// Namen der feinen Kanäle („Pan fine“, „Pan fine^2“).
    pub fn fine_names(&self) -> Vec<String> {
        fine_names(&self.name, self.fine)
    }

    /// Kanalart, die DMXCraft daraus ableitet (wie der OFL-Konverter).
    pub fn kind(&self) -> ChannelKind {
        let main = self
            .capabilities
            .iter()
            .find(|c| c.kind != CapabilityKind::NoFunction)
            .or(self.capabilities.first());
        let Some(main) = main else {
            return ChannelKind::Generic;
        };
        if main.kind != CapabilityKind::WheelSlot {
            return main.channel_kind();
        }
        // wie der Konverter: Mehrheit der Slot-Arten des Rads
        let slots: BTreeMap<u8, SlotKind> = self
            .capabilities
            .iter()
            .filter(|c| c.kind == CapabilityKind::WheelSlot)
            .map(|c| (c.slot.max(1), c.slot_kind))
            .collect();
        let count = |k: SlotKind| slots.values().filter(|s| **s == k).count();
        let (color, gobo) = (count(SlotKind::Color), count(SlotKind::Gobo));
        if color >= gobo && color > 0 {
            ChannelKind::ColorWheel
        } else if gobo > 0 {
            ChannelKind::Gobo
        } else {
            ChannelKind::Generic
        }
    }

    /// Bereich nach dem letzten teilen: hängt eine neue Funktion an, die die
    /// obere Hälfte des letzten Bereichs übernimmt.
    pub fn split_last(&mut self) {
        let Some(last) = self.capabilities.last_mut() else {
            self.capabilities.push(Capability::default());
            return;
        };
        if last.end <= last.start {
            return;
        }
        let mid = last.start + (last.end - last.start) / 2;
        let end = last.end;
        last.end = mid;
        self.capabilities.push(Capability {
            start: mid + 1,
            end,
            ..Capability::default()
        });
    }
}

fn fine_names(name: &str, fine: u8) -> Vec<String> {
    (1..=fine)
        .map(|i| {
            if i == 1 {
                format!("{name} fine")
            } else {
                format!("{name} fine^{i}")
            }
        })
        .collect()
}

/// Ein DMX-Bereich eines Kanals mit seiner Bedeutung.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Capability {
    pub start: u8,
    pub end: u8,
    pub kind: CapabilityKind,
    /// Farbe bei `ColorIntensity` (z. B. „Red“).
    pub color: String,
    /// Shutter-Effekt bei `ShutterStrobe`.
    pub shutter: ShutterEffect,
    /// Rad-Slot (1-basiert) bei `WheelSlot`.
    pub slot: u8,
    pub slot_kind: SlotKind,
    /// Name des Slots, des Effekts oder der Farbe.
    pub label: String,
    /// Anzeigefarbe (#rrggbb) für Farb-Slots und Farbvorgaben.
    pub hex: String,
    /// Wert am Anfang/Ende des Bereichs als OFL-Angabe, z. B. „0deg“ / „540deg“,
    /// „slow“ / „fast“, „0%“ / „100%“. Leer = keine Angabe.
    pub from: String,
    pub to: String,
    pub comment: String,
}

impl Default for Capability {
    fn default() -> Self {
        Self {
            start: 0,
            end: 255,
            kind: CapabilityKind::Generic,
            color: String::new(),
            shutter: ShutterEffect::Strobe,
            slot: 1,
            slot_kind: SlotKind::Color,
            label: String::new(),
            hex: String::new(),
            from: String::new(),
            to: String::new(),
            comment: String::new(),
        }
    }
}

impl Capability {
    pub fn new(kind: CapabilityKind) -> Self {
        let mut c = Self::default();
        c.set_kind(kind);
        c
    }

    pub fn range(mut self, start: u8, end: u8) -> Self {
        self.start = start;
        self.end = end;
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = color.into();
        self
    }

    /// Typ wechseln und typische Werte vorbelegen.
    pub fn set_kind(&mut self, kind: CapabilityKind) {
        if self.kind == kind {
            return;
        }
        self.kind = kind;
        let (from, to) = kind.default_values();
        self.from = from.into();
        self.to = to.into();
        if kind == CapabilityKind::ColorIntensity && self.color.is_empty() {
            self.color = "Red".into();
        }
    }

    /// Kanalart nach den Regeln des OFL-Konverters.
    pub fn channel_kind(&self) -> ChannelKind {
        use CapabilityKind as C;
        match self.kind {
            C::Intensity => ChannelKind::Dimmer,
            C::ColorIntensity => match self.color.as_str() {
                "Red" => ChannelKind::Red,
                "Green" => ChannelKind::Green,
                "Blue" => ChannelKind::Blue,
                "White" | "Warm White" | "Cold White" => ChannelKind::White,
                "Amber" => ChannelKind::Amber,
                "UV" => ChannelKind::Uv,
                _ => ChannelKind::Generic,
            },
            C::ShutterStrobe | C::StrobeSpeed => ChannelKind::Strobe,
            C::Pan | C::PanContinuous => ChannelKind::Pan,
            C::Tilt | C::TiltContinuous => ChannelKind::Tilt,
            C::ColorPreset => ChannelKind::ColorWheel,
            C::WheelSlot => match self.slot_kind {
                SlotKind::Gobo => ChannelKind::Gobo,
                SlotKind::Color => ChannelKind::ColorWheel,
                _ => ChannelKind::Generic,
            },
            C::EffectSpeed | C::PanTiltSpeed | C::Speed => ChannelKind::Speed,
            _ => ChannelKind::Generic,
        }
    }

    fn to_ofl(&self, with_range: bool) -> Value {
        use CapabilityKind as C;
        let mut o = Map::new();
        if with_range {
            o.insert("dmxRange".into(), json!([self.start, self.end]));
        }
        o.insert("type".into(), json!(self.kind.ofl()));
        match self.kind {
            C::ColorIntensity => {
                o.insert("color".into(), json!(self.color));
            }
            C::ShutterStrobe => {
                o.insert("shutterEffect".into(), json!(self.shutter.ofl()));
            }
            C::WheelSlot => {
                o.insert("slotNumber".into(), json!(self.slot.max(1)));
            }
            C::Effect => {
                let name = if self.label.trim().is_empty() {
                    "Effect"
                } else {
                    self.label.trim()
                };
                o.insert("effectName".into(), json!(name));
            }
            C::ColorPreset if !self.hex.is_empty() => {
                o.insert("colors".into(), json!([self.hex]));
            }
            _ => {}
        }
        if let Some(prop) = self.kind.property() {
            let (a, b) = (self.from.trim(), self.to.trim());
            match (a.is_empty(), b.is_empty()) {
                (false, false) if a != b => {
                    o.insert(format!("{prop}Start"), entity(a));
                    o.insert(format!("{prop}End"), entity(b));
                }
                (false, _) => {
                    o.insert(prop.into(), entity(a));
                }
                (true, false) => {
                    o.insert(prop.into(), entity(b));
                }
                _ => {}
            }
        }
        let mut comment = self.comment.trim().to_string();
        if self.kind == C::ColorPreset && !self.label.trim().is_empty() && comment.is_empty() {
            comment = self.label.trim().to_string();
        }
        if !comment.is_empty() {
            o.insert("comment".into(), json!(comment));
        }
        Value::Object(o)
    }

    fn from_ofl(v: &Value, channel: &str, wheels: Option<&Value>) -> Capability {
        let s = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let ty = s("type");
        let kind = CapabilityKind::from_ofl(&ty);
        let mut c = Capability {
            kind,
            ..Capability::default()
        };
        if let Some(r) = v.get("dmxRange").and_then(Value::as_array) {
            let n = |i: usize| r.get(i).and_then(Value::as_u64).unwrap_or(0).min(255) as u8;
            c.start = n(0);
            c.end = n(1);
        }
        c.color = s("color");
        c.shutter = ShutterEffect::from_ofl(&s("shutterEffect"));
        c.comment = s("comment");
        c.label = s("effectName");
        if let Some(col) = v
            .get("colors")
            .and_then(Value::as_array)
            .and_then(|a| a.first())
            .and_then(Value::as_str)
        {
            c.hex = col.to_string();
        }
        if kind == CapabilityKind::WheelSlot {
            let nr = v.get("slotNumber").and_then(Value::as_f64).unwrap_or(1.0);
            c.slot = nr.floor().clamp(1.0, 255.0) as u8;
            let wheel = v.get("wheel").and_then(Value::as_str).unwrap_or(channel);
            let slot = wheels
                .and_then(|w| w.get(wheel))
                .and_then(|w| w.get("slots"))
                .and_then(Value::as_array)
                .and_then(|a| a.get(usize::from(c.slot) - 1));
            if let Some(slot) = slot {
                let t = slot.get("type").and_then(Value::as_str).unwrap_or("");
                c.slot_kind = SlotKind::from_ofl(t);
                c.label = slot
                    .get("name")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                if let Some(col) = slot
                    .get("colors")
                    .and_then(Value::as_array)
                    .and_then(|a| a.first())
                    .and_then(Value::as_str)
                {
                    c.hex = col.to_string();
                }
            }
        }
        if let Some(prop) = kind.property() {
            let get = |k: &str| v.get(k).map(entity_text).unwrap_or_default();
            let single = get(prop);
            if single.is_empty() {
                c.from = get(&format!("{prop}Start"));
                c.to = get(&format!("{prop}End"));
            } else {
                c.from = single.clone();
                c.to = single;
            }
        }
        c
    }
}

/// Zahlen bleiben Zahlen, alles andere Text („540deg“, „fast“).
fn entity(s: &str) -> Value {
    match s.parse::<f64>() {
        Ok(n) if n.fract() == 0.0 => json!(n as i64),
        Ok(n) => json!(n),
        Err(_) => json!(s),
    }
}

fn entity_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::new(),
    }
}

/// Bedeutung eines DMX-Bereichs (OFL-Capability-Typen, Auswahl).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityKind {
    NoFunction,
    Intensity,
    ColorIntensity,
    ColorPreset,
    ColorTemperature,
    ShutterStrobe,
    StrobeSpeed,
    Pan,
    PanContinuous,
    Tilt,
    TiltContinuous,
    PanTiltSpeed,
    WheelSlot,
    WheelRotation,
    Effect,
    EffectSpeed,
    Prism,
    PrismRotation,
    Focus,
    Zoom,
    Iris,
    Frost,
    Fog,
    FogOutput,
    Rotation,
    Speed,
    Maintenance,
    #[default]
    Generic,
}

impl CapabilityKind {
    pub const ALL: [CapabilityKind; 28] = [
        CapabilityKind::Intensity,
        CapabilityKind::ColorIntensity,
        CapabilityKind::ColorPreset,
        CapabilityKind::ColorTemperature,
        CapabilityKind::ShutterStrobe,
        CapabilityKind::StrobeSpeed,
        CapabilityKind::Pan,
        CapabilityKind::PanContinuous,
        CapabilityKind::Tilt,
        CapabilityKind::TiltContinuous,
        CapabilityKind::PanTiltSpeed,
        CapabilityKind::WheelSlot,
        CapabilityKind::WheelRotation,
        CapabilityKind::Effect,
        CapabilityKind::EffectSpeed,
        CapabilityKind::Prism,
        CapabilityKind::PrismRotation,
        CapabilityKind::Focus,
        CapabilityKind::Zoom,
        CapabilityKind::Iris,
        CapabilityKind::Frost,
        CapabilityKind::Fog,
        CapabilityKind::FogOutput,
        CapabilityKind::Rotation,
        CapabilityKind::Speed,
        CapabilityKind::Maintenance,
        CapabilityKind::Generic,
        CapabilityKind::NoFunction,
    ];

    /// Typname in der OFL-Datei.
    pub fn ofl(self) -> &'static str {
        use CapabilityKind as C;
        match self {
            C::NoFunction => "NoFunction",
            C::Intensity => "Intensity",
            C::ColorIntensity => "ColorIntensity",
            C::ColorPreset => "ColorPreset",
            C::ColorTemperature => "ColorTemperature",
            C::ShutterStrobe => "ShutterStrobe",
            C::StrobeSpeed => "StrobeSpeed",
            C::Pan => "Pan",
            C::PanContinuous => "PanContinuous",
            C::Tilt => "Tilt",
            C::TiltContinuous => "TiltContinuous",
            C::PanTiltSpeed => "PanTiltSpeed",
            C::WheelSlot => "WheelSlot",
            C::WheelRotation => "WheelRotation",
            C::Effect => "Effect",
            C::EffectSpeed => "EffectSpeed",
            C::Prism => "Prism",
            C::PrismRotation => "PrismRotation",
            C::Focus => "Focus",
            C::Zoom => "Zoom",
            C::Iris => "Iris",
            C::Frost => "Frost",
            C::Fog => "Fog",
            C::FogOutput => "FogOutput",
            C::Rotation => "Rotation",
            C::Speed => "Speed",
            C::Maintenance => "Maintenance",
            C::Generic => "Generic",
        }
    }

    pub fn from_ofl(s: &str) -> CapabilityKind {
        Self::ALL
            .into_iter()
            .find(|k| k.ofl() == s)
            .unwrap_or(CapabilityKind::Generic)
    }

    /// OFL-Eigenschaft für den Wert des Bereichs (mit `…Start`/`…End`).
    pub fn property(self) -> Option<&'static str> {
        use CapabilityKind as C;
        Some(match self {
            C::Intensity | C::ColorIntensity => "brightness",
            C::ColorTemperature => "colorTemperature",
            C::Pan | C::Tilt | C::Zoom => "angle",
            C::ShutterStrobe
            | C::StrobeSpeed
            | C::PanContinuous
            | C::TiltContinuous
            | C::PanTiltSpeed
            | C::WheelRotation
            | C::EffectSpeed
            | C::PrismRotation
            | C::Rotation
            | C::Speed => "speed",
            C::Focus => "distance",
            C::Iris => "openPercent",
            C::Frost => "frostIntensity",
            C::FogOutput => "fogOutput",
            _ => return None,
        })
    }

    /// Muss die OFL-Eigenschaft angegeben sein?
    pub fn property_required(self) -> bool {
        use CapabilityKind as C;
        !matches!(
            self,
            C::Intensity | C::ColorIntensity | C::ShutterStrobe | C::Effect
        ) && self.property().is_some()
    }

    /// Typische Werte beim Wechsel auf diesen Typ.
    pub fn default_values(self) -> (&'static str, &'static str) {
        use CapabilityKind as C;
        match self {
            C::Pan => ("0deg", "540deg"),
            C::Tilt => ("0deg", "270deg"),
            C::Zoom => ("narrow", "wide"),
            C::Focus => ("near", "far"),
            C::Iris => ("closed", "open"),
            C::Frost => ("0%", "100%"),
            C::FogOutput => ("weak", "strong"),
            C::ColorTemperature => ("warm", "cold"),
            C::PanTiltSpeed | C::Speed => ("fast", "slow"),
            C::PanContinuous
            | C::TiltContinuous
            | C::StrobeSpeed
            | C::WheelRotation
            | C::EffectSpeed
            | C::PrismRotation
            | C::Rotation => ("slow", "fast"),
            _ => ("", ""),
        }
    }
}

/// Shutter-Effekte (`ShutterStrobe`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShutterEffect {
    Open,
    Closed,
    #[default]
    Strobe,
    Pulse,
    RampUp,
    RampDown,
    RampUpDown,
    Lightning,
    Spikes,
    Burst,
}

impl ShutterEffect {
    pub const ALL: [ShutterEffect; 10] = [
        ShutterEffect::Open,
        ShutterEffect::Closed,
        ShutterEffect::Strobe,
        ShutterEffect::Pulse,
        ShutterEffect::RampUp,
        ShutterEffect::RampDown,
        ShutterEffect::RampUpDown,
        ShutterEffect::Lightning,
        ShutterEffect::Spikes,
        ShutterEffect::Burst,
    ];

    pub fn ofl(self) -> &'static str {
        match self {
            ShutterEffect::Open => "Open",
            ShutterEffect::Closed => "Closed",
            ShutterEffect::Strobe => "Strobe",
            ShutterEffect::Pulse => "Pulse",
            ShutterEffect::RampUp => "RampUp",
            ShutterEffect::RampDown => "RampDown",
            ShutterEffect::RampUpDown => "RampUpDown",
            ShutterEffect::Lightning => "Lightning",
            ShutterEffect::Spikes => "Spikes",
            ShutterEffect::Burst => "Burst",
        }
    }

    pub fn from_ofl(s: &str) -> ShutterEffect {
        Self::ALL
            .into_iter()
            .find(|k| k.ofl() == s)
            .unwrap_or_default()
    }
}

/// Art eines Rad-Slots.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlotKind {
    Open,
    #[default]
    Color,
    Gobo,
    Prism,
    Frost,
}

impl SlotKind {
    pub const ALL: [SlotKind; 5] = [
        SlotKind::Open,
        SlotKind::Color,
        SlotKind::Gobo,
        SlotKind::Prism,
        SlotKind::Frost,
    ];

    pub fn ofl(self) -> &'static str {
        match self {
            SlotKind::Open => "Open",
            SlotKind::Color => "Color",
            SlotKind::Gobo => "Gobo",
            SlotKind::Prism => "Prism",
            SlotKind::Frost => "Frost",
        }
    }

    pub fn from_ofl(s: &str) -> SlotKind {
        if s.contains("Gobo") {
            return SlotKind::Gobo;
        }
        Self::ALL
            .into_iter()
            .find(|k| k.ofl() == s)
            .unwrap_or_default()
    }
}

/// Ein DMX-Modus: Reihenfolge der Kanäle.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ProfileMode {
    /// Fest pro Profil, bestimmt den Gerätetyp-Schlüssel (bleibt beim Umbenennen).
    pub id: u32,
    pub name: String,
    pub short_name: String,
    /// Kanalnamen (auch feine wie „Pan fine“); leer = unbenutzter Kanal.
    pub channels: Vec<String>,
}

/// Fehler im Profil, die das Speichern verhindern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    NoManufacturer,
    NoName,
    NoCategory,
    NoChannels,
    NoModes,
    EmptyChannelName,
    DuplicateChannel(String),
    NoCapabilities(String),
    /// Kanal, erster nicht abgedeckter Wert
    Gap(String, u8),
    /// Kanal, Wert, an dem sich Bereiche überschneiden oder rückwärts laufen
    Overlap(String, u8),
    /// Kanal, Wert, dem eine nötige Angabe fehlt (z. B. Winkel bei Pan)
    MissingValue(String, u8),
    EmptyModeName,
    DuplicateMode(String),
    EmptyMode(String),
    TooManyChannels(String),
    /// Modus, Kanal
    UnknownChannel(String, String),
    DuplicateInMode(String, String),
}

impl Profile {
    /// Name des Geräts in Listen.
    pub fn display_name(&self) -> String {
        match (self.manufacturer.trim(), self.name.trim()) {
            ("", n) => n.to_string(),
            (m, n) => format!("{m} {n}"),
        }
    }

    pub fn channel(&self, name: &str) -> Option<&ProfileChannel> {
        self.channels.iter().find(|c| c.name == name)
    }

    /// Alle Namen, die in einem Modus stehen dürfen (inkl. feiner Kanäle).
    pub fn available_names(&self) -> Vec<String> {
        let mut out = Vec::new();
        for c in &self.channels {
            out.push(c.name.clone());
            out.extend(c.fine_names());
        }
        out
    }

    /// Kanal und Feinstufe (0 = grob) zu einem Namen aus einem Modus.
    pub fn resolve(&self, name: &str) -> Option<(&ProfileChannel, u8)> {
        self.channels.iter().find_map(|c| {
            if c.name == name {
                return Some((c, 0));
            }
            c.fine_names()
                .iter()
                .position(|f| f == name)
                .map(|i| (c, i as u8 + 1))
        })
    }

    /// Kanalart eines Modus-Eintrags, wie DMXCraft sie verwendet.
    pub fn kind_of(&self, name: &str) -> ChannelKind {
        match self.resolve(name) {
            Some((c, 0)) => c.kind(),
            Some((c, _)) => match c.kind() {
                ChannelKind::Pan => ChannelKind::PanFine,
                ChannelKind::Tilt => ChannelKind::TiltFine,
                _ => ChannelKind::Generic,
            },
            None => ChannelKind::Generic,
        }
    }

    /// Nächste freie Modus-Id.
    pub fn next_mode_id(&self) -> u32 {
        self.modes.iter().map(|m| m.id).max().map_or(1, |m| m + 1)
    }

    /// Kanal umbenennen und alle Verweise in den Modi mitziehen.
    pub fn rename_channel(&mut self, index: usize, new_name: &str) {
        let Some(ch) = self.channels.get_mut(index) else {
            return;
        };
        let old: Vec<String> = std::iter::once(ch.name.clone())
            .chain(ch.fine_names())
            .collect();
        ch.name = new_name.to_string();
        let new: Vec<String> = std::iter::once(ch.name.clone())
            .chain(ch.fine_names())
            .collect();
        for m in &mut self.modes {
            for entry in &mut m.channels {
                if let Some(i) = old.iter().position(|o| o == entry) {
                    *entry = new[i].clone();
                }
            }
        }
    }

    /// Kanal entfernen (auch aus allen Modi).
    pub fn remove_channel(&mut self, index: usize) {
        if index >= self.channels.len() {
            return;
        }
        let ch = self.channels.remove(index);
        let names: HashSet<String> = std::iter::once(ch.name.clone())
            .chain(ch.fine_names())
            .collect();
        // gleichnamige Kanäle (Tippfehler-Duplikate) behalten ihre Verweise
        if self.channels.iter().any(|c| c.name == ch.name) {
            return;
        }
        for m in &mut self.modes {
            m.channels.retain(|e| !names.contains(e));
        }
    }

    /// Auflösung ändern: feine Kanäle, die wegfallen, verschwinden aus den Modi.
    pub fn set_fine(&mut self, index: usize, fine: u8) {
        let Some(ch) = self.channels.get_mut(index) else {
            return;
        };
        let old = ch.fine_names();
        ch.fine = fine.min(2);
        let keep: HashSet<String> = ch.fine_names().into_iter().collect();
        let gone: HashSet<String> = old.into_iter().filter(|n| !keep.contains(n)).collect();
        for m in &mut self.modes {
            m.channels.retain(|e| !gone.contains(e));
        }
    }

    /// Alle Fehler, die das Speichern verhindern.
    pub fn problems(&self) -> Vec<Problem> {
        let mut out = Vec::new();
        if self.manufacturer.trim().is_empty() {
            out.push(Problem::NoManufacturer);
        }
        if self.name.trim().is_empty() {
            out.push(Problem::NoName);
        }
        if self.categories.is_empty() {
            out.push(Problem::NoCategory);
        }
        if self.channels.is_empty() {
            out.push(Problem::NoChannels);
        }
        if self.modes.is_empty() {
            out.push(Problem::NoModes);
        }
        let mut seen = HashSet::new();
        for c in &self.channels {
            let name = c.name.trim();
            if name.is_empty() {
                out.push(Problem::EmptyChannelName);
                continue;
            }
            for n in std::iter::once(c.name.clone()).chain(c.fine_names()) {
                if !seen.insert(n.clone()) {
                    out.push(Problem::DuplicateChannel(n));
                }
            }
            if c.capabilities.is_empty() {
                out.push(Problem::NoCapabilities(c.name.clone()));
                continue;
            }
            let mut next: u16 = 0;
            for cap in &c.capabilities {
                if u16::from(cap.start) > next {
                    out.push(Problem::Gap(c.name.clone(), next as u8));
                } else if u16::from(cap.start) < next || cap.end < cap.start {
                    out.push(Problem::Overlap(c.name.clone(), cap.start));
                }
                if cap.kind.property_required()
                    && cap.from.trim().is_empty()
                    && cap.to.trim().is_empty()
                {
                    out.push(Problem::MissingValue(c.name.clone(), cap.start));
                }
                next = next.max(u16::from(cap.end) + 1);
            }
            if next <= 255 {
                out.push(Problem::Gap(c.name.clone(), next as u8));
            }
        }
        let available: HashSet<String> = self.available_names().into_iter().collect();
        let mut mode_names = HashSet::new();
        for m in &self.modes {
            if m.name.trim().is_empty() {
                out.push(Problem::EmptyModeName);
            } else if !mode_names.insert(m.name.trim().to_lowercase()) {
                out.push(Problem::DuplicateMode(m.name.clone()));
            }
            if m.channels.is_empty() {
                out.push(Problem::EmptyMode(m.name.clone()));
            }
            if m.channels.len() > 512 {
                out.push(Problem::TooManyChannels(m.name.clone()));
            }
            let mut in_mode = HashSet::new();
            for e in m.channels.iter().filter(|e| !e.is_empty()) {
                if !available.contains(e) {
                    out.push(Problem::UnknownChannel(m.name.clone(), e.clone()));
                } else if !in_mode.insert(e) {
                    out.push(Problem::DuplicateInMode(m.name.clone(), e.clone()));
                }
            }
        }
        out
    }

    /// Gültige OFL-Gerätedatei.
    pub fn to_ofl(&self) -> Value {
        let mut root = Map::new();
        root.insert("$schema".into(), json!(OFL_SCHEMA));
        root.insert("name".into(), json!(self.name.trim()));
        let short = self.short_name.trim();
        if !short.is_empty() && short != self.name.trim() {
            root.insert("shortName".into(), json!(short));
        }
        root.insert("categories".into(), json!(self.categories));
        let today = if self.modified.is_empty() {
            "2026-01-01".to_string()
        } else {
            self.modified.clone()
        };
        let created = if self.created.is_empty() {
            today.clone()
        } else {
            self.created.clone()
        };
        let author = if self.author.trim().is_empty() {
            "DMXCraft"
        } else {
            self.author.trim()
        };
        root.insert(
            "meta".into(),
            json!({ "authors": [author], "createDate": created, "lastModifyDate": today }),
        );
        if !self.comment.trim().is_empty() {
            root.insert("comment".into(), json!(self.comment.trim()));
        }
        let mut links = Map::new();
        if !self.manual_url.trim().is_empty() {
            links.insert("manual".into(), json!([self.manual_url.trim()]));
        }
        if !self.product_url.trim().is_empty() {
            links.insert("productPage".into(), json!([self.product_url.trim()]));
        }
        if !links.is_empty() {
            root.insert("links".into(), Value::Object(links));
        }
        let phys = self.physical.to_ofl();
        if !phys.is_empty() {
            root.insert("physical".into(), Value::Object(phys));
        }

        // Räder aus den WheelSlot-Bereichen (ein Rad pro Kanal)
        let mut wheels = Map::new();
        for c in &self.channels {
            let slots: BTreeMap<u8, &Capability> = c
                .capabilities
                .iter()
                .filter(|cap| cap.kind == CapabilityKind::WheelSlot)
                .map(|cap| (cap.slot.max(1), cap))
                .collect();
            let Some(&max) = slots.keys().last() else {
                continue;
            };
            let list: Vec<Value> = (1..=max)
                .map(|i| match slots.get(&i) {
                    Some(cap) => {
                        let mut s = Map::new();
                        s.insert("type".into(), json!(cap.slot_kind.ofl()));
                        if cap.slot_kind != SlotKind::Open && !cap.label.trim().is_empty() {
                            s.insert("name".into(), json!(cap.label.trim()));
                        }
                        if cap.slot_kind == SlotKind::Color && !cap.hex.is_empty() {
                            s.insert("colors".into(), json!([cap.hex]));
                        }
                        Value::Object(s)
                    }
                    None => json!({ "type": "Open" }),
                })
                .collect();
            wheels.insert(c.name.clone(), json!({ "slots": list }));
        }
        if !wheels.is_empty() {
            root.insert("wheels".into(), Value::Object(wheels));
        }

        let mut channels = Map::new();
        for c in &self.channels {
            let mut o = Map::new();
            if c.fine > 0 {
                o.insert("fineChannelAliases".into(), json!(c.fine_names()));
                o.insert("dmxValueResolution".into(), json!("8bit"));
            }
            if c.default != 0 {
                o.insert("defaultValue".into(), json!(c.default));
            }
            let full = c.capabilities.len() == 1
                && c.capabilities[0].start == 0
                && c.capabilities[0].end == 255;
            if full {
                o.insert("capability".into(), c.capabilities[0].to_ofl(false));
            } else {
                o.insert(
                    "capabilities".into(),
                    Value::Array(c.capabilities.iter().map(|cap| cap.to_ofl(true)).collect()),
                );
            }
            channels.insert(c.name.clone(), Value::Object(o));
        }
        root.insert("availableChannels".into(), Value::Object(channels));

        let modes: Vec<Value> = self
            .modes
            .iter()
            .map(|m| {
                let mut o = Map::new();
                o.insert("name".into(), json!(m.name.trim()));
                let short = m.short_name.trim();
                if !short.is_empty() && short != m.name.trim() {
                    o.insert("shortName".into(), json!(short));
                }
                let list: Vec<Value> = m
                    .channels
                    .iter()
                    .map(|e| if e.is_empty() { Value::Null } else { json!(e) })
                    .collect();
                o.insert("channels".into(), Value::Array(list));
                Value::Object(o)
            })
            .collect();
        root.insert("modes".into(), Value::Array(modes));
        Value::Object(root)
    }

    /// OFL-Datei als hübsch formatierter Text.
    pub fn to_ofl_string(&self) -> String {
        serde_json::to_string_pretty(&self.to_ofl()).unwrap_or_default()
    }

    /// Liest eine OFL-Gerätedatei. Matrix-Geräte (Pixel-Vorlagen) werden
    /// noch nicht unterstützt.
    pub fn from_ofl(manufacturer: &str, json: &str) -> Result<Profile, OflError> {
        let v: Value = serde_json::from_str(json)?;
        if v.get("redirectTo").is_some() {
            return Err(OflError::Skip("Weiterleitung auf anderes Gerät"));
        }
        if v.get("matrix").is_some() || v.get("templateChannels").is_some() {
            return Err(OflError::Invalid("Matrix-Geräte".into()));
        }
        let s = |o: &Value, k: &str| o.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let strings = |o: Option<&Value>| -> Vec<String> {
            o.and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .filter_map(|x| x.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default()
        };
        let name = s(&v, "name");
        if name.is_empty() {
            return Err(OflError::Invalid("kein Name".into()));
        }
        let meta = v.get("meta").cloned().unwrap_or(Value::Null);
        let links = v.get("links").cloned().unwrap_or(Value::Null);
        let mut p = Profile {
            manufacturer: manufacturer.to_string(),
            short_name: s(&v, "shortName"),
            name,
            categories: strings(v.get("categories")),
            author: strings(meta.get("authors")).join(", "),
            created: s(&meta, "createDate"),
            modified: s(&meta, "lastModifyDate"),
            comment: s(&v, "comment"),
            manual_url: strings(links.get("manual"))
                .into_iter()
                .next()
                .unwrap_or_default(),
            product_url: strings(links.get("productPage"))
                .into_iter()
                .next()
                .unwrap_or_default(),
            physical: Physical::from_ofl(v.get("physical")),
            ..Profile::default()
        };
        let wheels = v.get("wheels");
        let mut renames: Vec<(String, String)> = Vec::new();
        if let Some(chs) = v.get("availableChannels").and_then(Value::as_object) {
            for (ch_name, def) in chs {
                let fine = def
                    .get("fineChannelAliases")
                    .and_then(Value::as_array)
                    .map_or(0, |a| a.len().min(2) as u8);
                let res_8bit =
                    def.get("dmxValueResolution").and_then(Value::as_str) == Some("8bit");
                let shift = if res_8bit { 0 } else { 8 * u32::from(fine) };
                let default = match def.get("defaultValue") {
                    Some(Value::Number(n)) => (n.as_u64().unwrap_or(0) >> shift).min(255) as u8,
                    Some(Value::String(pct)) => pct
                        .trim_end_matches('%')
                        .trim()
                        .parse::<f64>()
                        .map(|x| (x.clamp(0.0, 100.0) / 100.0 * 255.0).round() as u8)
                        .unwrap_or(0),
                    _ => 0,
                };
                let mut caps: Vec<Capability> = if let Some(c) = def.get("capability") {
                    vec![Capability::from_ofl(c, ch_name, wheels)]
                } else {
                    def.get("capabilities")
                        .and_then(Value::as_array)
                        .map(|a| {
                            a.iter()
                                .map(|c| Capability::from_ofl(c, ch_name, wheels))
                                .collect()
                        })
                        .unwrap_or_default()
                };
                if def.get("capability").is_some() {
                    if let Some(c) = caps.first_mut() {
                        c.start = 0;
                        c.end = 255;
                    }
                } else if !res_8bit && fine > 0 {
                    // Bereiche in 16/24 Bit → aufs grobe Byte zurückrechnen
                    rescale_ranges(def, &mut caps, fine);
                }
                let ch = ProfileChannel {
                    name: ch_name.clone(),
                    fine,
                    default,
                    capabilities: caps,
                };
                // Aliasnamen der OFL-Datei auf unser Schema („X fine“) bringen
                let aliases = strings(def.get("fineChannelAliases"));
                for (alias, ours) in aliases.into_iter().zip(ch.fine_names()) {
                    if alias != ours {
                        renames.push((alias, ours));
                    }
                }
                p.channels.push(ch);
            }
        }
        let rename = |e: &str| -> String {
            renames
                .iter()
                .find(|(a, _)| a == e)
                .map_or(e.to_string(), |(_, o)| o.clone())
        };
        let mut modes = Vec::new();
        for (i, m) in v
            .get("modes")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            let channels = m
                .get("channels")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .map(|e| e.as_str().map(rename).unwrap_or_default())
                        .collect()
                })
                .unwrap_or_default();
            modes.push(ProfileMode {
                id: i as u32 + 1,
                name: s(m, "name"),
                short_name: s(m, "shortName"),
                channels,
            });
        }
        p.modes = modes;
        Ok(p)
    }

    /// Vorlage aus vorhandenen Gerätetypen (z. B. aus der Bibliothek, ohne
    /// Funktionsbereiche): Kanäle aus allen Modi, Feinkanäle von Pan/Tilt
    /// werden als 16-Bit-Auflösung übernommen.
    pub fn from_types(manufacturer: &str, name: &str, types: &[FixtureType]) -> Profile {
        let mut p = Profile {
            manufacturer: manufacturer.to_string(),
            name: name.to_string(),
            ..Profile::default()
        };
        // Name eines Feinkanals → Name in unserem Schema („Pan fine“)
        let mut fine_of: Vec<(String, String)> = Vec::new();
        for ty in types {
            for c in &ty.channels {
                let fine_parent = match c.kind {
                    ChannelKind::PanFine => last_matching(ty, c, ChannelKind::Pan),
                    ChannelKind::TiltFine => last_matching(ty, c, ChannelKind::Tilt),
                    _ => None,
                };
                if let Some(parent) = fine_parent {
                    if let Some(pc) = p.channels.iter_mut().find(|x| x.name == parent) {
                        pc.fine = pc.fine.max(1);
                    }
                    if !fine_of.iter().any(|(n, _)| *n == c.name) {
                        fine_of.push((c.name.clone(), format!("{parent} fine")));
                    }
                    continue;
                }
                if c.name == "(unbenutzt)" || p.channel(&c.name).is_some() {
                    continue;
                }
                let mut ch = ProfileChannel::new(c.name.clone(), cap_for_kind(c.kind, &c.name));
                ch.default = c.default;
                p.channels.push(ch);
            }
        }
        for (i, ty) in types.iter().enumerate() {
            let channels = ty
                .channels
                .iter()
                .map(|c| {
                    if c.name == "(unbenutzt)" {
                        return String::new();
                    }
                    fine_of
                        .iter()
                        .find(|(n, _)| *n == c.name)
                        .map_or(c.name.clone(), |(_, f)| f.clone())
                })
                .collect();
            p.modes.push(ProfileMode {
                id: i as u32 + 1,
                name: ty.mode.clone(),
                short_name: String::new(),
                channels,
            });
        }
        // Kategorie aus den Kanälen raten
        let has = |k: ChannelKind| types.iter().any(|t| t.has_kind(k));
        let cat = if has(ChannelKind::Pan) || has(ChannelKind::Tilt) {
            "Moving Head"
        } else if has(ChannelKind::Red) || has(ChannelKind::Green) {
            "Color Changer"
        } else if has(ChannelKind::Dimmer) {
            "Dimmer"
        } else {
            "Other"
        };
        p.categories = vec![cat.to_string()];
        p
    }

    /// Gerätetypen für Show und Patch (ein Typ pro Modus). Schlüssel:
    /// `user/<profil>/<modus-id>`.
    pub fn fixture_types(&self) -> Result<Vec<FixtureType>, OflError> {
        let key = if self.key.is_empty() {
            self.suggested_key()
        } else {
            self.key.clone()
        };
        let json = serde_json::to_string(&self.to_ofl())?;
        let lib = convert_fixture("user", self.manufacturer.trim(), &key, &json)?;
        if lib.modes.len() != self.modes.len() {
            return Err(OflError::Invalid("Modus ohne Kanäle".into()));
        }
        Ok(lib
            .modes
            .into_iter()
            .zip(&self.modes)
            .map(|(mut ty, m)| {
                ty.key = format!("{KEY_PREFIX}{key}/{}", m.id);
                ty
            })
            .collect())
    }

    /// Schlüssel aus Hersteller und Name (für neue Profile).
    pub fn suggested_key(&self) -> String {
        let k = format!("{}-{}", slug(&self.manufacturer), slug(&self.name));
        let k = k.trim_matches('-').to_string();
        if k.is_empty() {
            "geraet".into()
        } else {
            k
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }

    pub fn from_json(json: &str) -> Result<Profile, OflError> {
        Ok(serde_json::from_str(json)?)
    }
}

/// Der letzte grobe Kanal einer Art vor `fine` im selben Modus.
fn last_matching(
    ty: &FixtureType,
    fine: &dmxcraft_core::ChannelDef,
    kind: ChannelKind,
) -> Option<String> {
    let pos = ty.channels.iter().position(|c| std::ptr::eq(c, fine))?;
    ty.channels[..pos]
        .iter()
        .rev()
        .find(|c| c.kind == kind)
        .map(|c| c.name.clone())
}

/// Passende Funktion für eine DMXCraft-Kanalart (Vorlagen).
fn cap_for_kind(kind: ChannelKind, name: &str) -> Capability {
    use CapabilityKind as C;
    let cap = match kind {
        ChannelKind::Dimmer => Capability::new(C::Intensity),
        ChannelKind::Red => Capability::new(C::ColorIntensity).color("Red"),
        ChannelKind::Green => Capability::new(C::ColorIntensity).color("Green"),
        ChannelKind::Blue => Capability::new(C::ColorIntensity).color("Blue"),
        ChannelKind::White => Capability::new(C::ColorIntensity).color("White"),
        ChannelKind::Amber => Capability::new(C::ColorIntensity).color("Amber"),
        ChannelKind::Uv => Capability::new(C::ColorIntensity).color("UV"),
        ChannelKind::Pan | ChannelKind::PanFine => Capability::new(C::Pan),
        ChannelKind::Tilt | ChannelKind::TiltFine => Capability::new(C::Tilt),
        ChannelKind::Strobe => Capability::new(C::StrobeSpeed),
        ChannelKind::ColorWheel => Capability::new(C::ColorPreset),
        ChannelKind::Gobo => {
            let mut c = Capability::new(C::WheelSlot);
            c.slot_kind = SlotKind::Gobo;
            c.label = name.to_string();
            c
        }
        ChannelKind::Speed => Capability::new(C::Speed),
        ChannelKind::Generic => Capability::new(C::Generic),
    };
    cap.range(0, 255)
}

/// Bereiche in 16/24-Bit-Auflösung auf 8 Bit umrechnen (OFL-Standard ohne
/// `dmxValueResolution`).
fn rescale_ranges(def: &Value, caps: &mut [Capability], fine: u8) {
    let Some(list) = def.get("capabilities").and_then(Value::as_array) else {
        return;
    };
    let shift = 8 * u32::from(fine);
    for (cap, v) in caps.iter_mut().zip(list) {
        if let Some(r) = v.get("dmxRange").and_then(Value::as_array) {
            let n =
                |i: usize| (r.get(i).and_then(Value::as_u64).unwrap_or(0) >> shift).min(255) as u8;
            cap.start = n(0);
            cap.end = n(1);
        }
    }
}

impl Physical {
    fn to_ofl(&self) -> Map<String, Value> {
        let mut o = Map::new();
        let num = |x: f32| {
            if x.fract() == 0.0 {
                json!(x as i64)
            } else {
                json!(x)
            }
        };
        if self.dimensions.iter().all(|d| *d > 0.0) {
            o.insert(
                "dimensions".into(),
                Value::Array(self.dimensions.iter().map(|d| num(*d)).collect()),
            );
        }
        if self.weight > 0.0 {
            o.insert("weight".into(), num(self.weight));
        }
        if self.power > 0.0 {
            o.insert("power".into(), num(self.power));
        }
        if !self.dmx_connector.is_empty() {
            o.insert("DMXconnector".into(), json!(self.dmx_connector));
        }
        let mut bulb = Map::new();
        if !self.bulb.trim().is_empty() {
            bulb.insert("type".into(), json!(self.bulb.trim()));
        }
        if self.color_temperature > 0.0 {
            bulb.insert("colorTemperature".into(), num(self.color_temperature));
        }
        if self.lumens > 0.0 {
            bulb.insert("lumens".into(), num(self.lumens));
        }
        if !bulb.is_empty() {
            o.insert("bulb".into(), Value::Object(bulb));
        }
        let mut lens = Map::new();
        if !self.lens.trim().is_empty() {
            lens.insert("name".into(), json!(self.lens.trim()));
        }
        let [a, b] = self.beam_angle;
        if a > 0.0 || b > 0.0 {
            let (lo, hi) = if b > 0.0 {
                (a.min(b), a.max(b))
            } else {
                (a, a)
            };
            let lo = if lo > 0.0 { lo } else { hi };
            lens.insert("degreesMinMax".into(), json!([num(lo), num(hi)]));
        }
        if !lens.is_empty() {
            o.insert("lens".into(), Value::Object(lens));
        }
        o
    }

    fn from_ofl(v: Option<&Value>) -> Physical {
        let Some(v) = v else {
            return Physical::default();
        };
        let f = |o: Option<&Value>| o.and_then(Value::as_f64).unwrap_or(0.0) as f32;
        let mut p = Physical {
            weight: f(v.get("weight")),
            power: f(v.get("power")),
            dmx_connector: v
                .get("DMXconnector")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string(),
            ..Physical::default()
        };
        if let Some(d) = v.get("dimensions").and_then(Value::as_array) {
            for i in 0..3 {
                p.dimensions[i] = f(d.get(i));
            }
        }
        if let Some(b) = v.get("bulb") {
            p.bulb = b.get("type").and_then(Value::as_str).unwrap_or("").into();
            p.color_temperature = f(b.get("colorTemperature"));
            p.lumens = f(b.get("lumens"));
        }
        if let Some(l) = v.get("lens") {
            p.lens = l.get("name").and_then(Value::as_str).unwrap_or("").into();
            if let Some(d) = l.get("degreesMinMax").and_then(Value::as_array) {
                p.beam_angle = [f(d.first()), f(d.get(1))];
            }
        }
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot() -> Profile {
        let mut pan = ProfileChannel::new("Pan", Capability::new(CapabilityKind::Pan));
        pan.fine = 1;
        pan.default = 128;
        let mut gobo = ProfileChannel::new("Gobo", Capability::default());
        gobo.capabilities.clear();
        for (i, (s, e)) in [(0u8, 9u8), (10, 19), (20, 255)].into_iter().enumerate() {
            let mut c = Capability::new(CapabilityKind::WheelSlot).range(s, e);
            c.slot = i as u8 + 1;
            c.slot_kind = if i == 0 {
                SlotKind::Open
            } else {
                SlotKind::Gobo
            };
            if i > 0 {
                c.label = format!("Gobo {i}");
            }
            gobo.capabilities.push(c);
        }
        gobo.capabilities[2] = Capability::new(CapabilityKind::WheelRotation).range(20, 255);
        Profile {
            key: "test-spot".into(),
            manufacturer: "Test".into(),
            name: "Spot 1".into(),
            categories: vec!["Moving Head".into()],
            channels: vec![
                pan,
                ProfileChannel::new("Dimmer", Capability::new(CapabilityKind::Intensity)),
                ProfileChannel::new(
                    "Rot",
                    Capability::new(CapabilityKind::ColorIntensity).color("Red"),
                ),
                gobo,
            ],
            modes: vec![
                ProfileMode {
                    id: 1,
                    name: "Standard".into(),
                    channels: vec![
                        "Pan".into(),
                        "Pan fine".into(),
                        "Dimmer".into(),
                        "Rot".into(),
                        "Gobo".into(),
                    ],
                    ..ProfileMode::default()
                },
                ProfileMode {
                    id: 7,
                    name: "Basic".into(),
                    channels: vec!["Dimmer".into(), String::new()],
                    ..ProfileMode::default()
                },
            ],
            ..Profile::default()
        }
    }

    #[test]
    fn valid_profile_has_no_problems() {
        assert_eq!(spot().problems(), vec![]);
    }

    #[test]
    fn fixture_types_detect_kinds_and_keep_keys() {
        let types = spot().fixture_types().unwrap();
        assert_eq!(types.len(), 2);
        assert_eq!(types[0].key, "user/test-spot/1");
        assert_eq!(types[1].key, "user/test-spot/7");
        let kinds: Vec<ChannelKind> = types[0].channels.iter().map(|c| c.kind).collect();
        assert_eq!(
            kinds,
            vec![
                ChannelKind::Pan,
                ChannelKind::PanFine,
                ChannelKind::Dimmer,
                ChannelKind::Red,
                ChannelKind::Gobo
            ]
        );
        assert_eq!(types[0].channels[0].default, 128);
        assert_eq!(types[1].channel_count(), 2);
        // gleiche Erkennung wie die Vorschau im Builder
        let p = spot();
        for (c, name) in types[0].channels.iter().zip(&p.modes[0].channels) {
            assert_eq!(c.kind, p.kind_of(name), "{name}");
        }
    }

    #[test]
    fn ofl_round_trip() {
        let p = spot();
        let json = p.to_ofl_string();
        let back = Profile::from_ofl("Test", &json).unwrap();
        assert_eq!(back.channels, p.channels);
        assert_eq!(back.modes.len(), 2);
        assert_eq!(back.modes[0].channels, p.modes[0].channels);
        assert_eq!(back.modes[1].channels, p.modes[1].channels);
        assert_eq!(back.categories, p.categories);
    }

    #[test]
    fn own_format_round_trip() {
        let p = spot();
        assert_eq!(Profile::from_json(&p.to_json()).unwrap(), p);
        // ältere/fremde Dateien mit fehlenden Feldern
        let p = Profile::from_json(r#"{"name":"X","neu":1}"#).unwrap();
        assert_eq!(p.name, "X");
    }

    #[test]
    fn problems_found() {
        let mut p = spot();
        p.channels[1].capabilities[0].start = 5;
        p.channels[2].capabilities[0].end = 200;
        p.modes[1].channels.push("Gibt es nicht".into());
        p.modes[1].channels.push("Dimmer".into());
        p.channels[0].capabilities[0].from.clear();
        p.channels[0].capabilities[0].to.clear();
        let pr = p.problems();
        assert!(pr.contains(&Problem::Gap("Dimmer".into(), 0)));
        assert!(pr.contains(&Problem::Gap("Rot".into(), 201)));
        assert!(pr.contains(&Problem::MissingValue("Pan".into(), 0)));
        assert!(pr.contains(&Problem::UnknownChannel(
            "Basic".into(),
            "Gibt es nicht".into()
        )));
        assert!(pr.contains(&Problem::DuplicateInMode("Basic".into(), "Dimmer".into())));
    }

    #[test]
    fn rename_and_resolution_update_modes() {
        let mut p = spot();
        p.rename_channel(0, "Drehen");
        assert_eq!(p.modes[0].channels[..2], ["Drehen", "Drehen fine"]);
        p.set_fine(0, 0);
        assert_eq!(p.modes[0].channels.len(), 4);
        p.remove_channel(1);
        assert!(!p.modes[0].channels.contains(&"Dimmer".to_string()));
        assert_eq!(p.problems(), vec![]);
    }

    #[test]
    fn template_from_types() {
        let lib_types = spot().fixture_types().unwrap();
        let p = Profile::from_types("Test", "Kopie", &lib_types);
        assert_eq!(p.channels.len(), 4);
        assert_eq!(p.channel("Pan").unwrap().fine, 1);
        assert_eq!(p.modes[0].channels[1], "Pan fine");
        assert_eq!(p.modes[1].channels[1], "");
        assert_eq!(p.problems(), vec![]);
        let again = p.fixture_types().unwrap();
        let kinds = |t: &FixtureType| t.channels.iter().map(|c| c.kind).collect::<Vec<_>>();
        assert_eq!(kinds(&again[0]), kinds(&lib_types[0]));
    }

    #[test]
    fn import_real_ofl_with_16bit_ranges() {
        let json = r#"{
          "name": "Beam",
          "categories": ["Moving Head"],
          "meta": {"authors": ["A"], "createDate": "2020-01-01", "lastModifyDate": "2020-01-01"},
          "physical": {"lens": {"degreesMinMax": [4, 4]}},
          "availableChannels": {
            "Tilt": {"fineChannelAliases": ["Tilt LSB"], "defaultValue": 32768,
              "capabilities": [
                {"dmxRange": [0, 32767], "type": "Tilt", "angleStart": "0deg", "angleEnd": "135deg"},
                {"dmxRange": [32768, 65535], "type": "Tilt", "angleStart": "135deg", "angleEnd": "270deg"}
              ]}
          },
          "modes": [{"name": "16 bit", "channels": ["Tilt", "Tilt LSB"]}]
        }"#;
        let p = Profile::from_ofl("X", json).unwrap();
        let t = &p.channels[0];
        assert_eq!(t.default, 128);
        assert_eq!((t.capabilities[1].start, t.capabilities[1].end), (128, 255));
        assert_eq!(p.modes[0].channels, ["Tilt", "Tilt fine"]);
        assert_eq!(p.physical.beam_angle, [4.0, 4.0]);
        assert_eq!(p.problems(), vec![]);
    }
}
