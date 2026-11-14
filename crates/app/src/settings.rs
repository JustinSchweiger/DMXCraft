//! Program settings (not part of the show), stored as JSON in the user's
//! config folder.

use crate::i18n::t;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Language of the interface (“en” or “de”).
    pub language: String,
    /// Scaling of the interface (1.0 = 100 %).
    pub ui_scale: f32,
    /// Name of this computer, e.g. for remote control over the network.
    pub instance_name: String,
    /// Open directly in touch mode at startup.
    pub start_touch: bool,
    /// Color scheme (key of `theme::Theme`, e.g. “navy” for Midnight).
    pub theme: String,
    /// Own accent color instead of the theme's (None = theme default).
    pub accent_color: Option<[u8; 3]>,
    /// Own background of the work areas (None = theme default).
    pub canvas_color: Option<[u8; 3]>,
    /// Check for updates at startup.
    pub check_updates: bool,
    /// Last used show file.
    pub last_show: Option<String>,
    /// Recently opened shows, newest first.
    pub recent: Vec<String>,
    /// Load the last used show at startup.
    pub open_last_show: bool,
    /// Selected output: "simulation" or "open_dmx".
    pub output: String,
    /// USB DMX: per universe (index) the serial number of the cable, empty = none.
    pub open_dmx: Vec<String>,
    /// Wrap scenes at the bottom into more columns (instead of scrolling).
    pub wrap_scenes: bool,
    /// Scene sidebar hidden (only its tab bar is shown).
    #[serde(default)]
    pub hide_scene_props: bool,
    /// Programmer hidden (only a narrow bar is shown).
    #[serde(default)]
    pub hide_programmer: bool,
    /// Lock dialog: also lock playback (last choice).
    #[serde(default)]
    pub lock_playback: bool,
    /// Show the value ranges of the fixture file on the fader tracks.
    #[serde(default = "yes")]
    pub fader_ranges: bool,
    /// MCP server for AI assistants (reachable locally only).
    pub mcp_enabled: bool,
    /// Port of the MCP server (http://127.0.0.1:<port>/mcp).
    pub mcp_port: u16,
    /// Touch remote on the local network (web app for phone/tablet).
    pub remote_enabled: bool,
    /// Old setting (until 0.4): 3D visualizer and Profile Builder as tabs.
    /// Only read to take it over into `hidden_tabs`.
    #[serde(skip_serializing)]
    pub tools_as_tabs: bool,
    /// Save the show to its file every n minutes when it has unsaved changes
    /// (0 = off).
    pub autosave_minutes: u32,
    /// Tabs hidden in the title bar (keys of `state::Mode`).
    pub hidden_tabs: Vec<String>,
    /// Port of the remote (http://<computer>:<port>).
    pub remote_port: u16,
    /// Optional PIN of the remote (empty = none).
    pub remote_pin: String,
    /// Starting a SUPER SCENE sets the sound master to 100 %.
    pub audio_reset_on_start: bool,
    /// Maximum volume in percent: sound master 100 % corresponds to this value
    /// (e.g. quieter for testing without changing the system volume).
    pub audio_limit_pct: u8,
    /// Duration of “Fade out sound” in ms.
    pub audio_fade_ms: u32,
    /// Stop the SUPER SCENES with audio after the fade-out.
    pub audio_fade_stop: bool,
    /// Quality of the 3D visualizer (0 = low … 3 = ultra).
    pub viz_quality: u8,
    /// Configured MIDI controllers (type, LED output).
    pub midi_devices: Vec<crate::midi_device::MidiDevice>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".into(),
            ui_scale: 1.0,
            instance_name: default_instance_name(),
            start_touch: false,
            theme: "navy".into(),
            accent_color: None,
            canvas_color: None,
            check_updates: true,
            last_show: None,
            recent: Vec::new(),
            open_last_show: true,
            output: "simulation".into(),
            open_dmx: Vec::new(),
            wrap_scenes: false,
            hide_scene_props: false,
            hide_programmer: false,
            lock_playback: false,
            fader_ranges: true,
            mcp_enabled: false,
            mcp_port: dmxcraft_mcp::DEFAULT_PORT,
            remote_enabled: false,
            tools_as_tabs: false,
            hidden_tabs: vec!["visualizer".into(), "builder".into()],
            autosave_minutes: 5,
            remote_port: crate::remote::DEFAULT_PORT,
            remote_pin: String::new(),
            audio_reset_on_start: true,
            audio_fade_ms: 5000,
            audio_limit_pct: 100,
            audio_fade_stop: false,
            viz_quality: 1,
            midi_devices: Vec::new(),
        }
    }
}

/// Computer name as a suggestion for the instance name.
fn default_instance_name() -> String {
    ["COMPUTERNAME", "HOSTNAME"]
        .iter()
        .find_map(|v| std::env::var(v).ok())
        .or_else(|| {
            std::fs::read_to_string("/etc/hostname")
                .ok()
                .map(|s| s.trim().to_string())
        })
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "DMXCraft".into())
}

/// How many shows are listed under “Recently opened”.
pub const RECENT_MAX: usize = 8;

impl Settings {
    /// Active output: USB DMX as soon as a cable belongs to a universe.
    pub fn output_kind(&self) -> dmxcraft_output::OutputKind {
        if self.open_dmx.iter().any(|k| !k.is_empty()) {
            dmxcraft_output::OutputKind::OpenDmx
        } else {
            dmxcraft_output::OutputKind::Simulation
        }
    }

    /// Output settings for the output layer.
    pub fn output_config(&self) -> dmxcraft_output::OutputConfig {
        dmxcraft_output::OutputConfig {
            open_dmx: (0..usize::from(dmxcraft_core::UNIVERSES))
                .map(|u| self.open_dmx.get(u).filter(|k| !k.is_empty()).cloned())
                .collect(),
        }
    }

    /// Assign a cable to a universe (`None` = none). A cable can only belong to
    /// one universe.
    pub fn assign_open_dmx(&mut self, universe: usize, key: Option<String>) {
        if let Some(k) = &key {
            for slot in &mut self.open_dmx {
                if slot == k {
                    slot.clear();
                }
            }
        }
        if self.open_dmx.len() <= universe {
            self.open_dmx.resize(universe + 1, String::new());
        }
        self.open_dmx[universe] = key.unwrap_or_default();
        while self.open_dmx.last().is_some_and(|k| k.is_empty()) {
            self.open_dmx.pop();
        }
    }

    /// Remembers an opened or saved show.
    pub fn remember(&mut self, path: &str) {
        self.last_show = Some(path.to_string());
        self.recent.retain(|p| p != path);
        self.recent.insert(0, path.to_string());
        self.recent.truncate(RECENT_MAX);
    }

    /// Config file per operating system.
    pub fn path() -> Option<PathBuf> {
        let base = if cfg!(target_os = "windows") {
            std::env::var_os("APPDATA").map(PathBuf::from)
        } else if cfg!(target_os = "macos") {
            std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
        } else {
            std::env::var_os("XDG_CONFIG_HOME")
                .map(PathBuf::from)
                .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
        }?;
        let dir = if cfg!(target_os = "linux") {
            "dmxcraft"
        } else {
            "DmxCraft"
        };
        Some(base.join(dir).join("settings.json"))
    }

    pub fn load() -> Settings {
        Self::path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| Self::parse(&s))
            .unwrap_or_default()
    }

    fn parse(json: &str) -> Option<Settings> {
        let value: serde_json::Value = serde_json::from_str(json).ok()?;
        let mut s: Settings = serde_json::from_value(value.clone()).ok()?;
        // Settings from before the tab context menu: tools were shown as tabs
        // only with “tools_as_tabs”
        if value.get("hidden_tabs").is_none() && s.tools_as_tabs {
            s.hidden_tabs.clear();
        }
        Some(s)
    }

    /// Is this tab shown in the title bar?
    pub fn tab_visible(&self, m: crate::state::Mode) -> bool {
        !self.hidden_tabs.iter().any(|k| k == m.key())
    }

    /// Show or hide a tab. The last visible tab cannot be hidden.
    pub fn set_tab_visible(&mut self, m: crate::state::Mode, visible: bool) -> bool {
        if visible {
            self.hidden_tabs.retain(|k| k != m.key());
            return true;
        }
        let others = crate::state::Mode::TABS
            .iter()
            .any(|&o| o != m && self.tab_visible(o));
        if !others || !self.tab_visible(m) {
            return false;
        }
        self.hidden_tabs.push(m.key().into());
        true
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path().ok_or(t("No configuration folder found"))?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }
}

fn yes() -> bool {
    true
}

impl Settings {
    /// Own colors on top of the theme.
    pub fn theme_overrides(&self) -> crate::ui::theme::Overrides {
        crate::ui::theme::Overrides {
            accent: self.accent_color,
            canvas: self.canvas_color,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hidden_tabs_from_old_setting() {
        use crate::state::Mode;
        let old = Settings::parse(r#"{"tools_as_tabs": true}"#).unwrap();
        assert!(old.tab_visible(Mode::Visualizer) && old.tab_visible(Mode::Builder));
        let old = Settings::parse(r#"{"tools_as_tabs": false}"#).unwrap();
        assert!(!old.tab_visible(Mode::Visualizer));
        let new = Settings::parse(r#"{"hidden_tabs": ["touch"]}"#).unwrap();
        assert!(new.tab_visible(Mode::Visualizer) && !new.tab_visible(Mode::Touch));
        assert!(!serde_json::to_string(&new)
            .unwrap()
            .contains("tools_as_tabs"));
    }

    #[test]
    fn last_tab_stays() {
        use crate::state::Mode;
        let mut s = Settings::default();
        for m in Mode::TABS {
            s.set_tab_visible(m, false);
        }
        assert_eq!(Mode::TABS.iter().filter(|&&m| s.tab_visible(m)).count(), 1);
    }

    use super::*;

    #[test]
    fn unknown_or_missing_fields_use_defaults() {
        let s: Settings = serde_json::from_str(r#"{"ui_scale": 1.25, "neu": 1}"#).unwrap();
        assert_eq!(s.ui_scale, 1.25);
        assert!(s.check_updates);
        assert_eq!(s.output, "simulation");
    }

    #[test]
    fn cable_belongs_to_one_universe() {
        let mut s = Settings::default();
        s.assign_open_dmx(0, Some("A".into()));
        s.assign_open_dmx(1, Some("B".into()));
        s.assign_open_dmx(1, Some("A".into()));
        let cfg = s.output_config();
        assert_eq!(cfg.open_dmx[0], None);
        assert_eq!(cfg.open_dmx[1].as_deref(), Some("A"));
        assert_eq!(cfg.open_dmx.len(), 8);
        assert_eq!(s.output_kind(), dmxcraft_output::OutputKind::OpenDmx);
        s.assign_open_dmx(1, None);
        assert!(s.open_dmx.is_empty());
        assert_eq!(s.output_kind(), dmxcraft_output::OutputKind::Simulation);
    }
}
