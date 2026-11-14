//! Interface controls that also work while their element is not shown
//! (another tab, closed panel): keys and MIDI on them are executed here
//! directly on the show and the app state instead of waiting for the
//! element. Their state is reported too, so LEDs and touch buttons stay
//! right.

use std::collections::HashMap;

use dmxcraft_core::mapping::ui_key;
use dmxcraft_core::{Engine, LoopMode};

use super::UiFire;
use crate::state::{AppState, ProgrammerMode};

/// Highest target height (m), as in the tracking view.
const MAX_HEIGHT: f32 = 3.0;

/// What an input on a control means.
#[derive(Clone, Copy)]
enum Input {
    /// Button: pressed.
    Press,
    /// Fader: new value 0–1.
    Value(f32),
}

impl Input {
    fn of(f: &UiFire, fader_value: f32) -> Option<Input> {
        match f.value {
            Some(v) => Some(Input::Value(v.clamp(0.0, 1.0))),
            // a key on a slider toggles between 0 and 1
            None if f.press => Some(Input::Value(if fader_value > 0.5 { 0.0 } else { 1.0 })),
            None => None,
        }
    }

    fn pressed(f: &UiFire) -> Option<Input> {
        (f.press || f.value.is_some_and(|v| v > 0.0)).then_some(Input::Press)
    }
}

/// Execute and remove the inputs of known controls; report their states.
pub(super) fn apply(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    e: &mut Engine,
    st: &mut AppState,
) {
    tracking(fires, states, e);
    sound(fires, states, e, st);
    programmer(fires, states, e, st);
    scene(fires, states, e, st);
}

/// Take the input of a button key; reports `on`.
fn button(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    key: &str,
    on: bool,
) -> bool {
    let k = ui_key(key);
    states.insert(k, on);
    fires.remove(&k).and_then(|f| Input::pressed(&f)).is_some()
}

/// Take the input of a slider key (current value `value`).
fn fader(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    key: &str,
    value: f32,
) -> Option<f32> {
    let k = ui_key(key);
    states.insert(k, value > 0.0);
    match fires.remove(&k).and_then(|f| Input::of(&f, value))? {
        Input::Value(v) => Some(v),
        Input::Press => None,
    }
}

fn tracking(fires: &mut HashMap<u64, UiFire>, states: &mut HashMap<u64, bool>, e: &mut Engine) {
    let (xr, zr) = e.show().tracking_area();
    let norm = |v: f32, r: [f32; 2]| ((v - r[0]) / (r[1] - r[0]).max(0.001)).clamp(0.0, 1.0);
    let trackers = e.show().trackers.clone();
    for tr in trackers {
        let id = tr.id;
        let key = |k: &str| format!("tracking/{id}/{k}");
        let cells = e.show().tracker_cells(id).min(32);
        let edit = |e: &mut Engine, f: &dyn Fn(&mut dmxcraft_core::tracking::Tracker)| {
            if let Some(t) = e.show_mut().tracker_mut(id) {
                f(t);
            }
        };
        if button(fires, states, &key("follow"), tr.active) {
            edit(e, &|t| t.active = !t.active);
        }
        if button(fires, states, &key("light"), tr.light) {
            edit(e, &|t| t.light = !t.light);
        }
        if button(fires, states, &key("own_zoom"), tr.own_zoom) {
            edit(e, &|t| t.own_zoom = !t.own_zoom);
        }
        if let Some(v) = fader(fires, states, &key("x"), norm(tr.target[0], xr)) {
            edit(e, &|t| t.target[0] = xr[0] + v * (xr[1] - xr[0]));
        }
        if let Some(v) = fader(fires, states, &key("z"), norm(tr.target[2], zr)) {
            edit(e, &|t| t.target[2] = zr[0] + v * (zr[1] - zr[0]));
        }
        let height = (tr.target[1] / MAX_HEIGHT).clamp(0.0, 1.0);
        if let Some(v) = fader(fires, states, &key("y"), height) {
            edit(e, &|t| t.target[1] = v * MAX_HEIGHT);
        }
        if let Some(v) = fader(fires, states, &key("dimmer"), tr.dimmer) {
            edit(e, &|t| t.dimmer = v);
        }
        if let Some(v) = fader(fires, states, &key("zoom"), tr.zoom) {
            edit(e, &|t| t.zoom = v);
        }
        for n in 0..cells {
            let on = tr.cells & (1 << n) != 0;
            if button(fires, states, &key(&format!("cell{}", n + 1)), on) {
                if let Some(cells) = toggle_cell(tr.cells, cells, n) {
                    edit(e, &|t| t.cells = cells);
                }
            }
        }
    }
}

/// Switch color cell `n` of `count` used cells; `None` when that would
/// switch all off.
pub(crate) fn toggle_cell(cells: u32, count: usize, n: usize) -> Option<u32> {
    let used = if count >= 32 {
        u32::MAX
    } else {
        (1u32 << count) - 1
    };
    let mask = (cells & used) ^ (1 << n);
    (mask != 0).then_some(if mask == used { u32::MAX } else { mask })
}

fn sound(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    e: &mut Engine,
    st: &mut AppState,
) {
    use crate::sound::DeckState;
    let board = e.show().sound.clone();
    let sp = &mut st.sound;
    for d in 0..board.decks.len() {
        let key = |k: &str| format!("sound/deck/{d}/{k}");
        let state = sp.state(d);
        if button(fires, states, &key("start"), false) {
            sp.rewind(d);
        }
        let playing = matches!(state, DeckState::Playing | DeckState::Fading);
        if button(fires, states, &key("play"), playing) {
            sp.play_pause(&board, d);
        }
        if button(fires, states, &key("stop"), false) {
            sp.stop(d);
        }
        if button(fires, states, &key("cue"), false) {
            sp.cue(d);
        }
        if button(fires, states, &key("fade"), state == DeckState::Fading) {
            sp.fade_out(&board, d);
        }
    }
    if button(fires, states, "sound/xfade", sp.xfading()) {
        sp.crossfade(&board);
    }
    if button(fires, states, "sound/pads/stop", false) {
        sp.stop_pads();
    }
    for (i, pad) in board.pads.iter().enumerate() {
        let on = sp.pad_progress(i, pad.length_ms).is_some();
        if button(fires, states, &format!("sound/pad/{i}"), on) && !pad.path.is_empty() {
            sp.pad(&board, i);
        }
    }
}

fn programmer(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    e: &mut Engine,
    st: &mut AppState,
) {
    for m in [
        ProgrammerMode::Edit,
        ProgrammerMode::Blind,
        ProgrammerMode::Live,
    ] {
        let mode = st.programmer_mode;
        let active = match m {
            ProgrammerMode::Edit => mode != ProgrammerMode::Live,
            _ => mode == m,
        };
        if button(fires, states, &format!("programmer/mode/{m:?}"), active) && !st.locked {
            // The eye toggles Blind on/off (always together with Edit)
            st.programmer_mode = match (m, mode) {
                (ProgrammerMode::Blind, ProgrammerMode::Blind) => ProgrammerMode::Edit,
                (m, _) => m,
            };
        }
    }
    let layer = crate::views::quick::Layer::of(st.programmer_mode);
    let any = !layer.map(e).is_empty();
    if button(fires, states, "programmer/reset_all", any) && any && !st.locked {
        layer.clear_all(e);
    }
}

/// Controls of the scene sidebar: they act on the selected scene.
fn scene(
    fires: &mut HashMap<u64, UiFire>,
    states: &mut HashMap<u64, bool>,
    e: &mut Engine,
    st: &mut AppState,
) {
    let mut ids: Vec<_> = st
        .selected_scenes
        .iter()
        .copied()
        .filter(|id| e.show().scene(*id).is_some())
        .collect();
    ids.sort();
    let Some(&id) = ids.first() else {
        return;
    };
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let (reverse, loop_mode) = (scene.settings.reverse, scene.loop_mode);
    let active = e.is_active(id);
    let paused = e.progress(id).is_some_and(|p| p.paused);
    if button(fires, states, "scene/play_pause", active && !paused) {
        if active {
            e.toggle_pause(id);
        } else {
            e.activate(id);
        }
    }
    let pos = e.scene_position(id);
    for (to, key) in [(0.0, "scene/seek_start"), (1.0, "scene/seek_end")] {
        if button(fires, states, key, false) && pos.is_some() {
            e.seek(id, to);
        }
    }
    for (rev, key) in [(true, "scene/reverse"), (false, "scene/forward")] {
        if button(fires, states, key, reverse == rev) {
            for s in &ids {
                if let Some(s) = e.show_mut().scene_mut(*s) {
                    s.settings.reverse = rev;
                }
            }
        }
    }
    for mode in [LoopMode::Once, LoopMode::Loop] {
        if button(
            fires,
            states,
            &format!("scene/loop/{mode:?}"),
            loop_mode == mode,
        ) {
            for s in &ids {
                if let Some(s) = e.show_mut().scene_mut(*s) {
                    s.loop_mode = mode;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_toggle_but_never_all_off() {
        assert_eq!(toggle_cell(u32::MAX, 3, 1), Some(0b101));
        assert_eq!(toggle_cell(0b101, 3, 1), Some(u32::MAX));
        assert_eq!(toggle_cell(0b001, 3, 0), None);
    }

    #[test]
    fn hidden_tracking_follow_works() {
        let mut e = Engine::new(dmxcraft_core::Show::new("T", 1));
        let mut st = AppState::new(String::new());
        let id = e.show_mut().add_tracker("A", Vec::new());
        let mut fires = HashMap::new();
        fires.insert(
            ui_key(&format!("tracking/{id}/follow")),
            UiFire {
                press: true,
                value: None,
            },
        );
        let mut states = HashMap::new();
        apply(&mut fires, &mut states, &mut e, &mut st);
        assert!(e.show().tracker(id).unwrap().active);
        assert!(fires.is_empty(), "taken, the element does not run it again");
        // the next frame reports it as on
        apply(&mut fires, &mut states, &mut e, &mut st);
        assert!(states[&ui_key(&format!("tracking/{id}/follow"))]);
    }
}
