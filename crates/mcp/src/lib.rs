//! MCP-Server (Model Context Protocol) für DMXCraft.
//!
//! Ein KI-Assistent (z. B. Claude) kann darüber die laufende Show lesen und
//! bearbeiten: Geräte patchen, Bänke und Szenen anlegen, Effekte und
//! SUPER SCENES bauen, Szenen starten/stoppen, Live-Werte setzen …
//!
//! Aufbau:
//! * [`tools`] – die Werkzeuge (reine Logik auf der [`Engine`]),
//! * [`protocol`] – JSON-RPC/MCP (initialize, tools/list, tools/call),
//! * [`server`] – HTTP-Endpunkt `http://127.0.0.1:<port>/mcp`
//!   („Streamable HTTP“, nur lokal erreichbar),
//! * [`bridge`] – stdio-Brücke (`dmxcraft mcp`) für Clients, die Server als
//!   Prozess starten (z. B. Claude Desktop); leitet an den HTTP-Endpunkt weiter.

pub mod bridge;
pub mod protocol;
pub mod server;
pub mod tools;
mod venue;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use dmxcraft_core::{Engine, SceneId};

/// Standard-Port des HTTP-Endpunkts.
pub const DEFAULT_PORT: u16 = 7373;

/// Liefert die Länge einer Audiodatei in ms.
pub type AudioDuration = dyn Fn(&str) -> Option<u64> + Send + Sync;

/// Meldungen an die Oberfläche.
#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent {
    /// Ein Werkzeug wurde ausgeführt (Name, Kurzbeschreibung des Ergebnisses).
    ToolCalled(String),
    /// Die Show wurde unter diesem Pfad gespeichert.
    Saved(String),
    /// Diese Szene in der Oberfläche auswählen.
    SelectScene(SceneId),
}

/// Verbindung zur Anwendung: gemeinsame Engine und Rückmeldungen.
#[derive(Clone)]
pub struct Host {
    pub engine: Arc<Mutex<Engine>>,
    /// Pfad der geöffneten Show (leer = noch nicht gespeichert).
    pub show_path: Arc<Mutex<String>>,
    /// Meldung an die Oberfläche (danach neu zeichnen).
    pub notify: Arc<dyn Fn(AppEvent) + Send + Sync>,
    /// Länge einer Audiodatei in ms (für Audioblöcke in SUPER SCENES).
    pub audio_duration_ms: Arc<AudioDuration>,
    /// Oberfläche gesperrt: nur lesende Werkzeuge erlaubt.
    pub locked: Arc<AtomicBool>,
}

impl Host {
    /// Nur für Tests: Engine ohne Oberfläche.
    pub fn headless(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            show_path: Arc::new(Mutex::new(String::new())),
            notify: Arc::new(|_| {}),
            audio_duration_ms: Arc::new(|_| None),
            locked: Arc::new(AtomicBool::new(false)),
        }
    }
}
