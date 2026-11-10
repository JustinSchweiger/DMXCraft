//! MCP server (Model Context Protocol) for DMXCraft.
//!
//! An AI assistant (e.g. Claude) can use it to read and edit the running
//! show: patch fixtures, create banks and scenes, build effects and SUPER
//! SCENES, start/stop scenes, set live values …
//!
//! Structure:
//! * [`tools`] – the tools (pure logic on the [`Engine`]),
//! * [`profiles`] – Profile Builder (custom fixture profiles),
//! * [`protocol`] – JSON-RPC/MCP (initialize, tools/list, tools/call),
//! * [`server`] – HTTP endpoint `http://127.0.0.1:<port>/mcp`
//!   (“Streamable HTTP”, reachable locally only),
//! * [`bridge`] – stdio bridge (`dmxcraft mcp`) for clients that start
//!   servers as a process (e.g. Claude Desktop); forwards to the HTTP
//!   endpoint.

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

/// Default port of the HTTP endpoint.
pub const DEFAULT_PORT: u16 = 7373;

/// Returns the length of an audio file in ms.
pub type AudioDuration = dyn Fn(&str) -> Option<u64> + Send + Sync;

/// Messages to the UI.
#[derive(Clone, Debug, PartialEq)]
pub enum AppEvent {
    /// A tool was executed (name, short description of the result).
    ToolCalled(String),
    /// The show was saved under this path.
    Saved(String),
    /// Select this scene in the UI.
    SelectScene(SceneId),
    /// Custom fixture profiles were changed (reload the library).
    ProfilesChanged,
}

/// Storage of the custom fixture profiles (Profile Builder). The app stores
/// them in the config folder, tests in memory.
pub trait ProfileStore: Send + Sync {
    /// All custom profiles.
    fn load(&self) -> Vec<Profile>;
    /// Save; assigns the key on the first save.
    fn save(&self, profile: &mut Profile) -> Result<(), String>;
    /// Remove (the app moves the file into a trash folder).
    fn delete(&self, key: &str) -> Result<(), String>;
}

/// Profiles in memory only (tests, without a UI).
#[derive(Default)]
pub struct MemoryProfiles(pub Mutex<Vec<Profile>>);

impl ProfileStore for MemoryProfiles {
    fn load(&self) -> Vec<Profile> {
        self.0.lock().map(|l| l.clone()).unwrap_or_default()
    }
    fn save(&self, p: &mut Profile) -> Result<(), String> {
        let mut list = self.0.lock().map_err(|_| "locked")?;
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
        let mut list = self.0.lock().map_err(|_| "locked")?;
        list.retain(|x| x.key != key);
        Ok(())
    }
}

/// Answer of the 3D visualizer: data and optionally a PNG image.
pub type VizReply = (Value, Option<Vec<u8>>);

/// Request to the 3D visualizer (action, arguments). Runs in the UI; the
/// call waits for the answer.
pub type VizFn = dyn Fn(&str, &Value) -> Result<VizReply, String> + Send + Sync;

/// Connection to the application: shared engine and feedback.
#[derive(Clone)]
pub struct Host {
    pub engine: Arc<Mutex<Engine>>,
    /// Path of the open show (empty = not saved yet).
    pub show_path: Arc<Mutex<String>>,
    /// Message to the UI (redraw afterwards).
    pub notify: Arc<dyn Fn(AppEvent) + Send + Sync>,
    /// Length of an audio file in ms (for audio blocks in SUPER SCENES).
    pub audio_duration_ms: Arc<AudioDuration>,
    /// UI locked: only read-only tools allowed.
    pub locked: Arc<AtomicBool>,
    /// Custom fixture profiles (Profile Builder).
    pub profiles: Arc<dyn ProfileStore>,
    /// 3D visualizer (camera, quality, image).
    pub viz: Arc<VizFn>,
}

impl Host {
    /// For tests only: engine without a UI.
    pub fn headless(engine: Engine) -> Self {
        Self {
            engine: Arc::new(Mutex::new(engine)),
            show_path: Arc::new(Mutex::new(String::new())),
            notify: Arc::new(|_| {}),
            audio_duration_ms: Arc::new(|_| None),
            locked: Arc::new(AtomicBool::new(false)),
            profiles: Arc::new(MemoryProfiles::default()),
            viz: Arc::new(|_, _| Err("No 3D visualizer available".into())),
        }
    }
}
