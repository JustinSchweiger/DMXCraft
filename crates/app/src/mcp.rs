//! Anbindung des MCP-Servers (KI-Assistent) an die Anwendung: startet bzw.
//! stoppt den Server nach den Einstellungen und reicht seine Meldungen an
//! die Oberfläche weiter.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use dmxcraft_core::Engine;
use dmxcraft_mcp::server::McpServer;
use dmxcraft_mcp::{AppEvent, Host};
use eframe::egui;

use crate::settings::Settings;

/// Zustand für die Anzeige in den Einstellungen.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum McpStatus {
    #[default]
    Off,
    Running(u16),
    Error(String),
}

pub struct McpLink {
    server: Option<McpServer>,
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

    /// Erneut versuchen (z. B. nach Ändern des Ports).
    pub fn retry(&mut self) {
        self.server = None;
        self.wanted = None;
    }

    fn host(&self, ctx: &egui::Context, engine: &Arc<Mutex<Engine>>) -> Host {
        let tx = self.tx.clone();
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
        }
    }
}

/// Befehl, mit dem MCP-Clients DMXCraft als Prozess starten (stdio-Brücke).
pub fn bridge_command() -> String {
    std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "dmxcraft".into())
}
