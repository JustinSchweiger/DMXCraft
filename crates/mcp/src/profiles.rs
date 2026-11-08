//! Profile Builder per MCP: eigene Geräteprofile auflisten, lesen, anlegen,
//! ändern und entfernen. Gespeichert wird über den [`ProfileStore`] der App
//! (dieselben Dateien wie im Profile Builder).

use dmxcraft_core::{Engine, FixtureType};
use dmxcraft_ofl::profile::{
    Capability, CapabilityKind, Problem, Profile, ProfileChannel, ProfileMode,
};
use serde_json::{json, Value};

use crate::ProfileStore;

type R = Result<Value, String>;

/// Gerätetypen aller eigenen Profile (für Bibliothekssuche und Patch).
pub fn user_types(store: &dyn ProfileStore) -> Vec<FixtureType> {
    store
        .load()
        .iter()
        .flat_map(|p| p.fixture_types().unwrap_or_default())
        .collect()
}

fn summary(p: &Profile) -> Value {
    let types = p.fixture_types().unwrap_or_default();
    json!({
        "key": p.key,
        "manufacturer": p.manufacturer,
        "name": p.name,
        "categories": p.categories,
        "channels": p.channels.len(),
        "modes": p.modes.iter().zip(types.iter().map(Some).chain(std::iter::repeat(None))).map(|(m, t)| json!({
            "name": m.name,
            "channels": m.channels.len(),
            "type_key": t.map(|t| t.key.clone()),
        })).collect::<Vec<_>>(),
        "problems": p.problems().iter().map(problem_text).collect::<Vec<_>>(),
    })
}

pub fn list(store: &dyn ProfileStore) -> R {
    Ok(json!({ "profiles": store.load().iter().map(summary).collect::<Vec<_>>() }))
}

pub fn get(store: &dyn ProfileStore, key: &str) -> R {
    let p = store
        .load()
        .into_iter()
        .find(|p| p.key == key)
        .ok_or_else(|| {
            format!("Eigenes Profil „{key}“ nicht gefunden – list_profiles zeigt alle")
        })?;
    Ok(json!({
        "profile": serde_json::to_value(&p).map_err(|e| e.to_string())?,
        "summary": summary(&p),
    }))
}

/// Vorlage und erlaubte Werte für `save_profile`.
pub fn format() -> R {
    let mut example = Profile {
        manufacturer: "Beispiel".into(),
        name: "LED Par RGBW".into(),
        categories: vec!["Color Changer".into()],
        ..Default::default()
    };
    let dimmer = ProfileChannel::new("Dimmer", Capability::new(CapabilityKind::Intensity));
    let mut strobe = ProfileChannel::new(
        "Strobe",
        Capability::new(CapabilityKind::NoFunction).range(0, 9),
    );
    let mut s = Capability::new(CapabilityKind::ShutterStrobe).range(10, 255);
    s.from = "slow".into();
    s.to = "fast".into();
    strobe.capabilities.push(s);
    let color = |n: &str, c: &str| {
        ProfileChannel::new(n, Capability::new(CapabilityKind::ColorIntensity).color(c))
    };
    example.channels = vec![
        dimmer,
        color("Red", "Red"),
        color("Green", "Green"),
        color("Blue", "Blue"),
        color("White", "White"),
        strobe,
    ];
    example.modes = vec![ProfileMode {
        id: 1,
        name: "6-Kanal".into(),
        short_name: "6ch".into(),
        channels: example.channels.iter().map(|c| c.name.clone()).collect(),
    }];
    Ok(json!({
        "example": serde_json::to_value(&example).map_err(|e| e.to_string())?,
        "notes": [
            "channels: jeder Kanal mit Namen und Bereichen (capabilities) von start bis end (0–255). Die Bereiche werden beim Speichern lückenlos gemacht (in der Reihenfolge, der erste ab 0, der letzte bis 255).",
            "fine: 1 = 16 Bit; der Feinkanal heißt dann „<Name> fine“ und wird im Modus eigens aufgeführt (z. B. \"Pan\", \"Pan fine\").",
            "modes: Reihenfolge der Kanäle je DMX-Modus (Kanalnamen, \"\" = unbenutzt). id fest je Modus, bestimmt den Gerätetyp-Schlüssel.",
            "kind ColorIntensity braucht color (Red, Green, Blue, White, Amber, UV, Cyan, Magenta, Yellow, Lime, Indigo, Warm White, Cold White).",
            "kind WheelSlot: slot (1-basiert), slot_kind (Color, Gobo, Open, …), label und hex (#rrggbb) für Farbfelder.",
            "Pan/Tilt brauchen from/to als Winkel, z. B. \"0deg\"/\"540deg\"; Geschwindigkeiten \"slow\"/\"fast\"; Prozent \"0%\"/\"100%\".",
            "key leer lassen beim Anlegen; zum Ändern den key des bestehenden Profils angeben (gepatchte Geräte werden mit aktualisiert).",
        ],
        "capability_kinds": CapabilityKind::ALL.iter().map(|k| json!({
            "kind": k,
            "value": k.property(),
            "required": k.property_required(),
            "defaults": k.default_values(),
        })).collect::<Vec<_>>(),
        "categories": ["Dimmer", "Color Changer", "Moving Head", "Scanner", "Strobe", "Blinder", "Pixel Bar", "Matrix", "Laser", "Smoke", "Hazer", "Fan", "Effect", "Flower", "Barrel Scanner", "Stand", "Other"],
    }))
}

/// Profil anlegen oder ändern. Bereiche werden lückenlos gemacht, Modi ohne
/// eindeutige ID bekommen eine. Gepatchte Geräte mit diesem Profil werden in
/// der Show aktualisiert.
pub fn save(store: &dyn ProfileStore, e: &mut Engine, a: &Value) -> R {
    let raw = a
        .get("profile")
        .ok_or("„profile“ fehlt – Vorlage siehe describe_profile_format")?;
    let mut p: Profile =
        serde_json::from_value(raw.clone()).map_err(|e| format!("Profil ungültig: {e}"))?;
    let existing = store.load();
    let old = existing
        .iter()
        .find(|x| !p.key.is_empty() && x.key == p.key);
    if !p.key.is_empty() && old.is_none() {
        return Err(format!(
            "Eigenes Profil „{}“ nicht gefunden – zum Anlegen key weglassen",
            p.key
        ));
    }
    for c in &mut p.channels {
        c.capabilities.sort_by_key(|k| k.start);
        c.normalize_ranges();
    }
    // Modus-IDs eindeutig (bestehende behalten)
    let mut used = std::collections::HashSet::new();
    let mut next = p.modes.iter().map(|m| m.id).max().unwrap_or(0) + 1;
    for m in &mut p.modes {
        if m.id == 0 || !used.insert(m.id) {
            m.id = next;
            used.insert(next);
            next += 1;
        }
    }
    if let Some(o) = old {
        if p.created.is_empty() {
            p.created = o.created.clone();
        }
    }
    let problems = p.problems();
    if !problems.is_empty() {
        return Err(format!(
            "Nicht gespeichert – {} Problem(e):\n- {}",
            problems.len(),
            problems
                .iter()
                .map(problem_text)
                .collect::<Vec<_>>()
                .join("\n- ")
        ));
    }
    p.fixture_types().map_err(|e| e.to_string())?;
    store.save(&mut p)?;
    let types = p.fixture_types().unwrap_or_default();
    let mut updated = 0;
    for ty in &types {
        if e.show().fixture_type(&ty.key).is_some() {
            e.show_mut().upsert_fixture_type(ty.clone());
            updated += e
                .show()
                .fixtures
                .iter()
                .filter(|f| f.type_key == ty.key)
                .count();
        }
    }
    if updated > 0 {
        e.sync();
    }
    Ok(json!({
        "saved": summary(&p),
        "patched_fixtures_updated": updated,
        "hint": "Mit add_fixtures und dem type_key eines Modus patchen.",
    }))
}

pub fn delete(store: &dyn ProfileStore, e: &Engine, key: &str) -> R {
    let p = store
        .load()
        .into_iter()
        .find(|p| p.key == key)
        .ok_or_else(|| format!("Eigenes Profil „{key}“ nicht gefunden"))?;
    let keys: Vec<String> = p
        .fixture_types()
        .unwrap_or_default()
        .into_iter()
        .map(|t| t.key)
        .collect();
    store.delete(key)?;
    // gepatchte Geräte behalten ihren Gerätetyp in der Show
    let still = e
        .show()
        .fixtures
        .iter()
        .filter(|f| keys.contains(&f.type_key))
        .count();
    Ok(json!({ "deleted": key, "patched_fixtures_keep_their_type": still }))
}

fn problem_text(p: &Problem) -> String {
    match p {
        Problem::NoManufacturer => "Hersteller fehlt".into(),
        Problem::NoName => "Name fehlt".into(),
        Problem::NoCategory => "Kategorie fehlt".into(),
        Problem::NoChannels => "keine Kanäle".into(),
        Problem::NoModes => "kein Modus".into(),
        Problem::EmptyChannelName => "Kanal ohne Namen".into(),
        Problem::DuplicateChannel(c) => format!("Kanalname doppelt: {c}"),
        Problem::NoCapabilities(c) => format!("Kanal {c}: keine Bereiche"),
        Problem::Gap(c, v) => format!("Kanal {c}: Wert {v} nicht abgedeckt"),
        Problem::Overlap(c, v) => format!("Kanal {c}: Bereiche überschneiden sich bei {v}"),
        Problem::MissingValue(c, v) => {
            format!("Kanal {c}: Bereich ab {v} braucht from/to (z. B. Winkel)")
        }
        Problem::EmptyModeName => "Modus ohne Namen".into(),
        Problem::DuplicateMode(m) => format!("Modusname doppelt: {m}"),
        Problem::EmptyMode(m) => format!("Modus {m}: keine Kanäle"),
        Problem::TooManyChannels(m) => format!("Modus {m}: mehr als 512 Kanäle"),
        Problem::UnknownChannel(m, c) => format!("Modus {m}: unbekannter Kanal {c}"),
        Problem::DuplicateInMode(m, c) => format!("Modus {m}: Kanal {c} doppelt"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MemoryProfiles;

    #[test]
    fn create_list_update_delete() {
        let store = MemoryProfiles::default();
        let mut e = Engine::new(dmxcraft_core::Show::new("T", 1));
        let ex = format().unwrap()["example"].clone();
        let r = save(&store, &mut e, &json!({ "profile": ex })).unwrap();
        let key = r["saved"]["key"].as_str().unwrap().to_string();
        assert!(!key.is_empty());
        let l = list(&store).unwrap();
        assert_eq!(l["profiles"].as_array().unwrap().len(), 1);
        // Gerätetyp ist über die Bibliothek erreichbar
        let types = user_types(&store);
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].channels.len(), 6);
        // ändern: Bereiche mit Lücke werden lückenlos
        let mut p = get(&store, &key).unwrap()["profile"].clone();
        p["name"] = json!("LED Par RGBW 2");
        p["channels"][5]["capabilities"][1]["start"] = json!(20);
        save(&store, &mut e, &json!({ "profile": p })).unwrap();
        let p = get(&store, &key).unwrap()["profile"].clone();
        assert_eq!(p["name"], "LED Par RGBW 2");
        assert_eq!(p["channels"][5]["capabilities"][1]["start"], 10);
        // fehlerhaft: kein Modus
        let mut bad = p.clone();
        bad["modes"] = json!([]);
        assert!(save(&store, &mut e, &json!({ "profile": bad }))
            .unwrap_err()
            .contains("kein Modus"));
        delete(&store, &e, &key).unwrap();
        assert!(store.load().is_empty());
    }
}
