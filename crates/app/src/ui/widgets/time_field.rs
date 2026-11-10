//! Time field in the format `00h00m00s00` or without hours `00m00s00`
//! (minutes, seconds, hundredths). Each number can be changed by clicking
//! and dragging up/down; carries (100 hundredths = 1 s, 60 s = 1 min,
//! 60 min = 1 h) happen automatically because everything is calculated in
//! milliseconds internally.

use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

/// Largest value: 99:59:59.96.
pub const MAX_MS: u64 = 99 * 3_600_000 + 59 * 60_000 + 59_960;
/// Points of mouse travel per step.
const PX_PER_STEP: f32 = 3.0;
/// Hundredths change in steps of 4 (40 ms).
const FINE_STEP_MS: u64 = 40;

/// Step size per field: hours, minutes, seconds, hundredths.
const STEPS_MS: [u64; 4] = [3_600_000, 60_000, 1_000, FINE_STEP_MS];

/// Splits milliseconds into hours, minutes, seconds and hundredths.
pub fn split(ms: u64) -> [u64; 4] {
    [
        ms / 3_600_000,
        ms / 60_000 % 60,
        ms / 1_000 % 60,
        ms % 1_000 / 10,
    ]
}

/// Text as in the field, e.g. `01m01s24`; with `hours` also `00h…`.
pub fn format(ms: u64, hours: bool) -> String {
    let [h, m, s, cs] = split(ms);
    if hours {
        format!("{h:02}h{m:02}m{s:02}s{cs:02}")
    } else {
        // without an hours field the minutes keep counting
        format!("{:02}m{s:02}s{cs:02}", h * 60 + m)
    }
}

/// Draws the field; `true` if the value changed.
/// `hours` = show the hours field, `accent` = color of the number under the
/// mouse.
pub fn time_field(
    ui: &mut Ui,
    id_salt: &str,
    ms: &mut u32,
    enabled: bool,
    hours: bool,
    accent: Color32,
) -> bool {
    time_field_ex(ui, id_salt, ms, enabled, hours, accent, false)
}

/// Compact variant without a frame (for tables), width [`COMPACT_W`].
pub fn time_field_compact(
    ui: &mut Ui,
    id_salt: &str,
    ms: &mut u32,
    enabled: bool,
    accent: Color32,
) -> bool {
    time_field_ex(ui, id_salt, ms, enabled, false, accent, true)
}

/// Width of the compact variant (without hours).
pub const COMPACT_W: f32 = 3.0 * 17.0 + 2.0 * 9.0;

fn time_field_ex(
    ui: &mut Ui,
    id_salt: &str,
    ms: &mut u32,
    enabled: bool,
    hours: bool,
    accent: Color32,
    compact: bool,
) -> bool {
    let digits = egui::FontId::proportional(if compact {
        theme::FS_BODY
    } else {
        theme::FS_BODY + 2.0
    });
    let units_font = egui::FontId::proportional(theme::FS_CAPTION);
    let (pair_w, unit_w, pad, height) = if compact {
        (17.0, 9.0, 0.0, 22.0)
    } else {
        (22.0, 12.0, 8.0, 28.0)
    };
    let fields: &[usize] = if hours { &[0, 1, 2, 3] } else { &[1, 2, 3] };
    let w = pad * 2.0 + fields.len() as f32 * pair_w + (fields.len() - 1) as f32 * unit_w;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, height), Sense::hover());
    let id = ui.make_persistent_id(("time_field", id_salt));
    let p = ui.painter().clone();
    if !compact {
        p.rect_filled(rect, theme::R_S, theme::bg());
        p.rect_stroke(
            rect,
            theme::R_S,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );
    }
    let inset = if compact { 1.0 } else { 3.0 };

    let mut changed = false;
    let mut parts = split(u64::from(*ms));
    if !hours {
        parts[1] += parts[0] * 60;
    }
    let mut x = rect.left() + pad;
    for (n, &i) in fields.iter().enumerate() {
        let cell = Rect::from_min_size(
            pos2(x, rect.top() + inset),
            Vec2::new(pair_w, rect.height() - inset * 2.0),
        );
        x += pair_w;
        let sense = if enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let resp = ui.interact(cell, id.with(i), sense);
        let hot = enabled && (resp.hovered() || resp.dragged());
        if hot {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
            p.rect_filled(cell, 3.0, theme::raised_hover());
        }
        let delta = crate::ui::widgets::drag::endless_delta(ui, &resp);
        if resp.dragged() {
            let acc_id = id.with(("acc", i));
            let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
            acc -= delta.y / PX_PER_STEP;
            let steps = acc.trunc();
            acc -= steps;
            ui.data_mut(|d| d.insert_temp(acc_id, acc));
            if steps != 0.0 {
                let delta = steps as i64 * STEPS_MS[i] as i64;
                let mut v = (i64::from(*ms) + delta).clamp(0, MAX_MS as i64) as u64;
                // Hundredths stay in the grid of 4
                v -= v % FINE_STEP_MS;
                if v as u32 != *ms {
                    *ms = v as u32;
                    changed = true;
                }
            }
        }
        if resp.drag_stopped() {
            ui.data_mut(|d| d.remove::<f32>(id.with(("acc", i))));
        }
        p.text(
            cell.center(),
            egui::Align2::CENTER_CENTER,
            format!("{:02}", parts[i]),
            digits.clone(),
            if !enabled {
                theme::text_faint()
            } else if hot {
                accent
            } else {
                theme::text()
            },
        );
        if n + 1 < fields.len() {
            let unit = ["h", "m", "s"][i];
            p.text(
                pos2(x + unit_w * 0.5, rect.center().y + 1.0),
                egui::Align2::CENTER_CENTER,
                unit,
                units_font.clone(),
                theme::text_faint(),
            );
            x += unit_w;
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_and_format_carry_over() {
        assert_eq!(format(0, true), "00h00m00s00");
        assert_eq!(format(1_000, true), "00h00m01s00");
        assert_eq!(format(61_240, false), "01m01s24");
        assert_eq!(format(3_600_000 + 120_000, true), "01h02m00s00");
        assert_eq!(format(3_600_000, false), "60m00s00");
        assert_eq!(split(999), [0, 0, 0, 99]);
    }
}
