//! Video output for OBS: a small web server on this computer serves a page
//! that OBS shows in a browser source. The page plays the video blocks of the
//! running SUPER SCENES and follows DMXCraft's clock: small deviations are
//! evened out by playing a little faster or slower, only large jumps seek.
//! So picture, sound (played by DMXCraft) and light stay together, without a
//! visible window and without decoding the video in DMXCraft.
//!
//! * `/` – the page (transparent where no video plays)
//! * `/state` – what to show now (polled by the page)
//! * `/media/<id>` – the video files of the show (with byte ranges)
//! * `/error` – the page reports files it cannot play

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tiny_http::{Header, Method, Request, Response, Server, StatusCode};

const PAGE: &str = include_str!("video_out.html");
/// First port tried; the next free one is used if it is taken.
pub const DEFAULT_PORT: u16 = 8421;
/// At most this much of a file per answer: the page asks again for more.
const CHUNK: u64 = 16 << 20;
/// The page counts as connected while it polls at least this often.
const SEEN: Duration = Duration::from_secs(2);

/// A video block for the page.
#[derive(Clone, Debug, PartialEq)]
pub struct OutClip {
    /// Stable for the block (scene and clip).
    pub key: String,
    pub path: String,
    /// Position in the file (ms) at `Shared::at`.
    pub pos: f64,
    /// Playback speed (0 = paused).
    pub rate: f32,
    pub opacity: f32,
    /// Volume of the video's own sound (0 when DMXCraft plays it).
    pub volume: f32,
    /// Higher lies on top.
    pub z: i32,
    /// Starts soon: load and wait at the first frame, invisible.
    pub preload: bool,
}

#[derive(Default)]
struct Shared {
    clips: Vec<OutClip>,
    at: Option<Instant>,
    delay_ms: i32,
    /// Files the page may load: id → path.
    media: HashMap<String, String>,
    /// Last poll of the page.
    seen: Option<Instant>,
    /// Files that could not be played: path → message.
    errors: HashMap<String, String>,
}

pub struct VideoOut {
    port: u16,
    shared: Arc<Mutex<Shared>>,
    server: Arc<Server>,
}

impl Drop for VideoOut {
    fn drop(&mut self) {
        self.server.unblock();
    }
}

fn lock(s: &Mutex<Shared>) -> std::sync::MutexGuard<'_, Shared> {
    s.lock().unwrap_or_else(|p| p.into_inner())
}

/// Id of a file in URLs (does not reveal the path).
fn media_id(path: &str) -> String {
    use sha2::{Digest, Sha256};
    let h = Sha256::digest(path.as_bytes());
    h[..8].iter().map(|b| format!("{b:02x}")).collect()
}

impl VideoOut {
    /// Start the server on the first free port from `DEFAULT_PORT`.
    pub fn start() -> Result<Self, String> {
        let mut last = String::new();
        for port in DEFAULT_PORT..DEFAULT_PORT + 20 {
            match Server::http(("127.0.0.1", port)) {
                Ok(server) => {
                    let server = Arc::new(server);
                    let shared = Arc::new(Mutex::new(Shared::default()));
                    let (srv, sh) = (server.clone(), shared.clone());
                    std::thread::Builder::new()
                        .name("dmxcraft-video".into())
                        .spawn(move || {
                            for req in srv.incoming_requests() {
                                serve(&sh, port, req);
                            }
                        })
                        .map_err(|e| e.to_string())?;
                    return Ok(Self {
                        port,
                        shared,
                        server,
                    });
                }
                Err(e) => last = e.to_string(),
            }
        }
        Err(last)
    }

    /// Address of the page for the browser source.
    pub fn url(&self) -> String {
        format!("http://127.0.0.1:{}/", self.port)
    }

    /// Publish what the page should show now.
    pub fn set(&self, clips: Vec<OutClip>, delay_ms: i32) {
        let mut s = lock(&self.shared);
        for c in &clips {
            s.media
                .entry(media_id(&c.path))
                .or_insert_with(|| c.path.clone());
        }
        s.clips = clips;
        s.at = Some(Instant::now());
        s.delay_ms = delay_ms;
    }

    /// The browser source is open and polling.
    pub fn connected(&self) -> bool {
        lock(&self.shared).seen.is_some_and(|t| t.elapsed() < SEEN)
    }

    /// Files the browser source could not play (path, message).
    pub fn errors(&self) -> Vec<(String, String)> {
        let mut v: Vec<_> = lock(&self.shared)
            .errors
            .iter()
            .map(|(p, m)| (p.clone(), m.clone()))
            .collect();
        v.sort();
        v
    }
}

/// Only requests to this computer by its own name (no DNS rebinding).
fn host_ok(req: &Request, port: u16) -> bool {
    req.headers()
        .iter()
        .find(|h| h.field.equiv("Host"))
        .is_none_or(|h| {
            let v = h.value.as_str();
            v == format!("127.0.0.1:{port}") || v == format!("localhost:{port}")
        })
}

fn header(k: &str, v: &str) -> Header {
    Header::from_bytes(k.as_bytes(), v.as_bytes()).expect("valid header")
}

fn serve(shared: &Arc<Mutex<Shared>>, port: u16, mut req: Request) {
    if !host_ok(&req, port) {
        let _ = req.respond(Response::empty(403));
        return;
    }
    let url = req.url().split('?').next().unwrap_or("").to_string();
    match (req.method(), url.as_str()) {
        (Method::Get, "/") => {
            let _ = req.respond(
                Response::from_string(PAGE)
                    .with_header(header("Content-Type", "text/html; charset=utf-8"))
                    .with_header(header("Cache-Control", "no-store")),
            );
        }
        (Method::Get, "/state") => {
            let body = state_json(&mut lock(shared)).to_string();
            let _ = req.respond(
                Response::from_string(body)
                    .with_header(header("Content-Type", "application/json"))
                    .with_header(header("Cache-Control", "no-store")),
            );
        }
        (Method::Post, "/error") => {
            let mut body = String::new();
            let _ = req.as_reader().take(4096).read_to_string(&mut body);
            if let Ok(v) = serde_json::from_str::<Value>(&body) {
                let mut s = lock(shared);
                let id = v["media"].as_str().unwrap_or("");
                if let Some(path) = s.media.get(id).cloned() {
                    let msg = v["message"].as_str().unwrap_or("").to_string();
                    s.errors.insert(path, msg);
                }
            }
            let _ = req.respond(Response::empty(204));
        }
        (Method::Get, u) if u.starts_with("/media/") => {
            let path = lock(shared).media.get(&u[7..]).cloned();
            match path {
                // a file can take long: answer in its own thread, so the
                // page's polls go on
                Some(path) => {
                    std::thread::spawn(move || send_file(req, &path));
                }
                None => {
                    let _ = req.respond(Response::empty(404));
                }
            }
        }
        _ => {
            let _ = req.respond(Response::empty(404));
        }
    }
}

fn state_json(s: &mut Shared) -> Value {
    s.seen = Some(Instant::now());
    let elapsed = s.at.map_or(0.0, |t| t.elapsed().as_secs_f64() * 1000.0);
    let clips: Vec<Value> = s
        .clips
        .iter()
        .map(|c| {
            let pos = if c.preload {
                c.pos
            } else {
                c.pos + elapsed * f64::from(c.rate) - f64::from(s.delay_ms)
            };
            json!({
                "key": c.key,
                "media": media_id(&c.path),
                "pos": pos.max(0.0),
                "rate": c.rate,
                "opacity": c.opacity,
                "volume": c.volume,
                "z": c.z,
                "preload": c.preload,
            })
        })
        .collect();
    json!({ "clips": clips })
}

fn mime(path: &str) -> &'static str {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "webm" => "video/webm",
        "mkv" => "video/x-matroska",
        "ogv" | "ogg" => "video/ogg",
        _ => "video/mp4",
    }
}

/// Requested byte range `bytes=a-b` (inclusive), limited to the file.
fn range(req: &Request, len: u64) -> Option<(u64, u64)> {
    let h = req.headers().iter().find(|h| h.field.equiv("Range"))?;
    let spec = h.value.as_str().strip_prefix("bytes=")?;
    let (a, b) = spec.split(',').next()?.split_once('-')?;
    let (a, b) = match (a.trim(), b.trim()) {
        ("", n) => {
            let n: u64 = n.parse().ok()?;
            (len.saturating_sub(n), len.checked_sub(1)?)
        }
        (a, "") => (a.parse().ok()?, len.checked_sub(1)?),
        (a, b) => (
            a.parse().ok()?,
            b.parse::<u64>().ok()?.min(len.checked_sub(1)?),
        ),
    };
    (a <= b && a < len).then_some((a, b))
}

fn send_file(req: Request, path: &str) {
    let Ok(mut file) = File::open(path) else {
        let _ = req.respond(Response::empty(404));
        return;
    };
    let len = file.metadata().map_or(0, |m| m.len());
    let ctype = header("Content-Type", mime(path));
    let ranges = header("Accept-Ranges", "bytes");
    let Some((a, b)) = range(&req, len) else {
        if req.headers().iter().any(|h| h.field.equiv("Range")) {
            let _ = req.respond(
                Response::empty(416)
                    .with_header(header("Content-Range", &format!("bytes */{len}"))),
            );
            return;
        }
        // whole file (the page always asks for ranges; just in case)
        let n = len as usize;
        let _ = req.respond(Response::new(
            StatusCode(200),
            vec![ctype, ranges],
            file,
            Some(n),
            None,
        ));
        return;
    };
    let b = b.min(a + CHUNK - 1);
    if file.seek(SeekFrom::Start(a)).is_err() {
        let _ = req.respond(Response::empty(500));
        return;
    }
    let n = b - a + 1;
    let _ = req.respond(Response::new(
        StatusCode(206),
        vec![
            ctype,
            ranges,
            header("Content-Range", &format!("bytes {a}-{b}/{len}")),
        ],
        file.take(n),
        Some(n as usize),
        None,
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn get(port: u16, path: &str, extra: &str) -> (String, Vec<u8>) {
        let mut s = std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
        write!(
            s,
            "GET {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\n{extra}Connection: close\r\n\r\n"
        )
        .unwrap();
        let mut buf = Vec::new();
        s.read_to_end(&mut buf).unwrap();
        let split = buf.windows(4).position(|w| w == b"\r\n\r\n").unwrap();
        (
            String::from_utf8_lossy(&buf[..split]).to_string(),
            buf[split + 4..].to_vec(),
        )
    }

    #[test]
    fn serves_state_and_byte_ranges() {
        let dir = std::env::temp_dir().join(format!("dmxcraft-vo-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("clip.mp4");
        std::fs::write(&file, (0..100u8).collect::<Vec<_>>()).unwrap();
        let path = file.to_string_lossy().to_string();

        let out = VideoOut::start().unwrap();
        out.set(
            vec![OutClip {
                key: "1-2".into(),
                path: path.clone(),
                pos: 1000.0,
                rate: 1.0,
                opacity: 1.0,
                volume: 0.0,
                z: 1,
                preload: false,
            }],
            0,
        );
        let (head, body) = get(out.port, "/state", "");
        assert!(head.starts_with("HTTP/1.1 200"));
        let v: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["clips"][0]["key"], "1-2");
        assert!(v["clips"][0]["pos"].as_f64().unwrap() >= 1000.0);
        assert!(out.connected());

        let id = media_id(&path);
        let (head, body) = get(out.port, &format!("/media/{id}"), "Range: bytes=10-19\r\n");
        assert!(head.starts_with("HTTP/1.1 206"), "{head}");
        assert!(head.contains("bytes 10-19/100"));
        assert_eq!(body, (10..20u8).collect::<Vec<_>>());

        // unknown files are not served
        let (head, _) = get(out.port, "/media/0000", "");
        assert!(head.starts_with("HTTP/1.1 404"));
        // only by the computer's own name
        let mut s = std::net::TcpStream::connect(("127.0.0.1", out.port)).unwrap();
        write!(
            s,
            "GET /state HTTP/1.1\r\nHost: evil.example\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
        let mut buf = String::new();
        s.read_to_string(&mut buf).unwrap();
        assert!(buf.starts_with("HTTP/1.1 403"));
        let _ = std::fs::remove_dir_all(dir);
    }
}
