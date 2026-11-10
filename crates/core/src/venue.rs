//! Venue for the 3D visualizer: room, structures, trusses.
//!
//! Units: meters, right-handed, y points up. Rotations in degrees (Euler,
//! applied in the order X, then Y, then Z). The venue is saved in the show
//! and can be passed on to other shows as its own file (`.dmxvenue`, JSON).

pub mod ev2;
pub mod model;

use serde::{Deserialize, Serialize};

/// Current version of the `.dmxvenue` format.
pub const VENUE_FORMAT_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    #[serde(default = "format_version")]
    pub format_version: u32,
    pub name: String,
    /// Base dimensions of the room (width x, height y, depth z) – for camera and
    /// grid.
    pub size: [f32; 3],
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub objects: Vec<Object>,
    #[serde(default)]
    pub cameras: Vec<CameraPreset>,
    #[serde(default)]
    pub next_id: u32,
    /// Images for materials (ID → image), used by `Material::texture`.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub textures: std::collections::BTreeMap<u32, model::Texture>,
    /// Folders for fixtures (only for sorting the fixture list in the
    /// visualizer).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture_folders: Vec<FixtureFolder>,
    /// Embedded 3D models (key → model), used by `Shape::Model`.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub models: std::collections::BTreeMap<String, model::Model>,
}

fn format_version() -> u32 {
    VENUE_FORMAT_VERSION
}

/// Folder of the fixture list. A fixture is in at most one folder.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureFolder {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub fixtures: Vec<crate::FixtureId>,
}

/// Light and air in the room.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Environment {
    /// Ambient light 0–1.
    pub ambient: f32,
    /// Visibility of the light beams 0–1.
    pub beams: f32,
    /// Haze/fog 0–1.
    pub fog: f32,
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            ambient: 0.25,
            beams: 0.3,
            fog: 0.2,
        }
    }
}

/// Camera point of the 3D visualizer (reachable in the visualizer's title
/// bar with a click or keys 1–9).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraPreset {
    pub name: String,
    pub target: [f32; 3],
    /// Rotation around the vertical axis (degrees).
    pub yaw: f32,
    /// Tilt (degrees, positive = from above).
    pub pitch: f32,
    pub distance: f32,
    /// Vertical field of view (degrees).
    #[serde(default = "default_fov")]
    pub fov: f32,
}

fn default_fov() -> f32 {
    45.0
}

/// An object in the room; groups contain further objects.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub id: u32,
    pub name: String,
    pub shape: Shape,
    #[serde(default)]
    pub transform: Transform,
    #[serde(default)]
    pub material: Material,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Object>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub locked: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
}

/// Shape of an object. Dimensions in meters (before scaling).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Shape {
    /// Only a container for `children`.
    Group,
    /// Box, center at the origin.
    Box { size: [f32; 3] },
    /// Plane in the x/z plane (normal towards +y), center at the origin.
    Plane { size: [f32; 2] },
    /// Cylinder along y, center at the origin.
    Cylinder { radius: f32, height: f32 },
    /// Custom model from `Venue::models`; centered in x/z, bottom edge at y = 0.
    Model { model: String },
    /// Truss along x, center at the origin.
    Truss {
        length: f32,
        #[serde(default)]
        profile: TrussProfile,
        /// Spacing of the chord tubes (edge length of the profile).
        #[serde(default = "truss_width")]
        width: f32,
    },
}

fn truss_width() -> f32 {
    0.29
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrussProfile {
    /// Three-point.
    #[default]
    Triangle,
    /// Four-point.
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub position: [f32; 3],
    /// Euler angles in degrees (X, then Y, then Z).
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: [1.0; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Material {
    /// sRGB color.
    pub color: [u8; 3],
    /// Opacity 0–1.
    pub opacity: f32,
    /// Gloss 0 (matte) – 1 (mirror).
    pub gloss: f32,
    /// Self-illumination 0–1.
    pub emissive: f32,
    /// Image from `Venue::textures` (0 = none); the color tints it.
    #[serde(skip_serializing_if = "is_zero")]
    pub texture: u32,
    /// Size of one tile of the image in meters.
    pub tile: f32,
}

fn is_zero(v: &u32) -> bool {
    *v == 0
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: [160, 160, 160],
            opacity: 1.0,
            gloss: 0.1,
            emissive: 0.0,
            texture: 0,
            tile: 1.0,
        }
    }
}

impl Material {
    pub fn color(c: [u8; 3]) -> Self {
        Self {
            color: c,
            ..Self::default()
        }
    }
}

/// Placement of a fixture in 3D space (stored on the show's fixture).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Placement {
    pub position: [f32; 3],
    /// Euler angles in degrees. Without rotation a fixture points down (−y).
    pub rotation: [f32; 3],
}

impl Default for Placement {
    fn default() -> Self {
        Self {
            position: [0.0, 4.0, 0.0],
            rotation: [0.0; 3],
        }
    }
}

impl Placement {
    /// Aims the fixture so it points at `target` (rotation around X, then Z;
    /// Y stays 0).
    pub fn aim_at(&mut self, target: [f32; 3]) {
        let d = [
            target[0] - self.position[0],
            target[1] - self.position[1],
            target[2] - self.position[2],
        ];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if len < 1e-4 {
            return;
        }
        let d = d.map(|v| v / len);
        // direction = Rz·Rx·(0,−1,0) = (cos a·sin c, −cos a·cos c, −sin a)
        let a = (-d[2]).clamp(-1.0, 1.0).asin();
        let c = d[0].atan2(-d[1]);
        let r = |v: f32| (v.to_degrees() * 100.0).round() / 100.0;
        self.rotation = [r(a), 0.0, r(c)];
    }

    /// Direction the fixture points in (unit vector).
    pub fn direction(&self) -> [f32; 3] {
        let m = rotation(self.rotation);
        [-m[1][0], -m[1][1], -m[1][2]]
    }
}

/// Beam direction (unit vector, room coordinates) of a moving head with this
/// placement rotation. `pan`/`tilt` 0–1 (0.5 = center), ranges in degrees.
/// Rotation 0 = hanging (beam down at the center); the DMX direction of
/// moving heads is defined for standing fixtures, so hanging both axes turn
/// the other way round. Same model as the 3D visualizer.
pub fn moving_head_direction(
    rotation_deg: [f32; 3],
    pan: f32,
    tilt: f32,
    pan_range: f32,
    tilt_range: f32,
) -> [f32; 3] {
    let p = (0.5 - pan) * pan_range.to_radians();
    let t = (0.5 - tilt) * tilt_range.to_radians();
    let (st, ct) = t.sin_cos();
    let (sp, cp) = p.sin_cos();
    // local: Ry(pan) · Rx(tilt) · (0, −1, 0)
    let l = [-st * sp, -ct, -st * cp];
    let r = rotation(rotation_deg);
    [0, 1, 2].map(|i| r[0][i] * l[0] + r[1][i] * l[1] + r[2][i] * l[2])
}

/// Hanging point on a truss.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrussHang {
    pub object: u32,
    /// Point below the truss (world, m)
    pub point: [f32; 3],
    /// Distance of the point searched for to the truss axis (m)
    pub distance: f32,
}

impl Default for Venue {
    fn default() -> Self {
        Self::room("Room", [12.0, 5.0, 16.0])
    }
}

impl Venue {
    /// Empty room with floor and four walls.
    pub fn room(name: impl Into<String>, size: [f32; 3]) -> Self {
        let mut v = Self {
            format_version: VENUE_FORMAT_VERSION,
            name: name.into(),
            size,
            environment: Environment::default(),
            objects: Vec::new(),
            cameras: Vec::new(),
            next_id: 1,
            models: Default::default(),
            fixture_folders: Vec::new(),
            textures: Default::default(),
        };
        let [w, h, d] = size;
        let floor = v.object(
            "Floor",
            Shape::Plane { size: [w, d] },
            Transform::default(),
            Material {
                color: [120, 90, 66],
                gloss: 0.25,
                ..Material::default()
            },
        );
        let wall = Material::color([150, 140, 125]);
        let mut walls = v.object("Walls", Shape::Group, Transform::default(), wall);
        let mk = |v: &mut Venue, name: &str, pos: [f32; 3], rot: [f32; 3], s: [f32; 2]| {
            v.object(
                name,
                Shape::Plane { size: s },
                Transform {
                    position: pos,
                    rotation: rot,
                    scale: [1.0; 3],
                },
                wall,
            )
        };
        walls.children = vec![
            mk(
                &mut v,
                "Back",
                [0.0, h / 2.0, d / 2.0],
                [-90.0, 0.0, 0.0],
                [w, h],
            ),
            mk(
                &mut v,
                "Front",
                [0.0, h / 2.0, -d / 2.0],
                [90.0, 0.0, 0.0],
                [w, h],
            ),
            mk(
                &mut v,
                "Left",
                [-w / 2.0, h / 2.0, 0.0],
                [0.0, 0.0, -90.0],
                [h, d],
            ),
            mk(
                &mut v,
                "Right",
                [w / 2.0, h / 2.0, 0.0],
                [0.0, 0.0, 90.0],
                [h, d],
            ),
        ];
        v.objects = vec![floor, walls];
        v
    }

    /// New object with a fresh ID (not inserted yet).
    pub fn object(
        &mut self,
        name: &str,
        shape: Shape,
        transform: Transform,
        material: Material,
    ) -> Object {
        let id = self.new_id();
        Object {
            id,
            name: name.into(),
            shape,
            transform,
            material,
            children: Vec::new(),
            locked: false,
            hidden: false,
        }
    }

    pub fn new_id(&mut self) -> u32 {
        self.next_id = self.next_id.max(self.max_id() + 1);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn max_id(&self) -> u32 {
        fn walk(o: &[Object]) -> u32 {
            o.iter()
                .map(|x| x.id.max(walk(&x.children)))
                .max()
                .unwrap_or(0)
        }
        walk(&self.objects)
    }

    /// Find an object (also in groups).
    pub fn find(&self, id: u32) -> Option<&Object> {
        fn walk(o: &[Object], id: u32) -> Option<&Object> {
            o.iter()
                .find_map(|x| (x.id == id).then_some(x).or_else(|| walk(&x.children, id)))
        }
        walk(&self.objects, id)
    }

    pub fn find_mut(&mut self, id: u32) -> Option<&mut Object> {
        fn walk(o: &mut [Object], id: u32) -> Option<&mut Object> {
            for x in o {
                if x.id == id {
                    return Some(x);
                }
                if let Some(f) = walk(&mut x.children, id) {
                    return Some(f);
                }
            }
            None
        }
        walk(&mut self.objects, id)
    }

    /// List of the children of a group (`None` = top level).
    pub fn list_mut(&mut self, parent: Option<u32>) -> Option<&mut Vec<Object>> {
        match parent {
            None => Some(&mut self.objects),
            Some(id) => self.find_mut(id).map(|o| &mut o.children),
        }
    }

    /// Group an object lies in (`None` = top level).
    pub fn parent_of(&self, id: u32) -> Option<u32> {
        self.ancestors(id).last().copied()
    }

    /// Sort the children of a group (or the top level) by name.
    pub fn sort_by_name(&mut self, parent: Option<u32>) {
        if let Some(list) = self.list_mut(parent) {
            list.sort_by_key(|o| o.name.to_lowercase());
        }
    }

    /// Folder a fixture is in.
    pub fn folder_of(&self, f: crate::FixtureId) -> Option<u32> {
        self.fixture_folders
            .iter()
            .find(|d| d.fixtures.contains(&f))
            .map(|d| d.id)
    }

    /// Move fixtures into a folder (`None` = take them out of all folders).
    pub fn move_fixtures(&mut self, ids: &[crate::FixtureId], folder: Option<u32>) {
        for d in &mut self.fixture_folders {
            d.fixtures.retain(|f| !ids.contains(f));
        }
        if let Some(d) = folder.and_then(|id| self.fixture_folders.iter_mut().find(|d| d.id == id))
        {
            d.fixtures.extend_from_slice(ids);
        }
    }

    /// New fixture folder, returns its ID.
    pub fn add_fixture_folder(&mut self, name: impl Into<String>) -> u32 {
        let id = self.new_id();
        self.fixture_folders.push(FixtureFolder {
            id,
            name: name.into(),
            fixtures: Vec::new(),
        });
        id
    }

    /// Remove an object (also from groups).
    pub fn remove(&mut self, id: u32) -> Option<Object> {
        fn walk(o: &mut Vec<Object>, id: u32) -> Option<Object> {
            if let Some(i) = o.iter().position(|x| x.id == id) {
                return Some(o.remove(i));
            }
            o.iter_mut().find_map(|x| walk(&mut x.children, id))
        }
        walk(&mut self.objects, id)
    }

    /// All visible objects with their world matrix (columns, 4×4).
    /// Nearest truss to `p` and the point below it where a fixture hangs.
    pub fn nearest_truss(&self, p: [f32; 3]) -> Option<TrussHang> {
        let mut best: Option<TrussHang> = None;
        for (ob, m) in self.flatten() {
            let Shape::Truss { length, width, .. } = ob.shape else {
                continue;
            };
            let at = |x: f32| -> [f32; 3] {
                [
                    m[0][0] * x + m[3][0],
                    m[0][1] * x + m[3][1],
                    m[0][2] * x + m[3][2],
                ]
            };
            let (a, b) = (at(-length * 0.5), at(length * 0.5));
            let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let ap = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
            let ll = ab.iter().map(|v| v * v).sum::<f32>().max(1e-6);
            let t = (ab.iter().zip(ap).map(|(x, y)| x * y).sum::<f32>() / ll).clamp(0.0, 1.0);
            let c = [a[0] + ab[0] * t, a[1] + ab[1] * t, a[2] + ab[2] * t];
            let distance = c
                .iter()
                .zip(p)
                .map(|(x, y)| (x - y) * (x - y))
                .sum::<f32>()
                .sqrt();
            if best.is_none_or(|b| distance < b.distance) {
                // Clamp under the bottom chord
                let scale = (m[1][0] * m[1][0] + m[1][1] * m[1][1] + m[1][2] * m[1][2]).sqrt();
                let drop = width * 0.5 * scale + 0.12;
                best = Some(TrussHang {
                    object: ob.id,
                    point: [c[0], c[1] - drop, c[2]],
                    distance,
                });
            }
        }
        best
    }

    /// Remove models no object uses any more.
    pub fn prune_models(&mut self) {
        let used: std::collections::BTreeSet<String> = self
            .flatten()
            .into_iter()
            .filter_map(|(o, _)| match &o.shape {
                Shape::Model { model } => Some(model.clone()),
                _ => None,
            })
            .collect();
        self.models.retain(|k, _| used.contains(k));
        let tex: std::collections::BTreeSet<u32> = self
            .flatten()
            .into_iter()
            .map(|(o, _)| o.material.texture)
            .collect();
        self.textures.retain(|k, _| tex.contains(k));
    }

    /// Add an image; an identical image is reused.
    pub fn add_texture(&mut self, t: model::Texture) -> u32 {
        if let Some((id, _)) = self.textures.iter().find(|(_, x)| x.data == t.data) {
            return *id;
        }
        let id = self.new_id();
        self.textures.insert(id, t);
        id
    }

    /// Chain of groups above an object (outermost first, without the object).
    pub fn ancestors(&self, id: u32) -> Vec<u32> {
        fn walk(list: &[Object], id: u32, path: &mut Vec<u32>) -> bool {
            for o in list {
                if o.id == id {
                    return true;
                }
                path.push(o.id);
                if walk(&o.children, id, path) {
                    return true;
                }
                path.pop();
            }
            false
        }
        let mut path = Vec::new();
        if walk(&self.objects, id, &mut path) {
            path
        } else {
            Vec::new()
        }
    }

    /// World matrix of an object's parent (identity matrix at the top level).
    pub fn parent_matrix(&self, id: u32) -> [[f32; 4]; 4] {
        let mut m = IDENTITY;
        let mut list = &self.objects;
        for a in self.ancestors(id) {
            let Some(o) = list.iter().find(|o| o.id == a) else {
                break;
            };
            m = mul(m, o.transform.matrix());
            list = &o.children;
        }
        m
    }

    pub fn flatten(&self) -> Vec<(&Object, [[f32; 4]; 4])> {
        fn walk<'a>(
            o: &'a [Object],
            parent: [[f32; 4]; 4],
            out: &mut Vec<(&'a Object, [[f32; 4]; 4])>,
        ) {
            for x in o.iter().filter(|x| !x.hidden) {
                let m = mul(parent, x.transform.matrix());
                out.push((x, m));
                walk(&x.children, m, out);
            }
        }
        let mut out = Vec::new();
        walk(&self.objects, IDENTITY, &mut out);
        out
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

// ---------- small matrix helpers (column matrices as in WGSL) ----------

pub const IDENTITY: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

/// a · b
pub fn mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut r = [[0.0; 4]; 4];
    for (c, col) in r.iter_mut().enumerate() {
        for (row, v) in col.iter_mut().enumerate() {
            *v = (0..4).map(|k| a[k][row] * b[c][k]).sum();
        }
    }
    r
}

/// Rotation matrix (3×3, columns) from Euler angles in degrees: first X,
/// then Y, then Z.
pub fn rotation(deg: [f32; 3]) -> [[f32; 3]; 3] {
    let [x, y, z] = deg.map(f32::to_radians);
    let (sx, cx) = x.sin_cos();
    let (sy, cy) = y.sin_cos();
    let (sz, cz) = z.sin_cos();
    // Rz · Ry · Rx
    [
        [cy * cz, cy * sz, -sy],
        [sx * sy * cz - cx * sz, sx * sy * sz + cx * cz, sx * cy],
        [cx * sy * cz + sx * sz, cx * sy * sz - sx * cz, cx * cy],
    ]
}

impl Transform {
    pub fn matrix(&self) -> [[f32; 4]; 4] {
        let r = rotation(self.rotation);
        let s = self.scale;
        let p = self.position;
        [
            [r[0][0] * s[0], r[0][1] * s[0], r[0][2] * s[0], 0.0],
            [r[1][0] * s[1], r[1][1] * s[1], r[1][2] * s[1], 0.0],
            [r[2][0] * s[2], r[2][1] * s[2], r[2][2] * s[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn apply(m: [[f32; 4]; 4], v: [f32; 3]) -> [f32; 3] {
        let mut r = [0.0; 3];
        for (i, o) in r.iter_mut().enumerate() {
            *o = m[0][i] * v[0] + m[1][i] * v[1] + m[2][i] * v[2] + m[3][i];
        }
        r
    }

    fn close(a: [f32; 3], b: [f32; 3]) -> bool {
        a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4)
    }

    #[test]
    fn fixture_folders_are_exclusive() {
        use crate::FixtureId;
        let mut v = Venue::room("t", [10.0, 6.0, 10.0]);
        let a = v.add_fixture_folder("A");
        let b = v.add_fixture_folder("B");
        v.move_fixtures(&[FixtureId(1), FixtureId(2)], Some(a));
        v.move_fixtures(&[FixtureId(2)], Some(b));
        assert_eq!(v.folder_of(FixtureId(1)), Some(a));
        assert_eq!(v.folder_of(FixtureId(2)), Some(b));
        v.move_fixtures(&[FixtureId(1)], None);
        assert_eq!(v.folder_of(FixtureId(1)), None);
        let j = v.to_json().unwrap();
        assert_eq!(Venue::from_json(&j).unwrap(), v);
    }

    #[test]
    fn aim_at_points_beam_to_target() {
        for target in [
            [3.0, 0.0, -2.0],
            [0.0, 0.0, 0.0],
            [-4.0, 1.0, 5.0],
            [2.0, 6.0, 0.0],
        ] {
            let mut p = Placement {
                position: [0.5, 4.0, 1.0],
                rotation: [0.0; 3],
            };
            p.aim_at(target);
            let d = p.direction();
            let w = [target[0] - 0.5, target[1] - 4.0, target[2] - 1.0];
            let l = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
            assert!(close(d, w.map(|v| v / l)), "{target:?} → {d:?}");
        }
    }

    #[test]
    fn hang_below_nearest_truss() {
        let mut v = Venue::room("t", [10.0, 6.0, 10.0]);
        let o = v.object(
            "Truss",
            Shape::Truss {
                length: 4.0,
                profile: TrussProfile::Square,
                width: 0.3,
            },
            Transform {
                position: [0.0, 5.0, 0.0],
                ..Default::default()
            },
            Material::default(),
        );
        let id = o.id;
        v.objects.push(o);
        let h = v.nearest_truss([1.0, 4.0, 0.2]).unwrap();
        assert_eq!(h.object, id);
        assert!(close(h.point, [1.0, 5.0 - 0.27, 0.0]), "{:?}", h.point);
    }

    #[test]
    fn rotation_order_x_then_y_then_z() {
        // first 90° around X (y → z), then 90° around Y (z → x)
        let t = Transform {
            rotation: [90.0, 90.0, 0.0],
            ..Transform::default()
        };
        assert!(close(apply(t.matrix(), [0.0, 1.0, 0.0]), [1.0, 0.0, 0.0]));
        let t = Transform {
            position: [1.0, 2.0, 3.0],
            scale: [2.0, 2.0, 2.0],
            ..Transform::default()
        };
        assert!(close(apply(t.matrix(), [1.0, 0.0, 0.0]), [3.0, 2.0, 3.0]));
    }

    #[test]
    fn room_and_tree() {
        let mut v = Venue::room("T", [10.0, 4.0, 8.0]);
        assert_eq!(v.flatten().len(), 6);
        let walls = v.objects[1].id;
        let id = v.new_id();
        let back = v.objects[1].children[0].id;
        assert!(id > back);
        v.find_mut(walls).unwrap().hidden = true;
        assert_eq!(v.flatten().len(), 1);
        assert!(v.remove(back).is_some());
        assert!(v.find(back).is_none());
        let j = v.to_json().unwrap();
        assert_eq!(Venue::from_json(&j).unwrap(), v);
    }
}
