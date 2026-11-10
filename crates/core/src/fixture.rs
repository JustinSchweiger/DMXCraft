use serde::{Deserialize, Serialize};

/// Meaning of a channel. Determines the merge behavior (HTP/LTP) and how the
/// master affects the channel.
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
    /// Laser output/pattern.
    Laser,
    /// Fog/hazer output (pump).
    Fog,
    /// Fan (fog, hazer).
    Fan,
    Zoom,
    Focus,
    /// Programs, macros, auto/sound modes, effects.
    Program,
    /// Operating mode, reset, settings.
    Control,
    /// Channel without a function.
    NoFunction,
}

impl ChannelKind {
    pub const ALL: [ChannelKind; 24] = [
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
        ChannelKind::Laser,
        ChannelKind::Fog,
        ChannelKind::Fan,
        ChannelKind::Zoom,
        ChannelKind::Focus,
        ChannelKind::Program,
        ChannelKind::Control,
        ChannelKind::NoFunction,
    ];

    /// Intensity channels are merged HTP (highest value wins), all others LTP
    /// (most recently activated scene wins).
    pub fn is_htp(self) -> bool {
        matches!(self, ChannelKind::Dimmer)
    }

    /// Color channels that are scaled by the master for fixtures without their
    /// own dimmer channel (“virtual dimmer”).
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
            ChannelKind::Red => "Red",
            ChannelKind::Green => "Green",
            ChannelKind::Blue => "Blue",
            ChannelKind::White => "White",
            ChannelKind::Amber => "Amber",
            ChannelKind::Uv => "UV",
            ChannelKind::Pan => "Pan",
            ChannelKind::PanFine => "Pan fine",
            ChannelKind::Tilt => "Tilt",
            ChannelKind::TiltFine => "Tilt fine",
            ChannelKind::Strobe => "Strobe",
            ChannelKind::ColorWheel => "Color wheel",
            ChannelKind::Gobo => "Gobo",
            ChannelKind::Speed => "Speed",
            ChannelKind::Generic => "Generic",
            ChannelKind::Laser => "Laser",
            ChannelKind::Fog => "Fog",
            ChannelKind::Fan => "Fan",
            ChannelKind::Zoom => "Zoom",
            ChannelKind::Focus => "Focus",
            ChannelKind::Program => "Program",
            ChannelKind::Control => "Controls",
            ChannelKind::NoFunction => "No function",
        }
    }

    /// Refine the channel kind based on the channel name. Fixture files often
    /// only give “Dimmer”, “Speed” or nothing at all (generic) – the name tells
    /// more: “UV1 Dimmer” is UV, “Pump” is fog, “Flash rate” is strobe,
    /// “Laser (10CH)” is laser, “Auto Programs” a program.
    ///
    /// Only dimmer, speed and generic are refined; fine channels stay unchanged.
    /// Applying it several times changes nothing further.
    pub fn refine(self, name: &str) -> ChannelKind {
        use ChannelKind as K;
        if !matches!(self, K::Dimmer | K::Speed | K::Generic) {
            return self;
        }
        let n = name.to_lowercase();
        let words: Vec<&str> = n
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();
        let word = |w: &str| words.contains(&w);
        let any = |list: &[&str]| list.iter().any(|x| n.contains(x));
        if word("fine")
            || word("lsb")
            || word("fein")
            || n.contains("16bit")
            || n.contains("16 bit")
        {
            return self;
        }
        let generic = self == K::Generic;
        if generic
            && any(&[
                "no function",
                "nofunction",
                "not used",
                "unused",
                "reserved",
                "unbenutzt",
                "keine funktion",
                "ohne funktion",
            ])
        {
            return K::NoFunction;
        }
        if self != K::Speed && n.contains("laser") {
            return K::Laser;
        }
        if any(&["fog", "smoke", "haze", "nebel", "dunst"]) || word("pump") {
            return K::Fog;
        }
        if word("fan") || any(&["lüfter", "luefter", "ventilator", "gebläse"]) {
            return K::Fan;
        }
        if self != K::Dimmer && any(&["strob", "flash", "shutter", "blitz"]) {
            return K::Strobe;
        }
        let uv = words.iter().any(|w| {
            w.strip_prefix("uv")
                .is_some_and(|rest| rest.chars().all(|c| c.is_ascii_digit()))
        }) || any(&["ultraviolet", "ultra violet", "blacklight", "schwarzlicht"]);
        if self != K::Speed && uv {
            return K::Uv;
        }
        if !generic {
            return self;
        }
        if n.contains("zoom") {
            return K::Zoom;
        }
        if n.contains("focus") || n.contains("fokus") {
            return K::Focus;
        }
        if any(&[
            "macro",
            "makro",
            "program",
            "auto",
            "color change",
            "colour change",
            "pattern",
            "effect",
            "effekt",
            "matrix",
            "chase",
            "sound",
            "preset",
            "show",
            "rainbow",
            "smd",
            "ring",
            "animation",
            "fade",
        ]) || word("fx")
        {
            return K::Program;
        }
        if any(&[
            "mode",
            "modus",
            "reset",
            "control",
            "function",
            "operating",
            "setting",
            "special",
            "curve",
            "kurve",
            "steuer",
            "betrieb",
        ]) {
            return K::Control;
        }
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelDef {
    pub name: String,
    pub kind: ChannelKind,
    /// Value when no scene and no programmer sets the channel.
    #[serde(default)]
    pub default: u8,
    /// Value ranges with meaning (from OFL/QLC+ or the Profile Builder).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<ChannelRange>,
}

/// Value range of a channel, e.g. 0–7 “shutter open”, 8–250 strobe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelRange {
    pub from: u8,
    pub to: u8,
    /// Label from the fixture file (empty: derived from `effect`).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    pub effect: RangeEffect,
}

/// What a range does – as far as the visualizer can show it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RangeEffect {
    #[default]
    Other,
    /// Shutter open (light on, no strobe).
    Open,
    /// Shutter closed (no light).
    Closed,
    /// Flashing; rate from `from_hz` (start of the range) to `to_hz` (end).
    Strobe { from_hz: f32, to_hz: f32 },
    /// Color (color wheel slot or color preset), sRGB.
    Color { rgb: [u8; 3] },
    /// Gobo slot (1-based).
    Gobo { slot: u8 },
    /// Open slot of a wheel (white or without gobo).
    WheelOpen,
}

impl ChannelRange {
    pub fn new(from: u8, to: u8, effect: RangeEffect) -> Self {
        Self {
            from: from.min(to),
            to: to.max(from),
            label: String::new(),
            effect,
        }
    }

    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = l.into();
        self
    }

    /// Position of the value within the range, 0–1.
    pub fn fraction(&self, v: u8) -> f32 {
        if self.to <= self.from {
            0.0
        } else {
            f32::from(v.saturating_sub(self.from)) / f32::from(self.to - self.from)
        }
    }
}

impl RangeEffect {
    /// Strobe rate or shutter state for `v` in `r`:
    /// `Some(None)` open, `Some(Some(0.0))` closed, `Some(Some(hz))` flashing.
    pub fn shutter(&self, r: &ChannelRange, v: u8) -> Option<Option<f32>> {
        match *self {
            RangeEffect::Open | RangeEffect::WheelOpen => Some(None),
            RangeEffect::Closed => Some(Some(0.0)),
            RangeEffect::Strobe { from_hz, to_hz } => {
                Some(Some((from_hz + (to_hz - from_hz) * r.fraction(v)).max(0.1)))
            }
            _ => None,
        }
    }
}

impl ChannelDef {
    pub fn new(name: impl Into<String>, kind: ChannelKind) -> Self {
        Self {
            name: name.into(),
            kind,
            default: 0,
            ranges: Vec::new(),
        }
    }

    /// Range `v` lies in.
    pub fn range_at(&self, v: u8) -> Option<&ChannelRange> {
        self.ranges.iter().find(|r| (r.from..=r.to).contains(&v))
    }

    /// Shutter/strobe for the value `v`: `None` open, `Some(0.0)` closed,
    /// `Some(hz)` flashing. Without ranges: below 10 open, above that 1–20 Hz.
    pub fn strobe(&self, v: u8) -> Option<f32> {
        if !self.ranges.is_empty() {
            return self
                .range_at(v)
                .and_then(|r| r.effect.shutter(r, v))
                .unwrap_or(None);
        }
        (v >= 10).then(|| 1.0 + f32::from(v - 10) / 245.0 * 19.0)
    }

    pub fn with_default(mut self, default: u8) -> Self {
        self.default = default;
        self
    }
}

/// Physical data of a fixture (for the 3D visualizer).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Physics {
    /// Beam angle in degrees (min, max); equal for fixed optics.
    pub beam_angle: [f32; 2],
    /// Pan range in degrees (0 = no pan).
    pub pan_range: f32,
    /// Tilt range in degrees (0 = no tilt).
    pub tilt_range: f32,
    /// Width, height, depth in mm (0 = unknown).
    pub size_mm: [f32; 3],
    /// The zoom channel goes from wide (0) to narrow (255).
    pub zoom_reversed: bool,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            beam_angle: [25.0, 25.0],
            pan_range: 540.0,
            tilt_range: 270.0,
            size_mm: [0.0; 3],
            zoom_reversed: false,
        }
    }
}

impl Physics {
    /// From the `physical` block of an OFL fixture file (missing data = default).
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

/// A fixture type in a specific DMX mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureType {
    /// Unique key, e.g. `generic/rgb-par-3ch`.
    pub key: String,
    pub manufacturer: String,
    pub model: String,
    pub mode: String,
    pub channels: Vec<ChannelDef>,
    /// Beam angle, pan/tilt range, dimensions – if known.
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

    /// Color fixture without its own dimmer channel: gets a virtual dimmer
    /// ([`crate::ChannelRef::VIRTUAL_DIMMER`]) that scales the color channels.
    pub fn needs_virtual_dimmer(&self) -> bool {
        !self.channels.iter().any(|c| c.kind.is_htp())
            && self.channels.iter().any(|c| c.kind.is_color_intensity())
    }

    pub fn display_name(&self) -> String {
        format!(
            "{} {} ({}, {} ch.)",
            self.manufacturer,
            self.model,
            self.mode,
            self.channels.len()
        )
    }
}

#[cfg(test)]
mod refine_tests {
    use super::ChannelKind as K;

    #[test]
    fn strobe_ranges() {
        use crate::{ChannelDef, ChannelKind, ChannelRange, RangeEffect};
        let mut c = ChannelDef::new("Shutter", ChannelKind::Strobe);
        assert_eq!(c.strobe(0), None);
        assert!(c.strobe(200).is_some());
        c.ranges = vec![
            ChannelRange::new(0, 7, RangeEffect::Closed),
            ChannelRange::new(8, 15, RangeEffect::Open),
            ChannelRange::new(
                16,
                131,
                RangeEffect::Strobe {
                    from_hz: 1.0,
                    to_hz: 20.0,
                },
            ),
            ChannelRange::new(132, 255, RangeEffect::Open),
        ];
        assert_eq!(c.strobe(3), Some(0.0));
        assert_eq!(c.strobe(10), None);
        assert_eq!(c.strobe(16), Some(1.0));
        assert_eq!(c.strobe(131), Some(20.0));
        assert_eq!(c.strobe(255), None);
    }

    #[test]
    fn refine_by_name() {
        let cases = [
            (K::Dimmer, "UV1 Dimmer", K::Uv),
            (K::Dimmer, "UV2 Dimmer", K::Uv),
            (K::Generic, "Laser (10CH)", K::Laser),
            (K::Strobe, "Laser Strobe", K::Strobe),
            (K::Speed, "Laser Rotation", K::Speed),
            (K::Dimmer, "Pump", K::Fog),
            (K::Generic, "Fog", K::Fog),
            (K::Speed, "Fan", K::Fan),
            (K::Speed, "Flash rate", K::Strobe),
            (K::Generic, "Focus", K::Focus),
            (K::Generic, "Zoom", K::Zoom),
            (K::Generic, "Auto Programs", K::Program),
            (K::Generic, "Auto mode / Sound control", K::Program),
            (K::Generic, "Color Change", K::Program),
            (K::Generic, "Ring effect", K::Program),
            (K::Generic, "Matrix", K::Program),
            (K::Generic, "White SMDs", K::Program),
            (K::Generic, "Mode", K::Control),
            (K::Generic, "Operating mode selection", K::Control),
            (K::Generic, "Reset", K::Control),
            (K::Generic, "No function", K::NoFunction),
            (K::Generic, "(unused)", K::NoFunction),
            (K::Dimmer, "Master Dimmer", K::Dimmer),
            (K::Dimmer, "Dimmer", K::Dimmer),
            (K::Speed, "Speed (Matrix)", K::Speed),
            (K::Speed, "Pan/Tilt Speed", K::Speed),
            (K::Generic, "Dimmer fine", K::Generic),
            (K::Red, "UV", K::Red),
            (K::Generic, "Blubb", K::Generic),
        ];
        for (k, name, want) in cases {
            assert_eq!(k.refine(name), want, "{name}");
            assert_eq!(want.refine(name), want, "{name} stable");
        }
    }
}
