//! Schnellaktionen auf ausgewählte Geräte (Beams aus/an, zentrieren) –
//! genutzt unter EINRICHTEN (Geräte) und im Programmer.

use dmxcraft_core::{ChannelKind, ChannelRef, Engine, FixtureId};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Quick {
    Off,
    On,
    Center,
}

/// Setzt Programmer-Werte für einen schnellen Test der Auswahl.
pub fn quick_action(e: &mut Engine, ids: &[FixtureId], q: Quick) {
    for id in ids {
        let Some(ty) = e
            .show()
            .fixture(*id)
            .and_then(|f| e.show().type_of(f))
            .cloned()
        else {
            continue;
        };
        let has_white = ty.has_kind(ChannelKind::White);
        for (i, def) in ty.channels.iter().enumerate() {
            let ch = ChannelRef {
                fixture: *id,
                channel: i as u16,
            };
            let value = match (q, def.kind) {
                (Quick::Off, ChannelKind::Dimmer) => Some(0),
                (Quick::Off, k) if k.is_color_intensity() => Some(0),
                (Quick::On, ChannelKind::Dimmer) => Some(255),
                (Quick::On, ChannelKind::White) => Some(255),
                (Quick::On, ChannelKind::Red | ChannelKind::Green | ChannelKind::Blue)
                    if !has_white =>
                {
                    Some(255)
                }
                (Quick::Center, ChannelKind::Pan | ChannelKind::Tilt) => Some(128),
                (Quick::Center, ChannelKind::PanFine | ChannelKind::TiltFine) => Some(0),
                _ => None,
            };
            if let Some(v) = value {
                e.set_programmer(ch, v);
            }
        }
    }
}
