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

/// Physische Angaben eines Geräts (für den 3D-Visualisierer).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Physics {
    /// Abstrahlwinkel in Grad (min, max); gleich bei fester Optik.
    pub beam_angle: [f32; 2],
    /// Schwenkbereich in Grad (0 = kein Pan).
    pub pan_range: f32,
    /// Neigebereich in Grad (0 = kein Tilt).
    pub tilt_range: f32,
    /// Breite, Höhe, Tiefe in mm (0 = unbekannt).
    pub size_mm: [f32; 3],
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            beam_angle: [25.0, 25.0],
            pan_range: 540.0,
            tilt_range: 270.0,
            size_mm: [0.0; 3],
        }
    }
}

impl Physics {
    /// Aus dem `physical`-Block einer OFL-Gerätedatei (fehlende Angaben = Standard).
    pub fn from_ofl(v: Option<&serde_json::Value>) -> Option<Self> {
        let v = v?;
        let num = |p: &[&str]| {
            let mut cur = v;
            for k in p {
                cur = cur.get(*k)?;
            }
            cur.as_f64().map(|f| f as f32)
        };
        let mut out = Self::default();
        let mut any = false;
        if let Some(a) = v
            .get("lens")
            .and_then(|l| l.get("degreesMinMax"))
            .and_then(|d| d.as_array())
        {
            let n: Vec<f32> = a
                .iter()
                .filter_map(|x| x.as_f64())
                .map(|f| f as f32)
                .collect();
            if n.len() == 2 && n[0] > 0.0 {
                out.beam_angle = [n[0], n[1].max(n[0])];
                any = true;
            }
        }
        if let Some(p) = num(&["focus", "panMax"]) {
            out.pan_range = p;
            any = true;
        }
        if let Some(t) = num(&["focus", "tiltMax"]) {
            out.tilt_range = t;
            any = true;
        }
        if let Some(d) = v.get("dimensions").and_then(|d| d.as_array()) {
            let n: Vec<f32> = d
                .iter()
                .filter_map(|x| x.as_f64())
                .map(|f| f as f32)
                .collect();
            if n.len() == 3 {
                out.size_mm = [n[0], n[1], n[2]];
                any = true;
            }
        }
        any.then_some(out)
    }
}

/// Ein Gerätetyp in einem bestimmten DMX-Modus.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureType {
    /// Eindeutiger Schlüssel, z. B. `generic/rgb-par-3ch`.
    pub key: String,
    pub manufacturer: String,
    pub model: String,
    pub mode: String,
    pub channels: Vec<ChannelDef>,
    /// Abstrahlwinkel, Pan/Tilt-Bereich, Maße – falls bekannt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<Physics>,
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
