//! Zeitfeld im Format `00h00m00s00` (Stunden, Minuten, Sekunden,
//! Hundertstel). Jede Zahl lässt sich per Klicken und Ziehen nach oben/unten
//! ändern; Überträge (100 Hundertstel = 1 s, 60 s = 1 min, 60 min = 1 h)
//! ergeben sich von selbst, weil intern in Millisekunden gerechnet wird.

use eframe::egui::{self, pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

/// Größter Wert: 99:59:59.99.
pub const MAX_MS: u64 = 99 * 3_600_000 + 59 * 60_000 + 59_990;
/// Punkte Mausweg je Schritt.
const PX_PER_STEP: f32 = 3.0;

/// Schrittweite je Feld: Stunden, Minuten, Sekunden, Hundertstel (4 ms).
const STEPS_MS: [u64; 4] = [3_600_000, 60_000, 1_000, 4];

/// Zerlegt Millisekunden in Stunden, Minuten, Sekunden und Hundertstel.
pub fn split(ms: u64) -> [u64; 4] {
    [
        ms / 3_600_000,
        ms / 60_000 % 60,
        ms / 1_000 % 60,
        ms % 1_000 / 10,
    ]
}

/// Text wie im Feld, z. B. `00h01m01s25` (für Hinweise und Tests).
#[allow(dead_code)]
pub fn format(ms: u64) -> String {
    let [h, m, s, cs] = split(ms);
    format!("{h:02}h{m:02}m{s:02}s{cs:02}")
}

/// Zeichnet das Feld; `true`, wenn sich der Wert geändert hat.
pub fn time_field(ui: &mut Ui, id_salt: &str, ms: &mut u32, enabled: bool) -> bool {
    let font = egui::FontId::monospace(theme::FS_BODY);
    let digit_w = ui
        .painter()
        .layout_no_wrap("0".into(), font.clone(), theme::text())
        .size()
        .x;
    let unit_w = digit_w; // Buchstaben h/m/s gleich breit (Monospace)
    let pad = 8.0;
    let w = pad * 2.0 + digit_w * 8.0 + unit_w * 3.0;
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 26.0), Sense::hover());
    let id = ui.make_persistent_id(("time_field", id_salt));
    let p = ui.painter().clone();
    p.rect_filled(rect, theme::R_S, theme::bg());
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );

    let mut changed = false;
    let parts = split(u64::from(*ms));
    let mut x = rect.left() + pad;
    let units = ["h", "m", "s", ""];
    for (i, (val, unit)) in parts.iter().zip(units).enumerate() {
        let cell = Rect::from_min_size(
            pos2(x, rect.top() + 2.0),
            Vec2::new(digit_w * 2.0, rect.height() - 4.0),
        );
        x += digit_w * 2.0;
        let sense = if enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let resp = ui.interact(cell.expand2(Vec2::new(1.0, 0.0)), id.with(i), sense);
        let hot = enabled && (resp.hovered() || resp.dragged());
        if hot {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
            p.rect_filled(
                cell.expand2(Vec2::new(1.0, 0.0)),
                2.0,
                theme::raised_hover(),
            );
        }
        if resp.dragged() {
            let acc_id = id.with(("acc", i));
            let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
            acc -= resp.drag_delta().y / PX_PER_STEP;
            let steps = acc.trunc();
            acc -= steps;
            ui.data_mut(|d| d.insert_temp(acc_id, acc));
            if steps != 0.0 {
                let delta = steps as i64 * STEPS_MS[i] as i64;
                let v = (i64::from(*ms) + delta).clamp(0, MAX_MS as i64) as u32;
                if v != *ms {
                    *ms = v;
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
            format!("{val:02}"),
            font.clone(),
            if !enabled {
                theme::text_faint()
            } else if hot {
                theme::accent()
            } else {
                theme::text()
            },
        );
        if !unit.is_empty() {
            p.text(
                pos2(x + unit_w * 0.5, rect.center().y),
                egui::Align2::CENTER_CENTER,
                unit,
                font.clone(),
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
        assert_eq!(format(0), "00h00m00s00");
        assert_eq!(format(1_000), "00h00m01s00");
        assert_eq!(format(61_250), "00h01m01s25");
        assert_eq!(format(3_600_000 + 120_000), "01h02m00s00");
        assert_eq!(split(999), [0, 0, 0, 99]);
    }
}
