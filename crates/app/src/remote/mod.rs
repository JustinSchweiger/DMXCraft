//! Touch-Fernbedienung im lokalen Netzwerk: ein kleiner Webserver liefert
//! eine Web-App (iOS, iPadOS, Android – im Browser, auf Wunsch auf dem
//! Home-Bildschirm), die die Seiten des Touch-Modus zeigt und bedient.
//!
//! * Der Server läuft nur, wenn er in den Einstellungen eingeschaltet ist
//!   (lauscht dann auf allen Netzwerkschnittstellen), optional mit PIN.
//! * Die Web-App fragt Layout und Zustände ab (`/api/layout`, `/api/state`)
//!   und schickt Bedienbefehle (`/api/cmd`). Die Befehle landen hier in einer
//!   Warteschlange und werden im Oberflächen-Thread genauso ausgeführt wie
//!   die Bedienung im Touch-Modus (Zuordnungen, Live-Werte).

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use dmxcraft_core::touch::{TouchItem, TouchKind, TouchLayout};
use dmxcraft_core::Engine;
use eframe::egui;
use serde::Deserialize;
use serde_json::{json, Value};
use tiny_http::{Header, Method, Request, Response, Server};

use crate::settings::Settings;
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::icon_library;
use crate::views::touch::{
    current_color, current_position, display_label, item_value, mapping_of, set_color, set_position,
};

/// Standard-Port der Fernbedienung.
pub const DEFAULT_PORT: u16 = 8420;

const INDEX_HTML: &str = include_str!("web/index.html");
const APP_JS: &str = include_str!("web/app.js");
const MANIFEST: &str = include_str!("web/manifest.webmanifest");
const FONT_TEXT: &[u8] = include_bytes!("../../assets/fonts/NotoSans-SemiBold.ttf");
const FONT_ICONS: &[u8] = include_bytes!("../../assets/fonts/Phosphor.ttf");

/// Ein Gerät gilt als verbunden, solange es sich so oft meldet.
const CLIENT_TIMEOUT: Duration = Duration::from_secs(4);

/// Zustand für die Anzeige in den Einstellungen.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum RemoteStatus {
    #[default]
    Off,
    Running(u16),
    Error(String),
}

/// Bedienbefehl der Web-App.
#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "t", rename_all = "snake_case")]
pub enum Cmd {
    /// Schaltfläche gedrückt / losgelassen.
    Press { item: u32, down: bool },
    /// Fader / Drehregler (0–1).
    Value { item: u32, v: f32 },
    /// Farbrad.
    Color { item: u32, rgb: [u8; 3] },
    /// XY-Feld (0–1).
    Xy { item: u32, x: f32, y: f32 },
}

/// Mit dem Server-Thread geteilt.
#[derive(Default)]
struct Shared {
    layout: Arc<String>,
    layout_rev: u64,
    state: Arc<String>,
    images: HashMap<u32, Arc<[u8]>>,
    inbox: Vec<Cmd>,
    clients: HashMap<IpAddr, Instant>,
    pin: String,
    ctx: Option<egui::Context>,
}

struct Running {
    server: Arc<Server>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Drop for Running {
    fn drop(&mut self) {
        self.server.unblock();
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

pub struct RemoteLink {
    shared: Arc<Mutex<Shared>>,
    running: Option<Running>,
    /// Zuletzt angewendete Einstellung (Port, wenn eingeschaltet).
    wanted: Option<u16>,
    pub status: RemoteStatus,
    /// Verbundene Geräte (für die Anzeige).
    pub clients: usize,
    /// Adressen der Web-App (beim Start ermittelt).
    pub addresses: Vec<String>,
    last_layout: Option<String>,
    last_layout_at: Option<Instant>,
    last_state_at: Option<Instant>,
}

impl Default for RemoteLink {
    fn default() -> Self {
        Self {
            shared: Arc::default(),
            running: None,
            wanted: None,
            status: RemoteStatus::Off,
            clients: 0,
            addresses: Vec::new(),
            last_layout: None,
            last_layout_at: None,
            last_state_at: None,
        }
    }
}

impl RemoteLink {
    /// Einmal pro Frame: Server starten/stoppen, Befehle ausführen, Layout
    /// und Zustände für die Web-App bereitstellen.
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        e: &mut Engine,
        st: &mut AppState,
        settings: &Settings,
    ) {
        let wanted = settings.remote_enabled.then_some(settings.remote_port);
        if wanted != self.wanted {
            self.wanted = wanted;
            self.running = None;
            self.last_layout = None;
            self.status = match wanted {
                None => RemoteStatus::Off,
                Some(port) => match self.start(ctx, port) {
                    Ok(()) => {
                        self.addresses = addresses(port);
                        RemoteStatus::Running(port)
                    }
                    Err(err) => RemoteStatus::Error(err),
                },
            };
        }
        if self.running.is_none() {
            self.clients = 0;
            return;
        }
        let inbox = {
            let mut sh = self.shared.lock().unwrap_or_else(|p| p.into_inner());
            sh.pin = settings.remote_pin.trim().to_string();
            sh.clients.retain(|_, t| t.elapsed() < CLIENT_TIMEOUT);
            self.clients = sh.clients.len();
            std::mem::take(&mut sh.inbox)
        };
        for cmd in inbox {
            apply(e, st, &cmd);
        }
        if self.clients == 0 {
            return;
        }
        // Layout höchstens zweimal pro Sekunde neu aufbauen (Namen, Farben,
        // Elemente können sich ändern), Zustände etwa 15-mal pro Sekunde
        let now = Instant::now();
        if self
            .last_layout_at
            .is_none_or(|t| now - t > Duration::from_millis(500))
        {
            self.last_layout_at = Some(now);
            let layout = layout_json(e, settings);
            if self.last_layout.as_ref() != Some(&layout) {
                let mut sh = self.shared.lock().unwrap_or_else(|p| p.into_inner());
                sh.layout_rev += 1;
                sh.layout = Arc::new(layout.clone());
                sh.images = e
                    .show()
                    .touch
                    .images
                    .iter()
                    .map(|i| (i.id, i.data.0.clone()))
                    .collect();
                self.last_layout = Some(layout);
            }
        }
        if self
            .last_state_at
            .is_none_or(|t| now - t > Duration::from_millis(60))
        {
            self.last_state_at = Some(now);
            let states = crate::mapping::ui_states(ctx);
            let rev = self
                .shared
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .layout_rev;
            let state = state_json(e, st, &states, rev).to_string();
            self.shared.lock().unwrap_or_else(|p| p.into_inner()).state = Arc::new(state);
        }
        // solange Geräte verbunden sind, regelmäßig aktualisieren
        ctx.request_repaint_after(Duration::from_millis(60));
    }

    fn start(&mut self, ctx: &egui::Context, port: u16) -> Result<(), String> {
        let server = Arc::new(Server::http(("0.0.0.0", port)).map_err(|e| e.to_string())?);
        self.shared.lock().unwrap_or_else(|p| p.into_inner()).ctx = Some(ctx.clone());
        let srv = server.clone();
        let shared = self.shared.clone();
        let thread = std::thread::Builder::new()
            .name("dmxcraft-remote".into())
            .spawn(move || {
                for req in srv.incoming_requests() {
                    serve(&shared, req);
                }
            })
            .map_err(|e| e.to_string())?;
        self.running = Some(Running {
            server,
            thread: Some(thread),
        });
        Ok(())
    }

    /// Erneut versuchen (z. B. nach „Port belegt“).
    pub fn retry(&mut self) {
        self.wanted = None;
        self.running = None;
    }
}

/// Befehl ausführen – wie die Bedienung im Touch-Modus.
fn apply(e: &mut Engine, st: &mut AppState, cmd: &Cmd) {
    let id = match cmd {
        Cmd::Press { item, .. }
        | Cmd::Value { item, .. }
        | Cmd::Color { item, .. }
        | Cmd::Xy { item, .. } => *item,
    };
    let Some(item) = find_item(&e.show().touch, id).cloned() else {
        return;
    };
    match (cmd, item.kind) {
        (Cmd::Press { down, .. }, TouchKind::Button) => {
            if let Some(m) = mapping_of(&item) {
                st.touch_fire
                    .push((m, *down, if *down { 127 } else { 0 }, false));
            }
        }
        (Cmd::Value { v, .. }, TouchKind::Fader | TouchKind::Knob) => {
            let v = v.clamp(0.0, 1.0);
            st.touch_values.insert(item.id, v);
            if let Some(m) = mapping_of(&item) {
                let raw = (v * 127.0).round() as u8;
                st.touch_fire.push((m, raw > 0, raw, true));
            }
        }
        (Cmd::Color { rgb, .. }, TouchKind::Color) => set_color(e, &item.fixtures, *rgb),
        (Cmd::Xy { x, y, .. }, TouchKind::Xy) => {
            set_position(e, &item.fixtures, x.clamp(0.0, 1.0), y.clamp(0.0, 1.0));
        }
        _ => {}
    }
}

fn find_item(t: &TouchLayout, id: u32) -> Option<&TouchItem> {
    t.pages.iter().flat_map(|p| &p.items).find(|i| i.id == id)
}

fn hex(c: egui::Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r(), c.g(), c.b())
}

fn rgb_hex(c: [u8; 3]) -> String {
    format!("#{:02x}{:02x}{:02x}", c[0], c[1], c[2])
}

/// Seiten und Elemente für die Web-App (ohne Bilddaten).
fn layout_json(e: &Engine, settings: &Settings) -> String {
    let show = e.show();
    let t = &show.touch;
    let pages: Vec<Value> = t
        .pages
        .iter()
        .map(|p| {
            let items: Vec<Value> = p
                .items
                .iter()
                .map(|i| {
                    json!({
                        "id": i.id,
                        "kind": i.kind,
                        "x": i.x, "y": i.y, "w": i.w, "h": i.h,
                        "label": display_label(show, i),
                        "own_label": !i.label.is_empty(),
                        "color": i.color.map(rgb_hex),
                        "text_color": i.text_color.map(rgb_hex),
                        "image": i.image,
                        "icon": i.icon.as_deref().and_then(icon_library::glyph).map(String::from),
                        "active": i.control.is_some() || !i.fixtures.is_empty(),
                        "text_size": i.text_size,
                    })
                })
                .collect();
            json!({
                "id": p.id,
                "name": p.name,
                "background": p.background,
                "background_color": p.background_color.map(rgb_hex),
                "items": items,
            })
        })
        .collect();
    json!({
        "name": settings.instance_name,
        "show": show.name,
        "theme": {
            "bg": hex(theme::bg()),
            "surface": hex(theme::surface()),
            "raised": hex(theme::raised()),
            "raised_hover": hex(theme::raised_hover()),
            "border": hex(theme::border()),
            "accent": hex(theme::accent()),
            "text": hex(theme::text()),
            "text_dim": hex(theme::text_dim()),
            "danger": hex(theme::DANGER),
            "light": theme::is_light(),
        },
        "pages": pages,
    })
    .to_string()
}

/// Zustände aller Elemente: Werte, Ein/Aus, Farbe, Position.
fn state_json(e: &Engine, st: &AppState, states: &HashMap<u64, bool>, layout_rev: u64) -> Value {
    let mut values = serde_json::Map::new();
    let mut on = Vec::new();
    let mut colors = serde_json::Map::new();
    let mut xy = serde_json::Map::new();
    for i in e.show().touch.pages.iter().flat_map(|p| &p.items) {
        let key = i.id.to_string();
        match i.kind {
            TouchKind::Fader | TouchKind::Knob => {
                let v = item_value(e, st, i);
                values.insert(key, json!((f64::from(v) * 1000.0).round() / 1000.0));
            }
            TouchKind::Button => {
                if mapping_of(i).is_some_and(|m| crate::mapping::is_on(e, st, states, &m)) {
                    on.push(i.id);
                }
            }
            TouchKind::Color => {
                if let Some(c) = i.fixtures.iter().find_map(|f| current_color(e, *f)) {
                    colors.insert(key, json!(c));
                }
            }
            TouchKind::Xy => {
                if let Some((x, y)) = i.fixtures.iter().find_map(|f| current_position(e, *f)) {
                    xy.insert(key, json!([x, y]));
                }
            }
            TouchKind::Label => {}
        }
    }
    json!({
        "layout": layout_rev,
        "values": values,
        "on": on,
        "colors": colors,
        "xy": xy,
        "blackout": e.blackout(),
    })
}

// ------------------------------------------------------------ HTTP

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).expect("gültiger Header")
}

fn send(req: Request, status: u16, ctype: &str, body: Vec<u8>, cache: bool) {
    let mut r = Response::from_data(body)
        .with_status_code(status)
        .with_header(header("Content-Type", ctype));
    r.add_header(header(
        "Cache-Control",
        if cache {
            "public, max-age=86400"
        } else {
            "no-store"
        },
    ));
    let _ = req.respond(r);
}

fn query_param<'a>(url: &'a str, key: &str) -> Option<&'a str> {
    url.split_once('?')?
        .1
        .split('&')
        .filter_map(|kv| kv.split_once('='))
        .find(|(k, _)| *k == key)
        .map(|(_, v)| v)
}

fn pin_ok(req: &Request, pin: &str) -> bool {
    if pin.is_empty() {
        return true;
    }
    let from_header = req
        .headers()
        .iter()
        .find(|h| h.field.equiv("X-Pin"))
        .map(|h| h.value.as_str().to_string());
    let from_query = query_param(req.url(), "pin").map(String::from);
    from_header.or(from_query).is_some_and(|p| p == pin)
}

fn image_type(data: &[u8]) -> &'static str {
    if data.starts_with(&[0x89, b'P', b'N', b'G']) {
        "image/png"
    } else if data.starts_with(&[0xff, 0xd8]) {
        "image/jpeg"
    } else {
        "application/octet-stream"
    }
}

fn serve(shared: &Arc<Mutex<Shared>>, mut req: Request) {
    let path = req.url().split('?').next().unwrap_or("/").to_string();
    let method = req.method().clone();
    // statische Dateien (ohne PIN – enthalten nichts von der Show)
    let fixed: Option<(&str, &[u8], bool)> = match path.as_str() {
        "/" | "/index.html" => Some(("text/html; charset=utf-8", INDEX_HTML.as_bytes(), false)),
        "/app.js" => Some(("text/javascript; charset=utf-8", APP_JS.as_bytes(), false)),
        "/manifest.webmanifest" => Some(("application/manifest+json", MANIFEST.as_bytes(), true)),
        "/icon.png" => Some(("image/png", crate::icon::ICON_PNG, true)),
        "/icon-512.png" => Some((
            "image/png",
            include_bytes!("../../assets/icon-512.png").as_slice(),
            true,
        )),
        "/fonts/text.ttf" => Some(("font/ttf", FONT_TEXT, true)),
        "/fonts/icons.ttf" => Some(("font/ttf", FONT_ICONS, true)),
        _ => None,
    };
    if let Some((ctype, body, cache)) = fixed {
        send(req, 200, ctype, body.to_vec(), cache);
        return;
    }
    let (pin, ip) = {
        let sh = shared.lock().unwrap_or_else(|p| p.into_inner());
        (sh.pin.clone(), req.remote_addr().map(|a| a.ip()))
    };
    if !pin_ok(&req, &pin) {
        send(
            req,
            401,
            "application/json",
            br#"{"error":"pin"}"#.to_vec(),
            false,
        );
        return;
    }
    match (method, path.as_str()) {
        (Method::Get, "/api/layout") => {
            let (layout, rev) = {
                let mut sh = shared.lock().unwrap_or_else(|p| p.into_inner());
                if let Some(ip) = ip {
                    sh.clients.insert(ip, Instant::now());
                }
                wake(&sh);
                (sh.layout.clone(), sh.layout_rev)
            };
            // Rev mitschicken, damit die Web-App weiß, welchen Stand sie hat
            let body = if layout.is_empty() {
                format!(r#"{{"rev":{rev},"loading":true}}"#)
            } else {
                format!(r#"{{"rev":{rev},"layout":{layout}}}"#)
            };
            send(req, 200, "application/json", body.into_bytes(), false);
        }
        (Method::Get, "/api/state") => {
            let state = {
                let mut sh = shared.lock().unwrap_or_else(|p| p.into_inner());
                if let Some(ip) = ip {
                    sh.clients.insert(ip, Instant::now());
                }
                wake(&sh);
                sh.state.clone()
            };
            let body = if state.is_empty() {
                r#"{"layout":0}"#.to_string()
            } else {
                state.to_string()
            };
            send(req, 200, "application/json", body.into_bytes(), false);
        }
        (Method::Post, "/api/cmd") => {
            let mut body = String::new();
            if std::io::Read::read_to_string(req.as_reader(), &mut body).is_err()
                || body.len() > 64 * 1024
            {
                send(req, 400, "text/plain", b"bad request".to_vec(), false);
                return;
            }
            let cmds: Vec<Cmd> = serde_json::from_str::<Vec<Cmd>>(&body)
                .or_else(|_| serde_json::from_str::<Cmd>(&body).map(|c| vec![c]))
                .unwrap_or_default();
            {
                let mut sh = shared.lock().unwrap_or_else(|p| p.into_inner());
                if let Some(ip) = ip {
                    sh.clients.insert(ip, Instant::now());
                }
                sh.inbox.extend(cmds);
                wake(&sh);
            }
            send(req, 204, "text/plain", Vec::new(), false);
        }
        (Method::Get, p) if p.starts_with("/img/") => {
            let id: Option<u32> = p["/img/".len()..].parse().ok();
            let data = id.and_then(|id| {
                shared
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .images
                    .get(&id)
                    .cloned()
            });
            match data {
                Some(d) => send(req, 200, image_type(&d), d.to_vec(), true),
                None => send(req, 404, "text/plain", b"not found".to_vec(), false),
            }
        }
        _ => send(req, 404, "text/plain", b"not found".to_vec(), false),
    }
}

/// Oberfläche aufwecken, damit Befehle sofort ausgeführt werden.
fn wake(sh: &Shared) {
    if let Some(ctx) = &sh.ctx {
        ctx.request_repaint();
    }
}

/// Adressen, unter denen die Fernbedienung erreichbar ist (IP im lokalen
/// Netz und Rechnername mit `.local`).
pub fn addresses(port: u16) -> Vec<String> {
    let mut out = Vec::new();
    // Trick ohne Netzverkehr: ausgehende Schnittstelle bestimmen
    if let Ok(sock) = std::net::UdpSocket::bind(("0.0.0.0", 0)) {
        if sock.connect(("8.8.8.8", 80)).is_ok() {
            if let Ok(addr) = sock.local_addr() {
                if !addr.ip().is_loopback() && !addr.ip().is_unspecified() {
                    out.push(format!("http://{}:{port}", addr.ip()));
                }
            }
        }
    }
    if let Some(host) = hostname() {
        out.push(format!("http://{}.local:{port}", host.to_lowercase()));
    }
    out
}

fn hostname() -> Option<String> {
    ["COMPUTERNAME", "HOSTNAME"]
        .iter()
        .find_map(|v| std::env::var(v).ok())
        .or_else(|| {
            std::fs::read_to_string("/etc/hostname")
                .ok()
                .map(|s| s.trim().to_string())
        })
        .filter(|h| !h.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_parse() {
        let c: Vec<Cmd> = serde_json::from_str(
            r#"[{"t":"press","item":3,"down":true},{"t":"value","item":4,"v":0.5},
               {"t":"color","item":5,"rgb":[255,0,10]},{"t":"xy","item":6,"x":0.1,"y":0.9}]"#,
        )
        .unwrap();
        assert_eq!(
            c[0],
            Cmd::Press {
                item: 3,
                down: true
            }
        );
        assert_eq!(c[1], Cmd::Value { item: 4, v: 0.5 });
        assert_eq!(
            c[2],
            Cmd::Color {
                item: 5,
                rgb: [255, 0, 10]
            }
        );
        assert_eq!(
            c[3],
            Cmd::Xy {
                item: 6,
                x: 0.1,
                y: 0.9
            }
        );
    }

    #[test]
    fn query_and_image_type() {
        assert_eq!(query_param("/img/3?pin=1234&x=1", "pin"), Some("1234"));
        assert_eq!(query_param("/img/3", "pin"), None);
        assert_eq!(image_type(&[0x89, b'P', b'N', b'G', 0]), "image/png");
        assert_eq!(image_type(&[0xff, 0xd8, 0]), "image/jpeg");
    }

    /// Server im Prozess starten und die Web-App/API abfragen.
    #[test]
    fn serves_app_and_api() {
        let shared: Arc<Mutex<Shared>> = Arc::default();
        {
            let mut sh = shared.lock().unwrap();
            sh.layout = Arc::new(r#"{"pages":[]}"#.into());
            sh.layout_rev = 7;
            sh.pin = "42".into();
        }
        let server = Arc::new(Server::http(("127.0.0.1", 0)).unwrap());
        let port = server.server_addr().to_ip().unwrap().port();
        let (srv, sh2) = (server.clone(), shared.clone());
        let t = std::thread::spawn(move || {
            for req in srv.incoming_requests() {
                serve(&sh2, req);
            }
        });
        let get = |path: &str, extra: &str| -> String {
            use std::io::{Read, Write};
            let mut s = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
            write!(
                s,
                "GET {path} HTTP/1.1\r\nHost: x\r\n{extra}Connection: close\r\n\r\n"
            )
            .unwrap();
            let mut out = String::new();
            s.read_to_string(&mut out).unwrap();
            out
        };
        assert!(get("/", "").contains("<html"));
        assert!(get("/api/layout", "").starts_with("HTTP/1.1 401"));
        let ok = get("/api/layout", "X-Pin: 42\r\n");
        assert!(ok.contains(r#""rev":7"#), "{ok}");
        // Befehl landet in der Warteschlange
        {
            use std::io::{Read, Write};
            let body = r#"{"t":"press","item":1,"down":true}"#;
            let mut s = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
            write!(
                s,
                "POST /api/cmd?pin=42 HTTP/1.1\r\nHost: x\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            )
            .unwrap();
            let mut out = String::new();
            s.read_to_string(&mut out).unwrap();
            assert!(out.starts_with("HTTP/1.1 204"), "{out}");
        }
        assert_eq!(
            shared.lock().unwrap().inbox,
            vec![Cmd::Press {
                item: 1,
                down: true
            }]
        );
        server.unblock();
        t.join().unwrap();
    }
}
