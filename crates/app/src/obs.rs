//! OBS-Anbindung über obs-websocket 5 (in OBS ab Version 28 eingebaut).
//!
//! Ein Hintergrund-Thread hält die Verbindung (mit Anmeldung und
//! automatischem Neuverbinden) und fragt regelmäßig Szenen und Quellen ab.
//! [`Obs::update`] gleicht jedes Frame die laufenden SUPER SCENES ab:
//! Videoblöcke spielt OBS in einer Medienquelle ab (Start, Sprung, Pause,
//! Geschwindigkeit, Nachstellen bei Abweichung), OBS-Blöcke lösen beim
//! Erreichen ihre Aktion aus.

use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use dmxcraft_core::obs::{ObsAction, ObsSettings};
use dmxcraft_core::timeline::ClipContent;
use dmxcraft_core::{Engine, SceneId};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tungstenite::Message;

/// Verbindungszustand.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Status {
    #[default]
    Off,
    Connecting,
    Connected,
    Failed(String),
}

/// Vom Verbindungs-Thread gepflegter Stand von OBS.
#[derive(Default)]
struct Shared {
    status: Status,
    /// Szenen (oben zuerst, wie in OBS angezeigt).
    scenes: Vec<String>,
    inputs: Vec<String>,
    program: String,
    /// Medienstatus je Quelle: Position (ms), läuft, Zeitpunkt der Antwort.
    media: HashMap<String, (f64, bool, Instant)>,
    /// Letzte Fehlermeldung von OBS zu einer Anfrage.
    last_error: Option<String>,
}

/// Laufende Verbindung (Thread + Kanal).
struct Link {
    cfg: ObsSettings,
    tx: Sender<(String, Value)>,
    stop: Arc<AtomicBool>,
}

impl Drop for Link {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// Ein Video, das gerade in OBS läuft.
struct VideoPlay {
    source: String,
    playing: bool,
    rate: f32,
    volume: f32,
    /// Erwartete Position (ms) beim letzten Frame.
    pos: f64,
    /// Letzte Abfrage des Medienstatus und ob ihre Antwort ausgewertet ist.
    asked: Instant,
    checked: bool,
}

#[derive(Default)]
pub struct Obs {
    /// Fenster „OBS“ geöffnet.
    pub open: bool,
    link: Option<Link>,
    shared: Arc<Mutex<Shared>>,
    videos: HashMap<(SceneId, u32), VideoPlay>,
    actions: HashSet<(SceneId, u32)>,
    last: Option<Instant>,
}

impl Obs {
    pub fn status(&self) -> Status {
        if self.link.is_none() {
            return Status::Off;
        }
        self.lock().status.clone()
    }

    pub fn scenes(&self) -> Vec<String> {
        self.lock().scenes.clone()
    }

    pub fn inputs(&self) -> Vec<String> {
        self.lock().inputs.clone()
    }

    pub fn last_error(&self) -> Option<String> {
        self.lock().last_error.clone()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Shared> {
        self.shared.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn send(&self, kind: &str, data: Value) {
        if let Some(l) = &self.link {
            let _ = l.tx.send((kind.to_string(), data));
        }
    }

    /// Verbindung nach den Einstellungen der Show auf- bzw. abbauen und die
    /// laufenden SUPER SCENES mit OBS abgleichen.
    pub fn update(&mut self, e: &Engine, scrubbing: bool, ctx: &eframe::egui::Context) {
        let cfg = &e.show().obs;
        if !cfg.enabled {
            if self.link.take().is_some() {
                self.videos.clear();
                self.actions.clear();
                *self.lock() = Shared::default();
            }
            return;
        }
        if self.link.as_ref().is_none_or(|l| l.cfg != *cfg) {
            self.connect(cfg.clone(), ctx);
        }
        let dt = self
            .last
            .map_or(0.0, |t| t.elapsed().as_secs_f64() * 1000.0);
        self.last = Some(Instant::now());
        let connected = self.status() == Status::Connected;

        // gewünschter Zustand aus den laufenden SUPER SCENES
        struct Want {
            path: String,
            source: String,
            obs_scene: String,
            pos: f64,
            rate: f32,
            volume: f32,
        }
        let mut videos: HashMap<(SceneId, u32), Want> = HashMap::new();
        let mut actions: Vec<((SceneId, u32), ObsAction, bool)> = Vec::new();
        for scene in &e.show().scenes {
            let (Some(tl), Some(t)) = (scene.timeline(), e.timeline_time(scene.id)) else {
                continue;
            };
            let running = e.timeline_running(scene.id) && !scrubbing;
            let rate = if running {
                e.speed() * f32::from(scene.settings.speed_pct) / 100.0
            } else {
                0.0
            };
            let now = t as u64;
            for (ti, clip) in tl.active_at(now) {
                if tl.tracks.get(ti).is_some_and(|tr| tr.muted) {
                    continue;
                }
                match &clip.content {
                    ClipContent::Video {
                        path,
                        offset_ms,
                        obs_scene,
                        source,
                        ..
                    } => {
                        let source = if source.is_empty() {
                            cfg.video_source.clone()
                        } else {
                            source.clone()
                        };
                        videos.insert(
                            (scene.id, clip.id),
                            Want {
                                path: path.clone(),
                                source,
                                obs_scene: obs_scene.clone(),
                                pos: (offset_ms + (now - clip.start_ms)) as f64,
                                rate,
                                volume: (clip.level * clip.fade_at(now)).clamp(0.0, 1.0),
                            },
                        );
                    }
                    ClipContent::Obs(a) => {
                        actions.push(((scene.id, clip.id), a.clone(), rate > 0.0));
                    }
                    _ => {}
                }
            }
        }

        if !connected {
            // nach dem Verbinden nur noch Neues auslösen, Videos neu starten
            self.videos.clear();
            self.actions = actions.iter().map(|(k, _, _)| *k).collect();
            return;
        }

        let mut out: Vec<(&str, Value)> = Vec::new();
        // OBS-Blöcke: beim Erreichen (nur bei laufender Wiedergabe)
        let current: HashSet<(SceneId, u32)> = actions.iter().map(|(k, _, _)| *k).collect();
        for (key, action, playing) in &actions {
            if !self.actions.contains(key) && *playing {
                fire(&mut out, action);
            }
        }
        self.actions = current;

        // beendete Videos stoppen
        let gone: Vec<(SceneId, u32)> = self
            .videos
            .keys()
            .filter(|k| !videos.contains_key(k))
            .copied()
            .collect();
        for key in gone {
            if let Some(v) = self.videos.remove(&key) {
                if !videos.values().any(|w| w.source == v.source) {
                    media_action(&mut out, &v.source, "STOP");
                }
            }
        }

        let (inputs, program, media) = {
            let s = self.lock();
            (s.inputs.clone(), s.program.clone(), s.media.clone())
        };
        for (key, w) in videos {
            let playing = w.rate > 0.0;
            match self.videos.get_mut(&key) {
                None => {
                    if !w.obs_scene.is_empty() {
                        out.push((
                            "SetCurrentProgramScene",
                            json!({ "sceneName": w.obs_scene }),
                        ));
                    }
                    let settings = media_settings(&w.path, w.rate);
                    if inputs.contains(&w.source) {
                        out.push((
                            "SetInputSettings",
                            json!({ "inputName": w.source, "inputSettings": settings, "overlay": true }),
                        ));
                    } else {
                        // Medienquelle fehlt: in der Szene des Blocks anlegen
                        let scene = if w.obs_scene.is_empty() {
                            program.clone()
                        } else {
                            w.obs_scene.clone()
                        };
                        out.push((
                            "CreateInput",
                            json!({
                                "sceneName": scene,
                                "inputName": w.source,
                                "inputKind": "ffmpeg_source",
                                "inputSettings": settings,
                                "sceneItemEnabled": true,
                            }),
                        ));
                        out.push(("GetInputList", json!({})));
                    }
                    out.push((
                        "SetInputVolume",
                        json!({ "inputName": w.source, "inputVolumeMul": w.volume }),
                    ));
                    media_action(&mut out, &w.source, "RESTART");
                    if w.pos > 100.0 {
                        seek(&mut out, &w.source, w.pos);
                    }
                    if !playing {
                        media_action(&mut out, &w.source, "PAUSE");
                    }
                    self.videos.insert(
                        key,
                        VideoPlay {
                            source: w.source,
                            playing,
                            rate: w.rate.max(0.01),
                            volume: w.volume,
                            pos: w.pos,
                            asked: Instant::now(),
                            checked: true,
                        },
                    );
                }
                Some(v) => {
                    let expected = v.pos + if v.playing { dt * v.rate as f64 } else { 0.0 };
                    let jumped = (w.pos - expected).abs() > 300.0;
                    if playing && (w.rate - v.rate).abs() > 0.01 {
                        v.rate = w.rate;
                        out.push((
                            "SetInputSettings",
                            json!({
                                "inputName": v.source,
                                "inputSettings": { "speed_percent": speed_pct(w.rate) },
                                "overlay": true,
                            }),
                        ));
                        seek(&mut out, &v.source, w.pos);
                    } else if jumped {
                        seek(&mut out, &v.source, w.pos);
                    }
                    if playing != v.playing {
                        v.playing = playing;
                        if !playing {
                            media_action(&mut out, &v.source, "PAUSE");
                        } else {
                            seek(&mut out, &v.source, w.pos);
                            media_action(&mut out, &v.source, "PLAY");
                        }
                    }
                    if (w.volume - v.volume).abs() > 0.02 {
                        v.volume = w.volume;
                        out.push((
                            "SetInputVolume",
                            json!({ "inputName": v.source, "inputVolumeMul": w.volume }),
                        ));
                    }
                    // Abweichung prüfen: Status abfragen, Antwort beim
                    // nächsten Mal mit der erwarteten Stelle vergleichen
                    if playing {
                        if let Some((cursor, true, at)) = media.get(&v.source).copied() {
                            if at > v.asked && !v.checked {
                                v.checked = true;
                                let now_cursor =
                                    cursor + at.elapsed().as_secs_f64() * 1000.0 * v.rate as f64;
                                if (now_cursor - w.pos).abs() > 350.0 {
                                    seek(&mut out, &v.source, w.pos);
                                }
                            }
                        }
                        if v.asked.elapsed() > Duration::from_millis(1500) {
                            v.asked = Instant::now();
                            v.checked = false;
                            out.push(("GetMediaInputStatus", json!({ "inputName": v.source })));
                        }
                    }
                    v.pos = w.pos;
                }
            }
        }
        for (kind, data) in out {
            self.send(kind, data);
        }
    }

    fn connect(&mut self, cfg: ObsSettings, ctx: &eframe::egui::Context) {
        self.videos.clear();
        self.link = None;
        let shared = Arc::new(Mutex::new(Shared {
            status: Status::Connecting,
            ..Default::default()
        }));
        self.shared = shared.clone();
        let (tx, rx) = channel();
        let stop = Arc::new(AtomicBool::new(false));
        let st = stop.clone();
        let c = cfg.clone();
        let ctx = ctx.clone();
        std::thread::Builder::new()
            .name("obs".into())
            .spawn(move || run(c, rx, shared, st, ctx))
            .ok();
        self.link = Some(Link { cfg, tx, stop });
    }
}

fn fire(out: &mut Vec<(&str, Value)>, action: &ObsAction) {
    match action {
        ObsAction::Scene(name) if !name.is_empty() => {
            out.push(("SetCurrentProgramScene", json!({ "sceneName": name })));
        }
        ObsAction::Source {
            scene,
            source,
            visible,
        } => {
            // ID des Szenenelements erfragen; der Thread schaltet es dann
            out.push((
                "GetSceneItemId",
                json!({ "sceneName": scene, "sourceName": source, "_enable": visible }),
            ));
        }
        _ => {}
    }
}

fn media_action(out: &mut Vec<(&str, Value)>, source: &str, action: &str) {
    out.push((
        "TriggerMediaInputAction",
        json!({
            "inputName": source,
            "mediaAction": format!("OBS_WEBSOCKET_MEDIA_INPUT_ACTION_{action}"),
        }),
    ));
}

fn seek(out: &mut Vec<(&str, Value)>, source: &str, ms: f64) {
    out.push((
        "SetMediaInputCursor",
        json!({ "inputName": source, "mediaCursor": ms.max(0.0) as u64 }),
    ));
}

/// Fenster „OBS“: Verbindung (wird mit der Show gespeichert) und Status.
pub fn window(ctx: &eframe::egui::Context, cx: &mut crate::views::ViewCtx<'_>) {
    use crate::i18n::t;
    use crate::ui::theme;
    use crate::ui::widgets::{Dialog, Toggle};
    use eframe::egui;
    if !cx.state.obs.open {
        return;
    }
    let mut open = true;
    let mut cfg = cx.engine.show().obs.clone();
    let status = cx.state.obs.status();
    let lists = (cx.state.obs.scenes().len(), cx.state.obs.inputs().len());
    let err = cx.state.obs.last_error();
    let locked = cx.state.locked;
    Dialog::new(t("OBS"))
        .id("obs_window")
        .cancel(t("Schließen"))
        .show(ctx, &mut open, |ui, _| {
            ui.add_enabled_ui(!locked, |ui| {
                egui::Grid::new("obs_grid")
                    .num_columns(2)
                    .spacing([theme::SP_M, theme::SP_S])
                    .show(ui, |ui| {
                        ui.label(t("Mit OBS verbinden"));
                        ui.add(Toggle::new(&mut cfg.enabled, ""));
                        ui.end_row();
                        ui.label(t("Adresse"));
                        ui.add(egui::TextEdit::singleline(&mut cfg.host).desired_width(200.0));
                        ui.end_row();
                        ui.label(t("Port"));
                        ui.add(egui::DragValue::new(&mut cfg.port).range(1..=65535));
                        ui.end_row();
                        ui.label(t("Passwort"));
                        ui.add(
                            egui::TextEdit::singleline(&mut cfg.password)
                                .password(true)
                                .desired_width(200.0),
                        );
                        ui.end_row();
                        ui.label(t("Medienquelle für Videos"));
                        ui.add(
                            egui::TextEdit::singleline(&mut cfg.video_source).desired_width(200.0),
                        );
                        ui.end_row();
                    });
            });
            ui.add_space(theme::SP_S);
            let (text, color) = match &status {
                Status::Off => (t("Nicht verbunden").to_string(), theme::text_faint()),
                Status::Connecting => (t("Verbinde …").to_string(), theme::WARN),
                Status::Connected => (
                    crate::tr!(
                        "Verbunden – {scenes} Szenen, {inputs} Quellen",
                        scenes = lists.0,
                        inputs = lists.1
                    ),
                    theme::OK,
                ),
                Status::Failed(e) => (crate::tr!("Fehler: {e}", e = e), theme::DANGER),
            };
            ui.label(egui::RichText::new(text).color(color));
            if let (Status::Connected, Some(e)) = (&status, err) {
                ui.label(
                    egui::RichText::new(e)
                        .font(theme::caption())
                        .color(theme::text_faint()),
                );
            }
        });
    if cfg != cx.engine.show().obs {
        cx.engine.show_mut().obs = cfg;
    }
    cx.state.obs.open = open;
}

fn speed_pct(rate: f32) -> u32 {
    (rate * 100.0).round().clamp(1.0, 200.0) as u32
}

/// Einstellungen der Medienquelle (ffmpeg_source) für ein Video.
fn media_settings(path: &str, rate: f32) -> Value {
    json!({
        "is_local_file": true,
        "local_file": path,
        "looping": false,
        "restart_on_activate": false,
        "close_when_inactive": false,
        "clear_on_media_end": true,
        "hw_decode": true,
        "speed_percent": speed_pct(if rate > 0.0 { rate } else { 1.0 }),
    })
}

/// Antwort auf die Anmelde-Herausforderung von obs-websocket.
pub fn auth_response(password: &str, salt: &str, challenge: &str) -> String {
    let secret = b64(&Sha256::digest(format!("{password}{salt}").as_bytes()));
    b64(&Sha256::digest(format!("{secret}{challenge}").as_bytes()))
}

fn b64(data: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let n = (u32::from(c[0]) << 16)
            | (u32::from(*c.get(1).unwrap_or(&0)) << 8)
            | u32::from(*c.get(2).unwrap_or(&0));
        for i in 0..4 {
            if i <= c.len() {
                out.push(T[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

type Ws = tungstenite::WebSocket<TcpStream>;

/// Verbindungs-Thread: verbinden, anmelden, Anfragen senden, Antworten
/// auswerten; bei Abbruch nach kurzer Pause neu verbinden.
fn run(
    cfg: ObsSettings,
    rx: Receiver<(String, Value)>,
    shared: Arc<Mutex<Shared>>,
    stop: Arc<AtomicBool>,
    ctx: eframe::egui::Context,
) {
    let set_status = |s: Status| {
        shared.lock().unwrap_or_else(|p| p.into_inner()).status = s;
        ctx.request_repaint();
    };
    while !stop.load(Ordering::Relaxed) {
        set_status(Status::Connecting);
        let err = match open(&cfg) {
            Ok(mut ws) => {
                set_status(Status::Connected);
                session(&mut ws, &rx, &shared, &stop, &ctx)
            }
            Err(e) => e,
        };
        // liegengebliebene Anfragen verwerfen (gelten nach dem Neuverbinden nicht mehr)
        while rx.try_recv().is_ok() {}
        if stop.load(Ordering::Relaxed) {
            break;
        }
        set_status(Status::Failed(err));
        for _ in 0..30 {
            if stop.load(Ordering::Relaxed) {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

/// Verbinden und anmelden (Hello → Identify → Identified).
fn open(cfg: &ObsSettings) -> Result<Ws, String> {
    let addr = (cfg.host.as_str(), cfg.port)
        .to_socket_addrs()
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("Adresse nicht gefunden")?;
    let tcp = TcpStream::connect_timeout(&addr, Duration::from_secs(2))
        .map_err(|_| crate::i18n::t("OBS nicht erreichbar").to_string())?;
    tcp.set_read_timeout(Some(Duration::from_secs(3))).ok();
    tcp.set_nodelay(true).ok();
    let url = format!("ws://{}:{}", cfg.host, cfg.port);
    let (mut ws, _) = tungstenite::client(url, tcp).map_err(|e| e.to_string())?;
    let hello = read_json(&mut ws)?;
    if hello["op"] != 0 {
        return Err("Unerwartete Antwort".into());
    }
    let mut identify = json!({ "rpcVersion": 1, "eventSubscriptions": 0 });
    if let Some(a) = hello["d"].get("authentication") {
        let (Some(ch), Some(salt)) = (a["challenge"].as_str(), a["salt"].as_str()) else {
            return Err("Unerwartete Anmeldung".into());
        };
        identify["authentication"] = json!(auth_response(&cfg.password, salt, ch));
    }
    send_json(&mut ws, &json!({ "op": 1, "d": identify }))?;
    match read_json(&mut ws) {
        Ok(v) if v["op"] == 2 => {}
        Ok(_) => return Err("Unerwartete Antwort".into()),
        Err(e) if e.contains("4009") => {
            return Err(crate::i18n::t("Passwort falsch").to_string());
        }
        Err(e) => return Err(e),
    }
    ws.get_mut()
        .set_read_timeout(Some(Duration::from_millis(30)))
        .ok();
    Ok(ws)
}

fn read_json(ws: &mut Ws) -> Result<Value, String> {
    loop {
        match ws.read() {
            Ok(Message::Text(t)) => {
                return serde_json::from_str(t.as_str()).map_err(|e| e.to_string());
            }
            Ok(Message::Close(f)) => {
                return Err(f.map_or("Verbindung geschlossen".into(), |f| {
                    format!("{} {}", u16::from(f.code), f.reason)
                }));
            }
            Ok(_) => continue,
            Err(e) => return Err(e.to_string()),
        }
    }
}

fn send_json(ws: &mut Ws, v: &Value) -> Result<(), String> {
    ws.send(Message::text(v.to_string()))
        .map_err(|e| e.to_string())
}

/// Angemeldete Verbindung bedienen, bis sie abbricht (liefert den Grund).
fn session(
    ws: &mut Ws,
    rx: &Receiver<(String, Value)>,
    shared: &Arc<Mutex<Shared>>,
    stop: &Arc<AtomicBool>,
    ctx: &eframe::egui::Context,
) -> String {
    let mut next_id = 0u64;
    // offene Anfragen: ID → (Typ, Daten)
    let mut pending: HashMap<String, (String, Value)> = HashMap::new();
    let mut last_poll = Instant::now() - Duration::from_secs(10);
    let lock = || shared.lock().unwrap_or_else(|p| p.into_inner());
    loop {
        if stop.load(Ordering::Relaxed) {
            let _ = ws.close(None);
            return String::new();
        }
        let mut out: Vec<(String, Value)> = rx.try_iter().collect();
        if last_poll.elapsed() > Duration::from_secs(3) {
            last_poll = Instant::now();
            out.push(("GetSceneList".into(), json!({})));
            out.push(("GetInputList".into(), json!({})));
        }
        for (kind, mut data) in out {
            next_id += 1;
            let id = next_id.to_string();
            // interne Felder (mit „_“) nicht an OBS schicken
            let extra = data.as_object_mut().map(|o| {
                let keys: Vec<String> = o.keys().filter(|k| k.starts_with('_')).cloned().collect();
                keys.into_iter()
                    .filter_map(|k| o.remove(&k).map(|v| (k, v)))
                    .collect::<serde_json::Map<String, Value>>()
            });
            let msg = json!({ "op": 6, "d": { "requestType": kind, "requestId": id, "requestData": data } });
            if let Err(e) = send_json(ws, &msg) {
                return e;
            }
            let mut keep = data;
            if let (Some(o), Some(extra)) = (keep.as_object_mut(), extra) {
                o.extend(extra);
            }
            pending.insert(id, (kind, keep));
        }
        match ws.read() {
            Ok(Message::Text(t)) => {
                let Ok(v) = serde_json::from_str::<Value>(t.as_str()) else {
                    continue;
                };
                if v["op"] != 7 {
                    continue;
                }
                let d = &v["d"];
                let Some((kind, req)) = d["requestId"].as_str().and_then(|id| pending.remove(id))
                else {
                    continue;
                };
                let ok = d["requestStatus"]["result"].as_bool().unwrap_or(false);
                let r = &d["responseData"];
                if !ok {
                    let comment = d["requestStatus"]["comment"].as_str().unwrap_or("");
                    lock().last_error = Some(format!("{kind}: {comment}"));
                    ctx.request_repaint();
                    continue;
                }
                match kind.as_str() {
                    "GetSceneList" => {
                        let mut s = lock();
                        s.scenes = names(&r["scenes"], "sceneName");
                        s.scenes.reverse();
                        s.program = r["currentProgramSceneName"]
                            .as_str()
                            .unwrap_or("")
                            .to_string();
                        ctx.request_repaint();
                    }
                    "GetInputList" => {
                        lock().inputs = names(&r["inputs"], "inputName");
                    }
                    "GetMediaInputStatus" => {
                        let name = req["inputName"].as_str().unwrap_or("").to_string();
                        let cursor = r["mediaCursor"].as_f64().unwrap_or(0.0);
                        let playing = r["mediaState"] == "OBS_MEDIA_STATE_PLAYING";
                        lock().media.insert(name, (cursor, playing, Instant::now()));
                    }
                    "GetSceneItemId" => {
                        // zweiter Schritt von „Quelle ein-/ausblenden“
                        if let Some(item) = r["sceneItemId"].as_i64() {
                            next_id += 1;
                            let msg = json!({ "op": 6, "d": {
                                "requestType": "SetSceneItemEnabled",
                                "requestId": next_id.to_string(),
                                "requestData": {
                                    "sceneName": req["sceneName"],
                                    "sceneItemId": item,
                                    "sceneItemEnabled": req["_enable"].as_bool().unwrap_or(true),
                                },
                            }});
                            if let Err(e) = send_json(ws, &msg) {
                                return e;
                            }
                            pending.insert(
                                next_id.to_string(),
                                ("SetSceneItemEnabled".into(), Value::Null),
                            );
                        }
                    }
                    _ => {}
                }
            }
            Ok(Message::Close(f)) => {
                return f.map_or("Verbindung geschlossen".into(), |f| f.reason.to_string());
            }
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => return e.to_string(),
        }
    }
}

fn names(list: &Value, key: &str) -> Vec<String> {
    list.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v[key].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dmxcraft_core::{Show, Step};
    use std::net::TcpListener;

    /// Nachgebautes OBS: meldet sich mit Passwort an, beantwortet alle
    /// Anfragen und merkt sich deren Typen.
    fn fake_obs(password: &'static str) -> (u16, Arc<Mutex<Vec<String>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let log = seen.clone();
        std::thread::spawn(move || {
            let (tcp, _) = listener.accept().unwrap();
            let mut ws = tungstenite::accept(tcp).unwrap();
            let (salt, challenge) = ("c2FsdA==", "Y2hhbGxlbmdl");
            let hello = json!({"op":0,"d":{"rpcVersion":1,"authentication":{"salt":salt,"challenge":challenge}}});
            ws.send(Message::text(hello.to_string())).unwrap();
            let ident: Value = match ws.read().unwrap() {
                Message::Text(t) => serde_json::from_str(t.as_str()).unwrap(),
                _ => panic!(),
            };
            if ident["d"]["authentication"] != auth_response(password, salt, challenge) {
                let _ = ws.close(Some(tungstenite::protocol::CloseFrame {
                    code: 4009.into(),
                    reason: "auth".into(),
                }));
                let _ = ws.flush();
                return;
            }
            ws.send(Message::text(
                json!({"op":2,"d":{"negotiatedRpcVersion":1}}).to_string(),
            ))
            .unwrap();
            while let Ok(Message::Text(t)) = ws.read() {
                let v: Value = serde_json::from_str(t.as_str()).unwrap();
                let kind = v["d"]["requestType"].as_str().unwrap().to_string();
                let data = match kind.as_str() {
                    "GetSceneList" => {
                        json!({"currentProgramSceneName":"Live","scenes":[{"sceneName":"Video"},{"sceneName":"Live"}]})
                    }
                    "GetInputList" => json!({"inputs":[{"inputName":"Kamera"}]}),
                    _ => json!({}),
                };
                log.lock().unwrap().push(kind.clone());
                let r = json!({"op":7,"d":{"requestType":kind,"requestId":v["d"]["requestId"],
                    "requestStatus":{"result":true,"code":100},"responseData":data}});
                if ws.send(Message::text(r.to_string())).is_err() {
                    break;
                }
            }
        });
        (port, seen)
    }

    fn engine(port: u16, password: &str) -> (Engine, SceneId) {
        let mut show = Show::new("t", 1);
        show.obs.enabled = true;
        show.obs.host = "127.0.0.1".into();
        show.obs.port = port;
        show.obs.password = password.into();
        let sup = show.add_scene("S", None, vec![Step::new(Vec::new())]);
        let sc = show.scene_mut(sup).unwrap();
        sc.add_effect(dmxcraft_core::EffectKind::SuperScene);
        let tl = sc.timeline_mut().unwrap();
        tl.add_clip(
            0,
            ClipContent::Video {
                path: "C:/clip.mp4".into(),
                offset_ms: 0,
                length_ms: 60_000,
                obs_scene: "Video".into(),
                source: String::new(),
            },
            0,
            60_000,
        );
        sc.sync_timeline_length();
        (Engine::new(show), sup)
    }

    fn wait(obs: &mut Obs, e: &Engine, ctx: &eframe::egui::Context, ok: impl Fn(&Obs) -> bool) {
        for _ in 0..200 {
            obs.update(e, false, ctx);
            if ok(obs) {
                return;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        panic!("Zeitüberschreitung: {:?}", obs.status());
    }

    #[test]
    fn connects_and_plays_video_block() {
        let (port, seen) = fake_obs("geheim");
        let (mut e, sup) = engine(port, "geheim");
        let ctx = eframe::egui::Context::default();
        let mut obs = Obs::default();
        wait(&mut obs, &e, &ctx, |o| o.scenes().len() == 2);
        assert_eq!(obs.scenes(), vec!["Live".to_string(), "Video".to_string()]);
        e.activate(sup);
        e.tick(Duration::from_millis(10));
        let has = |k: &str| seen.lock().unwrap().iter().any(|s| s == k);
        wait(&mut obs, &e, &ctx, |_| has("TriggerMediaInputAction"));
        // Quelle fehlt in OBS: wird in der Szene des Blocks angelegt
        assert!(has("SetCurrentProgramScene"));
        assert!(has("CreateInput"));
    }

    #[test]
    fn wrong_password_is_reported() {
        let (port, _) = fake_obs("geheim");
        let (e, _) = engine(port, "falsch");
        let ctx = eframe::egui::Context::default();
        let mut obs = Obs::default();
        wait(&mut obs, &e, &ctx, |o| {
            matches!(o.status(), Status::Failed(_))
        });
        assert_eq!(
            obs.status(),
            Status::Failed(crate::i18n::t("Passwort falsch").to_string())
        );
    }

    #[test]
    fn auth_matches_obs_websocket_example() {
        // Beispiel aus der Protokollbeschreibung von obs-websocket 5
        let a = auth_response(
            "supersecretpassword",
            "lM1GncleQOaCu9lT1yeUZhFYnqhsLLP1G5lAGo3ixaI=",
            "+IxH4CnCiqpX1rM9scsNynZzbOe4KhDeYcTNS3PDaeY=",
        );
        assert_eq!(a, "1Ct943GAT+6YQUUX47Ia/ncufilbe6+oD6lY+5kaCu4=");
    }
}
