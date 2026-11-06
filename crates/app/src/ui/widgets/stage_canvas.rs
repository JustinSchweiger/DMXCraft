//! Bühnenplan-Zeichenfläche.
//!
//! * [`CanvasMode::Select`] (STEUERN): Klicken/Aufziehen wählt Geräte aus.
//! * [`CanvasMode::Edit`] (EINRICHTEN → Bühnenplan): Ziehen verschiebt Geräte,
//!   Aufziehen auf freier Fläche wählt aus.
//!
//! Treffer werden am Druckpunkt ermittelt (`press_origin`), damit ein Ziehen,
//! das erst nach der Bewegungsschwelle erkannt wird, das richtige Gerät greift.

use std::collections::BTreeSet;

use dmxcraft_core::{plan, ChannelKind, DmxFrame, FixtureId, PlanShape, Show};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const BOX: f32 = 28.0;

/// Rückruf für das Kontextmenü: (Ui, Gerät unter der Maus, aktuelle Auswahl).
pub type MenuFn<'m> = dyn FnMut(&mut Ui, Option<FixtureId>, &BTreeSet<FixtureId>) + 'm;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CanvasMode {
    Select,
    Edit,
}

/// Interaktionszustand, den die besitzende Ansicht zwischen Frames hält.
#[derive(Default)]
pub struct StageInteraction {
    lasso_start: Option<Pos2>,
    moving: bool,
    /// Gerät, auf dem das Kontextmenü geöffnet wurde.
    context_target: Option<FixtureId>,
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
        painter.rect_filled(rect, 0.0, theme::BG);
        if self.show_grid {
            self.paint_grid(&painter, rect);
        }

        if self.show.fixtures.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Noch keine Geräte – unter EINRICHTEN → Patch hinzufügen",
                theme::body(),
                theme::TEXT_DIM,
            );
            return moves;
        }

        let to_screen = |p: [f32; 2]| {
            let inner = rect.shrink(BOX * 0.6);
            Pos2::new(
                inner.left() + p[0] * inner.width(),
                inner.top() + p[1] * inner.height(),
            )
        };
        let boxes: Vec<(FixtureId, Rect)> = self
            .show
            .fixtures
            .iter()
            .map(|f| {
                (
                    f.id,
                    fixture_rect(self.show, f.id, to_screen(fixture_pos(self.show, f.id))),
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
        if resp.drag_started() {
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
        if resp.dragged() && state.moving {
            let d = resp.drag_delta();
            let inner = rect.shrink(BOX * 0.6);
            let (dx, dy) = (d.x / inner.width(), d.y / inner.height());
            for id in selected.iter() {
                let p = fixture_pos(self.show, *id);
                moves.push((*id, plan::clamp([p[0] + dx, p[1] + dy])));
            }
        }
        let pointer = ui.input(|i| i.pointer.latest_pos());
        let lasso = match (state.lasso_start, pointer) {
            (Some(s), Some(p)) if resp.dragged() || resp.drag_stopped() => {
                Some(Rect::from_two_pos(s, p))
            }
            _ => None,
        };
        if resp.drag_stopped() {
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
        resp.context_menu(|ui| menu(ui, target, selected));

        // ----- Zeichnen -----
        let hovered = resp.hover_pos().and_then(hit);
        for (id, b) in &boxes {
            // während des Ziehens die neue Position zeigen
            let b = moves
                .iter()
                .find(|(m, _)| m == id)
                .map(|(_, p)| fixture_rect(self.show, *id, to_screen(*p)))
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
                        if sel { theme::ACCENT } else { theme::TEXT_DIM },
                    );
                }
            }
        }
        if let Some(l) = lasso {
            painter.rect_filled(l, 2.0, theme::ACCENT.gamma_multiply(0.07));
            painter.rect_stroke(l, 2.0, Stroke::new(1.0, theme::ACCENT), StrokeKind::Inside);
        }
        if self.mode == CanvasMode::Edit {
            ui.ctx().set_cursor_icon(if state.moving {
                egui::CursorIcon::Grabbing
            } else if hovered.is_some() {
                egui::CursorIcon::Grab
            } else {
                egui::CursorIcon::Default
            });
        }
        moves
    }

    fn paint_grid(&self, painter: &egui::Painter, rect: Rect) {
        let cells = self.snap.unwrap_or(24) as f32;
        let inner = rect.shrink(BOX * 0.6);
        let line = Stroke::new(1.0, Color32::from_rgb(0x16, 0x1b, 0x22));
        for i in 0..=cells as u32 {
            let f = i as f32 / cells;
            let x = inner.left() + inner.width() * f;
            let y = inner.top() + inner.height() * f;
            painter.line_segment(
                [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
                line,
            );
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                line,
            );
        }
        painter.rect_stroke(
            rect,
            theme::R_M as f32,
            Stroke::new(1.0, theme::BORDER),
            StrokeKind::Inside,
        );
    }
}

/// Bildschirm-Rechteck eines Geräts nach Form und Größe.
pub fn fixture_rect(show: &Show, id: FixtureId, center: Pos2) -> Rect {
    let s = BOX * show.plan_size(id);
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
        Stroke::new(2.0, theme::ACCENT)
    } else if hovered {
        Stroke::new(1.0, theme::TEXT_DIM)
    } else {
        Stroke::new(1.0, theme::BORDER)
    };
    let fill = |c: Color32, l: f32| if l > 0.02 { c } else { theme::FIXTURE_OFF };
    let glow = |center: Pos2, radius: f32, c: Color32, l: f32| {
        if l > 0.02 {
            p.circle_filled(center, radius, c.gamma_multiply(0.12 * l));
        }
    };
    let (c0, l0) = colors.first().copied().unwrap_or((theme::BG, 0.0));
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
                        Stroke::new(0.5, theme::BG),
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
