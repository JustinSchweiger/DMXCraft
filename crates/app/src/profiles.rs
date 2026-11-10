//! Eigene Geräteprofile (Profile Builder) im Konfigurationsordner:
//! `<config>/fixtures/<schlüssel>.dmxfixture`.

use std::path::PathBuf;

use dmxcraft_core::FixtureType;
use dmxcraft_ofl::profile::PROFILE_EXT;
use dmxcraft_ofl::Profile;

use crate::i18n::t;

pub fn dir() -> Option<PathBuf> {
    crate::settings::Settings::path().map(|p| p.with_file_name("fixtures"))
}

fn file(key: &str) -> Option<PathBuf> {
    dir().map(|d| d.join(format!("{key}.{PROFILE_EXT}")))
}

/// Ein gespeichertes Profil mit seinen Gerätetypen (für Bibliothek und Patch).
#[derive(Clone, Debug)]
pub struct UserProfile {
    pub profile: Profile,
    pub types: Vec<FixtureType>,
}

impl UserProfile {
    pub fn new(profile: Profile) -> Self {
        let types = profile.fixture_types().unwrap_or_default();
        Self { profile, types }
    }
}

/// Alle gespeicherten Profile mit Gerätetypen.
pub fn load_user() -> Vec<UserProfile> {
    let mut list: Vec<UserProfile> = load_all().into_iter().map(UserProfile::new).collect();
    sort_user(&mut list);
    list
}

pub fn sort_user(list: &mut [UserProfile]) {
    list.sort_by_key(|u| {
        (
            u.profile.manufacturer.to_lowercase(),
            u.profile.name.to_lowercase(),
        )
    });
}

/// Alle gespeicherten Profile, nach Hersteller und Name sortiert.
pub fn load_all() -> Vec<Profile> {
    let Some(dir) = dir() else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut out: Vec<Profile> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == PROFILE_EXT))
        .filter_map(|p| {
            let mut prof = Profile::from_json(&std::fs::read_to_string(&p).ok()?).ok()?;
            if prof.key.is_empty() {
                prof.key = p.file_stem()?.to_string_lossy().to_string();
            }
            Some(prof)
        })
        .collect();
    sort(&mut out);
    out
}

pub fn sort(list: &mut [Profile]) {
    list.sort_by_key(|p| (p.manufacturer.to_lowercase(), p.name.to_lowercase()));
}

/// Vergibt beim ersten Speichern einen freien Schlüssel und schreibt die Datei.
pub fn save(p: &mut Profile, taken: &[Profile]) -> Result<PathBuf, String> {
    if p.key.is_empty() {
        let base = p.suggested_key();
        let free =
            |k: &str| !taken.iter().any(|x| x.key == k) && file(k).is_some_and(|f| !f.exists());
        let mut key = base.clone();
        let mut n = 2;
        while !free(&key) {
            key = format!("{base}-{n}");
            n += 1;
        }
        p.key = key;
    }
    let path = file(&p.key).ok_or(t("No configuration folder found"))?;
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d).map_err(|e| e.to_string())?;
    }
    std::fs::write(&path, p.to_json()).map_err(|e| e.to_string())?;
    Ok(path)
}

/// Entfernt ein Profil aus der Bibliothek. Die Datei wandert nach
/// `fixtures/geloescht/`, damit nichts endgültig verloren geht.
pub fn delete(key: &str) -> Result<(), String> {
    let (Some(f), Some(d)) = (file(key), dir()) else {
        return Ok(());
    };
    if !f.exists() {
        return Ok(());
    }
    let trash = d.join("geloescht");
    std::fs::create_dir_all(&trash).map_err(|e| e.to_string())?;
    let target = trash.join(format!("{key}-{}.{PROFILE_EXT}", today()));
    std::fs::rename(&f, target).map_err(|e| e.to_string())
}

/// Heutiges Datum JJJJ-MM-TT (UTC, ohne zusätzliche Abhängigkeit).
pub fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    let days = (secs / 86_400) as i64;
    // Howard Hinnant, civil_from_days
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn today_is_a_date() {
        let d = super::today();
        assert_eq!(d.len(), 10);
        assert!(d.starts_with("20"));
    }
}
