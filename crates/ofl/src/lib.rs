//! Gerätebibliotheken: Open Fixture Library (<https://open-fixture-library.org>)
//! und QLC+ (<https://www.qlcplus.org>).
//!
//! * [`convert`] wandelt eine OFL-Gerätedatei (JSON) um, [`qxf`] eine
//!   QLC+-Gerätedatei (XML) – jeweils ein [`FixtureType`] pro DMX-Modus.
//! * [`Library`] ist die eingebettete, vorkonvertierte Gesamtbibliothek aus
//!   `data/ofl-library.json.gz` (Tool `ofl-bundle`) und
//!   `data/qlc-library.json.gz` (Tool `qlc-bundle`).
//!
//! Lizenzen: OFL-Daten MIT (`data/OFL-LICENSE`), QLC+-Daten Apache 2.0
//! (`data/QLC-LICENSE`, `data/QLC-NOTICE`).

pub mod convert;
pub mod qxf;

use std::io::Read;
use std::sync::OnceLock;

use dmxcraft_core::FixtureType;
use serde::{Deserialize, Serialize};

pub use convert::{convert_fixture, OflError};
pub use qxf::convert_qxf;

/// Herkunft eines Bibliotheksgeräts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    #[default]
    Ofl,
    Qlc,
}

impl Source {
    /// Kurzname für die Anzeige.
    pub fn label(self) -> &'static str {
        match self {
            Source::Ofl => "OFL",
            Source::Qlc => "QLC+",
        }
    }
}

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
    #[serde(default)]
    pub source: Source,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Library {
    /// Git-Commit der OFL, aus dem die Bibliothek erzeugt wurde.
    pub source_commit: String,
    pub source_date: String,
    pub fixtures: Vec<LibraryFixture>,
}

static EMBEDDED_OFL: &[u8] = include_bytes!("../data/ofl-library.json.gz");
static EMBEDDED_QLC: &[u8] = include_bytes!("../data/qlc-library.json.gz");

impl Library {
    /// Die mit dem Programm ausgelieferte Bibliothek: OFL und QLC+ zusammen,
    /// nach Hersteller und Name sortiert (wird beim ersten Zugriff entpackt).
    pub fn embedded() -> &'static Library {
        static LIB: OnceLock<Library> = OnceLock::new();
        LIB.get_or_init(|| {
            let mut lib = Library::from_gzip(EMBEDDED_OFL).unwrap_or_default();
            if let Ok(qlc) = Library::from_gzip(EMBEDDED_QLC) {
                lib.fixtures.extend(qlc.fixtures.into_iter().map(|mut f| {
                    f.source = Source::Qlc;
                    f
                }));
            }
            lib.fixtures.sort_by(|a, b| {
                (
                    a.manufacturer.to_lowercase(),
                    a.name.to_lowercase(),
                    a.source as u8,
                )
                    .cmp(&(
                        b.manufacturer.to_lowercase(),
                        b.name.to_lowercase(),
                        b.source as u8,
                    ))
            });
            lib
        })
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
        let words: Vec<String> = query.split_whitespace().map(|w| w.to_lowercase()).collect();
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
    fn embedded_library_has_ofl_and_qlc() {
        let lib = Library::embedded();
        let qlc = lib
            .fixtures
            .iter()
            .filter(|f| f.source == Source::Qlc)
            .count();
        let ofl = lib
            .fixtures
            .iter()
            .filter(|f| f.source == Source::Ofl)
            .count();
        assert!(qlc > 1000, "nur {qlc} QLC+-Geräte");
        assert!(ofl > 500, "nur {ofl} OFL-Geräte");
        // Schlüssel der Modi sind eindeutig (Shows finden ihren Typ wieder)
        let mut keys = std::collections::HashSet::new();
        for f in &lib.fixtures {
            for m in &f.modes {
                assert!(keys.insert(m.key.as_str()), "doppelter Schlüssel {}", m.key);
            }
        }
    }

    #[test]
    fn embedded_library_loads() {
        let lib = Library::embedded();
        assert!(
            lib.fixtures.len() > 500,
            "nur {} Geräte",
            lib.fixtures.len()
        );
        assert!(lib.fixtures.iter().all(|f| !f.modes.is_empty()));
        assert!(!lib.search("cameo", 10).is_empty());
        assert!(lib.search("rgb par", 5).len() <= 5);
    }
}
