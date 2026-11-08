//! glTF/GLB laden und in ein eingebettetes Modell umrechnen.
//!
//! Übernommen werden Geometrie (mit allen Knoten-Lagen) und die Grundfarbe
//! der Materialien; Texturen werden nicht gelesen (die Farbe bleibt der
//! Grundfarbwert). glTF ist wie DMXCraft rechtshändig, y oben, Meter.

use std::path::Path;

use dmxcraft_core::venue::model::{b64_decode, Model, ModelPart};
use glam::{Mat3, Mat4, Vec3};

/// Mehr Dreiecke werden abgelehnt (Show-Datei und Grafikspeicher).
pub const MAX_TRIANGLES: usize = 2_000_000;

/// Farbe, Ecken (Position + Normale), Dreiecke.
type RawPart = ([u8; 3], Vec<[f32; 6]>, Vec<u32>);

pub fn load(path: &Path) -> Result<Model, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let gltf = gltf::Gltf::from_slice(&bytes).map_err(|e| e.to_string())?;
    let dir = path.parent().unwrap_or(Path::new("."));
    // Puffer: im GLB, als data:-URI oder als Datei daneben
    let mut buffers: Vec<Vec<u8>> = Vec::new();
    for b in gltf.buffers() {
        let data = match b.source() {
            gltf::buffer::Source::Bin => gltf
                .blob
                .clone()
                .ok_or_else(|| "GLB ohne Binärteil".to_string())?,
            gltf::buffer::Source::Uri(uri) => {
                if let Some(rest) = uri.strip_prefix("data:") {
                    let (_, b64) = rest.split_once(";base64,").ok_or("data-URI ohne Base64")?;
                    b64_decode(b64).ok_or("ungültige data-URI")?
                } else {
                    let file = dir.join(percent_decode(uri));
                    std::fs::read(&file).map_err(|e| format!("{}: {e}", file.display()))?
                }
            }
        };
        buffers.push(data);
    }
    // Dreiecke je Farbe sammeln
    let mut parts: Vec<RawPart> = Vec::new();
    let scene = gltf
        .default_scene()
        .or_else(|| gltf.scenes().next())
        .ok_or("Datei enthält keine Szene")?;
    let mut tris = 0usize;
    let mut stack: Vec<(gltf::Node, Mat4)> = scene.nodes().map(|n| (n, Mat4::IDENTITY)).collect();
    while let Some((node, parent)) = stack.pop() {
        let m = parent * Mat4::from_cols_array_2d(&node.transform().matrix());
        for c in node.children() {
            stack.push((c, m));
        }
        let Some(mesh) = node.mesh() else {
            continue;
        };
        let nm = Mat3::from_mat4(m).inverse().transpose();
        let flip = Mat3::from_mat4(m).determinant() < 0.0;
        for prim in mesh.primitives() {
            if prim.mode() != gltf::mesh::Mode::Triangles {
                continue;
            }
            let reader = prim.reader(|b| buffers.get(b.index()).map(Vec::as_slice));
            let Some(pos) = reader.read_positions() else {
                continue;
            };
            let pos: Vec<Vec3> = pos.map(|p| m.transform_point3(Vec3::from(p))).collect();
            let mut idx: Vec<u32> = match reader.read_indices() {
                Some(i) => i.into_u32().collect(),
                None => (0..pos.len() as u32).collect(),
            };
            idx.truncate(idx.len() / 3 * 3);
            if flip {
                for t in idx.chunks_exact_mut(3) {
                    t.swap(1, 2);
                }
            }
            let normals: Vec<Vec3> = match reader.read_normals() {
                Some(n) => n
                    .map(|v| (nm * Vec3::from(v)).normalize_or_zero())
                    .collect(),
                None => smooth_normals(&pos, &idx),
            };
            tris += idx.len() / 3;
            if tris > MAX_TRIANGLES {
                return Err(format!(
                    "Modell zu groß (über {MAX_TRIANGLES} Dreiecke) – bitte vereinfachen"
                ));
            }
            let base = prim.material().pbr_metallic_roughness().base_color_factor();
            let color =
                [base[0], base[1], base[2]].map(|c| (linear_to_srgb(c) * 255.0).round() as u8);
            let part = match parts.iter_mut().find(|p| p.0 == color) {
                Some(p) => p,
                None => {
                    parts.push((color, Vec::new(), Vec::new()));
                    parts.last_mut().expect("eben angelegt")
                }
            };
            let off = part.1.len() as u32;
            part.1.extend(
                pos.iter()
                    .zip(&normals)
                    .map(|(p, n)| [p.x, p.y, p.z, n.x, n.y, n.z]),
            );
            part.2.extend(idx.iter().map(|i| i + off));
        }
    }
    if tris == 0 {
        return Err("Keine Dreiecke gefunden".into());
    }
    // Mitte in x/z, Unterkante auf y = 0
    let (mut lo, mut hi) = (Vec3::splat(f32::MAX), Vec3::splat(f32::MIN));
    for p in parts.iter().flat_map(|p| p.1.iter()) {
        let v = Vec3::new(p[0], p[1], p[2]);
        lo = lo.min(v);
        hi = hi.max(v);
    }
    let shift = Vec3::new((lo.x + hi.x) * 0.5, lo.y, (lo.z + hi.z) * 0.5);
    let parts = parts
        .into_iter()
        .map(|(c, mut v, i)| {
            for p in &mut v {
                p[0] -= shift.x;
                p[1] -= shift.y;
                p[2] -= shift.z;
            }
            ModelPart::new(c, &v, &i)
        })
        .collect();
    Ok(Model {
        name: path
            .file_name()
            .map_or_else(|| "Modell".into(), |s| s.to_string_lossy().into_owned()),
        size: (hi - lo).to_array(),
        parts,
    })
}

fn smooth_normals(pos: &[Vec3], idx: &[u32]) -> Vec<Vec3> {
    let mut n = vec![Vec3::ZERO; pos.len()];
    for t in idx.chunks_exact(3) {
        let [a, b, c] = [t[0], t[1], t[2]].map(|i| i as usize);
        if a.max(b).max(c) >= pos.len() {
            continue;
        }
        let f = (pos[b] - pos[a]).cross(pos[c] - pos[a]);
        n[a] += f;
        n[b] += f;
        n[c] += f;
    }
    n.into_iter().map(|v| v.normalize_or(Vec3::Y)).collect()
}

fn linear_to_srgb(c: f32) -> f32 {
    let c = c.clamp(0.0, 1.0);
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn percent_decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimaler glTF mit einem Dreieck (data-URI), um 90° gedreht und verschoben.
    #[test]
    fn loads_embedded_triangle_with_node_transform() {
        let mut bin = Vec::new();
        for f in [0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0] {
            bin.extend_from_slice(&f.to_le_bytes());
        }
        let uri = dmxcraft_core::venue::model::b64_encode(&bin);
        let json = format!(
            r#"{{"asset":{{"version":"2.0"}},"scene":0,"scenes":[{{"nodes":[0]}}],
            "nodes":[{{"mesh":0,"translation":[5,1,0]}}],
            "meshes":[{{"primitives":[{{"attributes":{{"POSITION":0}},"material":0}}]}}],
            "materials":[{{"pbrMetallicRoughness":{{"baseColorFactor":[1,0,0,1]}}}}],
            "buffers":[{{"byteLength":36,"uri":"data:application/octet-stream;base64,{uri}"}}],
            "bufferViews":[{{"buffer":0,"byteLength":36}}],
            "accessors":[{{"bufferView":0,"componentType":5126,"count":3,"type":"VEC3","min":[0,0,0],"max":[1,2,0]}}]}}"#
        );
        let path = std::env::temp_dir().join("dmxcraft_test_tri.gltf");
        std::fs::write(&path, json).unwrap();
        let m = load(&path).unwrap();
        assert_eq!(m.triangles(), 1);
        assert_eq!(m.parts[0].color, [255, 0, 0]);
        assert_eq!(m.size, [1.0, 2.0, 0.0]);
        // Unterkante bei 0, Mitte in x bei 0
        let v = m.parts[0].vertices();
        assert_eq!(v[0][..3], [-0.5, 0.0, 0.0]);
        assert_eq!(v[2][..3], [-0.5, 2.0, 0.0]);
        // Normale aus den Dreiecken (z)
        assert!((v[0][5].abs() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn percent_decoding() {
        assert_eq!(percent_decode("a%20b.bin"), "a b.bin");
        assert_eq!(percent_decode("x%2"), "x%2");
    }
}
