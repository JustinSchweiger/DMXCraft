//! Conversion of an OFL fixture file into DMXCraft fixture types.
//!
//! Supported: channels with `capability`/`capabilities`, fine channels
//! (`fineChannelAliases`), default values (number or percent), matrix
//! fixtures (`pixelCount`, `pixelKeys`, `pixelGroups`, `templateChannels`,
//! `insert: matrixChannels`) and switching channels (`switchChannels`).
//! Unknown channel names are taken over as generic channels so the channel
//! count of the mode is always right.

use std::collections::HashMap;

use dmxcraft_core::{ChannelDef, ChannelKind, FixtureType};
use serde_json::{Map, Value};
use thiserror::Error;

use crate::LibraryFixture;

#[derive(Debug, Error)]
pub enum OflError {
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Skipped: {0}")]
    Skip(&'static str),
    #[error("Invalid OFL file: {0}")]
    Invalid(String),
}

/// A pixel of a matrix with position (x, y, z – 0-based).
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

    /// Order of the pixels/groups for a `repeatFor`.
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
                // e.g. eachPixelXYZ: X runs fastest, then Y, then Z.
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

/// Comparison that sorts numbers numerically (“2” < “10”).
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

/// A resolved channel: OFL channel definition + whether it is the n-th fine
/// channel.
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

/// OFL speed (“fast”, “5Hz”, “120bpm”, “50%”) in Hz.
fn speed_hz(v: Option<&Value>) -> Option<f32> {
    let s = v?.as_str()?.trim().to_lowercase();
    let num = |suffix: &str| {
        s.strip_suffix(suffix)
            .and_then(|n| n.trim().parse::<f32>().ok())
    };
    if s.starts_with("fast") {
        Some(20.0)
    } else if s.starts_with("slow") {
        Some(1.0)
    } else if s == "stop" {
        Some(0.0)
    } else if let Some(hz) = num("hz") {
        Some(hz)
    } else if let Some(bpm) = num("bpm") {
        Some(bpm / 60.0)
    } else {
        num("%").map(|p| 1.0 + p / 100.0 * 19.0)
    }
}

fn hex_rgb(s: &str) -> Option<[u8; 3]> {
    let h = s.trim().trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let p = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok();
    Some([p(0)?, p(2)?, p(4)?])
}

/// Value ranges of a (coarse) channel from the OFL capabilities.
fn ofl_ranges(
    fixture: &Value,
    ch_name: &str,
    r: &Resolved<'_>,
) -> Vec<dmxcraft_core::ChannelRange> {
    use dmxcraft_core::{ChannelRange, RangeEffect};
    // Range values in higher resolution to 8 bit
    let shift = match r.def.get("dmxValueResolution").and_then(Value::as_str) {
        Some("8bit") => 0,
        Some("16bit") => 8,
        Some("24bit") => 16,
        _ => 8 * r.fine_count as u32,
    };
    let caps = capabilities(r.def);
    if caps.len() < 2 && caps.first().is_none_or(|c| c.get("dmxRange").is_none()) {
        // one capability for the whole channel: only strobe/shutter is interesting
        let Some(c) = caps.first() else {
            return Vec::new();
        };
        if c.get("type").and_then(Value::as_str) != Some("ShutterStrobe") {
            return Vec::new();
        }
    }
    let mut out = Vec::new();
    for c in caps {
        let (from, to) = match c.get("dmxRange").and_then(Value::as_array) {
            Some(a) => {
                let n = |i: usize| {
                    (a.get(i).and_then(Value::as_u64).unwrap_or(0) >> shift).min(255) as u8
                };
                (n(0), n(1))
            }
            None => (0, 255),
        };
        let ty = c.get("type").and_then(Value::as_str).unwrap_or("");
        let comment = c
            .get("comment")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        let mut label = comment;
        let effect = match ty {
            "ShutterStrobe" => match c.get("shutterEffect").and_then(Value::as_str) {
                Some("Open") => RangeEffect::Open,
                Some("Closed") => RangeEffect::Closed,
                _ => {
                    let a = speed_hz(c.get("speedStart")).or(speed_hz(c.get("speed")));
                    let b = speed_hz(c.get("speedEnd")).or(a);
                    RangeEffect::Strobe {
                        from_hz: a.unwrap_or(1.0).max(0.2),
                        to_hz: b.unwrap_or(20.0).max(0.2),
                    }
                }
            },
            "WheelSlot" => {
                let wheel = c.get("wheel").and_then(Value::as_str).unwrap_or(ch_name);
                let n = c
                    .get("slotNumber")
                    .or_else(|| c.get("slotNumberStart"))
                    .and_then(Value::as_f64)
                    .unwrap_or(1.0)
                    .floor()
                    .max(1.0) as usize;
                let slot = fixture
                    .get("wheels")
                    .and_then(|w| w.get(wheel))
                    .and_then(|w| w.get("slots"))
                    .and_then(Value::as_array)
                    .and_then(|s| s.get(n - 1));
                let slot_ty = slot
                    .and_then(|s| s.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                if label.is_empty() {
                    label = slot
                        .and_then(|s| s.get("name"))
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .to_string();
                }
                match slot_ty {
                    "Open" => RangeEffect::WheelOpen,
                    "Closed" => RangeEffect::Closed,
                    "Color" => slot
                        .and_then(|s| s.get("colors"))
                        .and_then(Value::as_array)
                        .and_then(|c| c.first())
                        .and_then(Value::as_str)
                        .and_then(hex_rgb)
                        .map_or(RangeEffect::Other, |rgb| RangeEffect::Color { rgb }),
                    "Gobo" | "GoboShake" => RangeEffect::Gobo {
                        slot: n.min(255) as u8,
                    },
                    _ => RangeEffect::Other,
                }
            }
            "ColorPreset" => c
                .get("colors")
                .and_then(Value::as_array)
                .and_then(|c| c.first())
                .and_then(Value::as_str)
                .and_then(hex_rgb)
                .map_or(RangeEffect::Other, |rgb| RangeEffect::Color { rgb }),
            _ => RangeEffect::Other,
        };
        if label.is_empty() && matches!(effect, RangeEffect::Other) {
            // “PanTiltSpeed” → “Pan Tilt Speed”
            let mut l = String::new();
            for (i, ch) in ty.chars().enumerate() {
                if i > 0 && ch.is_uppercase() {
                    l.push(' ');
                }
                l.push(ch);
            }
            label = l;
        }
        out.push(ChannelRange::new(from, to, effect).label(label));
    }
    out
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
        "Fog" | "FogOutput" | "FogType" => ChannelKind::Fog,
        "Zoom" => ChannelKind::Zoom,
        "Focus" => ChannelKind::Focus,
        "Effect" | "EffectParameter" | "SoundSensitivity" => ChannelKind::Program,
        "Maintenance" => ChannelKind::Control,
        "NoFunction" => ChannelKind::NoFunction,
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
                // Given in 16/24-bit resolution → only the coarse byte.
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

pub(crate) fn slug(s: &str) -> String {
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

/// Converts an OFL fixture file.
///
/// `manufacturer_key` / `fixture_key` are the folder and file name in the
/// OFL (e.g. `cameo` / `flat-pro-18`), `manufacturer` the display name.
pub fn convert_fixture(
    manufacturer_key: &str,
    manufacturer: &str,
    fixture_key: &str,
    json: &str,
) -> Result<LibraryFixture, OflError> {
    let fixture: Value = serde_json::from_str(json)?;
    if fixture.get("redirectTo").is_some() {
        return Err(OflError::Skip("Redirect to another device"));
    }
    let name = fixture
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| OflError::Invalid("no name".into()))?
        .to_string();
    let modes = fixture
        .get("modes")
        .and_then(Value::as_array)
        .ok_or_else(|| OflError::Invalid("no modes".into()))?;
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
                kind: if r.fine > 0 {
                    channel_kind(&fixture, ch_name, &r)
                } else {
                    channel_kind(&fixture, ch_name, &r).refine(ch_name)
                },
                default: default_value(&r),
                ranges: if r.fine > 0 {
                    Vec::new()
                } else {
                    ofl_ranges(&fixture, ch_name, &r)
                },
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
                Value::Null => channels.push(ChannelDef::new("(unused)", ChannelKind::NoFunction)),
                Value::String(s) => channels.push(make_channel(s)),
                Value::Object(o)
                    if o.get("insert").and_then(Value::as_str) == Some("matrixChannels") =>
                {
                    let Some(m) = matrix.as_ref() else {
                        return Err(OflError::Invalid("matrixChannels without matrix".into()));
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
        // physical data: overridden in the mode, otherwise that of the fixture
        let physical = dmxcraft_core::fixture::Physics::from_ofl(mode.get("physical"))
            .or_else(|| dmxcraft_core::fixture::Physics::from_ofl(fixture.get("physical")));
        out_modes.push(FixtureType {
            physical,
            key,
            manufacturer: manufacturer.to_string(),
            model: name.clone(),
            mode: mode_name,
            channels,
        });
    }
    if out_modes.is_empty() {
        return Err(OflError::Invalid("no usable mode".into()));
    }
    Ok(LibraryFixture {
        manufacturer_key: manufacturer_key.to_string(),
        manufacturer: manufacturer.to_string(),
        key: fixture_key.to_string(),
        name,
        categories,
        modes: out_modes,
        source: crate::Source::Ofl,
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
            [
                K::Dimmer,
                K::Red,
                K::Green,
                K::Blue,
                K::Strobe,
                K::NoFunction
            ]
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
    fn shutter_and_wheel_ranges() {
        use dmxcraft_core::RangeEffect;
        let fx = r##"{
            "name": "Spot", "categories": ["Moving Head"],
            "wheels": {"Color Wheel": {"slots": [{"type":"Open"},{"type":"Color","name":"Red","colors":["#ff0000"]}]}},
            "availableChannels": {
                "Shutter": {"capabilities": [
                    {"dmxRange":[0,7],"type":"ShutterStrobe","shutterEffect":"Closed"},
                    {"dmxRange":[8,15],"type":"ShutterStrobe","shutterEffect":"Open"},
                    {"dmxRange":[16,255],"type":"ShutterStrobe","shutterEffect":"Strobe","speedStart":"1Hz","speedEnd":"25Hz"}
                ]},
                "Color Wheel": {"capabilities": [
                    {"dmxRange":[0,9],"type":"WheelSlot","slotNumber":1},
                    {"dmxRange":[10,19],"type":"WheelSlot","slotNumber":2}
                ]}
            },
            "modes": [{"name":"2ch","channels":["Shutter","Color Wheel"]}]
        }"##;
        let lib = convert_fixture("x", "X", "spot", fx).unwrap();
        let t = &lib.modes[0];
        let sh = &t.channels[0];
        assert_eq!(sh.ranges.len(), 3);
        assert_eq!(sh.strobe(10), None);
        assert_eq!(sh.strobe(3), Some(0.0));
        assert_eq!(sh.strobe(255), Some(25.0));
        let cw = &t.channels[1];
        assert_eq!(cw.ranges[0].effect, RangeEffect::WheelOpen);
        assert_eq!(cw.ranges[1].effect, RangeEffect::Color { rgb: [255, 0, 0] });
        assert_eq!(cw.ranges[1].label, "Red");
    }

    #[test]
    fn natural_sort() {
        let mut v = vec!["10", "2", "1", "A10", "A2"];
        v.sort_by(|a, b| natural_cmp(a, b));
        assert_eq!(v, ["1", "2", "10", "A2", "A10"]);
    }
}
