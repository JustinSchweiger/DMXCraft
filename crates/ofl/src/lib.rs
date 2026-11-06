//! Open Fixture Library (<https://open-fixture-library.org>).
//!
//! * [`convert`] wandelt eine OFL-Gerätedatei (JSON) in LightCraft-Gerätetypen
//!   um – ein [`FixtureType`] pro DMX-Modus.
//! * [`Library`] ist die eingebettete, vorkonvertierte Gesamtbibliothek
//!   (`data/ofl-library.json.gz`, erzeugt mit dem Tool `ofl-bundle`).
//!
//! Die OFL-Daten stehen unter MIT-Lizenz (siehe `data/OFL-LICENSE`).

pub mod convert;

use std::io::Read;
use std::sync::OnceLock;

use lightcraft_core::FixtureType;
use serde::{Deserialize, Serialize};

pub use convert::{convert_fixture, OflError};

/// Ein Gerät der Bibliothek mit allen Modi.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LibraryFixture {
    pub manufacturer_key: String,
    pub manufacturer: String,
    pub key: String,
    pub name: String,
    #[serde(default)]
    pub categories: Vec<String>,
    pub modes: Vec<FixtureType>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Library {
    /// Git-Commit der OFL, aus dem die Bibliothek erzeugt wurde.
    pub source_commit: String,
    pub source_date: String,
    pub fixtures: Vec<LibraryFixture>,
}

static EMBEDDED: &[u8] = include_bytes!("../data/ofl-library.json.gz");

impl Library {
    /// Die mit dem Programm ausgelieferte Bibliothek (wird beim ersten
    /// Zugriff entpackt).
    pub fn embedded() -> &'static Library {
        static LIB: OnceLock<Library> = OnceLock::new();
        LIB.get_or_init(|| Library::from_gzip(EMBEDDED).unwrap_or_default())
    }

    pub fn from_gzip(bytes: &[u8]) -> Result<Library, OflError> {
        let mut json = String::new();
        flate2::read::GzDecoder::new(bytes)
            .read_to_string(&mut json)
            .map_err(|e| OflError::Invalid(format!("gzip: {e}")))?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn to_gzip(&self) -> Result<Vec<u8>, OflError> {
        use std::io::Write;
        let json = serde_json::to_vec(self)?;
        let mut enc = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        enc.write_all(&json)
            .map_err(|e| OflError::Invalid(e.to_string()))?;
        enc.finish().map_err(|e| OflError::Invalid(e.to_string()))
    }

    /// Volltextsuche über Hersteller, Name und Kategorien. Alle Wörter der
    /// Suche müssen vorkommen (ohne Groß-/Kleinschreibung).
    pub fn search(&self, query: &str, limit: usize) -> Vec<&LibraryFixture> {
        let words: Vec<String> = query
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();
        self.fixtures
            .iter()
            .filter(|f| {
                if words.is_empty() {
                    return true;
                }
                let hay = format!(
                    "{} {} {} {}",
                    f.manufacturer,
                    f.manufacturer_key,
                    f.name,
                    f.categories.join(" ")
                )
                .to_lowercase();
                words.iter().all(|w| hay.contains(w))
            })
            .take(limit)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_library_loads() {
        let lib = Library::embedded();
        assert!(lib.fixtures.len() > 500, "nur {} Geräte", lib.fixtures.len());
        assert!(lib.fixtures.iter().all(|f| !f.modes.is_empty()));
        assert!(!lib.search("cameo", 10).is_empty());
        assert!(lib.search("rgb par", 5).len() <= 5);
    }
}
