//! Programmeinstellungen (nicht Teil der Show), gespeichert als JSON im
//! Konfigurationsordner des Benutzers.

use crate::i18n::t;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Sprache der Oberfläche („de“; weitere folgen).
    pub language: String,
    /// Skalierung der Oberfläche (1.0 = 100 %).
    pub ui_scale: f32,
    /// Name dieses Rechners, z. B. für Fernsteuerung im Netz.
    pub instance_name: String,
    /// Beim Start direkt im Touch-Modus öffnen.
    pub start_touch: bool,
    /// Farbschema („navy“ oder „gray“).
    pub theme: String,
    /// Beim Start nach Updates suchen.
    pub check_updates: bool,
    /// Zuletzt verwendete Show-Datei.
    pub last_show: Option<String>,
    /// Zuletzt geöffnete Shows, neueste zuerst.
    pub recent: Vec<String>,
    /// Beim Start die zuletzt verwendete Show laden.
    pub open_last_show: bool,
    /// Gewählter Ausgang: "simulation" oder "open_dmx".
    pub output: String,
    /// USB-DMX: je Universum (Index) die Seriennummer des Kabels, leer = keins.
    pub open_dmx: Vec<String>,
    /// Szenen am unteren Rand in weitere Spalten umbrechen (statt Scrollen).
    pub wrap_scenes: bool,
    /// Wertebereiche der Gerätedatei auf den Fader-Schienen zeigen.
    #[serde(default = "yes")]
    pub fader_ranges: bool,
    /// MCP-Server für KI-Assistenten (nur lokal erreichbar).
    pub mcp_enabled: bool,
    /// Port des MCP-Servers (http://127.0.0.1:<port>/mcp).
    pub mcp_port: u16,
    /// Touch-Fernbedienung im lokalen Netz (Web-App für Smartphone/Tablet).
    pub remote_enabled: bool,
    /// Port der Fernbedienung (http://<rechner>:<port>).
    pub remote_port: u16,
    /// Optionale PIN der Fernbedienung (leer = ohne).
    pub remote_pin: String,
    /// Start einer SUPER SCENE setzt den Ton-Master auf 100 %.
    pub audio_reset_on_start: bool,
    /// Höchstlautstärke in Prozent: Ton-Master 100 % entspricht diesem Wert
    /// (z. B. zum Testen leiser, ohne die Systemlautstärke zu ändern).
    pub audio_limit_pct: u8,
    /// Dauer von „Ton ausfaden“ in ms.
    pub audio_fade_ms: u32,
    /// Nach dem Ausfaden die SUPER SCENES mit Ton stoppen.
    pub audio_fade_stop: bool,
    /// Qualität des 3D-Visualisierers (0 = niedrig … 3 = ultra).
    pub viz_quality: u8,
    /// Eingerichtete MIDI-Controller (Typ, LED-Ausgang).
    pub midi_devices: Vec<crate::midi_device::MidiDevice>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "de".into(),
            ui_scale: 1.0,
            instance_name: default_instance_name(),
            start_touch: false,
            theme: "navy".into(),
            check_updates: true,
            last_show: None,
            recent: Vec::new(),
            open_last_show: true,
            output: "simulation".into(),
            open_dmx: Vec::new(),
            wrap_scenes: false,
            fader_ranges: true,
            mcp_enabled: false,
            mcp_port: dmxcraft_mcp::DEFAULT_PORT,
            remote_enabled: false,
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

/// Rechnername als Vorschlag für den Instanznamen.
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

/// Wie viele Shows unter „Zuletzt geöffnet“ stehen.
pub const RECENT_MAX: usize = 8;

impl Settings {
    /// Aktiver Ausgang: USB-DMX, sobald einem Universum ein Kabel gehört.
    pub fn output_kind(&self) -> dmxcraft_output::OutputKind {
        if self.open_dmx.iter().any(|k| !k.is_empty()) {
            dmxcraft_output::OutputKind::OpenDmx
        } else {
            dmxcraft_output::OutputKind::Simulation
        }
    }

    /// Ausgangs-Einstellungen für die Ausgabeschicht.
    pub fn output_config(&self) -> dmxcraft_output::OutputConfig {
        dmxcraft_output::OutputConfig {
            open_dmx: (0..usize::from(dmxcraft_core::UNIVERSES))
                .map(|u| self.open_dmx.get(u).filter(|k| !k.is_empty()).cloned())
                .collect(),
        }
    }

    /// Kabel einem Universum zuordnen (`None` = keins). Ein Kabel kann nur
    /// einem Universum gehören.
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

    /// Merkt sich eine geöffnete oder gespeicherte Show.
    pub fn remember(&mut self, path: &str) {
        self.last_show = Some(path.to_string());
        self.recent.retain(|p| p != path);
        self.recent.insert(0, path.to_string());
        self.recent.truncate(RECENT_MAX);
    }

    /// Konfigurationsdatei je Betriebssystem.
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
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::path().ok_or(t("Kein Konfigurationsordner gefunden"))?;
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

#[cfg(test)]
mod tests {
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
