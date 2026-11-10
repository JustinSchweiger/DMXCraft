//! Color palettes for COLOR FX. All palettes (initially the bundled ones)
//! are editable and live in the config folder (`palettes.json`).

use serde::{Deserialize, Serialize};

use crate::i18n::t;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamedPalette {
    pub name: String,
    pub colors: Vec<[u8; 3]>,
}

fn named(name: &str, colors: &[[u8; 3]]) -> NamedPalette {
    NamedPalette {
        name: name.into(),
        colors: colors.to_vec(),
    }
}

/// Bundled palettes (own selection).
pub fn builtin() -> Vec<NamedPalette> {
    vec![
        named(
            t("Rainbow"),
            &[
                [255, 0, 0],
                [255, 160, 0],
                [255, 255, 0],
                [0, 255, 0],
                [0, 255, 255],
                [0, 0, 255],
                [180, 0, 255],
                [255, 0, 160],
            ],
        ),
        named(t("Black and white"), &[[0, 0, 0], [255, 255, 255]]),
        named(
            t("Primary colors"),
            &[[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 0]],
        ),
        named(
            t("Pastel"),
            &[
                [255, 170, 170],
                [255, 220, 170],
                [255, 255, 180],
                [180, 255, 200],
                [180, 220, 255],
                [220, 190, 255],
            ],
        ),
        named(
            t("Fire"),
            &[[255, 0, 0], [255, 70, 0], [255, 140, 0], [255, 210, 40]],
        ),
        named(
            t("Ice"),
            &[
                [255, 255, 255],
                [150, 220, 255],
                [0, 140, 255],
                [0, 40, 200],
            ],
        ),
        named(
            t("Ocean"),
            &[[0, 30, 120], [0, 110, 200], [0, 200, 200], [0, 255, 160]],
        ),
        named(
            t("Forest"),
            &[[0, 80, 0], [40, 160, 0], [150, 220, 0], [255, 200, 0]],
        ),
        named(
            t("Sunset"),
            &[[255, 120, 0], [255, 40, 80], [180, 0, 160], [60, 0, 160]],
        ),
        named(
            t("Neon"),
            &[[255, 0, 200], [0, 255, 255], [180, 255, 0], [140, 0, 255]],
        ),
        named(
            t("Warm"),
            &[
                [255, 140, 30],
                [255, 190, 90],
                [255, 230, 170],
                [255, 250, 230],
            ],
        ),
        named(
            t("Blue-magenta"),
            &[[0, 0, 255], [120, 0, 255], [255, 0, 220]],
        ),
        named(
            t("Grayscale"),
            &[[0, 0, 0], [80, 80, 80], [160, 160, 160], [255, 255, 255]],
        ),
    ]
}

fn path() -> Option<std::path::PathBuf> {
    crate::settings::Settings::path().map(|p| p.with_file_name("palettes.json"))
}

/// Load all palettes. Without a saved file: the bundled ones.
pub fn load_user() -> Vec<NamedPalette> {
    path()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(builtin)
}

pub fn save_user(list: &[NamedPalette]) -> Result<(), String> {
    let path = path().ok_or(t("No configuration folder found"))?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(list).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}
