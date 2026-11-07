//! Bühnenplan-Zeichenfläche.
//!
//! * [`CanvasMode::Select`] (STEUERN): Klicken/Aufziehen wählt Geräte aus.
//! * [`CanvasMode::Edit`] (EINRICHTEN → Bühnenplan): Ziehen verschiebt Geräte,
//!   Aufziehen auf freier Fläche wählt aus.
//!
//! Treffer werden am Druckpunkt ermittelt (`press_origin`), damit ein Ziehen,
//! das erst nach der Bewegungsschwelle erkannt wird, das richtige Gerät greift.
//!
//! **Feste Bühne:** Positionen sind normiert (0..1) und liegen auf einer
//! quadratischen Bühne mit fester Größe [`WORLD`]. Eine Kamera (Zoom +
//! Mittelpunkt, [`StageInteraction`]) bildet sie auf die Fläche ab – ändert
//! sich die Größe der Fläche, ändert sich nur der Ausschnitt, nie das Layout.
//! Mittlere Maustaste zieht die Ansicht, das Mausrad zoomt zur Mausposition.

use crate::i18n::t;
use std::collections::BTreeSet;

use dmxcraft_core::{plan, ChannelKind, DmxFrame, FixtureId, PlanShape, Show};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const BOX: f32 = 28.0;
/// Kantenlänge der Bühne in Punkten bei Zoom 100 %.
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
        }
    }
}

impl StageInteraction {
    /// Zoom um einen Faktor ändern (Knöpfe −/+).
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
    }
}

/// Kamera für ein Frame: Bühne (normiert) ↔ Bildschirm.
#[derive(Clone, Copy)]
struct Camera {
    screen_center: Pos2,
    center: [f32; 2],
    zoom: f32,
}

impl Camera {
    fn scale(self) -> f32 {
        WORLD * self.zoom
    }
    fn to_screen(self, p: [f32; 2]) -> Pos2 {
        self.screen_center + Vec2::new(p[0] - self.center[0], p[1] - self.center[1]) * self.scale()
    }
    fn to_world(self, s: Pos2) -> [f32; 2] {
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
    pub show_labels: bool,
    /// Hintergrundraster zeichnen.
    pub show_grid: bool,
    /// Immer alles einpassen (ohne Zoom- und Ziehbedienung, z. B. STEUERN).
    pub auto_fit: bool,
}

/// Position eines Geräts (gespeichert oder Standard nach Patch-Reihenfolge).
pub fn fixture_pos(show: &Show, id: FixtureId) -> [f32; 2] {
    let idx = show.fixtures.iter().position(|f| f.id == id).unwrap_or(0);
    show.fixture(id)
        .and_then(|f| f.plan_pos)
        .unwrap_or_else(|| plan::default_position(idx))
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

        if self.show.fixtures.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                t("Noch keine Geräte – unter EINRICHTEN → Patch hinzufügen"),
                theme::body(),
                theme::text_dim(),
            );
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
                moves.push((*id, plan::clamp([p[0] + dx, p[1] + dy])));
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
            let colors = live_colors(self.show, *id, self.frame);
            paint_fixture(
                &painter,
                self.show.plan_shape(*id),
                b,
                &colors,
                sel,
                Some(*id) == hovered,
            );
            if self.show_labels || Some(*id) == hovered {
                if let Some(f) = self.show.fixture(*id) {
                    let text = if Some(*id) == hovered {
                        format!("{} · U{}/{}", f.name, f.universe + 1, f.address)
                    } else {
                        f.name.clone()
                    };
                    painter.text(
                        b.center_bottom() + Vec2::new(0.0, 4.0),
                        egui::Align2::CENTER_TOP,
                        text,
                        theme::caption(),
                        if sel {
                            theme::accent()
                        } else {
                            theme::text_dim()
                        },
                    );
                }
            }
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

    /// Bühnenfläche (etwas heller als der Hintergrund) und optional das Raster.
    fn paint_stage(&self, painter: &egui::Painter, rect: Rect, cam: Camera) {
        let stage = Rect::from_two_pos(cam.to_screen([0.0, 0.0]), cam.to_screen([1.0, 1.0]));
        painter.rect_filled(stage, theme::R_S as f32, theme::surface());
        if self.show_grid {
            let cells = self.snap.unwrap_or(24).max(1);
            let line = Stroke::new(1.0, theme::border().gamma_multiply(0.6));
            for i in 0..=cells {
                let f = i as f32 / cells as f32;
                let x = stage.left() + stage.width() * f;
                let y = stage.top() + stage.height() * f;
                if (rect.left()..=rect.right()).contains(&x) {
                    painter.line_segment(
                        [Pos2::new(x, stage.top()), Pos2::new(x, stage.bottom())],
                        line,
                    );
                }
                if (rect.top()..=rect.bottom()).contains(&y) {
                    painter.line_segment(
                        [Pos2::new(stage.left(), y), Pos2::new(stage.right(), y)],
                        line,
                    );
                }
            }
        }
        painter.rect_stroke(
            stage,
            theme::R_S as f32,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );
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

/// Eine Anzeigefarbe pro Pixel (bei Leisten mehrere, sonst eine) mit Helligkeit.
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
    let (reds, greens, blues, whites) = (
        all(ChannelKind::Red),
        all(ChannelKind::Green),
        all(ChannelKind::Blue),
        all(ChannelKind::White),
    );
    let dimmers = all(ChannelKind::Dimmer);
    let pixels = reds.len().max(greens.len()).max(blues.len()).max(1);
    let has_color = !(reds.is_empty() && greens.is_empty() && blues.is_empty());
    (0..pixels)
        .map(|px| {
            let pick = |v: &Vec<usize>| -> Option<f32> {
                v.get(px).or_else(|| v.first()).map(|i| value(*i))
            };
            let w = pick(&whites).unwrap_or(0.0);
            let base = if has_color {
                [
                    pick(&reds).unwrap_or(0.0) + w,
                    pick(&greens).unwrap_or(0.0) + w,
                    pick(&blues).unwrap_or(0.0) + w,
                ]
            } else {
                [1.0, 0.95, 0.85]
            };
            // Pixel-Dimmer, falls vorhanden, sonst der erste Dimmer
            let intensity = match (dimmers.len(), has_color) {
                (0, true) => 1.0,
                (0, false) => 0.0,
                (n, _) if n == pixels => value(dimmers[px]),
                _ => value(dimmers[0]),
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

/// Zeichnet ein Gerät in seiner Form – vollflächig in der aktuellen Farbe,
/// mit dünner Kontur. Mehrere Pixel werden als Segmente gezeigt.
fn paint_fixture(
    p: &egui::Painter,
    shape: PlanShape,
    r: Rect,
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
    let fill = |c: Color32, l: f32| if l > 0.02 { c } else { theme::fixture_off() };
    let glow = |center: Pos2, radius: f32, c: Color32, l: f32| {
        if l > 0.02 {
            p.circle_filled(center, radius, c.gamma_multiply(0.12 * l));
        }
    };
    let (c0, l0) = colors.first().copied().unwrap_or((theme::bg(), 0.0));
    let n = colors.len().max(1);
    match shape {
        PlanShape::Round | PlanShape::MovingHead => {
            let rad = r.width().min(r.height()) * 0.5;
            let max_l = colors.iter().map(|c| c.1).fold(0.0, f32::max);
            glow(r.center(), rad * 1.6, c0, max_l);
            if n == 1 {
                p.circle_filled(r.center(), rad, fill(c0, l0));
            } else {
                // Segmente wie ein LED-Ring
                let steps = 48 / n.min(48) + 1;
                for (i, (c, l)) in colors.iter().enumerate() {
                    let a0 =
                        std::f32::consts::TAU * i as f32 / n as f32 - std::f32::consts::FRAC_PI_2;
                    let a1 = std::f32::consts::TAU * (i + 1) as f32 / n as f32
                        - std::f32::consts::FRAC_PI_2;
                    let mut pts = vec![r.center()];
                    for k in 0..=steps {
                        let a = a0 + (a1 - a0) * k as f32 / steps as f32;
                        pts.push(r.center() + Vec2::angled(a) * rad);
                    }
                    p.add(egui::Shape::convex_polygon(
                        pts,
                        fill(*c, *l),
                        Stroke::new(0.5, theme::bg()),
                    ));
                }
            }
            p.circle_stroke(r.center(), rad, stroke);
        }
        PlanShape::Square => {
            glow(r.center(), r.width() * 0.8, c0, l0);
            p.rect_filled(r, 2.0, fill(c0, l0));
            p.rect_stroke(r, 2.0, stroke, StrokeKind::Outside);
        }
        PlanShape::Matrix => {
            let side = (n as f32).sqrt().ceil().max(1.0) as usize;
            let (cw, ch) = (r.width() / side as f32, r.height() / side as f32);
            for (i, (c, l)) in colors.iter().enumerate() {
                let cell = Rect::from_min_size(
                    Pos2::new(
                        r.left() + cw * (i % side) as f32,
                        r.top() + ch * (i / side) as f32,
                    ),
                    Vec2::new(cw, ch),
                );
                p.rect_filled(cell.shrink(0.5), 0.0, fill(*c, *l));
            }
            p.rect_stroke(r, 2.0, stroke, StrokeKind::Outside);
        }
        PlanShape::BarHorizontal | PlanShape::BarVertical => {
            let horizontal = shape == PlanShape::BarHorizontal;
            for (i, (c, l)) in colors.iter().enumerate() {
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
                glow(seg.center(), seg.width().max(seg.height()) * 0.8, *c, *l);
                p.rect_filled(if n > 1 { seg.shrink(0.5) } else { seg }, 0.0, fill(*c, *l));
            }
            p.rect_stroke(r, 2.0, stroke, StrokeKind::Outside);
        }
        PlanShape::Effect => {
            let pts = vec![
                Pos2::new(r.center().x, r.top()),
                Pos2::new(r.right(), r.bottom()),
                Pos2::new(r.left(), r.bottom()),
            ];
            glow(r.center(), r.width() * 0.8, c0, l0);
            p.add(egui::Shape::convex_polygon(pts, fill(c0, l0), stroke));
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
        Some(PlanShape::MovingHead) => {
            let rad = s * 0.42;
            for i in 0..6 {
                let a0 = std::f32::consts::TAU * i as f32 / 6.0;
                let a1 = std::f32::consts::TAU * (i as f32 + 0.8) / 6.0;
                let mut pts = vec![c];
                for k in 0..=4 {
                    let a = a0 + (a1 - a0) * k as f32 / 4.0;
                    pts.push(c + Vec2::angled(a) * rad);
                }
                p.add(egui::Shape::convex_polygon(pts, color, Stroke::NONE));
            }
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

/// Knopf mit Formsymbol.
pub struct ShapeButton {
    pub shape: Option<PlanShape>,
    pub selected: bool,
}

impl egui::Widget for ShapeButton {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(30.0), Sense::click());
        let p = ui.painter();
        let bg = if self.selected {
            theme::accent_dim()
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::raised()
        };
        p.rect_filled(rect, theme::R_S as f32, bg);
        if self.selected {
            p.rect_stroke(
                rect,
                theme::R_S as f32,
                Stroke::new(1.0, theme::accent()),
                StrokeKind::Inside,
            );
        }
        let color = if self.selected {
            theme::accent()
        } else {
            theme::text()
        };
        paint_shape_icon(p, self.shape, rect.shrink(7.0), color);
        let tip = self
            .shape
            .map_or(t("Automatisch (nach Gerätetyp)"), |s| t(s.label()));
        resp.on_hover_text(tip)
    }
}
