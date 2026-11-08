//! 3D-Visualisierer: Veranstaltungsort bauen und die Geräte live sehen.
//!
//! Gezeichnet wird auf der Grafikkarte (wgpu: Vulkan, Metal oder DirectX 12)
//! in eine Textur, die egui anzeigt – in einem eigenen
//! Fenster (z. B. auf einem zweiten Bildschirm).

pub mod camera;
pub mod gltf_import;
pub mod library;
pub mod mesh;
mod outline;
mod panels;
pub mod render;
pub mod rig;
pub mod textures;

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time::Instant;

use dmxcraft_core::venue::{
    self, Material, Object, Placement, Shape, Transform, TrussProfile, Venue,
};
use dmxcraft_core::{FixtureId, FixtureType, Show};
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
    ImportModel,
    LoadVenue,
    SaveVenue,
    LoadTexture,
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
    missing: Vec<Missing>,
    /// Griff, an dem gerade gezogen wird (Achse 0–2, Ring 3–5),
    /// Ausgangslage und Startpunkt des Zeigers.
    gizmo: Option<GizmoDrag>,
    /// Ausgewählte Objekte (Mehrfachauswahl; `selected` ist die Hauptauswahl).
    sel_objects: std::collections::BTreeSet<u32>,
    /// Lagen aller ausgewählten beim Beginn des Ziehens auf dem Boden.
    drag_frames: Vec<(Pick, SelFrame)>,
    /// Reiter der linken Leiste, Suchtext und Anker der Geräteliste.
    outline: panels::OutlineTab,
    fixture_filter: String,
    fixture_sort: outline::FixtureSort,
    /// Objekt oder Geräteordner, der gerade umbenannt wird.
    renaming: Option<outline::Renaming>,
    rename_focus: bool,
    /// Objekt-Bibliothek (beim ersten Öffnen geladen).
    library: Option<(Vec<library::Prefab>, Vec<library::UserPrefab>)>,
    fixture_anchor: Option<FixtureId>,
    /// Objekt im Baum aufklappen und zeigen / Gerät in der Liste zeigen.
    reveal: Option<u32>,
    reveal_fixture: Option<FixtureId>,
    /// Zielpunkt zum Ausrichten von Geräten.
    aim: [f32; 3],
    /// Nächster Klick ins Bild setzt den Zielpunkt.
    aim_pick: bool,
    /// Modell, das gerade im Hintergrund geladen wird.
    model_job: Option<Receiver<Result<venue::model::Model, String>>>,
}

/// Gerät aus einem Import ohne Gegenstück im Patch, mit vorgeschlagenem Typ.
struct Missing {
    fixture: venue::ev2::Ev2Fixture,
    /// Typ aus der Bibliothek (bzw. generisch) und ob er sicher passt.
    proposal: FixtureType,
    exact: bool,
    create: bool,
}

/// Gerätetyp zu einem Easy-View-Bibliotheksnamen: Bibliothek, sonst generisch.
fn propose_type(library_name: &str) -> (FixtureType, bool) {
    if let Some((_, m)) = dmxcraft_ofl::Library::embedded().best_match(library_name) {
        return (m.clone(), true);
    }
    let n = library_name.to_lowercase();
    let key = if ["moving", "wash", "spot", "beam", "head"]
        .iter()
        .any(|k| n.contains(k))
    {
        "generic/moving-head-spot-11ch"
    } else if n.contains("rgbw") {
        "generic/rgbw-par-6ch"
    } else if n.contains("rgb") || n.contains("bar") || n.contains("par") || n.contains("led") {
        "generic/drgb-4ch"
    } else {
        "generic/dimmer-1ch"
    };
    let types = dmxcraft_core::library::builtin_fixture_types();
    let t = types
        .iter()
        .find(|t| t.key == key)
        .or(types.first())
        .cloned()
        .expect("eingebaute Gerätetypen");
    (t, false)
}

/// Achsenfarben der Griffe (x rot, y grün, z blau).
const AXIS_COLORS: [Color32; 3] = [
    Color32::from_rgb(230, 80, 80),
    Color32::from_rgb(90, 200, 90),
    Color32::from_rgb(80, 140, 240),
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

/// Lage der Auswahl: eigene Werte, Weltposition und Elternmatrix.
#[derive(Clone, Copy, Debug)]
struct SelFrame {
    local: Placement,
    world: Vec3,
    parent: Mat4,
}

fn sel_frame(show: &Show, sel: Pick) -> Option<SelFrame> {
    match sel {
        Pick::Object(id) => {
            let o = show.venue.find(id)?;
            if o.locked {
                return None;
            }
            let parent = Mat4::from_cols_array_2d(&show.venue.parent_matrix(id));
            let local = Placement {
                position: o.transform.position,
                rotation: o.transform.rotation,
            };
            Some(SelFrame {
                local,
                world: parent.transform_point3(Vec3::from(local.position)),
                parent,
            })
        }
        Pick::Fixture(id) => {
            let local = rig::placement(show, id);
            Some(SelFrame {
                local,
                world: Vec3::from(local.position),
                parent: Mat4::IDENTITY,
            })
        }
    }
}

/// Ziehen an einem Griff: welcher, Lage der Hauptauswahl und aller
/// ausgewählten beim Start, Startpunkt des Zeigers.
struct GizmoDrag {
    handle: usize,
    primary: SelFrame,
    frames: Vec<(Pick, SelFrame)>,
    start: egui::Pos2,
}

/// Alle um denselben Weg (Welt) verschieben.
fn move_all(cx: &mut ViewCtx<'_>, frames: &[(Pick, SelFrame)], delta: Vec3) {
    for (pick, f) in frames {
        let mut np = f.local;
        np.position = f
            .parent
            .inverse()
            .transform_point3(f.world + delta)
            .to_array();
        apply_placement(cx, *pick, np);
    }
}

/// Alle an Ort und Stelle um dieselbe Weltachse drehen.
fn rotate_all(cx: &mut ViewCtx<'_>, frames: &[(Pick, SelFrame)], axis: Vec3, deg: f32) {
    for (pick, f) in frames {
        let local_axis = (glam::Mat3::from_mat4(f.parent).inverse() * axis).normalize_or(axis);
        let q =
            glam::Quat::from_axis_angle(local_axis, deg.to_radians()) * quat_deg(f.local.rotation);
        let mut np = f.local;
        np.rotation = deg_quat(q);
        apply_placement(cx, *pick, np);
    }
}

/// Euler-Winkel (Grad; X, dann Y, dann Z) als Quaternion.
pub fn quat_deg(r: [f32; 3]) -> glam::Quat {
    let [x, y, z] = r.map(f32::to_radians);
    glam::Quat::from_euler(glam::EulerRot::ZYX, z, y, x)
}

/// Quaternion als Euler-Winkel (Grad, auf 0,01° gerundet).
pub fn deg_quat(q: glam::Quat) -> [f32; 3] {
    let (z, y, x) = q.normalize().to_euler(glam::EulerRot::ZYX);
    [x, y, z].map(|a| {
        let d = (a.to_degrees() * 100.0).round() / 100.0;
        if d == -0.0 {
            0.0
        } else {
            d
        }
    })
}

/// Griffe um `pos`: drei Achsen (Anfang, Ende) und drei Drehringe.
struct Handles {
    center: egui::Pos2,
    axes: [Option<(egui::Pos2, egui::Pos2)>; 3],
    rings: [Vec<egui::Pos2>; 3],
    /// Weltlänge einer Achse
    len: f32,
}

fn handles(vp: Mat4, eye: Vec3, rect: Rect, pos: Vec3) -> Option<Handles> {
    let center = project(vp, rect, pos)?;
    let len = (pos.distance(eye) * 0.12).clamp(0.15, 3.0);
    let axes =
        [Vec3::X, Vec3::Y, Vec3::Z].map(|a| Some((center, project(vp, rect, pos + a * len)?)));
    let ring = |u: Vec3, v: Vec3| -> Vec<egui::Pos2> {
        (0..=64)
            .filter_map(|k| {
                let a = k as f32 / 64.0 * std::f32::consts::TAU;
                project(vp, rect, pos + (u * a.cos() + v * a.sin()) * len * 0.75)
            })
            .collect()
    };
    Some(Handles {
        center,
        axes,
        rings: [
            ring(Vec3::Y, Vec3::Z),
            ring(Vec3::X, Vec3::Z),
            ring(Vec3::X, Vec3::Y),
        ],
        len,
    })
}

fn seg_dist(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.length_sq().max(1e-3)).clamp(0.0, 1.0);
    (a + ab * t).distance(p)
}

impl Handles {
    /// Griff unter dem Zeiger: 0–2 Achsen, 3–5 Ringe (Achsen haben Vorrang).
    fn hit(&self, p: egui::Pos2) -> Option<usize> {
        let near = |d: f32, max: f32| d < max;
        let axis = self
            .axes
            .iter()
            .enumerate()
            .filter_map(|(i, a)| a.map(|(s, e)| (i, seg_dist(p, s, e))))
            .filter(|(_, d)| near(*d, 7.0))
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((i, _)) = axis {
            return Some(i);
        }
        self.rings
            .iter()
            .enumerate()
            .filter_map(|(i, r)| {
                r.windows(2)
                    .map(|w| seg_dist(p, w[0], w[1]))
                    .min_by(f32::total_cmp)
                    .map(|d| (i + 3, d))
            })
            .filter(|(_, d)| near(*d, 6.0))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .map(|(i, _)| i)
    }

    fn paint(&self, painter: &egui::Painter, active: Option<usize>, hover: Option<usize>) {
        let hot = |i: usize| active == Some(i) || hover == Some(i);
        for (k, r) in self.rings.iter().enumerate() {
            let i = k + 3;
            let c = if hot(i) {
                AXIS_COLORS[k]
            } else {
                AXIS_COLORS[k].gamma_multiply(0.6)
            };
            painter.add(egui::Shape::line(
                r.clone(),
                egui::Stroke::new(if hot(i) { 3.5 } else { 1.8 }, c),
            ));
        }
        for (i, a) in self.axes.iter().enumerate() {
            if let Some((s, e)) = a {
                let w = if hot(i) { 4.0 } else { 2.5 };
                painter.line_segment([*s, *e], egui::Stroke::new(w, AXIS_COLORS[i]));
                painter.circle_filled(*e, w + 2.5, AXIS_COLORS[i]);
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
            sel_objects: Default::default(),
            drag_frames: Vec::new(),
            aim: [0.0; 3],
            aim_pick: false,
            model_job: None,
            outline: Default::default(),
            fixture_filter: String::new(),
            fixture_sort: Default::default(),
            renaming: None,
            rename_focus: false,
            library: None,
            fixture_anchor: None,
            reveal: None,
            reveal_fixture: None,
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
        self.poll_model(cx);
        let locked = cx.state.locked;
        let size = cx.engine.show().venue.size;
        if self.camera.is_none() || self.room != size {
            self.camera = Some(OrbitCamera::for_room(size));
            self.room = size;
        }
        self.toolbar(ui, cx, locked, window);
        ui.add_space(theme::SP_XS);
        if self.mode == Mode::Build && !locked {
            let frame = egui::Frame::new()
                .fill(theme::surface())
                .inner_margin(egui::Margin::same(6));
            egui::Panel::left("viz_outline")
                .resizable(true)
                .default_size(270.0)
                .min_size(220.0)
                .frame(frame)
                .show(ui, |ui| self.outline_panel(ui, cx));
            egui::Panel::right("viz_props")
                .resizable(true)
                .default_size(320.0)
                .min_size(260.0)
                .frame(frame)
                .show(ui, |ui| self.props_panel(ui, cx));
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
            if self.mode == Mode::Build && !locked {
                if ui
                    .add_enabled(cx.state.can_undo, IconButton::new(Icon::Undo).size(26.0))
                    .on_hover_text(t("Rückgängig (Strg+Z)"))
                    .clicked()
                {
                    cx.state.history_request = Some(true);
                }
                if ui
                    .add_enabled(cx.state.can_redo, IconButton::new(Icon::Redo).size(26.0))
                    .on_hover_text(t("Wiederholen (Strg+Y)"))
                    .clicked()
                {
                    cx.state.history_request = Some(false);
                }
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
        // Griffe der Auswahl (Achsen verschieben, Ringe drehen um x/y/z des Raums)
        let vp = cam.proj(aspect) * cam.view();
        let sel_place = if building {
            self.selected.and_then(|s| sel_frame(cx.engine.show(), s))
        } else {
            None
        };
        let gizmo = sel_place.and_then(|f| handles(vp, cam.eye(), rect, f.world));
        let gizmo_hover = gizmo.as_ref().zip(ptr).and_then(|(g, p)| g.hit(p));
        if resp.drag_started_by(egui::PointerButton::Primary) {
            // dort prüfen, wo die Taste gedrückt wurde (bei wenig Bildern pro Sekunde
            // ist der Zeiger beim Erkennen des Ziehens schon weiter)
            let origin = ui.input(|i| i.pointer.press_origin());
            let hit = gizmo
                .as_ref()
                .zip(origin)
                .and_then(|(g, o)| g.hit(o).map(|i| (i, o)));
            if let (Some((i, o)), Some(f)) = (hit, sel_place) {
                self.gizmo = Some(GizmoDrag {
                    handle: i,
                    primary: f,
                    frames: Self::group_frames(self.selected, &self.sel_objects, cx),
                    start: o,
                });
            }
        }
        if let (Some(gd), Some(g), Some(p)) = (self.gizmo.as_ref(), gizmo.as_ref(), ptr) {
            let f0 = gd.primary;
            if gd.handle < 3 {
                if let Some((a, b)) = g.axes[gd.handle] {
                    let s = b - a;
                    let along = (p - gd.start).dot(s) / s.length_sq().max(1.0) * g.len;
                    let mut w = f0.world;
                    w[gd.handle] += along;
                    if !mods.command {
                        w[gd.handle] = (w[gd.handle] * 20.0).round() / 20.0;
                    }
                    move_all(cx, &gd.frames, w - f0.world);
                }
            } else {
                let axis = [Vec3::X, Vec3::Y, Vec3::Z][gd.handle - 3];
                let ang = |q: egui::Pos2| (q.y - g.center.y).atan2(q.x - g.center.x);
                let mut d = ang(p) - ang(gd.start);
                // Bildschirm-y zeigt nach unten: zeigt die Achse zum Betrachter, dreht
                // ein Ziehen im Uhrzeigersinn negativ
                if axis.dot(cam.eye() - f0.world) > 0.0 {
                    d = -d;
                }
                let mut deg = d.to_degrees();
                if !mods.command {
                    deg = (deg / 5.0).round() * 5.0;
                }
                rotate_all(cx, &gd.frames, axis, deg);
            }
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
                    Pick::Object(id) => show
                        .venue
                        .find(id)
                        .filter(|o| !o.locked)
                        .map(|o| o.transform.position),
                    Pick::Fixture(id) => Some(rig::placement(show, id).position),
                };
                if resp.drag_started() {
                    self.drag_frames = Self::group_frames(self.selected, &self.sel_objects, cx);
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
                    // Verschiebung der Hauptauswahl auf alle ausgewählten übertragen
                    // (in Weltkoordinaten, auch in gedrehten Gruppen)
                    let parent = self
                        .drag_frames
                        .iter()
                        .find(|f| f.0 == sel)
                        .map_or(Mat4::IDENTITY, |f| f.1.parent);
                    let dw = parent.transform_vector3(Vec3::from(new) - Vec3::from(p0));
                    let frames = std::mem::take(&mut self.drag_frames);
                    move_all(cx, &frames, dw);
                    self.drag_frames = frames;
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
                            self.sel_objects.clear();
                        }
                        self.selected = hit;
                        self.reveal_fixture = Some(id);
                    }
                    Some(Pick::Object(id)) if building => {
                        self.select_object(cx, id, mods.command || mods.shift);
                        self.reveal = Some(id);
                    }
                    _ => {
                        self.selected = None;
                        self.sel_objects.clear();
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
        for (key, model) in &cx.engine.show().venue.models {
            for (i, part) in model.parts.iter().enumerate() {
                let k = MeshKey::model(key, i);
                if !gpu.has_mesh(k) {
                    let m = mesh::Mesh {
                        vertices: part
                            .vertices()
                            .into_iter()
                            .zip(
                                part.uvs()
                                    .into_iter()
                                    .flatten()
                                    .map(Some)
                                    .chain(std::iter::repeat(None)),
                            )
                            .map(|(v, uv)| mesh::Vertex {
                                pos: [v[0], v[1], v[2]],
                                normal: [v[3], v[4], v[5]],
                                uv: uv.unwrap_or([0.0; 2]),
                            })
                            .collect(),
                        indices: part.indices(),
                    };
                    gpu.add_mesh(&rs.device, k, &m);
                }
            }
            for (i, img) in model.images.iter().enumerate() {
                let id = model_tex_id(key, i);
                if !gpu.has_texture(id) {
                    gpu.add_texture(rs, id, decode_texture(&img.data.0).as_ref());
                }
            }
        }
        for (id, img) in &cx.engine.show().venue.textures {
            if !gpu.has_texture(*id) {
                gpu.add_texture(rs, *id, decode_texture(&img.data.0).as_ref());
            }
        }
        gpu.render(rs, px, msaa, &frame);
        if let Some(tex) = gpu.texture {
            painter.image(
                tex,
                rect,
                Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        // Angaben oben rechts: Grafikkarte, Bilder pro Sekunde
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
        let at = rect.right_top() + vec2(-galley.size().x - 8.0, 6.0);
        painter.rect_filled(
            Rect::from_min_size(at - vec2(4.0, 2.0), galley.size() + vec2(8.0, 4.0)),
            3.0,
            Color32::from_black_alpha(140),
        );
        painter.galley(at, galley, theme::text_dim());
        if let Some(g) = &gizmo {
            g.paint(&painter, self.gizmo.as_ref().map(|g| g.handle), gizmo_hover);
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
            let galley = painter.layout(
                hint.to_string(),
                egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                theme::text_faint(),
                rect.width() - 16.0,
            );
            painter.galley(
                rect.left_bottom() + vec2(8.0, -8.0 - galley.size().y),
                galley,
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
            if let Shape::Model { model } = &ob.shape {
                if let Some(md) = show.venue.models.get(model) {
                    let s = Vec3::from(md.size).max(Vec3::splat(0.02));
                    let world = Mat4::from_cols_array_2d(&m)
                        * Mat4::from_translation(Vec3::new(0.0, s.y * 0.5, 0.0))
                        * Mat4::from_scale(s);
                    if let Some(t) = ray_box(o, d, world) {
                        take(t, Pick::Object(ob.id));
                    }
                }
                continue;
            }
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
            if let Shape::Model { model } = &ob.shape {
                let Some(md) = venue.models.get(model) else {
                    continue;
                };
                let sel = building
                    && (self.sel_objects.contains(&ob.id)
                        || self.selected == Some(Pick::Object(ob.id)));
                let mat = ob.material;
                for (i, part) in md.parts.iter().enumerate() {
                    let mut inst = Instance::new(
                        Mat4::from_cols_array_2d(&m),
                        rig::linear(part.color),
                        mat.opacity,
                        mat.gloss,
                        if sel { 0.12 } else { mat.emissive },
                    );
                    let tex = match part.texture {
                        Some(img) if part.uv.is_some() => {
                            inst.params[3] = 2.0;
                            model_tex_id(model, img)
                        }
                        _ => 0,
                    };
                    let key = MeshKey::model(model, i);
                    if mat.opacity < 0.99 {
                        f.transparent.push((key, tex, inst));
                    } else {
                        f.opaque.push((key, tex, inst));
                        f.casters.push((key, tex, inst));
                    }
                }
                continue;
            }
            let Some((scale, key)) = shape_mesh(&ob.shape) else {
                continue;
            };
            let sel = building
                && (self.sel_objects.contains(&ob.id)
                    || self.selected == Some(Pick::Object(ob.id)));
            let mat = ob.material;
            let color = rig::linear(mat.color);
            let mut inst = Instance::new(
                Mat4::from_cols_array_2d(&m) * scale,
                color,
                mat.opacity,
                mat.gloss,
                if sel { 0.12 } else { mat.emissive },
            );
            // Bild aufprojiziert (Traversen ohne)
            let tex = if mat.texture != 0
                && venue.textures.contains_key(&mat.texture)
                && !matches!(ob.shape, Shape::Truss { .. })
            {
                inst.params[2] = mat.tile.max(0.01);
                inst.params[3] = 1.0;
                mat.texture
            } else {
                0
            };
            if mat.opacity < 0.99 {
                f.transparent.push((key, tex, inst));
            } else {
                f.opaque.push((key, tex, inst));
                f.casters.push((key, tex, inst));
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
                    0,
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
                // Fokus: Schärfe dort, wo der Strahl auftrifft
                let sharp = b.sharpness(len);
                let gobo_sharp = if b.focus.is_some() { sharp } else { 1.0 };
                let mut inst = Instance::new(m, b.color, 1.0, 0.0, 0.0);
                inst.params[2] = b.gobo;
                inst.params[3] = gobo_sharp;
                f.beams.push(inst);
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
                            .extend((b.half_angle * (0.3 + 0.65 * sharp)).to_radians().cos())
                            .into(),
                        extra: [-1.0, b.half_angle * 1.25, b.gobo, gobo_sharp],
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
                DialogKind::ImportModel => rfd::FileDialog::new()
                    .set_title(t("3D-Modell importieren"))
                    .add_filter(t("glTF-Modell"), &["glb", "gltf"])
                    .pick_file(),
                DialogKind::LoadTexture => rfd::FileDialog::new()
                    .set_title(t("Bild laden"))
                    .add_filter(t("Bild (PNG/JPEG)"), &["png", "jpg", "jpeg"])
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

    /// Fertig geladenes Modell in den Ort setzen (an den Blickpunkt).
    fn poll_model(&mut self, cx: &mut ViewCtx<'_>) {
        let Some(rx) = &self.model_job else {
            return;
        };
        let Ok(result) = rx.try_recv() else {
            return;
        };
        self.model_job = None;
        match result {
            Ok(model) => {
                let at = self.camera.map_or(Vec3::ZERO, |c| c.target);
                let v = &mut cx.engine.show_mut().venue;
                let key = model.content_key();
                let name = model
                    .name
                    .rsplit_once('.')
                    .map_or(model.name.clone(), |(n, _)| n.to_string());
                let tris = model.triangles();
                v.models.entry(key.clone()).or_insert(model);
                let o = v.object(
                    &name,
                    Shape::Model { model: key },
                    Transform {
                        position: [at.x, 0.0, at.z],
                        ..Transform::default()
                    },
                    Material::default(),
                );
                self.selected = Some(Pick::Object(o.id));
                v.objects.push(o);
                cx.state.info(tr!(
                    "Modell importiert: {n} ({k} Dreiecke)",
                    n = name,
                    k = tris
                ));
            }
            Err(e) => cx.state.error(tr!("Import fehlgeschlagen: {e}", e = e)),
        }
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
            DialogKind::LoadTexture => match textures::load_file(&path) {
                Ok(tex) => {
                    let v = &mut cx.engine.show_mut().venue;
                    let id = v.add_texture(tex);
                    let ids: Vec<u32> = self
                        .sel_objects
                        .iter()
                        .copied()
                        .chain(match self.selected {
                            Some(Pick::Object(o)) => Some(o),
                            _ => None,
                        })
                        .collect();
                    panels::apply_texture(v, ids, id, None);
                }
                Err(e) => cx.state.error(tr!("Bild nicht geladen: {e}", e = e)),
            },
            DialogKind::ImportModel => {
                let (tx, rx) = channel();
                std::thread::spawn(move || {
                    let _ = tx.send(gltf_import::load(&path));
                });
                self.model_job = Some(rx);
            }
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
                                None => {
                                    let (proposal, exact) = propose_type(&f.library_name);
                                    missing.push(Missing {
                                        fixture: f.clone(),
                                        proposal,
                                        exact,
                                        create: true,
                                    });
                                }
                            }
                        }
                        self.selected = None;
                        cx.state.info(tr!(
                            "Easy View importiert: {m} Geräte zugeordnet, {n} ohne Gegenstück im Patch",
                            m = matched,
                            n = missing.len()
                        ));
                        if !missing.is_empty() {
                            self.mode = Mode::Build;
                        }
                        self.missing = missing;
                    }
                    Err(e) => cx.state.error(tr!("Import fehlgeschlagen: {e}", e = e)),
                }
            }
        }
    }
}

/// Bild-ID eines Modellbilds (oberes Bit gesetzt, getrennt von den Ort-Bildern).
fn model_tex_id(key: &str, index: usize) -> u32 {
    let h = key
        .bytes()
        .chain((index as u32).to_le_bytes())
        .fold(0x811c_9dc5u32, |h, b| {
            (h ^ u32::from(b)).wrapping_mul(0x0100_0193)
        });
    0x8000_0000 | (h & 0x7fff_ffff)
}

/// Bild lesen und auf höchstens 2048 Pixel Kantenlänge verkleinern.
pub fn decode_texture(bytes: &[u8]) -> Option<image::RgbaImage> {
    let img = image::load_from_memory(bytes).ok()?;
    let img = if img.width().max(img.height()) > 2048 {
        img.resize(2048, 2048, image::imageops::FilterType::Triangle)
    } else {
        img
    };
    Some(img.to_rgba8())
}

/// Netz und Skalierung einer Form (`None` = Gruppe).
fn shape_mesh(s: &Shape) -> Option<(Mat4, MeshKey)> {
    Some(match *s {
        Shape::Group | Shape::Model { .. } => return None,
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

fn vec3_row(ui: &mut egui::Ui, label: &str, v: &mut [f32; 3], speed: f64) {
    ui.horizontal(|ui| {
        ui.label(label);
        for c in v.iter_mut() {
            if *c == 0.0 {
                *c = 0.0; // kein „-0.0“ anzeigen
            }
            ui.add(egui::DragValue::new(c).speed(speed).max_decimals(3));
        }
    });
}

impl VizView {
    /// Alle ausgewählten (Objekte und Geräte) mit ihrer Lage; Objekte, deren
    /// Gruppe mit ausgewählt ist, bewegen sich mit der Gruppe.
    fn group_frames(
        selected: Option<Pick>,
        sel_objects: &std::collections::BTreeSet<u32>,
        cx: &ViewCtx<'_>,
    ) -> Vec<(Pick, SelFrame)> {
        let show = cx.engine.show();
        let mut picks: Vec<Pick> = sel_objects.iter().map(|id| Pick::Object(*id)).collect();
        picks.extend(cx.state.selected.iter().map(|id| Pick::Fixture(*id)));
        if let Some(p) = selected {
            if !picks.contains(&p) {
                picks.push(p);
            }
        }
        let objs: std::collections::BTreeSet<u32> = picks
            .iter()
            .filter_map(|p| match p {
                Pick::Object(id) => Some(*id),
                Pick::Fixture(_) => None,
            })
            .collect();
        picks
            .into_iter()
            .filter(|p| match p {
                Pick::Object(id) => !show.venue.ancestors(*id).iter().any(|a| objs.contains(a)),
                Pick::Fixture(_) => true,
            })
            .filter_map(|p| sel_frame(show, p).map(|f| (p, f)))
            .collect()
    }

    /// Objekt auswählen (`add`: zur Auswahl hinzufügen bzw. wieder entfernen).
    fn select_object(&mut self, cx: &mut ViewCtx<'_>, id: u32, add: bool) {
        self.select_object_in(&mut cx.state.selected, id, add);
    }

    fn select_object_in(
        &mut self,
        fixtures: &mut std::collections::BTreeSet<FixtureId>,
        id: u32,
        add: bool,
    ) {
        if add {
            if !self.sel_objects.remove(&id) {
                self.sel_objects.insert(id);
                self.selected = Some(Pick::Object(id));
            } else if self.selected == Some(Pick::Object(id)) {
                self.selected = self.sel_objects.iter().next().map(|i| Pick::Object(*i));
            }
        } else {
            self.sel_objects = [id].into();
            self.selected = Some(Pick::Object(id));
            fixtures.clear();
        }
    }
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
    fn euler_matches_core_rotation() {
        for r in [
            [0.0, 0.0, 0.0],
            [30.0, 0.0, 0.0],
            [10.0, 20.0, 30.0],
            [-45.0, 60.0, 120.0],
        ] {
            let m = glam::Mat3::from_cols_array_2d(&venue::rotation(r));
            let q = glam::Mat3::from_quat(quat_deg(r));
            assert!(m.abs_diff_eq(q, 1e-5), "{r:?}");
            let back = deg_quat(quat_deg(r));
            assert!(
                glam::Mat3::from_quat(quat_deg(back)).abs_diff_eq(m, 1e-3),
                "{r:?} → {back:?}"
            );
        }
    }

    #[test]
    fn proposals_for_easy_view_names() {
        let (t, exact) = propose_type("Stairville/LED PAR 56.ssl2");
        assert!(exact && t.manufacturer.contains("Stairville"));
        let (t, exact) = propose_type("_imported/Unbekannt Sky Wash 25ch.ssl2");
        assert!(!exact);
        assert_eq!(t.key, "generic/moving-head-spot-11ch");
    }

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
