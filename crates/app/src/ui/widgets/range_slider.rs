//! Vertical slider with two handles (lower and upper bound), looking like a
//! programmer fader – just without the status dot. The upper bound is at the
//! top, the lower bound at the bottom; both values can be dragged (up/down)
//! like on the fader or typed in with a click.

use eframe::egui::{self, pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const HANDLE: Vec2 = Vec2::new(24.0, 30.0);
const VALUE_H: f32 = 24.0;

pub struct RangeSlider<'a> {
    pub min: &'a mut u8,
    pub max: &'a mut u8,
    pub height: f32,
    pub enabled: bool,
}

impl RangeSlider<'_> {
    /// Returns `true` if a value was changed.
    pub fn show(self, ui: &mut Ui) -> bool {
        let w = super::fader::FADER_W;
        let (outer, _) = ui.allocate_exact_size(Vec2::new(w, self.height), Sense::hover());
        let id = ui.make_persistent_id(("range_slider", outer.min.x as i32));
        let p = ui.painter().clone();
        p.rect_filled(outer, 0.0, theme::raised());

        let mut max = *self.max;
        let mut min = *self.min;

        // Values at the top (upper bound) and bottom (lower bound)
        let top = Rect::from_min_size(
            pos2(outer.left() + 3.0, outer.top() + 2.0),
            Vec2::new(w - 6.0, VALUE_H - 4.0),
        );
        let bottom = Rect::from_min_size(
            pos2(outer.left() + 3.0, outer.bottom() - VALUE_H + 2.0),
            Vec2::new(w - 6.0, VALUE_H - 4.0),
        );
        if let Some(v) = value_field(ui, &p, top, id.with("max"), max, self.enabled) {
            max = v.max(min);
        }
        if let Some(v) = value_field(ui, &p, bottom, id.with("min"), min, self.enabled) {
            min = v.min(max);
        }

        // Track between the values
        let track_area = Rect::from_min_max(
            pos2(outer.left(), outer.top() + VALUE_H),
            pos2(outer.right(), outer.bottom() - VALUE_H),
        );
        let travel = track_area.shrink2(Vec2::new(0.0, HANDLE.y * 0.5 + 2.0));
        let y_of = |v: u8| travel.bottom() - f32::from(v) / 255.0 * travel.height();
        let v_of = |y: f32| {
            (((travel.bottom() - y) / travel.height()).clamp(0.0, 1.0) * 255.0).round() as u8
        };
        let handle = |v: u8| Rect::from_center_size(pos2(travel.center().x, y_of(v)), HANDLE);
        let sense = if self.enabled {
            Sense::drag()
        } else {
            Sense::hover()
        };
        let r_max = ui.interact(handle(max), id.with("h_max"), sense);
        let r_min = ui.interact(handle(min), id.with("h_min"), sense);
        if r_max.dragged() {
            if let Some(pos) = r_max.interact_pointer_pos() {
                max = v_of(pos.y).max(min);
            }
        }
        if r_min.dragged() {
            if let Some(pos) = r_min.interact_pointer_pos() {
                min = v_of(pos.y).min(max);
            }
        }

        // Slot, with the allowed range in between
        let slot = Rect::from_center_size(
            pos2(travel.center().x, track_area.center().y),
            Vec2::new(6.0, track_area.height() - 6.0),
        );
        p.rect_filled(slot, 3.0, theme::bg());
        let range = Rect::from_min_max(pos2(slot.left(), y_of(max)), pos2(slot.right(), y_of(min)));
        let c = if self.enabled {
            theme::accent()
        } else {
            theme::text_faint()
        };
        p.rect_filled(range, 3.0, c.gamma_multiply(0.7));

        // Handles like on the fader
        for (v, r) in [(max, &r_max), (min, &r_min)] {
            let h = handle(v);
            let hot = r.hovered() || r.dragged();
            let (fill, edge, line) = if self.enabled {
                (
                    theme::raised_hover(),
                    if hot {
                        theme::text()
                    } else {
                        theme::text_dim()
                    },
                    theme::text(),
                )
            } else {
                (theme::raised(), theme::border(), theme::text_faint())
            };
            p.rect_filled(h, theme::R_S, fill);
            p.rect_stroke(h, theme::R_S, Stroke::new(1.5, edge), StrokeKind::Inside);
            p.line_segment(
                [
                    pos2(h.left() + 7.0, h.center().y),
                    pos2(h.right() - 7.0, h.center().y),
                ],
                Stroke::new(2.5, line),
            );
        }

        let changed = max != *self.max || min != *self.min;
        *self.max = max;
        *self.min = min;
        changed
    }
}

fn pct(v: u8) -> u8 {
    (f32::from(v) / 255.0 * 100.0).round() as u8
}

fn from_pct(p: u8) -> u8 {
    (f32::from(p.min(100)) / 100.0 * 255.0).round() as u8
}

/// Value field like on the fader: dragging up/down changes it (1 % per
/// 3 points), a click opens input in percent.
fn value_field(
    ui: &mut Ui,
    p: &egui::Painter,
    rect: Rect,
    id: egui::Id,
    value: u8,
    enabled: bool,
) -> Option<u8> {
    let edit_id = id.with("edit");
    let editing: Option<String> = ui.data(|d| d.get_temp(edit_id));
    if let Some(mut text) = editing.filter(|_| enabled) {
        let r = ui.put(
            rect,
            egui::TextEdit::singleline(&mut text)
                .font(theme::caption())
                .align(egui::Align2::CENTER_CENTER)
                .margin(egui::Margin::symmetric(2, 1)),
        );
        if !r.has_focus() && !r.lost_focus() {
            r.request_focus();
            crate::ui::widgets::dialog::select_all(ui.ctx(), r.id, &text);
        }
        let (enter, esc) = ui.input(|i| {
            (
                i.key_pressed(egui::Key::Enter),
                i.key_pressed(egui::Key::Escape),
            )
        });
        if esc {
            ui.data_mut(|d| d.remove::<String>(edit_id));
        } else if enter || r.lost_focus() {
            ui.data_mut(|d| d.remove::<String>(edit_id));
            let t = text.trim().trim_end_matches('%').trim();
            if let Ok(v) = t.parse::<u16>() {
                return Some(from_pct(v.min(100) as u8));
            }
        } else {
            ui.data_mut(|d| d.insert_temp(edit_id, text));
        }
        return None;
    }
    let sense = if enabled {
        Sense::click_and_drag()
    } else {
        Sense::hover()
    };
    let r = ui.interact(rect, id.with("value"), sense);
    let mut out = None;
    if enabled {
        if r.hovered() || r.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
        }
        let acc_id = id.with("drag");
        if r.drag_started() {
            ui.data_mut(|d| d.insert_temp(acc_id, 0.0f32));
        }
        let delta = crate::ui::widgets::drag::endless_delta(ui, &r);
        if r.dragged() {
            let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
            acc -= delta.y / 3.0;
            let steps = acc.trunc();
            if steps != 0.0 {
                acc -= steps;
                let np = (f32::from(pct(value)) + steps).clamp(0.0, 100.0) as u8;
                out = Some(from_pct(np));
            }
            ui.data_mut(|d| d.insert_temp(acc_id, acc));
        }
        if r.hovered() || r.dragged() {
            p.rect_filled(rect, theme::R_S, theme::raised_hover());
        }
        if r.clicked() {
            ui.data_mut(|d| d.insert_temp(edit_id, pct(value).to_string()));
        }
    }
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        format!("{} %", pct(out.unwrap_or(value))),
        theme::caption(),
        if enabled {
            theme::text()
        } else {
            theme::text_faint()
        },
    );
    out
}
