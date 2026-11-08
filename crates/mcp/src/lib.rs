//! MCP-Server (Model Context Protocol) für DMXCraft.
//!
//! Ein KI-Assistent (z. B. Claude) kann darüber die laufende Show lesen und
//! bearbeiten: Geräte patchen, Bänke und Szenen anlegen, Effekte und
//! SUPER SCENES bauen, Szenen starten/stoppen, Live-Werte setzen …
//!
//! Aufbau:
//! * [`tools`] – die Werkzeuge (reine Logik auf der [`Engine`]),
//! * [`profiles`] – Profile Builder (eigene Geräteprofile),
//! * [`protocol`] – JSON-RPC/MCP (initialize, tools/list, tools/call),
//! * [`server`] – HTTP-Endpunkt `http://127.0.0.1:<port>/mcp`
//!   („Streamable HTTP“, nur lokal erreichbar),
//! * [`bridge`] – stdio-Brücke (`dmxcraft mcp`) für Clients, die Server als
//!   Prozess starten (z. B. Claude Desktop); leitet an den HTTP-Endpunkt weiter.

pub mod bridge;
mod profiles;
pub mod protocol;
pub mod server;
pub mod tools;
mod venue;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use dmxcraft_core::{Engine, SceneId};
use dmxcraft_ofl::Profile;
use serde_json::Value;

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
    /// Eigene Geräteprofile wurden geändert (Bibliothek neu laden).
    ProfilesChanged,
}

/// Ablage der eigenen Geräteprofile (Profile Builder). Die App speichert im
/// Konfigurationsordner, Tests im Speicher.
pub trait ProfileStore: Send + Sync {
    /// Alle eigenen Profile.
    fn load(&self) -> Vec<Profile>;
    /// Speichern; vergibt beim ersten Speichern den Schlüssel.
    fn save(&self, profile: &mut Profile) -> Result<(), String>;
    /// Entfernen (die App verschiebt die Datei in einen Papierkorb-Ordner).
    fn delete(&self, key: &str) -> Result<(), String>;
}

/// Profile nur im Speicher (Tests, ohne Oberfläche).
#[derive(Default)]
pub struct MemoryProfiles(pub Mutex<Vec<Profile>>);

impl ProfileStore for MemoryProfiles {
    fn load(&self) -> Vec<Profile> {
        self.0.lock().map(|l| l.clone()).unwrap_or_default()
    }
    fn save(&self, p: &mut Profile) -> Result<(), String> {
        let mut list = self.0.lock().map_err(|_| "gesperrt")?;
        if p.key.is_empty() {
            let base = p.suggested_key();
            let mut key = base.clone();
            let mut n = 2;
            while list.iter().any(|x| x.key == key) {
                key = format!("{base}-{n}");
                n += 1;
            }
            p.key = key;
        }
        list.retain(|x| x.key != p.key);
        list.push(p.clone());
        Ok(())
    }
    fn delete(&self, key: &str) -> Result<(), String> {
        let mut list = self.0.lock().map_err(|_| "gesperrt")?;
        list.retain(|x| x.key != key);
        Ok(())
    }
}

/// Antwort des 3D-Visualisierers: Angaben und optional ein PNG-Bild.
pub type VizReply = (Value, Option<Vec<u8>>);

/// Anfrage an den 3D-Visualisierer (Aktion, Argumente). Läuft in der
/// Oberfläche; der Aufruf wartet auf die Antwort.
pub type VizFn = dyn Fn(&str, &Value) -> Result<VizReply, String> + Send + Sync;

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
    /// Eigene Geräteprofile (Profile Builder).
    pub profiles: Arc<dyn ProfileStore>,
    /// 3D-Visualisierer (Kamera, Qualität, Bild).
    pub viz: Arc<VizFn>,
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
            profiles: Arc::new(MemoryProfiles::default()),
            viz: Arc::new(|_, _| Err("Kein 3D-Visualisierer verfügbar".into())),
        }
    }
}
