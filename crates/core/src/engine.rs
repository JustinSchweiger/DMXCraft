//! Lichtengine: spielt Szenen ab und berechnet daraus DMX-Frames.
//!
//! Merge-Regeln:
//! * Alle Kanäle: LTP – die zuletzt gestartete Szene gewinnt, aber nur für
//!   Kanäle, die sie gespeichert hat (auch gespeicherte 0-Werte). Nicht
//!   gespeicherte Kanäle bleiben frei für andere Szenen.
//! * Der Programmer (Edit-Werte) überschreibt die Szenen.
//! * Live-Werte überschreiben alles; sie gehören nicht zur Show und werden
//!   nicht gespeichert.
//! * Master/Blackout skalieren Dimmer-Kanäle; Geräte ohne Dimmer-Kanal werden
//!   über ihre Farbkanäle gedimmt.

use std::collections::HashMap;
use std::time::Duration;

use crate::dmx::DmxFrame;
use crate::fixture::ChannelKind;
use crate::show::{ChannelRef, FixtureLimits, LoopMode, SceneId, Show};

/// Obergrenze für Schrittwechsel pro Tick (Schutz vor Endlosschleifen bei
/// Szenen mit sehr kurzen Schritten).
const MAX_STEP_ADVANCES_PER_TICK: usize = 1000;

#[derive(Clone, Debug)]
struct PlayingScene {
    scene: SceneId,
    /// Startreihenfolge für LTP.
    seq: u64,
    step: usize,
    elapsed_ms: f64,
    /// Noch im ersten Durchlauf von Schritt 0 (es gibt keinen Vorgänger).
    first_entry: bool,
    finished: bool,
    paused: bool,
    /// Zeit seit dem Start (für das Einblenden).
    age_ms: f64,
    /// Zeit seit dem Stoppen, solange die Szene ausblendet.
    releasing: Option<f64>,
}

/// Ein-/Ausblendzustand einer Szene (Pegel 0..=1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FadeState {
    In(f32),
    Out(f32),
}

/// Fortschritt einer laufenden Szene für die Oberfläche.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneProgress {
    pub step: usize,
    pub step_count: usize,
    /// 0.0..=1.0 innerhalb des aktuellen Schritts.
    pub step_fraction: f32,
    pub finished: bool,
    pub paused: bool,
}

pub struct Engine {
    show: Show,
    playing: Vec<PlayingScene>,
    next_seq: u64,
    programmer: HashMap<ChannelRef, u8>,
    /// Vorschau-Szenen (Edit-Modus): werden ausgegeben, auch wenn sie nicht
    /// laufen – ihr erster Schritt liegt über den laufenden Szenen.
    preview: Vec<SceneId>,
    /// Live-Werte (Live-Modus): überschreiben alles, werden nicht gespeichert.
    live: HashMap<ChannelRef, u8>,
    /// Blind-Modus: Ausgang nutzt diesen eingefrorenen Stand ohne Edit-Werte.
    blind_show: Option<Show>,
    master: f32,
    blackout: bool,
    speed: f32,
}

impl Engine {
    pub fn new(show: Show) -> Self {
        Self {
            show,
            playing: Vec::new(),
            next_seq: 1,
            programmer: HashMap::new(),
            preview: Vec::new(),
            live: HashMap::new(),
            blind_show: None,
            master: 1.0,
            blackout: false,
            speed: 1.0,
        }
    }

    // ---------- Show ----------

    pub fn show(&self) -> &Show {
        &self.show
    }

    /// Zum Bearbeiten der Show. Danach ggf. [`Engine::sync`] aufrufen.
    pub fn show_mut(&mut self) -> &mut Show {
        &mut self.show
    }

    /// Ersetzt die Show komplett (z. B. nach dem Laden) und stoppt alles.
    pub fn replace_show(&mut self, show: Show) {
        self.show = show;
        self.playing.clear();
        self.programmer.clear();
        self.live.clear();
        self.blind_show = None;
    }

    /// Entfernt Wiedergabe-/Programmer-Zustand, der nach Show-Änderungen
    /// nicht mehr gültig ist.
    pub fn sync(&mut self) {
        let show = &self.show;
        self.playing
            .retain(|p| show.scene(p.scene).is_some_and(|s| !s.steps.is_empty()));
        for p in &mut self.playing {
            let len = show.scene(p.scene).map_or(1, |s| s.steps.len());
            if p.step >= len {
                p.step = 0;
                p.elapsed_ms = 0.0;
            }
        }
        self.programmer
            .retain(|ch, _| show.channel_def(*ch).is_some());
        self.live.retain(|ch, _| show.channel_def(*ch).is_some());
    }

    // ---------- Szenen ----------

    /// Läuft die Szene (ausblendende Szenen zählen nicht mehr)?
    pub fn is_active(&self, id: SceneId) -> bool {
        self.playing
            .iter()
            .any(|p| p.scene == id && p.releasing.is_none())
    }

    pub fn active_scenes(&self) -> Vec<SceneId> {
        self.playing
            .iter()
            .filter(|p| p.releasing.is_none())
            .map(|p| p.scene)
            .collect()
    }

    fn fade_out_ms(show: &Show, id: SceneId) -> u32 {
        show.scene(id).map_or(0, |s| s.settings.fade_out_ms)
    }

    /// Szene stoppen – mit ihrer Ausblendzeit, sonst sofort.
    fn stop(show: &Show, p: &mut PlayingScene) -> bool {
        if Self::fade_out_ms(show, p.scene) == 0 {
            return false; // entfernen
        }
        if p.releasing.is_none() {
            p.releasing = Some(0.0);
        }
        true
    }

    /// Startet eine Szene (von vorn). Andere Szenen derselben Zone stoppen.
    pub fn activate(&mut self, id: SceneId) {
        let Some(scene) = self.show.scene(id) else {
            return;
        };
        if scene.steps.is_empty() {
            return;
        }
        let (scene_reverse, scene_len) = (scene.settings.reverse, scene.steps.len());
        let zone = scene.zone;
        let show = &self.show;
        // dieselbe Szene startet neu; andere Szenen der Bank blenden aus
        self.playing.retain_mut(|p| {
            if p.scene == id {
                return false;
            }
            let same_zone = zone.is_some() && show.scene(p.scene).is_some_and(|s| s.zone == zone);
            !same_zone || Self::stop(show, p)
        });
        self.playing.push(PlayingScene {
            scene: id,
            seq: self.next_seq,
            step: if scene_reverse { scene_len - 1 } else { 0 },
            elapsed_ms: 0.0,
            first_entry: true,
            finished: false,
            paused: false,
            age_ms: 0.0,
            releasing: None,
        });
        self.next_seq += 1;
    }

    /// Stoppt eine Szene (mit ihrer Ausblendzeit).
    pub fn release(&mut self, id: SceneId) {
        let show = &self.show;
        self.playing
            .retain_mut(|p| p.scene != id || Self::stop(show, p));
    }

    /// Springt in einer laufenden Szene an eine Stelle (0 = Anfang, 1 = Ende).
    pub fn seek(&mut self, id: SceneId, fraction: f32) {
        let Some(scene) = self.show.scene(id) else {
            return;
        };
        let total = scene.duration_ms();
        let Some(p) = self
            .playing
            .iter_mut()
            .find(|p| p.scene == id && p.releasing.is_none())
        else {
            return;
        };
        let mut target = (f64::from(fraction.clamp(0.0, 1.0)) * total as f64)
            .min(total as f64 - 1.0)
            .max(0.0);
        p.finished = false;
        p.first_entry = target == 0.0;
        for (i, st) in scene.steps.iter().enumerate() {
            let d = st.duration_ms() as f64;
            if target < d || i + 1 == scene.steps.len() {
                p.step = i;
                p.elapsed_ms = target.min(d);
                return;
            }
            target -= d;
        }
    }

    /// Blendet die Szene gerade ein oder aus?
    pub fn fade_state(&self, id: SceneId) -> Option<FadeState> {
        let p = self.playing.iter().rev().find(|p| p.scene == id)?;
        let st = &self.show.scene(id)?.settings;
        if let Some(r) = p.releasing {
            let out = f64::from(st.fade_out_ms).max(1.0);
            return Some(FadeState::Out((1.0 - r / out).clamp(0.0, 1.0) as f32));
        }
        (st.fade_in_ms > 0 && p.age_ms < f64::from(st.fade_in_ms))
            .then(|| FadeState::In((p.age_ms / f64::from(st.fade_in_ms)) as f32))
    }

    /// Fortschritt über die ganze Szene (0..=1), falls sie läuft.
    pub fn scene_position(&self, id: SceneId) -> Option<f32> {
        let p = self
            .playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())?;
        let scene = self.show.scene(id)?;
        let total = scene.duration_ms().max(1) as f64;
        let before: u64 = scene.steps[..p.step].iter().map(|s| s.duration_ms()).sum();
        Some(((before as f64 + p.elapsed_ms) / total).clamp(0.0, 1.0) as f32)
    }

    pub fn toggle(&mut self, id: SceneId) {
        if self.is_active(id) {
            self.release(id);
        } else {
            self.activate(id);
        }
    }

    pub fn release_all(&mut self) {
        self.playing.clear();
    }

    pub fn progress(&self, id: SceneId) -> Option<SceneProgress> {
        let p = self
            .playing
            .iter()
            .find(|p| p.scene == id && p.releasing.is_none())?;
        let scene = self.show.scene(id)?;
        let step = scene.steps.get(p.step)?;
        let dur = step.duration_ms().max(1) as f64;
        Some(SceneProgress {
            step: p.step,
            step_count: scene.steps.len(),
            step_fraction: (p.elapsed_ms / dur).clamp(0.0, 1.0) as f32,
            finished: p.finished,
            paused: p.paused,
        })
    }

    /// Hält eine laufende Szene an bzw. setzt sie fort.
    pub fn toggle_pause(&mut self, id: SceneId) {
        if let Some(p) = self.playing.iter_mut().find(|p| p.scene == id) {
            p.paused = !p.paused;
        }
    }

    /// Springt zum nächsten (`delta = 1`) oder vorherigen (`-1`) Schritt.
    /// Der neue Schritt blendet mit seiner Überblendzeit ein.
    pub fn jump_step(&mut self, id: SceneId, delta: i32) {
        let Some(len) = self.show.scene(id).map(|s| s.steps.len()) else {
            return;
        };
        if len == 0 {
            return;
        }
        if let Some(p) = self.playing.iter_mut().find(|p| p.scene == id) {
            let n = len as i64;
            p.step = (p.step as i64 + i64::from(delta)).rem_euclid(n) as usize;
            p.elapsed_ms = 0.0;
            p.first_entry = false;
            p.finished = false;
        }
    }

    /// Aktive Szene einer Zone (bei Zonen ist höchstens eine aktiv).
    pub fn active_in_zone(&self, zone: crate::show::ZoneId) -> Option<SceneId> {
        self.playing
            .iter()
            .rev()
            .map(|p| p.scene)
            .find(|id| self.show.scene(*id).is_some_and(|s| s.zone == Some(zone)))
    }

    // ---------- Programmer ----------

    pub fn set_programmer(&mut self, ch: ChannelRef, value: u8) {
        self.programmer.insert(ch, value);
    }

    pub fn clear_programmer_channel(&mut self, ch: ChannelRef) {
        self.programmer.remove(&ch);
    }

    // ---------- Blind ----------

    /// Blind-Modus ein/aus: beim Einschalten wird der Ausgang auf dem
    /// aktuellen Stand eingefroren, Bearbeitungen wirken erst nach dem
    /// Ausschalten.
    pub fn set_blind(&mut self, on: bool) {
        if on && self.blind_show.is_none() {
            self.blind_show = Some(self.show.clone());
        } else if !on {
            self.blind_show = None;
        }
    }

    pub fn is_blind(&self) -> bool {
        self.blind_show.is_some()
    }

    // ---------- Live-Werte ----------

    pub fn set_live(&mut self, ch: ChannelRef, value: u8) {
        self.live.insert(ch, value);
    }

    pub fn clear_live_channel(&mut self, ch: ChannelRef) {
        self.live.remove(&ch);
    }

    pub fn clear_live(&mut self) {
        self.live.clear();
    }

    pub fn live(&self) -> &HashMap<ChannelRef, u8> {
        &self.live
    }

    /// Alle Live-Werte ersetzen (z. B. Rückgängig).
    pub fn replace_live(&mut self, live: HashMap<ChannelRef, u8>) {
        self.live = live;
    }

    /// Vorschau-Szenen setzen (leer = keine Vorschau).
    pub fn set_preview(&mut self, scenes: Vec<SceneId>) {
        self.preview = scenes;
    }

    pub fn preview(&self) -> &[SceneId] {
        &self.preview
    }

    pub fn clear_programmer(&mut self) {
        self.programmer.clear();
    }

    pub fn programmer(&self) -> &HashMap<ChannelRef, u8> {
        &self.programmer
    }

    // ---------- Master / Speed ----------

    pub fn master(&self) -> f32 {
        self.master
    }

    pub fn set_master(&mut self, v: f32) {
        self.master = v.clamp(0.0, 1.0);
    }

    pub fn blackout(&self) -> bool {
        self.blackout
    }

    pub fn set_blackout(&mut self, on: bool) {
        self.blackout = on;
    }

    /// Wiedergabegeschwindigkeit (1.0 = normal).
    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn set_speed(&mut self, v: f32) {
        self.speed = v.clamp(0.05, 20.0);
    }

    // ---------- Zeit ----------

    /// Zeit voranschreiten lassen.
    pub fn tick(&mut self, dt: Duration) {
        let dt_ms = dt.as_secs_f64() * 1000.0 * f64::from(self.speed);
        let real_ms = dt.as_secs_f64() * 1000.0;
        // ausgeblendete Szenen entfernen
        let show = &self.show;
        self.playing.retain_mut(|p| match &mut p.releasing {
            Some(r) => {
                *r += real_ms;
                *r < f64::from(Self::fade_out_ms(show, p.scene))
            }
            None => true,
        });
        for p in &mut self.playing {
            let Some(scene) = self.show.scene(p.scene) else {
                continue;
            };
            p.age_ms += real_ms;
            let len = scene.steps.len();
            if len == 0 || p.finished || p.paused {
                continue;
            }
            p.elapsed_ms += dt_ms * f64::from(scene.settings.speed_pct) / 100.0;
            let mut guard = 0;
            loop {
                let dur = scene.steps[p.step].duration_ms().max(1) as f64;
                if p.elapsed_ms < dur || guard >= MAX_STEP_ADVANCES_PER_TICK {
                    break;
                }
                guard += 1;
                let reverse = scene.settings.reverse;
                let has_next = if reverse {
                    p.step > 0
                } else {
                    p.step + 1 < len
                };
                if has_next {
                    p.elapsed_ms -= dur;
                    if reverse {
                        p.step -= 1;
                    } else {
                        p.step += 1;
                    }
                    p.first_entry = false;
                } else if scene.loop_mode == LoopMode::Loop {
                    p.elapsed_ms -= dur;
                    p.step = if reverse { len - 1 } else { 0 };
                    p.first_entry = false;
                } else {
                    p.elapsed_ms = dur;
                    p.finished = true;
                    break;
                }
            }
        }
    }

    // ---------- Rendering ----------

    /// Intensitätskanal für den Szenen-Dimmer: Dimmer, bei Geräten ohne
    /// Dimmer-Kanal die Farbkanäle.
    fn is_intensity(show: &Show, ch: ChannelRef) -> bool {
        let Some(def) = show.channel_def(ch) else {
            return false;
        };
        if def.kind.is_htp() {
            return true;
        }
        let has_dimmer = show
            .fixture(ch.fixture)
            .and_then(|f| show.type_of(f))
            .is_some_and(|t| t.channels.iter().any(|c| c.kind.is_htp()));
        !has_dimmer && def.kind.is_color_intensity()
    }

    fn is_htp(show: &Show, ch: ChannelRef) -> bool {
        show.channel_def(ch).is_some_and(|d| d.kind.is_htp())
    }

    /// Aktuelle (überblendete) Werte einer laufenden Szene.
    fn scene_values(&self, show: &Show, p: &PlayingScene) -> HashMap<ChannelRef, u8> {
        let mut out = HashMap::new();
        let Some(scene) = show.scene(p.scene) else {
            return out;
        };
        let len = scene.steps.len();
        let Some(step) = scene.steps.get(p.step) else {
            return out;
        };
        let current = step.value_map();
        // Fade-Anteil der Szene skaliert die Schritt-Überblendung
        let speed = f64::from(scene.settings.fade_speed.clamp(1, 40)) / 10.0;
        let fade = f64::from(step.fade_ms) / speed;
        let t = if fade < 1.0 {
            1.0
        } else {
            (p.elapsed_ms / fade).clamp(0.0, 1.0)
        };
        let previous = if p.first_entry {
            None
        } else {
            // Vorgänger in Abspielrichtung
            let prev = if scene.settings.reverse {
                (p.step + 1) % len
            } else {
                (p.step + len - 1) % len
            };
            Some(scene.steps[prev].value_map())
        };

        for (&ch, &to) in &current {
            let from = match previous.as_ref().and_then(|m| m.get(&ch)) {
                Some(&v) => v,
                // Ohne Vorgänger: Intensität blendet von 0 auf, Rest springt.
                None if Self::is_htp(show, ch) => 0,
                None => to,
            };
            out.insert(ch, lerp(from, to, t));
        }
        // Intensitäten, die im neuen Schritt fehlen, blenden aus.
        if let Some(prev) = previous {
            for (ch, from) in prev {
                if !current.contains_key(&ch) && Self::is_htp(show, ch) {
                    out.insert(ch, lerp(from, 0, t));
                }
            }
        }
        out
    }

    /// Berechnet den aktuellen DMX-Frame für den Ausgang. Im Blind-Modus ohne
    /// Edit-Werte und mit dem beim Umschalten eingefrorenen Show-Stand.
    pub fn render(&self, frame: &mut DmxFrame) {
        match &self.blind_show {
            Some(frozen) => self.render_show(frozen, false, frame),
            None => self.render_show(&self.show, true, frame),
        }
    }

    /// Frame mit allen Bearbeitungen (Vorschau für Bühne und Fader, auch im
    /// Blind-Modus).
    pub fn render_monitor(&self) -> DmxFrame {
        let mut f = DmxFrame::new(usize::from(self.show.universes));
        self.render_show(&self.show, true, &mut f);
        f
    }

    fn render_show(&self, show: &Show, edit: bool, frame: &mut DmxFrame) {
        frame.resize(usize::from(show.universes));
        frame.clear();

        // 1) Defaults
        let mut values: HashMap<ChannelRef, u8> = HashMap::new();
        for f in &show.fixtures {
            if let Some(ty) = show.type_of(f) {
                for (i, def) in ty.channels.iter().enumerate() {
                    let ch = ChannelRef {
                        fixture: f.id,
                        channel: i as u16,
                    };
                    values.insert(ch, def.default);
                }
            }
        }

        // 2) Szenen in Startreihenfolge (LTP: spätere überschreiben)
        let mut order: Vec<&PlayingScene> = self.playing.iter().collect();
        order.sort_by_key(|p| p.seq);
        // Nur gespeicherte Kanäle einer Szene wirken; nicht gespeicherte
        // bleiben frei für andere Szenen.
        for p in order {
            let Some(settings) = show.scene(p.scene).map(|s| &s.settings) else {
                continue;
            };
            // Ein-/Ausblenden der ganzen Szene (Überblendung zum Darunterliegenden)
            let mut k = 1.0f64;
            if settings.fade_in_ms > 0 {
                k *= (p.age_ms / f64::from(settings.fade_in_ms)).clamp(0.0, 1.0);
            }
            if let Some(r) = p.releasing {
                let out = f64::from(settings.fade_out_ms).max(1.0);
                k *= 1.0 - (r / out).clamp(0.0, 1.0);
            }
            let dim = f64::from(settings.dimmer) / 255.0;
            for (ch, v) in self.scene_values(show, p) {
                let v = if dim < 1.0 && Self::is_intensity(show, ch) {
                    (f64::from(v) * dim).round() as u8
                } else {
                    v
                };
                if let Some(slot) = values.get_mut(&ch) {
                    *slot = if k >= 1.0 { v } else { lerp(*slot, v, k) };
                }
            }
        }

        // 3) Vorschau nicht laufender Szenen (erster Schritt, LTP)
        let preview: &[SceneId] = if edit { &self.preview } else { &[] };
        for id in preview {
            if self.is_active(*id) {
                continue;
            }
            let Some(step) = show.scene(*id).and_then(|s| s.steps.first()) else {
                continue;
            };
            for cv in &step.values {
                let ch = ChannelRef {
                    fixture: cv.fixture,
                    channel: cv.channel,
                };
                if let Some(slot) = values.get_mut(&ch) {
                    *slot = cv.value;
                }
            }
        }

        // 4) Programmer
        for (ch, v) in self.programmer.iter().filter(|_| edit) {
            if let Some(slot) = values.get_mut(ch) {
                *slot = *v;
            }
        }

        // 5) Live-Werte
        for (ch, v) in &self.live {
            if let Some(slot) = values.get_mut(ch) {
                *slot = *v;
            }
        }

        // 6) Master / Blackout
        let master = if self.blackout { 0.0 } else { self.master };
        if master < 1.0 {
            for f in &show.fixtures {
                let Some(ty) = show.type_of(f) else {
                    continue;
                };
                let has_dimmer = ty.channels.iter().any(|c| c.kind.is_htp());
                for (i, def) in ty.channels.iter().enumerate() {
                    let scale = if has_dimmer {
                        def.kind.is_htp()
                    } else {
                        def.kind.is_color_intensity()
                    };
                    if scale {
                        let ch = ChannelRef {
                            fixture: f.id,
                            channel: i as u16,
                        };
                        if let Some(v) = values.get_mut(&ch) {
                            *v = (f32::from(*v) * master).round() as u8;
                        }
                    }
                }
            }
        }

        // 4b) Gerätegrenzen
        for f in &show.fixtures {
            let lim = f.limits;
            if lim.is_default() {
                continue;
            }
            let Some(ty) = show.type_of(f) else {
                continue;
            };
            let ch = |i: usize| ChannelRef {
                fixture: f.id,
                channel: i as u16,
            };
            if lim.affects_dimmer() {
                let has_dimmer = ty.channels.iter().any(|c| c.kind.is_htp());
                for (i, def) in ty.channels.iter().enumerate() {
                    let scale = if has_dimmer {
                        def.kind.is_htp()
                    } else {
                        def.kind.is_color_intensity()
                    };
                    if scale {
                        if let Some(v) = values.get_mut(&ch(i)) {
                            *v = lim.map_dimmer(*v);
                        }
                    }
                }
            }
            if lim.affects_position() {
                let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k);
                let (pan, pan_f, tilt, tilt_f) = (
                    idx(ChannelKind::Pan),
                    idx(ChannelKind::PanFine),
                    idx(ChannelKind::Tilt),
                    idx(ChannelKind::TiltFine),
                );
                let read = |coarse: Option<usize>, fine: Option<usize>| -> Option<u16> {
                    let c = values.get(&ch(coarse?)).copied()?;
                    let fv = fine.and_then(|i| values.get(&ch(i)).copied()).unwrap_or(0);
                    Some(u16::from(c) << 8 | u16::from(fv))
                };
                let (mut p16, mut t16) = (read(pan, pan_f), read(tilt, tilt_f));
                if lim.swap_pan_tilt {
                    std::mem::swap(&mut p16, &mut t16);
                }
                let mut write = |coarse: Option<usize>, fine: Option<usize>, v: u16| {
                    if let Some(slot) = coarse.and_then(|i| values.get_mut(&ch(i))) {
                        *slot = (v >> 8) as u8;
                    }
                    if let Some(slot) = fine.and_then(|i| values.get_mut(&ch(i))) {
                        *slot = (v & 0xFF) as u8;
                    }
                };
                if let Some(v) = p16 {
                    write(
                        pan,
                        pan_f,
                        FixtureLimits::map16(v, lim.pan_min, lim.pan_max, lim.invert_pan),
                    );
                }
                if let Some(v) = t16 {
                    write(
                        tilt,
                        tilt_f,
                        FixtureLimits::map16(v, lim.tilt_min, lim.tilt_max, lim.invert_tilt),
                    );
                }
            }
        }

        // 5) In den Frame schreiben
        for f in &show.fixtures {
            for (ch, v) in values.iter().filter(|(ch, _)| ch.fixture == f.id) {
                frame.set(usize::from(f.universe), f.address + ch.channel, *v);
            }
        }
    }

    /// Bequemlichkeitsfunktion: neuen Frame erzeugen.
    pub fn render_frame(&self) -> DmxFrame {
        let mut f = DmxFrame::new(usize::from(self.show.universes));
        self.render(&mut f);
        f
    }

    /// Aktueller Ausgabewert eines Gerätekanals (für Fader-Anzeigen).
    pub fn output_value(&self, frame: &DmxFrame, ch: ChannelRef) -> Option<u8> {
        let f = self.show.fixture(ch.fixture)?;
        frame.get(usize::from(f.universe), f.address + ch.channel)
    }
}

fn lerp(from: u8, to: u8, t: f64) -> u8 {
    (f64::from(from) + (f64::from(to) - f64::from(from)) * t).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::builtin_fixture_types;
    use crate::show::{ChannelValue, FixtureId, Step};

    const MS: fn(u64) -> Duration = Duration::from_millis;

    fn cv(fixture: FixtureId, channel: u16, value: u8) -> ChannelValue {
        ChannelValue {
            fixture,
            channel,
            value,
        }
    }

    /// Show mit einem D+RGB-PAR (Adr. 1) und einem RGB-PAR ohne Dimmer (Adr. 10).
    fn setup() -> (Show, FixtureId, FixtureId) {
        let mut s = Show::new("T", 2);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        let a = s.add_fixture("A", "generic/drgb-4ch", 0, 1).unwrap();
        let b = s.add_fixture("B", "generic/rgb-3ch", 1, 10).unwrap();
        (s, a, b)
    }

    #[test]
    fn defaults_are_rendered() {
        let mut s = Show::new("T", 1);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        s.add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
            .unwrap();
        let e = Engine::new(s);
        let f = e.render_frame();
        assert_eq!(f.get(0, 100), Some(128)); // Pan
        assert_eq!(f.get(0, 102), Some(128)); // Tilt
        assert_eq!(f.get(0, 105), Some(0)); // Dimmer
    }

    #[test]
    fn scene_values_land_on_correct_addresses() {
        let (mut s, a, b) = setup();
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![cv(a, 0, 255), cv(a, 1, 200), cv(b, 2, 50)])],
        );
        let mut e = Engine::new(s);
        e.activate(id);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(255));
        assert_eq!(f.get(0, 2), Some(200));
        assert_eq!(f.get(1, 12), Some(50));
    }

    #[test]
    fn latest_scene_wins_for_stored_channels() {
        let (mut s, a, _) = setup();
        let s1 = s.add_scene(
            "1",
            None,
            vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 10)])],
        );
        let s2 = s.add_scene(
            "2",
            None,
            vec![Step::new(vec![cv(a, 0, 100), cv(a, 1, 99)])],
        );
        let mut e = Engine::new(s);
        e.activate(s1);
        e.activate(s2);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(100), "Dimmer LTP");
        assert_eq!(f.get(0, 2), Some(99), "Rot LTP");
        e.activate(s1); // neu gestartet -> jetzt die neueste
        assert_eq!(e.render_frame().get(0, 2), Some(10));
    }

    #[test]
    fn unstored_channels_stay_free() {
        // Rot-Szene speichert nur Rot, Blau-Szene nur Blau -> beide sichtbar.
        let (mut s, a, _) = setup();
        let red = s.add_scene("R", None, vec![Step::new(vec![cv(a, 1, 255)])]);
        let blue = s.add_scene("B", None, vec![Step::new(vec![cv(a, 3, 255)])]);
        let mut e = Engine::new(s);
        e.activate(red);
        e.activate(blue);
        let f = e.render_frame();
        assert_eq!(f.get(0, 2), Some(255));
        assert_eq!(f.get(0, 4), Some(255));
        // speichert die Blau-Szene Rot = 0, gewinnt sie (zuletzt gestartet)
        e.show_mut().scene_mut(blue).unwrap().steps[0]
            .values
            .push(cv(a, 1, 0));
        assert_eq!(e.render_frame().get(0, 2), Some(0));
    }

    #[test]
    fn zones_are_exclusive() {
        let (mut s, a, _) = setup();
        let z = s.add_zone("Farbe");
        let s1 = s.add_scene("Rot", Some(z), vec![Step::new(vec![cv(a, 1, 255)])]);
        let s2 = s.add_scene("Blau", Some(z), vec![Step::new(vec![cv(a, 3, 255)])]);
        let free = s.add_scene("Frei", None, vec![Step::new(vec![cv(a, 0, 255)])]);
        let mut e = Engine::new(s);
        e.activate(free);
        e.activate(s1);
        e.activate(s2);
        assert!(!e.is_active(s1));
        assert!(e.is_active(s2));
        assert!(e.is_active(free));
        e.toggle(s2);
        assert!(!e.is_active(s2));
    }

    #[test]
    fn fades_between_steps_and_loops() {
        let (mut s, a, _) = setup();
        let mut st1 = Step::new(vec![cv(a, 1, 0)]);
        st1.fade_ms = 0;
        st1.hold_ms = 100;
        let mut st2 = Step::new(vec![cv(a, 1, 200)]);
        st2.fade_ms = 1000;
        st2.hold_ms = 0;
        let id = s.add_scene("Fade", None, vec![st1, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.render_frame().get(0, 2), Some(0));
        e.tick(MS(100)); // Schritt 2 beginnt
        assert_eq!(e.progress(id).unwrap().step, 1);
        e.tick(MS(500)); // Hälfte der Überblendung
        assert_eq!(e.render_frame().get(0, 2), Some(100));
        e.tick(MS(500)); // Ende -> Loop zu Schritt 1 (fade 0)
        assert_eq!(e.progress(id).unwrap().step, 0);
        assert_eq!(e.render_frame().get(0, 2), Some(0));
    }

    #[test]
    fn first_step_fades_intensity_in() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 0, 200), cv(a, 1, 80)]);
        st.fade_ms = 1000;
        let id = s.add_scene("In", None, vec![st]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(MS(250));
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(50), "Dimmer blendet auf");
        assert_eq!(f.get(0, 2), Some(80), "Farbe springt");
    }

    #[test]
    fn once_mode_stops_on_last_step() {
        let (mut s, a, _) = setup();
        let id = s.add_scene(
            "Once",
            None,
            vec![Step::new(vec![cv(a, 1, 1)]), Step::new(vec![cv(a, 1, 2)])],
        );
        s.scene_mut(id).unwrap().loop_mode = LoopMode::Once;
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(MS(10_000));
        let p = e.progress(id).unwrap();
        assert!(p.finished);
        assert_eq!(p.step, 1);
        assert_eq!(e.render_frame().get(0, 2), Some(2));
    }

    #[test]
    fn zero_length_steps_do_not_hang() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 1, 1)]);
        st.hold_ms = 0;
        let mut st2 = st.clone();
        st2.values[0].value = 2;
        let id = s.add_scene("Z", None, vec![st, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.tick(Duration::from_secs(3600)); // darf nicht hängen
        assert!(e.is_active(id));
    }

    #[test]
    fn scene_dimmer_fades_and_seek() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 200)])]);
        s.scene_mut(id).unwrap().settings.dimmer = 128;
        s.scene_mut(id).unwrap().settings.fade_in_ms = 1000;
        s.scene_mut(id).unwrap().settings.fade_out_ms = 1000;
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(
            e.render_frame().get(0, 1),
            Some(0),
            "Einblenden beginnt bei 0"
        );
        e.tick(Duration::from_millis(500));
        assert_eq!(
            e.render_frame().get(0, 1),
            Some(50),
            "halb eingeblendet, Dimmer 50 %"
        );
        e.tick(Duration::from_millis(600));
        assert_eq!(e.render_frame().get(0, 1), Some(100));
        e.release(id);
        assert!(!e.is_active(id));
        e.tick(Duration::from_millis(500));
        assert_eq!(e.render_frame().get(0, 1), Some(50), "blendet aus");
        e.tick(Duration::from_millis(600));
        assert_eq!(e.render_frame().get(0, 1), Some(0));
        assert!(e.playing.is_empty());

        // Seek in einem Ablauf
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 1, 1)]);
        st.hold_ms = 1000;
        let mut st2 = st.clone();
        st2.values[0].value = 2;
        let id = s.add_scene("Z", None, vec![st, st2]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.seek(id, 0.75);
        assert_eq!(e.render_frame().get(0, 2), Some(2));
        assert!((e.scene_position(id).unwrap() - 0.75).abs() < 0.01);
    }

    #[test]
    fn reverse_plays_steps_backwards() {
        let (mut s, a, _) = setup();
        let steps: Vec<Step> = (1..=3)
            .map(|v| {
                let mut st = Step::new(vec![cv(a, 1, v)]);
                st.hold_ms = 100;
                st
            })
            .collect();
        let id = s.add_scene("R", None, steps);
        s.scene_mut(id).unwrap().settings.reverse = true;
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.render_frame().get(0, 2), Some(3));
        e.tick(Duration::from_millis(110));
        assert_eq!(e.render_frame().get(0, 2), Some(2));
        e.tick(Duration::from_millis(100));
        e.tick(Duration::from_millis(100));
        assert_eq!(e.render_frame().get(0, 2), Some(3), "Schleife rückwärts");
    }

    #[test]
    fn blind_keeps_output_but_monitor_shows_edits() {
        let (mut s, a, _) = setup();
        let red = s.add_scene("Rot", None, vec![Step::new(vec![cv(a, 1, 255)])]);
        let mut e = Engine::new(s);
        e.activate(red);
        e.set_blind(true);
        // Szene bearbeiten und Programmer setzen: Ausgang bleibt
        e.show_mut().scene_mut(red).unwrap().steps[0].values[0].value = 10;
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 3,
            },
            200,
        );
        let out = e.render_frame();
        assert_eq!((out.get(0, 2), out.get(0, 4)), (Some(255), Some(0)));
        let mon = e.render_monitor();
        assert_eq!((mon.get(0, 2), mon.get(0, 4)), (Some(10), Some(200)));
        e.set_blind(false);
        assert_eq!(e.render_frame().get(0, 2), Some(10));
    }

    #[test]
    fn live_overrides_scenes_and_resets() {
        let (mut s, a, _) = setup();
        let red = s.add_scene(
            "Rot",
            None,
            vec![Step::new(vec![cv(a, 1, 255), cv(a, 2, 0), cv(a, 3, 0)])],
        );
        let mut e = Engine::new(s);
        e.activate(red);
        let blue = ChannelRef {
            fixture: a,
            channel: 3,
        };
        e.set_live(blue, 255);
        let f = e.render_frame();
        assert_eq!((f.get(0, 2), f.get(0, 4)), (Some(255), Some(255)));
        e.clear_live();
        assert_eq!(e.render_frame().get(0, 4), Some(0));
    }

    #[test]
    fn preview_outputs_inactive_scene() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("P", None, vec![Step::new(vec![cv(a, 1, 77)])]);
        let mut e = Engine::new(s);
        let ch = ChannelRef {
            fixture: a,
            channel: 1,
        };
        let before = e.render_frame();
        e.set_preview(vec![id]);
        let f = e.render_frame();
        assert_eq!(e.output_value(&f, ch), Some(77));
        assert!(!e.is_active(id));
        e.set_preview(vec![]);
        assert_eq!(e.render_frame(), before);
    }

    #[test]
    fn programmer_overrides_and_master_scales() {
        let (mut s, a, b) = setup();
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 200), cv(b, 0, 200)])],
        );
        let mut e = Engine::new(s);
        e.activate(id);
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 0,
            },
            100,
        );
        e.set_master(0.5);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(50), "Programmer 100 * Master 0.5");
        assert_eq!(
            f.get(0, 2),
            Some(200),
            "Farbe bei Gerät mit Dimmer unverändert"
        );
        assert_eq!(f.get(1, 10), Some(100), "Gerät ohne Dimmer: Farbe skaliert");
        e.set_blackout(true);
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(0));
        assert_eq!(f.get(1, 10), Some(0));
    }

    #[test]
    fn pause_and_step_jumps() {
        let (mut s, a, _) = setup();
        let steps = (0..3u8)
            .map(|i| {
                let mut st = Step::new(vec![cv(a, 1, i * 10)]);
                st.hold_ms = 1000;
                st
            })
            .collect();
        let z = s.add_zone("Z");
        let id = s.add_scene("S", Some(z), steps);
        let mut e = Engine::new(s);
        e.activate(id);
        assert_eq!(e.active_in_zone(z), Some(id));
        e.toggle_pause(id);
        e.tick(MS(5000));
        assert_eq!(e.progress(id).unwrap().step, 0, "pausiert");
        e.jump_step(id, 1);
        assert_eq!(e.render_frame().get(0, 2), Some(10));
        e.jump_step(id, -2);
        assert_eq!(e.progress(id).unwrap().step, 2, "rückwärts mit Umlauf");
        e.toggle_pause(id);
        e.tick(MS(1000));
        assert_eq!(e.progress(id).unwrap().step, 0);
    }

    #[test]
    fn limits_cap_dimmer_and_position() {
        let mut s = Show::new("T", 1);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        let mh = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        // Kanäle: 0 Pan, 1 Pan fein, 2 Tilt, 3 Tilt fein, 5 Dimmer
        let id = s.add_scene(
            "S",
            None,
            vec![Step::new(vec![
                cv(mh, 0, 255),
                cv(mh, 1, 255),
                cv(mh, 2, 0),
                cv(mh, 5, 200),
            ])],
        );
        {
            let l = &mut s.fixture_mut(mh).unwrap().limits;
            l.dimmer_max = 128;
            l.pan_min = 64;
            l.pan_max = 191;
            l.invert_tilt = true;
        }
        let mut e = Engine::new(s);
        e.activate(id);
        let f = e.render_frame();
        assert_eq!(f.get(0, 6), Some(100), "Dimmer 200 * 128/255");
        e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 64;
        assert_eq!(
            e.render_frame().get(0, 6),
            Some((64 + 200u32 * 64 / 255) as u8),
            "Bereich 64..128"
        );
        e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 0;
        assert_eq!(f.get(0, 1), Some(191), "Pan voll -> Pan max");
        assert_eq!(f.get(0, 2), Some(255), "Pan fein am oberen Ende");
        assert_eq!(f.get(0, 3), Some(255), "Tilt 0 invertiert -> voll");
        // Vertauschen
        e.show_mut().fixture_mut(mh).unwrap().limits = FixtureLimits {
            swap_pan_tilt: true,
            ..Default::default()
        };
        let f = e.render_frame();
        assert_eq!(f.get(0, 1), Some(0), "Pan bekommt Tilt-Wert");
        assert_eq!(f.get(0, 3), Some(255), "Tilt bekommt Pan-Wert");
    }

    #[test]
    fn map16_ranges() {
        assert_eq!(FixtureLimits::map16(0, 0, 255, false), 0);
        assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, false), u16::MAX);
        assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, true), 0);
        assert_eq!(FixtureLimits::map16(0, 64, 191, false) >> 8, 64);
    }

    #[test]
    fn speed_multiplies_time() {
        let (mut s, a, _) = setup();
        let mut st = Step::new(vec![cv(a, 0, 200)]);
        st.fade_ms = 1000;
        let id = s.add_scene("S", None, vec![st]);
        let mut e = Engine::new(s);
        e.set_speed(2.0);
        e.activate(id);
        e.tick(MS(250));
        assert_eq!(e.render_frame().get(0, 1), Some(100));
    }

    #[test]
    fn sync_drops_removed_scenes_and_fixtures() {
        let (mut s, a, _) = setup();
        let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 1)])]);
        let mut e = Engine::new(s);
        e.activate(id);
        e.set_programmer(
            ChannelRef {
                fixture: a,
                channel: 0,
            },
            5,
        );
        e.show_mut().remove_scene(id);
        e.show_mut().remove_fixture(a);
        e.sync();
        assert!(e.active_scenes().is_empty());
        assert!(e.programmer().is_empty());
    }
}
