//! EINRICHTEN → Patch-Raster: Adressbelegung eines Universums.
//!
//! * Modus **Patch**: Geräte als Balken über ihren Adressbereich; per Ziehen
//!   auf eine neue Startadresse verschieben, freie Adressen sind sofort sichtbar.
//! * Modus **Ausgang**: die aktuell gesendeten DMX-Werte.

use crate::i18n::t;
use dmxcraft_core::{FixtureId, UNIVERSE_SIZE};
use eframe::egui::{self, pos2, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::state::LibraryDrag;
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
    /// Name noch markieren, bis die Maus nach dem Doppelklick losgelassen ist
    /// (sonst setzt der zweite Klick den Cursor ins Feld).
    rename_fresh: bool,
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
                self.rename_fresh = true;
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
            Segmented::new(&[
                (GridMode::Patch, t("Patch")),
                (GridMode::Output, t("Ausgang")),
            ])
            .show(ui, &mut self.mode);
            ui.add_space(theme::SP_M);
            if ui
                .add(IconButton::new(Icon::Prev).enabled(self.universe > 0))
                .clicked()
            {
                self.universe -= 1;
            }
            ui.label(
                egui::RichText::new(tr!(
                    "Universum {universe_nr} / {universes}",
                    universes = universes,
                    universe_nr = self.universe + 1
                ))
                .strong(),
            );
            if ui
                .add(IconButton::new(Icon::Next).enabled(self.universe + 1 < universes))
                .clicked()
            {
                self.universe += 1;
            }
            ui.add_space(theme::SP_M);
            ui.label(
                egui::RichText::new(tr!("{free} von 512 frei", free = free))
                    .color(theme::text_dim()),
            );
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
                        Ok(()) => tr!("Adresse {start_nr}", start_nr = start + 1),
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
                    cx.state.error(tr!("Nicht verschoben: {msg}", msg = msg));
                }
            }
            self.drag = None;
        }
        if resp.double_clicked() && self.mode == GridMode::Patch {
            if let Some(id) = hovered_fixture {
                self.rename = show.fixture(id).map(|f| (id, f.name.clone()));
                self.rename_fresh = true;
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
                            theme::bg()
                        } else {
                            theme::raised()
                        },
                    );
                    p.text(
                        r.center(),
                        egui::Align2::CENTER_CENTER,
                        (i + 1).to_string(),
                        theme::caption(),
                        theme::text_faint(),
                    );
                }
                GridMode::Output => {
                    let v = data.map_or(0, |d| d[i]);
                    let level = f32::from(v) / 255.0;
                    p.rect_filled(r, 3.0, theme::raised());
                    if v > 0 {
                        let fill = Rect::from_min_max(
                            pos2(r.left(), r.bottom() - r.height() * level),
                            r.max,
                        );
                        p.rect_filled(fill, 3.0, theme::accent().gamma_multiply(0.35));
                    }
                    p.text(
                        r.center(),
                        egui::Align2::CENTER_CENTER,
                        v.to_string(),
                        theme::caption(),
                        if v > 0 {
                            theme::text()
                        } else {
                            theme::text_faint()
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
                            theme::accent_dim()
                        } else if hovered {
                            theme::raised_hover()
                        } else {
                            theme::fixture_bar()
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
                                    theme::accent()
                                } else {
                                    theme::border()
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
                                    theme::text()
                                } else {
                                    theme::text_dim()
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
                                    theme::accent()
                                } else {
                                    theme::accent_dim()
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
                        .align(egui::Align2::CENTER_CENTER)
                        .margin(egui::Margin::symmetric(4, 2)),
                );
                if !edit.has_focus() && !edit.lost_focus() {
                    edit.request_focus();
                }
                if self.rename_fresh {
                    crate::ui::widgets::dialog::select_all(ui.ctx(), edit.id, text);
                    let (down, released) =
                        ui.input(|i| (i.pointer.any_down(), i.pointer.any_released()));
                    if !down && !released && edit.has_focus() {
                        self.rename_fresh = false;
                    }
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
                    tr!(
                        "Adresse {c_nr} · {v} · Kanal {ch_nr} {ch_name}",
                        c_nr = c + 1,
                        v = f.name,
                        ch_nr = ch + 1,
                        ch_name = ch_name
                    )
                }
                None => tr!("Adresse {c_nr} · frei", c_nr = c + 1),
            };
            resp.on_hover_text_at_pointer(text);
        }

        // Gerät aus der Bibliothek ablegen
        let mut lib_drop = None;
        if let Some(d) = egui::DragAndDrop::payload::<LibraryDrag>(ui.ctx()) {
            if let Some(cell) = pointer.and_then(cell_at) {
                let n = usize::from(d.0.channel_count()).max(1);
                let fits = cell + n <= map.len() && map[cell..cell + n].iter().all(|s| s.is_none());
                let color = if fits { theme::OK } else { theme::DANGER };
                for i in cell..(cell + n).min(map.len()) {
                    ui.painter()
                        .rect_filled(cell_rect(i), theme::R_S, color.gamma_multiply(0.35));
                }
                if ui.input(|i| i.pointer.any_released()) {
                    lib_drop = Some((d.0.clone(), cell, fits));
                    egui::DragAndDrop::clear_payload(ui.ctx());
                }
            }
        }
        if let Some((ty, cell, fits)) = lib_drop {
            if fits {
                let e = &mut *cx.engine;
                e.show_mut().upsert_fixture_type(ty.clone());
                let n = e
                    .show()
                    .fixtures
                    .iter()
                    .filter(|f| f.type_key == ty.key)
                    .count()
                    + 1;
                let name = format!("{} {n}", ty.model);
                match e
                    .show_mut()
                    .add_fixture(name.clone(), &ty.key, universe, cell as u16 + 1)
                {
                    Ok(id) => {
                        cx.state.selected = std::iter::once(id).collect();
                        cx.state.info(tr!(
                            "„{name}“ gepatcht ab Adresse {addr}",
                            name = name,
                            addr = cell + 1
                        ));
                    }
                    Err(err) => cx.state.error(err.to_string()),
                }
            } else {
                cx.state.error(t("Dort ist nicht genug Platz frei"));
            }
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
                cx.state.info(tr!("Neue Startadresse {addr}", addr = addr));
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
