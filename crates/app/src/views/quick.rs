//! Quick actions on selected fixtures (beams off/on, center) – used in
//! SETUP (fixtures) and in the programmer.

use std::collections::HashMap;

use dmxcraft_core::{ChannelKind, ChannelRef, Engine, FixtureId};

use crate::state::ProgrammerMode;

/// Value layer that faders and quick actions act on: Edit/Blind write into
/// the programmer (which mirrors the selected scene in Edit), Live into the
/// live values, which override everything and are not saved.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Layer {
    Programmer,
    Live,
}

impl Layer {
    pub fn of(mode: ProgrammerMode) -> Self {
        match mode {
            ProgrammerMode::Live => Layer::Live,
            _ => Layer::Programmer,
        }
    }

    pub fn map(self, e: &Engine) -> &HashMap<ChannelRef, u8> {
        match self {
            Layer::Programmer => e.programmer(),
            Layer::Live => e.live(),
        }
    }

    pub fn get(self, e: &Engine, ch: ChannelRef) -> Option<u8> {
        self.map(e).get(&ch).copied()
    }

    pub fn set(self, e: &mut Engine, ch: ChannelRef, v: u8) {
        match self {
            Layer::Programmer => e.set_programmer(ch, v),
            Layer::Live => e.set_live(ch, v),
        }
    }

    pub fn clear(self, e: &mut Engine, ch: ChannelRef) {
        match self {
            Layer::Programmer => e.clear_programmer_channel(ch),
            Layer::Live => e.clear_live_channel(ch),
        }
    }

    pub fn clear_all(self, e: &mut Engine) {
        match self {
            Layer::Programmer => e.clear_programmer(),
            Layer::Live => e.clear_live(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quick {
    Off,
    On,
    Center,
}

/// Sets programmer values for a quick test of the selection.
pub fn quick_action(e: &mut Engine, layer: Layer, ids: &[FixtureId], q: Quick) {
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
        // pure UV fixtures (e.g. UV bars) have UV as their only light
        let uv_only = !has_white
            && ![ChannelKind::Red, ChannelKind::Green, ChannelKind::Blue]
                .iter()
                .any(|k| ty.has_kind(*k));
        for (i, def) in ty.channels.iter().enumerate() {
            let ch = ChannelRef {
                fixture: *id,
                channel: i as u16,
            };
            let value = match (q, def.kind) {
                (Quick::Off, ChannelKind::Dimmer) => Some(0),
                (Quick::Off, k) if k.is_color_intensity() => Some(0),
                (Quick::Off, ChannelKind::Laser | ChannelKind::Fog) => Some(0),
                (Quick::On, ChannelKind::Uv) if uv_only => Some(255),
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
                layer.set(e, ch, v);
            }
        }
    }
}
