//! Selbst-Update über GitHub-Releases (dist/axoupdater).
//!
//! Funktioniert für Installationen über die offiziellen Installer (Shell,
//! PowerShell, MSI) – diese hinterlegen eine „Install-Receipt“, aus der der
//! Updater Installationsort und Quelle kennt. Bei `cargo run` gibt es keine
//! Receipt; dann wird das in der Oberfläche angezeigt.

use std::sync::{Arc, Mutex};

use axoupdater::AxoUpdater;

const APP_NAME: &str = "dmxcraft";

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    UpToDate,
    Available,
    Installing,
    /// Neue Version installiert – Neustart nötig.
    Installed(String),
    /// Keine Installation per Installer (z. B. Entwickler-Build).
    NotInstalled,
    Error(String),
}

impl UpdateState {
    pub fn busy(&self) -> bool {
        matches!(self, UpdateState::Checking | UpdateState::Installing)
    }
}

#[derive(Clone)]
pub struct Updater {
    state: Arc<Mutex<UpdateState>>,
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            state: Arc::new(Mutex::new(UpdateState::Idle)),
        }
    }
}

impl Updater {
    pub fn state(&self) -> UpdateState {
        self.state
            .lock()
            .map(|s| s.clone())
            .unwrap_or(UpdateState::Idle)
    }

    fn set(&self, s: UpdateState) {
        if let Ok(mut g) = self.state.lock() {
            *g = s;
        }
    }

    pub fn current_version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    /// Prüft im Hintergrund, ob eine neuere Version auf GitHub liegt.
    pub fn check(&self, ctx: eframe::egui::Context) {
        if self.state().busy() {
            return;
        }
        self.set(UpdateState::Checking);
        let me = self.clone();
        std::thread::spawn(move || {
            let mut up = AxoUpdater::new_for(APP_NAME);
            let state = if up.load_receipt().is_err() {
                UpdateState::NotInstalled
            } else {
                match up.is_update_needed_sync() {
                    Ok(true) => UpdateState::Available,
                    Ok(false) => UpdateState::UpToDate,
                    Err(e) => UpdateState::Error(e.to_string()),
                }
            };
            me.set(state);
            ctx.request_repaint();
        });
    }

    /// Lädt die neueste Version herunter und installiert sie.
    pub fn install(&self, ctx: eframe::egui::Context) {
        if self.state().busy() {
            return;
        }
        self.set(UpdateState::Installing);
        let me = self.clone();
        std::thread::spawn(move || {
            let mut up = AxoUpdater::new_for(APP_NAME);
            let state = match up.load_receipt() {
                Err(_) => UpdateState::NotInstalled,
                Ok(up) => {
                    up.disable_installer_output();
                    match up.run_sync() {
                        Ok(Some(r)) => UpdateState::Installed(r.new_version.to_string()),
                        Ok(None) => UpdateState::UpToDate,
                        Err(e) => UpdateState::Error(e.to_string()),
                    }
                }
            };
            me.set(state);
            ctx.request_repaint();
        });
    }
}
