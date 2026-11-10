//! Profile Builder via MCP: list, read, create, change and remove custom
//! fixture profiles. Saving goes through the app's [`ProfileStore`] (the
//! same files as in the Profile Builder).

use dmxcraft_core::{Engine, FixtureType};
use dmxcraft_ofl::profile::{
    Capability, CapabilityKind, Problem, Profile, ProfileChannel, ProfileMode,
};
use serde_json::{json, Value};

use crate::ProfileStore;

type R = Result<Value, String>;

/// Fixture types of all custom profiles (for library search and patch).
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
        .ok_or_else(|| format!("Custom profile “{key}” not found – list_profiles shows all"))?;
    Ok(json!({
        "profile": serde_json::to_value(&p).map_err(|e| e.to_string())?,
        "summary": summary(&p),
    }))
}

/// Template and allowed values for `save_profile`.
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
        name: "6 channel".into(),
        short_name: "6ch".into(),
        channels: example.channels.iter().map(|c| c.name.clone()).collect(),
    }];
    Ok(json!({
        "example": serde_json::to_value(&example).map_err(|e| e.to_string())?,
        "notes": [
            "channels: each channel with name and ranges (capabilities) from start to end (0–255). The ranges are made gapless when saving (in order, the first from 0, the last up to 255).",
            "fine: 1 = 16 bit; the fine channel is then called “<name> fine” and is listed separately in the mode (e.g. \"Pan\", \"Pan fine\").",
            "modes: order of the channels per DMX mode (channel names, \"\" = unused). id is fixed per mode and determines the fixture type key.",
            "kind ColorIntensity needs color (Red, Green, Blue, White, Amber, UV, Cyan, Magenta, Yellow, Lime, Indigo, Warm White, Cold White).",
            "kind WheelSlot: slot (1-based), slot_kind (Color, Gobo, Open, …), label and hex (#rrggbb) for color slots.",
            "Pan/Tilt need from/to as angles, e.g. \"0deg\"/\"540deg\"; speeds \"slow\"/\"fast\"; percent \"0%\"/\"100%\".",
            "leave key empty when creating; to change, give the key of the existing profile (patched fixtures are updated too).",
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

/// Create or change a profile. Ranges are made gapless, modes without a
/// unique ID get one. Patched fixtures with this profile are updated in the
/// show.
pub fn save(store: &dyn ProfileStore, e: &mut Engine, a: &Value) -> R {
    let raw = a
        .get("profile")
        .ok_or("“profile” missing – see describe_profile_format for a template")?;
    let mut p: Profile =
        serde_json::from_value(raw.clone()).map_err(|e| format!("Invalid profile: {e}"))?;
    let existing = store.load();
    let old = existing
        .iter()
        .find(|x| !p.key.is_empty() && x.key == p.key);
    if !p.key.is_empty() && old.is_none() {
        return Err(format!(
            "Custom profile “{}” not found – leave out key to create one",
            p.key
        ));
    }
    for c in &mut p.channels {
        c.capabilities.sort_by_key(|k| k.start);
        c.normalize_ranges();
    }
    // Mode IDs unique (keep existing ones)
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
            "Not saved – {} problem(s):\n- {}",
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
        "hint": "Patch with add_fixtures and the type_key of a mode.",
    }))
}

pub fn delete(store: &dyn ProfileStore, e: &Engine, key: &str) -> R {
    let p = store
        .load()
        .into_iter()
        .find(|p| p.key == key)
        .ok_or_else(|| format!("Custom profile “{key}” not found"))?;
    let keys: Vec<String> = p
        .fixture_types()
        .unwrap_or_default()
        .into_iter()
        .map(|t| t.key)
        .collect();
    store.delete(key)?;
    // patched fixtures keep their fixture type in the show
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
        Problem::NoManufacturer => "Manufacturer missing".into(),
        Problem::NoName => "Name missing".into(),
        Problem::NoCategory => "Category missing".into(),
        Problem::NoChannels => "no channels".into(),
        Problem::NoModes => "no mode".into(),
        Problem::EmptyChannelName => "Channel without a name".into(),
        Problem::DuplicateChannel(c) => format!("Duplicate channel name: {c}"),
        Problem::NoCapabilities(c) => format!("Channel {c}: no ranges"),
        Problem::Gap(c, v) => format!("Channel {c}: value {v} not covered"),
        Problem::Overlap(c, v) => format!("Channel {c}: ranges overlap at {v}"),
        Problem::MissingValue(c, v) => {
            format!("Channel {c}: range from {v} needs from/to (e.g. angle)")
        }
        Problem::EmptyModeName => "Mode without a name".into(),
        Problem::DuplicateMode(m) => format!("Duplicate mode name: {m}"),
        Problem::EmptyMode(m) => format!("Mode {m}: no channels"),
        Problem::TooManyChannels(m) => format!("Mode {m}: more than 512 channels"),
        Problem::UnknownChannel(m, c) => format!("Mode {m}: unknown channel {c}"),
        Problem::DuplicateInMode(m, c) => format!("Mode {m}: channel {c} duplicated"),
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
        // The fixture type is reachable through the library
        let types = user_types(&store);
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].channels.len(), 6);
        // change: ranges with a gap become gapless
        let mut p = get(&store, &key).unwrap()["profile"].clone();
        p["name"] = json!("LED Par RGBW 2");
        p["channels"][5]["capabilities"][1]["start"] = json!(20);
        save(&store, &mut e, &json!({ "profile": p })).unwrap();
        let p = get(&store, &key).unwrap()["profile"].clone();
        assert_eq!(p["name"], "LED Par RGBW 2");
        assert_eq!(p["channels"][5]["capabilities"][1]["start"], 10);
        // invalid: no mode
        let mut bad = p.clone();
        bad["modes"] = json!([]);
        assert!(save(&store, &mut e, &json!({ "profile": bad }))
            .unwrap_err()
            .contains("no mode"));
        delete(&store, &e, &key).unwrap();
        assert!(store.load().is_empty());
    }
}
