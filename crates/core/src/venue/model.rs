//! Eigene 3D-Modelle (aus glTF/GLB importiert), in der Show eingebettet.
//!
//! Die Geometrie wird beim Import fertig berechnet (Knoten-Lagen
//! eingerechnet, Normalen vorhanden) und je Materialfarbe zu einem Teil
//! zusammengefasst. Gespeichert werden die Zahlen als Base64 (kompakt im
//! JSON); `Blob` teilt den Inhalt beim Kopieren (Rückgängig-Verlauf).

use std::sync::Arc;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Unveränderliche Bytes, beim Klonen geteilt.
#[derive(Clone, Debug, Default)]
pub struct Blob(pub Arc<[u8]>);

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || self.0 == other.0
    }
}

impl Serialize for Blob {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&b64_encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = String::deserialize(d)?;
        b64_decode(&s)
            .map(|b| Blob(b.into()))
            .ok_or_else(|| serde::de::Error::custom("ungültiges Base64"))
    }
}

/// Ein Bild (PNG oder JPEG, unverändert gespeichert).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Texture {
    pub name: String,
    pub data: Blob,
}

/// Ein importiertes Modell.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
    /// Dateiname beim Import.
    pub name: String,
    /// Ausdehnung (m): Mitte in x/z bei 0, Unterkante bei y = 0.
    pub size: [f32; 3],
    pub parts: Vec<ModelPart>,
    /// Bilder der Materialien (Index aus `ModelPart::texture`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<Texture>,
}

/// Dreiecke mit einer Farbe.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelPart {
    /// sRGB-Farbe
    pub color: [u8; 3],
    /// je Ecke x, y, z, nx, ny, nz (f32, little endian)
    pub vertices: Blob,
    /// Dreiecke (u32, little endian)
    pub indices: Blob,
    /// Bildkoordinaten je Ecke (u, v als f32), wenn das Teil ein Bild hat
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uv: Option<Blob>,
    /// Bild aus `Model::images`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub texture: Option<usize>,
}

impl ModelPart {
    pub fn new(color: [u8; 3], vertices: &[[f32; 6]], indices: &[u32]) -> Self {
        let v: Vec<u8> = vertices
            .iter()
            .flatten()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        let i: Vec<u8> = indices.iter().flat_map(|f| f.to_le_bytes()).collect();
        Self {
            color,
            vertices: Blob(v.into()),
            indices: Blob(i.into()),
            uv: None,
            texture: None,
        }
    }

    /// Bild mit Bildkoordinaten je Ecke setzen.
    pub fn with_texture(mut self, texture: usize, uv: &[[f32; 2]]) -> Self {
        let b: Vec<u8> = uv.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
        self.uv = Some(Blob(b.into()));
        self.texture = Some(texture);
        self
    }

    pub fn uvs(&self) -> Option<Vec<[f32; 2]>> {
        self.uv.as_ref().map(|b| {
            b.0.chunks_exact(8)
                .map(|c| {
                    [
                        f32::from_le_bytes(c[0..4].try_into().unwrap()),
                        f32::from_le_bytes(c[4..8].try_into().unwrap()),
                    ]
                })
                .collect()
        })
    }

    pub fn vertices(&self) -> Vec<[f32; 6]> {
        self.vertices
            .0
            .chunks_exact(24)
            .map(|c| {
                std::array::from_fn(|k| f32::from_le_bytes(c[k * 4..k * 4 + 4].try_into().unwrap()))
            })
            .collect()
    }

    pub fn indices(&self) -> Vec<u32> {
        self.indices
            .0
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect()
    }

    pub fn triangles(&self) -> usize {
        self.indices.0.len() / 12
    }
}

impl Model {
    pub fn triangles(&self) -> usize {
        self.parts.iter().map(ModelPart::triangles).sum()
    }

    /// Schlüssel aus dem Inhalt (gleiche Datei zweimal importiert → ein Modell).
    pub fn content_key(&self) -> String {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for p in &self.parts {
            for b in p
                .color
                .iter()
                .chain(p.vertices.0.iter())
                .chain(p.indices.0.iter())
            {
                h ^= u64::from(*b);
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        }
        format!("m{h:016x}")
    }
}

const B64: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn b64_encode(data: &[u8]) -> String {
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for c in data.chunks(3) {
        let n = (u32::from(c[0]) << 16)
            | (u32::from(*c.get(1).unwrap_or(&0)) << 8)
            | u32::from(*c.get(2).unwrap_or(&0));
        for k in 0..4 {
            if k <= c.len() {
                out.push(B64[(n >> (18 - 6 * k) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

pub fn b64_decode(s: &str) -> Option<Vec<u8>> {
    let mut val = [255u8; 256];
    for (i, c) in B64.iter().enumerate() {
        val[*c as usize] = i as u8;
    }
    let bytes: Vec<u8> = s.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if bytes.len() % 4 != 0 {
        return None;
    }
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    for c in bytes.chunks(4) {
        let pad = c.iter().rev().take_while(|b| **b == b'=').count();
        let mut n = 0u32;
        for (k, b) in c.iter().enumerate() {
            let v = if *b == b'=' && k >= 4 - pad {
                0
            } else {
                val[*b as usize]
            };
            if v == 255 {
                return None;
            }
            n = (n << 6) | u32::from(v);
        }
        let bytes = [(n >> 16) as u8, (n >> 8) as u8, n as u8];
        out.extend_from_slice(&bytes[..3 - pad.min(2)]);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_roundtrip() {
        for n in 0..20 {
            let d: Vec<u8> = (0..n).map(|i| (i * 37 + 5) as u8).collect();
            assert_eq!(b64_decode(&b64_encode(&d)).unwrap(), d);
        }
        assert_eq!(b64_encode(b"Man"), "TWFu");
        assert_eq!(b64_encode(b"Ma"), "TWE=");
    }

    #[test]
    fn part_roundtrip_through_json() {
        let p = ModelPart::new([1, 2, 3], &[[0.0, 1.0, 2.0, 0.0, 1.0, 0.0]; 3], &[0, 1, 2]);
        let j = serde_json::to_string(&p).unwrap();
        let q: ModelPart = serde_json::from_str(&j).unwrap();
        assert_eq!(p, q);
        assert_eq!(q.vertices()[2], [0.0, 1.0, 2.0, 0.0, 1.0, 0.0]);
        assert_eq!(q.indices(), vec![0, 1, 2]);
        assert_eq!(q.triangles(), 1);
    }
}
