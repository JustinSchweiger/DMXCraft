//! Drehregler: Ziehen nach oben/unten ändert den Wert, Doppelklick setzt
//! ihn auf den Standard zurück. Darunter steht die Bezeichnung; der Wert
//! erscheint nur beim Drehen als Hinweis am Mauszeiger.

use eframe::egui::{self, pos2, Color32, Sense, Shape, Stroke, Ui, Vec2};

use crate::ui::theme;

/// Durchmesser des Reglers.
const SIZE: f32 = 44.0;
/// Drehbereich (ab „7 Uhr“ bis „5 Uhr“).
const SWEEP: f32 = std::f32::consts::PI * 1.5;

pub struct Knob<'a> {
    /// Wert 0..=1.
    value: &'a mut f32,
    label: &'a str,
    text: String,
    default: f32,
    color: Color32,
    enabled: bool,
    /// Zuordenbar über Tastatur/MIDI: (Schlüssel, Kategorie).
    map: Option<(String, String)>,
}

impl<'a> Knob<'a> {
    pub fn new(value: &'a mut f32, label: &'a str, text: impl Into<String>) -> Self {
        Self {
            value,
            label,
            text: text.into(),
            default: 1.0,
            color: theme::accent(),
            enabled: true,
            map: None,
        }
    }

    /// Wert für den Doppelklick.
    pub fn default_value(mut self, v: f32) -> Self {
        self.default = v;
        self
    }

    /// Akzentfarbe (z. B. Farbe der Szene).
    pub fn color(mut self, c: Color32) -> Self {
        self.color = c;
        self
    }

    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }

    /// Über Tastatur/MIDI zuordenbar machen (fester Schlüssel, Kategorie für
    /// die Liste; das Ziel ist die Beschriftung).
    pub fn map(mut self, key: impl Into<String>, category: impl Into<String>) -> Self {
        self.map = Some((key.into(), category.into()));
        self
    }

    /// Zeichnet den Regler; `true`, wenn sich der Wert geändert hat.
    pub fn show(self, ui: &mut Ui) -> bool {
        let w = 64.0;
        let (rect, _) = ui.allocate_exact_size(Vec2::new(w, SIZE + 20.0), Sense::hover());
        let knob = egui::Rect::from_center_size(
            pos2(rect.center().x, rect.top() + SIZE * 0.5 + 2.0),
            Vec2::splat(SIZE),
        );
        let sense = if self.enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let resp = ui.interact(knob, ui.make_persistent_id(("knob", self.label)), sense);
        let mut changed = false;
        let delta = crate::ui::widgets::drag::endless_delta(ui, &resp);
        if resp.dragged() {
            let d = -delta.y / 150.0;
            let fine = ui.input(|i| i.modifiers.shift);
            let v = (*self.value + if fine { d * 0.2 } else { d }).clamp(0.0, 1.0);
            if v != *self.value {
                *self.value = v;
                changed = true;
            }
        }
        if resp.double_clicked() && *self.value != self.default {
            *self.value = self.default;
            changed = true;
        }
        if resp.hovered() && !resp.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
        }
        if let Some((key, cat)) = &self.map {
            let label = self.label;
            if let Some(v) = crate::mapping::fader(ui, rect, key, *self.value, || {
                (cat.clone(), label.to_string())
            }) {
                if self.enabled && v != *self.value {
                    *self.value = v;
                    changed = true;
                }
            }
        }

        let p = ui.painter();
        let c = knob.center();
        let r = SIZE * 0.5;
        let start = std::f32::consts::PI * 0.75; // 7 Uhr
        let arc = |from: f32, to: f32, radius: f32| -> Vec<egui::Pos2> {
            let n = 32;
            (0..=n)
                .map(|i| {
                    let a = from + (to - from) * i as f32 / n as f32;
                    c + Vec2::angled(a) * radius
                })
                .collect()
        };
        let (track, fill) = if self.enabled {
            (theme::bg(), self.color)
        } else {
            (theme::bg(), theme::text_faint())
        };
        p.add(Shape::line(
            arc(start, start + SWEEP, r - 3.0),
            Stroke::new(4.0, track),
        ));
        let v = *self.value;
        if v > 0.001 {
            p.add(Shape::line(
                arc(start, start + SWEEP * v, r - 3.0),
                Stroke::new(4.0, fill),
            ));
        }
        let body = if resp.hovered() || resp.dragged() {
            theme::raised_hover()
        } else {
            theme::raised()
        };
        p.circle_filled(c, r - 8.0, body);
        p.circle_stroke(c, r - 8.0, Stroke::new(1.0, theme::border()));
        let a = start + SWEEP * v;
        p.line_segment(
            [
                c + Vec2::angled(a) * (r - 18.0),
                c + Vec2::angled(a) * (r - 10.0),
            ],
            Stroke::new(2.0, theme::text()),
        );
        p.text(
            pos2(rect.center().x, knob.bottom() + 8.0),
            egui::Align2::CENTER_CENTER,
            self.label,
            theme::caption(),
            theme::text_dim(),
        );
        if resp.dragged() {
            if let Some(pos) = ui.ctx().pointer_interact_pos() {
                egui::Area::new(egui::Id::unique(("knob_value", self.label)))
                    .order(egui::Order::Tooltip)
                    .fixed_pos(pos + Vec2::new(14.0, -10.0))
                    .interactable(false)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::new()
                            .fill(theme::raised())
                            .stroke(Stroke::new(1.0, self.color))
                            .corner_radius(theme::R_S)
                            .inner_margin(egui::Margin::symmetric(8, 4))
                            .show(ui, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(&self.text).color(theme::text()),
                                    )
                                    .wrap_mode(egui::TextWrapMode::Extend),
                                );
                            });
                    });
            }
        }
        changed
    }
}
