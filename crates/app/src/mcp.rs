//! Connects the MCP server (AI assistant) to the application: starts or
//! stops the server according to the settings and passes its messages on
//! to the UI.

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

/// State for display in the settings.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum McpStatus {
    #[default]
    Off,
    Running(u16),
    Error(String),
}

/// Request to the 3D visualizer (executed in the UI).
pub struct VizJob {
    pub action: String,
    pub args: Value,
    pub reply: Sender<Result<VizReply, String>>,
}

/// Custom fixture profiles in the config folder (like the Profile Builder).
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
    /// Last applied setting (port, if enabled).
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
    /// Once per frame: start/stop the server according to the settings, update
    /// shared data, collect messages.
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
        // only (re)start when the switch/port changes
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

    /// Pending requests to the 3D visualizer.
    pub fn take_viz_jobs(&self) -> Vec<VizJob> {
        self.viz_jobs
            .lock()
            .map(|mut j| std::mem::take(&mut *j))
            .unwrap_or_default()
    }

    /// Try again (e.g. after changing the port).
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
            // Queue the request, wake the UI and wait for the answer
            viz: Arc::new(move |action, args| {
                let (reply, rx) = channel();
                jobs.lock()
                    .map_err(|_| "Visualizer not available".to_string())?
                    .push(VizJob {
                        action: action.to_string(),
                        args: args.clone(),
                        reply,
                    });
                vctx.request_repaint();
                rx.recv_timeout(std::time::Duration::from_secs(20))
                    .map_err(|_| {
                        "The visualizer did not respond (UI busy or minimized?)".to_string()
                    })?
            }),
        }
    }
}

/// Command MCP clients use to start DMXCraft as a process (stdio bridge).
/// Copy of the `dmxcraft-mcp` bridge in the config folder. Clients such as
/// Claude Desktop start this copy and keep it open – so the file in the
/// build or program folder stays free (on Windows a running .exe cannot be
/// replaced). Updated once per start when it differs and is not running.
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
            // fails while the old copy is running – then it stays
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

/// Command and arguments for clients that start the stdio bridge: prefers
/// the copy of `dmxcraft-mcp` in the config folder (locks neither
/// `dmxcraft` nor the build folder), otherwise `dmxcraft mcp`.
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
