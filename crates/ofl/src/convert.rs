//! Umwandlung einer OFL-Gerätedatei in DmxCraft-Gerätetypen.
//!
//! Unterstützt: Kanäle mit `capability`/`capabilities`, feine Kanäle
//! (`fineChannelAliases`), Standardwerte (Zahl oder Prozent), Matrix-Geräte
//! (`pixelCount`, `pixelKeys`, `pixelGroups`, `templateChannels`,
//! `insert: matrixChannels`) und Schaltkanäle (`switchChannels`).
//! Unbekannte Kanalnamen werden als generische Kanäle übernommen, damit die
//! Kanalzahl des Modus immer stimmt.

use std::collections::HashMap;

use dmxcraft_core::{ChannelDef, ChannelKind, FixtureType};
use serde_json::{Map, Value};
use thiserror::Error;

use crate::LibraryFixture;

#[derive(Debug, Error)]
pub enum OflError {
    #[error("Ungültiges JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Übersprungen: {0}")]
    Skip(&'static str),
    #[error("Ungültige OFL-Datei: {0}")]
    Invalid(String),
}

/// Ein Pixel einer Matrix mit Position (x, y, z – 0-basiert).
#[derive(Clone, Debug)]
struct Pixel {
    key: String,
    pos: [usize; 3],
}

struct Matrix {
    pixels: Vec<Pixel>,
    groups: Vec<(String, Vec<String>)>,
}

impl Matrix {
    fn parse(fixture: &Value) -> Option<Matrix> {
        let m = fixture.get("matrix")?;
        let mut pixels = Vec::new();
        if let Some(keys) = m.get("pixelKeys").and_then(Value::as_array) {
            // [z][y][x]
            for (z, plane) in keys.iter().enumerate() {
                for (y, row) in plane.as_array().into_iter().flatten().enumerate() {
                    for (x, k) in row.as_array().into_iter().flatten().enumerate() {
                        if let Some(k) = k.as_str() {
                            pixels.push(Pixel {
                                key: k.to_string(),
                                pos: [x, y, z],
                            });
                        }
                    }
                }
            }
        } else if let Some(count) = m.get("pixelCount").and_then(Value::as_array) {
            let dims: Vec<usize> = (0..3)
                .map(|i| count.get(i).and_then(Value::as_u64).unwrap_or(1).max(1) as usize)
                .collect();
            let multi: Vec<usize> = (0..3).filter(|&i| dims[i] > 1).collect();
            for z in 0..dims[2] {
                for y in 0..dims[1] {
                    for x in 0..dims[0] {
                        let pos = [x, y, z];
                        let key = match multi.len() {
                            0 => "1".to_string(),
                            1 => (pos[multi[0]] + 1).to_string(),
                            2 => format!("({}, {})", pos[multi[0]] + 1, pos[multi[1]] + 1),
                            _ => format!("({}, {}, {})", x + 1, y + 1, z + 1),
                        };
                        pixels.push(Pixel { key, pos });
                    }
                }
            }
        }
        let groups = m
            .get("pixelGroups")
            .and_then(Value::as_object)
            .map(|g| {
                g.iter()
                    .map(|(name, members)| {
                        let list = members
                            .as_array()
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default();
                        (name.clone(), list)
                    })
                    .collect()
            })
            .unwrap_or_default();
        Some(Matrix { pixels, groups })
    }

    fn all_keys(&self) -> impl Iterator<Item = &str> {
        self.pixels
            .iter()
            .map(|p| p.key.as_str())
            .chain(self.groups.iter().map(|(g, _)| g.as_str()))
    }

    /// Reihenfolge der Pixel/Gruppen für ein `repeatFor`.
    fn order(&self, repeat_for: &Value) -> Vec<String> {
        if let Some(list) = repeat_for.as_array() {
            return list
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }
        let spec = repeat_for.as_str().unwrap_or("eachPixelABC");
        match spec {
            "eachPixelGroup" => self.groups.iter().map(|(g, _)| g.clone()).collect(),
            "eachPixelABC" => {
                let mut keys: Vec<String> = self.pixels.iter().map(|p| p.key.clone()).collect();
                keys.sort_by(|a, b| natural_cmp(a, b));
                keys
            }
            s if s.starts_with("eachPixel") && s.len() == 12 => {
                // z. B. eachPixelXYZ: X läuft am schnellsten, dann Y, dann Z.
                let axis = |c: char| match c {
                    'X' => 0,
                    'Y' => 1,
                    _ => 2,
                };
                let letters: Vec<usize> = s[9..].chars().map(axis).collect();
                let mut px = self.pixels.clone();
                px.sort_by_key(|p| (p.pos[letters[2]], p.pos[letters[1]], p.pos[letters[0]]));
                px.into_iter().map(|p| p.key).collect()
            }
            _ => self.pixels.iter().map(|p| p.key.clone()).collect(),
        }
    }
}

/// Vergleich, bei dem Zahlen numerisch sortiert werden („2“ < „10“).
fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let key = |s: &str| -> Vec<(u8, u64, String)> {
        let mut out = Vec::new();
        let mut num = String::new();
        let mut txt = String::new();
        for c in s.chars() {
            if c.is_ascii_digit() {
                if !txt.is_empty() {
                    out.push((1, 0, std::mem::take(&mut txt)));
                }
                num.push(c);
            } else {
                if !num.is_empty() {
                    out.push((0, num.parse().unwrap_or(u64::MAX), String::new()));
                    num.clear();
                }
                txt.push(c);
            }
        }
        if !num.is_empty() {
            out.push((0, num.parse().unwrap_or(u64::MAX), String::new()));
        }
        if !txt.is_empty() {
            out.push((1, 0, txt));
        }
        out
    };
    key(a).cmp(&key(b))
}

/// Ein aufgelöster Kanal: OFL-Kanaldefinition + ob es der n-te feine Kanal ist.
#[derive(Clone, Copy)]
struct Resolved<'a> {
    def: &'a Value,
    fine: usize,
    fine_count: usize,
}

fn fine_aliases(ch: &Value) -> Vec<&str> {
    ch.get("fineChannelAliases")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default()
}

fn capabilities(ch: &Value) -> Vec<&Value> {
    if let Some(c) = ch.get("capability") {
        vec![c]
    } else {
        ch.get("capabilities")
            .and_then(Value::as_array)
            .map(|a| a.iter().collect())
            .unwrap_or_default()
    }
}

struct Lookup<'a> {
    channels: HashMap<String, Resolved<'a>>,
    switch_alias: HashMap<String, String>,
}

impl<'a> Lookup<'a> {
    fn build(fixture: &'a Value, matrix: Option<&Matrix>) -> Lookup<'a> {
        let mut channels = HashMap::new();
        let mut switch_alias = HashMap::new();
        let mut add = |name: String, def: &'a Value| {
            let fines = fine_aliases(def);
            channels.insert(
                name,
                Resolved {
                    def,
                    fine: 0,
                    fine_count: fines.len(),
                },
            );
            (fines, def)
        };

        static EMPTY: std::sync::OnceLock<Map<String, Value>> = std::sync::OnceLock::new();
        let available = fixture
            .get("availableChannels")
            .and_then(Value::as_object)
            .unwrap_or_else(|| EMPTY.get_or_init(Map::new));
        let mut fine_entries: Vec<(String, Resolved<'a>)> = Vec::new();
        for (name, def) in available {
            let (fines, def) = add(name.clone(), def);
            for (i, alias) in fines.iter().enumerate() {
                fine_entries.push((
                    alias.to_string(),
                    Resolved {
                        def,
                        fine: i + 1,
                        fine_count: fines.len(),
                    },
                ));
            }
            for cap in capabilities(def) {
                if let Some(sw) = cap.get("switchChannels").and_then(Value::as_object) {
                    for (alias, target) in sw {
                        if let Some(t) = target.as_str() {
                            switch_alias
                                .entry(alias.clone())
                                .or_insert_with(|| t.to_string());
                        }
                    }
                }
            }
        }

        if let (Some(templates), Some(matrix)) = (
            fixture.get("templateChannels").and_then(Value::as_object),
            matrix,
        ) {
            for key in matrix.all_keys() {
                for (tname, def) in templates {
                    let (fines, def) = add(tname.replace("$pixelKey", key), def);
                    for (i, alias) in fines.iter().enumerate() {
                        fine_entries.push((
                            alias.replace("$pixelKey", key),
                            Resolved {
                                def,
                                fine: i + 1,
                                fine_count: fines.len(),
                            },
                        ));
                    }
                }
            }
        }
        for (name, r) in fine_entries {
            channels.entry(name).or_insert(r);
        }
        Lookup {
            channels,
            switch_alias,
        }
    }

    fn resolve(&self, name: &str) -> Option<Resolved<'a>> {
        self.channels.get(name).copied().or_else(|| {
            self.switch_alias
                .get(name)
                .and_then(|t| self.channels.get(t).copied())
        })
    }
}

fn wheel_kind(fixture: &Value, wheel: &str) -> ChannelKind {
    let slots = fixture
        .get("wheels")
        .and_then(|w| w.get(wheel))
        .and_then(|w| w.get("slots"))
        .and_then(Value::as_array);
    let Some(slots) = slots else {
        return ChannelKind::Generic;
    };
    let count = |t: &str| {
        slots
            .iter()
            .filter(|s| {
                s.get("type")
                    .and_then(Value::as_str)
                    .is_some_and(|x| x.contains(t))
            })
            .count()
    };
    if count("Color") >= count("Gobo") && count("Color") > 0 {
        ChannelKind::ColorWheel
    } else if count("Gobo") > 0 {
        ChannelKind::Gobo
    } else {
        ChannelKind::Generic
    }
}

fn channel_kind(fixture: &Value, name: &str, r: &Resolved<'_>) -> ChannelKind {
    let caps = capabilities(r.def);
    let main = caps
        .iter()
        .find(|c| c.get("type").and_then(Value::as_str) != Some("NoFunction"))
        .or(caps.first());
    let Some(cap) = main else {
        return ChannelKind::Generic;
    };
    let ty = cap.get("type").and_then(Value::as_str).unwrap_or("");
    if r.fine > 0 {
        return match ty {
            "Pan" | "PanContinuous" => ChannelKind::PanFine,
            "Tilt" | "TiltContinuous" => ChannelKind::TiltFine,
            _ => ChannelKind::Generic,
        };
    }
    match ty {
        "Intensity" => ChannelKind::Dimmer,
        "ColorIntensity" => match cap.get("color").and_then(Value::as_str).unwrap_or("") {
            "Red" => ChannelKind::Red,
            "Green" => ChannelKind::Green,
            "Blue" => ChannelKind::Blue,
            "White" | "Warm White" | "Cold White" => ChannelKind::White,
            "Amber" => ChannelKind::Amber,
            "UV" => ChannelKind::Uv,
            _ => ChannelKind::Generic,
        },
        "ShutterStrobe" | "StrobeSpeed" | "StrobeDuration" => ChannelKind::Strobe,
        "Pan" | "PanContinuous" => ChannelKind::Pan,
        "Tilt" | "TiltContinuous" => ChannelKind::Tilt,
        "ColorPreset" => ChannelKind::ColorWheel,
        "WheelSlot" => {
            let wheel = cap
                .get("wheel")
                .and_then(Value::as_str)
                .unwrap_or(name)
                .to_string();
            match wheel_kind(fixture, &wheel) {
                ChannelKind::Generic if name.to_lowercase().contains("gobo") => ChannelKind::Gobo,
                ChannelKind::Generic if name.to_lowercase().contains("colo") => {
                    ChannelKind::ColorWheel
                }
                k => k,
            }
        }
        "EffectSpeed" | "PanTiltSpeed" | "Speed" => ChannelKind::Speed,
        _ => ChannelKind::Generic,
    }
}

fn default_value(r: &Resolved<'_>) -> u8 {
    if r.fine > 0 {
        return 0;
    }
    match r.def.get("defaultValue") {
        Some(Value::Number(n)) => {
            let v = n.as_u64().unwrap_or(0);
            if v > 255 && r.fine_count > 0 {
                // In 16/24-Bit-Auflösung angegeben → nur das grobe Byte.
                (v >> (8 * r.fine_count)).min(255) as u8
            } else {
                v.min(255) as u8
            }
        }
        Some(Value::String(s)) => s
            .trim_end_matches('%')
            .trim()
            .parse::<f64>()
            .map(|p| (p.clamp(0.0, 100.0) / 100.0 * 255.0).round() as u8)
            .unwrap_or(0),
        _ => 0,
    }
}

fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Wandelt eine OFL-Gerätedatei um.
///
/// `manufacturer_key` / `fixture_key` sind Ordner- und Dateiname in der OFL
/// (z. B. `cameo` / `flat-pro-18`), `manufacturer` der Anzeigename.
pub fn convert_fixture(
    manufacturer_key: &str,
    manufacturer: &str,
    fixture_key: &str,
    json: &str,
) -> Result<LibraryFixture, OflError> {
    let fixture: Value = serde_json::from_str(json)?;
    if fixture.get("redirectTo").is_some() {
        return Err(OflError::Skip("Weiterleitung auf anderes Gerät"));
    }
    let name = fixture
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| OflError::Invalid("kein Name".into()))?
        .to_string();
    let modes = fixture
        .get("modes")
        .and_then(Value::as_array)
        .ok_or_else(|| OflError::Invalid("keine Modi".into()))?;
    let categories = fixture
        .get("categories")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let matrix = Matrix::parse(&fixture);
    let lookup = Lookup::build(&fixture, matrix.as_ref());

    let make_channel = |ch_name: &str| -> ChannelDef {
        match lookup.resolve(ch_name) {
            Some(r) => ChannelDef {
                name: ch_name.to_string(),
                kind: channel_kind(&fixture, ch_name, &r),
                default: default_value(&r),
            },
            None => ChannelDef::new(ch_name, ChannelKind::Generic),
        }
    };

    let mut used_keys: HashMap<String, usize> = HashMap::new();
    let mut out_modes = Vec::new();
    for mode in modes {
        let mode_name = mode
            .get("name")
            .and_then(Value::as_str)
            .unwrap_or("Standard")
            .to_string();
        let short = mode
            .get("shortName")
            .and_then(Value::as_str)
            .unwrap_or(&mode_name)
            .to_string();
        let mut channels = Vec::new();
        for entry in mode
            .get("channels")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            match entry {
                Value::Null => channels.push(ChannelDef::new("(unbenutzt)", ChannelKind::Generic)),
                Value::String(s) => channels.push(make_channel(s)),
                Value::Object(o)
                    if o.get("insert").and_then(Value::as_str) == Some("matrixChannels") =>
                {
                    let Some(m) = matrix.as_ref() else {
                        return Err(OflError::Invalid("matrixChannels ohne matrix".into()));
                    };
                    let keys = m.order(o.get("repeatFor").unwrap_or(&Value::Null));
                    let templates: Vec<&str> = o
                        .get("templateChannels")
                        .and_then(Value::as_array)
                        .map(|a| a.iter().map(|v| v.as_str().unwrap_or("")).collect())
                        .unwrap_or_default();
                    let per_channel =
                        o.get("channelOrder").and_then(Value::as_str) == Some("perChannel");
                    let names: Vec<String> = if per_channel {
                        templates
                            .iter()
                            .flat_map(|t| keys.iter().map(move |k| t.replace("$pixelKey", k)))
                            .collect()
                    } else {
                        keys.iter()
                            .flat_map(|k| templates.iter().map(move |t| t.replace("$pixelKey", k)))
                            .collect()
                    };
                    channels.extend(names.iter().map(|n| make_channel(n)));
                }
                _ => {
                    return Err(OflError::Invalid(format!(
                        "unbekannter Kanaleintrag in {mode_name}"
                    )))
                }
            }
        }
        if channels.is_empty() || channels.len() > 512 {
            continue;
        }
        let base = format!("ofl/{manufacturer_key}/{fixture_key}/{}", slug(&short));
        let n = used_keys.entry(base.clone()).or_insert(0);
        *n += 1;
        let key = if *n == 1 { base } else { format!("{base}-{n}") };
        out_modes.push(FixtureType {
            key,
            manufacturer: manufacturer.to_string(),
            model: name.clone(),
            mode: mode_name,
            channels,
        });
    }
    if out_modes.is_empty() {
        return Err(OflError::Invalid("kein verwendbarer Modus".into()));
    }
    Ok(LibraryFixture {
        manufacturer_key: manufacturer_key.to_string(),
        manufacturer: manufacturer.to_string(),
        key: fixture_key.to_string(),
        name,
        categories,
        modes: out_modes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ChannelKind as K;

    fn kinds(t: &FixtureType) -> Vec<K> {
        t.channels.iter().map(|c| c.kind).collect()
    }

    #[test]
    fn simple_par_with_modes_defaults_and_null() {
        let json = r#"{
          "name": "Test PAR", "categories": ["Color Changer"],
          "availableChannels": {
            "Dimmer": {"defaultValue": "100%", "capability": {"type": "Intensity"}},
            "Red": {"capability": {"type": "ColorIntensity", "color": "Red"}},
            "Green": {"capability": {"type": "ColorIntensity", "color": "Green"}},
            "Blue": {"capability": {"type": "ColorIntensity", "color": "Blue"}},
            "Strobe": {"defaultValue": 0, "capabilities": [
              {"dmxRange": [0, 9], "type": "NoFunction"},
              {"dmxRange": [10, 255], "type": "ShutterStrobe", "shutterEffect": "Strobe"}]}
          },
          "modes": [
            {"name": "3-channel", "shortName": "3ch", "channels": ["Red", "Green", "Blue"]},
            {"name": "6-channel", "shortName": "6ch", "channels": ["Dimmer", "Red", "Green", "Blue", "Strobe", null]}
          ]
        }"#;
        let f = convert_fixture("test", "Testhersteller", "test-par", json).unwrap();
        assert_eq!(f.modes.len(), 2);
        assert_eq!(f.modes[0].key, "ofl/test/test-par/3ch");
        assert_eq!(kinds(&f.modes[0]), [K::Red, K::Green, K::Blue]);
        let m = &f.modes[1];
        assert_eq!(m.channel_count(), 6);
        assert_eq!(
            kinds(m),
            [K::Dimmer, K::Red, K::Green, K::Blue, K::Strobe, K::Generic]
        );
        assert_eq!(m.channels[0].default, 255);
        assert_eq!(m.manufacturer, "Testhersteller");
        assert_eq!(f.categories, ["Color Changer"]);
    }

    #[test]
    fn moving_head_fine_channels_and_wheels() {
        let json = r#"{
          "name": "Spot",
          "wheels": {"Color Wheel": {"slots": [{"type":"Open"},{"type":"Color"},{"type":"Color"}]},
                     "Gobo Wheel": {"slots": [{"type":"Open"},{"type":"Gobo"}]}},
          "availableChannels": {
            "Pan": {"fineChannelAliases": ["Pan fine"], "defaultValue": 32768, "capability": {"type": "Pan", "angle": "540deg"}},
            "Tilt": {"fineChannelAliases": ["Tilt fine"], "defaultValue": "50%", "capability": {"type": "Tilt"}},
            "Color Wheel": {"capability": {"type": "WheelSlot", "slotNumber": 1}},
            "Gobo Wheel": {"capability": {"type": "WheelSlot", "slotNumber": 1}},
            "Speed": {"capability": {"type": "PanTiltSpeed"}},
            "Prism": {"capability": {"type": "Prism"}}
          },
          "modes": [{"name": "Extended", "channels": ["Pan", "Pan fine", "Tilt", "Tilt fine", "Speed", "Color Wheel", "Gobo Wheel", "Prism", "Mystery"]}]
        }"#;
        let f = convert_fixture("x", "X", "spot", json).unwrap();
        let m = &f.modes[0];
        assert_eq!(
            kinds(m),
            [
                K::Pan,
                K::PanFine,
                K::Tilt,
                K::TiltFine,
                K::Speed,
                K::ColorWheel,
                K::Gobo,
                K::Generic,
                K::Generic
            ]
        );
        assert_eq!(m.channels[0].default, 128, "16-Bit-Default → grobes Byte");
        assert_eq!(m.channels[2].default, 128, "50 %");
        assert_eq!(m.channels[1].default, 0);
        assert_eq!(m.key, "ofl/x/spot/extended");
    }

    #[test]
    fn matrix_pixel_count_and_order() {
        let json = r#"{
          "name": "Bar",
          "matrix": {"pixelCount": [3, 1, 1]},
          "availableChannels": {"Master": {"capability": {"type": "Intensity"}}},
          "templateChannels": {
            "Red $pixelKey": {"capability": {"type": "ColorIntensity", "color": "Red"}},
            "Green $pixelKey": {"capability": {"type": "ColorIntensity", "color": "Green"}}
          },
          "modes": [
            {"name": "per pixel", "channels": ["Master", {"insert": "matrixChannels", "repeatFor": "eachPixelXYZ", "channelOrder": "perPixel", "templateChannels": ["Red $pixelKey", "Green $pixelKey"]}]},
            {"name": "per channel", "channels": [{"insert": "matrixChannels", "repeatFor": ["3", "1"], "channelOrder": "perChannel", "templateChannels": ["Red $pixelKey", "Green $pixelKey"]}]},
            {"name": "direct", "channels": ["Red 2"]}
          ]
        }"#;
        let f = convert_fixture("x", "X", "bar", json).unwrap();
        let names = |i: usize| -> Vec<String> {
            f.modes[i].channels.iter().map(|c| c.name.clone()).collect()
        };
        assert_eq!(
            names(0),
            ["Master", "Red 1", "Green 1", "Red 2", "Green 2", "Red 3", "Green 3"]
        );
        assert_eq!(names(1), ["Red 3", "Red 1", "Green 3", "Green 1"]);
        assert_eq!(kinds(&f.modes[0])[0], K::Dimmer);
        assert_eq!(kinds(&f.modes[0])[1], K::Red);
        assert_eq!(kinds(&f.modes[2]), [K::Red]);
    }

    #[test]
    fn matrix_pixel_keys_abc_and_groups() {
        let json = r#"{
          "name": "Quad",
          "matrix": {"pixelKeys": [[["10", "2", "1"]]], "pixelGroups": {"All": ["1", "2", "10"]}},
          "templateChannels": {"Dimmer $pixelKey": {"capability": {"type": "Intensity"}}},
          "modes": [
            {"name": "abc", "channels": [{"insert": "matrixChannels", "repeatFor": "eachPixelABC", "channelOrder": "perPixel", "templateChannels": ["Dimmer $pixelKey"]}]},
            {"name": "xyz", "channels": [{"insert": "matrixChannels", "repeatFor": "eachPixelXYZ", "channelOrder": "perPixel", "templateChannels": ["Dimmer $pixelKey"]}]},
            {"name": "grp", "channels": [{"insert": "matrixChannels", "repeatFor": "eachPixelGroup", "channelOrder": "perPixel", "templateChannels": ["Dimmer $pixelKey"]}]}
          ]
        }"#;
        let f = convert_fixture("x", "X", "quad", json).unwrap();
        let names = |i: usize| -> Vec<String> {
            f.modes[i].channels.iter().map(|c| c.name.clone()).collect()
        };
        assert_eq!(names(0), ["Dimmer 1", "Dimmer 2", "Dimmer 10"]);
        assert_eq!(names(1), ["Dimmer 10", "Dimmer 2", "Dimmer 1"]);
        assert_eq!(names(2), ["Dimmer All"]);
        assert!(f.modes[2].channels[0].kind == K::Dimmer);
    }

    #[test]
    fn switch_channels_resolve_to_default_target() {
        let json = r#"{
          "name": "Sw",
          "availableChannels": {
            "Mode": {"capabilities": [
              {"dmxRange": [0, 127], "type": "NoFunction", "switchChannels": {"Speed / Dimmer": "Speed"}},
              {"dmxRange": [128, 255], "type": "Effect", "effectName": "x", "switchChannels": {"Speed / Dimmer": "Dim"}}]},
            "Speed": {"capability": {"type": "EffectSpeed"}},
            "Dim": {"capability": {"type": "Intensity"}}
          },
          "modes": [{"name": "m", "channels": ["Mode", "Speed / Dimmer"]}]
        }"#;
        let f = convert_fixture("x", "X", "sw", json).unwrap();
        assert_eq!(f.modes[0].channels[1].name, "Speed / Dimmer");
        assert_eq!(f.modes[0].channels[1].kind, K::Speed);
    }

    #[test]
    fn redirects_are_skipped() {
        let r = convert_fixture(
            "x",
            "X",
            "r",
            r#"{"name": "R", "redirectTo": "y/z", "reason": "SameAsDifferentBrand"}"#,
        );
        assert!(matches!(r, Err(OflError::Skip(_))));
    }

    #[test]
    fn natural_sort() {
        let mut v = vec!["10", "2", "1", "A10", "A2"];
        v.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(v, ["1", "2", "10", "A2", "A10"]);
    }
}
