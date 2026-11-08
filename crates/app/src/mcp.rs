//! Anbindung des MCP-Servers (KI-Assistent) an die Anwendung: startet bzw.
//! stoppt den Server nach den Einstellungen und reicht seine Meldungen an
//! die Oberfläche weiter.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use dmxcraft_core::Engine;
use dmxcraft_mcp::server::McpServer;
use dmxcraft_mcp::{AppEvent, Host, ProfileStore, VizReply};
use dmxcraft_ofl::Profile;
use eframe::egui;
use serde_json::Value;

use crate::settings::Settings;

/// Zustand für die Anzeige in den Einstellungen.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum McpStatus {
    #[default]
    Off,
    Running(u16),
    Error(String),
}

/// Auftrag an den 3D-Visualisierer (wird in der Oberfläche ausgeführt).
pub struct VizJob {
    pub action: String,
    pub args: Value,
    pub reply: Sender<Result<VizReply, String>>,
}

/// Eigene Geräteprofile im Konfigurationsordner (wie der Profile Builder).
struct AppProfiles;

impl ProfileStore for AppProfiles {
    fn load(&self) -> Vec<Profile> {
        crate::profiles::load_all()
    }
    fn save(&self, p: &mut Profile) -> Result<(), String> {
        let others = crate::profiles::load_all();
        let today = crate::profiles::today();
        if p.created.is_empty() {
            p.created = today.clone();
        }
        p.modified = today;
        crate::profiles::save(p, &others).map(|_| ())
    }
    fn delete(&self, key: &str) -> Result<(), String> {
        crate::profiles::delete(key)
    }
}

pub struct McpLink {
    server: Option<McpServer>,
    viz_jobs: Arc<Mutex<Vec<VizJob>>>,
    tx: Sender<AppEvent>,
    rx: Receiver<AppEvent>,
    show_path: Arc<Mutex<String>>,
    locked: Arc<AtomicBool>,
    /// Zuletzt angewendete Einstellung (Port, wenn eingeschaltet).
    wanted: Option<u16>,
    pub status: McpStatus,
}

impl Default for McpLink {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            server: None,
            viz_jobs: Arc::default(),
            tx,
            rx,
            show_path: Arc::default(),
            locked: Arc::default(),
            wanted: None,
            status: McpStatus::Off,
        }
    }
}

impl McpLink {
    /// Einmal pro Frame: Server nach den Einstellungen starten/stoppen,
    /// gemeinsame Angaben aktualisieren, Meldungen abholen.
    pub fn update(
        &mut self,
        ctx: &egui::Context,
        engine: &Arc<Mutex<Engine>>,
        settings: &Settings,
        show_path: &str,
        locked: bool,
    ) -> Vec<AppEvent> {
        if let Ok(mut p) = self.show_path.lock() {
            if *p != show_path {
                *p = show_path.to_string();
            }
        }
        self.locked.store(locked, Ordering::Relaxed);
        // nur bei Änderung des Schalters/Ports (neu) starten
        let want = settings.mcp_enabled.then_some(settings.mcp_port);
        if want != self.wanted {
            self.wanted = want;
            self.server = None;
            self.status = match want {
                None => McpStatus::Off,
                Some(port) => match McpServer::start(self.host(ctx, engine), port) {
                    Ok(srv) => {
                        self.server = Some(srv);
                        McpStatus::Running(port)
                    }
                    Err(e) => McpStatus::Error(e),
                },
            };
        }
        self.rx.try_iter().collect()
    }

    /// Wartende Aufträge an den 3D-Visualisierer.
    pub fn take_viz_jobs(&self) -> Vec<VizJob> {
        self.viz_jobs
            .lock()
            .map(|mut j| std::mem::take(&mut *j))
            .unwrap_or_default()
    }

    /// Erneut versuchen (z. B. nach Ändern des Ports).
    pub fn retry(&mut self) {
        self.server = None;
        self.wanted = None;
    }

    fn host(&self, ctx: &egui::Context, engine: &Arc<Mutex<Engine>>) -> Host {
        let tx = self.tx.clone();
        let jobs = self.viz_jobs.clone();
        let vctx = ctx.clone();
        let ctx = ctx.clone();
        Host {
            engine: engine.clone(),
            show_path: self.show_path.clone(),
            notify: Arc::new(move |ev| {
                let _ = tx.send(ev);
                ctx.request_repaint();
            }),
            audio_duration_ms: Arc::new(crate::audio::duration_ms),
            locked: self.locked.clone(),
            profiles: Arc::new(AppProfiles),
            // Auftrag einreihen, Oberfläche wecken und auf die Antwort warten
            viz: Arc::new(move |action, args| {
                let (reply, rx) = channel();
                jobs.lock()
                    .map_err(|_| "Visualisierer nicht verfügbar".to_string())?
                    .push(VizJob {
                        action: action.to_string(),
                        args: args.clone(),
                        reply,
                    });
                vctx.request_repaint();
                rx.recv_timeout(std::time::Duration::from_secs(20))
                    .map_err(|_| "Der Visualisierer hat nicht geantwortet (Oberfläche beschäftigt oder minimiert?)".to_string())?
            }),
        }
    }
}

/// Befehl, mit dem MCP-Clients DMXCraft als Prozess starten (stdio-Brücke).
/// Kopie der Brücke `dmxcraft-mcp` im Konfigurationsordner. Clients wie
/// Claude Desktop starten diese Kopie und halten sie offen – so bleibt die
/// Datei im Build- bzw. Programmordner frei (unter Windows lässt sich eine
/// laufende .exe nicht ersetzen). Einmal je Start aktualisiert, wenn sie sich
/// unterscheidet und gerade nicht läuft.
fn installed_bridge() -> Option<std::path::PathBuf> {
    static PATH: std::sync::OnceLock<Option<std::path::PathBuf>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        let name = format!("dmxcraft-mcp{}", std::env::consts::EXE_SUFFIX);
        let src = std::env::current_exe().ok()?.with_file_name(&name);
        let src_bytes = std::fs::read(&src).ok()?;
        let dst = crate::settings::Settings::path()?.with_file_name(&name);
        let same = std::fs::read(&dst).is_ok_and(|b| b == src_bytes);
        if !same {
            if let Some(dir) = dst.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            // schlägt fehl, solange die alte Kopie läuft – dann bleibt sie
            let _ = std::fs::write(&dst, &src_bytes);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&dst, std::fs::Permissions::from_mode(0o755));
            }
        }
        if dst.exists() {
            Some(dst)
        } else {
            Some(src)
        }
    })
    .clone()
}

/// Befehl und Argumente für Clients, die die stdio-Brücke starten: bevorzugt
/// die Kopie von `dmxcraft-mcp` im Konfigurationsordner (sperrt weder
/// `dmxcraft` noch den Build-Ordner), sonst `dmxcraft mcp`.
pub fn bridge_command(port: u16) -> (String, Vec<String>) {
    let exe = std::env::current_exe().ok();
    let small = installed_bridge();
    let port_args = if port == dmxcraft_mcp::DEFAULT_PORT {
        Vec::new()
    } else {
        vec!["--port".to_string(), port.to_string()]
    };
    match small {
        Some(p) => (p.display().to_string(), port_args),
        None => (
            exe.map_or_else(|| "dmxcraft".into(), |p| p.display().to_string()),
            vec!["mcp".to_string()],
        ),
    }
}
