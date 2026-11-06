//! Schiebeschalter – ersetzt Checkboxen in der ganzen Oberfläche.

use eframe::egui::{self, pos2, Response, Sense, Ui, Vec2, Widget};

use crate::ui::theme;

pub struct Toggle<'a> {
    value: &'a mut bool,
    label: &'a str,
    enabled: bool,
}

impl<'a> Toggle<'a> {
    pub fn new(value: &'a mut bool, label: &'a str) -> Self {
        Self {
            value,
            label,
            enabled: true,
        }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
}

impl Widget for Toggle<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let galley =
            ui.painter()
                .layout_no_wrap(self.label.to_string(), theme::body(), theme::text());
        let track = Vec2::new(30.0, 16.0);
        let gap = if self.label.is_empty() {
            0.0
        } else {
            theme::SP_S
        };
        let size = Vec2::new(track.x + gap + galley.size().x, 22.0);
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, mut resp) = ui.allocate_exact_size(size, sense);
        if resp.clicked() {
            *self.value = !*self.value;
            resp.mark_changed();
        }
        let t = ui.ctx().animate_bool(resp.id, *self.value);
        let p = ui.painter();
        let tr =
            egui::Rect::from_min_size(pos2(rect.left(), rect.center().y - track.y / 2.0), track);
        let on_color = if self.enabled {
            theme::accent_dim()
        } else {
            theme::raised()
        };
        let bg = if t > 0.5 {
            on_color
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::raised()
        };
        p.rect_filled(tr, track.y / 2.0, bg);
        p.rect_stroke(
            tr,
            track.y / 2.0,
            egui::Stroke::new(1.0, theme::border()),
            egui::StrokeKind::Inside,
        );
        let x = tr.left() + track.y / 2.0 + t * (track.x - track.y);
        let knob = if !self.enabled {
            theme::text_faint()
        } else if *self.value {
            theme::accent()
        } else {
            theme::text_dim()
        };
        p.circle_filled(pos2(x, tr.center().y), track.y / 2.0 - 2.5, knob);
        if !self.label.is_empty() {
            p.galley(
                pos2(tr.right() + gap, rect.center().y - galley.size().y / 2.0),
                galley,
                if self.enabled {
                    theme::text()
                } else {
                    theme::text_faint()
                },
            );
        }
        resp
    }
}
