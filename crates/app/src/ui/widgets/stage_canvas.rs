//! Bühnenplan-Zeichenfläche.
//!
//! * [`CanvasMode::Select`] (STEUERN): Klicken/Aufziehen wählt Geräte aus.
//! * [`CanvasMode::Edit`] (EINRICHTEN → Bühnenplan): Ziehen verschiebt Geräte,
//!   Aufziehen auf freier Fläche wählt aus.
//!
//! Treffer werden am Druckpunkt ermittelt (`press_origin`), damit ein Ziehen,
//! das erst nach der Bewegungsschwelle erkannt wird, das richtige Gerät greift.

use std::collections::BTreeSet;

use dmxcraft_core::{plan, ChannelKind, DmxFrame, FixtureId, Show};
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
        let r = theme::R_M as f32;
        painter.rect_filled(rect, r, theme::BG);
        if self.show_grid {
            self.paint_grid(&painter, rect);
        }
        painter.rect_stroke(rect, r, Stroke::new(1.0, theme::BORDER), StrokeKind::Inside);

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
                    Rect::from_center_size(
                        to_screen(fixture_pos(self.show, f.id)),
                        Vec2::splat(BOX),
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
                .map(|(_, p)| Rect::from_center_size(to_screen(*p), Vec2::splat(BOX)))
                .unwrap_or(*b);
            let (c, lum) = live_color(self.show, *id, self.frame);
            if lum > 0.02 {
                painter.circle_filled(b.center(), BOX * 0.85, c.gamma_multiply(0.14 * lum));
            }
            painter.rect_filled(b, 6.0, theme::RAISED);
            painter.circle_filled(
                b.center(),
                BOX * 0.3,
                if lum > 0.02 { c } else { theme::BG },
            );
            let sel = selected.contains(id);
            let stroke = if sel {
                Stroke::new(2.0, theme::ACCENT)
            } else if Some(*id) == hovered {
                Stroke::new(1.0, theme::TEXT_DIM)
            } else {
                Stroke::new(1.0, theme::BORDER)
            };
            painter.rect_stroke(b, 6.0, stroke, StrokeKind::Outside);
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

/// Anzeigefarbe und Helligkeit eines Geräts aus den aktuellen DMX-Werten.
pub fn live_color(show: &Show, id: FixtureId, frame: &DmxFrame) -> (Color32, f32) {
    let Some(f) = show.fixture(id) else {
        return (Color32::DARK_GRAY, 0.0);
    };
    let Some(ty) = show.type_of(f) else {
        return (Color32::DARK_GRAY, 0.0);
    };
    let val = |k: ChannelKind| -> Option<f32> {
        let i = ty.channels.iter().position(|c| c.kind == k)?;
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map(|v| f32::from(v) / 255.0)
    };
    let (r, g, b) = (
        val(ChannelKind::Red),
        val(ChannelKind::Green),
        val(ChannelKind::Blue),
    );
    let w = val(ChannelKind::White).unwrap_or(0.0);
    let has_color = r.is_some() || g.is_some() || b.is_some();
    let base = if has_color {
        [
            r.unwrap_or(0.0) + w,
            g.unwrap_or(0.0) + w,
            b.unwrap_or(0.0) + w,
        ]
    } else {
        [1.0, 0.95, 0.85]
    };
    let intensity = match (val(ChannelKind::Dimmer), has_color) {
        (Some(d), _) => d,
        (None, true) => 1.0,
        (None, false) => 0.0,
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
}
