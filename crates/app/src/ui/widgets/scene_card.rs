//! Szenenkarte für das Szenen-Raster.

use eframe::egui::{
    self, pos2, Color32, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, Widget,
};

use crate::ui::theme;

pub struct SceneCard<'a> {
    pub name: &'a str,
    pub kind: String,
    pub color: Color32,
    pub active: bool,
    pub paused: bool,
    /// Gesamtfortschritt 0..=1 (nur wenn aktiv).
    pub progress: f32,
    pub width: f32,
    /// Für Bearbeitungen markiert (Strg/Umschalt+Klick, „Alle Szenen auswählen“).
    pub selected: bool,
}

impl Widget for SceneCard<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(self.width, 50.0), Sense::click());
        let p = ui.painter();
        let r = theme::R_M as f32;
        let fill = match (self.active, resp.hovered()) {
            (true, _) => self.color.gamma_multiply(0.16),
            (false, true) => theme::raised_hover(),
            (false, false) => theme::raised(),
        };
        p.rect_filled(rect, r, fill);
        let stroke = if self.active {
            Stroke::new(1.5, self.color)
        } else {
            Stroke::new(1.0, theme::border())
        };
        p.rect_stroke(rect, r, stroke, StrokeKind::Inside);
        if self.selected {
            p.rect_stroke(
                rect.expand(2.0),
                r + 2.0,
                Stroke::new(2.0, theme::accent()),
                StrokeKind::Inside,
            );
        }
        // Farbmarke links
        let mark = Rect::from_min_size(
            rect.min + Vec2::new(0.0, 8.0),
            Vec2::new(3.0, rect.height() - 16.0),
        );
        p.rect_filled(
            mark,
            1.5,
            if self.active {
                self.color
            } else {
                self.color.gamma_multiply(0.4)
            },
        );

        let inner = rect.shrink2(Vec2::new(12.0, 8.0));
        p.text(
            inner.left_top(),
            egui::Align2::LEFT_TOP,
            clip(self.name, ((self.width - 24.0) / 7.2) as usize),
            theme::body(),
            if self.active {
                theme::text()
            } else {
                theme::text().gamma_multiply(0.9)
            },
        );
        let kind = if self.paused {
            tr!("{kind} · PAUSE", kind = self.kind)
        } else {
            self.kind
        };
        p.text(
            inner.left_bottom() - Vec2::new(0.0, 3.0),
            egui::Align2::LEFT_BOTTOM,
            kind,
            theme::caption(),
            theme::text_dim(),
        );
        // Fortschritt unten
        if self.active {
            let bar = Rect::from_min_max(
                pos2(rect.left() + 10.0, rect.bottom() - 4.0),
                pos2(rect.right() - 10.0, rect.bottom() - 2.0),
            );
            p.rect_filled(bar, 1.0, theme::bg());
            let w = bar.width() * self.progress.clamp(0.0, 1.0);
            let done = Rect::from_min_size(bar.min, Vec2::new(w, bar.height()));
            p.rect_filled(
                done,
                1.0,
                if self.paused {
                    theme::text_dim()
                } else {
                    self.color
                },
            );
        }
        resp
    }
}

fn clip(s: &str, max: usize) -> String {
    let max = max.max(4);
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}
