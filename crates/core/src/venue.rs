//! Veranstaltungsort für den 3D-Visualisierer: Raum, Bauten, Traversen.
//!
//! Einheiten: Meter, rechtshändig, y zeigt nach oben. Drehungen in Grad
//! (Euler, angewendet in der Reihenfolge X, dann Y, dann Z). Gespeichert wird
//! der Ort in der Show und lässt sich als eigene Datei (`.dmxvenue`, JSON)
//! für andere Shows weitergeben.

pub mod ev2;
pub mod model;

use serde::{Deserialize, Serialize};

/// Aktuelle Version des `.dmxvenue`-Formats.
pub const VENUE_FORMAT_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Venue {
    #[serde(default = "format_version")]
    pub format_version: u32,
    pub name: String,
    /// Grundmaße des Raums (Breite x, Höhe y, Tiefe z) – für Kamera und Raster.
    pub size: [f32; 3],
    #[serde(default)]
    pub environment: Environment,
    #[serde(default)]
    pub objects: Vec<Object>,
    #[serde(default)]
    pub cameras: Vec<CameraPreset>,
    #[serde(default)]
    pub next_id: u32,
    /// Ordner für Geräte (nur zum Sortieren der Geräteliste im Visualisierer).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixture_folders: Vec<FixtureFolder>,
    /// Eingebettete 3D-Modelle (Schlüssel → Modell), benutzt von `Shape::Model`.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub models: std::collections::BTreeMap<String, model::Model>,
}

fn format_version() -> u32 {
    VENUE_FORMAT_VERSION
}

/// Ordner der Geräteliste. Ein Gerät steht in höchstens einem Ordner.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixtureFolder {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub fixtures: Vec<crate::FixtureId>,
}

/// Licht und Luft im Raum.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Environment {
    /// Raumlicht 0–1.
    pub ambient: f32,
    /// Sichtbarkeit der Lichtstrahlen 0–1.
    pub beams: f32,
    /// Dunst/Nebel 0–1.
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraPreset {
    pub name: String,
    pub target: [f32; 3],
    /// Drehung um die Hochachse (Grad).
    pub yaw: f32,
    /// Neigung (Grad, positiv = von oben).
    pub pitch: f32,
    pub distance: f32,
}

/// Ein Objekt im Raum; Gruppen enthalten weitere Objekte.
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

/// Form eines Objekts. Maße in Metern (vor der Skalierung).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Shape {
    /// Nur Container für `children`.
    Group,
    /// Quader, Mittelpunkt im Ursprung.
    Box { size: [f32; 3] },
    /// Fläche in der x/z-Ebene (Normale nach +y), Mittelpunkt im Ursprung.
    Plane { size: [f32; 2] },
    /// Zylinder entlang y, Mittelpunkt im Ursprung.
    Cylinder { radius: f32, height: f32 },
    /// Eigenes Modell aus `Venue::models`; Mitte in x/z, Unterkante bei y = 0.
    Model { model: String },
    /// Traverse entlang x, Mittelpunkt im Ursprung.
    Truss {
        length: f32,
        #[serde(default)]
        profile: TrussProfile,
        /// Abstand der Gurtrohre (Kantenlänge des Profils).
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
    /// Dreipunkt.
    #[default]
    Triangle,
    /// Vierpunkt.
    Square,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Transform {
    pub position: [f32; 3],
    /// Euler-Winkel in Grad (X, dann Y, dann Z).
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
    /// sRGB-Farbe.
    pub color: [u8; 3],
    /// Deckkraft 0–1.
    pub opacity: f32,
    /// Glanz 0 (matt) – 1 (spiegelnd).
    pub gloss: f32,
    /// Selbstleuchten 0–1.
    pub emissive: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: [160, 160, 160],
            opacity: 1.0,
            gloss: 0.1,
            emissive: 0.0,
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

/// Lage eines Geräts im 3D-Raum (gespeichert am Gerät der Show).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Placement {
    pub position: [f32; 3],
    /// Euler-Winkel in Grad. Ohne Drehung strahlt ein Gerät nach unten (−y).
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
    /// Richtet das Gerät so aus, dass es auf `target` strahlt
    /// (Drehung um X, dann Z; Y bleibt 0).
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
        // Richtung = Rz·Rx·(0,−1,0) = (cos a·sin c, −cos a·cos c, −sin a)
        let a = (-d[2]).clamp(-1.0, 1.0).asin();
        let c = d[0].atan2(-d[1]);
        let r = |v: f32| (v.to_degrees() * 100.0).round() / 100.0;
        self.rotation = [r(a), 0.0, r(c)];
    }

    /// Richtung, in die das Gerät strahlt (Einheitsvektor).
    pub fn direction(&self) -> [f32; 3] {
        let m = rotation(self.rotation);
        [-m[1][0], -m[1][1], -m[1][2]]
    }
}

/// Aufhängepunkt an einer Traverse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TrussHang {
    pub object: u32,
    /// Punkt unter der Traverse (Welt, m)
    pub point: [f32; 3],
    /// Abstand des gesuchten Punkts zur Traversenachse (m)
    pub distance: f32,
}

impl Default for Venue {
    fn default() -> Self {
        Self::room("Raum", [12.0, 5.0, 16.0])
    }
}

impl Venue {
    /// Leerer Raum mit Boden und vier Wänden.
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
        };
        let [w, h, d] = size;
        let floor = v.object(
            "Boden",
            Shape::Plane { size: [w, d] },
            Transform::default(),
            Material {
                color: [120, 90, 66],
                gloss: 0.25,
                ..Material::default()
            },
        );
        let wall = Material::color([150, 140, 125]);
        let mut walls = v.object("Wände", Shape::Group, Transform::default(), wall);
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
                "Hinten",
                [0.0, h / 2.0, d / 2.0],
                [-90.0, 0.0, 0.0],
                [w, h],
            ),
            mk(
                &mut v,
                "Vorn",
                [0.0, h / 2.0, -d / 2.0],
                [90.0, 0.0, 0.0],
                [w, h],
            ),
            mk(
                &mut v,
                "Links",
                [-w / 2.0, h / 2.0, 0.0],
                [0.0, 0.0, -90.0],
                [h, d],
            ),
            mk(
                &mut v,
                "Rechts",
                [w / 2.0, h / 2.0, 0.0],
                [0.0, 0.0, 90.0],
                [h, d],
            ),
        ];
        v.objects = vec![floor, walls];
        v
    }

    /// Neues Objekt mit frischer ID (noch nicht eingefügt).
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

    /// Objekt suchen (auch in Gruppen).
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

    /// Liste der Kinder einer Gruppe (`None` = oberste Ebene).
    pub fn list_mut(&mut self, parent: Option<u32>) -> Option<&mut Vec<Object>> {
        match parent {
            None => Some(&mut self.objects),
            Some(id) => self.find_mut(id).map(|o| &mut o.children),
        }
    }

    /// Gruppe, in der ein Objekt liegt (`None` = oberste Ebene).
    pub fn parent_of(&self, id: u32) -> Option<u32> {
        self.ancestors(id).last().copied()
    }

    /// Kinder einer Gruppe (bzw. oberste Ebene) nach Namen sortieren.
    pub fn sort_by_name(&mut self, parent: Option<u32>) {
        if let Some(list) = self.list_mut(parent) {
            list.sort_by_key(|o| o.name.to_lowercase());
        }
    }

    /// Ordner, in dem ein Gerät steht.
    pub fn folder_of(&self, f: crate::FixtureId) -> Option<u32> {
        self.fixture_folders
            .iter()
            .find(|d| d.fixtures.contains(&f))
            .map(|d| d.id)
    }

    /// Geräte in einen Ordner verschieben (`None` = aus allen Ordnern nehmen).
    pub fn move_fixtures(&mut self, ids: &[crate::FixtureId], folder: Option<u32>) {
        for d in &mut self.fixture_folders {
            d.fixtures.retain(|f| !ids.contains(f));
        }
        if let Some(d) = folder.and_then(|id| self.fixture_folders.iter_mut().find(|d| d.id == id))
        {
            d.fixtures.extend_from_slice(ids);
        }
    }

    /// Neuer Geräteordner, gibt seine ID zurück.
    pub fn add_fixture_folder(&mut self, name: impl Into<String>) -> u32 {
        let id = self.new_id();
        self.fixture_folders.push(FixtureFolder {
            id,
            name: name.into(),
            fixtures: Vec::new(),
        });
        id
    }

    /// Objekt entfernen (auch aus Gruppen).
    pub fn remove(&mut self, id: u32) -> Option<Object> {
        fn walk(o: &mut Vec<Object>, id: u32) -> Option<Object> {
            if let Some(i) = o.iter().position(|x| x.id == id) {
                return Some(o.remove(i));
            }
            o.iter_mut().find_map(|x| walk(&mut x.children, id))
        }
        walk(&mut self.objects, id)
    }

    /// Alle sichtbaren Objekte mit ihrer Weltmatrix (Spalten, 4×4).
    /// Nächste Traverse zu `p` und der Punkt darunter, an dem ein Gerät hängt.
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
                // Bügel unter dem unteren Gurt
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

    /// Modelle entfernen, die kein Objekt mehr benutzt.
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
    }

    /// Kette der Gruppen über einem Objekt (äußerste zuerst, ohne das Objekt).
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

    /// Weltmatrix des Elternteils eines Objekts (Einheitsmatrix auf oberster Ebene).
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

// ---------- kleine Matrixhilfen (Spaltenmatrizen wie in WGSL) ----------

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

/// Drehmatrix (3×3, Spalten) aus Euler-Winkeln in Grad: erst X, dann Y, dann Z.
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
            "Traverse",
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
        // erst 90° um X (y → z), dann 90° um Y (z → x)
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
