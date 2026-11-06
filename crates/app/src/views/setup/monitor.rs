//! EINRICHTEN → DMX: Adressbelegung eines Universums.
//!
//! * Modus **Patch**: Geräte als Balken über ihren Adressbereich; per Ziehen
//!   auf eine neue Startadresse verschieben, freie Adressen sind sofort sichtbar.
//! * Modus **Ausgang**: die aktuell gesendeten DMX-Werte.

use dmxcraft_core::{Engine, FixtureId, UNIVERSE_SIZE};
use eframe::egui::{self, pos2, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{section_header, DipSwitch, Icon, IconButton, Segmented};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

const COLUMNS: usize = 32;
const ROWS: usize = UNIVERSE_SIZE / COLUMNS;
const CELL_H: f32 = 30.0;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum GridMode {
    #[default]
    Patch,
    Output,
}

struct Drag {
    fixture: FixtureId,
    /// Zelle innerhalb des Geräts, an der gegriffen wurde.
    offset: usize,
}

#[derive(Default)]
pub struct MonitorView {
    universe: usize,
    mode: GridMode,
    drag: Option<Drag>,
    context_target: Option<FixtureId>,
}

impl View for MonitorView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let universes = usize::from(cx.engine.show().universes);
        if let Some(id) = cx.state.monitor_focus.take() {
            if let Some(f) = cx.engine.show().fixture(id) {
                self.universe = usize::from(f.universe);
                cx.state.selected.clear();
                cx.state.selected.insert(id);
            }
        }
        self.universe = self.universe.min(universes.saturating_sub(1));

        let map = cx.engine.show().address_map(self.universe as u8);
        let free = map.iter().filter(|s| s.is_none()).count();
        section_header(ui, "DMX-Adressen", |ui| {
            ui.label(
                egui::RichText::new(format!("{free} von 512 Adressen frei")).color(theme::TEXT_DIM),
            );
        });
        ui.horizontal(|ui| {
            Segmented::new(&[(GridMode::Patch, "Patch"), (GridMode::Output, "Ausgang")])
                .show(ui, &mut self.mode);
            ui.add_space(theme::SP_L);
            if ui
                .add(IconButton::new(Icon::Prev).enabled(self.universe > 0))
                .clicked()
            {
                self.universe -= 1;
            }
            ui.label(
                egui::RichText::new(format!("Universum {} / {universes}", self.universe + 1))
                    .strong(),
            );
            if ui
                .add(IconButton::new(Icon::Next).enabled(self.universe + 1 < universes))
                .clicked()
            {
                self.universe += 1;
            }
            ui.add_space(theme::SP_L);
            ui.label(
                egui::RichText::new(match self.mode {
                    GridMode::Patch => "Gerät ziehen = Adresse ändern · Rechtsklick = Menü",
                    GridMode::Output => "Aktuell gesendete Werte",
                })
                .color(theme::TEXT_FAINT),
            );
        });
        ui.add_space(theme::SP_XS);

        egui::Panel::right("dmx_details")
            .resizable(false)
            .exact_size(300.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                left: 12,
                ..Default::default()
            }))
            .show(ui, |ui| details(ui, cx.engine, cx.state));

        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.grid(ui, cx, &map);
                    });
            });
    }
}

impl MonitorView {
    fn grid(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, map: &[Option<FixtureId>]) {
        let width = ui.available_width().max(400.0);
        let cell_w = width / COLUMNS as f32;
        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(width, CELL_H * ROWS as f32),
            Sense::click_and_drag(),
        );
        let cell_rect = |i: usize| {
            Rect::from_min_size(
                pos2(
                    rect.left() + (i % COLUMNS) as f32 * cell_w,
                    rect.top() + (i / COLUMNS) as f32 * CELL_H,
                ),
                Vec2::new(cell_w, CELL_H),
            )
            .shrink(1.0)
        };
        let cell_at = |p: Pos2| -> Option<usize> {
            if !rect.contains(p) {
                return None;
            }
            let c = ((p.x - rect.left()) / cell_w) as usize;
            let r = ((p.y - rect.top()) / CELL_H) as usize;
            (c < COLUMNS && r < ROWS).then_some(r * COLUMNS + c)
        };
        let show = cx.engine.show();
        let universe = self.universe as u8;
        let fixtures: Vec<(FixtureId, usize, usize, String)> = show
            .fixtures
            .iter()
            .filter(|f| f.universe == universe)
            .map(|f| {
                (
                    f.id,
                    usize::from(f.address).saturating_sub(1),
                    usize::from(show.fixture_channels(f.id)),
                    f.name.clone(),
                )
            })
            .collect();
        let pointer = ui.input(|i| i.pointer.latest_pos());
        let hovered_cell = resp.hover_pos().and_then(cell_at);
        let hovered_fixture = hovered_cell.and_then(|c| map[c]);
        let additive = ui.input(|i| i.modifiers.shift || i.modifiers.command);

        // ----- Interaktion (nur Patch-Modus verschiebt) -----
        if self.mode == GridMode::Patch && resp.drag_started() {
            let origin = ui
                .input(|i| i.pointer.press_origin())
                .or(resp.interact_pointer_pos());
            if let Some(cell) = origin.and_then(cell_at) {
                if let Some(fid) = map[cell] {
                    let start = fixtures.iter().find(|f| f.0 == fid).map_or(cell, |f| f.1);
                    self.drag = Some(Drag {
                        fixture: fid,
                        offset: cell - start,
                    });
                    cx.state.selected.clear();
                    cx.state.selected.insert(fid);
                }
            }
        }
        // Zielposition des gezogenen Geräts
        let mut ghost: Option<(usize, usize, bool, String)> = None;
        if let Some(d) = &self.drag {
            if let (Some(p), Some((_, _, len, _))) =
                (pointer, fixtures.iter().find(|f| f.0 == d.fixture))
            {
                if let Some(cell) = cell_at(p) {
                    let max_start = UNIVERSE_SIZE.saturating_sub(*len);
                    let start = cell.saturating_sub(d.offset).min(max_start);
                    let type_key = show
                        .fixture(d.fixture)
                        .map(|f| f.type_key.clone())
                        .unwrap_or_default();
                    let check = show.check_placement(
                        &type_key,
                        universe,
                        start as u16 + 1,
                        Some(d.fixture),
                    );
                    let msg = match &check {
                        Ok(()) => format!("Adresse {}", start + 1),
                        Err(e) => e.to_string(),
                    };
                    ghost = Some((start, *len, check.is_ok(), msg));
                }
            }
        }
        let mut repatch: Option<(FixtureId, u16)> = None;
        if resp.drag_stopped() {
            if let (Some(d), Some((start, _, ok, msg))) = (self.drag.take(), ghost.clone()) {
                if ok {
                    repatch = Some((d.fixture, start as u16 + 1));
                } else {
                    cx.state.error(format!("Nicht verschoben: {msg}"));
                }
            }
            self.drag = None;
        }
        if resp.clicked() {
            match hovered_fixture {
                Some(id) if additive => {
                    if !cx.state.selected.remove(&id) {
                        cx.state.selected.insert(id);
                    }
                }
                Some(id) => {
                    cx.state.selected.clear();
                    cx.state.selected.insert(id);
                }
                None if !additive => cx.state.selected.clear(),
                None => {}
            }
        }
        if resp.secondary_clicked() {
            self.context_target = hovered_fixture;
            if let Some(id) = hovered_fixture {
                if !cx.state.selected.contains(&id) {
                    cx.state.selected.clear();
                    cx.state.selected.insert(id);
                }
            }
        }
        let mut action = None;
        let target = self.context_target;
        let has_clip = cx.state.layout_clipboard.is_some();
        let selected = &cx.state.selected;
        resp.context_menu(|ui| {
            action = fixture_menu::menu(ui, show, selected, target, MenuPlace::Patch, has_clip);
        });

        // ----- Zeichnen -----
        let p = ui.painter_at(rect);
        let data = cx.frame.universe(self.universe);
        for (i, owner) in map.iter().enumerate() {
            let r = cell_rect(i);
            match self.mode {
                GridMode::Patch => {
                    p.rect_filled(
                        r,
                        3.0,
                        if owner.is_some() {
                            theme::BG
                        } else {
                            theme::RAISED
                        },
                    );
                    p.text(
                        r.left_top() + Vec2::new(4.0, 3.0),
                        egui::Align2::LEFT_TOP,
                        (i + 1).to_string(),
                        theme::caption(),
                        theme::TEXT_FAINT,
                    );
                }
                GridMode::Output => {
                    let v = data.map_or(0, |d| d[i]);
                    let level = f32::from(v) / 255.0;
                    p.rect_filled(r, 3.0, theme::RAISED);
                    if v > 0 {
                        let fill = Rect::from_min_max(
                            pos2(r.left(), r.bottom() - r.height() * level),
                            r.max,
                        );
                        p.rect_filled(fill, 3.0, theme::ACCENT.gamma_multiply(0.35));
                    }
                    p.text(
                        r.center(),
                        egui::Align2::CENTER_CENTER,
                        v.to_string(),
                        theme::caption(),
                        if v > 0 {
                            theme::TEXT
                        } else {
                            theme::TEXT_FAINT
                        },
                    );
                }
            }
        }

        // Geräte als Balken (pro Zeile ein Segment)
        let segments = |start: usize, len: usize| -> Vec<(usize, usize)> {
            let mut out = Vec::new();
            let end = (start + len).min(UNIVERSE_SIZE);
            let mut i = start;
            while i < end {
                let row_end = ((i / COLUMNS) + 1) * COLUMNS;
                let seg_end = end.min(row_end);
                out.push((i, seg_end - 1));
                i = seg_end;
            }
            out
        };
        for (id, start, len, name) in &fixtures {
            let dragging = self.drag.as_ref().is_some_and(|d| d.fixture == *id);
            let selected = cx.state.selected.contains(id);
            let hovered = hovered_fixture == Some(*id);
            for (si, (a, b)) in segments(*start, *len).into_iter().enumerate() {
                let seg = cell_rect(a).union(cell_rect(b));
                match self.mode {
                    GridMode::Patch => {
                        let fill = if selected {
                            theme::ACCENT_DIM
                        } else if hovered {
                            theme::RAISED_HOVER
                        } else {
                            theme::FIXTURE_BAR
                        };
                        let fill = if dragging {
                            fill.gamma_multiply(0.4)
                        } else {
                            fill
                        };
                        p.rect_filled(seg, 4.0, fill);
                        p.rect_stroke(
                            seg,
                            4.0,
                            Stroke::new(
                                1.0,
                                if selected {
                                    theme::ACCENT
                                } else {
                                    theme::BORDER
                                },
                            ),
                            StrokeKind::Inside,
                        );
                        if si == 0 {
                            let label = format!("{}  {}", start + 1, name);
                            let max_chars = ((seg.width() - 10.0) / 6.6).max(1.0) as usize;
                            p.text(
                                seg.left_center() + Vec2::new(6.0, 0.0),
                                egui::Align2::LEFT_CENTER,
                                clip(&label, max_chars),
                                theme::caption(),
                                if selected {
                                    theme::TEXT
                                } else {
                                    theme::TEXT_DIM
                                },
                            );
                        }
                    }
                    GridMode::Output => {
                        p.rect_stroke(
                            seg,
                            4.0,
                            Stroke::new(
                                if selected { 2.0 } else { 1.0 },
                                if selected {
                                    theme::ACCENT
                                } else {
                                    theme::ACCENT_DIM
                                },
                            ),
                            StrokeKind::Inside,
                        );
                    }
                }
            }
        }
        // Vorschau beim Ziehen
        if let Some((start, len, ok, msg)) = &ghost {
            let color = if *ok { theme::OK } else { theme::DANGER };
            for (a, b) in segments(*start, *len) {
                let seg = cell_rect(a).union(cell_rect(b));
                p.rect_filled(seg, 4.0, color.gamma_multiply(0.25));
                p.rect_stroke(seg, 4.0, Stroke::new(2.0, color), StrokeKind::Inside);
            }
            if let Some(pt) = pointer {
                p.text(
                    pt + Vec2::new(14.0, -14.0),
                    egui::Align2::LEFT_BOTTOM,
                    msg,
                    theme::body(),
                    color,
                );
            }
        }
        if self.drag.is_some() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else if self.mode == GridMode::Patch && hovered_fixture.is_some() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }
        if let (Some(c), None) = (hovered_cell, &self.drag) {
            let text = match hovered_fixture.and_then(|id| show.fixture(id)) {
                Some(f) => {
                    let ch = c + 1 - usize::from(f.address);
                    let ch_name = show
                        .type_of(f)
                        .and_then(|t| t.channels.get(ch))
                        .map_or(String::new(), |d| d.name.clone());
                    format!(
                        "Adresse {} · {} · Kanal {} {}",
                        c + 1,
                        f.name,
                        ch + 1,
                        ch_name
                    )
                }
                None => format!("Adresse {} · frei", c + 1),
            };
            resp.on_hover_text_at_pointer(text);
        }

        if let Some((id, addr)) = repatch {
            if let Err(e) = cx.engine.show_mut().repatch_fixture(id, universe, addr) {
                cx.state.error(e.to_string());
            } else {
                cx.state.info(format!("Neue Startadresse {addr}"));
            }
        }
        if let Some(a) = action {
            fixture_menu::apply(cx.engine, cx.state, a);
        }
    }
}

/// Seitenleiste: Details und DIP-Schalter des ausgewählten Geräts.
fn details(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
    section_header(ui, "Gerät", |_| {});
    let Some(id) = st.selected.iter().next().copied() else {
        ui.label(
            egui::RichText::new("Ein Gerät anklicken, um Adresse und DIP-Schalter zu sehen.")
                .color(theme::TEXT_DIM),
        );
        return;
    };
    let Some(f) = e.show().fixture(id).cloned() else {
        return;
    };
    let ch = e.show().fixture_channels(id);
    let ty = e
        .show()
        .type_of(&f)
        .map(|t| t.display_name())
        .unwrap_or_default();
    ui.label(egui::RichText::new(&f.name).size(theme::FS_TITLE).strong());
    ui.label(egui::RichText::new(ty).color(theme::TEXT_DIM));
    ui.add_space(theme::SP_S);
    egui::Grid::new("dmx_detail_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label(egui::RichText::new("Universum").color(theme::TEXT_DIM));
            ui.label((f.universe + 1).to_string());
            ui.end_row();
            ui.label(egui::RichText::new("Adressen").color(theme::TEXT_DIM));
            ui.label(format!(
                "{} – {}  ({ch} Kanäle)",
                f.address,
                f.address + ch.saturating_sub(1)
            ));
            ui.end_row();
        });
    ui.add_space(theme::SP_S);
    ui.label(
        egui::RichText::new("DIP-SCHALTER")
            .font(theme::caption())
            .color(theme::TEXT_DIM)
            .strong(),
    );
    DipSwitch { address: f.address }.show(ui);
    let on = dmxcraft_core::dip::on_switches(f.address);
    ui.label(
        egui::RichText::new(if on.is_empty() {
            "Alle Schalter aus".to_string()
        } else {
            format!(
                "AN: {}",
                on.iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .color(theme::TEXT_DIM),
    );
    if f.address > 511 {
        ui.label(
            egui::RichText::new("Adresse 512 ist mit 9 Schaltern nicht einstellbar.")
                .color(theme::WARN),
        );
    }
    if st.selected.len() > 1 {
        ui.add_space(theme::SP_S);
        ui.label(
            egui::RichText::new(format!(
                "{} weitere Geräte ausgewählt",
                st.selected.len() - 1
            ))
            .color(theme::TEXT_FAINT),
        );
    }
    ui.add_space(theme::SP_M);
    if ui.button("Auf nächste freie Adresse").clicked() {
        fixture_menu::apply(e, st, fixture_menu::FixtureAction::NextFreeAddress);
    }
}

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else if max <= 3 {
        s.chars().take(max).collect()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}
