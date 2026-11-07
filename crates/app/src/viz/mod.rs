//! 3D-Visualisierer: Veranstaltungsort bauen und die Geräte live sehen.
//!
//! Gezeichnet wird auf der Grafikkarte (wgpu: Vulkan, Metal oder DirectX 12)
//! in eine Textur, die egui anzeigt – im Reiter „3D“ oder in einem eigenen
//! Fenster (z. B. auf einem zweiten Bildschirm).

pub mod camera;
pub mod mesh;
pub mod render;
pub mod rig;

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Instant;

use dmxcraft_core::venue::{
    self, Material, Object, Placement, Shape, Transform, TrussProfile, Venue,
};
use dmxcraft_core::{FixtureId, Show};
use eframe::egui::{self, pos2, vec2, Color32, Rect, Sense, Stroke};
use eframe::egui_wgpu;
use glam::{Mat4, Vec2, Vec3};

use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, Segmented};
use crate::views::ViewCtx;
use camera::OrbitCamera;
use render::{Frame3d, Gpu, Instance, Light, MeshKey};

/// Darstellungsqualität.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    Low,
    #[default]
    Medium,
    High,
    Ultra,
}

impl Quality {
    pub const ALL: [Quality; 4] = [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra];

    pub fn label(self) -> &'static str {
        match self {
            Quality::Low => t("Niedrig"),
            Quality::Medium => t("Mittel"),
            Quality::High => t("Hoch"),
            Quality::Ultra => t("Ultra"),
        }
    }

    pub fn key(self) -> u8 {
        self as u8
    }

    pub fn from_key(k: u8) -> Self {
        Self::ALL.get(usize::from(k)).copied().unwrap_or_default()
    }

    /// Einstellungen der Stufe.
    fn params(self) -> QualityParams {
        let p = |scale, msaa, lights, shadows, shadow_size, bloom| QualityParams {
            scale,
            msaa,
            lights,
            shadows,
            shadow_size,
            bloom,
        };
        match self {
            Quality::Low => p(0.6, 1, 32, 0, 0, 0.0),
            Quality::Medium => p(1.0, 4, 96, 8, 512, 0.6),
            Quality::High => p(1.0, 4, 256, 16, 1024, 0.8),
            Quality::Ultra => p(1.5, 4, 256, 32, 1024, 0.8),
        }
    }
}

/// Auflösung (Anteil der Pixel), Mehrfachabtastung, Lichtquellen,
/// Scheinwerfer mit Schatten, Größe der Schattenkarten, Bloom-Stärke.
#[derive(Clone, Copy, Debug)]
struct QualityParams {
    scale: f32,
    msaa: u32,
    lights: usize,
    shadows: usize,
    shadow_size: u32,
    bloom: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Live,
    Build,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pick {
    Object(u32),
    Fixture(FixtureId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DialogKind {
    ImportEv2,
    LoadVenue,
    SaveVenue,
}

pub struct VizView {
    gpu: Option<Gpu>,
    camera: Option<OrbitCamera>,
    room: [f32; 3],
    mode: Mode,
    selected: Option<Pick>,
    start: Instant,
    /// Ziehen eines Objekts auf dem Boden: Startpunkt und Ausgangslage.
    drag: Option<(Vec3, [f32; 3])>,
    dialog: Option<(DialogKind, Receiver<Option<PathBuf>>)>,
    fps: (Instant, u32, f32),
    /// Geräte aus dem letzten Import ohne Gegenstück im Patch.
    missing: Vec<String>,
    /// Griff, an dem gerade gezogen wird (Achse 0–2, 3 = Drehung um y),
    /// Ausgangslage und Startpunkt des Zeigers.
    gizmo: Option<(usize, Placement, egui::Pos2)>,
    /// Zielpunkt zum Ausrichten von Geräten.
    aim: [f32; 3],
    /// Nächster Klick ins Bild setzt den Zielpunkt.
    aim_pick: bool,
}

/// Achsenfarben der Griffe (x rot, y grün, z blau).
const AXIS_COLORS: [Color32; 4] = [
    Color32::from_rgb(230, 80, 80),
    Color32::from_rgb(90, 200, 90),
    Color32::from_rgb(80, 140, 240),
    Color32::from_rgb(235, 190, 60),
];

/// Welt- in Bildschirmkoordinaten.
fn project(vp: Mat4, rect: Rect, p: Vec3) -> Option<egui::Pos2> {
    let c = vp * p.extend(1.0);
    if c.w <= 0.01 {
        return None;
    }
    let n = c.truncate() / c.w;
    Some(pos2(
        rect.left() + (n.x * 0.5 + 0.5) * rect.width(),
        rect.top() + (0.5 - n.y * 0.5) * rect.height(),
    ))
}

/// Griffe um `pos`: drei Achsen (Anfang, Ende) und der Drehring (Punkte).
struct Handles {
    center: egui::Pos2,
    axes: [Option<(egui::Pos2, egui::Pos2)>; 3],
    ring: Vec<egui::Pos2>,
    /// Weltlänge einer Achse
    len: f32,
}

fn handles(vp: Mat4, eye: Vec3, rect: Rect, pos: Vec3) -> Option<Handles> {
    let center = project(vp, rect, pos)?;
    let len = (pos.distance(eye) * 0.12).clamp(0.15, 3.0);
    let axes =
        [Vec3::X, Vec3::Y, Vec3::Z].map(|a| Some((center, project(vp, rect, pos + a * len)?)));
    let ring = (0..=48)
        .filter_map(|k| {
            let a = k as f32 / 48.0 * std::f32::consts::TAU;
            project(
                vp,
                rect,
                pos + Vec3::new(a.cos(), 0.0, a.sin()) * len * 0.75,
            )
        })
        .collect();
    Some(Handles {
        center,
        axes,
        ring,
        len,
    })
}

fn seg_dist(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_sq().max(1e-3)).clamp(0.0, 1.0);
    (a + ab * t).distance(p)
}

impl Handles {
    /// Griff unter dem Zeiger (Achsen vor dem Ring).
    fn hit(&self, p: egui::Pos2) -> Option<usize> {
        let axis = self
            .axes
            .iter()
            .enumerate()
            .filter_map(|(i, a)| a.map(|(s, e)| (i, seg_dist(p, s, e))))
            .filter(|(_, d)| *d < 7.0)
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((i, _)) = axis {
            return Some(i);
        }
        self.ring
            .windows(2)
            .any(|w| seg_dist(p, w[0], w[1]) < 6.0)
            .then_some(3)
    }

    fn paint(&self, painter: &egui::Painter, active: Option<usize>, hover: Option<usize>) {
        let width = |i: usize| {
            if active == Some(i) || hover == Some(i) {
                4.0
            } else {
                2.5
            }
        };
        painter.add(egui::Shape::line(
            self.ring.clone(),
            egui::Stroke::new(width(3), AXIS_COLORS[3]),
        ));
        for (i, a) in self.axes.iter().enumerate() {
            if let Some((s, e)) = a {
                painter.line_segment([*s, *e], egui::Stroke::new(width(i), AXIS_COLORS[i]));
                painter.circle_filled(*e, width(i) + 2.5, AXIS_COLORS[i]);
            }
        }
        painter.circle_filled(self.center, 3.5, Color32::WHITE);
    }
}

impl Default for VizView {
    fn default() -> Self {
        Self {
            gpu: None,
            camera: None,
            room: [0.0; 3],
            mode: Mode::Live,
            selected: None,
            start: Instant::now(),
            drag: None,
            dialog: None,
            fps: (Instant::now(), 0, 0.0),
            missing: Vec::new(),
            gizmo: None,
            aim: [0.0; 3],
            aim_pick: false,
        }
    }
}

const VENUE_EXT: &str = "dmxvenue";

impl VizView {
    /// `window`: läuft im eigenen Fenster (dort: Vollbild-Schalter statt „Fenster“).
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        cx: &mut ViewCtx<'_>,
        rs: Option<&egui_wgpu::RenderState>,
        window: bool,
    ) {
        self.poll_dialog(cx);
        let locked = cx.state.locked;
        let size = cx.engine.show().venue.size;
        if self.camera.is_none() || self.room != size {
            self.camera = Some(OrbitCamera::for_room(size));
            self.room = size;
        }
        self.toolbar(ui, cx, locked, window);
        ui.add_space(theme::SP_XS);
        if self.mode == Mode::Build && !locked {
            egui::Panel::right("viz_build")
                .resizable(true)
                .default_size(320.0)
                .min_size(260.0)
                .frame(
                    egui::Frame::new()
                        .fill(theme::surface())
                        .inner_margin(egui::Margin::same(6)),
                )
                .show(ui, |ui| self.build_panel(ui, cx));
        }
        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new())
            .show(ui, |ui| self.viewport(ui, cx, rs));
    }

    fn toolbar(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, locked: bool, window: bool) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            let mut mode = self.mode;
            Segmented::new(&[(Mode::Build, t("Bauen")), (Mode::Live, t("Live"))])
                .show(ui, &mut mode);
            if !locked {
                self.mode = mode;
            }
            ui.separator();
            let env = &mut cx.engine.show_mut().venue.environment;
            for (label, v) in [
                (t("Umgebung"), &mut env.ambient),
                (t("Strahlen"), &mut env.beams),
                (t("Nebel"), &mut env.fog),
            ] {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 0.0;
                    ui.label(
                        egui::RichText::new(label.to_uppercase())
                            .small()
                            .color(theme::text_dim()),
                    );
                    let mut pct = (*v * 100.0).round() as i32;
                    let r = ui.add_enabled(
                        !locked,
                        egui::DragValue::new(&mut pct)
                            .range(0..=100)
                            .suffix(" %")
                            .speed(0.5),
                    );
                    if r.changed() {
                        *v = pct as f32 / 100.0;
                    }
                });
            }
            ui.separator();
            let mut q = cx.state.viz_quality;
            egui::ComboBox::from_id_salt("viz_quality")
                .selected_text(tr!("Qualität: {q}", q = q.label()))
                .show_ui(ui, |ui| {
                    for v in Quality::ALL {
                        ui.selectable_value(&mut q, v, v.label());
                    }
                });
            cx.state.viz_quality = q;
            if ui
                .add(IconButton::new(Icon::ZoomFit))
                .on_hover_text(t("Ansicht zurücksetzen"))
                .clicked()
            {
                self.camera = Some(OrbitCamera::for_room(self.room));
            }
            ui.separator();
            if ui
                .add(IconButton::new(Icon::Import).enabled(!locked))
                .on_hover_text(t("Easy View 2 importieren (als XML) …"))
                .clicked()
            {
                self.open_dialog(ui.ctx(), DialogKind::ImportEv2);
            }
            if ui
                .add(IconButton::new(Icon::FileOpen).enabled(!locked))
                .on_hover_text(t("Veranstaltungsort laden …"))
                .clicked()
            {
                self.open_dialog(ui.ctx(), DialogKind::LoadVenue);
            }
            if ui
                .add(IconButton::new(Icon::Save))
                .on_hover_text(t("Veranstaltungsort speichern unter …"))
                .clicked()
            {
                self.open_dialog(ui.ctx(), DialogKind::SaveVenue);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if window {
                    let full = ui.ctx().input(|i| i.viewport().fullscreen.unwrap_or(false));
                    if ui
                        .add(IconButton::new(Icon::Fullscreen).active(full))
                        .on_hover_text(t("Vollbild"))
                        .clicked()
                    {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!full));
                    }
                } else if ui
                    .add(IconButton::new(Icon::Fullscreen).active(cx.state.viz_window))
                    .on_hover_text(t("In eigenem Fenster öffnen (z. B. zweiter Bildschirm)"))
                    .clicked()
                {
                    cx.state.viz_window = !cx.state.viz_window;
                }
            });
        });
    }

    fn viewport(
        &mut self,
        ui: &mut egui::Ui,
        cx: &mut ViewCtx<'_>,
        rs: Option<&egui_wgpu::RenderState>,
    ) {
        let rect = ui.available_rect_before_wrap();
        let resp = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        let Some(rs) = rs else {
            painter.rect_filled(rect, 0.0, Color32::BLACK);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                t("Keine Grafikkarte verfügbar"),
                theme::body(),
                theme::text_dim(),
            );
            return;
        };
        let cam = self
            .camera
            .get_or_insert_with(|| OrbitCamera::for_room(self.room));
        let aspect = rect.width() / rect.height().max(1.0);
        // ----- Eingabe -----
        let (scroll, ptr, mods) = ui.input(|i| {
            (
                i.smooth_scroll_delta.y + (i.zoom_delta() - 1.0) * 200.0,
                i.pointer.hover_pos(),
                i.modifiers,
            )
        });
        if resp.hovered() && scroll != 0.0 {
            cam.zoom((-scroll * 0.0015).exp());
        }
        let delta = resp.drag_delta();
        let uv = |p: egui::Pos2| {
            Vec2::new(
                (p.x - rect.left()) / rect.width(),
                (p.y - rect.top()) / rect.height(),
            )
        };
        let building = self.mode == Mode::Build && !cx.state.locked;
        // Griffe der Auswahl (Achsen verschieben, Ring dreht um y)
        let vp = cam.proj(aspect) * cam.view();
        let sel_place = match self.selected {
            Some(Pick::Object(id)) if building => {
                cx.engine.show().venue.find(id).map(|o| Placement {
                    position: o.transform.position,
                    rotation: o.transform.rotation,
                })
            }
            Some(Pick::Fixture(id)) if building => Some(rig::placement(cx.engine.show(), id)),
            _ => None,
        };
        let gizmo = sel_place.and_then(|p| handles(vp, cam.eye(), rect, Vec3::from(p.position)));
        let gizmo_hover = gizmo.as_ref().zip(ptr).and_then(|(g, p)| g.hit(p));
        if resp.drag_started_by(egui::PointerButton::Primary) {
            // dort prüfen, wo die Taste gedrückt wurde (bei wenig Bildern pro Sekunde
            // ist der Zeiger beim Erkennen des Ziehens schon weiter)
            let origin = ui.input(|i| i.pointer.press_origin());
            let hit = gizmo
                .as_ref()
                .zip(origin)
                .and_then(|(g, o)| g.hit(o).map(|i| (i, o)));
            if let (Some((i, o)), Some(p0)) = (hit, sel_place) {
                self.gizmo = Some((i, p0, o));
            }
        }
        if let (Some((axis, p0, start)), Some(g), Some(p), Some(sel)) =
            (self.gizmo, gizmo.as_ref(), ptr, self.selected)
        {
            let mut np = p0;
            if axis < 3 {
                if let Some((a, b)) = g.axes[axis] {
                    let s = b - a;
                    let along = (p - start).dot(s) / s.length_sq().max(1.0) * g.len;
                    let v = p0.position[axis] + along;
                    np.position[axis] = if mods.command {
                        v
                    } else {
                        (v * 20.0).round() / 20.0
                    };
                }
            } else {
                let ang = |q: egui::Pos2| (q.y - g.center.y).atan2(q.x - g.center.x);
                let mut d = (ang(p) - ang(start)).to_degrees();
                // Bildschirm-y zeigt nach unten: Drehsinn hängt von der Blickrichtung ab
                if cam.eye().y < p0.position[1] {
                    d = -d;
                }
                let d = if mods.command {
                    d
                } else {
                    (d / 5.0).round() * 5.0
                };
                match sel {
                    Pick::Object(_) => np.rotation[1] = p0.rotation[1] - d,
                    Pick::Fixture(_) => {
                        let dir = Vec3::from(p0.direction());
                        let rot = glam::Quat::from_rotation_y(-d.to_radians());
                        np.aim_at((Vec3::from(p0.position) + rot * dir).into());
                    }
                }
            }
            apply_placement(cx, sel, np);
        }
        if self.gizmo.is_some() {
            // Griff zieht – die Kamera bleibt stehen
        } else if resp.dragged_by(egui::PointerButton::Secondary)
            || resp.dragged_by(egui::PointerButton::Middle)
        {
            cam.pan(Vec2::new(delta.x, delta.y), rect.height());
        } else if resp.dragged_by(egui::PointerButton::Primary) {
            // Bauen: ausgewähltes Objekt/Gerät auf dem Boden verschieben (Umschalt: Höhe)
            if let (true, Some(sel), Some(p)) = (building, self.selected, ptr) {
                let (o, d) = cam.ray(uv(p), aspect);
                let show = cx.engine.show();
                let pos = match sel {
                    Pick::Object(id) => show.venue.find(id).map(|o| o.transform.position),
                    Pick::Fixture(id) => Some(rig::placement(show, id).position),
                };
                if resp.drag_started() {
                    self.drag = pos.and_then(|pos| hit_y(o, d, pos[1]).map(|h| (h, pos)));
                    // nur ziehen, wenn auf/nahe der Auswahl begonnen
                    if let (Some((h, p0)), Some(pick)) =
                        (self.drag, Self::pick(cx.engine.show(), o, d))
                    {
                        let near = Vec3::from(p0).distance(h) < 1.5;
                        if pick != sel && !near {
                            self.drag = None;
                        }
                    }
                }
                if let Some((h0, p0)) = self.drag {
                    let new = hit_y(o, d, p0[1]).map_or(p0, |h| {
                        let dv = h - h0;
                        [p0[0] + dv.x, p0[1], p0[2] + dv.z]
                    });
                    let snap = |v: f32| {
                        if mods.command {
                            v
                        } else {
                            (v * 20.0).round() / 20.0
                        }
                    };
                    let new = [snap(new[0]), new[1], snap(new[2])];
                    match sel {
                        Pick::Object(id) => {
                            if let Some(ob) = cx.engine.show_mut().venue.find_mut(id) {
                                if !ob.locked {
                                    ob.transform.position = new;
                                }
                            }
                        }
                        Pick::Fixture(id) => {
                            let mut pl = rig::placement(cx.engine.show(), id);
                            pl.position = new;
                            if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                                f.place3d = Some(pl);
                            }
                        }
                    }
                } else {
                    cam.rotate(Vec2::new(delta.x, delta.y));
                }
            } else {
                cam.rotate(Vec2::new(delta.x, delta.y));
            }
        }
        if !resp.dragged() {
            self.drag = None;
            self.gizmo = None;
        }
        let cam = *self.camera.as_ref().expect("Kamera");
        if resp.clicked() && self.aim_pick {
            self.aim_pick = false;
            if let Some(p) = ptr {
                let (o, d) = cam.ray(uv(p), aspect);
                if let Some(h) = hit_y(o, d, 0.0) {
                    self.aim = (h * 100.0).round().to_array().map(|v| v / 100.0);
                    aim_selected(cx, self.selected, self.aim);
                }
            }
        } else if resp.clicked() {
            if let Some(p) = ptr {
                let (o, d) = cam.ray(uv(p), aspect);
                let hit = Self::pick(cx.engine.show(), o, d);
                match hit {
                    Some(Pick::Fixture(id)) => {
                        // Auswahl wie überall in DMXCraft (Programmer, Bühne)
                        if mods.command || mods.shift {
                            if !cx.state.selected.remove(&id) {
                                cx.state.selected.insert(id);
                            }
                        } else {
                            cx.state.selected = [id].into();
                        }
                        self.selected = hit;
                    }
                    Some(Pick::Object(_)) if building => self.selected = hit,
                    _ => {
                        self.selected = None;
                        if !building {
                            cx.state.selected.clear();
                        }
                    }
                }
            }
        }

        // ----- Bild -----
        let q = cx.state.viz_quality.params();
        let (scale, msaa) = (q.scale, q.msaa);
        let ppp = ui.ctx().pixels_per_point();
        let px = [
            (rect.width() * ppp * scale).round() as u32,
            (rect.height() * ppp * scale).round() as u32,
        ];
        let time = self.start.elapsed().as_secs_f32();
        let frame = self.frame3d(cx, &cam, aspect, time, q, building);
        let gpu = self.gpu.get_or_insert_with(|| Gpu::new(rs));
        gpu.render(rs, px, msaa, &frame);
        if let Some(tex) = gpu.texture {
            painter.image(
                tex,
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        // Angaben unten rechts: Grafikkarte, Bilder pro Sekunde
        self.fps.1 += 1;
        let el = self.fps.0.elapsed().as_secs_f32();
        if el >= 1.0 {
            self.fps = (Instant::now(), 0, self.fps.1 as f32 / el);
        }
        let info = format!(
            "{} · {:.0} fps · {}×{}",
            gpu.adapter, self.fps.2, px[0], px[1]
        );
        let galley = painter.layout_no_wrap(
            info,
            egui::FontId::proportional(theme::FS_CAPTION - 1.0),
            theme::text_dim(),
        );
        let at = rect.right_bottom() - galley.size() - vec2(8.0, 6.0);
        painter.rect_filled(
            Rect::from_min_size(at - vec2(4.0, 2.0), galley.size() + vec2(8.0, 4.0)),
            3.0,
            Color32::from_black_alpha(140),
        );
        painter.galley(at, galley, theme::text_dim());
        if let Some(g) = &gizmo {
            g.paint(&painter, self.gizmo.map(|g| g.0), gizmo_hover);
        }
        if self.aim_pick {
            painter.text(
                rect.center_top() + vec2(0.0, 12.0),
                egui::Align2::CENTER_TOP,
                t("Zielpunkt auf dem Boden anklicken"),
                theme::body(),
                theme::accent(),
            );
        }
        if building {
            let hint = t("Linke Maustaste: drehen · auf der Auswahl: verschieben · Griffe: Achse bzw. Drehung (Strg: ohne Raster) · Rechts/Mitte: verschieben · Rad: zoomen");
            painter.text(
                rect.left_bottom() + vec2(8.0, -8.0),
                egui::Align2::LEFT_BOTTOM,
                hint,
                egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                theme::text_faint(),
            );
        }
        if resp.hovered() || resp.dragged() {
            ui.ctx().set_cursor_icon(if resp.dragged() {
                egui::CursorIcon::Grabbing
            } else {
                egui::CursorIcon::Grab
            });
        }
        // laufend neu zeichnen (Szenen, Strobe)
        ui.ctx().request_repaint();
        let _ = Stroke::NONE;
    }

    /// Nächstes Objekt bzw. Gerät unter einem Strahl.
    fn pick(show: &Show, o: Vec3, d: Vec3) -> Option<Pick> {
        let mut best: Option<(f32, Pick)> = None;
        let mut take = |t: f32, p: Pick| {
            if best.is_none_or(|b| t < b.0) {
                best = Some((t, p));
            }
        };
        for f in &show.fixtures {
            let c = Vec3::from(rig::placement(show, f.id).position) + Vec3::new(0.0, -0.15, 0.0);
            if let Some(t) = ray_sphere(o, d, c, 0.3) {
                take(t * 0.98, Pick::Fixture(f.id));
            }
        }
        for (ob, m) in show.venue.flatten() {
            let Some((key_scale, _)) = shape_mesh(&ob.shape) else {
                continue;
            };
            // Flächen zum Anklicken etwas dicker
            let s = match ob.shape {
                Shape::Plane { .. } => key_scale * Mat4::from_scale(Vec3::new(1.0, 0.02, 1.0)),
                _ => key_scale,
            };
            // die Raumhülle (große Flächen) nur anklicken, wenn nichts anderes getroffen wird
            let world = Mat4::from_cols_array_2d(&m) * s;
            if let Some(t) = ray_box(o, d, world) {
                let penalty = if matches!(ob.shape, Shape::Plane { .. }) {
                    1000.0
                } else {
                    0.0
                };
                take(t + penalty, Pick::Object(ob.id));
            }
        }
        best.map(|b| b.1)
    }

    fn frame3d(
        &self,
        cx: &ViewCtx<'_>,
        cam: &OrbitCamera,
        aspect: f32,
        time: f32,
        q: QualityParams,
        building: bool,
    ) -> Frame3d {
        let show = cx.engine.show();
        let venue = &show.venue;
        let mut f = Frame3d {
            view_proj: cam.proj(aspect) * cam.view(),
            eye: cam.eye(),
            time,
            ..Default::default()
        };
        for (ob, m) in venue.flatten() {
            let Some((scale, key)) = shape_mesh(&ob.shape) else {
                continue;
            };
            let sel = building && self.selected == Some(Pick::Object(ob.id));
            let mat = ob.material;
            let color = rig::linear(mat.color);
            let inst = Instance::new(
                Mat4::from_cols_array_2d(&m) * scale,
                color,
                mat.opacity,
                mat.gloss,
                if sel { 0.12 } else { mat.emissive },
            );
            if mat.opacity < 0.99 {
                f.transparent.push((key, inst));
            } else {
                f.opaque.push((key, inst));
                f.casters.push((key, inst));
            }
        }
        let (rig, fog_boost) = rig::build(cx.engine, cx.frame, &cx.state.selected, time);
        let half = Vec3::from(venue.size) * 0.5;
        let room_min = Vec3::new(-half.x, 0.0, -half.z) - Vec3::splat(0.5);
        let room_max = Vec3::new(half.x, venue.size[1], half.z) + Vec3::splat(0.5);
        // Hindernisse für Lichtstrahlen (Traversen lassen Licht durch)
        let solids: Vec<Mat4> = venue
            .flatten()
            .into_iter()
            .filter_map(|(ob, m)| {
                let (scale, _) = shape_mesh(&ob.shape)?;
                let s = match ob.shape {
                    Shape::Truss { .. } => return None,
                    Shape::Plane { .. } => scale * Mat4::from_scale(Vec3::new(1.0, 0.002, 1.0)),
                    _ => scale,
                };
                (ob.material.opacity > 0.5).then(|| Mat4::from_cols_array_2d(&m) * s)
            })
            .collect();
        let mut lights: Vec<(f32, Light)> = Vec::new();
        for fx in &rig {
            for p in &fx.parts {
                let key = match p.prim {
                    rig::Prim::Cube => MeshKey::Cube,
                    rig::Prim::Cylinder => MeshKey::Cylinder,
                };
                f.opaque.push((
                    key,
                    Instance::new(p.matrix, p.color, 1.0, p.gloss, p.emissive),
                ));
            }
            for b in &fx.beams {
                // Strahl endet am ersten Hindernis (etwas hinter der Linse beginnen)
                let start = b.origin + b.dir * 0.05;
                let wall = ray_aabb_exit(b.origin, b.dir, room_min, room_max).unwrap_or(15.0);
                let hit = solids
                    .iter()
                    .filter_map(|m| ray_box(start, b.dir, *m))
                    .fold(wall, f32::min);
                let len = (hit + 0.05).clamp(0.3, 40.0);
                let r = b.lens + len * b.half_angle.to_radians().tan();
                let rot = glam::Quat::from_rotation_arc(Vec3::NEG_Y, b.dir);
                let m = Mat4::from_scale_rotation_translation(Vec3::new(r, len, r), rot, b.origin);
                f.beams.push(Instance::new(m, b.color, 1.0, 0.0, 0.0));
                let power = b.color.max_element();
                lights.push((
                    power,
                    Light {
                        // Reichweite bis knapp hinter das Hindernis (ersetzt Schatten grob)
                        pos: b.origin.extend(len + 0.8).into(),
                        dir: b
                            .dir
                            .extend((b.half_angle * 1.25).to_radians().cos())
                            .into(),
                        color: (b.color * 2.5)
                            .extend((b.half_angle * 0.6).to_radians().cos())
                            .into(),
                        extra: [-1.0, b.half_angle * 1.25, 0.0, 0.0],
                    },
                ));
            }
        }
        // hellste zuerst, so viele wie die Qualität erlaubt
        lights.sort_by(|a, b| b.0.total_cmp(&a.0));
        f.lights = lights.into_iter().take(q.lights).map(|l| l.1).collect();
        // Schattenkarten für die hellsten Scheinwerfer
        if q.shadows > 0 {
            for (i, l) in f.lights.iter_mut().take(q.shadows).enumerate() {
                let pos = Vec3::new(l.pos[0], l.pos[1], l.pos[2]);
                let dir = Vec3::new(l.dir[0], l.dir[1], l.dir[2]).normalize_or(Vec3::NEG_Y);
                let up = if dir.y.abs() > 0.95 { Vec3::X } else { Vec3::Y };
                let fov = (l.extra[1].to_radians() * 2.0 * 1.1).clamp(0.05, 3.0);
                let proj = Mat4::perspective_rh(fov, 1.0, 0.05, l.pos[3].max(0.2));
                f.shadow_mats
                    .push(proj * Mat4::look_at_rh(pos, pos + dir, up));
                l.extra[0] = i as f32;
            }
            f.shadow_size = q.shadow_size;
        }
        f.bloom = q.bloom;
        let env = venue.environment;
        f.env = Vec3::new(env.ambient, env.beams, env.fog.max(fog_boost * 0.9));
        f.clear = Vec3::splat(0.002 + env.ambient * 0.01);
        f
    }

    // ---------- Bauen ----------

    fn build_panel(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            ui.label(theme::heading_text(t("Raum")));
            let v = &mut cx.engine.show_mut().venue;
            ui.horizontal(|ui| {
                ui.label(t("Name"));
                ui.text_edit_singleline(&mut v.name);
            });
            ui.horizontal(|ui| {
                ui.label(t("Größe (m)"));
                for (i, c) in ["B", "H", "T"].iter().enumerate() {
                    ui.add(egui::DragValue::new(&mut v.size[i]).range(1.0..=200.0).speed(0.05).prefix(format!("{c} ")).max_decimals(2));
                }
            });
            if ui.button(t("Leeren Raum mit dieser Größe anlegen")).clicked() {
                let size = v.size;
                let name = v.name.clone();
                let env = v.environment;
                *v = Venue::room(name, size);
                v.environment = env;
                self.selected = None;
            }
            ui.add_space(theme::SP_M);
            ui.label(theme::heading_text(t("Hinzufügen")));
            ui.horizontal_wrapped(|ui| {
                let at = self.camera.map_or(Vec3::ZERO, |c| c.target);
                let items: [(&str, Shape, [f32; 3]); 6] = [
                    (t("Quader"), Shape::Box { size: [1.0, 1.0, 1.0] }, [at.x, 0.5, at.z]),
                    (t("Podest"), Shape::Box { size: [2.0, 0.4, 1.0] }, [at.x, 0.2, at.z]),
                    (t("Fläche"), Shape::Plane { size: [2.0, 2.0] }, [at.x, 0.01, at.z]),
                    (t("Zylinder"), Shape::Cylinder { radius: 0.2, height: 2.0 }, [at.x, 1.0, at.z]),
                    (t("Traverse 2 m"), Shape::Truss { length: 2.0, profile: TrussProfile::Triangle, width: 0.29 }, [at.x, 4.0, at.z]),
                    (t("Traverse 3 m"), Shape::Truss { length: 3.0, profile: TrussProfile::Square, width: 0.29 }, [at.x, 4.0, at.z]),
                ];
                for (label, shape, pos) in items {
                    if ui.button(label).clicked() {
                        let material = if matches!(shape, Shape::Truss { .. }) {
                            Material { color: [190, 190, 195], gloss: 0.6, ..Material::default() }
                        } else {
                            Material::default()
                        };
                        let o = v.object(label, shape, Transform { position: pos, ..Transform::default() }, material);
                        self.selected = Some(Pick::Object(o.id));
                        v.objects.push(o);
                    }
                }
            });
            ui.add_space(theme::SP_M);
            ui.label(theme::heading_text(t("Objekte")));
            let mut select = None;
            let mut toggle = None;
            object_tree(ui, &v.objects, 0, self.selected, &mut select, &mut toggle);
            if let Some(id) = select {
                self.selected = Some(Pick::Object(id));
            }
            if let Some(id) = toggle {
                if let Some(o) = v.find_mut(id) {
                    o.hidden = !o.hidden;
                }
            }
            if !self.missing.is_empty() {
                ui.add_space(theme::SP_M);
                ui.label(theme::heading_text(t("Ohne Gegenstück im Patch")));
                ui.label(egui::RichText::new(t("Diese Geräte aus dem Import haben im Patch kein Gerät mit gleichem Universum und gleicher Adresse.")).small().color(theme::text_faint()));
                for m in &self.missing {
                    ui.label(egui::RichText::new(m).small());
                }
                if ui.button(t("Liste ausblenden")).clicked() {
                    self.missing.clear();
                }
            }
            ui.add_space(theme::SP_M);
            match self.selected {
                Some(Pick::Object(id)) => self.object_props(ui, cx, id),
                Some(Pick::Fixture(id)) => self.fixture_props(ui, cx, id),
                None => {
                    ui.label(egui::RichText::new(t("Objekt oder Gerät anklicken, um es zu bearbeiten.")).color(theme::text_faint()));
                }
            }
        });
    }

    fn object_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: u32) {
        let v = &mut cx.engine.show_mut().venue;
        let mut remove = false;
        let mut duplicate = false;
        let Some(o) = v.find_mut(id) else {
            self.selected = None;
            return;
        };
        ui.label(theme::heading_text(t("Eigenschaften")));
        ui.horizontal(|ui| {
            ui.label(t("Name"));
            ui.text_edit_singleline(&mut o.name);
        });
        ui.checkbox(&mut o.locked, t("Gesperrt"));
        let enabled = !o.locked;
        ui.add_enabled_ui(enabled, |ui| {
            vec3_row(ui, t("Position (m)"), &mut o.transform.position, 0.01);
            vec3_row(ui, t("Drehung (°)"), &mut o.transform.rotation, 0.5);
            vec3_row(ui, t("Skalierung"), &mut o.transform.scale, 0.01);
            match &mut o.shape {
                Shape::Box { size } => vec3_row(ui, t("Maße (m)"), size, 0.01),
                Shape::Plane { size } => {
                    ui.horizontal(|ui| {
                        ui.label(t("Maße (m)"));
                        ui.add(
                            egui::DragValue::new(&mut size[0])
                                .speed(0.01)
                                .range(0.01..=500.0),
                        );
                        ui.add(
                            egui::DragValue::new(&mut size[1])
                                .speed(0.01)
                                .range(0.01..=500.0),
                        );
                    });
                }
                Shape::Cylinder { radius, height } => {
                    ui.horizontal(|ui| {
                        ui.label(t("Radius / Höhe (m)"));
                        ui.add(
                            egui::DragValue::new(radius)
                                .speed(0.005)
                                .range(0.005..=50.0),
                        );
                        ui.add(egui::DragValue::new(height).speed(0.01).range(0.01..=100.0));
                    });
                }
                Shape::Truss {
                    length, profile, ..
                } => {
                    ui.horizontal(|ui| {
                        ui.label(t("Länge (m)"));
                        ui.add(egui::DragValue::new(length).speed(0.05).range(0.25..=30.0));
                        let mut sq = *profile == TrussProfile::Square;
                        ui.checkbox(&mut sq, t("Vierpunkt"));
                        *profile = if sq {
                            TrussProfile::Square
                        } else {
                            TrussProfile::Triangle
                        };
                    });
                }
                Shape::Group => {}
            }
            if !matches!(o.shape, Shape::Group) {
                ui.horizontal(|ui| {
                    ui.label(t("Farbe"));
                    let mut c = Color32::from_rgb(
                        o.material.color[0],
                        o.material.color[1],
                        o.material.color[2],
                    );
                    if ui.color_edit_button_srgba(&mut c).changed() {
                        o.material.color = [c.r(), c.g(), c.b()];
                    }
                });
                ui.add(egui::Slider::new(&mut o.material.opacity, 0.05..=1.0).text(t("Deckkraft")));
                ui.add(egui::Slider::new(&mut o.material.gloss, 0.0..=1.0).text(t("Glanz")));
                ui.add(egui::Slider::new(&mut o.material.emissive, 0.0..=1.0).text(t("Leuchten")));
            }
        });
        ui.horizontal(|ui| {
            duplicate = ui
                .add_enabled(enabled, egui::Button::new(t("Duplizieren")))
                .clicked();
            remove = ui
                .add_enabled(enabled, egui::Button::new(t("Löschen")))
                .clicked();
        });
        if duplicate {
            if let Some(mut copy) = v.find(id).cloned() {
                renumber(v, &mut copy);
                copy.transform.position[0] += 0.5;
                self.selected = Some(Pick::Object(copy.id));
                v.objects.push(copy);
            }
        }
        if remove {
            v.remove(id);
            self.selected = None;
        }
    }

    // ---------- Dateien ----------

    fn open_dialog(&mut self, ctx: &egui::Context, kind: DialogKind) {
        if self.dialog.is_some() {
            return;
        }
        let (tx, rx) = channel();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let r = match kind {
                DialogKind::ImportEv2 => rfd::FileDialog::new()
                    .set_title(t("Easy View 2 importieren"))
                    .add_filter(t("Easy View 2 (XML)"), &["xml", "ev2"])
                    .pick_file(),
                DialogKind::LoadVenue => rfd::FileDialog::new()
                    .set_title(t("Veranstaltungsort laden"))
                    .add_filter(t("Veranstaltungsort"), &[VENUE_EXT])
                    .pick_file(),
                DialogKind::SaveVenue => rfd::FileDialog::new()
                    .set_title(t("Veranstaltungsort speichern unter"))
                    .add_filter(t("Veranstaltungsort"), &[VENUE_EXT])
                    .set_file_name(format!("Ort.{VENUE_EXT}"))
                    .save_file()
                    .map(|p| {
                        if p.extension().is_some() {
                            p
                        } else {
                            p.with_extension(VENUE_EXT)
                        }
                    }),
            };
            let _ = tx.send(r);
            ctx.request_repaint();
        });
        self.dialog = Some((kind, rx));
    }

    fn poll_dialog(&mut self, cx: &mut ViewCtx<'_>) {
        let Some((kind, rx)) = &self.dialog else {
            return;
        };
        let Ok(result) = rx.try_recv() else {
            return;
        };
        let kind = *kind;
        self.dialog = None;
        let Some(path) = result else {
            return;
        };
        let p = path.display().to_string();
        match kind {
            DialogKind::SaveVenue => {
                let v = &cx.engine.show().venue;
                match v
                    .to_json()
                    .map_err(|e| e.to_string())
                    .and_then(|j| std::fs::write(&path, j).map_err(|e| e.to_string()))
                {
                    Ok(()) => cx
                        .state
                        .info(tr!("Veranstaltungsort gespeichert: {p}", p = p)),
                    Err(e) => cx.state.error(tr!("Nicht gespeichert: {e}", e = e)),
                }
            }
            DialogKind::LoadVenue => match std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| Venue::from_json(&s).map_err(|e| e.to_string()))
            {
                Ok(v) => {
                    cx.engine.show_mut().venue = v;
                    self.selected = None;
                    cx.state.info(tr!("Veranstaltungsort geladen: {p}", p = p));
                }
                Err(e) => cx.state.error(tr!("Nicht geladen: {e}", e = e)),
            },
            DialogKind::ImportEv2 => {
                let text = match std::fs::read(&path) {
                    Ok(b) => String::from_utf8_lossy(&b).into_owned(),
                    Err(e) => {
                        cx.state.error(e.to_string());
                        return;
                    }
                };
                match venue::ev2::import(&text) {
                    Ok(imp) => {
                        let show = cx.engine.show_mut();
                        let mut venue = imp.venue;
                        venue.name = path
                            .file_stem()
                            .map_or(venue.name, |s| s.to_string_lossy().into_owned());
                        show.venue = venue;
                        // Geräte über Universum und Adresse zuordnen
                        let mut matched = 0;
                        let mut missing = Vec::new();
                        for f in &imp.fixtures {
                            match show
                                .fixtures
                                .iter_mut()
                                .find(|x| x.universe == f.universe && x.address == f.address)
                            {
                                Some(x) => {
                                    x.place3d = Some(f.placement);
                                    matched += 1;
                                }
                                None => missing.push(format!(
                                    "{} (U{}/{})",
                                    f.name,
                                    f.universe + 1,
                                    f.address
                                )),
                            }
                        }
                        self.selected = None;
                        cx.state.info(tr!(
                            "Easy View importiert: {m} Geräte zugeordnet, {n} ohne Gegenstück im Patch",
                            m = matched,
                            n = missing.len()
                        ));
                        self.missing = missing;
                    }
                    Err(e) => cx.state.error(tr!("Import fehlgeschlagen: {e}", e = e)),
                }
            }
        }
    }
}

/// Netz und Skalierung einer Form (`None` = Gruppe).
fn shape_mesh(s: &Shape) -> Option<(Mat4, MeshKey)> {
    Some(match *s {
        Shape::Group => return None,
        Shape::Box { size } => (Mat4::from_scale(Vec3::from(size)), MeshKey::Cube),
        Shape::Plane { size } => (
            Mat4::from_scale(Vec3::new(size[0], 1.0, size[1])),
            MeshKey::Plane,
        ),
        Shape::Cylinder { radius, height } => (
            Mat4::from_scale(Vec3::new(radius, height, radius)),
            MeshKey::Cylinder,
        ),
        Shape::Truss {
            length,
            profile,
            width,
        } => (
            Mat4::IDENTITY,
            MeshKey::Truss(
                (length * 1000.0).round() as u32,
                (width * 1000.0).round() as u32,
                profile == TrussProfile::Square,
            ),
        ),
    })
}

fn renumber(v: &mut Venue, o: &mut Object) {
    o.id = v.new_id();
    for c in &mut o.children {
        renumber(v, c);
    }
}

fn object_tree(
    ui: &mut egui::Ui,
    list: &[Object],
    depth: usize,
    sel: Option<Pick>,
    select: &mut Option<u32>,
    toggle: &mut Option<u32>,
) {
    for o in list {
        ui.horizontal(|ui| {
            ui.add_space(depth as f32 * 14.0);
            if ui
                .add(
                    IconButton::new(if o.hidden { Icon::EyeOff } else { Icon::Eye })
                        .size(20.0)
                        .flat(),
                )
                .clicked()
            {
                *toggle = Some(o.id);
            }
            let active = sel == Some(Pick::Object(o.id));
            let text = if o.locked {
                format!("{} ·", o.name)
            } else {
                o.name.clone()
            };
            if ui.selectable_label(active, text).clicked() {
                *select = Some(o.id);
            }
        });
        object_tree(ui, &o.children, depth + 1, sel, select, toggle);
    }
}

fn vec3_row(ui: &mut egui::Ui, label: &str, v: &mut [f32; 3], speed: f64) {
    ui.horizontal(|ui| {
        ui.label(label);
        for c in v.iter_mut() {
            ui.add(egui::DragValue::new(c).speed(speed).max_decimals(3));
        }
    });
}

/// Lage eines Objekts oder Geräts setzen (gesperrte Objekte bleiben).
fn apply_placement(cx: &mut ViewCtx<'_>, sel: Pick, p: Placement) {
    match sel {
        Pick::Object(id) => {
            if let Some(ob) = cx.engine.show_mut().venue.find_mut(id) {
                if !ob.locked {
                    ob.transform.position = p.position;
                    ob.transform.rotation = p.rotation;
                }
            }
        }
        Pick::Fixture(id) => {
            if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                f.place3d = Some(p);
            }
        }
    }
}

/// Geräte, auf die eine Aktion wirkt: alle ausgewählten, mindestens das angeklickte.
fn targets(cx: &ViewCtx<'_>, sel: Option<Pick>) -> Vec<FixtureId> {
    let mut ids: Vec<FixtureId> = cx.state.selected.iter().copied().collect();
    if let Some(Pick::Fixture(id)) = sel {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    ids
}

fn aim_selected(cx: &mut ViewCtx<'_>, sel: Option<Pick>, target: [f32; 3]) {
    for id in targets(cx, sel) {
        let mut p = rig::placement(cx.engine.show(), id);
        p.aim_at(target);
        apply_placement(cx, Pick::Fixture(id), p);
    }
}

impl VizView {
    fn fixture_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: FixtureId) {
        let show = cx.engine.show();
        let Some(f) = show.fixture(id) else {
            return;
        };
        ui.label(theme::heading_text(&f.name));
        ui.label(
            egui::RichText::new(format!("U{} · {}", f.universe + 1, f.address))
                .color(theme::text_dim()),
        );
        let auto = f.place3d.is_none();
        let mut p: Placement = rig::placement(show, id);
        let before = p;
        vec3_row(ui, t("Position (m)"), &mut p.position, 0.01);
        vec3_row(ui, t("Drehung (°)"), &mut p.rotation, 0.5);
        if p != before {
            apply_placement(cx, Pick::Fixture(id), p);
        }
        let n = targets(cx, self.selected).len();
        ui.add_space(theme::SP_S);
        let title = if n > 1 {
            tr!("Ausrichten ({n} Geräte)", n = n)
        } else {
            t("Ausrichten").to_string()
        };
        ui.label(theme::heading_text(&title));
        vec3_row(ui, t("Zielpunkt (m)"), &mut self.aim, 0.05);
        ui.horizontal_wrapped(|ui| {
            if ui.button(t("Auf Zielpunkt richten")).clicked() {
                aim_selected(cx, self.selected, self.aim);
            }
            if ui
                .selectable_label(self.aim_pick, t("Zielpunkt im Bild wählen"))
                .clicked()
            {
                self.aim_pick = !self.aim_pick;
            }
            if ui.button(t("Senkrecht nach unten")).clicked() {
                for fid in targets(cx, self.selected) {
                    let mut p = rig::placement(cx.engine.show(), fid);
                    p.rotation = [0.0; 3];
                    apply_placement(cx, Pick::Fixture(fid), p);
                }
            }
        });
        let has_truss = cx
            .engine
            .show()
            .venue
            .flatten()
            .iter()
            .any(|(o, _)| matches!(o.shape, Shape::Truss { .. }));
        if ui
            .add_enabled(
                has_truss,
                egui::Button::new(t("An nächste Traverse hängen")),
            )
            .clicked()
        {
            for fid in targets(cx, self.selected) {
                let mut p = rig::placement(cx.engine.show(), fid);
                if let Some(h) = cx.engine.show().venue.nearest_truss(p.position) {
                    p.position = h.point;
                    apply_placement(cx, Pick::Fixture(fid), p);
                }
            }
        }
        ui.add_space(theme::SP_S);
        if ui
            .add_enabled(
                !auto,
                egui::Button::new(t("Automatisch (aus dem Bühnenplan)")),
            )
            .clicked()
        {
            if let Some(fm) = cx.engine.show_mut().fixture_mut(id) {
                fm.place3d = None;
            }
        }
    }
}

// ---------- Strahl-Tests ----------

fn hit_y(o: Vec3, d: Vec3, y: f32) -> Option<Vec3> {
    if d.y.abs() < 1e-5 {
        return None;
    }
    let t = (y - o.y) / d.y;
    (t > 0.0).then(|| o + d * t)
}

fn ray_sphere(o: Vec3, d: Vec3, c: Vec3, r: f32) -> Option<f32> {
    let oc = o - c;
    let b = oc.dot(d);
    let q = oc.length_squared() - r * r;
    let disc = b * b - q;
    if disc < 0.0 {
        return None;
    }
    let t = -b - disc.sqrt();
    (t > 0.0).then_some(t)
}

/// Strahl gegen den Einheitswürfel nach `m` (Rückgabe: Abstand in Welt-Einheiten).
fn ray_box(o: Vec3, d: Vec3, m: Mat4) -> Option<f32> {
    let inv = m.inverse();
    let lo = inv.transform_point3(o);
    let ld = inv.transform_vector3(d);
    let (t0, _) = slab(lo, ld, Vec3::splat(-0.5), Vec3::splat(0.5))?;
    let p = m.transform_point3(lo + ld * t0.max(0.0));
    Some(p.distance(o))
}

fn slab(o: Vec3, d: Vec3, min: Vec3, max: Vec3) -> Option<(f32, f32)> {
    let mut t0 = f32::NEG_INFINITY;
    let mut t1 = f32::INFINITY;
    for i in 0..3 {
        if d[i].abs() < 1e-8 {
            if o[i] < min[i] || o[i] > max[i] {
                return None;
            }
            continue;
        }
        let a = (min[i] - o[i]) / d[i];
        let b = (max[i] - o[i]) / d[i];
        t0 = t0.max(a.min(b));
        t1 = t1.min(a.max(b));
    }
    (t1 >= t0.max(0.0)).then_some((t0, t1))
}

/// Wie weit ein Strahl im Raum läuft, bis er ihn verlässt.
fn ray_aabb_exit(o: Vec3, d: Vec3, min: Vec3, max: Vec3) -> Option<f32> {
    slab(o, d, min, max).map(|(_, t1)| t1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_tests() {
        assert!(
            ray_sphere(Vec3::ZERO, Vec3::X, Vec3::new(5.0, 0.0, 0.0), 1.0)
                .is_some_and(|t| (t - 4.0).abs() < 1e-4)
        );
        let m = Mat4::from_scale_rotation_translation(
            Vec3::splat(2.0),
            glam::Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 10.0),
        );
        assert!(ray_box(Vec3::ZERO, Vec3::Z, m).is_some_and(|t| (t - 9.0).abs() < 1e-3));
        assert!(ray_box(Vec3::ZERO, Vec3::X, m).is_none());
        let exit = ray_aabb_exit(
            Vec3::new(0.0, 4.0, 0.0),
            Vec3::NEG_Y,
            Vec3::new(-5.0, 0.0, -5.0),
            Vec3::new(5.0, 5.0, 5.0),
        );
        assert!(exit.is_some_and(|t| (t - 4.0).abs() < 1e-4));
    }
}
