//! DMX-Monitor: zeigt die 512 Kanäle eines Universums.

use eframe::egui;
use lightcraft_core::{DmxFrame, Engine, UNIVERSE_SIZE};

use crate::app::UiState;

const COLUMNS: usize = 32;

pub fn ui(ui: &mut egui::Ui, e: &Engine, st: &mut UiState, frame: &DmxFrame) {
    let universes = usize::from(e.show().universes);
    st.monitor_universe = st.monitor_universe.min(universes.saturating_sub(1));
    ui.horizontal(|ui| {
        ui.label("Universum");
        for u in 0..universes {
            ui.selectable_value(&mut st.monitor_universe, u, format!("{}", u + 1));
        }
    });

    // Welche Adressen sind gepatcht? (für Tooltip und Rahmen)
    let mut owner: Vec<Option<String>> = vec![None; UNIVERSE_SIZE];
    for f in e
        .show()
        .fixtures
        .iter()
        .filter(|f| usize::from(f.universe) == st.monitor_universe)
    {
        if let Some(ty) = e.show().type_of(f) {
            for (i, def) in ty.channels.iter().enumerate() {
                let idx = usize::from(f.address) - 1 + i;
                if idx < UNIVERSE_SIZE {
                    owner[idx] = Some(format!("{} – {}", f.name, def.name));
                }
            }
        }
    }

    let Some(data) = frame.universe(st.monitor_universe) else {
        ui.label("Keine Daten");
        return;
    };

    let avail = ui.available_width();
    let cell = ((avail - 40.0) / COLUMNS as f32).clamp(22.0, 44.0);
    let rows = UNIVERSE_SIZE.div_ceil(COLUMNS);
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(40.0 + cell * COLUMNS as f32, cell * rows as f32),
        egui::Sense::hover(),
    );
    let painter = ui.painter_at(rect);
    let font = egui::FontId::monospace((cell * 0.38).clamp(8.0, 13.0));
    let text_color = ui.visuals().text_color();
    let hover = ui.ctx().pointer_hover_pos();
    let mut tooltip: Option<String> = None;

    for row in 0..rows {
        let y = rect.top() + row as f32 * cell;
        painter.text(
            egui::pos2(rect.left() + 34.0, y + cell / 2.0),
            egui::Align2::RIGHT_CENTER,
            format!("{}", row * COLUMNS + 1),
            font.clone(),
            ui.visuals().weak_text_color(),
        );
        for col in 0..COLUMNS {
            let idx = row * COLUMNS + col;
            if idx >= UNIVERSE_SIZE {
                break;
            }
            let v = data[idx];
            let r = egui::Rect::from_min_size(
                egui::pos2(rect.left() + 40.0 + col as f32 * cell, y),
                egui::vec2(cell - 2.0, cell - 2.0),
            );
            let g = v / 2 + 20;
            let fill = egui::Color32::from_rgb(g / 3, g, g / 2);
            painter.rect_filled(r, 2.0, fill);
            if owner[idx].is_some() {
                painter.rect_stroke(
                    r,
                    2.0,
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(90, 140, 220)),
                    egui::StrokeKind::Inside,
                );
            }
            painter.text(
                r.center(),
                egui::Align2::CENTER_CENTER,
                v.to_string(),
                font.clone(),
                text_color,
            );
            if hover.is_some_and(|p| r.contains(p)) {
                tooltip = Some(format!(
                    "Adresse {}: {}{}",
                    idx + 1,
                    v,
                    owner[idx]
                        .as_ref()
                        .map(|o| format!("\n{o}"))
                        .unwrap_or_default()
                ));
            }
        }
    }

    if let Some(t) = tooltip {
        ui.label(t);
    } else {
        ui.weak("Maus über einen Kanal bewegen für Details. Blau umrandet = gepatcht.");
    }
}
