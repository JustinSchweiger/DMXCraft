//! Eingebaute generische Gerätetypen. Herstellerspezifische Geräte sollen
//! später aus der freien Open Fixture Library importiert werden – die
//! Daslight-Bibliotheken werden bewusst nicht verwendet.

use crate::fixture::{ChannelDef, ChannelKind as K, FixtureType};

fn ft(key: &str, model: &str, mode: &str, mut channels: Vec<ChannelDef>) -> FixtureType {
    use crate::fixture::{ChannelRange, RangeEffect};
    // generischer Strobe: unten offen, darüber langsam → schnell
    for c in channels.iter_mut().filter(|c| c.kind == K::Strobe) {
        c.ranges = vec![
            ChannelRange::new(0, 9, RangeEffect::Open),
            ChannelRange::new(
                10,
                255,
                RangeEffect::Strobe {
                    from_hz: 1.0,
                    to_hz: 20.0,
                },
            ),
        ];
    }
    FixtureType {
        physical: None,
        key: key.into(),
        manufacturer: "Generisch".into(),
        model: model.into(),
        mode: mode.into(),
        channels,
    }
}

pub fn builtin_fixture_types() -> Vec<FixtureType> {
    vec![
        ft(
            "generic/dimmer-1ch",
            "Dimmer",
            "1 Kanal",
            vec![ChannelDef::new("Dimmer", K::Dimmer)],
        ),
        ft(
            "generic/rgb-3ch",
            "RGB-PAR",
            "3 Kanal",
            vec![
                ChannelDef::new("Rot", K::Red),
                ChannelDef::new("Grün", K::Green),
                ChannelDef::new("Blau", K::Blue),
            ],
        ),
        ft(
            "generic/drgb-4ch",
            "RGB-PAR",
            "4 Kanal (D+RGB)",
            vec![
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Rot", K::Red),
                ChannelDef::new("Grün", K::Green),
                ChannelDef::new("Blau", K::Blue),
            ],
        ),
        ft(
            "generic/rgbw-par-6ch",
            "RGBW-PAR",
            "6 Kanal",
            vec![
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Rot", K::Red),
                ChannelDef::new("Grün", K::Green),
                ChannelDef::new("Blau", K::Blue),
                ChannelDef::new("Weiß", K::White),
                ChannelDef::new("Strobe", K::Strobe),
            ],
        ),
        ft(
            "generic/moving-head-spot-11ch",
            "Moving Head Spot",
            "11 Kanal",
            vec![
                ChannelDef::new("Pan", K::Pan).with_default(128),
                ChannelDef::new("Pan fein", K::PanFine),
                ChannelDef::new("Tilt", K::Tilt).with_default(128),
                ChannelDef::new("Tilt fein", K::TiltFine),
                ChannelDef::new("Speed", K::Speed),
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Strobe", K::Strobe),
                ChannelDef::new("Farbrad", K::ColorWheel),
                ChannelDef::new("Gobo", K::Gobo),
                ChannelDef::new("Gobo-Rotation", K::Generic),
                ChannelDef::new("Reset", K::Control),
            ],
        ),
    ]
}
