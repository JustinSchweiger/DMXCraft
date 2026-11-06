//! EINRICHTEN → Patch-Raster: Adressbelegung eines Universums.
//!
//! * Modus **Patch**: Geräte als Balken über ihren Adressbereich; per Ziehen
//!   auf eine neue Startadresse verschieben, freie Adressen sind sofort sichtbar.
//! * Modus **Ausgang**: die aktuell gesendeten DMX-Werte.

use dmxcraft_core::{FixtureId, UNIVERSE_SIZE};
use eframe::egui::{self, pos2, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::ui::theme;
use crate::ui::widgets::{DipSwitch, Icon, IconButton, Segmented};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

const COLUMNS: usize = 32;
const ROWS: usize = UNIVERSE_SIZE / COLUMNS;
/// Zellhöhe passt sich der verfügbaren Höhe an (zwischen diesen Grenzen).
const CELL_H_MIN: f32 = 18.0;
const CELL_H_MAX: f32 = 30.0;

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
pub struct PatchGrid {
    universe: usize,
    mode: GridMode,
    drag: Option<Drag>,
    context_target: Option<FixtureId>,
    /// Laufende Umbenennung: Gerät und bearbeiteter Name.
    rename: Option<(FixtureId, String)>,
}

impl View for PatchGrid {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let universes = usize::from(cx.engine.show().universes);
        if std::mem::take(&mut cx.state.show_dmx_levels) {
            self.mode = GridMode::Output;
        }
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
        if let Some(id) = cx.state.rename_request.take() {
            if let Some(f) = cx.engine.show().fixture(id) {
                self.universe = usize::from(f.universe);
                self.rename = Some((id, f.name.clone()));
            }
        }
        let dip_address = (cx.state.selected.len() == 1)
            .then(|| {
                cx.state
                    .selected
                    .iter()
                    .next()
                    .and_then(|id| cx.engine.show().fixture(*id))
            })
            .flatten()
            .filter(|f| usize::from(f.universe) == self.universe)
            .map(|f| (f.address, ()));
        ui.horizontal(|ui| {
            ui.set_min_height(30.0);
            ui.label(theme::heading_text("Patch"));
            ui.add_space(theme::SP_S);
            Segmented::new(&[(GridMode::Patch, "Patch"), (GridMode::Output, "Ausgang")])
                .show(ui, &mut self.mode);
            ui.add_space(theme::SP_M);
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
            ui.add_space(theme::SP_M);
            ui.label(egui::RichText::new(format!("{free} von 512 frei")).color(theme::TEXT_DIM));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some((addr, _)) = &dip_address {
                    DipSwitch {
                        address: *addr,
                        compact: true,
                    }
                    .show(ui);
                }
            });
        });
        ui.add_space(theme::SP_XS);

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

impl PatchGrid {
    fn grid(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, map: &[Option<FixtureId>]) {
        let width = ui.available_width().max(400.0);
        let cell_w = width / COLUMNS as f32;
        let cell_h = (ui.available_height() / ROWS as f32).clamp(CELL_H_MIN, CELL_H_MAX);
        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(width, cell_h * ROWS as f32),
            Sense::click_and_drag(),
        );
        let cell_rect = |i: usize| {
            Rect::from_min_size(
                pos2(
                    rect.left() + (i % COLUMNS) as f32 * cell_w,
                    rect.top() + (i / COLUMNS) as f32 * cell_h,
                ),
                Vec2::new(cell_w, cell_h),
            )
            .shrink(1.0)
        };
        let cell_at = |p: Pos2| -> Option<usize> {
            if !rect.contains(p) {
                return None;
            }
            let c = ((p.x - rect.left()) / cell_w) as usize;
            let r = ((p.y - rect.top()) / cell_h) as usize;
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
        if resp.double_clicked() && self.mode == GridMode::Patch {
            if let Some(id) = hovered_fixture {
                self.rename = show.fixture(id).map(|f| (id, f.name.clone()));
            }
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
        crate::ui::widgets::menu::context(&resp, |ui| {
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
                        r.center(),
                        egui::Align2::CENTER_CENTER,
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
        // Umbenennen direkt im Raster
        let mut rename_commit: Option<(FixtureId, String)> = None;
        if let Some((id, text)) = &mut self.rename {
            if let Some((_, start, len, _)) = fixtures.iter().find(|f| f.0 == *id) {
                let first_end = (*start + *len).min(((*start / COLUMNS) + 1) * COLUMNS) - 1;
                let seg = cell_rect(*start).union(cell_rect(first_end)).shrink(2.0);
                let seg =
                    Rect::from_min_size(seg.min, Vec2::new(seg.width().max(160.0), seg.height()));
                let edit = ui.put(
                    seg,
                    egui::TextEdit::singleline(text)
                        .font(theme::body())
                        .margin(egui::Margin::symmetric(4, 2)),
                );
                if !edit.has_focus() && !edit.lost_focus() {
                    edit.request_focus();
                }
                let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
                let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
                if esc {
                    self.rename = None;
                } else if edit.lost_focus() || enter {
                    rename_commit = Some((*id, text.as_str().trim().to_string()));
                }
            } else {
                // Gerät liegt in einem anderen Universum
                self.rename = None;
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

        if let Some((id, name)) = rename_commit {
            self.rename = None;
            if !name.is_empty() {
                if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                    f.name = name;
                }
            }
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

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else if max <= 3 {
        s.chars().take(max).collect()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}
