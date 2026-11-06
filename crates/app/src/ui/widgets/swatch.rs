use eframe::egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget};

use crate::ui::theme;

/// Klickbares Farbfeld.
pub struct Swatch(pub Color32);

impl Widget for Swatch {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(26.0), Sense::click());
        let p = ui.painter();
        p.rect_filled(rect, theme::R_S as f32, self.0);
        let stroke = if resp.hovered() {
            Stroke::new(2.0, Color32::WHITE)
        } else {
            Stroke::new(1.0, theme::BORDER)
        };
        p.rect_stroke(rect, theme::R_S as f32, stroke, StrokeKind::Outside);
        resp
    }
}
