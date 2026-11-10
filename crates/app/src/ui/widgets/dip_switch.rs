//! Display of a 10-way DIP switch bank for a DMX address.

use dmxcraft_core::dip;
use eframe::egui::{self, pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

pub struct DipSwitch {
    pub address: u16,
    /// Small rendering for header rows.
    pub compact: bool,
}

impl DipSwitch {
    pub fn show(self, ui: &mut Ui) -> egui::Response {
        if self.compact {
            return self.show_compact(ui);
        }
        let sw = dip::switches(self.address);
        let k = 1.0;
        let (w, h) = (22.0 * k, 46.0 * k);
        let size = Vec2::new(w * dip::SWITCHES as f32 + 12.0 * k, h + 34.0 * k);
        let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
        let p = ui.painter();
        let r = theme::R_M as f32;
        p.rect_filled(rect, r, theme::DIP_BODY);
        p.rect_stroke(
            rect,
            r,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );
        if !self.compact {
            p.text(
                rect.left_top() + Vec2::new(8.0, 5.0),
                egui::Align2::LEFT_TOP,
                "ON",
                theme::caption(),
                theme::text(),
            );
        }
        for (i, on) in sw.iter().enumerate() {
            let x = rect.left() + 6.0 + i as f32 * w;
            let slot = Rect::from_min_size(
                pos2(x + 4.0, rect.top() + 18.0),
                Vec2::new(w - 8.0, h - 8.0),
            );
            p.rect_filled(slot, 3.0, theme::bg());
            let knob_h = slot.height() * 0.45;
            let knob = if *on {
                Rect::from_min_size(
                    slot.min + Vec2::new(1.5, 1.5),
                    Vec2::new(slot.width() - 3.0, knob_h),
                )
            } else {
                Rect::from_min_size(
                    pos2(slot.left() + 1.5, slot.bottom() - knob_h - 1.5),
                    Vec2::new(slot.width() - 3.0, knob_h),
                )
            };
            p.rect_filled(
                knob,
                2.0,
                if *on {
                    theme::accent()
                } else {
                    theme::text_dim()
                },
            );
            p.text(
                pos2(slot.center().x, rect.bottom() - 9.0 * k),
                egui::Align2::CENTER_CENTER,
                (i + 1).to_string(),
                egui::FontId::proportional(if self.compact { 7.5 } else { theme::FS_CAPTION }),
                if *on {
                    theme::accent()
                } else {
                    theme::text_dim()
                },
            );
        }
        let on = dip::on_switches(self.address);
        let text = if on.is_empty() {
            "keiner".to_string()
        } else {
            on.iter()
                .map(|s| format!("{s}"))
                .collect::<Vec<_>>()
                .join(" + ")
        };
        resp.on_hover_text(tr!("Address {address} = switches {text}\n(switch n has the value 2^(n-1); switch 10 stays off)", text = text, address = self.address))
    }

    /// Flat bar with a fixed height of 24 px for header rows.
    fn show_compact(self, ui: &mut Ui) -> egui::Response {
        const H: f32 = 24.0;
        const W: f32 = 11.0;
        let sw = dip::switches(self.address);
        let size = Vec2::new(W * dip::SWITCHES as f32 + 6.0, H);
        let (rect, resp) = ui.allocate_exact_size(size, Sense::hover());
        let p = ui.painter();
        p.rect_filled(rect, 3.0, theme::DIP_BODY);
        for (i, on) in sw.iter().enumerate() {
            let x = rect.left() + 3.0 + i as f32 * W;
            let slot =
                Rect::from_min_size(pos2(x + 2.0, rect.top() + 3.0), Vec2::new(W - 4.0, 12.0));
            p.rect_filled(slot, 1.5, theme::bg());
            let knob = if *on {
                Rect::from_min_size(slot.min, Vec2::new(slot.width(), 6.0))
            } else {
                Rect::from_min_size(
                    pos2(slot.left(), slot.bottom() - 6.0),
                    Vec2::new(slot.width(), 6.0),
                )
            };
            p.rect_filled(
                knob,
                1.0,
                if *on {
                    theme::accent()
                } else {
                    theme::text_dim()
                },
            );
            p.text(
                pos2(slot.center().x, rect.bottom() - 4.5),
                egui::Align2::CENTER_CENTER,
                (i + 1).to_string(),
                egui::FontId::proportional(7.0),
                if *on {
                    theme::accent()
                } else {
                    theme::text_dim()
                },
            );
        }
        resp.on_hover_text(tr!(
            "DIP switches for address {address}",
            address = self.address
        ))
    }
}
