//! Mitgelieferte Bilder für Materialien, im Programm erzeugt (nahtlos kachelbar).

use dmxcraft_core::venue::model::{Blob, Texture};
use image::{Rgb, RgbImage};

use crate::i18n::t;

const N: u32 = 512;

/// Ganzzahl-Hash → 0–1.
fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x8da6_b343)
        ^ (y as u32).wrapping_mul(0xd816_3841)
        ^ seed.wrapping_mul(0xcb1a_b31f);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^= h >> 15;
    (h & 0xffff) as f32 / 65535.0
}

/// Glattes Rauschen mit Periode `period` Zellen (kachelbar).
fn noise(x: f32, y: f32, period: i32, seed: u32) -> f32 {
    let (x0, y0) = (x.floor() as i32, y.floor() as i32);
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let s = |v: f32| v * v * (3.0 - 2.0 * v);
    let w = |a: i32| a.rem_euclid(period);
    let v = |a: i32, b: i32| hash(w(a), w(b), seed);
    let a = v(x0, y0) + (v(x0 + 1, y0) - v(x0, y0)) * s(fx);
    let b = v(x0, y0 + 1) + (v(x0 + 1, y0 + 1) - v(x0, y0 + 1)) * s(fx);
    a + (b - a) * s(fy)
}

/// Mehrere Oktaven Rauschen, 0–1. `u`, `v` in 0–1 (eine Kachel).
fn fbm(u: f32, v: f32, base: i32, seed: u32) -> f32 {
    let mut sum = 0.0;
    let mut amp = 0.5;
    let mut p = base;
    for k in 0..5 {
        sum += noise(u * p as f32, v * p as f32, p, seed + k) * amp;
        amp *= 0.5;
        p *= 2;
    }
    sum / 0.97
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

fn render(f: impl Fn(f32, f32) -> [f32; 3]) -> RgbImage {
    RgbImage::from_fn(N, N, |x, y| {
        let c = f(x as f32 / N as f32, y as f32 / N as f32);
        Rgb(c.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8))
    })
}

fn wood(u: f32, v: f32) -> [f32; 3] {
    // fünf Dielen je Kachel, jede eigener Ton und versetzte Fugen
    let planks = 5.0;
    let row = (v * planks).floor();
    let fv = v * planks - row;
    let shift = hash(row as i32, 3, 7);
    let fu = (u + shift).fract();
    let tone = hash(row as i32, 1, 11);
    let grain = fbm(fu * 0.25, fv * 4.0 + row, 4, 3);
    let lines = ((fv * 9.0 + grain * 6.0) * std::f32::consts::PI).sin() * 0.5 + 0.5;
    let c = mix(
        [0.42, 0.27, 0.15],
        [0.62, 0.42, 0.25],
        tone * 0.6 + lines * 0.4,
    );
    let joint = !(0.012..=0.988).contains(&fv)
        || ((fu * 2.0).fract() < 0.004 && hash(row as i32, 5, 1) > 0.5);
    if joint {
        mix(c, [0.1, 0.06, 0.03], 0.7)
    } else {
        c
    }
}

fn stage(u: f32, v: f32) -> [f32; 3] {
    let n = fbm(u, v, 8, 21);
    let planks = 8.0;
    let fv = (v * planks).fract();
    let base = 0.045 + n * 0.035;
    let c = [base, base, base * 1.05];
    if !(0.01..=0.99).contains(&fv) {
        [0.02, 0.02, 0.02]
    } else {
        c
    }
}

fn concrete(u: f32, v: f32) -> [f32; 3] {
    let n = fbm(u, v, 6, 31);
    let speck = if hash((u * 400.0) as i32, (v * 400.0) as i32, 9) > 0.985 {
        -0.12
    } else {
        0.0
    };
    let g = 0.48 + (n - 0.5) * 0.35 + speck;
    [g, g * 0.99, g * 0.97]
}

fn carpet(u: f32, v: f32) -> [f32; 3] {
    let n = fbm(u, v, 16, 41);
    let fine = hash((u * N as f32) as i32, (v * N as f32) as i32, 5);
    let t = n * 0.6 + fine * 0.4;
    mix([0.28, 0.05, 0.06], [0.45, 0.1, 0.1], t)
}

fn brick(u: f32, v: f32) -> [f32; 3] {
    // 4 Reihen × 2 Steine je Kachel, versetzt
    let rows = 4.0;
    let r = (v * rows).floor();
    let fv = v * rows - r;
    let off = if r as i32 % 2 == 0 { 0.0 } else { 0.25 };
    let x = (u + off).fract() * 2.0;
    let col = x.floor();
    let fu = x - col;
    let mortar = !(0.04..=0.96).contains(&fv) || !(0.02..=0.98).contains(&fu);
    if mortar {
        let g = 0.62 + fbm(u, v, 16, 2) * 0.1;
        return [g, g * 0.97, g * 0.92];
    }
    let tone = hash(r as i32, col as i32 + if off > 0.0 { 9 } else { 0 }, 13);
    let n = fbm(u, v, 12, 17);
    mix([0.45, 0.17, 0.11], [0.62, 0.28, 0.18], tone * 0.6 + n * 0.4)
}

fn tiles(u: f32, v: f32) -> [f32; 3] {
    let n = 4.0;
    let (fu, fv) = ((u * n).fract(), (v * n).fract());
    if !(0.015..=0.985).contains(&fu) || !(0.015..=0.985).contains(&fv) {
        return [0.55, 0.55, 0.53];
    }
    let g = 0.86 + fbm(u, v, 8, 61) * 0.06;
    [g, g, g * 0.98]
}

fn molton(u: f32, v: f32) -> [f32; 3] {
    // schwarzer Bühnenstoff mit senkrechten Falten
    let fold = ((u * 12.0 + fbm(u, v, 2, 71) * 1.5) * std::f32::consts::TAU).sin() * 0.5 + 0.5;
    let g = 0.025 + fold * 0.03 + hash((u * N as f32) as i32, (v * N as f32) as i32, 3) * 0.008;
    [g, g, g * 1.1]
}

/// Erzeuger eines Bilds: (u, v) in 0–1 → Farbe.
pub type Pattern = fn(f32, f32) -> [f32; 3];

/// Mitgelieferte Bilder: Name, empfohlene Kachelgröße (m), Erzeuger.
pub fn builtin() -> Vec<(&'static str, f32, Pattern)> {
    vec![
        (t("Holzdielen"), 2.0, wood),
        (t("Bühnenboden schwarz"), 2.0, stage),
        (t("Beton"), 2.0, concrete),
        (t("Teppich"), 1.0, carpet),
        (t("Ziegel"), 1.0, brick),
        (t("Fliesen"), 1.2, tiles),
        (t("Molton"), 1.5, molton),
    ]
}

/// Bild erzeugen und als PNG speichern.
pub fn make(name: &str, f: Pattern) -> Texture {
    let img = render(f);
    let mut png = Vec::new();
    img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
        .expect("PNG in den Speicher");
    Texture {
        name: name.to_string(),
        data: Blob(png.into()),
    }
}

/// Bilddatei laden (PNG oder JPEG, höchstens 16 MB).
pub fn load_file(path: &std::path::Path) -> Result<Texture, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    if bytes.len() > 16 * 1024 * 1024 {
        return Err(t("Bild zu groß (höchstens 16 MB)").to_string());
    }
    match image::guess_format(&bytes) {
        Ok(image::ImageFormat::Png | image::ImageFormat::Jpeg) => {}
        _ => return Err(t("Nur PNG- oder JPEG-Bilder").to_string()),
    }
    image::load_from_memory(&bytes).map_err(|e| e.to_string())?;
    Ok(Texture {
        name: path
            .file_stem()
            .map_or_else(|| "Bild".into(), |s| s.to_string_lossy().into_owned()),
        data: Blob(bytes.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_are_seamless_and_decode() {
        for (name, _, f) in builtin() {
            let tex = make(name, f);
            let img = image::load_from_memory(&tex.data.0).unwrap().to_rgb8();
            assert_eq!(img.dimensions(), (N, N));
            // Ränder passen zusammen (kleiner Unterschied, keine Kante)
            let diff = |a: &Rgb<u8>, b: &Rgb<u8>| {
                a.0.iter()
                    .zip(b.0)
                    .map(|(x, y)| (*x as i32 - y as i32).abs())
                    .max()
                    .unwrap()
            };
            let mut worst = 0;
            for y in 0..N {
                worst = worst.max(diff(img.get_pixel(0, y), img.get_pixel(N - 1, y)));
            }
            assert!(worst < 90, "{name}: {worst}");
        }
    }
}
