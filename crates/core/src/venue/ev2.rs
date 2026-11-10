//! Import from Easy View 2 (`.ev2` file decrypted to XML).
//!
//! Only the user's own data is taken over: room size, environment,
//! structures (boxes and planes with color), trusses and the placement of
//! the fixtures including address. Easy View models, textures and the
//! embedded Daslight fixture library are deliberately not taken over –
//! trusses and speakers are recreated with own shapes.
//!
//! Easy View works in centimeters with y up, left-handed (DirectX);
//! rotations in degrees (X, then Y, then Z). DMXCraft is right-handed, so
//! the x axis is mirrored.

use super::{Material, Object, Placement, Shape, Transform, TrussProfile, Venue};

/// Base dimensions of the Easy View box “rect 75x100x200” in cm (x, y, z).
const RECT_SIZE: [f32; 3] = [200.0, 75.0, 100.0];
/// Length of the truss parts from the file name (“s3000” = 3.0 m).
const DEFAULT_TRUSS_LEN: f32 = 2.0;

/// A fixture from the Easy View file.
#[derive(Clone, Debug, PartialEq)]
pub struct Ev2Fixture {
    pub name: String,
    /// Library name in Easy View, e.g. “Stairville/PAR 56 7CH.ssl2” (only as a
    /// hint for matching).
    pub library_name: String,
    /// 0-based.
    pub universe: u8,
    pub address: u16,
    pub placement: Placement,
}

#[derive(Debug)]
pub struct Ev2Import {
    pub venue: Venue,
    pub fixtures: Vec<Ev2Fixture>,
    /// Objects that were only approximated (e.g. unknown models).
    pub approximated: Vec<String>,
}

fn nums(s: &str) -> Vec<f32> {
    s.split(',').filter_map(|v| v.trim().parse().ok()).collect()
}

fn vec3(n: &roxmltree::Node, attr: &str, default: f32) -> [f32; 3] {
    let v = n.attribute(attr).map(nums).unwrap_or_default();
    [0, 1, 2].map(|i| v.get(i).copied().unwrap_or(default))
}

/// ARGB as a decimal number → RGB + opacity.
fn argb(n: &roxmltree::Node) -> ([u8; 3], f32) {
    let v: u32 = n
        .attribute("COLOR")
        .and_then(|c| c.parse::<u64>().ok())
        .map_or(0xffa0_a0a0, |c| c as u32);
    let [a, r, g, b] = v.to_be_bytes();
    let _ = a;
    // TRANSPARENCY in Easy View is the opacity (1 = opaque)
    let opacity = n
        .attribute("TRANSPARENCY")
        .and_then(|t| t.parse::<f32>().ok())
        .unwrap_or(1.0);
    (
        [r, g, b],
        if opacity <= 0.0 {
            1.0
        } else {
            opacity.clamp(0.05, 1.0)
        },
    )
}

/// Easy View is left-handed (DirectX): x is mirrored. Mirroring at x keeps
/// the rotation around x and reverses those around y and z.
fn transform(n: &roxmltree::Node) -> Transform {
    let p = vec3(n, "POSITION", 0.0);
    let r = vec3(n, "ROTATION", 0.0);
    Transform {
        position: [-p[0] / 100.0, p[1] / 100.0, p[2] / 100.0],
        rotation: [r[0], -r[1], -r[2]],
        scale: vec3(n, "SCALE", 1.0),
    }
}

/// Reads an Easy View 2 file available as XML.
pub fn import(xml: &str) -> Result<Ev2Import, String> {
    let doc = roxmltree::Document::parse(xml).map_err(|e| format!("XML: {e}"))?;
    let root = doc.root_element();
    if root.tag_name().name() != "EV2FILE" {
        return Err("Not an Easy View 2 file (EV2FILE missing)".into());
    }
    let cfg = root.children().find(|c| c.has_tag_name("CONFIGURATION"));
    let num = |a: &str, d: f32| {
        cfg.and_then(|c| c.attribute(a))
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(d)
    };
    let size = [
        num("SCENESIZEX", 1200.0) / 100.0,
        num("SCENESIZEY", 500.0) / 100.0,
        num("SCENESIZEZ", 1600.0) / 100.0,
    ];
    let mut venue = Venue {
        name: "Easy View Import".into(),
        size,
        objects: Vec::new(),
        ..Venue::default()
    };
    venue.environment.ambient = num("AMBIENT", 0.25).clamp(0.0, 1.0).max(0.05);
    venue.environment.beams = num("BEAMS", 0.3).clamp(0.0, 1.0);
    venue.environment.fog = num("FOG", 0.2).clamp(0.0, 1.0);

    let mut out = Ev2Import {
        venue,
        fixtures: Vec::new(),
        approximated: Vec::new(),
    };
    if let Some(objects) = root.children().find(|c| c.has_tag_name("OBJECTS")) {
        let mut list = Vec::new();
        for n in objects.children().filter(|c| c.is_element()) {
            if let Some(o) = node(&mut out, &n, super::IDENTITY) {
                list.push(o);
            }
        }
        out.venue.objects = list;
    }
    Ok(out)
}

/// Converts a node; fixtures end up in `out.fixtures` (with world
/// coordinates, hence the matrix of the parent groups).
fn node(out: &mut Ev2Import, n: &roxmltree::Node, parent: [[f32; 4]; 4]) -> Option<Object> {
    let name = n.attribute("NAME").unwrap_or("").to_string();
    let hidden = n.attribute("VISIBLE") == Some("0");
    let mut t = transform(n);
    match n.tag_name().name() {
        "OBJECT" => {
            let world = super::mul(parent, t.matrix());
            let mut children = Vec::new();
            for c in n.children().filter(|c| c.is_element()) {
                if let Some(o) = node(out, &c, world) {
                    children.push(o);
                }
            }
            // Groups that only contained fixtures are dropped
            if children.is_empty() {
                return None;
            }
            let mut g = out
                .venue
                .object(&name, Shape::Group, t, Material::default());
            g.children = children;
            g.hidden = hidden;
            Some(g)
        }
        "FIXTURE" => {
            let world = super::mul(parent, t.matrix());
            let num = |a: &str| n.attribute(a).and_then(|v| v.parse::<u32>().ok());
            out.fixtures.push(Ev2Fixture {
                name: name.clone(),
                library_name: n.attribute("SSLNAME").unwrap_or("").to_string(),
                universe: num("UNIVERS").unwrap_or(1).saturating_sub(1).min(255) as u8,
                address: num("ADDRESS").unwrap_or(1).clamp(1, 512) as u16,
                placement: Placement {
                    position: [world[3][0], world[3][1], world[3][2]],
                    // Groups are not rotated in Easy View; the own rotation is enough
                    rotation: t.rotation,
                },
            });
            None
        }
        "OBJECT3D" => {
            let file = n.attribute("FILE3D").unwrap_or("").to_lowercase();
            let (color, opacity) = argb(n);
            let material = Material {
                color,
                opacity,
                gloss: n
                    .attribute("SPECULAR")
                    .and_then(|v| v.parse::<f32>().ok())
                    .unwrap_or(0.0)
                    .clamp(0.0, 1.0)
                    .max(0.1),
                emissive: n
                    .attribute("EMISSIVE")
                    .and_then(|v| v.parse::<f32>().ok())
                    .unwrap_or(0.0)
                    .clamp(0.0, 1.0),
                ..Material::default()
            };
            let shape = if file.contains("plane") {
                Shape::Plane { size: [1.0, 1.0] }
            } else if file.contains("rect") {
                Shape::Box {
                    size: RECT_SIZE.map(|v| v / 100.0),
                }
            } else if file.contains("truss") {
                // Length from “…-s3000.x” (mm); trusses lie along y in Easy View
                let len = file
                    .rsplit("-s")
                    .next()
                    .and_then(|s| s.trim_end_matches(".x").parse::<f32>().ok())
                    .map_or(DEFAULT_TRUSS_LEN, |mm| mm / 1000.0);
                let profile =
                    if file.contains("quad") || file.contains("4") && !file.contains("ao3") {
                        TrussProfile::Square
                    } else {
                        TrussProfile::Triangle
                    };
                // our truss lies along x: rotate around z so it stands as in Easy View
                let inner = Object {
                    id: out.venue.new_id(),
                    name: name.clone(),
                    shape: Shape::Truss {
                        length: len,
                        profile,
                        width: 0.29,
                    },
                    transform: Transform {
                        rotation: [0.0, 0.0, 90.0],
                        ..Transform::default()
                    },
                    material: Material {
                        color: [190, 190, 195],
                        gloss: 0.6,
                        ..Material::default()
                    },
                    children: Vec::new(),
                    locked: false,
                    hidden: false,
                };
                let mut g = out
                    .venue
                    .object(&name, Shape::Group, t, Material::default());
                g.children = vec![inner];
                g.hidden = hidden;
                return Some(g);
            } else {
                // unknown model (e.g. speaker): as a dark box
                out.approximated.push(name.clone());
                t.scale = t.scale.map(|s| s.abs());
                let mut o = out.venue.object(
                    &name,
                    Shape::Box {
                        size: [0.45, 0.6, 0.45],
                    },
                    t,
                    Material::color([30, 30, 32]),
                );
                o.hidden = hidden;
                return Some(o);
            };
            let mut o = out.venue.object(&name, shape, t, material);
            o.hidden = hidden;
            o.locked = n.attribute("LOCK") == Some("1");
            Some(o)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"<EV2FILE VERSION="3" TYPE="EV2">
      <CONFIGURATION AMBIENT="0.008" BEAMS="0.26" FOG="0.18" SCENESIZEX="1354" SCENESIZEY="547" SCENESIZEZ="1958"/>
      <FIXTURES><SSLLIBRARY SSLNAME="x"/></FIXTURES>
      <OBJECTS>
        <OBJECT NAME="Scene" POSITION="0,0,0" ROTATION="0,0,0" SCALE="1,1,1">
          <OBJECT3D NAME="floor" POSITION="0,0,0" ROTATION="0,0,0" SCALE="1354,1,1958" FILE3D=":/library/plane.dae" COLOR="4288980132" TRANSPARENCY="1"/>
          <OBJECT NAME="Bau" POSITION="100,200,0" ROTATION="0,0,0" SCALE="1,1,1">
            <OBJECT3D NAME="rect" POSITION="0,0,0" ROTATION="0,90,0" SCALE="2,0.1,1" FILE3D="/Construction/rect 75x100x200.x" COLOR="4278190080" TRANSPARENCY="1"/>
          </OBJECT>
        </OBJECT>
        <OBJECT NAME="Fixtures" POSITION="0,0,0" ROTATION="0,0,0" SCALE="1,1,1">
          <OBJECT NAME="Pars" POSITION="0,100,0" ROTATION="0,0,0" SCALE="1,1,1">
            <FIXTURE NAME="par.1" POSITION="50,400,-20" ROTATION="10,0,0" SCALE="1,1,1" SSLNAME="Stairville/PAR 56 7CH.ssl2" ADDRESS="230" UNIVERS="2"/>
          </OBJECT>
        </OBJECT>
        <OBJECT NAME="Traversen" POSITION="0,0,0" ROTATION="0,0,0" SCALE="1,1,1">
          <OBJECT3D NAME="t1" POSITION="0,516,0" ROTATION="0,30,90" SCALE="1,1,1" FILE3D="/Truss/TriLite/Straight/ao3-s3000.x"/>
          <OBJECT3D NAME="Cluster.x" POSITION="0,300,0" ROTATION="0,0,0" SCALE="1,1,1" FILE3D="/Sound systems/Cluster.x"/>
        </OBJECT>
      </OBJECTS>
    </EV2FILE>"#;

    #[test]
    fn imports_geometry_and_fixtures() {
        let r = import(SAMPLE).unwrap();
        assert_eq!(r.venue.size, [13.54, 5.47, 19.58]);
        assert!((r.venue.environment.fog - 0.18).abs() < 1e-6);
        // Stage/structure, trusses (a fixtures group without structures is
        // dropped)
        assert_eq!(r.venue.objects.len(), 2);
        let scene = &r.venue.objects[0];
        assert!(matches!(scene.children[0].shape, Shape::Plane { .. }));
        let rect = &scene.children[1].children[0];
        assert!(matches!(rect.shape, Shape::Box { size } if size == [2.0, 0.75, 1.0]));
        assert_eq!(rect.material.color, [0, 0, 0]);
        // Fixture in world coordinates (group raised by 1 m)
        let f = &r.fixtures[0];
        assert_eq!((f.universe, f.address), (1, 230));
        assert!((f.placement.position[1] - 5.0).abs() < 1e-4);
        // mirrored: x reversed, the rotation around x stays
        assert!((f.placement.position[0] + 0.5).abs() < 1e-4);
        assert_eq!(f.placement.rotation, [10.0, -0.0, -0.0]);
        // Truss with the length from the file name, speaker approximated
        let truss = &r.venue.objects[1].children[0].children[0];
        assert!(matches!(truss.shape, Shape::Truss { length, .. } if (length - 3.0).abs() < 1e-6));
        assert_eq!(r.approximated, vec!["Cluster.x".to_string()]);
    }

    #[test]
    fn rejects_other_xml() {
        assert!(import("<foo/>").is_err());
    }
}
