//! XY-Feld für Bewegungsgrenzen: das Rechteck zeigt den erlaubten Pan-
//! (waagerecht) und Tilt-Bereich (senkrecht). Griffe an Ecken und Kanten
//! ändern die Grenzen, Ziehen im Inneren verschiebt den ganzen Bereich.

use eframe::egui::{self, pos2, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

pub struct LimitPad<'a> {
    pub pan: (&'a mut u8, &'a mut u8),
    pub tilt: (&'a mut u8, &'a mut u8),
    pub size: Vec2,
    pub enabled: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum Grip {
    Corner(bool, bool),
    EdgeX(bool),
    EdgeY(bool),
    Body,
}

impl LimitPad<'_> {
    pub fn show(self, ui: &mut Ui) -> bool {
        let (rect, resp) = ui.allocate_exact_size(
            self.size,
            if self.enabled {
                Sense::drag()
            } else {
                Sense::hover()
            },
        );
        let to_x = |v: u8| rect.left() + f32::from(v) / 255.0 * rect.width();
        let to_y = |v: u8| rect.top() + f32::from(v) / 255.0 * rect.height();
        let from_x =
            |x: f32| (((x - rect.left()) / rect.width()).clamp(0.0, 1.0) * 255.0).round() as u8;
        let from_y =
            |y: f32| (((y - rect.top()) / rect.height()).clamp(0.0, 1.0) * 255.0).round() as u8;
        let area = Rect::from_min_max(
            pos2(to_x(*self.pan.0), to_y(*self.tilt.0)),
            pos2(to_x(*self.pan.1), to_y(*self.tilt.1)),
        );
        let grips: Vec<(Grip, Pos2)> = vec![
            (Grip::Corner(false, false), area.left_top()),
            (Grip::Corner(true, false), area.right_top()),
            (Grip::Corner(false, true), area.left_bottom()),
            (Grip::Corner(true, true), area.right_bottom()),
            (Grip::EdgeX(false), area.left_center()),
            (Grip::EdgeX(true), area.right_center()),
            (Grip::EdgeY(false), area.center_top()),
            (Grip::EdgeY(true), area.center_bottom()),
        ];
        let id = resp.id.with("grip");
        let mut changed = false;
        if resp.drag_started() {
            let origin = ui
                .input(|i| i.pointer.press_origin())
                .unwrap_or(rect.center());
            let grip = grips
                .iter()
                .find(|(_, p)| p.distance(origin) < 9.0)
                .map(|(g, _)| *g)
                .or_else(|| area.contains(origin).then_some(Grip::Body));
            ui.data_mut(|d| d.insert_temp(id, grip));
        }
        let grip: Option<Grip> = ui.data(|d| d.get_temp(id)).flatten();
        if resp.dragged() {
            if let (Some(g), Some(p)) = (grip, resp.interact_pointer_pos()) {
                let (x, y) = (from_x(p.x), from_y(p.y));
                match g {
                    Grip::Corner(right, bottom) => {
                        if right {
                            *self.pan.1 = x.max(*self.pan.0)
                        } else {
                            *self.pan.0 = x.min(*self.pan.1)
                        }
                        if bottom {
                            *self.tilt.1 = y.max(*self.tilt.0)
                        } else {
                            *self.tilt.0 = y.min(*self.tilt.1)
                        }
                    }
                    Grip::EdgeX(right) => {
                        if right {
                            *self.pan.1 = x.max(*self.pan.0)
                        } else {
                            *self.pan.0 = x.min(*self.pan.1)
                        }
                    }
                    Grip::EdgeY(bottom) => {
                        if bottom {
                            *self.tilt.1 = y.max(*self.tilt.0)
                        } else {
                            *self.tilt.0 = y.min(*self.tilt.1)
                        }
                    }
                    Grip::Body => {
                        let d = resp.drag_delta();
                        let shift = |lo: &mut u8, hi: &mut u8, delta: f32, span: f32| {
                            let dv = (delta / span * 255.0).round() as i32;
                            let dv = dv.clamp(-i32::from(*lo), 255 - i32::from(*hi));
                            *lo = (i32::from(*lo) + dv) as u8;
                            *hi = (i32::from(*hi) + dv) as u8;
                        };
                        shift(self.pan.0, self.pan.1, d.x, rect.width());
                        shift(self.tilt.0, self.tilt.1, d.y, rect.height());
                    }
                }
                changed = true;
            }
        }
        if resp.drag_stopped() {
            ui.data_mut(|d| d.remove::<Option<Grip>>(id));
        }

        // Zeichnen
        let p = ui.painter();
        p.rect_filled(rect, 0.0, theme::BG);
        let faint = Stroke::new(1.0, theme::RAISED);
        for i in 1..4 {
            let f = i as f32 / 4.0;
            p.line_segment(
                [
                    pos2(rect.left() + rect.width() * f, rect.top()),
                    pos2(rect.left() + rect.width() * f, rect.bottom()),
                ],
                faint,
            );
            p.line_segment(
                [
                    pos2(rect.left(), rect.top() + rect.height() * f),
                    pos2(rect.right(), rect.top() + rect.height() * f),
                ],
                faint,
            );
        }
        p.rect_stroke(
            rect,
            0.0,
            Stroke::new(1.0, theme::BORDER),
            StrokeKind::Inside,
        );
        let area = Rect::from_min_max(
            pos2(to_x(*self.pan.0), to_y(*self.tilt.0)),
            pos2(to_x(*self.pan.1), to_y(*self.tilt.1)),
        );
        let c = if self.enabled {
            theme::ACCENT
        } else {
            theme::TEXT_FAINT
        };
        p.rect_filled(area, 0.0, c.gamma_multiply(0.08));
        p.rect_stroke(area, 0.0, Stroke::new(1.5, c), StrokeKind::Middle);
        p.line_segment(
            [area.center_top(), area.center_bottom()],
            Stroke::new(1.0, c.gamma_multiply(0.4)),
        );
        p.line_segment(
            [area.left_center(), area.right_center()],
            Stroke::new(1.0, c.gamma_multiply(0.4)),
        );
        if self.enabled {
            let hover = resp.hover_pos();
            for (_, gp) in [
                (0, area.left_top()),
                (0, area.right_top()),
                (0, area.left_bottom()),
                (0, area.right_bottom()),
                (0, area.left_center()),
                (0, area.right_center()),
                (0, area.center_top()),
                (0, area.center_bottom()),
            ] {
                let hot = hover.is_some_and(|h| h.distance(gp) < 9.0);
                p.rect_filled(
                    Rect::from_center_size(gp, Vec2::splat(if hot { 10.0 } else { 8.0 })),
                    1.5,
                    if hot { theme::TEXT } else { theme::TEXT_DIM },
                );
            }
        }
        p.text(
            rect.left_bottom() + Vec2::new(4.0, -4.0),
            egui::Align2::LEFT_BOTTOM,
            "PAN",
            theme::caption(),
            theme::TEXT_FAINT,
        );
        p.text(
            rect.right_top() + Vec2::new(-4.0, 4.0),
            egui::Align2::RIGHT_TOP,
            "TILT",
            theme::caption(),
            theme::TEXT_FAINT,
        );
        changed
    }
}
