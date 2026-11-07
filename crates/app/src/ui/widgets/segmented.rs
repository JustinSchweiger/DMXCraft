//! Segment-Umschalter (horizontal oder vertikal).

use eframe::egui::{self, Color32, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

pub struct Segmented<'a, T: PartialEq + Copy> {
    items: &'a [(T, &'a str)],
    vertical: bool,
    item_width: Option<f32>,
    dimmed: &'a dyn Fn(T) -> bool,
}

impl<'a, T: PartialEq + Copy> Segmented<'a, T> {
    pub fn new(items: &'a [(T, &'a str)]) -> Self {
        Self {
            items,
            vertical: false,
            item_width: None,
            dimmed: &|_| false,
        }
    }
    #[allow(dead_code)]
    pub fn vertical(mut self, w: f32) -> Self {
        self.vertical = true;
        self.item_width = Some(w);
        self
    }
    /// Einträge, die gedämpft angezeigt werden (z. B. leere Kategorien).
    #[allow(dead_code)]
    pub fn dimmed(mut self, f: &'a dyn Fn(T) -> bool) -> Self {
        self.dimmed = f;
        self
    }

    /// Zeichnet den Umschalter; gibt `true` zurück, wenn sich `value` geändert hat.
    pub fn show(self, ui: &mut Ui, value: &mut T) -> bool {
        let mut changed = false;
        let mut body = |ui: &mut Ui| {
            for (item, label) in self.items {
                let selected = *value == *item;
                let galley =
                    ui.painter()
                        .layout_no_wrap(label.to_string(), theme::body(), Color32::WHITE);
                let w = self
                    .item_width
                    .unwrap_or(galley.size().x + 2.0 * theme::SP_M);
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 28.0), Sense::click());
                let p = ui.painter();
                let fill = if selected {
                    theme::accent_dim()
                } else if resp.hovered() {
                    theme::raised_hover()
                } else {
                    Color32::TRANSPARENT
                };
                p.rect_filled(rect, theme::R_S as f32, fill);
                if selected {
                    p.rect_stroke(
                        rect,
                        theme::R_S as f32,
                        Stroke::new(1.0, theme::accent()),
                        StrokeKind::Inside,
                    );
                }
                let color = if selected {
                    theme::accent()
                } else if (self.dimmed)(*item) {
                    theme::text_faint()
                } else {
                    theme::text_dim()
                };
                let pos = if self.vertical {
                    egui::pos2(rect.left() + theme::SP_M, rect.center().y)
                } else {
                    rect.center()
                };
                let align = if self.vertical {
                    egui::Align2::LEFT_CENTER
                } else {
                    egui::Align2::CENTER_CENTER
                };
                p.text(pos, align, *label, theme::body(), color);
                if resp.clicked() && !selected {
                    *value = *item;
                    changed = true;
                }
            }
        };
        if self.vertical {
            ui.vertical(|ui| body(ui));
        } else {
            ui.horizontal(|ui| body(ui));
        }
        changed
    }
}
