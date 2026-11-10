//! Eingerichtete MIDI-Controller (Einstellungen → MIDI): Gerätetyp, Ausgang
//! für die LED-Rückmeldung und „alle LEDs löschen“ beim Verbinden.
//!
//! Für den Akai APC mini mk2 kennt DMXCraft die Farbtabelle aus Akais
//! veröffentlichtem Kommunikationsprotokoll (Velocity → RGB, gleiche Tabelle
//! wie bei den anderen APC-mk2-Geräten) und die Bedeutung der MIDI-Kanäle
//! (Helligkeit, Pulsieren, Blinken).

use serde::{Deserialize, Serialize};

/// Bekannte Controller.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceModel {
    #[default]
    Generic,
    ApcMiniMk2,
}

impl DeviceModel {
    pub const ALL: [DeviceModel; 2] = [DeviceModel::Generic, DeviceModel::ApcMiniMk2];

    pub fn label(self) -> &'static str {
        match self {
            DeviceModel::Generic => crate::i18n::t("Generic MIDI controller"),
            DeviceModel::ApcMiniMk2 => "Akai APC mini mk2",
        }
    }

    /// Typ am Gerätenamen erkennen.
    pub fn detect(name: &str) -> Self {
        let n = name.to_lowercase();
        if n.contains("apc") && n.contains("mini") && n.contains("mk2") {
            DeviceModel::ApcMiniMk2
        } else {
            DeviceModel::Generic
        }
    }

    /// Farbe je Anschlagstärke, falls das Gerät eine feste Palette hat.
    pub fn palette(self) -> Option<&'static [u32; 128]> {
        match self {
            DeviceModel::ApcMiniMk2 => Some(&APC_MK2_PALETTE),
            DeviceModel::Generic => None,
        }
    }

    /// Nachrichten, die alle LEDs ausschalten.
    pub fn clear_messages(self) -> Vec<[u8; 3]> {
        let notes: Vec<u8> = match self {
            // 8×8-Pads, Spur- und Szenentasten
            DeviceModel::ApcMiniMk2 => (0x00..=0x3f)
                .chain(0x64..=0x6b)
                .chain(0x70..=0x77)
                .collect(),
            DeviceModel::Generic => (0..=127).collect(),
        };
        notes.into_iter().map(|n| [0x90, n, 0]).collect()
    }

    /// Bedeutung eines Rückmelde-Kanals (1–16), falls bekannt.
    pub fn channel_meaning(self, c: u8) -> Option<String> {
        if self != DeviceModel::ApcMiniMk2 {
            return None;
        }
        const BRIGHT: [u8; 7] = [10, 25, 50, 65, 75, 90, 100];
        const RATE: [&str; 9] = [
            "1/16", "1/8", "1/4", "1/2", "1/24", "1/16", "1/8", "1/4", "1/2",
        ];
        Some(match c {
            1..=7 => crate::tr!("On, {p} % brightness", p = BRIGHT[usize::from(c - 1)]),
            8..=11 => crate::tr!("Pulsing {r}", r = RATE[usize::from(c - 8)]),
            12..=16 => crate::tr!("Blinking {r}", r = RATE[usize::from(c - 8)]),
            _ => return None,
        })
    }
}

/// Ein eingerichteter Controller.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MidiDevice {
    /// Name des Eingangs.
    pub input: String,
    /// Ausgang für die LEDs (leer = keiner).
    pub output: String,
    pub model: DeviceModel,
    /// Beim Verbinden (Programmstart, Einstecken) alle LEDs ausschalten.
    pub clear_on_connect: bool,
}

impl Default for MidiDevice {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
            model: DeviceModel::Generic,
            clear_on_connect: true,
        }
    }
}

impl MidiDevice {
    /// Neues Gerät zum Eingang; Ausgang und Typ werden am Namen erkannt.
    pub fn new(input: &str, outputs: &[String]) -> Self {
        Self {
            input: input.to_string(),
            output: matching_output(input, outputs).unwrap_or_default(),
            model: DeviceModel::detect(input),
            clear_on_connect: true,
        }
    }
}

/// Ausgang mit gleichem Namen wie der Eingang (Windows/macOS) oder gleichem
/// Gerätenamen vor dem Port (ALSA: „APC mini mk2:APC mini mk2 Control 24:0“).
pub fn matching_output(input: &str, outputs: &[String]) -> Option<String> {
    if let Some(o) = outputs.iter().find(|o| *o == input) {
        return Some(o.clone());
    }
    let stem = |s: &str| {
        s.split([':', '('])
            .next()
            .unwrap_or(s)
            .trim()
            .to_lowercase()
    };
    let key = stem(input);
    outputs.iter().find(|o| stem(o) == key).cloned()
}

/// Nächstliegende Palettenfarbe (für den Vorschlag „An“-Farbe = Szenenfarbe).
pub fn nearest(palette: &[u32; 128], rgb: [u8; 3]) -> u8 {
    let d = |c: u32| {
        let r = ((c >> 16) & 0xff) as i32 - i32::from(rgb[0]);
        let g = ((c >> 8) & 0xff) as i32 - i32::from(rgb[1]);
        let b = (c & 0xff) as i32 - i32::from(rgb[2]);
        r * r + g * g + b * b
    };
    // 1–127; 0 ist „aus“
    (1..128u8)
        .min_by_key(|&v| d(palette[usize::from(v)]))
        .unwrap_or(3)
}

/// Velocity → RGB laut „APC mk2 Communication Protocol“ von Akai.
pub const APC_MK2_PALETTE: [u32; 128] = [
    0x000000, 0x1E1E1E, 0x7F7F7F, 0xFFFFFF, 0xFF4C4C, 0xFF0000, 0x590000, 0x190000, //
    0xFFBD6C, 0xFF5400, 0x591D00, 0x271B00, 0xFFFF4C, 0xFFFF00, 0x595900, 0x191900, //
    0x88FF4C, 0x54FF00, 0x1D5900, 0x142B00, 0x4CFF4C, 0x00FF00, 0x005900, 0x001900, //
    0x4CFF5E, 0x00FF19, 0x00590D, 0x001902, 0x4CFF88, 0x00FF55, 0x00591D, 0x001F12, //
    0x4CFFB7, 0x00FF99, 0x005935, 0x001912, 0x4CC3FF, 0x00A9FF, 0x004152, 0x001019, //
    0x4C88FF, 0x0055FF, 0x001D59, 0x000819, 0x4C4CFF, 0x0000FF, 0x000059, 0x000019, //
    0x874CFF, 0x5400FF, 0x190064, 0x0F0030, 0xFF4CFF, 0xFF00FF, 0x590059, 0x190019, //
    0xFF4C87, 0xFF0054, 0x59001D, 0x220013, 0xFF1500, 0x993500, 0x795100, 0x436400, //
    0x033900, 0x005735, 0x00547F, 0x0000FF, 0x00454F, 0x2500CC, 0x7F7F7F, 0x202020, //
    0xFF0000, 0xBDFF2D, 0xAFED06, 0x64FF09, 0x108B00, 0x00FF87, 0x00A9FF, 0x002AFF, //
    0x3F00FF, 0x7A00FF, 0xB21A7D, 0x402100, 0xFF4A00, 0x88E106, 0x72FF15, 0x00FF00, //
    0x3BFF26, 0x59FF71, 0x38FFCC, 0x5B8AFF, 0x3151C6, 0x877FE9, 0xD31DFF, 0xFF005D, //
    0xFF7F00, 0xB9B000, 0x90FF00, 0x835D07, 0x392B00, 0x144C10, 0x0D5038, 0x15152A, //
    0x16205A, 0x693C1C, 0xA8000A, 0xDE513D, 0xD86A1C, 0xFFE126, 0x9EE12F, 0x67B50F, //
    0x1E1E30, 0xDCFF6B, 0x80FFBD, 0x9A99FF, 0x8E66FF, 0x404040, 0x757575, 0xE0FFFF, //
    0xA00000, 0x350000, 0x1AD000, 0x074200, 0xB9B000, 0x3F3100, 0xB35F00, 0x4B1502, //
];

/// Palettenfarbe als egui-Farbe.
pub fn color32(c: u32) -> eframe::egui::Color32 {
    eframe::egui::Color32::from_rgb((c >> 16) as u8, (c >> 8) as u8, c as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_and_match() {
        assert_eq!(DeviceModel::detect("APC mini mk2"), DeviceModel::ApcMiniMk2);
        assert_eq!(DeviceModel::detect("nanoKONTROL2"), DeviceModel::Generic);
        let outs = vec![
            "Foo".to_string(),
            "APC mini mk2:APC mini mk2 Control 24:0".into(),
        ];
        assert_eq!(
            matching_output("APC mini mk2:APC mini mk2 Control 24:0", &outs).as_deref(),
            Some("APC mini mk2:APC mini mk2 Control 24:0")
        );
        assert_eq!(
            matching_output("APC mini mk2 (In)", &["APC mini mk2".into()]).as_deref(),
            Some("APC mini mk2")
        );
        assert_eq!(matching_output("X", &outs), None);
    }

    #[test]
    fn palette_lookup() {
        assert_eq!(APC_MK2_PALETTE[5], 0xFF0000);
        assert_eq!(nearest(&APC_MK2_PALETTE, [255, 0, 0]), 5);
        assert_eq!(nearest(&APC_MK2_PALETTE, [0, 255, 0]), 21);
        let clear = DeviceModel::ApcMiniMk2.clear_messages();
        assert_eq!(clear.len(), 64 + 8 + 8);
    }

    #[test]
    fn channel_meanings() {
        let m = DeviceModel::ApcMiniMk2;
        assert!(m.channel_meaning(7).unwrap().contains("100"));
        assert!(m.channel_meaning(12).unwrap().contains("1/24"));
        assert!(DeviceModel::Generic.channel_meaning(3).is_none());
    }
}
