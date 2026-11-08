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
    /// Laser-Ausgabe/Muster.
    Laser,
    /// Nebel-/Hazer-Ausstoß (Pumpe).
    Fog,
    /// Lüfter (Nebel, Hazer).
    Fan,
    Zoom,
    Focus,
    /// Programme, Makros, Auto-/Sound-Modi, Effekte.
    Program,
    /// Betriebsart, Reset, Einstellungen.
    Control,
    /// Kanal ohne Funktion.
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
            ChannelKind::Laser => "Laser",
            ChannelKind::Fog => "Nebel",
            ChannelKind::Fan => "Lüfter",
            ChannelKind::Zoom => "Zoom",
            ChannelKind::Focus => "Fokus",
            ChannelKind::Program => "Programm",
            ChannelKind::Control => "Steuerung",
            ChannelKind::NoFunction => "Keine Funktion",
        }
    }

    /// Kanalart anhand des Kanalnamens verfeinern. Gerätedateien liefern oft
    /// nur „Dimmer“, „Speed“ oder gar nichts (Generisch) – der Name verrät
    /// mehr: „UV1 Dimmer“ ist UV, „Pump“ ist Nebel, „Flash rate“ ist Strobe,
    /// „Laser (10CH)“ ist Laser, „Auto Programs“ ein Programm.
    ///
    /// Nur Dimmer, Speed und Generisch werden verfeinert; Feinkanäle bleiben
    /// unverändert. Mehrfaches Anwenden ändert nichts mehr.
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
    /// Wert, wenn keine Szene und kein Programmer den Kanal setzt.
    #[serde(default)]
    pub default: u8,
    /// Wertebereiche mit Bedeutung (aus OFL/QLC+ bzw. dem Profile Builder).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<ChannelRange>,
}

/// Wertebereich eines Kanals, z. B. 0–7 „Shutter offen“, 8–250 Strobe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelRange {
    pub from: u8,
    pub to: u8,
    /// Bezeichnung aus der Gerätedatei (leer: aus `effect` abgeleitet).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub label: String,
    #[serde(default)]
    pub effect: RangeEffect,
}

/// Was ein Bereich bewirkt – soweit der Visualisierer es darstellen kann.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RangeEffect {
    #[default]
    Other,
    /// Shutter offen (Licht an, kein Strobe).
    Open,
    /// Shutter zu (kein Licht).
    Closed,
    /// Blitzen; Frequenz von `from_hz` (Bereichsanfang) bis `to_hz` (Ende).
    Strobe { from_hz: f32, to_hz: f32 },
    /// Farbe (Farbrad-Feld oder Farbvorgabe), sRGB.
    Color { rgb: [u8; 3] },
    /// Gobo-Feld (1-basiert).
    Gobo { slot: u8 },
    /// Offenes Feld eines Rads (weiß bzw. ohne Gobo).
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

    /// Lage des Werts im Bereich, 0–1.
    pub fn fraction(&self, v: u8) -> f32 {
        if self.to <= self.from {
            0.0
        } else {
            f32::from(v.saturating_sub(self.from)) / f32::from(self.to - self.from)
        }
    }
}

impl RangeEffect {
    /// Strobe-Frequenz bzw. Shutter-Zustand für `v` in `r`:
    /// `Some(None)` offen, `Some(Some(0.0))` zu, `Some(Some(hz))` blitzt.
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

    /// Bereich, in dem `v` liegt.
    pub fn range_at(&self, v: u8) -> Option<&ChannelRange> {
        self.ranges.iter().find(|r| (r.from..=r.to).contains(&v))
    }

    /// Shutter/Strobe für den Wert `v`: `None` offen, `Some(0.0)` zu,
    /// `Some(hz)` blitzt. Ohne Bereiche: unter 10 offen, darüber 1–20 Hz.
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

    /// Farbgerät ohne eigenen Dimmer-Kanal: bekommt einen virtuellen Dimmer
    /// ([`crate::ChannelRef::VIRTUAL_DIMMER`]), der die Farbkanäle skaliert.
    pub fn needs_virtual_dimmer(&self) -> bool {
        !self.channels.iter().any(|c| c.kind.is_htp())
            && self.channels.iter().any(|c| c.kind.is_color_intensity())
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
            (K::Generic, "(unbenutzt)", K::NoFunction),
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
            assert_eq!(want.refine(name), want, "{name} stabil");
        }
    }
}
