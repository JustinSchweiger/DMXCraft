//! Vertikaler Fader, der die ihm gegebene Höhe vollständig nutzt.
//!
//! Bedienung: Ziehen/Klicken = setzen, Mausrad = ±1 (Shift ±10),
//! Doppelklick = 0 ↔ voll, Rechtsklick = Menü (Freigeben, 0, Voll).

use crate::i18n::t;
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const LABEL_H: f32 = 18.0;
const VALUE_H: f32 = 20.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaderEvent {
    None,
    Changed(u8),
    Release,
}

pub struct Fader<'a> {
    value: u8,
    label: &'a str,
    hint: Option<String>,
    active: bool,
    color: Color32,
    width: f32,
    height: Option<f32>,
}

impl<'a> Fader<'a> {
    pub fn new(value: u8, label: &'a str) -> Self {
        Self {
            value,
            label,
            hint: None,
            active: false,
            color: theme::accent(),
            width: 56.0,
            height: None,
        }
    }

    /// Wert liegt im Programmer (hervorgehoben).
    pub fn active(mut self, on: bool) -> Self {
        self.active = on;
        self
    }

    /// Pegelfarbe (z. B. Rot für einen Rot-Kanal).
    pub fn color(mut self, c: Color32) -> Self {
        self.color = c;
        self
    }

    /// Gesamthöhe; Standard: die gesamte verfügbare Höhe.
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn hint(mut self, text: impl Into<String>) -> Self {
        self.hint = Some(text.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> FaderEvent {
        let h = self
            .height
            .unwrap_or_else(|| ui.available_height())
            .max(LABEL_H + VALUE_H + 60.0);
        let (outer, _) = ui.allocate_exact_size(Vec2::new(self.width, h), Sense::hover());
        let track = Rect::from_min_max(
            pos2(outer.left() + 6.0, outer.top() + LABEL_H),
            pos2(outer.right() - 6.0, outer.bottom() - VALUE_H),
        );
        let id = ui
            .scope_id()
            .with(("fader", self.label, outer.min.x as i32));
        let resp = ui.interact(track, id, Sense::click_and_drag());
        let mut event = FaderEvent::None;
        let mut value = self.value;

        // Eingaben
        if resp.double_clicked() {
            value = if value > 0 { 0 } else { 255 };
            event = FaderEvent::Changed(value);
        } else if resp.dragged() || resp.clicked() {
            if let Some(p) = resp.interact_pointer_pos() {
                let t = 1.0 - ((p.y - track.top()) / track.height()).clamp(0.0, 1.0);
                let v = (t * 255.0).round() as u8;
                if v != value || !self.active {
                    value = v;
                    event = FaderEvent::Changed(v);
                }
            }
        }
        if resp.hovered() {
            let (scroll, shift) = ui.input(|i| (i.smooth_scroll_delta.y, i.modifiers.shift));
            if scroll.abs() >= 1.0 {
                let step = if shift { 10 } else { 1 };
                value = if scroll > 0.0 {
                    value.saturating_add(step)
                } else {
                    value.saturating_sub(step)
                };
                event = FaderEvent::Changed(value);
            }
        }
        crate::ui::widgets::menu::context(&resp, |ui| {
            use crate::ui::widgets::menu::{self, MenuItem};
            menu::begin(ui);
            if menu::item(ui, MenuItem::new(t("Freigeben")).icon(super::Icon::Reset)) {
                event = FaderEvent::Release;
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Aus (0)")).icon(super::Icon::LightOff)) {
                event = FaderEvent::Changed(0);
            }
            if menu::item(
                ui,
                MenuItem::new(t("Voll (255)")).icon(super::Icon::LightOn),
            ) {
                event = FaderEvent::Changed(255);
            }
        });

        // Zeichnen
        let p = ui.painter();
        let r = theme::R_M as f32;
        p.rect_filled(track, r, theme::bg());
        let level_color = if self.active {
            self.color
        } else {
            self.color.gamma_multiply(0.35)
        };
        let t = f32::from(value) / 255.0;
        if t > 0.0 {
            let fill = Rect::from_min_max(
                pos2(track.left(), track.bottom() - track.height() * t),
                track.max,
            );
            p.rect_filled(fill, r, level_color.gamma_multiply(0.55));
            // Kante am Pegel
            let y = fill.top();
            p.line_segment(
                [pos2(track.left() + 3.0, y), pos2(track.right() - 3.0, y)],
                Stroke::new(2.0, level_color),
            );
        }
        let border = if resp.hovered() || resp.dragged() {
            theme::accent_dim()
        } else {
            theme::border()
        };
        p.rect_stroke(track, r, Stroke::new(1.0, border), StrokeKind::Inside);

        let label_color = if self.active {
            theme::accent()
        } else {
            theme::text_dim()
        };
        p.text(
            pos2(outer.center().x, outer.top() + LABEL_H / 2.0),
            egui::Align2::CENTER_CENTER,
            truncate(self.label, 9),
            theme::caption(),
            label_color,
        );
        let pct = (t * 100.0).round() as u32;
        p.text(
            pos2(outer.center().x, outer.bottom() - VALUE_H / 2.0),
            egui::Align2::CENTER_CENTER,
            format!("{value} · {pct}%"),
            theme::caption(),
            if self.active {
                theme::text()
            } else {
                theme::text_faint()
            },
        );
        if let Some(h) = self.hint {
            resp.on_hover_text(h);
        }
        event
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}.", s.chars().take(max - 1).collect::<String>())
    }
}
