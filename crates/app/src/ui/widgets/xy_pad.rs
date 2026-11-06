//! Zweiachsiges Feld für Pan/Tilt.

use eframe::egui::{self, pos2, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

pub struct XyPad {
    pub x: u8,
    pub y: u8,
    pub size: f32,
}

impl XyPad {
    /// Gibt die neuen Werte zurück, wenn der Benutzer sie geändert hat.
    pub fn show(self, ui: &mut Ui) -> Option<(u8, u8)> {
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::click_and_drag());
        let p = ui.painter();
        let r = theme::R_M as f32;
        p.rect_filled(rect, r, theme::bg());
        p.rect_stroke(
            rect,
            r,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );
        let faint = Stroke::new(1.0, theme::raised());
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
        let dot = pos2(
            rect.left() + f32::from(self.x) / 255.0 * rect.width(),
            rect.top() + f32::from(self.y) / 255.0 * rect.height(),
        );
        p.line_segment(
            [pos2(dot.x, rect.top()), pos2(dot.x, rect.bottom())],
            Stroke::new(1.0, theme::accent_dim()),
        );
        p.line_segment(
            [pos2(rect.left(), dot.y), pos2(rect.right(), dot.y)],
            Stroke::new(1.0, theme::accent_dim()),
        );
        p.circle_filled(dot, 7.0, theme::accent());
        p.text(
            rect.left_top() + Vec2::new(8.0, 6.0),
            egui::Align2::LEFT_TOP,
            tr!("PAN {x} · TILT {y}", x = self.x, y = self.y),
            theme::caption(),
            theme::text_dim(),
        );
        if resp.dragged() || resp.clicked() {
            let pos = resp.interact_pointer_pos()?;
            let nx = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            let ny = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
            return Some(((nx * 255.0).round() as u8, (ny * 255.0).round() as u8));
        }
        None
    }
}
