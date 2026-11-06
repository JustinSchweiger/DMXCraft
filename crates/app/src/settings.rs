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
    /// Beim Start die zuletzt verwendete Show laden.
    pub open_last_show: bool,
    /// Gewählter Ausgang: "simulation" oder "dvc_gold".
    pub output: String,
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
            open_last_show: true,
            output: "simulation".into(),
            lock_hash: None,
            locked: false,
        }
    }
}

impl Settings {
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
}
