//! Built-in generic fixture types. Manufacturer-specific fixtures come from
//! the free Open Fixture Library and QLC+ – the Daslight libraries are
//! deliberately not used.

use crate::fixture::{ChannelDef, ChannelKind as K, FixtureType};

fn ft(key: &str, model: &str, mode: &str, mut channels: Vec<ChannelDef>) -> FixtureType {
    use crate::fixture::{ChannelRange, RangeEffect};
    // generic strobe: open at the bottom, slow → fast above that
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
        manufacturer: "Generic".into(),
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
            "1 channel",
            vec![ChannelDef::new("Dimmer", K::Dimmer)],
        ),
        ft(
            "generic/rgb-3ch",
            "RGB-PAR",
            "3 channel",
            vec![
                ChannelDef::new("Red", K::Red),
                ChannelDef::new("Green", K::Green),
                ChannelDef::new("Blue", K::Blue),
            ],
        ),
        ft(
            "generic/drgb-4ch",
            "RGB-PAR",
            "4 channel (D+RGB)",
            vec![
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Red", K::Red),
                ChannelDef::new("Green", K::Green),
                ChannelDef::new("Blue", K::Blue),
            ],
        ),
        ft(
            "generic/rgbw-par-6ch",
            "RGBW-PAR",
            "6 channel",
            vec![
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Red", K::Red),
                ChannelDef::new("Green", K::Green),
                ChannelDef::new("Blue", K::Blue),
                ChannelDef::new("White", K::White),
                ChannelDef::new("Strobe", K::Strobe),
            ],
        ),
        ft(
            "generic/moving-head-spot-11ch",
            "Moving Head Spot",
            "11 channel",
            vec![
                ChannelDef::new("Pan", K::Pan).with_default(128),
                ChannelDef::new("Pan fine", K::PanFine),
                ChannelDef::new("Tilt", K::Tilt).with_default(128),
                ChannelDef::new("Tilt fine", K::TiltFine),
                ChannelDef::new("Speed", K::Speed),
                ChannelDef::new("Dimmer", K::Dimmer),
                ChannelDef::new("Strobe", K::Strobe),
                ChannelDef::new("Color wheel", K::ColorWheel),
                ChannelDef::new("Gobo", K::Gobo),
                ChannelDef::new("Gobo-Rotation", K::Generic),
                ChannelDef::new("Reset", K::Control),
            ],
        ),
    ]
}
