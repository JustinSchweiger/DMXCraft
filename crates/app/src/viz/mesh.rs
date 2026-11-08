//! Einfache Netze (Dreiecke mit Normalen) für den Visualisierer.

use glam::{Mat4, Vec3};

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
    /// Bildkoordinaten (nur Modelle mit Bild; Grundformen projizieren auf)
    pub uv: [f32; 2],
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    fn push(&mut self, p: Vec3, n: Vec3) -> u32 {
        self.vertices.push(Vertex {
            pos: p.into(),
            normal: n.into(),
            uv: [0.0; 2],
        });
        self.vertices.len() as u32 - 1
    }

    fn quad(&mut self, a: Vec3, b: Vec3, c: Vec3, d: Vec3, n: Vec3) {
        let i = [a, b, c, d].map(|p| self.push(p, n));
        self.indices.extend([i[0], i[1], i[2], i[0], i[2], i[3]]);
    }

    /// Anderes Netz mit Matrix anhängen.
    pub fn append(&mut self, other: &Mesh, m: Mat4) {
        let base = self.vertices.len() as u32;
        let nm = m.inverse().transpose();
        for v in &other.vertices {
            let p = m.transform_point3(Vec3::from(v.pos));
            let n = nm
                .transform_vector3(Vec3::from(v.normal))
                .normalize_or_zero();
            self.vertices.push(Vertex {
                pos: p.into(),
                normal: n.into(),
                uv: v.uv,
            });
        }
        self.indices.extend(other.indices.iter().map(|i| i + base));
    }
}

/// Würfel mit Kantenlänge 1, Mittelpunkt im Ursprung.
pub fn cube() -> Mesh {
    let mut m = Mesh::default();
    let h = 0.5;
    let faces: [(Vec3, Vec3, Vec3); 6] = [
        (Vec3::X, Vec3::Y, Vec3::Z),
        (Vec3::NEG_X, Vec3::Y, Vec3::NEG_Z),
        (Vec3::Y, Vec3::Z, Vec3::X),
        (Vec3::NEG_Y, Vec3::NEG_Z, Vec3::X),
        (Vec3::Z, Vec3::Y, Vec3::NEG_X),
        (Vec3::NEG_Z, Vec3::Y, Vec3::X),
    ];
    for (n, u, v) in faces {
        let c = n * h;
        let (u, v) = (u * h, v * h);
        // gegen den Uhrzeigersinn von außen gesehen
        m.quad(c - u - v, c + u - v, c + u + v, c - u + v, n);
    }
    m
}

/// Fläche 1 × 1 in der x/z-Ebene, Normale +y.
pub fn plane() -> Mesh {
    let mut m = Mesh::default();
    let h = 0.5;
    m.quad(
        Vec3::new(-h, 0.0, -h),
        Vec3::new(-h, 0.0, h),
        Vec3::new(h, 0.0, h),
        Vec3::new(h, 0.0, -h),
        Vec3::Y,
    );
    m
}

/// Zylinder: Radius 1, Höhe 1 entlang y, Mittelpunkt im Ursprung, mit Deckeln.
pub fn cylinder(segments: u32) -> Mesh {
    let mut m = Mesh::default();
    let ring = |k: u32| {
        let a = k as f32 / segments as f32 * std::f32::consts::TAU;
        Vec3::new(a.cos(), 0.0, a.sin())
    };
    for k in 0..segments {
        let (a, b) = (ring(k), ring(k + 1));
        let (na, nb) = (a, b);
        let i = [
            m.push(a + Vec3::Y * 0.5, na),
            m.push(b + Vec3::Y * 0.5, nb),
            m.push(b - Vec3::Y * 0.5, nb),
            m.push(a - Vec3::Y * 0.5, na),
        ];
        m.indices.extend([i[0], i[1], i[2], i[0], i[2], i[3]]);
        // Deckel
        let top = [Vec3::Y * 0.5, b + Vec3::Y * 0.5, a + Vec3::Y * 0.5].map(|p| m.push(p, Vec3::Y));
        m.indices.extend(top);
        let bot = [Vec3::NEG_Y * 0.5, a - Vec3::Y * 0.5, b - Vec3::Y * 0.5]
            .map(|p| m.push(p, Vec3::NEG_Y));
        m.indices.extend(bot);
    }
    m
}

/// Lichtkegel: Spitze im Ursprung, Achse −y, Länge 1, Radius 1 am Ende (offen).
pub fn cone(segments: u32) -> Mesh {
    let mut m = Mesh::default();
    for k in 0..segments {
        let a0 = k as f32 / segments as f32 * std::f32::consts::TAU;
        let a1 = (k + 1) as f32 / segments as f32 * std::f32::consts::TAU;
        let p0 = Vec3::new(a0.cos(), -1.0, a0.sin());
        let p1 = Vec3::new(a1.cos(), -1.0, a1.sin());
        // Normale senkrecht zur Mantelfläche
        let n0 = Vec3::new(a0.cos(), 1.0, a0.sin()).normalize();
        let n1 = Vec3::new(a1.cos(), 1.0, a1.sin()).normalize();
        let i = [
            m.push(Vec3::ZERO, (n0 + n1).normalize()),
            m.push(p1, n1),
            m.push(p0, n0),
        ];
        m.indices.extend(i);
    }
    m
}

/// Rohr von `a` nach `b` mit Radius `r` (aus einem Zylinder).
fn tube(out: &mut Mesh, cyl: &Mesh, a: Vec3, b: Vec3, r: f32) {
    let d = b - a;
    let len = d.length();
    if len < 1e-5 {
        return;
    }
    let rot = glam::Quat::from_rotation_arc(Vec3::Y, d / len);
    let m = Mat4::from_scale_rotation_translation(Vec3::new(r, len, r), rot, (a + b) * 0.5);
    out.append(cyl, m);
}

/// Traverse entlang x (Mittelpunkt im Ursprung): Gurtrohre plus Diagonalen.
pub fn truss(length: f32, width: f32, square: bool) -> Mesh {
    let cyl = cylinder(8);
    let mut m = Mesh::default();
    let h = width * 0.5;
    // Lage der Gurtrohre im Querschnitt (y/z)
    let corners: Vec<(f32, f32)> = if square {
        vec![(-h, -h), (h, -h), (h, h), (-h, h)]
    } else {
        let r = width / 3f32.sqrt();
        (0..3)
            .map(|k| {
                let a = std::f32::consts::FRAC_PI_2 + k as f32 * std::f32::consts::TAU / 3.0;
                (a.sin() * r, a.cos() * r)
            })
            .collect()
    };
    let x0 = -length * 0.5;
    let x1 = length * 0.5;
    for &(y, z) in &corners {
        tube(
            &mut m,
            &cyl,
            Vec3::new(x0, y, z),
            Vec3::new(x1, y, z),
            0.024,
        );
    }
    // Diagonalen zwischen benachbarten Gurten, Zickzack
    let segs = (length / (width * 1.6)).round().max(1.0) as usize;
    let step = length / segs as f32;
    for k in 0..corners.len() {
        let (ya, za) = corners[k];
        let (yb, zb) = corners[(k + 1) % corners.len()];
        for s in 0..segs {
            let xa = x0 + s as f32 * step;
            let xb = xa + step;
            let (p, q) = if s % 2 == 0 {
                (Vec3::new(xa, ya, za), Vec3::new(xb, yb, zb))
            } else {
                (Vec3::new(xa, yb, zb), Vec3::new(xb, ya, za))
            };
            tube(&mut m, &cyl, p, q, 0.009);
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meshes_are_valid() {
        for m in [
            cube(),
            plane(),
            cylinder(12),
            cone(16),
            truss(3.0, 0.29, false),
            truss(2.0, 0.29, true),
        ] {
            assert!(!m.indices.is_empty());
            assert_eq!(m.indices.len() % 3, 0);
            assert!(m.indices.iter().all(|i| (*i as usize) < m.vertices.len()));
        }
        // Würfel: Normalen zeigen nach außen und passen zur Windung
        let c = cube();
        for t in c.indices.chunks(3) {
            let [a, b, d] = [t[0], t[1], t[2]].map(|i| Vec3::from(c.vertices[i as usize].pos));
            let n = (b - a).cross(d - a).normalize();
            assert!(n.dot(Vec3::from(c.vertices[t[0] as usize].normal)) > 0.9);
        }
    }
}
