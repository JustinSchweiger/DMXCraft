//! What touch elements do: values, colors, positions, picked targets.

#[allow(unused_imports)]
use super::*;

/// Current value 0–1 of a fader/knob (readable state, otherwise the last
/// set value – shared by touch mode and remote).
pub(crate) fn item_value(e: &Engine, st: &AppState, item: &TouchItem) -> f32 {
    let live = |c: &ChannelRef| {
        e.live()
            .get(c)
            .map_or(if c.is_virtual() { 1.0 } else { 0.0 }, |v| {
                f32::from(*v) / 255.0
            })
    };
    match item.control {
        Some(Control::Master) => e.master(),
        Some(Control::AudioMaster) => st.audio.master,
        Some(Control::SoundVolume { deck }) => crate::mapping::sound_volume(e, deck),
        Some(Control::Channels) => item.channels.first().map_or(0.0, live),
        Some(Control::Channel { channel }) => live(&channel),
        _ => st.touch_values.get(&item.id).copied().unwrap_or(0.0),
    }
}

/// Channels of one kind of a fixture.
pub(crate) fn channels_of(show: &Show, f: FixtureId, kind: ChannelKind) -> Vec<ChannelRef> {
    show.fixture(f)
        .and_then(|fx| show.type_of(fx))
        .map(|ty| {
            ty.channels
                .iter()
                .enumerate()
                .filter(|(_, c)| c.kind == kind)
                .map(|(i, _)| ChannelRef {
                    fixture: f,
                    channel: i as u16,
                })
                .collect()
        })
        .unwrap_or_default()
}

/// Set a color as live values (red/green/blue of all cells).
pub(crate) fn set_color(e: &mut Engine, fixtures: &[FixtureId], rgb: [u8; 3]) {
    for f in fixtures {
        for (kind, v) in [
            (ChannelKind::Red, rgb[0]),
            (ChannelKind::Green, rgb[1]),
            (ChannelKind::Blue, rgb[2]),
        ] {
            for ch in channels_of(e.show(), *f, kind) {
                e.set_live(ch, v);
            }
        }
    }
}

pub(crate) fn current_color(e: &Engine, f: FixtureId) -> Option<[u8; 3]> {
    let get = |k| {
        channels_of(e.show(), f, k)
            .first()
            .and_then(|c| e.live().get(c).copied())
    };
    Some([
        get(ChannelKind::Red)?,
        get(ChannelKind::Green)?,
        get(ChannelKind::Blue)?,
    ])
}

/// Set pan/tilt as live values (0–1, fine channels 0).
pub(crate) fn set_position(e: &mut Engine, fixtures: &[FixtureId], x: f32, y: f32) {
    for f in fixtures {
        for (kind, v) in [
            (ChannelKind::Pan, (x * 255.0).round() as u8),
            (ChannelKind::Tilt, (y * 255.0).round() as u8),
            (ChannelKind::PanFine, 0),
            (ChannelKind::TiltFine, 0),
        ] {
            for ch in channels_of(e.show(), *f, kind) {
                e.set_live(ch, v);
            }
        }
    }
}

pub(crate) fn current_position(e: &Engine, f: FixtureId) -> Option<(f32, f32)> {
    let get = |k| {
        channels_of(e.show(), f, k)
            .first()
            .and_then(|c| e.live().get(c).copied())
    };
    let x = get(ChannelKind::Pan)?;
    let y = get(ChannelKind::Tilt).unwrap_or(128);
    Some((f32::from(x) / 255.0, f32::from(y) / 255.0))
}

/// Take over the queued element (Alt + click): give it to the selected
/// element as target (editing touch) or create it on the current page.
pub fn apply_pick(e: &mut Engine, st: &mut AppState, pick: crate::mapping::TouchPick) {
    use crate::state::Mode;
    let show = e.show();
    let label = match (&pick.name, pick.control) {
        (Some(n), _) => n.1.clone(),
        (None, Some(_)) if !pick.channels.is_empty() => {
            crate::mapping::channel_name(show, pick.channels[0])
        }
        (None, Some(c)) => crate::mapping::describe(show, &c).1,
        (None, None) => String::new(),
    };
    // Programmer fader: fixed to the channels currently controlled
    let (control, channels) = if pick.channels.is_empty() {
        (pick.control, Vec::new())
    } else {
        (Some(Control::Channels), pick.channels.clone())
    };
    let touch = &mut e.show_mut().touch;
    // the selected element in edit mode gets the target
    if st.mode == Mode::Touch && st.touch_edit {
        if let (Some(page), Some(sel)) = (st.touch_page, st.touch_sel) {
            if let Some(item) = touch
                .page_mut(page)
                .and_then(|p| p.items.iter_mut().find(|i| i.id == sel))
            {
                if item.kind.uses_fixtures() {
                    item.fixtures = pick.fixtures.clone();
                } else if item.kind != TouchKind::Label {
                    item.control = control;
                    item.channels = channels;
                    if item.label.is_empty() {
                        item.label = short_name(&label);
                    }
                }
                st.info(t("Target assigned"));
                return;
            }
        }
    }
    let page = match st.touch_page.filter(|p| touch.page(*p).is_some()) {
        Some(p) => p,
        None => match touch.pages.first() {
            Some(p) => p.id,
            None => {
                let n = touch.pages.len() + 1;
                touch.add_page(crate::tr!("Page {n}", n = n))
            }
        },
    };
    st.touch_page = Some(page);
    let mut item = TouchItem::new(pick.kind);
    item.control = control;
    item.channels = channels;
    item.fixtures = pick.fixtures;
    if pick.kind.uses_fixtures() {
        item.label = kind_name(pick.kind).to_string();
    } else if matches!(control, Some(Control::Ui { .. })) {
        // interface controls have no name of their own in the show
        item.label = label.clone();
    } else {
        item.label = short_name(&label);
    }
    if let Some(Control::Scene {
        target: dmxcraft_core::mapping::SceneTarget::Scene { id },
    }) = item.control
    {
        if let Some(s) = e.show().scene(id) {
            item.color = s.color;
            item.flash = s.settings.flash;
            item.label = s.name.clone();
        }
    }
    let name = e
        .show()
        .touch
        .page(page)
        .map_or_else(String::new, |p| p.name.clone());
    if let Some(id) = e.show_mut().touch.add_item(page, item) {
        st.touch_sel = Some(id);
        st.info(crate::tr!("Added to touch page “{page}”", page = name));
    }
}
