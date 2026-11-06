//! Programmeinstellungen (nicht Teil der Show), gespeichert als JSON im
//! Konfigurationsordner des Benutzers.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// Skalierung der Oberfläche (1.0 = 100 %).
    pub ui_scale: f32,
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
    /// Gesalzener Hash des Sperr-Passworts („salz:hash“).
    pub lock_hash: Option<String>,
    /// Oberfläche war beim Beenden gesperrt (bleibt nach Neustart gesperrt).
    pub locked: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            ui_scale: 1.0,
            check_updates: true,
            last_show: None,
            recent: Vec::new(),
            open_last_show: true,
            output: "simulation".into(),
            open_dmx: Vec::new(),
            lock_hash: None,
            locked: false,
        }
    }
}

/// Wie viele Shows unter „Zuletzt geöffnet“ stehen.
pub const RECENT_MAX: usize = 8;

impl Settings {
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
        let path = Self::path().ok_or("Kein Konfigurationsordner gefunden")?;
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(path, json).map_err(|e| e.to_string())
    }
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
        s.assign_open_dmx(1, None);
        assert!(s.open_dmx.is_empty());
    }
}
