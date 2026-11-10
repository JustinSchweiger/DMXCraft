//! Object library: bundled objects (built from basic shapes) and custom ones
//! stored in the config folder (`objects/*.dmxobj`).

use std::collections::BTreeMap;
use std::path::PathBuf;

use dmxcraft_core::venue::model::{Model, Texture};
use dmxcraft_core::venue::{Material, Object, Shape, Transform, TrussProfile, Venue};
use serde::{Deserialize, Serialize};

use crate::i18n::t;

pub const EXT: &str = "dmxobj";

/// An object (usually a folder with parts) together with the models it uses.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prefab {
    pub name: String,
    pub object: Object,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub models: BTreeMap<String, Model>,
    /// Images used (IDs as in the object; reassigned on insertion)
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub textures: BTreeMap<u32, Texture>,
}

/// Custom object with a file path.
pub struct UserPrefab {
    pub path: PathBuf,
    pub prefab: Prefab,
}

impl Prefab {
    /// Insert into the venue (new IDs, take over models) at `x`/`z`.
    pub fn insert(&self, v: &mut Venue, x: f32, z: f32) -> u32 {
        let mut o = self.object.clone();
        super::renumber(v, &mut o);
        o.transform.position[0] = x;
        o.transform.position[2] = z;
        o.locked = false;
        for (k, m) in &self.models {
            v.models.entry(k.clone()).or_insert_with(|| m.clone());
        }
        // Take over images and rewrite IDs
        let map: BTreeMap<u32, u32> = self
            .textures
            .iter()
            .map(|(old, tex)| (*old, v.add_texture(tex.clone())))
            .collect();
        fn remap(o: &mut Object, map: &BTreeMap<u32, u32>) {
            if o.material.texture != 0 {
                o.material.texture = map.get(&o.material.texture).copied().unwrap_or(0);
            }
            for c in &mut o.children {
                remap(c, map);
            }
        }
        remap(&mut o, &map);
        let id = o.id;
        v.objects.push(o);
        id
    }

    /// Object from the venue as a template (placement in the room, models
    /// included).
    pub fn from_venue(v: &Venue, id: u32) -> Option<Self> {
        let mut object = v.find(id)?.clone();
        let world = glam::Mat4::from_cols_array_2d(&v.parent_matrix(id))
            * glam::Mat4::from_cols_array_2d(&object.transform.matrix());
        let (s, r, p) = world.to_scale_rotation_translation();
        object.transform = Transform {
            position: [0.0, p.y, 0.0],
            rotation: super::deg_quat(r),
            scale: s.to_array(),
        };
        let mut models = BTreeMap::new();
        let mut textures = BTreeMap::new();
        fn collect(
            o: &Object,
            v: &Venue,
            out: &mut BTreeMap<String, Model>,
            tex: &mut BTreeMap<u32, Texture>,
        ) {
            if let Shape::Model { model } = &o.shape {
                if let Some(m) = v.models.get(model) {
                    out.insert(model.clone(), m.clone());
                }
            }
            if let Some(t) = v.textures.get(&o.material.texture) {
                tex.insert(o.material.texture, t.clone());
            }
            for c in &o.children {
                collect(c, v, out, tex);
            }
        }
        collect(&object, v, &mut models, &mut textures);
        Some(Self {
            name: object.name.clone(),
            object,
            models,
            textures,
        })
    }
}

/// Folder of the custom objects.
pub fn dir() -> Option<PathBuf> {
    crate::settings::Settings::path().map(|p| p.with_file_name("objects"))
}

pub fn load_user() -> Vec<UserPrefab> {
    let Some(dir) = dir() else {
        return Vec::new();
    };
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return Vec::new();
    };
    let mut out: Vec<UserPrefab> = entries
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == EXT))
        .filter_map(|path| {
            let s = std::fs::read_to_string(&path).ok()?;
            let prefab = serde_json::from_str(&s).ok()?;
            Some(UserPrefab { path, prefab })
        })
        .collect();
    out.sort_by_key(|p| p.prefab.name.to_lowercase());
    out
}

pub fn save_user(p: &Prefab) -> Result<PathBuf, String> {
    let dir = dir().ok_or(t("No configuration folder found"))?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let stem: String = p
        .name
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    let stem = if stem.is_empty() {
        "object".into()
    } else {
        stem
    };
    let mut path = dir.join(format!("{stem}.{EXT}"));
    let mut n = 2;
    while path.exists() {
        path = dir.join(format!("{stem}_{n}.{EXT}"));
        n += 1;
    }
    let json = serde_json::to_string(p).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(path)
}

// ---------- bundled objects ----------

const DARK: [u8; 3] = [42, 42, 46];
const BLACK: [u8; 3] = [18, 18, 20];
const WOOD: [u8; 3] = [124, 88, 58];
const WHITE: [u8; 3] = [236, 236, 236];
const METAL: [u8; 3] = [190, 190, 195];
const GREY: [u8; 3] = [120, 124, 130];

fn mat(c: [u8; 3], gloss: f32) -> Material {
    Material {
        color: c,
        gloss,
        ..Material::default()
    }
}

struct Builder {
    v: Venue,
    parts: Vec<Object>,
}

impl Builder {
    fn new() -> Self {
        Self {
            v: Venue::room("", [10.0, 5.0, 10.0]),
            parts: Vec::new(),
        }
    }

    fn part(mut self, name: &str, shape: Shape, pos: [f32; 3], rot: [f32; 3], m: Material) -> Self {
        let o = self.v.object(
            name,
            shape,
            Transform {
                position: pos,
                rotation: rot,
                ..Transform::default()
            },
            m,
        );
        self.parts.push(o);
        self
    }

    fn cube(self, name: &str, size: [f32; 3], pos: [f32; 3], c: [u8; 3]) -> Self {
        self.part(name, Shape::Box { size }, pos, [0.0; 3], mat(c, 0.15))
    }

    fn cyl(self, name: &str, r: f32, h: f32, pos: [f32; 3], c: [u8; 3]) -> Self {
        self.part(
            name,
            Shape::Cylinder {
                radius: r,
                height: h,
            },
            pos,
            [0.0; 3],
            mat(c, 0.4),
        )
    }

    fn truss(self, len: f32, pos: [f32; 3], rot: [f32; 3]) -> Self {
        self.part(
            t("Truss"),
            Shape::Truss {
                length: len,
                profile: TrussProfile::Square,
                width: 0.29,
            },
            pos,
            rot,
            mat(METAL, 0.6),
        )
    }

    fn done(mut self, name: &str) -> Prefab {
        let mut g = self.v.object(
            name,
            Shape::Group,
            Transform::default(),
            Material::default(),
        );
        g.children = self.parts;
        Prefab {
            name: name.to_string(),
            object: g,
            models: BTreeMap::new(),
            textures: BTreeMap::new(),
        }
    }
}

/// Bundled objects (names translated).
pub fn builtin() -> Vec<Prefab> {
    let tower = |b: Builder, x: f32, h: f32| {
        b.cube(t("Base plate"), [0.8, 0.02, 0.8], [x, 0.01, 0.0], METAL)
            .truss(h, [x, h * 0.5 + 0.02, 0.0], [0.0, 0.0, 90.0])
    };
    vec![
        Builder::new()
            .cube(t("Riser"), [2.0, 0.4, 1.0], [0.0, 0.2, 0.0], DARK)
            .done(t("Stage riser 2 × 1 m")),
        Builder::new()
            .cube(t("Stage"), [6.0, 0.6, 4.0], [0.0, 0.3, 0.0], DARK)
            .cube(t("Riser"), [1.0, 0.3, 0.4], [2.2, 0.15, 2.2], DARK)
            .done(t("Stage 6 × 4 m")),
        Builder::new()
            .cube(t("Riser"), [1.0, 0.2, 0.3], [0.0, 0.1, 0.3], DARK)
            .cube(t("Riser"), [1.0, 0.4, 0.3], [0.0, 0.2, 0.0], DARK)
            .cube(t("Riser"), [1.0, 0.6, 0.3], [0.0, 0.3, -0.3], DARK)
            .done(t("Stairs")),
        Builder::new()
            .cube("Front", [1.8, 0.88, 0.04], [0.0, 0.44, 0.38], BLACK)
            .cube(t("Side"), [0.04, 0.88, 0.8], [-0.88, 0.44, 0.0], BLACK)
            .cube(t("Side"), [0.04, 0.88, 0.8], [0.88, 0.44, 0.0], BLACK)
            .cube(t("Top"), [1.8, 0.04, 0.8], [0.0, 0.9, 0.0], DARK)
            .cube(t("Mixer"), [0.6, 0.08, 0.4], [0.0, 0.96, -0.1], GREY)
            .done(t("DJ booth")),
        Builder::new()
            .cyl(t("Stand"), 0.02, 1.4, [0.0, 0.7, 0.0], BLACK)
            .cyl(t("Base"), 0.35, 0.02, [0.0, 0.01, 0.0], BLACK)
            .cube(t("Cabinet"), [0.45, 0.75, 0.4], [0.0, 1.78, 0.0], BLACK)
            .done(t("Speaker on stand")),
        Builder::new()
            .cube(t("Cabinet"), [0.6, 0.6, 0.7], [0.0, 0.3, 0.0], BLACK)
            .done(t("Subwoofer")),
        tower(Builder::new(), 0.0, 3.0).done(t("Truss tower 3 m")),
        tower(tower(Builder::new(), -2.0, 3.0), 2.0, 3.0)
            .truss(4.29, [0.0, 3.17, 0.0], [0.0; 3])
            .done(t("Truss goalpost 4 × 3 m")),
        Builder::new()
            .part(
                t("Screen"),
                Shape::Plane { size: [3.0, 2.0] },
                [0.0, 1.9, 0.0],
                [90.0, 0.0, 0.0],
                Material {
                    color: WHITE,
                    gloss: 0.05,
                    emissive: 0.15,
                    ..Material::default()
                },
            )
            .cyl(t("Post"), 0.02, 2.9, [-1.55, 1.45, 0.0], BLACK)
            .cyl(t("Post"), 0.02, 2.9, [1.55, 1.45, 0.0], BLACK)
            .done(t("Screen 3 × 2 m")),
        Builder::new()
            .cyl(t("Top"), 0.35, 0.03, [0.0, 1.1, 0.0], WOOD)
            .cyl(t("Leg"), 0.04, 1.1, [0.0, 0.55, 0.0], METAL)
            .cyl(t("Base"), 0.25, 0.02, [0.0, 0.01, 0.0], METAL)
            .done(t("Bar table")),
        Builder::new()
            .cube(t("Top"), [2.0, 0.04, 0.8], [0.0, 0.74, 0.0], WOOD)
            .cube(t("Leg"), [0.05, 0.72, 0.05], [-0.95, 0.36, -0.35], METAL)
            .cube(t("Leg"), [0.05, 0.72, 0.05], [0.95, 0.36, -0.35], METAL)
            .cube(t("Leg"), [0.05, 0.72, 0.05], [-0.95, 0.36, 0.35], METAL)
            .cube(t("Leg"), [0.05, 0.72, 0.05], [0.95, 0.36, 0.35], METAL)
            .done(t("Table 2 × 0.8 m")),
        Builder::new()
            .cube(t("Body"), [3.0, 1.08, 0.6], [0.0, 0.54, 0.0], WOOD)
            .cube(t("Top"), [3.1, 0.05, 0.7], [0.0, 1.105, 0.0], DARK)
            .done(t("Bar counter 3 m")),
        Builder::new()
            .cyl(t("Torso"), 0.18, 1.45, [0.0, 0.725, 0.0], GREY)
            .cyl(t("Head"), 0.1, 0.24, [0.0, 1.6, 0.0], GREY)
            .done(t("Person 1.75 m")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_insert_with_fresh_ids() {
        let mut v = Venue::room("t", [10.0, 5.0, 10.0]);
        let before = v.flatten().len();
        let list = builtin();
        assert!(list.len() >= 10);
        for p in &list {
            let id = p.insert(&mut v, 1.0, 2.0);
            assert_eq!(v.find(id).unwrap().transform.position[0], 1.0);
        }
        let ids: std::collections::BTreeSet<u32> = v.flatten().iter().map(|(o, _)| o.id).collect();
        assert_eq!(ids.len(), v.flatten().len(), "IDs eindeutig");
        assert!(v.flatten().len() > before + list.len());
    }

    #[test]
    fn prefab_from_venue_roundtrip() {
        let mut v = Venue::room("t", [10.0, 5.0, 10.0]);
        let id = builtin()[3].insert(&mut v, 3.0, 4.0);
        let p = Prefab::from_venue(&v, id).unwrap();
        assert_eq!(p.object.transform.position, [0.0, 0.0, 0.0]);
        let j = serde_json::to_string(&p).unwrap();
        let q: Prefab = serde_json::from_str(&j).unwrap();
        assert_eq!(q.object.children.len(), p.object.children.len());
    }
}
