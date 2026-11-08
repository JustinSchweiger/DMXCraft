//! Aufträge des KI-Assistenten (MCP) an den 3D-Visualisierer: Zustand
//! abfragen, Fenster/Qualität/Kamera setzen und Bilder rendern. Die Bilder
//! entstehen ohne offenes Fenster direkt auf der Grafikkarte.

use eframe::egui_wgpu;
use glam::Vec3;
use serde_json::{json, Value};

use super::camera::OrbitCamera;
use super::render::Gpu;
use super::{upload_assets, Quality, VizView};
use crate::views::ViewCtx;
use dmxcraft_core::venue::CameraPreset;

impl VizView {
    /// Auftrag ausführen: `get`, `set` oder `screenshot`.
    pub fn mcp(
        &mut self,
        action: &str,
        args: &Value,
        cx: &mut ViewCtx<'_>,
        rs: Option<&egui_wgpu::RenderState>,
    ) -> Result<dmxcraft_mcp::VizReply, String> {
        let room = cx.engine.show().venue.size;
        if self.camera.is_none() || self.room != room {
            self.camera = Some(OrbitCamera::for_room(room));
            self.room = room;
        }
        match action {
            "get" => Ok((self.info(cx), None)),
            "set" => {
                if let Some(open) = args.get("open").and_then(Value::as_bool) {
                    cx.state.viz_window = open;
                }
                if let Some(q) = args.get("quality").and_then(Value::as_str) {
                    cx.state.viz_quality = quality(q).ok_or_else(|| {
                        format!("Qualität „{q}“ unbekannt – low, medium, high oder ultra")
                    })?;
                }
                let points = cx.engine.show().venue.cameras.clone();
                if let Some(c) = args.get("camera") {
                    let cam = apply_camera(self.camera.expect("Kamera"), room, &points, c)?;
                    // sichtbar hinfahren, wenn das Fenster offen ist
                    self.fly_to(cam);
                    if !cx.state.viz_window {
                        self.camera = Some(cam);
                        self.fly = None;
                    }
                }
                if let Some(name) = args.get("save_point").and_then(Value::as_str) {
                    let name = name.trim();
                    if name.is_empty() {
                        return Err("save_point braucht einen Namen".into());
                    }
                    let cam = match args.get("camera") {
                        Some(c) => apply_camera(self.camera.expect("Kamera"), room, &points, c)?,
                        None => self.camera.expect("Kamera"),
                    };
                    let list = &mut cx.engine.show_mut().venue.cameras;
                    match list.iter_mut().find(|p| p.name.eq_ignore_ascii_case(name)) {
                        Some(p) => *p = cam.to_preset(p.name.clone()),
                        None => list.push(cam.to_preset(name)),
                    }
                }
                if let Some(name) = args.get("delete_point").and_then(Value::as_str) {
                    let list = &mut cx.engine.show_mut().venue.cameras;
                    let i = find_point(list, &Value::String(name.to_string()))
                        .ok_or_else(|| format!("Kamerapunkt „{name}“ nicht gefunden"))?;
                    list.remove(i);
                }
                Ok((self.info(cx), None))
            }
            "screenshot" => {
                let rs = rs
                    .filter(|rs| Gpu::supported(rs))
                    .ok_or("Keine geeignete Grafikkarte für den Visualisierer verfügbar")?;
                let mut cam = self.camera.expect("Kamera");
                if let Some(c) = args.get("camera") {
                    let points = cx.engine.show().venue.cameras.clone();
                    cam = apply_camera(cam, room, &points, c)?;
                }
                let dim = |k: &str, d: u64, lo: u64, hi: u64| {
                    args.get(k)
                        .and_then(Value::as_u64)
                        .unwrap_or(d)
                        .clamp(lo, hi) as u32
                };
                let (w, h) = (dim("width", 1280, 320, 2560), dim("height", 720, 240, 1600));
                let q = cx.state.viz_quality.params();
                let time = self.start.elapsed().as_secs_f32();
                let frame = self.frame3d(cx, &cam, w as f32 / h as f32, time, q, false);
                let gpu = self.gpu.get_or_insert_with(|| Gpu::new(rs));
                upload_assets(gpu, rs, cx.engine.show());
                gpu.render(rs, [w, h], q.msaa, &frame);
                let img = gpu
                    .read_image(rs)
                    .ok_or("Bild konnte nicht gelesen werden")?;
                let mut png = Vec::new();
                img.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
                    .map_err(|e| e.to_string())?;
                let mut info = json!({
                    "width": w,
                    "height": h,
                    "camera": camera_json(&cam),
                    "note": "Bild mit dem aktuellen DMX-Ausgang. Koordinaten: x nach rechts, y nach oben, z zum Publikum (Meter).",
                });
                if cx.engine.blackout() {
                    info["blackout"] = json!(true);
                }
                Ok((info, Some(png)))
            }
            _ => Err(format!("Unbekannte Aktion {action}")),
        }
    }

    fn info(&self, cx: &ViewCtx<'_>) -> Value {
        // während einer Kamerafahrt deren Ziel
        let cam = self.fly.map(|f| f.1).or(self.camera).expect("Kamera");
        json!({
            "window_open": cx.state.viz_window,
            "quality": quality_key(cx.state.viz_quality),
            "camera": camera_json(&cam),
            "room": cx.engine.show().venue.size,
            "camera_points": cx.engine.show().venue.cameras.iter().enumerate().map(|(i, p)| json!({
                "number": i + 1,
                "name": p.name,
                "camera": camera_json(&OrbitCamera::from_preset(p)),
            })).collect::<Vec<_>>(),
        })
    }
}

fn quality(s: &str) -> Option<Quality> {
    Some(match s.to_lowercase().as_str() {
        "low" | "niedrig" => Quality::Low,
        "medium" | "mittel" => Quality::Medium,
        "high" | "hoch" => Quality::High,
        "ultra" => Quality::Ultra,
        _ => return None,
    })
}

fn quality_key(q: Quality) -> &'static str {
    match q {
        Quality::Low => "low",
        Quality::Medium => "medium",
        Quality::High => "high",
        Quality::Ultra => "ultra",
    }
}

fn round2(v: f32) -> f32 {
    (v * 100.0).round() / 100.0
}

fn camera_json(c: &OrbitCamera) -> Value {
    let e = c.eye();
    json!({
        "yaw": round2(c.yaw),
        "pitch": round2(c.pitch),
        "distance": round2(c.distance),
        "target": [round2(c.target.x), round2(c.target.y), round2(c.target.z)],
        "fov": round2(c.fov),
        "eye": [round2(e.x), round2(e.y), round2(e.z)],
    })
}

/// Kamerapunkt nach Name (ohne Groß/Klein) oder Nummer (ab 1).
fn find_point(list: &[CameraPreset], v: &Value) -> Option<usize> {
    if let Some(n) = v.as_u64() {
        return (n >= 1 && (n as usize) <= list.len()).then(|| n as usize - 1);
    }
    let s = v.as_str()?.trim();
    if let Ok(n) = s.parse::<usize>() {
        if n >= 1 && n <= list.len() {
            return Some(n - 1);
        }
    }
    list.iter().position(|p| p.name.eq_ignore_ascii_case(s))
}

/// Kameraangaben anwenden: erst ein Kamerapunkt (`point`) oder eine Ansicht
/// (`view`), dann einzelne Werte. Wie im Ort: z zeigt zum Publikum, „front“
/// blickt also von +z.
fn apply_camera(
    mut cam: OrbitCamera,
    room: [f32; 3],
    points: &[CameraPreset],
    c: &Value,
) -> Result<OrbitCamera, String> {
    if let Some(v) = c.get("point") {
        let i = find_point(points, v).ok_or_else(|| {
            format!(
                "Kamerapunkt {v} nicht gefunden – vorhanden: {}",
                points
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })?;
        cam = OrbitCamera::from_preset(&points[i]);
    }
    let base = OrbitCamera::for_room(room);
    let [w, h, d] = room;
    let far = (w.max(d)) * 1.1 + h;
    if let Some(v) = c.get("view").and_then(Value::as_str) {
        cam = base;
        match v {
            "overview" | "default" => {}
            "front" => {
                cam.yaw = 0.0;
                cam.pitch = 12.0;
                cam.distance = far;
            }
            "back" => {
                cam.yaw = 180.0;
                cam.pitch = 12.0;
                cam.distance = far;
            }
            "left" => {
                cam.yaw = 270.0;
                cam.pitch = 12.0;
                cam.distance = far;
            }
            "right" => {
                cam.yaw = 90.0;
                cam.pitch = 12.0;
                cam.distance = far;
            }
            "top" => {
                cam.yaw = 0.0;
                cam.pitch = 89.0;
                cam.distance = far * 1.2;
                cam.target = Vec3::ZERO;
            }
            other => {
                return Err(format!(
                    "Ansicht „{other}“ unbekannt – front, back, left, right, top oder overview"
                ))
            }
        }
    }
    let num = |k: &str| c.get(k).and_then(Value::as_f64).map(|v| v as f32);
    if let Some(v) = num("yaw") {
        cam.yaw = v.rem_euclid(360.0);
    }
    if let Some(v) = num("pitch") {
        cam.pitch = v.clamp(-89.0, 89.0);
    }
    if let Some(v) = num("distance") {
        cam.distance = v.clamp(0.5, 300.0);
    }
    if let Some(v) = num("fov") {
        cam.fov = v.clamp(10.0, 120.0);
    }
    if let Some(t) = c.get("target").and_then(Value::as_array) {
        let t: Vec<f32> = t
            .iter()
            .filter_map(Value::as_f64)
            .map(|v| v as f32)
            .collect();
        if t.len() != 3 {
            return Err("target braucht [x, y, z]".into());
        }
        cam.target = Vec3::new(t[0], t[1], t[2]);
    }
    Ok(cam)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_views_and_values() {
        let room = [10.0, 5.0, 8.0];
        let c = OrbitCamera::for_room(room);
        let top = apply_camera(c, room, &[], &json!({ "view": "top" })).unwrap();
        assert!(top.pitch > 80.0);
        let f = apply_camera(
            c,
            room,
            &[],
            &json!({ "view": "front", "pitch": 30, "target": [1, 2, 3] }),
        )
        .unwrap();
        assert_eq!(f.yaw, 0.0);
        assert_eq!(f.pitch, 30.0);
        assert_eq!(f.target, Vec3::new(1.0, 2.0, 3.0));
        // von vorn: Auge auf der Publikumsseite (+z)
        assert!(
            apply_camera(c, room, &[], &json!({ "view": "front" }))
                .unwrap()
                .eye()
                .z
                > 0.0
        );
        assert!(apply_camera(c, room, &[], &json!({ "view": "sideways" })).is_err());
        assert_eq!(quality("Hoch"), Some(Quality::High));
        // Kamerapunkt nach Name oder Nummer
        let pts = vec![top.to_preset("Bühne")];
        let p = apply_camera(c, room, &pts, &json!({ "point": "bühne" })).unwrap();
        assert!(p.near(&top));
        assert!(apply_camera(c, room, &pts, &json!({ "point": 1 }))
            .unwrap()
            .near(&top));
        assert!(apply_camera(c, room, &pts, &json!({ "point": "x" })).is_err());
    }
}
