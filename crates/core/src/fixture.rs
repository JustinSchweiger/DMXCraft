use serde::{Deserialize, Serialize};

/// Bedeutung eines Kanals. Bestimmt Merge-Verhalten (HTP/LTP) und
/// wie der Master auf den Kanal wirkt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelKind {
    Dimmer,
    Red,
    Green,
    Blue,
    White,
    Amber,
    Uv,
    Pan,
    PanFine,
    Tilt,
    TiltFine,
    Strobe,
    ColorWheel,
    Gobo,
    Speed,
    Generic,
}

impl ChannelKind {
    pub const ALL: [ChannelKind; 16] = [
        ChannelKind::Dimmer,
        ChannelKind::Red,
        ChannelKind::Green,
        ChannelKind::Blue,
        ChannelKind::White,
        ChannelKind::Amber,
        ChannelKind::Uv,
        ChannelKind::Pan,
        ChannelKind::PanFine,
        ChannelKind::Tilt,
        ChannelKind::TiltFine,
        ChannelKind::Strobe,
        ChannelKind::ColorWheel,
        ChannelKind::Gobo,
        ChannelKind::Speed,
        ChannelKind::Generic,
    ];

    /// Intensitätskanäle werden HTP (höchster Wert gewinnt) gemischt,
    /// alle anderen LTP (zuletzt aktivierte Szene gewinnt).
    pub fn is_htp(self) -> bool {
        matches!(self, ChannelKind::Dimmer)
    }

    /// Farbkanäle, die bei Geräten ohne eigenen Dimmer-Kanal vom Master
    /// skaliert werden („virtueller Dimmer“).
    pub fn is_color_intensity(self) -> bool {
        matches!(
            self,
            ChannelKind::Red
                | ChannelKind::Green
                | ChannelKind::Blue
                | ChannelKind::White
                | ChannelKind::Amber
                | ChannelKind::Uv
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            ChannelKind::Dimmer => "Dimmer",
            ChannelKind::Red => "Rot",
            ChannelKind::Green => "Grün",
            ChannelKind::Blue => "Blau",
            ChannelKind::White => "Weiß",
            ChannelKind::Amber => "Amber",
            ChannelKind::Uv => "UV",
            ChannelKind::Pan => "Pan",
            ChannelKind::PanFine => "Pan fein",
            ChannelKind::Tilt => "Tilt",
            ChannelKind::TiltFine => "Tilt fein",
            ChannelKind::Strobe => "Strobe",
            ChannelKind::ColorWheel => "Farbrad",
            ChannelKind::Gobo => "Gobo",
            ChannelKind::Speed => "Speed",
            ChannelKind::Generic => "Generisch",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelDef {
    pub name: String,
    pub kind: ChannelKind,
    /// Wert, wenn keine Szene und kein Programmer den Kanal setzt.
    #[serde(default)]
    pub default: u8,
}

impl ChannelDef {
    pub fn new(name: impl Into<String>, kind: ChannelKind) -> Self {
        Self {
            name: name.into(),
            kind,
            default: 0,
        }
    }

    pub fn with_default(mut self, default: u8) -> Self {
        self.default = default;
        self
    }
}

/// Ein Gerätetyp in einem bestimmten DMX-Modus.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureType {
    /// Eindeutiger Schlüssel, z. B. `generic/rgb-par-3ch`.
    pub key: String,
    pub manufacturer: String,
    pub model: String,
    pub mode: String,
    pub channels: Vec<ChannelDef>,
}

impl FixtureType {
    pub fn channel_count(&self) -> u16 {
        u16::try_from(self.channels.len()).unwrap_or(u16::MAX)
    }

    pub fn has_kind(&self, kind: ChannelKind) -> bool {
        self.channels.iter().any(|c| c.kind == kind)
    }

    pub fn display_name(&self) -> String {
        format!(
            "{} {} ({}, {} Kan.)",
            self.manufacturer,
            self.model,
            self.mode,
            self.channels.len()
        )
    }
}
