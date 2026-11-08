//! Bühnenplan-Zeichenfläche.
//!
//! * [`CanvasMode::Select`] (STEUERN): Klicken/Aufziehen wählt Geräte aus.
//! * [`CanvasMode::Edit`] (EINRICHTEN → Bühnenplan): Ziehen verschiebt Geräte,
//!   Aufziehen auf freier Fläche wählt aus.
//!
//! Treffer werden am Druckpunkt ermittelt (`press_origin`), damit ein Ziehen,
//! das erst nach der Bewegungsschwelle erkannt wird, das richtige Gerät greift.
//!
//! **Unbegrenzter Plan:** Positionen sind Planeinheiten (eine Einheit =
//! [`WORLD`] Punkte bei Zoom 100 %), Geräte dürfen überall liegen. Eine Kamera
//! (Zoom + Mittelpunkt, [`StageInteraction`]) bildet den Plan auf die Fläche ab
//! – ändert sich die Größe der Fläche, ändert sich nur der Ausschnitt, nie
//! das Layout.
//! Mittlere Maustaste zieht die Ansicht, das Mausrad zoomt zur Mausposition.

use std::collections::BTreeSet;

use dmxcraft_core::{plan, ChannelKind, DmxFrame, FixtureId, PlanShape, Show};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const BOX: f32 = 28.0;
/// Länge einer Planeinheit in Punkten bei Zoom 100 %.
pub const WORLD: f32 = 720.0;
/// Zoom-Bereich.
pub const ZOOM_MIN: f32 = 0.2;
pub const ZOOM_MAX: f32 = 4.0;

/// Rückruf für das Kontextmenü: (Ui, Gerät unter der Maus, aktuelle Auswahl).
pub type MenuFn<'m> = dyn FnMut(&mut Ui, Option<FixtureId>, &BTreeSet<FixtureId>) + 'm;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CanvasMode {
    Select,
    Edit,
}

/// Interaktionszustand, den die besitzende Ansicht zwischen Frames hält.
pub struct StageInteraction {
    lasso_start: Option<Pos2>,
    moving: bool,
    /// Gerät, auf dem das Kontextmenü geöffnet wurde.
    context_target: Option<FixtureId>,
    /// Kamera: Zoom (1.0 = 100 %) …
    pub zoom: f32,
    /// … und Bühnenpunkt (normiert) in der Mitte der Fläche.
    pub center: [f32; 2],
    /// Beim nächsten Zeichnen so zoomen, dass alle Geräte sichtbar sind.
    pub fit_request: bool,
    /// Fläche und Kamera des letzten Zeichnens (für Ebenen darüber).
    pub view: Option<(Rect, Camera)>,
}

impl Default for StageInteraction {
    fn default() -> Self {
        Self {
            lasso_start: None,
            moving: false,
            context_target: None,
            zoom: 1.0,
            center: [0.5, 0.5],
            // Beim ersten Anzeigen alles einpassen
            fit_request: true,
            view: None,
        }
    }
}

impl StageInteraction {
    /// Zoom um einen Faktor ändern (Knöpfe −/+).
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
    }
}

/// Zeichnung unter den Geräten.
pub type Underlay = dyn Fn(&egui::Painter, Camera);

/// Kamera für ein Frame: Bühne (normiert) ↔ Bildschirm.
#[derive(Clone, Copy)]
pub struct Camera {
    screen_center: Pos2,
    center: [f32; 2],
    zoom: f32,
}

impl Camera {
    pub fn scale(self) -> f32 {
        WORLD * self.zoom
    }
    pub fn to_screen(self, p: [f32; 2]) -> Pos2 {
        self.screen_center + Vec2::new(p[0] - self.center[0], p[1] - self.center[1]) * self.scale()
    }
    pub fn to_world(self, s: Pos2) -> [f32; 2] {
        let d = (s - self.screen_center) / self.scale();
        [self.center[0] + d.x, self.center[1] + d.y]
    }
}

/// Zoom und Mittelpunkt, sodass alle Geräte in `rect` passen.
fn fit(show: &Show, rect: Rect) -> (f32, [f32; 2]) {
    let pts: Vec<[f32; 2]> = show
        .fixtures
        .iter()
        .map(|f| fixture_pos(show, f.id))
        .collect();
    if pts.is_empty() {
        let z = (rect.width().min(rect.height()) / WORLD).clamp(ZOOM_MIN, ZOOM_MAX);
        return (z, [0.5, 0.5]);
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for p in &pts {
        x0 = x0.min(p[0]);
        y0 = y0.min(p[1]);
        x1 = x1.max(p[0]);
        y1 = y1.max(p[1]);
    }
    // Platz für Gerätegröße und Beschriftung lassen
    let pad = 2.5
        * BOX
        * 3.2_f32.min(
            show.fixtures
                .iter()
                .map(|f| show.plan_size(f.id))
                .fold(1.0, f32::max),
        );
    let (w, h) = ((x1 - x0) * WORLD, (y1 - y0) * WORLD);
    let zx = (rect.width() - pad) / w.max(1.0);
    let zy = (rect.height() - pad) / h.max(1.0);
    let zoom = zx.min(zy).clamp(ZOOM_MIN, ZOOM_MAX).min(2.0);
    (zoom, [(x0 + x1) / 2.0, (y0 + y1) / 2.0])
}

pub struct StageCanvas<'a> {
    pub show: &'a Show,
    pub frame: &'a DmxFrame,
    pub mode: CanvasMode,
    /// Raster zum Einrasten beim Verschieben (Felder pro Achse), `None` = frei.
    pub snap: Option<u32>,
    /// Hintergrundraster zeichnen.
    pub show_grid: bool,
    /// Immer alles einpassen (ohne Zoom- und Ziehbedienung, z. B. STEUERN).
    pub auto_fit: bool,
    /// Auswahlreihenfolge: ausgewählte Geräte tragen ihre Nummer.
    pub order: &'a [FixtureId],
    /// Unter den Geräten zeichnen (z. B. das Bild eines Mappings).
    pub underlay: Option<&'a Underlay>,
}

/// Position eines Geräts (gespeichert oder Standard nach Patch-Reihenfolge).
pub fn fixture_pos(show: &Show, id: FixtureId) -> [f32; 2] {
    show.plan_position(id)
}

impl StageCanvas<'_> {
    /// Zeichnet den Plan und verarbeitet Eingaben. Ändert die Auswahl direkt;
    /// gibt Positionsänderungen zurück (die Show ist hier nur geliehen).
    pub fn show(
        &self,
        ui: &mut Ui,
        state: &mut StageInteraction,
        selected: &mut BTreeSet<FixtureId>,
        menu: &mut MenuFn<'_>,
    ) -> Vec<(FixtureId, [f32; 2])> {
        let mut moves = Vec::new();
        let size = ui.available_size().max(Vec2::new(120.0, 120.0));
        let (rect, resp) = ui.allocate_exact_size(size, Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, theme::bg());

        // ----- Kamera -----
        if state.fit_request || self.auto_fit {
            // Unten Platz für die Zoom-Leiste lassen (nur mit Bedienung)
            let area = if self.auto_fit {
                rect
            } else {
                rect.with_max_y(rect.bottom() - 44.0)
            };
            (state.zoom, state.center) = fit(self.show, area);
            // Mittelpunkt auf die ganze Fläche beziehen
            let shift = (area.center().y - rect.center().y) / (WORLD * state.zoom);
            state.center[1] -= shift;
            state.fit_request = false;
        }
        // Mittlere Maustaste: Ansicht ziehen
        let panning = resp.dragged_by(egui::PointerButton::Middle);
        if panning {
            let d = resp.drag_delta() / (WORLD * state.zoom);
            state.center = [state.center[0] - d.x, state.center[1] - d.y];
        }
        // Mausrad: zur Mausposition zoomen
        if resp.hovered() && !self.auto_fit {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                if let Some(mouse) = resp.hover_pos() {
                    let before = Camera {
                        screen_center: rect.center(),
                        center: state.center,
                        zoom: state.zoom,
                    };
                    let anchor = before.to_world(mouse);
                    state.zoom_by((scroll * 0.0025).exp());
                    // Punkt unter der Maus bleibt stehen
                    let d = (mouse - rect.center()) / (WORLD * state.zoom);
                    state.center = [anchor[0] - d.x, anchor[1] - d.y];
                }
                ui.input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
            }
        }
        let cam = Camera {
            screen_center: rect.center(),
            center: state.center,
            zoom: state.zoom,
        };
        self.paint_stage(&painter, rect, cam);
        state.view = Some((rect, cam));
        if let Some(under) = self.underlay {
            under(&painter, cam);
        }

        if self.show.fixtures.is_empty() {
            return moves;
        }

        let to_screen = |p: [f32; 2]| cam.to_screen(p);
        let zoom = cam.zoom;
        let boxes: Vec<(FixtureId, Rect)> = self
            .show
            .fixtures
            .iter()
            .map(|f| {
                (
                    f.id,
                    fixture_rect(
                        self.show,
                        f.id,
                        to_screen(fixture_pos(self.show, f.id)),
                        zoom,
                    ),
                )
            })
            .collect();
        let hit = |p: Pos2| {
            boxes
                .iter()
                .rev()
                .find(|(_, b)| b.expand(3.0).contains(p))
                .map(|(id, _)| *id)
        };
        let (mods, press_origin) = ui.input(|i| (i.modifiers, i.pointer.press_origin()));
        let additive = mods.shift || mods.command;

        // ----- Ziehen -----
        if resp.drag_started_by(egui::PointerButton::Primary) {
            let origin = press_origin.or(resp.interact_pointer_pos());
            let target = origin.and_then(hit);
            match (self.mode, target) {
                (CanvasMode::Edit, Some(id)) => {
                    if !selected.contains(&id) {
                        if !additive {
                            selected.clear();
                        }
                        selected.insert(id);
                    }
                    state.moving = true;
                }
                _ => {
                    state.lasso_start = origin;
                }
            }
        }
        if resp.dragged_by(egui::PointerButton::Primary) && state.moving {
            let d = resp.drag_delta() / cam.scale();
            let (dx, dy) = (d.x, d.y);
            for id in selected.iter() {
                let p = fixture_pos(self.show, *id);
                moves.push((*id, [p[0] + dx, p[1] + dy]));
            }
        }
        let pointer = ui.input(|i| i.pointer.latest_pos());
        let lasso = match (state.lasso_start, pointer) {
            (Some(s), Some(p))
                if resp.dragged_by(egui::PointerButton::Primary) || resp.drag_stopped() =>
            {
                Some(Rect::from_two_pos(s, p))
            }
            _ => None,
        };
        if resp.drag_stopped() && !panning {
            if let Some(l) = lasso {
                if !additive {
                    selected.clear();
                }
                selected.extend(
                    boxes
                        .iter()
                        .filter(|(_, b)| l.intersects(*b))
                        .map(|(id, _)| *id),
                );
            }
            if state.moving {
                if let Some(cells) = self.snap {
                    for id in selected.iter() {
                        // Position inkl. der Bewegung dieses Frames
                        let p = moves
                            .iter()
                            .find(|(m, _)| m == id)
                            .map(|(_, p)| *p)
                            .unwrap_or_else(|| fixture_pos(self.show, *id));
                        moves.retain(|(m, _)| m != id);
                        moves.push((*id, plan::snap(p, cells)));
                    }
                }
            }
            state.lasso_start = None;
            state.moving = false;
        }

        // ----- Klicken -----
        if resp.clicked() {
            match resp.interact_pointer_pos().and_then(hit) {
                Some(id) if additive => {
                    if !selected.remove(&id) {
                        selected.insert(id);
                    }
                }
                Some(id) => {
                    selected.clear();
                    selected.insert(id);
                }
                None if !additive => selected.clear(),
                None => {}
            }
        }

        // ----- Kontextmenü -----
        if resp.secondary_clicked() {
            let target = resp.interact_pointer_pos().and_then(hit);
            if let Some(id) = target {
                if !selected.contains(&id) {
                    selected.clear();
                    selected.insert(id);
                }
            }
            state.context_target = target;
        }
        let target = state.context_target;
        crate::ui::widgets::menu::context(&resp, |ui| menu(ui, target, selected));

        // ----- Zeichnen -----
        let hovered = resp.hover_pos().and_then(hit);
        for (id, b) in &boxes {
            // während des Ziehens die neue Position zeigen
            let b = moves
                .iter()
                .find(|(m, _)| m == id)
                .map(|(_, p)| fixture_rect(self.show, *id, to_screen(*p), zoom))
                .unwrap_or(*b);
            let sel = selected.contains(id);
            let mut colors = live_colors(self.show, *id, self.frame);
            // Strobe: in der Dunkelphase aus
            if let Some(hz) = strobe_hz(self.show, *id, self.frame) {
                let time = ui.input(|i| i.time);
                // 0 Hz = Shutter zu
                if hz <= 0.0 || (time * f64::from(hz)).fract() > 0.35 {
                    for c in &mut colors {
                        *c = (c.0, 0.0);
                    }
                }
                ui.ctx().request_repaint();
            }
            paint_fixture(
                &painter,
                self.show.plan_shape(*id),
                b,
                self.show.plan_rotation(*id),
                &colors,
                sel,
                Some(*id) == hovered,
            );
            let time = ui.input(|i| i.time);
            if paint_effects(&painter, self.show, *id, self.frame, b, time) {
                ui.ctx().request_repaint();
            }
            // Nummer in der Auswahlreihenfolge
            if let Some(n) = sel
                .then(|| self.order.iter().position(|f| f == id))
                .flatten()
            {
                let size = (b.height().min(b.width()) * 0.45).clamp(9.0, 16.0);
                let galley = painter.layout_no_wrap(
                    (n + 1).to_string(),
                    egui::FontId::proportional(size),
                    Color32::WHITE,
                );
                let pad = Vec2::new(4.0, 1.0);
                let tag = Rect::from_center_size(b.center(), galley.size() + pad * 2.0);
                painter.rect_filled(tag, 3.0, Color32::from_black_alpha(150));
                painter.galley(tag.min + pad, galley, Color32::WHITE);
            }
        }
        // Infos zum Gerät nur beim Überfahren
        if let Some(f) = hovered.and_then(|id| self.show.fixture(id)) {
            let ty = self
                .show
                .type_of(f)
                .map(|t| format!("{} {}", t.manufacturer, t.model).trim().to_string())
                .unwrap_or_else(|| f.type_key.clone());
            let ch = self.show.fixture_channels(f.id);
            let addr = tr!(
                "Universum {u} · Adresse {a}–{b}",
                u = f.universe + 1,
                a = f.address,
                b = f.address + ch.max(1) - 1
            );
            let name = f.name.clone();
            // Einrichten: alles inkl. DIP-Schalter; Steuern: nur Name und Adresse
            let full = self.mode == CanvasMode::Edit;
            let address = f.address;
            resp.clone().on_hover_ui_at_pointer(|ui| {
                ui.label(egui::RichText::new(name).strong());
                if full {
                    ui.label(egui::RichText::new(ty).color(theme::text_dim()));
                }
                ui.label(egui::RichText::new(addr).color(theme::text_dim()));
                if full {
                    ui.add_space(theme::SP_XS);
                    crate::ui::widgets::DipSwitch {
                        address,
                        compact: false,
                    }
                    .show(ui);
                }
            });
        }
        if let Some(l) = lasso {
            painter.rect_filled(l, 2.0, theme::accent().gamma_multiply(0.07));
            painter.rect_stroke(
                l,
                2.0,
                Stroke::new(1.0, theme::accent()),
                StrokeKind::Inside,
            );
        }
        // Nur setzen, wenn nötig – sonst würde z. B. der Größenänderungs-Cursor
        // der Bereichsgrenzen überschrieben.
        if panning {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else if self.mode == CanvasMode::Edit {
            if state.moving {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            } else if hovered.is_some() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
        moves
    }

    /// Optional ein Raster über die ganze sichtbare Fläche (unbegrenzt).
    fn paint_stage(&self, painter: &egui::Painter, rect: Rect, cam: Camera) {
        if !self.show_grid {
            return;
        }
        let cells = self.snap.unwrap_or(24).max(1) as f32;
        let step = cam.scale() / cells;
        // Bei starkem Herauszoomen nur jede n-te Linie, damit es ruhig bleibt
        let mut every = 1.0;
        while step * every < 8.0 {
            every *= 2.0;
        }
        let unit = every / cells;
        let tl = cam.to_world(rect.min);
        let br = cam.to_world(rect.max);
        let line = Stroke::new(1.0, theme::border().gamma_multiply(0.6));
        let mut x = (tl[0] / unit).floor() * unit;
        while x <= br[0] {
            let sx = cam.to_screen([x, 0.0]).x;
            painter.line_segment(
                [Pos2::new(sx, rect.top()), Pos2::new(sx, rect.bottom())],
                line,
            );
            x += unit;
        }
        let mut y = (tl[1] / unit).floor() * unit;
        while y <= br[1] {
            let sy = cam.to_screen([0.0, y]).y;
            painter.line_segment(
                [Pos2::new(rect.left(), sy), Pos2::new(rect.right(), sy)],
                line,
            );
            y += unit;
        }
    }
}

/// Bildschirm-Rechteck eines Geräts bei einem Zoomfaktor.
pub fn fixture_rect(show: &Show, id: FixtureId, center: Pos2, zoom: f32) -> Rect {
    let s = BOX * show.plan_size(id) * zoom;
    let size = match show.plan_shape(id) {
        PlanShape::BarHorizontal => Vec2::new(s * 2.8, s * 0.55),
        PlanShape::BarVertical => Vec2::new(s * 0.55, s * 2.8),
        _ => Vec2::splat(s),
    };
    Rect::from_center_size(center, size)
}

/// Blitzfrequenz eines Geräts, wenn sein Strobe-Kanal blitzt (`Some(0.0)`:
/// Shutter zu, `None`: offen). Nutzt die Wertebereiche der Gerätedatei; ohne
/// sie gelten Werte unter 10 als offen, darüber 1 bis 20 Hz.
pub fn strobe_hz(show: &Show, id: FixtureId, frame: &DmxFrame) -> Option<f32> {
    let f = show.fixture(id)?;
    let ty = show.type_of(f)?;
    let i = ty
        .channels
        .iter()
        .position(|c| c.kind == ChannelKind::Strobe)?;
    let v = frame.get(usize::from(f.universe), f.address + i as u16)?;
    ty.channels[i].strobe(v)
}

/// Anzeigefarben der einzelnen Lichtarten (linear 0–1).
const AMBER: [f32; 3] = [1.0, 0.55, 0.05];
const UV: [f32; 3] = [0.5, 0.12, 1.0];
const WHITE: [f32; 3] = [1.0, 0.95, 0.88];

/// Eine Anzeigefarbe pro Pixel (bei Leisten mehrere, sonst eine) mit Helligkeit.
///
/// Berücksichtigt Rot, Grün, Blau, Weiß, Amber und UV (UV violett), dazu
/// Farbräder/Farbmakros: Stehen alle Farbkanäle auf 0, zeigt das Gerät die
/// angenäherte Farbe des Rads.
pub fn live_colors(show: &Show, id: FixtureId, frame: &DmxFrame) -> Vec<(Color32, f32)> {
    let Some(f) = show.fixture(id) else {
        return vec![(Color32::DARK_GRAY, 0.0)];
    };
    let Some(ty) = show.type_of(f) else {
        return vec![(Color32::DARK_GRAY, 0.0)];
    };
    let value = |i: usize| -> f32 {
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map_or(0.0, |v| f32::from(v) / 255.0)
    };
    let all = |k: ChannelKind| -> Vec<usize> {
        ty.channels
            .iter()
            .enumerate()
            .filter(|(_, c)| c.kind == k)
            .map(|(i, _)| i)
            .collect()
    };
    let (reds, greens, blues, whites, ambers, uvs) = (
        all(ChannelKind::Red),
        all(ChannelKind::Green),
        all(ChannelKind::Blue),
        all(ChannelKind::White),
        all(ChannelKind::Amber),
        all(ChannelKind::Uv),
    );
    let dimmers = all(ChannelKind::Dimmer);
    let wheel_ch = all(ChannelKind::ColorWheel).first().copied();
    let wheel = wheel_ch.map(value);
    let pixels = [&reds, &greens, &blues, &whites, &uvs]
        .iter()
        .map(|v| v.len())
        .max()
        .unwrap_or(0)
        .max(1);
    let has_color = [&reds, &greens, &blues, &whites, &ambers, &uvs]
        .iter()
        .any(|v| !v.is_empty());
    (0..pixels)
        .map(|px| {
            let pick = |v: &Vec<usize>| -> f32 {
                v.get(px).or_else(|| v.first()).map_or(0.0, |i| value(*i))
            };
            let mut base = [0.0f32; 3];
            let mut add = |c: [f32; 3], x: f32| {
                for k in 0..3 {
                    base[k] += c[k] * x;
                }
            };
            add([1.0, 0.0, 0.0], pick(&reds));
            add([0.0, 1.0, 0.0], pick(&greens));
            add([0.0, 0.0, 1.0], pick(&blues));
            add(WHITE, pick(&whites));
            add(AMBER, pick(&ambers));
            add(UV, pick(&uvs));
            let mixed = base.iter().any(|x| *x > 0.003);
            if !has_color || !mixed {
                // Farbrad bzw. Farbmakro, sonst Halogen-Weiß
                if let Some(w) = wheel.filter(|w| *w > 0.03 || !has_color) {
                    let raw = (w * 255.0).round() as u8;
                    let c = wheel_ch.map_or_else(
                        || crate::viz::rig::color_wheel(w),
                        |i| crate::viz::rig::wheel_color(&ty.channels[i], raw),
                    );
                    base = [c.x, c.y, c.z];
                } else if !has_color {
                    base = WHITE;
                }
            }
            // Pixel-Dimmer, falls vorhanden, sonst der erste Dimmer
            let intensity = match (dimmers.len(), has_color || wheel.is_some()) {
                (0, true) => 1.0,
                (0, false) => 0.0,
                (n, _) if n == pixels => value(dimmers[px]),
                _ => value(dimmers[0]),
            };
            // Farbrad ohne Farbkanäle und ohne Dimmer: an, sobald das Rad steht
            let intensity = if !has_color && dimmers.is_empty() {
                wheel.map_or(0.0, |w| if w > 0.03 { 1.0 } else { 0.0 })
            } else {
                intensity
            };
            let c = base.map(|x| (x * intensity).min(1.0));
            let lum = c[0].max(c[1]).max(c[2]);
            (
                Color32::from_rgb(
                    (c[0] * 255.0) as u8,
                    (c[1] * 255.0) as u8,
                    (c[2] * 255.0) as u8,
                ),
                lum,
            )
        })
        .collect()
}

/// Effekte über dem Gerät: Laser (bunte Punkte/Strahlen, drehen sich mit
/// der Laser-Rotation), Nebel (Wolke nach Ausstoß und Lüfter) und die
/// Pan/Tilt-Richtung bei Moving Heads. Gibt `true` zurück, solange etwas
/// animiert wird.
pub fn paint_effects(
    p: &egui::Painter,
    show: &Show,
    id: FixtureId,
    frame: &DmxFrame,
    r: Rect,
    time: f64,
) -> bool {
    let Some(f) = show.fixture(id) else {
        return false;
    };
    let Some(ty) = show.type_of(f) else {
        return false;
    };
    let value = |i: usize| -> f32 {
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map_or(0.0, |v| f32::from(v) / 255.0)
    };
    let find = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k);
    let named = |k: ChannelKind, word: &str| {
        ty.channels
            .iter()
            .position(|c| c.kind == k && c.name.to_lowercase().contains(word))
    };
    let c = r.center();
    let size = r.width().max(r.height());
    let t = time as f32;
    let mut animating = false;

    // ----- Nebel -----
    if let Some(i) = find(ChannelKind::Fog) {
        let out = value(i);
        let fan = find(ChannelKind::Fan).map_or(0.5, value);
        if out > 0.02 {
            animating = true;
            let n = 7;
            let spread = size * (0.9 + fan * 1.3);
            for k in 0..n {
                let seed = k as f32 * 1.7;
                let phase = (t * (0.15 + fan * 0.35) + seed * 0.37).fract();
                let dir = egui::Vec2::angled(seed * 2.4 + (t * 0.1).sin() * 0.3);
                let pos = c + dir * spread * 0.25 + Vec2::new(0.0, -phase * spread * 0.6);
                let rad = size * (0.35 + phase * 0.5);
                let alpha = (out * (1.0 - phase) * 70.0) as u8;
                p.circle_filled(pos, rad, Color32::from_white_alpha(alpha));
            }
        }
    }

    // ----- Laser -----
    if let Some(i) = find(ChannelKind::Laser) {
        let v = value(i);
        // eigener Laser-Strobe (Name enthält „laser“) schaltet dunkel
        let gate = named(ChannelKind::Strobe, "laser").is_none_or(|s| {
            let sv = value(s);
            sv < 0.04 || (time * f64::from(1.0 + sv * 19.0)).fract() < 0.35
        });
        if v > 0.02 && gate {
            animating = true;
            let rot = named(ChannelKind::Speed, "laser")
                .or_else(|| named(ChannelKind::Speed, "rot"))
                .map_or(0.3, value);
            let angle = t * (0.2 + rot * 3.0);
            // Muster aus dem Kanalwert: Anzahl Punkte und Farben wechseln
            let pattern = (v * 7.99) as usize;
            let dots = 6 + pattern * 3;
            let cols = [
                Color32::from_rgb(0xff, 0x30, 0x30),
                Color32::from_rgb(0x30, 0xff, 0x50),
                Color32::from_rgb(0x40, 0x80, 0xff),
            ];
            let reach = size * 2.2;
            for k in 0..dots {
                let a = angle + std::f32::consts::TAU * k as f32 / dots as f32;
                let wobble = ((k * 7 + pattern) % 5) as f32 / 5.0;
                let end = c + Vec2::angled(a) * reach * (0.55 + wobble * 0.45);
                let col = cols[(k + pattern) % 3];
                p.line_segment([c, end], Stroke::new(1.0, col.gamma_multiply(0.35)));
                p.circle_filled(end, 2.0, col);
            }
        }
    }

    // ----- Pan/Tilt -----
    if let (Some(pi), Some(ti)) = (find(ChannelKind::Pan), find(ChannelKind::Tilt)) {
        // Pan 0–540°, Tilt aus der Mitte (128) heraus = Länge des Zeigers
        let pan = value(pi) * 540f32.to_radians() - std::f32::consts::FRAC_PI_2;
        let tilt = (value(ti) - 0.5) * 2.0;
        // nach hinten geneigt = Zeiger in Gegenrichtung
        let dir = if tilt < 0.0 {
            -Vec2::angled(pan)
        } else {
            Vec2::angled(pan)
        };
        let end = c + dir * (size * 0.35 + tilt.abs() * size * 0.7);
        let s = Stroke::new(1.5, theme::accent());
        p.line_segment([c, end], s);
        p.circle_filled(end, 2.5, theme::accent());
    }
    animating
}

/// Zeichnet ein Gerät in seiner Form – vollflächig in der aktuellen Farbe,
/// mit dünner Kontur, um `rot` Grad gedreht. Pixel-Geräte (Leisten, Matrix)
/// zeigen ihre Pixel als Segmente, alle anderen eine Farbe.
fn paint_fixture(
    p: &egui::Painter,
    shape: PlanShape,
    r: Rect,
    rot: f32,
    colors: &[(Color32, f32)],
    sel: bool,
    hovered: bool,
) {
    let stroke = if sel {
        Stroke::new(2.0, theme::accent())
    } else if hovered {
        Stroke::new(1.0, theme::text_dim())
    } else {
        Stroke::new(1.0, theme::border())
    };
    let c = r.center();
    let (sin, cos) = rot.to_radians().sin_cos();
    // Punkt um die Mitte drehen
    let turn = |q: Pos2| -> Pos2 {
        let d = q - c;
        c + Vec2::new(d.x * cos - d.y * sin, d.x * sin + d.y * cos)
    };
    let quad = |q: Rect| -> Vec<Pos2> {
        [
            q.left_top(),
            q.right_top(),
            q.right_bottom(),
            q.left_bottom(),
        ]
        .into_iter()
        .map(turn)
        .collect()
    };
    // hellste Farbe für einfarbige Formen
    let (c0, l0) = colors
        .iter()
        .copied()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap_or((Color32::BLACK, 0.0));
    let n = colors.len().max(1);
    match shape {
        PlanShape::Round => {
            let rad = r.width().min(r.height()) * 0.5;
            p.circle_filled(c, rad, c0);
            p.circle_stroke(c, rad, stroke);
        }
        PlanShape::Square => {
            p.add(egui::Shape::convex_polygon(quad(r), c0, stroke));
        }
        PlanShape::Matrix => {
            let side = (n as f32).sqrt().ceil().max(1.0) as usize;
            let (cw, ch) = (r.width() / side as f32, r.height() / side as f32);
            for i in 0..side * side {
                let (col, _) = colors.get(i).copied().unwrap_or((c0, l0));
                let cell = Rect::from_min_size(
                    Pos2::new(
                        r.left() + cw * (i % side) as f32,
                        r.top() + ch * (i / side) as f32,
                    ),
                    Vec2::new(cw, ch),
                );
                p.add(egui::Shape::convex_polygon(
                    quad(cell.shrink(0.5)),
                    col,
                    Stroke::NONE,
                ));
            }
            p.add(egui::Shape::closed_line(quad(r), stroke));
        }
        PlanShape::BarHorizontal | PlanShape::BarVertical => {
            let horizontal = shape == PlanShape::BarHorizontal;
            for (i, (col, _)) in colors.iter().enumerate() {
                let seg = if horizontal {
                    let w = r.width() / n as f32;
                    Rect::from_min_size(
                        Pos2::new(r.left() + w * i as f32, r.top()),
                        Vec2::new(w, r.height()),
                    )
                } else {
                    let h = r.height() / n as f32;
                    Rect::from_min_size(
                        Pos2::new(r.left(), r.top() + h * i as f32),
                        Vec2::new(r.width(), h),
                    )
                };
                let seg = if n > 1 { seg.shrink(0.5) } else { seg };
                p.add(egui::Shape::convex_polygon(quad(seg), *col, Stroke::NONE));
            }
            p.add(egui::Shape::closed_line(quad(r), stroke));
        }
        PlanShape::Effect => {
            let pts = [
                Pos2::new(c.x, r.top()),
                Pos2::new(r.right(), r.bottom()),
                Pos2::new(r.left(), r.bottom()),
            ]
            .into_iter()
            .map(turn)
            .collect();
            p.add(egui::Shape::convex_polygon(pts, c0, stroke));
        }
    }
}

/// Kleines Symbol einer Geräteform (für Menüs und Knöpfe). `None` = automatisch.
pub fn paint_shape_icon(p: &egui::Painter, shape: Option<PlanShape>, r: Rect, color: Color32) {
    let stroke = Stroke::new(1.4, color);
    let c = r.center();
    let s = r.width().min(r.height());
    match shape {
        None => {
            p.circle_stroke(c, s * 0.42, Stroke::new(1.2, color.gamma_multiply(0.6)));
            p.text(
                c,
                egui::Align2::CENTER_CENTER,
                "A",
                egui::FontId::proportional(s * 0.55),
                color,
            );
        }
        Some(PlanShape::Round) => {
            p.circle_filled(c, s * 0.42, color);
        }
        Some(PlanShape::Square) => {
            p.rect_filled(Rect::from_center_size(c, Vec2::splat(s * 0.8)), 1.5, color);
        }
        Some(PlanShape::Matrix) => {
            let cell = s * 0.8 / 3.0;
            let origin = c - Vec2::splat(s * 0.4);
            for i in 0..9 {
                let rr = Rect::from_min_size(
                    origin + Vec2::new(cell * (i % 3) as f32, cell * (i / 3) as f32),
                    Vec2::splat(cell),
                );
                p.rect_filled(rr.shrink(0.8), 0.5, color);
            }
        }
        Some(PlanShape::BarHorizontal) => {
            let bar = Rect::from_center_size(c, Vec2::new(s * 0.95, s * 0.3));
            for i in 0..4 {
                let w = bar.width() / 4.0;
                let seg = Rect::from_min_size(
                    egui::pos2(bar.left() + w * i as f32, bar.top()),
                    Vec2::new(w, bar.height()),
                );
                p.rect_filled(seg.shrink(0.6), 0.5, color);
            }
        }
        Some(PlanShape::BarVertical) => {
            let bar = Rect::from_center_size(c, Vec2::new(s * 0.3, s * 0.95));
            for i in 0..4 {
                let h = bar.height() / 4.0;
                let seg = Rect::from_min_size(
                    egui::pos2(bar.left(), bar.top() + h * i as f32),
                    Vec2::new(bar.width(), h),
                );
                p.rect_filled(seg.shrink(0.6), 0.5, color);
            }
        }
        Some(PlanShape::Effect) => {
            let pts = vec![
                egui::pos2(c.x, c.y - s * 0.42),
                egui::pos2(c.x + s * 0.45, c.y + s * 0.38),
                egui::pos2(c.x - s * 0.45, c.y + s * 0.38),
            ];
            p.add(egui::Shape::convex_polygon(pts, color, stroke));
        }
    }
}
