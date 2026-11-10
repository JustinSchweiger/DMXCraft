//! Timeline of a SUPER SCENE – replaces the programmer while a SUPER SCENE
//! is selected. Laid out like a video editor: toolbar at the top, below it
//! the zoom/scroll bar, the time ruler and the tracks with their blocks
//! (scenes and audio).

use crate::i18n::t;
use crate::ui::widgets::color_picker::color_swatches;
use dmxcraft_core::timeline::{Clip, ClipContent, GridMode, Timeline};
use dmxcraft_core::{Engine, SceneId};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Shape, Stroke, StrokeKind, Vec2};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton};

const HEADER_W: f32 = 132.0;
const RULER_H: f32 = 22.0;
const ZOOMBAR_H: f32 = 12.0;
const TRACK_H: f32 = 44.0;
/// Tracks with audio are taller (waveform).
const AUDIO_TRACK_H: f32 = 84.0;
const LANE_H: f32 = 36.0;
const EDGE: f32 = 6.0;
/// Length of new scene blocks without their own duration.
const DEFAULT_CLIP_MS: u64 = 5000;

/// Ongoing drag on a block.
#[derive(Clone, Copy, PartialEq)]
enum Drag {
    Move { grab_ms: f64 },
    Left,
    Right,
    FadeIn,
    FadeOut,
}

pub struct TimelineView {
    /// Pixels per millisecond.
    zoom: f32,
    /// Time at the left edge (ms).
    scroll_ms: f64,
    scroll_y: f32,
    /// Dragging in the ruler (zoom/scroll): start point of the pointer.
    ruler_grab: Option<egui::Pos2>,
    /// Axis when dragging in the ruler: `Some(true)` = horizontal (scroll),
    /// `Some(false)` = vertical (zoom), `None` = not decided yet.
    ruler_axis: Option<bool>,
    /// Movement until the axis is decided.
    ruler_acc: Vec2,
    /// Expected jump after resetting the pointer.
    ruler_jump: Option<Vec2>,
    /// Frames still waited for after resetting the pointer.
    ruler_warp: u8,
    ruler_no_warp: bool,
    /// Last clicked track: target for pasting (Ctrl+V).
    paste_track: Option<usize>,
    /// Track currently being renamed (ID, text).
    rename: Option<(u32, String)>,
    /// Where the track context menu was opened.
    menu_pos: Option<egui::Pos2>,
    /// Selection box: start point and the selection before it (Ctrl/Shift
    /// extends).
    marquee: Option<(egui::Pos2, std::collections::BTreeSet<u32>)>,
    drag: Option<(u32, Drag)>,
    /// Clipboard (cut/copy).
    clipboard: Vec<(usize, Clip)>,
    /// State before dragging (for “no overlap”).
    before_drag: Option<Clip>,
    /// First audio block: take over the grid as soon as the analysis is done.
    pending_grid: Option<u32>,
    /// Beats of the audio blocks in the visible range (grid follows the audio).
    marks: Vec<Mark>,
    /// Position (timeline, ms) of the last right-click on a block.
    menu_clip_ms: Option<f64>,
}

/// Beat of an audio block on the timeline.
#[derive(Clone, Copy)]
struct Mark {
    ms: f64,
    down: bool,
    /// Bar within the piece (from 1) and piece (from 0) with the number of
    /// pieces.
    bar: i64,
    seg: usize,
    segs: usize,
    beat_ms: f64,
}

impl Default for TimelineView {
    fn default() -> Self {
        Self {
            zoom: 0.05,
            scroll_ms: 0.0,
            scroll_y: 0.0,
            ruler_grab: None,
            ruler_axis: None,
            ruler_acc: Vec2::ZERO,
            ruler_jump: None,
            ruler_warp: 0,
            ruler_no_warp: false,
            paste_track: None,
            rename: None,
            menu_pos: None,
            marquee: None,
            drag: None,
            clipboard: Vec::new(),
            before_drag: None,
            pending_grid: None,
            marks: Vec::new(),
            menu_clip_ms: None,
        }
    }
}

/// Time as “00m40s86” (hundredths).
/// Height of the block area of a track (without sub-track).
fn track_h(tr: &dmxcraft_core::timeline::Track) -> f32 {
    if tr
        .clips
        .iter()
        .any(|c| matches!(c.content, ClipContent::Audio { .. }))
    {
        AUDIO_TRACK_H
    } else {
        TRACK_H
    }
}

fn row_h(tr: &dmxcraft_core::timeline::Track) -> f32 {
    track_h(tr) + if tr.expanded { LANE_H } else { 0.0 }
}

pub fn fmt_ms(ms: f64) -> String {
    let ms = ms.max(0.0) as u64;
    format!(
        "{:02}m{:02}s{:02}",
        ms / 60_000,
        ms / 1000 % 60,
        ms / 10 % 100
    )
}

/// Selected SUPER SCENE (exactly one scene selected).
pub fn super_scene(e: &Engine, st: &AppState) -> Option<SceneId> {
    if st.selected_scenes.len() != 1 {
        return None;
    }
    let id = *st.selected_scenes.iter().next()?;
    e.show().scene(id)?.timeline().map(|_| id)
}

impl TimelineView {
    pub fn ui(&mut self, ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState, scene: SceneId) {
        let Some(mut tl) = e.show().scene(scene).and_then(|s| s.timeline()).cloned() else {
            return;
        };
        let before = tl.clone();
        // Live: only view the timeline (play, navigate), do not edit
        let live = st.programmer_mode == crate::state::ProgrammerMode::Live;
        let locked = st.locked || live;
        // running playback: the play position follows the engine
        let running = e.timeline_running(scene);
        if let Some(t) = e.timeline_time(scene) {
            st.timeline_cursor = t;
        }
        if running {
            ui.ctx().request_repaint();
        }
        if !locked {
            self.pending_grid(ui.ctx(), &mut tl, st);
        }

        // Toolbar
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            super::programmer::mode_switch(ui, &mut st.programmer_mode, st.locked);
            super::programmer::tool_separator(ui);
            let play = ui
                .add(
                    IconButton::new(if running { Icon::Pause } else { Icon::Play })
                        .size(34.0)
                        .flat(),
                )
                .on_hover_text(t("Play scene (Space: play/pause)"));
            let space = ui.input(|i| i.key_pressed(egui::Key::Space))
                && ui.ctx().memory(|m| m.focused().is_none());
            if play.clicked() || space {
                toggle_play(e, scene, st.timeline_cursor);
            }
            super::programmer::tool_separator(ui);
            let toggle = |ui: &mut egui::Ui, on: &mut bool, icon: Icon, tip: &str| {
                if ui
                    .add(
                        IconButton::new(icon)
                            .size(34.0)
                            .flat()
                            .active(*on)
                            .enabled(!locked),
                    )
                    .on_hover_text(tip)
                    .clicked()
                {
                    *on = !*on;
                }
            };
            let (mut lp, mut st_) = (tl.loop_resize, tl.stretch_resize);
            toggle(ui, &mut lp, Icon::Cursor, t("Loop block when resizing"));
            toggle(
                ui,
                &mut st_,
                Icon::ArrowsH,
                t("Stretch block when resizing"),
            );
            // the two exclude each other
            if lp != tl.loop_resize && lp {
                st_ = false;
            } else if st_ != tl.stretch_resize && st_ {
                lp = false;
            }
            if !lp && !st_ {
                lp = true;
            }
            tl.loop_resize = lp;
            tl.stretch_resize = st_;
            toggle(
                ui,
                &mut tl.allow_overlap,
                Icon::Stack,
                t("Allow block overlap"),
            );
            super::programmer::tool_separator(ui);
            toggle(ui, &mut tl.auto_scroll, Icon::FollowRight, t("Auto scroll"));
            toggle(ui, &mut tl.snap, Icon::Magnet, t("Snap to grid"));
            let grid = ui
                .add(IconButton::new(Icon::GridFour).size(34.0).flat())
                .on_hover_text(t("Grid options"));
            menu::dropdown(&grid, |ui| grid_options(ui, &mut tl, locked));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(fmt_ms(st.timeline_cursor))
                        .monospace()
                        .color(theme::text()),
                );
            });
        });
        ui.add_space(theme::SP_XS);

        let area = ui.available_rect_before_wrap();
        ui.allocate_rect(area, Sense::hover());
        let p = ui.painter_at(area);
        p.rect_filled(area, 0.0, theme::bg());
        let lanes = Rect::from_min_max(
            pos2(area.left() + HEADER_W, area.top() + ZOOMBAR_H + RULER_H),
            area.max,
        );
        let total = (tl.length_ms() as f64 + 30_000.0).max(60_000.0);
        // Mouse wheel: Ctrl = zoom, Alt/Shift = horizontal, otherwise vertical
        if ui.rect_contains_pointer(area) {
            let (scroll, zoom, alt, shift, pointer) = ui.input(|i| {
                (
                    i.smooth_scroll_delta,
                    i.zoom_delta(),
                    i.modifiers.alt,
                    i.modifiers.shift,
                    i.pointer.hover_pos(),
                )
            });
            if (zoom - 1.0).abs() > 1e-4 {
                // egui turns Ctrl+mouse wheel into a zoom factor
                let x = pointer.map_or(lanes.left(), |q| q.x.max(lanes.left()));
                self.zoom_at(zoom, x, lanes);
            } else if alt || shift || scroll.x != 0.0 {
                let d = scroll.x + if alt || shift { scroll.y } else { 0.0 };
                self.scroll_ms -= f64::from(d / self.zoom);
            } else {
                self.scroll_y -= scroll.y;
            }
        }
        let visible_ms = f64::from(lanes.width() / self.zoom);
        // Auto-scroll keeps the play position visible
        if running && tl.auto_scroll {
            let c = st.timeline_cursor;
            if c > self.scroll_ms + visible_ms * 0.9 || c < self.scroll_ms {
                self.scroll_ms = c - visible_ms * 0.1;
            }
        }
        self.scroll_ms = self.scroll_ms.clamp(0.0, (total - visible_ms).max(0.0));

        self.marks = audio_marks(
            &tl,
            st,
            self.scroll_ms - visible_ms,
            self.scroll_ms + 2.0 * visible_ms,
        );
        self.zoom_bar(ui, area, lanes, total);
        self.ruler(ui, &p, area, lanes, &tl, e, scene, st);

        // Tracks
        let content_h: f32 = tl.tracks.iter().map(row_h).sum::<f32>() + 30.0;
        self.scroll_y = self
            .scroll_y
            .clamp(0.0, (content_h - lanes.height()).max(0.0));
        let clip_area = Rect::from_min_max(pos2(area.left(), lanes.top()), area.max);
        let tp = ui.painter_at(clip_area);
        // Do not draw track headers and blocks over the ruler/zoom bar
        let outer_clip = ui.clip_rect();
        ui.set_clip_rect(clip_area.intersect(outer_clip));
        // Blocks only to the right of the track headers (also when scrolling
        // horizontally)
        let lanes_clip = Rect::from_min_max(lanes.min, area.max).intersect(outer_clip);
        let lp = ui.painter_at(lanes_clip);
        self.grid_lines(&lp, lanes, &tl);
        let mut y = lanes.top() - self.scroll_y;
        let mut rows: Vec<(usize, f32, f32)> = Vec::new();
        for (i, tr) in tl.tracks.iter().enumerate() {
            let h = row_h(tr);
            rows.push((i, y, h));
            y += h;
        }
        let add_y = y;
        let track_at = |py: f32| {
            rows.iter()
                .find(|(_, ty, h)| py >= *ty && py < ty + h)
                .map(|(i, _, _)| *i)
        };

        // Background of the tracks (click clears the selection, right-click: menu)
        let bg = ui.interact(
            clip_area,
            ui.scope_id().with("tl_bg"),
            Sense::click_and_drag(),
        );
        if bg.clicked() {
            st.timeline_sel.clear();
            if let Some(q) = bg.interact_pointer_pos() {
                self.paste_track = track_at(q.y);
            }
            // A click into the tracks sets the play position
            if let Some(q) = bg.interact_pointer_pos().filter(|q| q.x >= lanes.left()) {
                let ms = self.snap(self.ms_at(q.x, lanes).max(0.0), &tl, &[], 0);
                st.timeline_cursor = ms;
                e.seek_timeline(scene, ms);
            }
        }
        // Remember the position of the right-click (when clicking into the menu the
        // pointer is elsewhere)
        if bg.secondary_clicked() {
            self.menu_pos = bg.interact_pointer_pos();
            if let Some(q) = self.menu_pos {
                self.paste_track = track_at(q.y);
            }
        }
        let menu_pos = self.menu_pos;
        let mut paste_at: Option<(usize, u64)> = None;
        menu::context(&bg, |ui| {
            menu::begin(ui);
            let at = menu_pos
                .map(|q| {
                    (
                        track_at(q.y).unwrap_or(0),
                        self.ms_at(q.x, lanes).max(0.0) as u64,
                    )
                })
                .unwrap_or((0, st.timeline_cursor as u64));
            if menu::item(
                ui,
                MenuItem::new(t("Paste"))
                    .icon(Icon::Paste)
                    .shortcut("Ctrl+V"),
            ) {
                paste_at = Some(at);
            }
            if menu::item(ui, MenuItem::new(t("Add audio…")).icon(Icon::Music)) {
                let first = !has_audio(&tl);
                if let Some(id) = add_audio_dialog(&mut tl, at.0, at.1) {
                    if first {
                        self.pending_grid = Some(id);
                    }
                }
            }
            if menu::item(ui, MenuItem::new(t("Add video…")).icon(Icon::Camera)) {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter(t("Video"), &VIDEO_EXT)
                    .pick_file()
                {
                    let id = add_video(&mut tl, at.0, at.1, &path.to_string_lossy()).0;
                    st.timeline_sel = [id].into();
                }
            }
            if menu::item(ui, MenuItem::new(t("Add OBS action")).icon(Icon::Camera)) {
                let first = st.obs.scenes().into_iter().next().unwrap_or_default();
                let id = tl.add_clip(
                    at.0,
                    ClipContent::Obs(dmxcraft_core::obs::ObsAction::Scene(first)),
                    at.1,
                    1000,
                );
                st.timeline_sel = [id].into();
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Grid: time")).icon(Icon::Clock)) {
                tl.grid = GridMode::Time;
            }
            if menu::item(ui, MenuItem::new(t("Grid: beats")).icon(Icon::MusicNote)) {
                tl.grid = GridMode::Beat;
            }
            if menu::item(ui, MenuItem::new(t("Show all")).icon(Icon::ZoomFit)) {
                let len = tl.length_ms().max(10_000) as f64;
                self.zoom = (lanes.width() / (len * 1.05) as f32).clamp(0.002, 2.0);
                self.scroll_ms = 0.0;
            }
        });
        if let Some((track, at)) = paste_at {
            self.paste(&mut tl, st, track, at);
        }

        let mut header_action: Option<(usize, HeaderAction)> = None;
        for &(i, ty, h) in &rows {
            let row = Rect::from_min_max(pos2(area.left(), ty), pos2(area.right(), ty + h));
            if row.bottom() < lanes.top() || row.top() > area.bottom() {
                continue;
            }
            // colored track: very light tint of the grid behind it
            if let Some(c) = tl.tracks[i].color {
                lp.rect_filled(
                    Rect::from_min_max(pos2(lanes.left(), row.top()), row.max),
                    0.0,
                    theme::rgb(c).gamma_multiply(0.06),
                );
            }
            // Row line
            tp.line_segment(
                [
                    pos2(area.left(), row.bottom()),
                    pos2(area.right(), row.bottom()),
                ],
                Stroke::new(1.0, theme::border()),
            );
            if let Some(a) =
                track_header(ui, &tp, row, i, &mut tl.tracks[i], locked, &mut self.rename)
            {
                header_action = Some((i, a));
            }
        }
        // “+ Track” below the last track
        let add_rect = Rect::from_min_size(
            pos2(area.left() + 6.0, add_y + 4.0),
            Vec2::new(HEADER_W - 12.0, 22.0),
        );
        if add_rect.bottom() < area.bottom() && add_rect.top() > lanes.top() - 10.0 {
            let r = ui
                .put(add_rect, egui::Button::new(t("+ Track")).small())
                .on_hover_text(t("Add track"));
            if r.clicked() && !locked {
                tl.add_track(None);
            }
        }
        // Blocks: draw and click only in the track area to the right of the track
        // headers
        let names = |id: SceneId| {
            e.show().scene(id).map(|s| {
                (
                    s.name.clone(),
                    s.color
                        .map_or_else(|| theme::zone_color(e.show(), s.zone), theme::rgb),
                    s.duration_ms(),
                )
            })
        };
        let mut clicked_clip: Option<(u32, bool)> = None;
        let mut clip_rects: Vec<(u32, Rect)> = Vec::new();
        let mut clip_menu: Option<(u32, ClipAction)> = None;
        let mut pointer_track = None;
        let mut double: Option<SceneId> = None;
        let snap_targets: Vec<u64> = tl
            .tracks
            .iter()
            .flat_map(|t| t.clips.iter().flat_map(|c| [c.start_ms, c.end_ms()]))
            .chain(std::iter::once(st.timeline_cursor as u64))
            .collect();
        let pointer = ui.input(|i| i.pointer.interact_pos());
        if let Some(q) = pointer {
            pointer_track = track_at(q.y);
        }
        ui.set_clip_rect(lanes_clip);
        for &(ti, ty, _) in &rows {
            let tr_locked = locked;
            let expanded = tl.tracks[ti].expanded;
            let th = track_h(&tl.tracks[ti]);
            let ids: Vec<u32> = tl.tracks[ti].clips.iter().map(|c| c.id).collect();
            for cid in ids {
                let Some(c) = tl.tracks[ti].clips.iter().find(|c| c.id == cid).cloned() else {
                    continue;
                };
                let x0 = self.x_at(c.start_ms as f64, lanes);
                let x1 = self.x_at(c.end_ms() as f64, lanes);
                if x1 < lanes.left() || x0 > lanes.right() {
                    continue;
                }
                let r =
                    Rect::from_min_max(pos2(x0, ty + 2.0), pos2(x1.max(x0 + 3.0), ty + th - 2.0));
                clip_rects.push((cid, r));
                let sel = st.timeline_sel.contains(&cid);
                let (name, color, scene_ms) = match &c.content {
                    ClipContent::Scene(id) => names(*id)
                        .unwrap_or_else(|| (t("Deleted").to_string(), theme::text_faint(), 1000)),
                    ClipContent::Audio { path, .. } => (
                        std::path::Path::new(path)
                            .file_name()
                            .map_or(path.clone(), |n| n.to_string_lossy().into_owned()),
                        Color32::from_gray(150),
                        0,
                    ),
                    ClipContent::Video { path, .. } => (
                        std::path::Path::new(path)
                            .file_name()
                            .map_or(path.clone(), |n| n.to_string_lossy().into_owned()),
                        VIDEO_COLOR,
                        0,
                    ),
                    ClipContent::Obs(a) => (obs_label(a), OBS_COLOR, 0),
                };
                paint_clip(ui, &lp, r, &c, &name, color, sel, scene_ms, st);
                // Sub-track: dimmer or volume as a line
                if expanded {
                    let lane = Rect::from_min_max(
                        pos2(r.left(), ty + th + 2.0),
                        pos2(r.right(), ty + th + LANE_H - 4.0),
                    );
                    lp.rect_filled(lane, 2.0, color.gamma_multiply(0.18));
                    let ly = lane.bottom() - lane.height() * c.level;
                    lp.line_segment(
                        [pos2(lane.left(), ly), pos2(lane.right(), ly)],
                        Stroke::new(2.0, color),
                    );
                    let lr = ui.interact(
                        lane,
                        ui.scope_id().with(("lane", cid)),
                        if tr_locked {
                            Sense::hover()
                        } else {
                            Sense::drag()
                        },
                    );
                    if let Some(q) = lr.interact_pointer_pos().filter(|_| lr.dragged()) {
                        if let Some(cm) = tl.clip_mut(cid) {
                            cm.level = ((lane.bottom() - q.y) / lane.height()).clamp(0.0, 1.0);
                        }
                    }
                    lr.on_hover_text(format!("{:.0} %", c.level * 100.0));
                }
                if tr_locked {
                    continue;
                }
                // Zones: edges (length), top corners (fade in/out), area
                let sense = Sense::click_and_drag();
                let body = ui.interact(r, ui.scope_id().with(("clip", cid)), sense);
                let left = ui.interact(
                    Rect::from_min_max(r.min, pos2(r.left() + EDGE, r.bottom())),
                    ui.scope_id().with(("cl", cid)),
                    Sense::drag(),
                );
                let right = ui.interact(
                    Rect::from_min_max(pos2(r.right() - EDGE, r.top()), r.max),
                    ui.scope_id().with(("cr", cid)),
                    Sense::drag(),
                );
                let fin_x = self.x_at((c.start_ms + u64::from(c.fade_in_ms)) as f64, lanes);
                let fout_x = self.x_at(
                    (c.end_ms() - u64::from(c.fade_out_ms).min(c.duration_ms)) as f64,
                    lanes,
                );
                let fin = ui.interact(
                    Rect::from_center_size(
                        pos2(fin_x.max(r.left() + 5.0), r.top() + 5.0),
                        Vec2::splat(10.0),
                    ),
                    ui.scope_id().with(("fi", cid)),
                    Sense::drag(),
                );
                let fout = ui.interact(
                    Rect::from_center_size(
                        pos2(fout_x.min(r.right() - 5.0), r.top() + 5.0),
                        Vec2::splat(10.0),
                    ),
                    ui.scope_id().with(("fo", cid)),
                    Sense::drag(),
                );
                // Trimming: horizontal arrow; fades: diagonal like the fade line (on
                // Windows all horizontal pointers look the same)
                for (resp, cursor) in [
                    (&left, egui::CursorIcon::ResizeHorizontal),
                    (&right, egui::CursorIcon::ResizeHorizontal),
                    (&fin, egui::CursorIcon::ResizeNeSw),
                    (&fout, egui::CursorIcon::ResizeNwSe),
                ] {
                    if resp.hovered() || resp.dragged() {
                        ui.ctx().set_cursor_icon(cursor);
                    }
                }
                if fin.hovered() || fout.hovered() {
                    for x in [fin_x.max(r.left() + 5.0), fout_x.min(r.right() - 5.0)] {
                        lp.circle_filled(pos2(x, r.top() + 5.0), 4.0, Color32::WHITE);
                    }
                }
                let start_drag = |resp: &egui::Response, d: Drag| resp.drag_started().then_some(d);
                let pointer_ms = pointer.map(|q| self.ms_at(q.x, lanes));
                if let Some(d) = start_drag(&fin, Drag::FadeIn)
                    .or(start_drag(&fout, Drag::FadeOut))
                    .or(start_drag(&left, Drag::Left))
                    .or(start_drag(&right, Drag::Right))
                    .or(start_drag(
                        &body,
                        Drag::Move {
                            grab_ms: pointer_ms.unwrap_or(0.0) - c.start_ms as f64,
                        },
                    ))
                {
                    self.drag = Some((cid, d));
                    self.before_drag = Some(c.clone());
                    if !st.timeline_sel.contains(&cid) {
                        st.timeline_sel = [cid].into();
                    }
                }
                if body.secondary_clicked() {
                    self.menu_clip_ms = body.interact_pointer_pos().map(|q| self.ms_at(q.x, lanes));
                }
                if body.clicked() {
                    clicked_clip =
                        Some((cid, ui.input(|i| i.modifiers.command || i.modifiers.shift)));
                }
                if body.double_clicked() {
                    double = c.scene();
                }
                menu::context(&body, |ui| {
                    menu::begin(ui);
                    for (a, label, icon, key) in [
                        (ClipAction::Split, t("Split block"), Icon::Scissors, ""),
                        (ClipAction::Cut, t("Cut"), Icon::Scissors, "Ctrl+X"),
                        (ClipAction::Copy, t("Copy"), Icon::Copy, "Ctrl+C"),
                        (ClipAction::Duplicate, t("Duplicate"), Icon::Copy, "Ctrl+D"),
                        (ClipAction::Delete, t("Delete"), Icon::Trash, "Del"),
                    ] {
                        if menu::item(ui, MenuItem::new(label).icon(icon).shortcut(key)) {
                            clip_menu = Some((cid, a));
                        }
                    }
                    menu::separator(ui);
                    if c.scene().is_some()
                        && menu::item(ui, MenuItem::new(t("Edit scene")).icon(Icon::Edit))
                    {
                        clip_menu = Some((cid, ClipAction::EditScene));
                    }
                    if menu::item(ui, MenuItem::new(t("Reset speed")).icon(Icon::Reset)) {
                        clip_menu = Some((cid, ClipAction::ResetSpeed));
                    }
                    if matches!(c.content, ClipContent::Audio { .. }) {
                        menu::separator(ui);
                        for (a, label, icon) in [
                            (
                                ClipAction::ApplyGrid,
                                t("Apply grid to timeline"),
                                Icon::MusicNote,
                            ),
                            (
                                ClipAction::Grid(GridEdit::Double),
                                t("Double tempo"),
                                Icon::MusicNote,
                            ),
                            (
                                ClipAction::Grid(GridEdit::Halve),
                                t("Halve tempo"),
                                Icon::MusicNote,
                            ),
                            (
                                ClipAction::Grid(GridEdit::Earlier),
                                t("Move downbeat one beat earlier"),
                                Icon::MusicNote,
                            ),
                            (
                                ClipAction::Grid(GridEdit::Later),
                                t("Move downbeat one beat later"),
                                Icon::MusicNote,
                            ),
                        ] {
                            if menu::item(ui, MenuItem::new(label).icon(icon)) {
                                clip_menu = Some((cid, a));
                            }
                        }
                        if matches!(c.content, ClipContent::Audio { grid: Some(_), .. })
                            && menu::item(
                                ui,
                                MenuItem::new(t("Restore detected grid")).icon(Icon::Reset),
                            )
                        {
                            clip_menu = Some((cid, ClipAction::Grid(GridEdit::Reset)));
                        }
                    }
                });
            }
        }

        ui.set_clip_rect(clip_area.intersect(outer_clip));
        // Evaluate the drag
        if let (Some((cid, d)), Some(q)) = (self.drag, pointer) {
            let down = ui.input(|i| i.pointer.primary_down());
            if !down {
                self.drag = None;
                self.before_drag = None;
            } else if let Some((ti, c)) = tl.clip(cid).map(|(t, c)| (t, c.clone())) {
                let ms = self.ms_at(q.x, lanes).max(0.0);
                let snap = |v: f64, tl: &Timeline, z: &Self| z.snap(v, tl, &snap_targets, cid);
                let mut n = c.clone();
                let mut target_track = ti;
                match d {
                    Drag::Move { grab_ms } => {
                        let s = snap((ms - grab_ms).max(0.0), &tl, self);
                        // the end may snap too
                        let e_ = snap(s + c.duration_ms as f64, &tl, self) - c.duration_ms as f64;
                        n.start_ms = if (e_ - s).abs() > 0.5
                            && (e_ - (ms - grab_ms)).abs() < (s - (ms - grab_ms)).abs()
                        {
                            e_.max(0.0) as u64
                        } else {
                            s as u64
                        };
                        if let Some(tt) = pointer_track {
                            target_track = tt;
                        }
                    }
                    Drag::Left => {
                        let end = c.end_ms();
                        let s = (snap(ms, &tl, self) as u64).min(end - 40);
                        let delta = s as i64 - c.start_ms as i64;
                        n.start_ms = s;
                        n.duration_ms = end - s;
                        if let Some(offset_ms) = n.content.offset_mut() {
                            *offset_ms = (*offset_ms as i64 + delta).max(0) as u64;
                        }
                        self.apply_resize(&mut n, &c, &tl, e);
                    }
                    Drag::Right => {
                        let end = (snap(ms, &tl, self) as u64).max(c.start_ms + 40);
                        n.duration_ms = end - c.start_ms;
                        if let Some((offset_ms, length_ms)) = n.content.media_len() {
                            // do not extend audio/video beyond the end of the file
                            if length_ms > 0 {
                                n.duration_ms = n
                                    .duration_ms
                                    .min(length_ms.saturating_sub(offset_ms).max(40));
                            }
                        }
                        self.apply_resize(&mut n, &c, &tl, e);
                    }
                    Drag::FadeIn => {
                        n.fade_in_ms =
                            (ms - c.start_ms as f64).clamp(0.0, c.duration_ms as f64) as u32;
                    }
                    Drag::FadeOut => {
                        n.fade_out_ms =
                            (c.end_ms() as f64 - ms).clamp(0.0, c.duration_ms as f64) as u32;
                    }
                }
                let group = matches!(d, Drag::Move { .. })
                    && st.timeline_sel.len() > 1
                    && st.timeline_sel.contains(&cid);
                if group {
                    // several selected blocks: all by the same offset
                    tl.move_group(
                        &st.timeline_sel,
                        n.start_ms as i64 - c.start_ms as i64,
                        target_track as isize - ti as isize,
                    );
                } else if tl.allow_overlap
                    || !tl.overlaps(target_track, n.start_ms, n.end_ms(), cid)
                {
                    if target_track != ti {
                        tl.move_clip_to_track(cid, target_track);
                    }
                    if let Some(cm) = tl.clip_mut(cid) {
                        *cm = n;
                    }
                }
            }
        }

        // Drag a scene in from the scene list
        if let Some(dragged) = egui::DragAndDrop::payload::<SceneId>(ui.ctx()) {
            if let Some(q) = pointer.filter(|q| clip_area.contains(*q)) {
                let ti = track_at(q.y).unwrap_or(tl.tracks.len());
                let at = self.snap(self.ms_at(q.x, lanes).max(0.0), &tl, &snap_targets, 0);
                let dur = e
                    .show()
                    .scene(*dragged)
                    .map_or(DEFAULT_CLIP_MS, |s| s.duration_ms().max(100));
                let ty = rows.iter().find(|r| r.0 == ti).map_or(add_y, |r| r.1);
                let th = tl.tracks.get(ti).map_or(TRACK_H, track_h);
                let ghost = Rect::from_min_size(
                    pos2(self.x_at(at, lanes), ty + 2.0),
                    Vec2::new(dur as f32 * self.zoom, th - 4.0),
                );
                lp.rect_stroke(
                    ghost,
                    3.0,
                    Stroke::new(2.0, theme::accent()),
                    StrokeKind::Inside,
                );
                if ui.input(|i| i.pointer.any_released()) && *dragged != scene && !locked {
                    if let Some(sc) = e.show().scene(*dragged) {
                        let id = tl.add_scene_clip(ti, sc, at as u64);
                        st.timeline_sel = [id].into();
                    }
                    egui::DragAndDrop::clear_payload(ui.ctx());
                }
            }
        }
        // Audio files from the file manager
        let dropped = ui.input(|i| i.raw.dropped_files.clone());
        if !dropped.is_empty() && !locked {
            let q = ui
                .input(|i| i.pointer.hover_pos())
                .unwrap_or(lanes.center());
            if clip_area.contains(q) {
                let ti = track_at(q.y).unwrap_or(tl.tracks.len());
                let mut at = self.ms_at(q.x, lanes).max(0.0) as u64;
                let first = !has_audio(&tl);
                for f in dropped {
                    let path = f.path();
                    if path.as_os_str().is_empty() {
                        continue;
                    }
                    if is_video(path) {
                        at += add_video(&mut tl, ti, at, &path.to_string_lossy()).1;
                    } else {
                        let (id, len) = add_audio(&mut tl, ti, at, &path.to_string_lossy());
                        if first && self.pending_grid.is_none() {
                            self.pending_grid = Some(id);
                        }
                        at += len;
                    }
                }
            }
        }

        // Selection and menu actions
        if let Some((cid, _)) = clicked_clip {
            self.paste_track = tl.clip(cid).map(|(t, _)| t);
        }
        // Selection box: click and drag in the track area
        if bg.drag_started() && !locked {
            let add = ui.input(|i| i.modifiers.command || i.modifiers.shift);
            let origin = ui.input(|i| i.pointer.press_origin());
            if let Some(o) = origin {
                let keep = if add {
                    st.timeline_sel.clone()
                } else {
                    Default::default()
                };
                self.marquee = Some((o, keep));
            }
        }
        if let Some((o, keep)) = &self.marquee {
            if let (true, Some(q)) = (bg.dragged(), pointer) {
                let rect = Rect::from_two_pos(*o, q);
                let mut sel = keep.clone();
                sel.extend(
                    clip_rects
                        .iter()
                        .filter(|(_, r)| r.intersects(rect))
                        .map(|(id, _)| *id),
                );
                st.timeline_sel = sel;
                let shown = rect.intersect(clip_area);
                lp.rect_filled(shown, 0.0, theme::accent().gamma_multiply(0.12));
                tp.rect_stroke(
                    shown,
                    0.0,
                    Stroke::new(1.0, theme::accent()),
                    StrokeKind::Inside,
                );
            } else {
                self.marquee = None;
            }
        }
        if let Some((cid, add)) = clicked_clip {
            if add {
                if !st.timeline_sel.remove(&cid) {
                    st.timeline_sel.insert(cid);
                }
            } else {
                st.timeline_sel = [cid].into();
            }
        }
        if let Some((cid, a)) = clip_menu {
            if !st.timeline_sel.contains(&cid) {
                st.timeline_sel = [cid].into();
            }
            self.clip_action(&mut tl, st, a, cid);
            if matches!(a, ClipAction::Cut | ClipAction::Copy) {
                mark_clipboard(ui.ctx());
            }
        }
        if let Some(id) = double {
            st.selected_scenes = [id].into();
        }
        // Keyboard
        // (Ctrl+X/C/V come from egui as cut/copy/paste events, not as keys)
        // A click into the timeline makes it the keyboard target
        if ui.input(|i| i.pointer.any_pressed())
            && ui
                .input(|i| i.pointer.interact_pos())
                .is_some_and(|q| area.contains(q))
        {
            st.focus = crate::state::Focus::Timeline;
        }
        let focus_free =
            !ui.ctx().egui_wants_keyboard_input() && st.focus == crate::state::Focus::Timeline;
        if focus_free && !locked {
            let (del, cut, copy, paste, dup) = ui.input(|i| {
                let ev = |f: fn(&egui::Event) -> bool| i.events.iter().any(f);
                let key = |k| i.modifiers.command && i.key_pressed(k);
                (
                    i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace),
                    ev(|e| matches!(e, egui::Event::Cut)) || key(egui::Key::X),
                    ev(|e| matches!(e, egui::Event::Copy)) || key(egui::Key::C),
                    ev(|e| matches!(e, egui::Event::Paste(_))) || key(egui::Key::V),
                    key(egui::Key::D),
                )
            });
            let first = st.timeline_sel.iter().next().copied();
            if let Some(cid) = first {
                if del {
                    self.clip_action(&mut tl, st, ClipAction::Delete, cid);
                } else if cut {
                    self.clip_action(&mut tl, st, ClipAction::Cut, cid);
                    mark_clipboard(ui.ctx());
                } else if copy {
                    self.clip_action(&mut tl, st, ClipAction::Copy, cid);
                    mark_clipboard(ui.ctx());
                } else if dup {
                    self.clip_action(&mut tl, st, ClipAction::Duplicate, cid);
                }
            }
            if paste {
                let track = self.paste_track.or(pointer_track).unwrap_or(0);
                self.paste(&mut tl, st, track, st.timeline_cursor as u64);
            }
        }

        ui.set_clip_rect(outer_clip);
        // Live: gray layer over everything – only view, do not edit
        if live {
            p.rect_filled(area, 0.0, theme::text_faint().gamma_multiply(0.32));
            let chip = p.layout_no_wrap(
                t("LIVE – view only").to_string(),
                egui::FontId::proportional(theme::FS_CAPTION),
                theme::text(),
            );
            let cr = Rect::from_min_size(
                pos2(
                    area.right() - chip.size().x - 22.0,
                    area.bottom() - chip.size().y - 18.0,
                ),
                chip.size() + Vec2::new(12.0, 6.0),
            );
            p.rect_filled(cr, 4.0, theme::raised());
            p.galley(cr.min + Vec2::new(6.0, 3.0), chip, theme::text());
        }
        // Play position
        let cx = self.x_at(st.timeline_cursor, lanes);
        if cx >= lanes.left() && cx <= lanes.right() {
            p.line_segment(
                [pos2(cx, area.top() + ZOOMBAR_H), pos2(cx, area.bottom())],
                Stroke::new(1.5, theme::OK),
            );
            let head = vec![
                pos2(cx - 6.0, area.top() + ZOOMBAR_H),
                pos2(cx + 6.0, area.top() + ZOOMBAR_H),
                pos2(cx, area.top() + ZOOMBAR_H + 9.0),
            ];
            p.add(Shape::convex_polygon(head, theme::OK, Stroke::NONE));
        }

        // Track actions only at the end: the rows above still point to the old
        // tracks
        if let Some((i, a)) = header_action.filter(|(i, _)| *i < tl.tracks.len()) {
            match a {
                HeaderAction::Toggle => tl.tracks[i].expanded = !tl.tracks[i].expanded,
                HeaderAction::ExpandAll => {
                    let all = tl.tracks.iter().all(|t| t.expanded);
                    for t in &mut tl.tracks {
                        t.expanded = !all;
                    }
                }
                HeaderAction::Insert => {
                    tl.add_track(Some(i));
                }
                HeaderAction::Delete => {
                    tl.tracks.remove(i);
                }
                HeaderAction::Duplicate => {
                    let mut copy = tl.tracks[i].clone();
                    copy.id = tl.next_id;
                    tl.next_id += 1;
                    for c in &mut copy.clips {
                        c.id = tl.next_id;
                        tl.next_id += 1;
                    }
                    tl.tracks.insert(i + 1, copy);
                }
            }
        }

        // Apply changes
        if tl != before && !locked {
            if let Some(sc) = e.show_mut().scene_mut(scene) {
                if let Some(t) = sc.timeline_mut() {
                    *t = tl;
                }
                sc.sync_timeline_length();
            }
        }
    }

    /// Zoom by the factor `f`, the time under `x` stays in place.
    fn zoom_at(&mut self, f: f32, x: f32, lanes: Rect) {
        let at = self.ms_at(x, lanes);
        self.zoom = (self.zoom * f).clamp(0.002, 2.0);
        self.scroll_ms = (at - f64::from((x - lanes.left()) / self.zoom)).max(0.0);
    }

    fn x_at(&self, ms: f64, lanes: Rect) -> f32 {
        lanes.left() + ((ms - self.scroll_ms) as f32) * self.zoom
    }

    fn ms_at(&self, x: f32, lanes: Rect) -> f64 {
        self.scroll_ms + f64::from((x - lanes.left()) / self.zoom)
    }

    /// Grid spacing (ms) matching the zoom.
    fn grid_step(&self, tl: &Timeline) -> f64 {
        match tl.grid {
            GridMode::Beat => {
                let beat = tl.beat_ms();
                let mut step = beat;
                while (step as f32) * self.zoom < 14.0 {
                    step *= 2.0;
                }
                while (step as f32) * self.zoom > 80.0 && step > beat / 8.0 {
                    step /= 2.0;
                }
                step
            }
            GridMode::Time => {
                const STEPS: [f64; 13] = [
                    10.0, 20.0, 40.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 5000.0, 10_000.0,
                    30_000.0, 60_000.0, 300_000.0,
                ];
                STEPS
                    .into_iter()
                    .find(|s| (*s as f32) * self.zoom >= 14.0)
                    .unwrap_or(600_000.0)
            }
        }
    }

    /// Snap to the grid, block edges and play position.
    fn snap(&self, v: f64, tl: &Timeline, targets: &[u64], _except: u32) -> f64 {
        if !tl.snap {
            return v.round();
        }
        let tol = f64::from(8.0 / self.zoom);
        let near = targets
            .iter()
            .map(|t| *t as f64)
            .filter(|t| (t - v).abs() < tol)
            .min_by(|a, b| (a - v).abs().total_cmp(&(b - v).abs()));
        if let Some(n) = near {
            return n;
        }
        if self.follows_audio(tl) {
            // next beat of the audio blocks (only bars when zoomed out)
            let bars_only = self
                .marks
                .first()
                .is_some_and(|m| (m.beat_ms as f32) * self.zoom < 8.0);
            if let Some(m) = self
                .marks
                .iter()
                .filter(|m| m.down || !bars_only)
                .min_by(|a, b| (a.ms - v).abs().total_cmp(&(b.ms - v).abs()))
            {
                return m.ms.round();
            }
        }
        let step = self.grid_step(tl);
        let off = if tl.grid == GridMode::Beat {
            f64::from(tl.offset_ms)
        } else {
            0.0
        };
        ((v - off) / step).round() * step + off
    }

    /// Length changed: loop (the scene repeats) or stretch (play faster/slower).
    fn apply_resize(&self, n: &mut Clip, old: &Clip, tl: &Timeline, e: &Engine) {
        if !tl.stretch_resize {
            return;
        }
        let Some(dur) = n
            .scene()
            .and_then(|s| e.show().scene(s))
            .map(|s| s.duration_ms())
        else {
            return;
        };
        if dur == 0 || old.duration_ms == 0 {
            return;
        }
        // same number of cycles in the new length
        let runs = old.duration_ms as f64 * f64::from(old.speed) / dur as f64;
        n.speed = ((runs * dur as f64) / n.duration_ms as f64).clamp(0.05, 20.0) as f32;
    }

    fn zoom_bar(&mut self, ui: &mut egui::Ui, area: Rect, lanes: Rect, total: f64) {
        let bar = Rect::from_min_max(
            pos2(lanes.left(), area.top() + 2.0),
            pos2(lanes.right(), area.top() + ZOOMBAR_H - 2.0),
        );
        let p = ui.painter();
        p.rect_filled(bar, 4.0, theme::raised());
        let visible = f64::from(lanes.width() / self.zoom);
        let a = bar.left() + (self.scroll_ms / total) as f32 * bar.width();
        let b = bar.left() + ((self.scroll_ms + visible) / total).min(1.0) as f32 * bar.width();
        let thumb = Rect::from_min_max(pos2(a, bar.top()), pos2(b.max(a + 8.0), bar.bottom()));
        let r = ui.interact(thumb, ui.scope_id().with("zoom_thumb"), Sense::drag());
        let rl = ui.interact(
            Rect::from_center_size(thumb.left_center(), Vec2::new(8.0, bar.height() + 4.0)),
            ui.scope_id().with("zoom_l"),
            Sense::drag(),
        );
        let rr = ui.interact(
            Rect::from_center_size(thumb.right_center(), Vec2::new(8.0, bar.height() + 4.0)),
            ui.scope_id().with("zoom_r"),
            Sense::drag(),
        );
        let ms_per_px = total / f64::from(bar.width());
        if rl.dragged() || rr.dragged() {
            // Handles at the ends change the visible range (zoom)
            let (mut s, mut e) = (self.scroll_ms, self.scroll_ms + visible);
            if rl.dragged() {
                s = (s + f64::from(rl.drag_delta().x) * ms_per_px).clamp(0.0, e - 500.0);
            } else {
                e = (e + f64::from(rr.drag_delta().x) * ms_per_px).max(s + 500.0);
            }
            self.scroll_ms = s;
            self.zoom = (lanes.width() / (e - s) as f32).clamp(0.002, 2.0);
        } else if r.dragged() {
            self.scroll_ms += f64::from(r.drag_delta().x) * ms_per_px;
        }
        let hot = r.hovered() || r.dragged() || rl.hovered() || rr.hovered();
        p.rect_filled(
            thumb,
            4.0,
            if hot {
                theme::text_dim()
            } else {
                theme::text_faint()
            },
        );
        for x in [thumb.left() + 3.0, thumb.right() - 3.0] {
            p.circle_filled(pos2(x, thumb.center().y), 3.0, theme::text());
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn ruler(
        &mut self,
        ui: &mut egui::Ui,
        p: &egui::Painter,
        area: Rect,
        lanes: Rect,
        tl: &Timeline,
        e: &mut Engine,
        scene: SceneId,
        st: &mut AppState,
    ) {
        let r = Rect::from_min_max(
            pos2(lanes.left(), area.top() + ZOOMBAR_H),
            pos2(lanes.right(), lanes.top()),
        );
        p.rect_filled(r, 0.0, theme::surface());
        let follow = self.follows_audio(tl);
        if follow {
            // Bars of the pieces: “piece · bar”, spacing at least 60 px; the start of
            // a piece takes precedence over the bar before it
            let mut labels: Vec<(f32, String)> = Vec::new();
            let mut last_seg = usize::MAX;
            for m in self.marks.iter().filter(|m| m.down) {
                let x = self.x_at(m.ms, lanes);
                if x < r.left() - 40.0 || x > r.right() {
                    continue;
                }
                let new_seg = m.seg != last_seg;
                last_seg = m.seg;
                let near = labels.last().is_some_and(|(lx, _)| x - lx < 60.0);
                if near && !new_seg {
                    continue;
                }
                if near {
                    labels.pop();
                }
                let label = if m.segs > 1 {
                    format!("{}·{}", m.seg + 1, m.bar)
                } else {
                    m.bar.to_string()
                };
                labels.push((x, label));
            }
            for (x, label) in labels {
                p.line_segment(
                    [pos2(x, r.bottom() - 6.0), pos2(x, r.bottom())],
                    Stroke::new(1.0, theme::text_faint()),
                );
                p.text(
                    pos2(x + 3.0, r.top() + 2.0),
                    egui::Align2::LEFT_TOP,
                    label,
                    egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                    theme::text_dim(),
                );
            }
        }
        // Labels at a sensible spacing
        let label_step = {
            let mut s = self.grid_step(tl);
            while (s as f32) * self.zoom < 70.0 {
                s *= 2.0;
            }
            s
        };
        // Beat grid: labels from the offset (bar 1)
        let off = if tl.grid == GridMode::Beat {
            f64::from(tl.offset_ms)
        } else {
            0.0
        };
        let start = ((self.scroll_ms - off) / label_step).floor() * label_step + off;
        let mut v = if follow { f64::INFINITY } else { start };
        while self.x_at(v, lanes) < r.right() {
            let x = self.x_at(v, lanes);
            if x >= r.left() {
                p.line_segment(
                    [pos2(x, r.bottom() - 6.0), pos2(x, r.bottom())],
                    Stroke::new(1.0, theme::text_faint()),
                );
                let label = match tl.grid {
                    GridMode::Time => fmt_ms(v),
                    GridMode::Beat => {
                        let beat = ((v - f64::from(tl.offset_ms)) / tl.beat_ms()).round() as i64;
                        format!("{}.{}", beat.div_euclid(4) + 1, beat.rem_euclid(4) + 1)
                    }
                };
                p.text(
                    pos2(x + 3.0, r.top() + 2.0),
                    egui::Align2::LEFT_TOP,
                    label,
                    egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                    theme::text_dim(),
                );
            }
            v += label_step;
        }
        // Click sets the play position; dragging (the mouse pointer disappears):
        // up/down zooms, left/right scrolls
        let resp = ui.interact(r, ui.scope_id().with("ruler"), Sense::click_and_drag());
        // The playhead (triangle) stays grabbable and lies on top of the ruler
        let cx = self.x_at(st.timeline_cursor, lanes);
        let head = ui.interact(
            Rect::from_min_max(pos2(cx - 7.0, r.top()), pos2(cx + 7.0, r.bottom())).intersect(r),
            ui.scope_id().with("playhead"),
            Sense::drag(),
        );
        if head.hovered() || head.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        } else if resp.hovered() && self.ruler_grab.is_none() {
            // own magnifier: the “zoom-in” system pointer is missing in many themes
            if let Some(q) = ui.input(|i| i.pointer.hover_pos()) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::None);
                let fg = ui.ctx().layer_painter(egui::LayerId::new(
                    egui::Order::Tooltip,
                    ui.scope_id().with("ruler_cursor"),
                ));
                let ir = Rect::from_center_size(q, Vec2::splat(22.0));
                Icon::Magnifier.paint(&fg, ir, theme::text());
            }
        }
        if head.dragged() {
            // no sound while dragging, only after releasing at the new position
            st.audio.scrubbing = true;
        }
        if let Some(q) = head.interact_pointer_pos().filter(|_| head.dragged()) {
            let ms = self.snap(self.ms_at(q.x, lanes).max(0.0), tl, &[], 0);
            st.timeline_cursor = ms;
            e.seek_timeline(scene, ms);
        }
        if resp.clicked() {
            if let Some(q) = resp.interact_pointer_pos() {
                let ms = self.snap(self.ms_at(q.x, lanes).max(0.0), tl, &[], 0);
                st.timeline_cursor = ms;
                e.seek_timeline(scene, ms);
            }
        }
        if resp.drag_started() {
            self.ruler_grab = resp.interact_pointer_pos();
            self.ruler_jump = None;
            self.ruler_warp = 0;
            self.ruler_no_warp = false;
            self.ruler_axis = None;
            self.ruler_acc = Vec2::ZERO;
        }
        if let Some(origin) = self.ruler_grab {
            if resp.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::None);
                // Movement as usual from egui; far from the start point the (invisible)
                // pointer is reset so it does not wander onto another screen. The jump is
                // subtracted.
                let (mut d, q) = ui.input(|i| (i.pointer.delta(), i.pointer.latest_pos()));
                if let Some(jump) = self.ruler_jump {
                    if (d - jump).length() < jump.length() * 0.5 {
                        d -= jump;
                        self.ruler_jump = None;
                        self.ruler_warp = 0;
                    } else {
                        self.ruler_warp = self.ruler_warp.saturating_sub(1);
                        if self.ruler_warp == 0 {
                            // Warping is not supported (e.g. Wayland)
                            self.ruler_jump = None;
                            self.ruler_no_warp = true;
                        }
                    }
                } else if let Some(q) = q.filter(|q| (*q - origin).length() > 150.0) {
                    if !self.ruler_no_warp {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::CursorPosition(origin));
                        self.ruler_jump = Some(origin - q);
                        self.ruler_warp = 20;
                    }
                }
                // always only one axis: the direction that clearly dominated last
                // (decaying sum of the recent movement); switching is possible at any time
                self.ruler_acc = self.ruler_acc * 0.85 + d;
                let (ax, ay) = (self.ruler_acc.x.abs(), self.ruler_acc.y.abs());
                match self.ruler_axis {
                    None if self.ruler_acc.length() >= 8.0 => {
                        self.ruler_axis = Some(ax >= ay);
                        d = self.ruler_acc;
                    }
                    Some(true) if ay >= 10.0 && ay > ax * 2.0 => self.ruler_axis = Some(false),
                    Some(false) if ax >= 10.0 && ax > ay * 2.0 => self.ruler_axis = Some(true),
                    _ => {}
                }
                match self.ruler_axis {
                    Some(true) => {
                        self.scroll_ms = (self.scroll_ms - f64::from(d.x / self.zoom)).max(0.0);
                    }
                    Some(false) => self.zoom_at((-d.y * 0.01).exp(), origin.x, lanes),
                    None => {}
                }
            } else {
                // Pointer back to where the drag started
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::CursorPosition(origin));
                self.ruler_grab = None;
            }
        }
    }

    /// Is the beat grid currently following the audio blocks?
    fn follows_audio(&self, tl: &Timeline) -> bool {
        tl.grid == GridMode::Beat && tl.follow_audio && !self.marks.is_empty()
    }

    fn grid_lines(&self, p: &egui::Painter, lanes: Rect, tl: &Timeline) {
        if self.follows_audio(tl) {
            for m in &self.marks {
                let beat_px = (m.beat_ms as f32) * self.zoom;
                if !m.down && beat_px < 8.0 || m.down && beat_px * 4.0 < 8.0 {
                    continue;
                }
                let x = self.x_at(m.ms, lanes);
                if x < lanes.left() || x > lanes.right() {
                    continue;
                }
                p.line_segment(
                    [pos2(x, lanes.top()), pos2(x, lanes.bottom())],
                    Stroke::new(
                        1.0,
                        theme::border().gamma_multiply(if m.down { 1.0 } else { 0.45 }),
                    ),
                );
            }
            return;
        }
        let step = self.grid_step(tl);
        let off = if tl.grid == GridMode::Beat {
            f64::from(tl.offset_ms)
        } else {
            0.0
        };
        let mut v = ((self.scroll_ms - off) / step).floor() * step + off;
        let mut k = (((self.scroll_ms - off) / step).floor()) as i64;
        while self.x_at(v, lanes) < lanes.right() {
            let x = self.x_at(v, lanes);
            if x >= lanes.left() {
                let strong = tl.grid == GridMode::Beat && k.rem_euclid(4) == 0;
                p.line_segment(
                    [pos2(x, lanes.top()), pos2(x, lanes.bottom())],
                    Stroke::new(
                        1.0,
                        theme::border().gamma_multiply(if strong { 1.0 } else { 0.45 }),
                    ),
                );
            }
            v += step;
            k += 1;
        }
    }

    fn clip_action(&mut self, tl: &mut Timeline, st: &mut AppState, a: ClipAction, cid: u32) {
        let sel: Vec<u32> = st.timeline_sel.iter().copied().collect();
        match a {
            ClipAction::Split => {
                let at = st.timeline_cursor as u64;
                for id in sel {
                    tl.split_clip(id, at);
                }
            }
            ClipAction::Copy | ClipAction::Cut => {
                self.clipboard = sel
                    .iter()
                    .filter_map(|id| tl.clip(*id).map(|(t, c)| (t, c.clone())))
                    .collect();
                if a == ClipAction::Cut {
                    for id in sel {
                        tl.remove_clip(id);
                    }
                    st.timeline_sel.clear();
                }
            }
            ClipAction::Duplicate => {
                let mut new = Vec::new();
                for id in sel {
                    if let Some((t, c)) = tl.clip(id).map(|(t, c)| (t, c.clone())) {
                        let nid = tl.add_clip(t, c.content.clone(), c.end_ms(), c.duration_ms);
                        if let Some(n) = tl.clip_mut(nid) {
                            *n = Clip {
                                id: nid,
                                start_ms: c.end_ms(),
                                ..c
                            };
                        }
                        new.push(nid);
                    }
                }
                st.timeline_sel = new.into_iter().collect();
            }
            ClipAction::Delete => {
                for id in sel {
                    tl.remove_clip(id);
                }
                st.timeline_sel.clear();
            }
            ClipAction::EditScene => {
                if let Some(s) = tl.clip(cid).and_then(|(_, c)| c.scene()) {
                    st.selected_scenes = [s].into();
                }
            }
            ClipAction::ResetSpeed => {
                for id in sel {
                    if let Some(c) = tl.clip_mut(id) {
                        c.speed = 1.0;
                    }
                }
            }
            ClipAction::ApplyGrid => {
                let at = self.menu_clip_ms;
                if let Some((c, (_, g))) = tl
                    .clip(cid)
                    .and_then(|(_, c)| audio_segment(st, c, at).map(|x| (c.clone(), x)))
                {
                    if let ClipContent::Audio { offset_ms, .. } = c.content {
                        tl.apply_audio_grid(c.start_ms, offset_ms, g);
                        tl.follow_audio = false;
                    }
                }
            }
            ClipAction::Grid(edit) => {
                let at = self.menu_clip_ms;
                let current = tl.clip(cid).and_then(|(_, c)| audio_segment(st, c, at));
                if let (
                    Some((key, g)),
                    Some(ClipContent::Audio {
                        segment_grids,
                        grid,
                        ..
                    }),
                ) = (current, tl.clip_mut(cid).map(|c| &mut c.content))
                {
                    // the old correction of the whole file turns into the piece correction
                    *grid = None;
                    segment_grids.retain(|(k, _)| *k != key);
                    let new = match edit {
                        GridEdit::Reset => None,
                        GridEdit::Double => Some(g.doubled()),
                        GridEdit::Halve => Some(g.halved()),
                        GridEdit::Earlier => Some(g.shifted(-1)),
                        GridEdit::Later => Some(g.shifted(1)),
                    };
                    if let Some(n) = new {
                        segment_grids.push((key, n));
                    }
                }
            }
        }
    }

    /// Take over the grid of the first audio block as soon as its analysis is
    /// done.
    fn pending_grid(&mut self, ctx: &egui::Context, tl: &mut Timeline, st: &AppState) {
        let Some(id) = self.pending_grid else {
            return;
        };
        let Some(c) = tl.clip(id).map(|(_, c)| c.clone()) else {
            self.pending_grid = None;
            return;
        };
        let ClipContent::Audio {
            path,
            offset_ms,
            grid,
            ..
        } = &c.content
        else {
            self.pending_grid = None;
            return;
        };
        match st.audio.wave(path, ctx) {
            crate::audio::WaveState::Loading => {}
            crate::audio::WaveState::Failed(_) => self.pending_grid = None,
            crate::audio::WaveState::Ready(a) => {
                // several pieces: the grid follows the audio; otherwise take over the tempo
                let segs = a.effective_segments(*grid, &[]);
                tl.grid = GridMode::Beat;
                if segs.len() > 1 {
                    tl.follow_audio = true;
                } else if let Some(g) = segs.first().and_then(|s| s.grid) {
                    tl.apply_audio_grid(c.start_ms, *offset_ms, g);
                }
                self.pending_grid = None;
            }
        }
    }

    /// Paste the clipboard at `at` (ms); first track = `track`.
    /// Paste at the first free position from `at` on track `track` (spacing and
    /// track offset of the copied blocks are kept).
    fn paste(&mut self, tl: &mut Timeline, st: &mut AppState, track: usize, at: u64) {
        let ids = tl.paste_group(&self.clipboard, track, at);
        if !ids.is_empty() {
            st.timeline_sel = ids.into_iter().collect();
        }
    }
}

/// Correction of the beat grid of an audio block.
#[derive(Clone, Copy, PartialEq)]
enum GridEdit {
    Double,
    Halve,
    Earlier,
    Later,
    Reset,
}

/// Piece of an audio block at position `at` (timeline, ms; if not given the
/// start of the block): key (start in the file) and grid.
fn audio_segment(
    st: &AppState,
    c: &Clip,
    at: Option<f64>,
) -> Option<(u64, dmxcraft_core::audio::AudioGrid)> {
    let ClipContent::Audio {
        path,
        offset_ms,
        grid,
        segment_grids,
        ..
    } = &c.content
    else {
        return None;
    };
    let a = st.audio.ready(path)?;
    let file_ms = at.map_or(*offset_ms as f64, |t| {
        *offset_ms as f64 + t - c.start_ms as f64
    });
    let segs = a.effective_segments(*grid, segment_grids);
    let i = segs
        .partition_point(|s| s.start_ms <= file_ms)
        .saturating_sub(1);
    let sg = segs.get(i)?;
    Some((sg.start_ms.round() as u64, sg.grid?))
}

/// Beats of all audio blocks in the range `from..to` (timeline, ms).
fn audio_marks(tl: &Timeline, st: &AppState, from: f64, to: f64) -> Vec<Mark> {
    let mut out = Vec::new();
    for tr in tl.tracks.iter().filter(|t| !t.muted) {
        for c in &tr.clips {
            let ClipContent::Audio {
                path,
                offset_ms,
                grid,
                segment_grids,
                ..
            } = &c.content
            else {
                continue;
            };
            let (cs, ce) = (c.start_ms as f64, c.end_ms() as f64);
            if ce < from || cs > to {
                continue;
            }
            let Some(a) = st.audio.ready(path) else {
                continue;
            };
            let shift = cs - *offset_ms as f64; // File → timeline
            let segs = a.effective_segments(*grid, segment_grids);
            let n = segs.len();
            for (si, sg) in segs.iter().enumerate() {
                let Some(g) = sg.grid else {
                    continue;
                };
                let a0 = (sg.start_ms + shift).max(cs).max(from);
                let b0 = (sg.end_ms + shift).min(ce).min(to);
                if a0 >= b0 {
                    continue;
                }
                let beat = g.beat_ms();
                // first downbeat in the piece = bar 1
                let first = g.downbeat_ms
                    + ((sg.start_ms - g.downbeat_ms) / g.bar_ms()).ceil() * g.bar_ms();
                for (ms, down) in g.beats(a0 - shift, b0 - shift) {
                    let k = ((ms - first) / beat).round() as i64;
                    out.push(Mark {
                        ms: ms + shift,
                        down,
                        bar: k.div_euclid(4) + 1,
                        seg: si,
                        segs: n,
                        beat_ms: beat,
                    });
                }
            }
        }
    }
    out.sort_by(|a, b| a.ms.total_cmp(&b.ms));
    out
}

fn has_audio(tl: &Timeline) -> bool {
    tl.tracks
        .iter()
        .flat_map(|t| &t.clips)
        .any(|c| matches!(c.content, ClipContent::Audio { .. }))
}

#[derive(Clone, Copy, PartialEq)]
enum ClipAction {
    Split,
    Cut,
    Copy,
    Duplicate,
    Delete,
    EditScene,
    ResetSpeed,
    ApplyGrid,
    Grid(GridEdit),
}

enum HeaderAction {
    Toggle,
    ExpandAll,
    Insert,
    Delete,
    Duplicate,
}

/// Play/pause; when not running: start at the play position.
/// Put text on the system clipboard: egui only reports Ctrl+V when there is
/// text on it (the blocks themselves stay in the timeline).
fn mark_clipboard(ctx: &egui::Context) {
    ctx.copy_text("DMXCraft".into());
}

fn toggle_play(e: &mut Engine, scene: SceneId, cursor: f64) {
    e.toggle_timeline(scene, cursor);
}

fn grid_options(ui: &mut egui::Ui, tl: &mut Timeline, locked: bool) {
    const W: f32 = 220.0;
    menu::begin(ui);
    ui.set_min_width(W);
    ui.set_max_width(W);
    menu::title(ui, t("Grid lines"));
    // Row: label on the left, control on the right (fixed width)
    let row = |ui: &mut egui::Ui, label: &str, add: &mut dyn FnMut(&mut egui::Ui)| {
        ui.allocate_ui_with_layout(
            Vec2::new(W, 28.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| {
                ui.add_space(theme::SP_S);
                ui.label(egui::RichText::new(label).color(theme::text()));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(theme::SP_S);
                    add(ui);
                });
            },
        );
    };
    ui.add_enabled_ui(!locked, |ui| {
        row(ui, "Display", &mut |ui| {
            for (mode, icon, tip) in [
                (GridMode::Beat, Icon::MusicNote, t("Beat based")),
                (GridMode::Time, Icon::Clock, t("Time based")),
            ] {
                if ui
                    .add(
                        IconButton::new(icon)
                            .size(24.0)
                            .flat()
                            .active(tl.grid == mode),
                    )
                    .on_hover_text(tip)
                    .clicked()
                {
                    tl.grid = mode;
                }
            }
        });
        menu::separator(ui);
        ui.add_enabled_ui(tl.grid == GridMode::Beat, |ui| {
            row(ui, t("Follows the audio"), &mut |ui| {
                ui.add(crate::ui::widgets::Toggle::new(&mut tl.follow_audio, ""));
            });
        });
        ui.add_enabled_ui(tl.grid == GridMode::Beat && !tl.follow_audio, |ui| {
            row(ui, t("Tempo (BPM)"), &mut |ui| {
                ui.add_sized(
                    Vec2::new(64.0, 22.0),
                    egui::DragValue::new(&mut tl.bpm)
                        .range(20.0..=400.0)
                        .speed(0.5)
                        .fixed_decimals(2),
                );
            });
            row(ui, t("Offset (ms)"), &mut |ui| {
                ui.add_sized(
                    Vec2::new(64.0, 22.0),
                    egui::DragValue::new(&mut tl.offset_ms)
                        .range(0..=60_000)
                        .speed(5),
                );
            });
        });
    });
}

/// Header of a track: arrow (sub-tracks), number, eye (mute), lock.
/// Header of a track: color strip, expand, name (double-click: rename),
/// show/hide; right-click: menu with color.
fn track_header(
    ui: &mut egui::Ui,
    p: &egui::Painter,
    row: Rect,
    i: usize,
    tr: &mut dmxcraft_core::timeline::Track,
    locked: bool,
    rename: &mut Option<(u32, String)>,
) -> Option<HeaderAction> {
    let head = Rect::from_min_max(row.min, pos2(row.left() + HEADER_W, row.bottom()));
    let th = track_h(tr);
    let top = Rect::from_min_size(head.min, Vec2::new(HEADER_W, th));
    let color = tr.color.map(theme::rgb);
    p.rect_filled(head, 0.0, theme::surface());
    if let Some(c) = color {
        p.rect_filled(top, 0.0, c.gamma_multiply(0.14));
    }
    // Color strip on the left
    p.rect_filled(
        Rect::from_min_size(top.min + Vec2::new(0.0, 3.0), Vec2::new(4.0, th - 6.0)),
        2.0,
        color.unwrap_or_else(theme::border),
    );
    p.line_segment(
        [head.right_top(), head.right_bottom()],
        Stroke::new(1.0, theme::border()),
    );
    let mut action = None;
    let resp = ui.interact(
        top,
        ui.scope_id().with(("track_head", tr.id)),
        Sense::click(),
    );
    let arrow = Rect::from_min_size(
        pos2(top.left() + 6.0, top.center().y - 11.0),
        Vec2::splat(22.0),
    );
    if ui
        .put(
            arrow,
            IconButton::new(if tr.expanded {
                Icon::ChevronDown
            } else {
                Icon::ChevronRight
            })
            .size(22.0)
            .flat(),
        )
        .clicked()
    {
        action = Some(HeaderAction::Toggle);
    }
    let eye = Rect::from_min_size(
        pos2(top.right() - 26.0, top.center().y - 11.0),
        Vec2::splat(22.0),
    );
    if ui
        .put(
            eye,
            IconButton::new(if tr.muted { Icon::EyeOff } else { Icon::Eye })
                .size(22.0)
                .flat(),
        )
        .on_hover_text(t("Show/hide track"))
        .clicked()
        && !locked
    {
        tr.muted = !tr.muted;
    }
    // Name
    let name_rect = Rect::from_min_max(
        pos2(arrow.right() + 2.0, top.center().y - 10.0),
        pos2(eye.left() - 2.0, top.center().y + 10.0),
    );
    let editing = matches!(rename, Some((id, _)) if *id == tr.id);
    if editing {
        if let Some((_, text)) = rename.as_mut() {
            let te = ui.put(
                name_rect,
                egui::TextEdit::singleline(text)
                    .font(theme::body())
                    .margin(Vec2::new(4.0, 1.0)),
            );
            if !te.has_focus() && !te.lost_focus() {
                te.request_focus();
            }
            let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
            let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
            if te.lost_focus() || enter || esc {
                if !esc {
                    tr.name = text.trim().to_string();
                }
                *rename = None;
            }
        }
    } else {
        let label = if tr.name.is_empty() {
            tr!("Track {n}", n = i + 1)
        } else {
            tr.name.clone()
        };
        let clip = p.with_clip_rect(name_rect.intersect(p.clip_rect()));
        clip.text(
            pos2(name_rect.left(), name_rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::body(),
            if tr.muted {
                theme::text_faint()
            } else {
                theme::text()
            },
        );
    }
    if tr.expanded {
        p.text(
            pos2(head.left() + 12.0, head.top() + th + LANE_H / 2.0),
            egui::Align2::LEFT_CENTER,
            t("Dimmer/Volume"),
            egui::FontId::proportional(theme::FS_CAPTION - 1.0),
            theme::text_faint(),
        );
    }
    if locked {
        return action;
    }
    if resp.double_clicked() {
        *rename = Some((tr.id, tr.name.clone()));
    }
    menu::context(&resp, |ui| {
        menu::begin(ui);
        if menu::item(ui, MenuItem::new(t("Rename")).icon(Icon::Edit)) {
            *rename = Some((tr.id, tr.name.clone()));
        }
        let label = if tr.expanded {
            t("Collapse")
        } else {
            t("Expand")
        };
        if menu::item(ui, MenuItem::new(label)) {
            action = Some(HeaderAction::Toggle);
        }
        if menu::item(ui, MenuItem::new(t("Expand all"))) {
            action = Some(HeaderAction::ExpandAll);
        }
        menu::separator(ui);
        ui.horizontal(|ui| {
            ui.add_space(theme::SP_S);
            ui.label(
                egui::RichText::new(t("Track colour"))
                    .small()
                    .color(theme::text_dim()),
            );
        });
        if let Some(c) = color_swatches(
            ui,
            ui.make_persistent_id(("track_color", tr.id)),
            tr.color,
            t("No colour"),
        ) {
            tr.color = c;
        }
        menu::separator(ui);
        if menu::item(ui, MenuItem::new(t("Insert new track")).icon(Icon::Plus)) {
            action = Some(HeaderAction::Insert);
        }
        if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
            action = Some(HeaderAction::Duplicate);
        }
        if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash)) {
            action = Some(HeaderAction::Delete);
        }
    });
    action
}

#[allow(clippy::too_many_arguments)]
fn paint_clip(
    ui: &egui::Ui,
    p: &egui::Painter,
    r: Rect,
    c: &Clip,
    name: &str,
    color: Color32,
    sel: bool,
    scene_ms: u64,
    st: &AppState,
) {
    let body = color.gamma_multiply(0.55);
    p.rect_filled(r, 3.0, theme::raised());
    p.rect_filled(r, 3.0, body);
    let head = Rect::from_min_max(r.min, pos2(r.right(), r.top() + 15.0));
    p.rect_filled(head, 3.0, color);
    // Audio: waveform
    if let ClipContent::Audio {
        path, offset_ms, ..
    } = &c.content
    {
        let state = st.audio.wave(path, ui.ctx());
        if let crate::audio::WaveState::Failed(e) = &state {
            p.text(
                pos2(r.left() + 4.0, r.center().y + 4.0),
                egui::Align2::LEFT_CENTER,
                tr!("File not readable: {e}", e = e),
                egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                theme::DANGER,
            );
        }
        if let crate::audio::WaveState::Ready(a) = state {
            let wave = Rect::from_min_max(pos2(r.left(), head.bottom()), r.max);
            paint_bands(p, wave, c, *offset_ms, &a);
            let segs = match &c.content {
                ClipContent::Audio {
                    grid,
                    segment_grids,
                    ..
                } => a.effective_segments(*grid, segment_grids),
                _ => Vec::new(),
            };
            if let ClipContent::Audio { triggers, .. } = &c.content {
                paint_triggers(ui, p, wave, c, *offset_ms, path, triggers, st);
            }
            paint_segments(p, r, head, wave, c, *offset_ms, &segs);
        }
    }
    // Loop markers of the scene
    if c.looped && scene_ms > 0 && c.scene().is_some() {
        let period = scene_ms as f64 / f64::from(c.speed.max(0.01));
        let per_px = c.duration_ms as f64 / f64::from(r.width().max(1.0));
        if period / per_px > 8.0 {
            let mut k = 1.0;
            while k * period < c.duration_ms as f64 {
                let x = r.left() + (k * period / per_px) as f32;
                let tri = vec![
                    pos2(x - 4.0, head.bottom()),
                    pos2(x + 4.0, head.bottom()),
                    pos2(x, head.bottom() + 5.0),
                ];
                p.add(Shape::convex_polygon(tri, color, Stroke::NONE));
                k += 1.0;
            }
        }
    }
    // Fade in/out as a slope
    let per_ms = r.width() / c.duration_ms.max(1) as f32;
    let shade = Color32::from_black_alpha(110);
    // only the slope as a line (a polygon outline forms spikes beyond the block
    // at acute angles) – and everything clipped to the block
    let clip = p.with_clip_rect(r.intersect(p.clip_rect()));
    let line = Stroke::new(1.0, Color32::WHITE);
    if c.fade_in_ms > 0 {
        let w = (c.fade_in_ms as f32 * per_ms).min(r.width());
        let top = pos2(r.left() + w, r.top());
        clip.add(Shape::convex_polygon(
            vec![r.left_top(), top, r.left_bottom()],
            shade,
            Stroke::NONE,
        ));
        clip.line_segment([r.left_bottom(), top], line);
    }
    if c.fade_out_ms > 0 {
        let w = (c.fade_out_ms as f32 * per_ms).min(r.width());
        let top = pos2(r.right() - w, r.top());
        clip.add(Shape::convex_polygon(
            vec![top, r.right_top(), r.right_bottom()],
            shade,
            Stroke::NONE,
        ));
        clip.line_segment([top, r.right_bottom()], line);
    }
    clip.text(
        pos2(r.left() + 4.0, head.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        egui::FontId::proportional(theme::FS_CAPTION),
        Color32::BLACK,
    );
    clip.text(
        pos2(r.left() + 4.0, r.bottom() - 3.0),
        egui::Align2::LEFT_BOTTOM,
        fmt_ms(c.duration_ms as f64),
        egui::FontId::proportional(theme::FS_CAPTION - 1.0),
        Color32::WHITE,
    );
    if (c.speed - 1.0).abs() > 0.01 {
        clip.text(
            pos2(r.right() - 4.0, r.bottom() - 3.0),
            egui::Align2::RIGHT_BOTTOM,
            format!("×{:.2}", c.speed),
            egui::FontId::proportional(theme::FS_CAPTION - 1.0),
            Color32::WHITE,
        );
    }
    p.rect_stroke(
        r,
        3.0,
        Stroke::new(
            if sel { 2.0 } else { 1.0 },
            if sel {
                Color32::WHITE
            } else {
                color.gamma_multiply(0.8)
            },
        ),
        StrokeKind::Inside,
    );
}

/// Waveform by bands: low (outside), mid, high (inside).
fn paint_bands(
    p: &egui::Painter,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    a: &dmxcraft_core::audio::AudioAnalysis,
) {
    use dmxcraft_core::audio::AudioAnalysis as A;
    p.rect_filled(wave, 0.0, Color32::from_black_alpha(150));
    let visible = wave.intersect(p.clip_rect());
    if visible.width() <= 0.0 {
        return;
    }
    let mid = wave.center().y;
    let half = wave.height() * 0.48;
    let per_px = c.duration_ms as f64 / f64::from(wave.width().max(1.0));
    let x0 = (visible.left() - wave.left()).floor().max(0.0) as usize;
    let x1 = (visible.right() - wave.left()).ceil() as usize;
    for x in x0..x1 {
        let from = offset_ms as f64 + x as f64 * per_px;
        let to = from + per_px.max(crate::audio::FRAME_MS);
        let px = wave.left() + x as f32 + 0.5;
        for (band, scale, color) in [
            (&a.low, 1.0, theme::BAND_LOW),
            (&a.mid, 0.7, theme::BAND_MID),
            (&a.high, 0.45, theme::BAND_HIGH),
        ] {
            let h = A::band_max(band, from, to) * half * scale;
            if h >= 0.5 {
                p.line_segment(
                    [pos2(px, mid - h), pos2(px, mid + h)],
                    Stroke::new(1.0, color),
                );
            }
        }
    }
}

/// Events of the triggers as bars along the bottom edge (one row per
/// trigger).
#[allow(clippy::too_many_arguments)]
fn paint_triggers(
    ui: &egui::Ui,
    p: &egui::Painter,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    path: &str,
    triggers: &[dmxcraft_core::timeline::AudioTrigger],
    st: &AppState,
) {
    const ROW: f32 = 5.0;
    let px_per_ms = wave.width() / c.duration_ms.max(1) as f32;
    let from = offset_ms as f64;
    let to = from + c.duration_ms as f64;
    let clip = p.with_clip_rect(wave.intersect(p.clip_rect()));
    let vis = clip.clip_rect();
    for (row, tr) in triggers.iter().filter(|t| !t.muted).enumerate() {
        let Some(events) = st.triggers.events(path, tr, ui.ctx()) else {
            continue;
        };
        let [r, g, b] = theme::ZONE_PALETTE[row % theme::ZONE_PALETTE.len()];
        let color = Color32::from_rgb(r, g, b);
        let y1 = wave.bottom() - 1.0 - row as f32 * (ROW + 1.0);
        let y0 = y1 - ROW;
        let lead = f64::from(tr.lead_ms);
        let start = events.partition_point(|e| e.1 - lead < from);
        for &(a, b) in &events[start..] {
            let (a, b) = (a - lead, b - lead);
            if a >= to {
                break;
            }
            let x0 = wave.left() + ((a - from) as f32) * px_per_ms;
            let x1 = (wave.left() + ((b - from) as f32) * px_per_ms).max(x0 + 1.5);
            if x1 < vis.left() || x0 > vis.right() {
                continue;
            }
            clip.rect_filled(Rect::from_min_max(pos2(x0, y0), pos2(x1, y1)), 0.0, color);
        }
    }
}

/// Pieces in the block: alternately lightly tinted, boundary as a light line
/// with number and tempo; inside it the beat grid of the piece.
#[allow(clippy::too_many_arguments)]
fn paint_segments(
    p: &egui::Painter,
    r: Rect,
    head: Rect,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    segs: &[dmxcraft_core::audio::AudioSegment],
) {
    let px_per_ms = wave.width() / c.duration_ms.max(1) as f32;
    let from = offset_ms as f64;
    let to = from + c.duration_ms as f64;
    let x_of = |ms: f64| wave.left() + ((ms - from) as f32) * px_per_ms;
    let clip = p.with_clip_rect(r.intersect(p.clip_rect()));
    let many = segs.len() > 1;
    for (i, sg) in segs.iter().enumerate() {
        if sg.end_ms <= from || sg.start_ms >= to {
            continue;
        }
        let (a, b) = (sg.start_ms.max(from), sg.end_ms.min(to));
        let (x0, x1) = (x_of(a), x_of(b));
        if many && i % 2 == 1 {
            clip.rect_filled(
                Rect::from_min_max(pos2(x0, wave.top()), pos2(x1, wave.bottom())),
                0.0,
                Color32::from_white_alpha(10),
            );
        }
        if let Some(g) = sg.grid {
            paint_beats(&clip, wave, x_of, a, b, &g);
        }
        // Marker in the header: number and tempo at the start of the piece
        let text = match (many, sg.grid) {
            (true, Some(g)) => format!("{} · {:.1} BPM", i + 1, g.bpm),
            (true, None) => format!("{}", i + 1),
            (false, Some(g)) => format!("{:.1} BPM", g.bpm),
            (false, None) => String::new(),
        };
        if many && sg.start_ms > from {
            clip.line_segment(
                [pos2(x0, r.top()), pos2(x0, r.bottom())],
                Stroke::new(2.0, Color32::WHITE),
            );
        }
        if !text.is_empty() {
            let font = egui::FontId::proportional(theme::FS_CAPTION - 1.0);
            if !many {
                clip.text(
                    pos2(r.right() - 4.0, head.center().y),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    font,
                    Color32::BLACK,
                );
            } else if x1 - x0 > 90.0 {
                // at the start of the piece; if the block name is there, at the end
                let (pos, align) = if x0 < r.left() + 90.0 {
                    (
                        pos2(x1.min(r.right()) - 4.0, head.center().y),
                        egui::Align2::RIGHT_CENTER,
                    )
                } else {
                    (pos2(x0 + 4.0, head.center().y), egui::Align2::LEFT_CENTER)
                };
                clip.text(pos, align, text, font, Color32::BLACK);
            }
        }
    }
}

/// Beat grid in the block: downbeats as lines, beats as short ticks.
fn paint_beats(
    clip: &egui::Painter,
    wave: Rect,
    x_of: impl Fn(f64) -> f32,
    from: f64,
    to: f64,
    g: &dmxcraft_core::audio::AudioGrid,
) {
    let beat_px = x_of(from + g.beat_ms()) - x_of(from);
    if beat_px * 4.0 < 6.0 {
        return;
    }
    let vis = clip.clip_rect();
    for (ms, down) in g.beats(from, to) {
        let x = x_of(ms);
        if x < vis.left() - 1.0 || x > vis.right() + 1.0 {
            continue;
        }
        if down {
            clip.line_segment(
                [pos2(x, wave.top()), pos2(x, wave.bottom())],
                Stroke::new(1.0, Color32::from_white_alpha(130)),
            );
        } else if beat_px >= 6.0 {
            let s = Stroke::new(1.0, Color32::from_white_alpha(90));
            clip.line_segment([pos2(x, wave.top()), pos2(x, wave.top() + 5.0)], s);
            clip.line_segment([pos2(x, wave.bottom() - 5.0), pos2(x, wave.bottom())], s);
        }
    }
}

const VIDEO_COLOR: Color32 = Color32::from_rgb(130, 105, 190);
const OBS_COLOR: Color32 = Color32::from_rgb(70, 130, 190);
const VIDEO_EXT: [&str; 6] = ["mp4", "mov", "m4v", "mkv", "webm", "avi"];

fn is_video(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| VIDEO_EXT.contains(&e.to_lowercase().as_str()))
}

/// Display name of an OBS block.
pub(crate) fn obs_label(a: &dmxcraft_core::obs::ObsAction) -> String {
    use dmxcraft_core::obs::ObsAction;
    match a {
        ObsAction::Scene(s) => tr!("OBS: {scene}", scene = s),
        ObsAction::Source {
            source,
            visible: true,
            ..
        } => tr!("OBS: {source} on", source = source),
        ObsAction::Source { source, .. } => tr!("OBS: {source} off", source = source),
    }
}

/// Create a block from a video file (OBS plays it); returns ID and length.
fn add_video(tl: &mut Timeline, track: usize, at: u64, path: &str) -> (u32, u64) {
    let len = std::fs::File::open(path)
        .ok()
        .and_then(|mut f| dmxcraft_core::obs::mp4_duration_ms(&mut f))
        .filter(|l| *l > 0)
        .unwrap_or(DEFAULT_CLIP_MS);
    let id = tl.add_clip(
        track,
        ClipContent::Video {
            path: path.to_string(),
            offset_ms: 0,
            length_ms: len,
            obs_scene: String::new(),
            source: String::new(),
        },
        at,
        len,
    );
    (id, len)
}

/// Create a block from an audio file; returns ID and length.
fn add_audio(tl: &mut Timeline, track: usize, at: u64, path: &str) -> (u32, u64) {
    let len = crate::audio::duration_ms(path).unwrap_or(DEFAULT_CLIP_MS);
    let id = tl.add_clip(
        track,
        ClipContent::Audio {
            path: path.to_string(),
            offset_ms: 0,
            length_ms: len,
            grid: None,
            segment_grids: Vec::new(),
            triggers: Vec::new(),
        },
        at,
        len,
    );
    (id, len)
}

fn add_audio_dialog(tl: &mut Timeline, track: usize, at: u64) -> Option<u32> {
    let path = rfd::FileDialog::new()
        .add_filter("Audio", &["mp3", "wav", "flac", "ogg", "m4a", "aac"])
        .pick_file()?;
    Some(add_audio(tl, track, at, &path.to_string_lossy()).0)
}
