//! Gruppenleiste und Bühnenplan: Geräte auswählen, anordnen, Live-Farbe sehen.

use std::collections::BTreeSet;

use dmxcraft_core::{ChannelKind, DmxFrame, Engine, FixtureId, GroupId};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::app::UiState;
use crate::theme;

const CELL: f32 = 30.0;

#[derive(Default)]
pub struct StageState {
    /// Geräte per Ziehen verschieben statt auswählen.
    pub arrange: bool,
    drag_start: Option<Pos2>,
    moving: Option<FixtureId>,
}

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, frame: &DmxFrame) {
    group_bar(ui, e, st);
    ui.separator();
    let height = ui.available_height().max(120.0);
    ui.horizontal(|ui| {
        ui.set_min_height(height);
        tools(ui, e, st);
        ui.separator();
        plan(ui, e, st, frame, height);
    });
}

fn group_bar(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    let mut remove: Option<GroupId> = None;
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new("GRUPPEN")
                .small()
                .color(theme::TEXT_WEAK),
        );
        let all: BTreeSet<FixtureId> = e.show().fixtures.iter().map(|f| f.id).collect();
        if ui
            .selectable_label(!all.is_empty() && st.selected == all, "Alle")
            .clicked()
        {
            st.selected = all;
        }
        for g in &e.show().groups {
            let set: BTreeSet<FixtureId> = g.fixtures.iter().copied().collect();
            let r = ui.selectable_label(!set.is_empty() && st.selected == set, &g.name);
            if r.clicked() {
                st.selected = set;
            }
            r.context_menu(|ui| {
                if ui.button("Gruppe löschen").clicked() {
                    remove = Some(g.id);
                    ui.close();
                }
            });
        }
        ui.add_space(8.0);
        ui.add(
            egui::TextEdit::singleline(&mut st.new_group_name)
                .hint_text("Neue Gruppe")
                .desired_width(110.0),
        );
        if ui
            .add_enabled(!st.selected.is_empty(), egui::Button::new("+ aus Auswahl"))
            .clicked()
        {
            let name = if st.new_group_name.trim().is_empty() {
                format!("Gruppe {}", e.show().groups.len() + 1)
            } else {
                st.new_group_name.trim().to_string()
            };
            e.show_mut()
                .add_group(name, st.selected.iter().copied().collect());
            st.new_group_name.clear();
        }
    });
    if let Some(g) = remove {
        e.show_mut().remove_group(g);
    }
}

fn tools(ui: &mut egui::Ui, e: &Engine, st: &mut UiState) {
    ui.vertical(|ui| {
        ui.set_width(84.0);
        if ui.button("Alle").clicked() {
            st.selected = e.show().fixtures.iter().map(|f| f.id).collect();
        }
        if ui.button("Keine").clicked() {
            st.selected.clear();
        }
        if ui.button("Umkehren").clicked() {
            st.selected = e
                .show()
                .fixtures
                .iter()
                .map(|f| f.id)
                .filter(|id| !st.selected.contains(id))
                .collect();
        }
        if ui
            .button("Jedes 2.")
            .on_hover_text("Jedes zweite Gerät der Auswahl")
            .clicked()
        {
            st.selected = st.selected.iter().copied().step_by(2).collect();
        }
        ui.add_space(6.0);
        ui.toggle_value(&mut st.stage.arrange, "Anordnen")
            .on_hover_text("Geräte im Plan per Ziehen verschieben");
    });
}

/// Position eines Geräts im Plan (0..1). Ohne gespeicherte Position: Raster.
fn position(e: &Engine, idx: usize, id: FixtureId) -> [f32; 2] {
    if let Some(p) = e.show().fixture(id).and_then(|f| f.plan_pos) {
        return p;
    }
    let per_row = 12;
    let col = (idx % per_row) as f32;
    let row = (idx / per_row) as f32;
    [0.06 + col * 0.075, 0.12 + row * 0.18]
}

/// Anzeigefarbe aus den aktuellen DMX-Werten.
fn live_color(e: &Engine, id: FixtureId, frame: &DmxFrame) -> (Color32, f32) {
    let Some(f) = e.show().fixture(id) else {
        return (Color32::DARK_GRAY, 0.0);
    };
    let Some(ty) = e.show().type_of(f) else {
        return (Color32::DARK_GRAY, 0.0);
    };
    let val = |k: ChannelKind| -> Option<f32> {
        let i = ty.channels.iter().position(|c| c.kind == k)?;
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map(|v| f32::from(v) / 255.0)
    };
    let (r, g, b, w) = (
        val(ChannelKind::Red),
        val(ChannelKind::Green),
        val(ChannelKind::Blue),
        val(ChannelKind::White).unwrap_or(0.0),
    );
    let dim = val(ChannelKind::Dimmer);
    let has_color = r.is_some() || g.is_some() || b.is_some();
    let (mut cr, mut cg, mut cb) = if has_color {
        (
            r.unwrap_or(0.0) + w,
            g.unwrap_or(0.0) + w,
            b.unwrap_or(0.0) + w,
        )
    } else {
        (1.0, 0.95, 0.85) // Weißlicht
    };
    let intensity = match (dim, has_color) {
        (Some(d), _) => d,
        (None, true) => 1.0,
        (None, false) => 0.0,
    };
    cr = (cr * intensity).min(1.0);
    cg = (cg * intensity).min(1.0);
    cb = (cb * intensity).min(1.0);
    let lum = cr.max(cg).max(cb);
    (
        Color32::from_rgb((cr * 255.0) as u8, (cg * 255.0) as u8, (cb * 255.0) as u8),
        lum,
    )
}

fn plan(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, frame: &DmxFrame, height: f32) {
    let size = Vec2::new(ui.available_width(), height);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, theme::BG);
    // dezentes Raster
    let grid = Color32::from_rgb(0x1c, 0x23, 0x2d);
    let mut x = rect.left();
    while x < rect.right() {
        painter.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(1.0, grid),
        );
        x += CELL;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        painter.line_segment(
            [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
            Stroke::new(1.0, grid),
        );
        y += CELL;
    }

    let ids: Vec<FixtureId> = e.show().fixtures.iter().map(|f| f.id).collect();
    if ids.is_empty() {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Noch keine Geräte – unter EINRICHTEN → Patch hinzufügen",
            egui::FontId::proportional(14.0),
            theme::TEXT_WEAK,
        );
        return;
    }
    let to_screen = |p: [f32; 2]| {
        Pos2::new(
            rect.left() + p[0] * rect.width(),
            rect.top() + p[1] * rect.height(),
        )
    };
    let boxes: Vec<(FixtureId, Rect)> = ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            (
                *id,
                Rect::from_center_size(to_screen(position(e, i, *id)), Vec2::splat(26.0)),
            )
        })
        .collect();

    let pointer = resp.interact_pointer_pos().or(resp.hover_pos());
    let hit = |p: Pos2| {
        boxes
            .iter()
            .rev()
            .find(|(_, r)| r.contains(p))
            .map(|(id, _)| *id)
    };
    let modifiers = ui.input(|i| i.modifiers);

    // ----- Interaktion -----
    if resp.drag_started() {
        if let Some(p) = resp.interact_pointer_pos() {
            if st.stage.arrange {
                st.stage.moving = hit(p);
            } else {
                st.stage.drag_start = Some(p);
            }
        }
    }
    if resp.dragged() {
        if let (Some(id), Some(p)) = (st.stage.moving, resp.interact_pointer_pos()) {
            let nx = ((p.x - rect.left()) / rect.width()).clamp(0.02, 0.98);
            let ny = ((p.y - rect.top()) / rect.height()).clamp(0.04, 0.96);
            // Mehrfachauswahl gemeinsam verschieben
            let delta = resp.drag_delta();
            if st.selected.contains(&id) && st.selected.len() > 1 {
                let dx = delta.x / rect.width();
                let dy = delta.y / rect.height();
                for (i, fid) in ids.iter().enumerate() {
                    if st.selected.contains(fid) {
                        let cur = position(e, i, *fid);
                        if let Some(f) = e.show_mut().fixture_mut(*fid) {
                            f.plan_pos = Some([
                                (cur[0] + dx).clamp(0.02, 0.98),
                                (cur[1] + dy).clamp(0.04, 0.96),
                            ]);
                        }
                    }
                }
            } else if let Some(f) = e.show_mut().fixture_mut(id) {
                f.plan_pos = Some([nx, ny]);
            }
        }
    }
    let mut lasso: Option<Rect> = None;
    if let (Some(start), Some(p)) = (st.stage.drag_start, pointer) {
        if resp.dragged() || resp.drag_stopped() {
            lasso = Some(Rect::from_two_pos(start, p));
        }
    }
    if resp.drag_stopped() {
        if let Some(l) = lasso {
            let inside: Vec<FixtureId> = boxes
                .iter()
                .filter(|(_, r)| l.intersects(*r))
                .map(|(id, _)| *id)
                .collect();
            if !(modifiers.shift || modifiers.command) {
                st.selected.clear();
            }
            st.selected.extend(inside);
        }
        st.stage.drag_start = None;
        st.stage.moving = None;
    }
    if resp.clicked() {
        if let Some(p) = resp.interact_pointer_pos() {
            match hit(p) {
                Some(id) if modifiers.shift || modifiers.command => {
                    if !st.selected.remove(&id) {
                        st.selected.insert(id);
                    }
                }
                Some(id) => {
                    st.selected.clear();
                    st.selected.insert(id);
                }
                None => st.selected.clear(),
            }
        }
    }

    // ----- Zeichnen -----
    let hovered = resp.hover_pos().and_then(hit);
    for (id, r) in &boxes {
        let (c, lum) = live_color(e, *id, frame);
        if lum > 0.02 {
            painter.circle_filled(r.center(), 20.0, c.gamma_multiply(0.18 * lum));
        }
        painter.rect_filled(*r, 5.0, Color32::from_rgb(0x2a, 0x32, 0x3e));
        painter.circle_filled(
            r.center(),
            8.5,
            if lum > 0.02 {
                c
            } else {
                Color32::from_rgb(0x10, 0x14, 0x1a)
            },
        );
        let sel = st.selected.contains(id);
        let stroke = if sel {
            Stroke::new(2.0, theme::ACCENT)
        } else if Some(*id) == hovered {
            Stroke::new(1.0, Color32::GRAY)
        } else {
            Stroke::new(1.0, Color32::from_rgb(0x3a, 0x44, 0x52))
        };
        painter.rect_stroke(*r, 5.0, stroke, StrokeKind::Outside);
    }
    if let Some(id) = hovered {
        if let Some(f) = e.show().fixture(id) {
            let r = boxes
                .iter()
                .find(|(i, _)| *i == id)
                .map(|(_, r)| *r)
                .unwrap_or(rect);
            painter.text(
                r.center_bottom() + Vec2::new(0.0, 4.0),
                egui::Align2::CENTER_TOP,
                format!("{} · U{} / {}", f.name, f.universe + 1, f.address),
                egui::FontId::proportional(11.0),
                Color32::WHITE,
            );
        }
    }
    if let Some(l) = lasso {
        painter.rect_filled(l, 0.0, theme::ACCENT.gamma_multiply(0.08));
        painter.rect_stroke(l, 0.0, Stroke::new(1.0, theme::ACCENT), StrokeKind::Inside);
    }
    painter.text(
        rect.right_bottom() - Vec2::new(8.0, 6.0),
        egui::Align2::RIGHT_BOTTOM,
        format!("{} ausgewählt", st.selected.len()),
        egui::FontId::proportional(11.0),
        theme::TEXT_WEAK,
    );
}
