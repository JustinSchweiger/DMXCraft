//! Symbolbibliothek für die Touch-Oberfläche: eine Auswahl aus der
//! Phosphor-Schrift (MIT, siehe assets/fonts/Phosphor-LICENSE.txt), die schon
//! für die Knöpfe der Oberfläche eingebunden ist.

use eframe::egui::{self, Align2, Color32, FontFamily, FontId, Pos2};

use super::icons::ICON_FAMILY;

/// (Name, Zeichen) – der Name wird in der Show gespeichert.
pub const LIBRARY: [(&str, char); 105] = [
    ("lightbulb", '\u{E2DC}'),
    ("lightbulb-filament", '\u{E63C}'),
    ("sun", '\u{E472}'),
    ("sun-dim", '\u{E474}'),
    ("moon", '\u{E330}'),
    ("moon-stars", '\u{E58E}'),
    ("star", '\u{E46A}'),
    ("sparkle", '\u{E6A2}'),
    ("heart", '\u{E2A8}'),
    ("fire", '\u{E242}'),
    ("flame", '\u{E624}'),
    ("snowflake", '\u{E5AA}'),
    ("lightning", '\u{E2DE}'),
    ("drop", '\u{E210}'),
    ("cloud", '\u{E1AA}'),
    ("wind", '\u{E5D2}'),
    ("music-note", '\u{E33C}'),
    ("music-notes", '\u{E340}'),
    ("microphone", '\u{E326}'),
    ("microphone-stage", '\u{E75C}'),
    ("speaker-high", '\u{E44A}'),
    ("headphones", '\u{E2A6}'),
    ("play", '\u{E3D0}'),
    ("pause", '\u{E39E}'),
    ("stop", '\u{E46C}'),
    ("record", '\u{E3EE}'),
    ("skip-forward", '\u{E5A6}'),
    ("skip-back", '\u{E5A4}'),
    ("repeat", '\u{E3F6}'),
    ("shuffle", '\u{E422}'),
    ("flag", '\u{E244}'),
    ("warning", '\u{E4E0}'),
    ("eye", '\u{E220}'),
    ("eye-slash", '\u{E224}'),
    ("palette", '\u{E6C8}'),
    ("paint-brush", '\u{E6F0}'),
    ("spiral", '\u{E9FA}'),
    ("circle", '\u{E18A}'),
    ("square", '\u{E45E}'),
    ("triangle", '\u{E4B0}'),
    ("diamond", '\u{E1EC}'),
    ("hexagon", '\u{E2AE}'),
    ("arrow-up", '\u{E08E}'),
    ("arrow-down", '\u{E03E}'),
    ("arrow-left", '\u{E058}'),
    ("arrow-right", '\u{E06C}'),
    ("arrows-clockwise", '\u{E094}'),
    ("confetti", '\u{E81A}'),
    ("champagne", '\u{EACA}'),
    ("beer-stein", '\u{EB62}'),
    ("wine", '\u{E6B2}'),
    ("cake", '\u{E780}'),
    ("gift", '\u{E276}'),
    ("balloon", '\u{E76C}'),
    ("crown", '\u{E614}'),
    ("trophy", '\u{E67E}'),
    ("medal", '\u{E320}'),
    ("smiley", '\u{E436}'),
    ("hand-waving", '\u{E580}'),
    ("hands-clapping", '\u{E6A0}'),
    ("guitar", '\u{EA8A}'),
    ("piano-keys", '\u{E9C8}'),
    ("vinyl-record", '\u{ECAC}'),
    ("disc", '\u{E564}'),
    ("television", '\u{E754}'),
    ("projector-screen", '\u{E658}'),
    ("film-strip", '\u{E792}'),
    ("camera", '\u{E10E}'),
    ("siren", '\u{E9B8}'),
    ("bell", '\u{E0CE}'),
    ("megaphone", '\u{E324}'),
    ("clock", '\u{E19A}'),
    ("timer", '\u{E492}'),
    ("hourglass", '\u{E2B2}'),
    ("power", '\u{E3DA}'),
    ("plus", '\u{E3D4}'),
    ("minus", '\u{E32A}'),
    ("x", '\u{E4F6}'),
    ("check", '\u{E182}'),
    ("number-one", '\u{E368}'),
    ("number-two", '\u{E380}'),
    ("number-three", '\u{E37A}'),
    ("number-four", '\u{E35C}'),
    ("number-five", '\u{E356}'),
    ("house", '\u{E2C2}'),
    ("door", '\u{E61C}'),
    ("door-open", '\u{E7E6}'),
    ("person", '\u{E3A8}'),
    ("users", '\u{E4D6}'),
    ("fan", '\u{E9F2}'),
    ("waves", '\u{E6DE}'),
    ("cloud-fog", '\u{E53C}'),
    ("rainbow", '\u{E598}'),
    ("atom", '\u{E5E4}'),
    ("planet", '\u{E652}'),
    ("rocket", '\u{E3FC}'),
    ("ghost", '\u{E62A}'),
    ("skull", '\u{E916}'),
    ("mask-happy", '\u{E9F4}'),
    ("lighthouse", '\u{E9F6}'),
    ("traffic-signal", '\u{E9AA}'),
    ("heartbeat", '\u{E2AC}'),
    ("sliders", '\u{E432}'),
    ("magic-wand", '\u{E6B6}'),
    ("eyedropper", '\u{E568}'),
];

/// Zeichen eines Symbols nach Namen.
pub fn glyph(name: &str) -> Option<char> {
    LIBRARY.iter().find(|(n, _)| *n == name).map(|(_, c)| *c)
}

/// Symbol zeichnen (mittig, Höhe `size`).
pub fn paint(p: &egui::Painter, center: Pos2, name: &str, size: f32, color: Color32) {
    if let Some(c) = glyph(name) {
        p.text(
            center,
            Align2::CENTER_CENTER,
            c.to_string(),
            FontId::new(size, FontFamily::Name(ICON_FAMILY.into())),
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_unique() {
        let mut names: Vec<&str> = LIBRARY.iter().map(|x| x.0).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), LIBRARY.len());
        assert!(glyph("lightbulb").is_some());
    }
}
