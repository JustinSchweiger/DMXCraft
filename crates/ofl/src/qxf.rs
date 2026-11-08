//! Umwandlung von QLC+-Gerätedateien (`.qxf`, XML) in DMXCraft-Gerätetypen.
//!
//! Aufbau einer QXF-Datei (vereinfacht):
//!
//! ```xml
//! <FixtureDefinition>
//!   <Manufacturer>American DJ</Manufacturer>
//!   <Model>12P Hex IP</Model>
//!   <Type>Color Changer</Type>
//!   <Channel Name="Red" Preset="IntensityRed"/>
//!   <Channel Name="Pan"><Group Byte="0">Pan</Group></Channel>
//!   <Mode Name="6 Channel"><Channel Number="0">Red</Channel>…</Mode>
//! </FixtureDefinition>
//! ```
//!
//! Die Kanalart kommt aus `Preset` (neuere Dateien) oder aus `Group`/`Colour`
//! (ältere). QLC+-Gerätedaten stehen unter der Apache-2.0-Lizenz
//! (siehe `data/QLC-LICENSE`).

use std::collections::HashMap;

use dmxcraft_core::{ChannelDef, ChannelKind, FixtureType};
use roxmltree::{Document, Node, ParsingOptions};

use crate::{LibraryFixture, OflError, Source};

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

fn child<'a>(n: Node<'a, 'a>, name: &str) -> Option<Node<'a, 'a>> {
    n.children().find(|c| c.has_tag_name(name))
}

fn text<'a>(n: Node<'a, 'a>, name: &str) -> Option<&'a str> {
    child(n, name).and_then(|c| c.text()).map(str::trim)
}

/// Kanalart aus dem QLC+-Preset des Kanals.
fn kind_from_preset(p: &str) -> Option<ChannelKind> {
    use ChannelKind as K;
    Some(match p {
        "IntensityMasterDimmer" | "IntensityDimmer" => K::Dimmer,
        "IntensityRed" => K::Red,
        "IntensityGreen" => K::Green,
        "IntensityBlue" => K::Blue,
        "IntensityWhite" => K::White,
        "IntensityAmber" => K::Amber,
        "IntensityUV" => K::Uv,
        "PositionPan" => K::Pan,
        "PositionPanFine" => K::PanFine,
        "PositionTilt" => K::Tilt,
        "PositionTiltFine" => K::TiltFine,
        p if p.starts_with("ShutterStrobe") => K::Strobe,
        "ColorMacro" | "ColorWheel" | "ColorDoubleMacro" => K::ColorWheel,
        p if p.starts_with("Gobo") => K::Gobo,
        p if p.starts_with("Speed") => K::Speed,
        _ => return None,
    })
}

/// Kanalart aus Gruppe (ältere Dateien), Farbangabe und Name.
fn kind_from_group(ch: Node<'_, '_>, name: &str) -> ChannelKind {
    use ChannelKind as K;
    let group = child(ch, "Group");
    let byte = group.and_then(|g| g.attribute("Byte")).unwrap_or("0");
    let fine = byte == "1";
    let lname = name.to_lowercase();
    match group.and_then(|g| g.text()).map(str::trim) {
        Some("Intensity") => {
            if fine {
                return K::Generic;
            }
            match text(ch, "Colour") {
                Some("Red") => K::Red,
                Some("Green") => K::Green,
                Some("Blue") => K::Blue,
                Some("White") => K::White,
                Some("Amber") => K::Amber,
                Some("UV") => K::Uv,
                Some(_) => K::Generic,
                None => K::Dimmer,
            }
        }
        Some("Pan") => {
            if fine {
                K::PanFine
            } else {
                K::Pan
            }
        }
        Some("Tilt") => {
            if fine {
                K::TiltFine
            } else {
                K::Tilt
            }
        }
        Some("Shutter") if !fine && lname.contains("strob") => K::Strobe,
        Some("Shutter") if !fine => K::Strobe,
        Some("Colour") if !fine => K::ColorWheel,
        Some("Gobo") if !fine && !lname.contains("rot") => K::Gobo,
        Some("Speed") if !fine => K::Speed,
        _ => K::Generic,
    }
}

fn qlc_hex(s: &str) -> Option<[u8; 3]> {
    let h = s.trim().trim_start_matches('#');
    if h.len() < 6 {
        return None;
    }
    let p = |i: usize| u8::from_str_radix(h.get(i..i + 2)?, 16).ok();
    Some([p(0)?, p(2)?, p(4)?])
}

/// Wertebereiche aus den `<Capability>`-Einträgen.
fn qlc_ranges(ch: Node<'_, '_>) -> Vec<dmxcraft_core::ChannelRange> {
    use dmxcraft_core::{ChannelRange, RangeEffect};
    let mut gobo = 0u8;
    ch.children()
        .filter(|c| c.has_tag_name("Capability"))
        .filter_map(|c| {
            let n = |a: &str| c.attribute(a).and_then(|v| v.trim().parse::<u16>().ok());
            let (from, to) = (n("Min")?.min(255) as u8, n("Max")?.min(255) as u8);
            let label = c.text().unwrap_or("").trim().to_string();
            let preset = c.attribute("Preset").unwrap_or("");
            let res = |a: &str| c.attribute(a).and_then(|v| v.trim().parse::<f32>().ok());
            let strobe = |a: f32, b: f32| RangeEffect::Strobe {
                from_hz: a,
                to_hz: b,
            };
            let effect = match preset {
                "ShutterOpen" => RangeEffect::Open,
                "ShutterClose" | "LampOff" => RangeEffect::Closed,
                "StrobeSlowToFast"
                | "StrobeRandomSlowToFast"
                | "PulseSlowToFast"
                | "RampUpSlowToFast"
                | "RampDownSlowToFast"
                | "ShutterStrobeSlowFast" => strobe(1.0, 20.0),
                "StrobeFastToSlow"
                | "StrobeRandomFastToSlow"
                | "PulseFastToSlow"
                | "RampUpFastToSlow"
                | "RampDownFastToSlow"
                | "ShutterStrobeFastSlow" => strobe(20.0, 1.0),
                "StrobeRandom" => strobe(8.0, 8.0),
                "StrobeFrequency" | "PulseFrequency" => {
                    let hz = res("Res1").unwrap_or(5.0);
                    strobe(hz, hz)
                }
                "StrobeFreqRange" | "PulseFreqRange" => {
                    strobe(res("Res1").unwrap_or(1.0), res("Res2").unwrap_or(20.0))
                }
                "ColorMacro" | "ColorDoubleMacro" => c
                    .attribute("Res1")
                    .or(c.attribute("Res"))
                    .or(c.attribute("Color"))
                    .and_then(qlc_hex)
                    .map_or(RangeEffect::Other, |rgb| RangeEffect::Color { rgb }),
                "GoboMacro" | "GoboShakeMacro" => {
                    gobo = gobo.saturating_add(1);
                    RangeEffect::Gobo { slot: gobo }
                }
                _ => {
                    // ältere Dateien: Farbe im Attribut, „open“ im Text
                    let l = label.to_lowercase();
                    if let Some(rgb) = c.attribute("Color").and_then(qlc_hex) {
                        RangeEffect::Color { rgb }
                    } else if l == "open" || l.starts_with("open ") || l.contains("shutter open") {
                        RangeEffect::Open
                    } else if l.contains("closed") || l == "close" || l.contains("blackout") {
                        RangeEffect::Closed
                    } else if l.contains("strob") {
                        strobe(1.0, 20.0)
                    } else {
                        RangeEffect::Other
                    }
                }
            };
            Some(ChannelRange::new(from, to, effect).label(label))
        })
        .collect()
}

/// Feinkanäle von Farben/Dimmer laufen als „Generic“ (die Engine kennt nur
/// 8 Bit dafür); Presets mit `Fine` am Ende sind also immer Generic.
fn channel_def(ch: Node<'_, '_>) -> Option<(String, ChannelDef)> {
    let name = ch.attribute("Name")?.trim().to_string();
    let kind = match ch.attribute("Preset") {
        Some(p) if p.ends_with("Fine") && !p.starts_with("Position") => ChannelKind::Generic,
        Some(p) => kind_from_preset(p).unwrap_or_else(|| kind_from_group(ch, &name)),
        None => kind_from_group(ch, &name),
    };
    let kind = kind.refine(&name);
    let default = ch
        .attribute("Default")
        .and_then(|d| d.parse::<u16>().ok())
        .map(|d| d.min(255) as u8)
        .unwrap_or(0);
    Some((
        name.clone(),
        ChannelDef {
            name,
            kind,
            default,
            ranges: qlc_ranges(ch),
        },
    ))
}

/// Wandelt eine QXF-Datei um. Ein [`FixtureType`] pro Modus.
pub fn convert_qxf(xml: &str) -> Result<LibraryFixture, OflError> {
    let opts = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    let doc = Document::parse_with_options(xml, opts)
        .map_err(|e| OflError::Invalid(format!("XML: {e}")))?;
    let root = doc.root_element();
    if !root.has_tag_name("FixtureDefinition") {
        return Err(OflError::Invalid("keine FixtureDefinition".into()));
    }
    let manufacturer = text(root, "Manufacturer")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| OflError::Invalid("kein Hersteller".into()))?
        .to_string();
    let model = text(root, "Model")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| OflError::Invalid("kein Modell".into()))?
        .to_string();
    let kind = text(root, "Type").unwrap_or("Other").to_string();

    let channels: HashMap<String, ChannelDef> = root
        .children()
        .filter(|c| c.has_tag_name("Channel"))
        .filter_map(channel_def)
        .collect();

    let man_key = slug(&manufacturer);
    let model_key = slug(&model);
    let mut used: HashMap<String, usize> = HashMap::new();
    let mut modes = Vec::new();
    for mode in root.children().filter(|c| c.has_tag_name("Mode")) {
        let mode_name = mode
            .attribute("Name")
            .unwrap_or("Standard")
            .trim()
            .to_string();
        // Kanäle nach Nummer sortiert; fehlende Nummern als unbenutzt
        let mut entries: Vec<(usize, &str)> = mode
            .children()
            .filter(|c| c.has_tag_name("Channel"))
            .filter_map(|c| {
                let n = c.attribute("Number")?.parse::<usize>().ok()?;
                Some((n, c.text().unwrap_or("").trim()))
            })
            .collect();
        entries.sort_by_key(|(n, _)| *n);
        let Some(&(last, _)) = entries.last() else {
            continue;
        };
        if last >= 512 {
            continue;
        }
        let mut out: Vec<ChannelDef> = (0..=last)
            .map(|_| ChannelDef::new("(unbenutzt)", ChannelKind::NoFunction))
            .collect();
        for (n, name) in entries {
            out[n] = channels
                .get(name)
                .cloned()
                .unwrap_or_else(|| ChannelDef::new(name, ChannelKind::Generic));
        }
        let base = format!("qlc/{man_key}/{model_key}/{}", slug(&mode_name));
        let count = used.entry(base.clone()).or_insert(0);
        *count += 1;
        let key = if *count == 1 {
            base
        } else {
            format!("{base}-{count}")
        };
        let physical = qlc_physical(mode).or_else(|| qlc_physical(root));
        modes.push(FixtureType {
            physical,
            key,
            manufacturer: manufacturer.clone(),
            model: model.clone(),
            mode: mode_name,
            channels: out,
        });
    }
    if modes.is_empty() {
        return Err(OflError::Invalid("kein verwendbarer Modus".into()));
    }
    Ok(LibraryFixture {
        manufacturer_key: man_key,
        manufacturer,
        key: model_key,
        name: model,
        categories: vec![kind],
        modes,
        source: Source::Qlc,
    })
}

/// `<Physical>` (Linse, Fokus, Maße) eines Geräts oder Modus.
fn qlc_physical(node: roxmltree::Node) -> Option<dmxcraft_core::fixture::Physics> {
    let p = node.children().find(|c| c.has_tag_name("Physical"))?;
    let num = |tag: &str, attr: &str| {
        p.children()
            .find(|c| c.has_tag_name(tag))
            .and_then(|c| c.attribute(attr))
            .and_then(|v| v.parse::<f32>().ok())
    };
    let mut out = dmxcraft_core::fixture::Physics::default();
    let mut any = false;
    if let (Some(a), b) = (num("Lens", "DegreesMin"), num("Lens", "DegreesMax")) {
        if a > 0.0 {
            out.beam_angle = [a, b.unwrap_or(a).max(a)];
            any = true;
        }
    }
    if let Some(v) = num("Focus", "PanMax") {
        out.pan_range = v;
        any = true;
    }
    if let Some(v) = num("Focus", "TiltMax") {
        out.tilt_range = v;
        any = true;
    }
    if let (Some(w), Some(h), Some(d)) = (
        num("Dimensions", "Width"),
        num("Dimensions", "Height"),
        num("Dimensions", "Depth"),
    ) {
        out.size_mm = [w, h, d];
        any = true;
    }
    any.then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ChannelKind as K;

    const SAMPLE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE FixtureDefinition>
<FixtureDefinition xmlns="http://www.qlcplus.org/FixtureDefinition">
 <Manufacturer>Test Maker</Manufacturer>
 <Model>Spot 2000</Model>
 <Type>Moving Head</Type>
 <Channel Name="Pan" Preset="PositionPan"/>
 <Channel Name="Pan fine" Preset="PositionPanFine"/>
 <Channel Name="Tilt"><Group Byte="0">Tilt</Group></Channel>
 <Channel Name="Dimmer" Default="255"><Group Byte="0">Intensity</Group></Channel>
 <Channel Name="Red"><Group Byte="0">Intensity</Group><Colour>Red</Colour></Channel>
 <Channel Name="Red fine" Preset="IntensityRedFine"/>
 <Channel Name="Strobe"><Group Byte="0">Shutter</Group></Channel>
 <Channel Name="Colors"><Group Byte="0">Colour</Group></Channel>
 <Channel Name="Gobo"><Group Byte="0">Gobo</Group></Channel>
 <Mode Name="Basic">
  <Channel Number="0">Pan</Channel>
  <Channel Number="1">Tilt</Channel>
  <Channel Number="2">Dimmer</Channel>
 </Mode>
 <Mode Name="Extended">
  <Channel Number="0">Pan</Channel>
  <Channel Number="1">Pan fine</Channel>
  <Channel Number="2">Tilt</Channel>
  <Channel Number="3">Dimmer</Channel>
  <Channel Number="4">Red</Channel>
  <Channel Number="5">Red fine</Channel>
  <Channel Number="6">Strobe</Channel>
  <Channel Number="7">Colors</Channel>
  <Channel Number="9">Gobo</Channel>
 </Mode>
</FixtureDefinition>"#;

    #[test]
    fn converts_modes_and_kinds() {
        let f = convert_qxf(SAMPLE).unwrap();
        assert_eq!(f.manufacturer, "Test Maker");
        assert_eq!(f.name, "Spot 2000");
        assert_eq!(f.categories, ["Moving Head"]);
        assert_eq!(f.source, Source::Qlc);
        assert_eq!(f.modes.len(), 2);
        assert_eq!(f.modes[0].key, "qlc/test-maker/spot-2000/basic");
        let k: Vec<K> = f.modes[1].channels.iter().map(|c| c.kind).collect();
        assert_eq!(
            k,
            [
                K::Pan,
                K::PanFine,
                K::Tilt,
                K::Dimmer,
                K::Red,
                K::Generic,
                K::Strobe,
                K::ColorWheel,
                K::NoFunction, // Lücke (Kanal 9 fehlt in der Datei)
                K::Gobo,
            ]
        );
        assert_eq!(f.modes[1].channels[3].default, 255);
        assert_eq!(f.modes[1].channels[8].name, "(unbenutzt)");
    }

    #[test]
    fn rejects_files_without_modes() {
        let xml = r#"<FixtureDefinition><Manufacturer>A</Manufacturer><Model>B</Model></FixtureDefinition>"#;
        assert!(convert_qxf(xml).is_err());
    }
}
