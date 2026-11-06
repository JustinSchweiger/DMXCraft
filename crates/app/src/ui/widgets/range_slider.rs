//! Vertikaler Regler mit zwei Griffen (Untergrenze und Obergrenze).

use eframe::egui::{self, pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::ui::theme;

pub struct RangeSlider<'a> {
    pub min: &'a mut u8,
    pub max: &'a mut u8,
    pub height: f32,
    pub enabled: bool,
}

impl RangeSlider<'_> {
    /// Gibt `true` zurück, wenn ein Wert geändert wurde.
    pub fn show(self, ui: &mut Ui) -> bool {
        let w = 64.0;
        let (rect, _) = ui.allocate_exact_size(Vec2::new(w, self.height), Sense::hover());
        let track = Rect::from_center_size(
            pos2(rect.center().x, rect.center().y),
            Vec2::new(6.0, rect.height() - 40.0),
        );
        let y_of = |v: u8| track.bottom() - f32::from(v) / 255.0 * track.height();
        let v_of = |y: f32| {
            (((track.bottom() - y) / track.height()).clamp(0.0, 1.0) * 255.0).round() as u8
        };
        let mut changed = false;
        let handle =
            |v: u8| Rect::from_center_size(pos2(track.center().x, y_of(v)), Vec2::new(22.0, 12.0));
        let sense = if self.enabled {
            Sense::drag()
        } else {
            Sense::hover()
        };
        let id = ui.scope_id().with("range_slider");
        let r_max = ui.interact(handle(*self.max).expand(3.0), id.with("max"), sense);
        let r_min = ui.interact(handle(*self.min).expand(3.0), id.with("min"), sense);
        if r_max.dragged() {
            if let Some(p) = r_max.interact_pointer_pos() {
                *self.max = v_of(p.y).max(*self.min);
                changed = true;
            }
        }
        if r_min.dragged() {
            if let Some(p) = r_min.interact_pointer_pos() {
                *self.min = v_of(p.y).min(*self.max);
                changed = true;
            }
        }
        let p = ui.painter();
        p.rect_filled(track, 3.0, theme::bg());
        let active = Rect::from_min_max(
            pos2(track.left(), y_of(*self.max)),
            pos2(track.right(), y_of(*self.min)),
        );
        let c = if self.enabled {
            theme::accent()
        } else {
            theme::text_faint()
        };
        p.rect_filled(active, 3.0, c.gamma_multiply(0.6));
        for (v, hovered) in [
            (*self.max, r_max.hovered() || r_max.dragged()),
            (*self.min, r_min.hovered() || r_min.dragged()),
        ] {
            let h = handle(v);
            p.rect_filled(
                h,
                3.0,
                if hovered {
                    theme::text()
                } else {
                    theme::text_dim()
                },
            );
            p.rect_stroke(
                h,
                3.0,
                Stroke::new(1.0, theme::bg()),
                egui::StrokeKind::Inside,
            );
        }
        let pct = |v: u8| format!("{:.0}%", f32::from(v) / 255.0 * 100.0);
        p.text(
            pos2(rect.center().x, rect.top() + 8.0),
            egui::Align2::CENTER_CENTER,
            pct(*self.max),
            theme::caption(),
            theme::text(),
        );
        p.text(
            pos2(rect.center().x, rect.bottom() - 8.0),
            egui::Align2::CENTER_CENTER,
            pct(*self.min),
            theme::caption(),
            theme::text(),
        );
        changed
    }
}
