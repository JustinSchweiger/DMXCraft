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
pub mod profile;
pub mod qxf;

use std::io::Read;
use std::sync::OnceLock;

use dmxcraft_core::FixtureType;
use serde::{Deserialize, Serialize};

pub use convert::{convert_fixture, OflError};
pub use profile::{Capability, CapabilityKind, Problem, Profile, ProfileChannel, ProfileMode};
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

/// Ergänzt bei Gerätetypen einer Show die physischen Angaben (Abstrahlwinkel,
/// Pan/Tilt-Bereich) aus der eingebauten Bibliothek, falls sie fehlen
/// (ältere Shows wurden ohne diese Angaben gepatcht).
pub fn fill_physical(show: &mut dmxcraft_core::Show) {
    fill_details(show, &[]);
}

/// Ergänzt in einer Show fehlende Angaben der Gerätetypen aus der
/// Bibliothek bzw. `extra` (eigene Profile), gefunden über den Schlüssel:
/// physische Daten und Wertebereiche der Kanäle (ältere Shows haben sie nicht).
pub fn fill_details(show: &mut dmxcraft_core::Show, extra: &[FixtureType]) {
    let needs =
        |t: &FixtureType| t.physical.is_none() || t.channels.iter().all(|c| c.ranges.is_empty());
    if !show.fixture_types.iter().any(needs) {
        return;
    }
    let lib = Library::embedded();
    let builtin = dmxcraft_core::library::builtin_fixture_types();
    for t in show.fixture_types.iter_mut().filter(|t| needs(t)) {
        let Some(src) = extra
            .iter()
            .chain(&builtin)
            .find(|m| m.key == t.key)
            .or_else(|| {
                lib.fixtures
                    .iter()
                    .flat_map(|f| f.modes.iter())
                    .find(|m| m.key == t.key)
            })
        else {
            continue;
        };
        if t.physical.is_none() {
            t.physical = src.physical;
        }
        // Bereiche nur übernehmen, wenn die Kanäle zusammenpassen
        if t.channels.iter().all(|c| c.ranges.is_empty())
            && t.channels.len() == src.channels.len()
            && t.channels
                .iter()
                .zip(&src.channels)
                .all(|(a, b)| a.name == b.name)
        {
            for (a, b) in t.channels.iter_mut().zip(&src.channels) {
                a.ranges = b.ranges.clone();
            }
        }
    }
}

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
    /// Bester Gerätetyp zu einem Namen aus einer fremden Bibliothek, z. B.
    /// „Stairville/LED PAR 64 CX_3 RGBW.ssl2“ (Hersteller/Modell). Enthält der
    /// Name eine Kanalzahl („7CH“), wird der passende Modus bevorzugt.
    /// `None`, wenn Hersteller oder Modell nicht ausreichend übereinstimmen.
    pub fn best_match(&self, library_name: &str) -> Option<(&LibraryFixture, &FixtureType)> {
        let name = library_name
            .rsplit_once('.')
            .filter(|(_, ext)| ext.len() <= 4 && !ext.contains(' '))
            .map_or(library_name, |(n, _)| n);
        let (maker, model) = name.rsplit_once('/').unwrap_or(("", name));
        let tokens = |s: &str| -> Vec<String> {
            s.to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| !w.is_empty())
                .map(str::to_string)
                .collect()
        };
        // „_Generic“, „_imported“: kein echter Hersteller, er steckt dann im Modellnamen
        let maker = match maker.to_lowercase().as_str() {
            "_generic" => vec!["generic".to_string()],
            m if m.starts_with('_') => Vec::new(),
            _ => tokens(maker),
        };
        let model_t = tokens(model);
        if model_t.is_empty() {
            return None;
        }
        let channels: Option<usize> = model_t.iter().find_map(|w| {
            w.strip_suffix("ch")
                .and_then(|n| n.parse().ok())
                .filter(|n: &usize| (1..=512).contains(n))
        });
        let mut best: Option<(f32, &LibraryFixture)> = None;
        for f in &self.fixtures {
            let fm = tokens(&format!("{} {}", f.manufacturer, f.manufacturer_key));
            let maker_ok = maker.is_empty() || maker.iter().any(|w| fm.contains(w));
            if !maker_ok {
                continue;
            }
            let fn_t = if maker.is_empty() {
                tokens(&format!("{} {}", f.manufacturer, f.name))
            } else {
                tokens(&f.name)
            };
            let hits = model_t.iter().filter(|w| fn_t.contains(w)).count() as f32;
            // Anteil getroffener Wörter beider Seiten
            let score = hits / model_t.len() as f32 * 0.6 + hits / fn_t.len().max(1) as f32 * 0.4;
            if best.is_none_or(|b| score > b.0) {
                best = Some((score, f));
            }
        }
        let (score, f) = best?;
        if score < 0.55 {
            return None;
        }
        let mode = channels
            .and_then(|n| f.modes.iter().find(|m| m.channels.len() == n))
            .or_else(|| f.modes.iter().max_by_key(|m| m.channels.len()))?;
        Some((f, mode))
    }

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

    #[test]
    fn best_match_finds_easy_view_names() {
        let lib = Library::embedded();
        let (f, m) = lib
            .best_match("Stairville/PAR 56 7CH.ssl2")
            .or_else(|| lib.best_match("Stairville/LED PAR 56.ssl2"))
            .expect("Stairville");
        assert!(f.manufacturer.contains("Stairville"), "{}", f.manufacturer);
        assert!(!m.channels.is_empty());
        assert!(lib
            .best_match("Unbekannt/Gibt es nicht 9000.ssl2")
            .is_none());
    }

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
