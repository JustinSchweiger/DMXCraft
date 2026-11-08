//! Audio-Trigger der SUPER SCENES: je Trigger wird die Datei einmal mit dem
//! gewählten Filter gelesen (Hüllkurve im Hintergrund), daraus entstehen mit
//! Schwellwert/Hysterese die Ereignisse. Beim Abspielen lösen sie das Ziel
//! wie eine Taste aus (über `touch_fire`, verarbeitet vom Zuordnungs-Rahmen).

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

use dmxcraft_core::audio::{event_at, trigger_events, Envelope};
use dmxcraft_core::mapping::{ButtonMode, Control, Mapping, SceneTarget};
use dmxcraft_core::timeline::{AudioTrigger, ClipContent};
use dmxcraft_core::{Engine, SceneId};
use eframe::egui;
use rodio::Source;

use crate::state::AppState;

type EnvKey = (String, u32, u32);
type EventKey = (String, [u32; 6]);
pub type Events = Arc<Vec<(f64, f64)>>;

#[derive(Clone)]
enum EnvState {
    Loading,
    Ready(Arc<Vec<u8>>),
    Failed,
}

#[derive(Default)]
pub struct TriggerEngine {
    envs: Arc<Mutex<HashMap<EnvKey, EnvState>>>,
    events: Mutex<HashMap<EventKey, Events>>,
    /// Gerade ausgelöste Trigger (Szene, Block, Trigger) und ihre Zuordnung.
    active: HashMap<(SceneId, u32, u32), Mapping>,
}

fn env_key(path: &str, t: &AudioTrigger) -> EnvKey {
    (path.to_string(), t.low_hz.to_bits(), t.high_hz.to_bits())
}

/// Ist das Ziel ein Regler (Wert statt Taste)?
fn is_fader(c: Control) -> bool {
    matches!(
        c,
        Control::Master
            | Control::AudioMaster
            | Control::Channel { .. }
            | Control::Channels
            | Control::Ui { fader: true, .. }
    )
}

fn mapping(t: &AudioTrigger, control: Control) -> Mapping {
    Mapping {
        id: 0,
        control,
        element: None,
        input: None,
        mode: if is_fader(control) {
            ButtonMode::Value
        } else {
            ButtonMode::Toggle
        },
        flash: t.flash,
        feedback: None,
        name: None,
        channels: t.channels.clone(),
    }
}

impl TriggerEngine {
    /// Hüllkurve des Trigger-Bands (startet die Berechnung beim ersten Aufruf).
    pub fn envelope(
        &self,
        path: &str,
        t: &AudioTrigger,
        ctx: &egui::Context,
    ) -> Option<Arc<Vec<u8>>> {
        let key = env_key(path, t);
        let mut map = self.envs.lock().expect("Hüllkurven");
        match map.get(&key) {
            Some(EnvState::Ready(v)) => return Some(v.clone()),
            Some(_) => return None,
            None => {}
        }
        map.insert(key.clone(), EnvState::Loading);
        let envs = self.envs.clone();
        let ctx = ctx.clone();
        let (lo, hi) = (t.low_hz, t.high_hz);
        std::thread::spawn(move || {
            let state = match compute(&key.0, lo, hi) {
                Some(v) => EnvState::Ready(Arc::new(v)),
                None => EnvState::Failed,
            };
            envs.lock().expect("Hüllkurven").insert(key, state);
            ctx.request_repaint();
        });
        None
    }

    /// Ereignisse eines Triggers (ms in der Datei), sobald die Hüllkurve da ist.
    pub fn events(&self, path: &str, t: &AudioTrigger, ctx: &egui::Context) -> Option<Events> {
        let key = (
            path.to_string(),
            [
                t.low_hz.to_bits(),
                t.high_hz.to_bits(),
                t.threshold.to_bits(),
                t.hysteresis.to_bits(),
                t.min_gap_ms,
                t.hold_ms,
            ],
        );
        if let Some(ev) = self.events.lock().expect("Ereignisse").get(&key) {
            return Some(ev.clone());
        }
        let env = self.envelope(path, t, ctx)?;
        let ev = Arc::new(trigger_events(
            &env,
            t.threshold,
            t.hysteresis,
            f64::from(t.min_gap_ms),
            f64::from(t.hold_ms),
        ));
        let mut map = self.events.lock().expect("Ereignisse");
        if map.len() > 256 {
            map.clear();
        }
        map.insert(key, ev.clone());
        Some(ev)
    }

    /// Laufende SUPER SCENES abgleichen: Ereignisse drücken bzw. lösen ihr Ziel.
    pub fn update(&mut self, ctx: &egui::Context, e: &mut Engine, st: &mut AppState) {
        let mut now_on: HashMap<(SceneId, u32, u32), Mapping> = HashMap::new();
        for scene in &e.show().scenes {
            let Some(tl) = scene.timeline() else {
                continue;
            };
            if !e.timeline_running(scene.id) {
                continue;
            }
            let Some(t) = e.timeline_time(scene.id) else {
                continue;
            };
            for (_, clip) in tl.active_at(t as u64) {
                let ClipContent::Audio {
                    path,
                    offset_ms,
                    triggers,
                    ..
                } = &clip.content
                else {
                    continue;
                };
                for tr in triggers.iter().filter(|tr| !tr.muted) {
                    let Some(control) = tr.control else {
                        continue;
                    };
                    let Some(ev) = self.events(path, tr, ctx) else {
                        continue;
                    };
                    let at = *offset_ms as f64 + (t - clip.start_ms as f64) + f64::from(tr.lead_ms);
                    if event_at(&ev, at).is_some() {
                        now_on.insert((scene.id, clip.id, tr.id), mapping(tr, control));
                    }
                }
            }
        }
        // loslassen, was nicht mehr aktiv ist
        let ended: Vec<_> = self
            .active
            .keys()
            .filter(|k| !now_on.contains_key(*k))
            .copied()
            .collect();
        for k in ended {
            if let Some(m) = self.active.remove(&k) {
                fire(e, st, m, false);
            }
        }
        let started: HashSet<_> = now_on
            .keys()
            .filter(|k| !self.active.contains_key(*k))
            .copied()
            .collect();
        for (k, m) in now_on {
            if started.contains(&k) {
                fire(e, st, m.clone(), true);
            }
            self.active.insert(k, m);
        }
        if !self.active.is_empty() || e.show().scenes.iter().any(|s| e.timeline_running(s.id)) {
            ctx.request_repaint();
        }
    }
}

/// Ziel drücken bzw. loslassen. Szenen starten zusätzlich (ohne die laufende
/// SUPER SCENE derselben Bank zu beenden); alles andere wie eine Taste.
fn fire(e: &mut Engine, st: &mut AppState, m: Mapping, pressed: bool) {
    if let Control::Scene {
        target: SceneTarget::Scene { id },
    } = m.control
    {
        match (pressed, m.flash) {
            (true, true) => e.activate_layered(id),
            (false, true) => e.release(id),
            (true, false) if e.is_active(id) => e.release(id),
            (true, false) => e.activate_layered(id),
            (false, false) => {}
        }
        return;
    }
    let fader = is_fader(m.control);
    let value = if pressed { 127 } else { 0 };
    st.touch_fire.push((m, pressed, value, fader));
}

/// Datei dekodieren (Mono) und die Hüllkurve des Bands berechnen.
fn compute(path: &str, lo: f32, hi: f32) -> Option<Vec<u8>> {
    let dec = crate::audio::open_decoder(path)?;
    let channels = usize::from(dec.channels()).max(1);
    let mut env = Envelope::new(dec.sample_rate(), lo, hi);
    let (mut sum, mut n) = (0.0f32, 0usize);
    for s in dec {
        sum += s;
        n += 1;
        if n == channels {
            env.push(sum / channels as f32);
            sum = 0.0;
            n = 0;
        }
    }
    Some(env.finish())
}
