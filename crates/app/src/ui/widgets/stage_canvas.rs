//! Stage plan canvas.
//!
//! * [`CanvasMode::Select`] (CONTROL): clicking/dragging a box selects
//!   fixtures.
//! * [`CanvasMode::Edit`] (SETUP → stage plan): dragging moves fixtures,
//!   dragging a box on an empty area selects.
//!
//! Hits are determined at the press point (`press_origin`) so a drag that is
//! only recognized after the movement threshold grabs the right fixture.
//!
//! **Unbounded plan:** positions are plan units (one unit = [`WORLD`] points
//! at 100 % zoom), fixtures may lie anywhere. A camera (zoom + center,
//! [`StageInteraction`]) maps the plan onto the canvas – if the size of the
//! canvas changes, only the visible section changes, never the layout.
//! The middle mouse button drags the view, the mouse wheel zooms towards the
//! mouse position.

use std::collections::BTreeSet;

use dmxcraft_core::{plan, ChannelKind, DmxFrame, FixtureId, PlanShape, Show};
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

const BOX: f32 = 28.0;
/// Length of a plan unit in points at 100 % zoom.
pub const WORLD: f32 = 720.0;
/// Zoom range.
pub const ZOOM_MIN: f32 = 0.2;
pub const ZOOM_MAX: f32 = 4.0;

/// Callback for the context menu: (Ui, fixture under the mouse, current
/// selection).
pub type MenuFn<'m> = dyn FnMut(&mut Ui, Option<FixtureId>, &BTreeSet<FixtureId>) + 'm;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CanvasMode {
    Select,
    Edit,
}

/// Interaction state the owning view keeps between frames.
pub struct StageInteraction {
    lasso_start: Option<Pos2>,
    moving: bool,
    /// Fixture the context menu was opened on.
    context_target: Option<FixtureId>,
    /// Camera: zoom (1.0 = 100 %) …
    pub zoom: f32,
    /// … and the stage point (normalized) in the middle of the canvas.
    pub center: [f32; 2],
    /// At the next draw, zoom so that all fixtures are visible.
    pub fit_request: bool,
    /// Canvas and camera of the last draw (for layers on top).
    pub view: Option<(Rect, Camera)>,
}

impl Default for StageInteraction {
    fn default() -> Self {
        Self {
            lasso_start: None,
            moving: false,
            context_target: None,
            zoom: 1.0,
            center: [0.5, 0.5],
            // Fit everything on first display
            fit_request: true,
            view: None,
        }
    }
}

impl StageInteraction {
    /// Change the zoom by a factor (buttons −/+).
    pub fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
    }
}

/// Drawing below the fixtures.
pub type Underlay = dyn Fn(&egui::Painter, Camera);

/// Camera for one frame: stage (normalized) ↔ screen.
#[derive(Clone, Copy)]
pub struct Camera {
    screen_center: Pos2,
    center: [f32; 2],
    zoom: f32,
}

impl Camera {
    pub fn zoom(self) -> f32 {
        self.zoom
    }
    pub fn scale(self) -> f32 {
        WORLD * self.zoom
    }
    pub fn to_screen(self, p: [f32; 2]) -> Pos2 {
        self.screen_center + Vec2::new(p[0] - self.center[0], p[1] - self.center[1]) * self.scale()
    }
    pub fn to_world(self, s: Pos2) -> [f32; 2] {
        let d = (s - self.screen_center) / self.scale();
        [self.center[0] + d.x, self.center[1] + d.y]
    }
}

/// Zoom and center so that all fixtures fit into `rect`.
fn fit(show: &Show, rect: Rect) -> (f32, [f32; 2]) {
    let pts: Vec<[f32; 2]> = show
        .fixtures
        .iter()
        .map(|f| fixture_pos(show, f.id))
        .collect();
    if pts.is_empty() {
        let z = (rect.width().min(rect.height()) / WORLD).clamp(ZOOM_MIN, ZOOM_MAX);
        return (z, [0.5, 0.5]);
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for p in &pts {
        x0 = x0.min(p[0]);
        y0 = y0.min(p[1]);
        x1 = x1.max(p[0]);
        y1 = y1.max(p[1]);
    }
    // Leave room for fixture size and label
    let pad = 2.5
        * BOX
        * 3.2_f32.min(
            show.fixtures
                .iter()
                .map(|f| show.plan_size(f.id))
                .fold(1.0, f32::max),
        );
    let (w, h) = ((x1 - x0) * WORLD, (y1 - y0) * WORLD);
    let zx = (rect.width() - pad) / w.max(1.0);
    let zy = (rect.height() - pad) / h.max(1.0);
    let zoom = zx.min(zy).clamp(ZOOM_MIN, ZOOM_MAX).min(2.0);
    (zoom, [(x0 + x1) / 2.0, (y0 + y1) / 2.0])
}

pub struct StageCanvas<'a> {
    pub show: &'a Show,
    pub frame: &'a DmxFrame,
    pub mode: CanvasMode,
    /// Grid to snap to while moving (cells per axis), `None` = free.
    pub snap: Option<u32>,
    /// Draw the background grid.
    pub show_grid: bool,
    /// Always fit everything (without zoom and drag controls, e.g. CONTROL).
    pub auto_fit: bool,
    /// Selection order: selected fixtures carry their number.
    pub order: &'a [FixtureId],
    /// Draw below the fixtures (e.g. the image of a mapping).
    pub underlay: Option<&'a Underlay>,
    /// Live values: affected fixtures get a corner in the live color.
    pub live: Option<&'a dmxcraft_core::fasthash::FastMap<dmxcraft_core::ChannelRef, u8>>,
}

/// Position of a fixture (saved or default by patch order).
pub fn fixture_pos(show: &Show, id: FixtureId) -> [f32; 2] {
    show.plan_position(id)
}

impl StageCanvas<'_> {
    /// Draws the plan and processes input. Changes the selection directly;
    /// returns position changes (the show is only borrowed here).
    pub fn show(
        &self,
        ui: &mut Ui,
        state: &mut StageInteraction,
        selected: &mut BTreeSet<FixtureId>,
        menu: &mut MenuFn<'_>,
    ) -> Vec<(FixtureId, [f32; 2])> {
        let mut moves = Vec::new();
        let size = ui.available_size().max(Vec2::new(120.0, 120.0));
        let (rect, resp) = ui.allocate_exact_size(size, Sense::click_and_drag());
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 0.0, theme::canvas());

        // ----- Camera -----
        if state.fit_request || self.auto_fit {
            // Leave room for the zoom bar at the bottom (only with controls)
            let area = if self.auto_fit {
                rect
            } else {
                rect.with_max_y(rect.bottom() - 44.0)
            };
            (state.zoom, state.center) = fit(self.show, area);
            // Relate the center to the whole canvas
            let shift = (area.center().y - rect.center().y) / (WORLD * state.zoom);
            state.center[1] -= shift;
            state.fit_request = false;
        }
        // Middle mouse button: drag the view
        let panning = resp.dragged_by(egui::PointerButton::Middle);
        if panning {
            let d = resp.drag_delta() / (WORLD * state.zoom);
            state.center = [state.center[0] - d.x, state.center[1] - d.y];
        }
        // Mouse wheel: zoom towards the mouse position
        if resp.hovered() && !self.auto_fit {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                if let Some(mouse) = resp.hover_pos() {
                    let before = Camera {
                        screen_center: rect.center(),
                        center: state.center,
                        zoom: state.zoom,
                    };
                    let anchor = before.to_world(mouse);
                    state.zoom_by((scroll * 0.0025).exp());
                    // The point under the mouse stays in place
                    let d = (mouse - rect.center()) / (WORLD * state.zoom);
                    state.center = [anchor[0] - d.x, anchor[1] - d.y];
                }
                ui.input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
            }
        }
        let cam = Camera {
            screen_center: rect.center(),
            center: state.center,
            zoom: state.zoom,
        };
        self.paint_stage(&painter, rect, cam);
        state.view = Some((rect, cam));
        if let Some(under) = self.underlay {
            under(&painter, cam);
        }

        if self.show.fixtures.is_empty() {
            return moves;
        }

        let to_screen = |p: [f32; 2]| cam.to_screen(p);
        let zoom = cam.zoom;
        let boxes: Vec<(FixtureId, Rect)> = self
            .show
            .fixtures
            .iter()
            .map(|f| {
                (
                    f.id,
                    fixture_rect(
                        self.show,
                        f.id,
                        to_screen(fixture_pos(self.show, f.id)),
                        zoom,
                    ),
                )
            })
            .collect();
        // Hit within the rotated outline (rotation around the center)
        let hit = |p: Pos2| {
            boxes
                .iter()
                .rev()
                .find(|(id, b)| {
                    let c = b.center();
                    let (sin, cos) = (-self.show.plan_rotation(*id)).to_radians().sin_cos();
                    let d = p - c;
                    let local = c + Vec2::new(d.x * cos - d.y * sin, d.x * sin + d.y * cos);
                    b.expand(3.0).contains(local)
                })
                .map(|(id, _)| *id)
        };
        let (mods, press_origin) = ui.input(|i| (i.modifiers, i.pointer.press_origin()));
        let additive = mods.shift || mods.command;

        // ----- Dragging -----
        if resp.drag_started_by(egui::PointerButton::Primary) {
            let origin = press_origin.or(resp.interact_pointer_pos());
            let target = origin.and_then(hit);
            match (self.mode, target) {
                (CanvasMode::Edit, Some(id)) => {
                    if !selected.contains(&id) {
                        if !additive {
                            selected.clear();
                        }
                        selected.insert(id);
                    }
                    state.moving = true;
                }
                _ => {
                    state.lasso_start = origin;
                }
            }
        }
        if resp.dragged_by(egui::PointerButton::Primary) && state.moving {
            let d = resp.drag_delta() / cam.scale();
            let (dx, dy) = (d.x, d.y);
            for id in selected.iter() {
                let p = fixture_pos(self.show, *id);
                moves.push((*id, [p[0] + dx, p[1] + dy]));
            }
        }
        let pointer = ui.input(|i| i.pointer.latest_pos());
        let lasso = match (state.lasso_start, pointer) {
            (Some(s), Some(p))
                if resp.dragged_by(egui::PointerButton::Primary) || resp.drag_stopped() =>
            {
                Some(Rect::from_two_pos(s, p))
            }
            _ => None,
        };
        if resp.drag_stopped() && !panning {
            if let Some(l) = lasso {
                if !additive {
                    selected.clear();
                }
                selected.extend(
                    boxes
                        .iter()
                        .filter(|(_, b)| l.intersects(*b))
                        .map(|(id, _)| *id),
                );
            }
            if state.moving {
                if let Some(cells) = self.snap {
                    for id in selected.iter() {
                        // Position including this frame's movement
                        let p = moves
                            .iter()
                            .find(|(m, _)| m == id)
                            .map(|(_, p)| *p)
                            .unwrap_or_else(|| fixture_pos(self.show, *id));
                        moves.retain(|(m, _)| m != id);
                        moves.push((*id, plan::snap(p, cells)));
                    }
                }
            }
            state.lasso_start = None;
            state.moving = false;
        }

        // ----- Clicking -----
        if resp.clicked() {
            match resp.interact_pointer_pos().and_then(hit) {
                Some(id) if additive => {
                    if !selected.remove(&id) {
                        selected.insert(id);
                    }
                }
                Some(id) => {
                    selected.clear();
                    selected.insert(id);
                }
                None if !additive => selected.clear(),
                None => {}
            }
        }

        // ----- Context menu -----
        if resp.secondary_clicked() {
            let target = resp.interact_pointer_pos().and_then(hit);
            if let Some(id) = target {
                if !selected.contains(&id) {
                    selected.clear();
                    selected.insert(id);
                }
            }
            state.context_target = target;
        }
        let target = state.context_target;
        crate::ui::widgets::menu::context(&resp, |ui| menu(ui, target, selected));

        // ----- Drawing -----
        let hovered = resp.hover_pos().and_then(hit);
        for (id, b) in &boxes {
            // show the new position while dragging
            let b = moves
                .iter()
                .find(|(m, _)| m == id)
                .map(|(_, p)| fixture_rect(self.show, *id, to_screen(*p), zoom))
                .unwrap_or(*b);
            let sel = selected.contains(id);
            let mut colors = live_colors(self.show, *id, self.frame);
            // Strobe: off in the dark phase
            if let Some(hz) = strobe_hz(self.show, *id, self.frame) {
                let time = ui.input(|i| i.time);
                // 0 Hz = shutter closed
                if hz <= 0.0 || (time * f64::from(hz)).fract() > 0.35 {
                    colors.fill((Color32::BLACK, 0.0));
                }
                ui.ctx().request_repaint();
            }
            paint_fixture(
                &painter,
                self.show.plan_shape(*id),
                b,
                self.show.plan_rotation(*id),
                &colors,
                sel,
                Some(*id) == hovered,
            );
            if self
                .live
                .is_some_and(|l| l.keys().any(|c| c.fixture == *id))
            {
                paint_live_corner(
                    &painter,
                    self.show.plan_shape(*id),
                    b,
                    self.show.plan_rotation(*id),
                );
            }
            let time = ui.input(|i| i.time);
            if paint_effects(&painter, self.show, *id, self.frame, b, time) {
                ui.ctx().request_repaint();
            }
            // Number in the selection order
            if let Some(n) = sel
                .then(|| self.order.iter().position(|f| f == id))
                .flatten()
            {
                let size = (b.height().min(b.width()) * 0.45).clamp(9.0, 16.0);
                let galley = painter.layout_no_wrap(
                    (n + 1).to_string(),
                    egui::FontId::proportional(size),
                    Color32::WHITE,
                );
                let pad = Vec2::new(4.0, 1.0);
                let tag = Rect::from_center_size(b.center(), galley.size() + pad * 2.0);
                painter.rect_filled(tag, 3.0, Color32::from_black_alpha(150));
                painter.galley(tag.min + pad, galley, Color32::WHITE);
            }
        }
        // Fixture info only on hover
        if let Some(f) = hovered.and_then(|id| self.show.fixture(id)) {
            let ty = self
                .show
                .type_of(f)
                .map(|t| format!("{} {}", t.manufacturer, t.model).trim().to_string())
                .unwrap_or_else(|| f.type_key.clone());
            let ch = self.show.fixture_channels(f.id);
            let addr = tr!(
                "Universe {u} · address {a}–{b}",
                u = f.universe + 1,
                a = f.address,
                b = f.address + ch.max(1) - 1
            );
            let name = f.name.clone();
            // affected channels of the live edit
            let live: Vec<String> = self.live.map_or_else(Vec::new, |l| {
                let mut chans: Vec<u16> = l
                    .keys()
                    .filter(|c| c.fixture == f.id)
                    .map(|c| c.channel)
                    .collect();
                chans.sort_unstable();
                chans
                    .into_iter()
                    .map(|c| {
                        let ch = dmxcraft_core::ChannelRef {
                            fixture: f.id,
                            channel: c,
                        };
                        if ch.is_virtual() {
                            crate::i18n::t("Dimmer (virtual)").to_string()
                        } else {
                            self.show
                                .channel_def(ch)
                                .map_or_else(|| (c + 1).to_string(), |d| d.name.clone())
                        }
                    })
                    .collect()
            });
            // Setup: everything incl. DIP switches; Control: only name and address
            let full = self.mode == CanvasMode::Edit;
            let address = f.address;
            resp.clone().on_hover_ui_at_pointer(|ui| {
                ui.label(egui::RichText::new(name).strong());
                if full {
                    ui.label(egui::RichText::new(ty).color(theme::text_dim()));
                }
                ui.label(egui::RichText::new(addr).color(theme::text_dim()));
                if !live.is_empty() {
                    ui.label(
                        egui::RichText::new(tr!("Live: {channels}", channels = live.join(", ")))
                            .color(theme::WARN),
                    );
                }
                if full {
                    ui.add_space(theme::SP_XS);
                    crate::ui::widgets::DipSwitch {
                        address,
                        compact: false,
                    }
                    .show(ui);
                }
            });
        }
        if let Some(l) = lasso {
            painter.rect_filled(l, 2.0, theme::accent().gamma_multiply(0.07));
            painter.rect_stroke(
                l,
                2.0,
                Stroke::new(1.0, theme::accent()),
                StrokeKind::Inside,
            );
        }
        // Only set when needed – otherwise e.g. the resize cursor of the area
        // borders would be overwritten.
        if panning {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        } else if self.mode == CanvasMode::Edit {
            if state.moving {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            } else if hovered.is_some() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
        moves
    }

    /// Optionally a grid across the whole visible canvas (unbounded).
    fn paint_stage(&self, painter: &egui::Painter, rect: Rect, cam: Camera) {
        if !self.show_grid {
            return;
        }
        let cells = self.snap.unwrap_or(24).max(1) as f32;
        let step = cam.scale() / cells;
        // When zoomed far out, only every n-th line so it stays calm
        let mut every = 1.0;
        while step * every < 8.0 {
            every *= 2.0;
        }
        let unit = every / cells;
        let tl = cam.to_world(rect.min);
        let br = cam.to_world(rect.max);
        let line = Stroke::new(1.0, theme::canvas_line().gamma_multiply(0.6));
        let mut x = (tl[0] / unit).floor() * unit;
        while x <= br[0] {
            let sx = cam.to_screen([x, 0.0]).x;
            painter.line_segment(
                [Pos2::new(sx, rect.top()), Pos2::new(sx, rect.bottom())],
                line,
            );
            x += unit;
        }
        let mut y = (tl[1] / unit).floor() * unit;
        while y <= br[1] {
            let sy = cam.to_screen([0.0, y]).y;
            painter.line_segment(
                [Pos2::new(rect.left(), sy), Pos2::new(rect.right(), sy)],
                line,
            );
            y += unit;
        }
    }
}

/// Screen rectangle of a fixture at a zoom factor.
pub fn fixture_rect(show: &Show, id: FixtureId, center: Pos2, zoom: f32) -> Rect {
    let s = BOX * show.plan_size(id) * zoom;
    let size = match show.plan_shape(id) {
        PlanShape::BarHorizontal => Vec2::new(s * 2.8, s * 0.55),
        PlanShape::BarVertical => Vec2::new(s * 0.55, s * 2.8),
        _ => Vec2::splat(s),
    };
    Rect::from_center_size(center, size)
}

/// Flash rate of a fixture when its strobe channel is flashing
/// (`Some(0.0)`: shutter closed, `None`: open). Uses the value ranges of the
/// fixture file; without them values below 10 count as open, above that
/// 1 to 20 Hz.
pub fn strobe_hz(show: &Show, id: FixtureId, frame: &DmxFrame) -> Option<f32> {
    let f = show.fixture(id)?;
    let ty = show.type_of(f)?;
    let i = ty
        .channels
        .iter()
        .position(|c| c.kind == ChannelKind::Strobe)?;
    let v = frame.get(usize::from(f.universe), f.address + i as u16)?;
    ty.channels[i].strobe(v)
}

/// Display colors of the individual light kinds (linear 0–1).
const AMBER: [f32; 3] = [1.0, 0.55, 0.05];
const UV: [f32; 3] = [0.5, 0.12, 1.0];
const WHITE: [f32; 3] = [1.0, 0.95, 0.88];

/// One display color per pixel (several for bars, otherwise one) with
/// brightness.
///
/// Takes red, green, blue, white, amber and UV (UV violet) into account, plus
/// color wheels/color macros: if all color channels are at 0, the fixture
/// shows the approximate color of the wheel.
pub fn live_colors(show: &Show, id: FixtureId, frame: &DmxFrame) -> Vec<(Color32, f32)> {
    let Some(f) = show.fixture(id) else {
        return vec![(Color32::DARK_GRAY, 0.0)];
    };
    let Some(ty) = show.type_of(f) else {
        return vec![(Color32::DARK_GRAY, 0.0)];
    };
    let value = |i: usize| -> f32 {
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map_or(0.0, |v| f32::from(v) / 255.0)
    };
    let all = |k: ChannelKind| -> Vec<usize> {
        ty.channels
            .iter()
            .enumerate()
            .filter(|(_, c)| c.kind == k)
            .map(|(i, _)| i)
            .collect()
    };
    let (reds, greens, blues, whites, ambers, uvs) = (
        all(ChannelKind::Red),
        all(ChannelKind::Green),
        all(ChannelKind::Blue),
        all(ChannelKind::White),
        all(ChannelKind::Amber),
        all(ChannelKind::Uv),
    );
    let dimmers = all(ChannelKind::Dimmer);
    let wheel_ch = all(ChannelKind::ColorWheel).first().copied();
    let wheel = wheel_ch.map(value);
    let pixels = [&reds, &greens, &blues, &whites, &uvs]
        .iter()
        .map(|v| v.len())
        .max()
        .unwrap_or(0)
        .max(1);
    let has_color = [&reds, &greens, &blues, &whites, &ambers, &uvs]
        .iter()
        .any(|v| !v.is_empty());
    (0..pixels)
        .map(|px| {
            let pick = |v: &Vec<usize>| -> f32 {
                v.get(px).or_else(|| v.first()).map_or(0.0, |i| value(*i))
            };
            let mut base = [0.0f32; 3];
            let mut add = |c: [f32; 3], x: f32| {
                for k in 0..3 {
                    base[k] += c[k] * x;
                }
            };
            add([1.0, 0.0, 0.0], pick(&reds));
            add([0.0, 1.0, 0.0], pick(&greens));
            add([0.0, 0.0, 1.0], pick(&blues));
            add(WHITE, pick(&whites));
            add(AMBER, pick(&ambers));
            add(UV, pick(&uvs));
            let mixed = base.iter().any(|x| *x > 0.003);
            if !has_color || !mixed {
                // Color wheel or color macro, otherwise halogen white
                if let Some(w) = wheel.filter(|w| *w > 0.03 || !has_color) {
                    let raw = (w * 255.0).round() as u8;
                    let c = wheel_ch.map_or_else(
                        || crate::viz::rig::color_wheel(w),
                        |i| crate::viz::rig::wheel_color(&ty.channels[i], raw),
                    );
                    base = [c.x, c.y, c.z];
                } else if !has_color {
                    base = WHITE;
                }
            }
            // Pixel dimmer if present, otherwise the first dimmer
            let intensity = match (dimmers.len(), has_color || wheel.is_some()) {
                (0, true) => 1.0,
                (0, false) => 0.0,
                (n, _) if n == pixels => value(dimmers[px]),
                _ => value(dimmers[0]),
            };
            // Color wheel without color channels and without dimmer: on as soon as the
            // wheel is set
            let intensity = if !has_color && dimmers.is_empty() {
                wheel.map_or(0.0, |w| if w > 0.03 { 1.0 } else { 0.0 })
            } else {
                intensity
            };
            let c = base.map(|x| (x * intensity).min(1.0));
            let lum = c[0].max(c[1]).max(c[2]);
            (
                Color32::from_rgb(
                    (c[0] * 255.0) as u8,
                    (c[1] * 255.0) as u8,
                    (c[2] * 255.0) as u8,
                ),
                lum,
            )
        })
        .collect()
}

/// Effects above the fixture: laser (colored dots/beams that turn with the
/// laser rotation), fog (cloud by output and fan) and the pan/tilt direction
/// of moving heads. Returns `true` while something is animating.
pub fn paint_effects(
    p: &egui::Painter,
    show: &Show,
    id: FixtureId,
    frame: &DmxFrame,
    r: Rect,
    time: f64,
) -> bool {
    let Some(f) = show.fixture(id) else {
        return false;
    };
    let Some(ty) = show.type_of(f) else {
        return false;
    };
    let value = |i: usize| -> f32 {
        frame
            .get(usize::from(f.universe), f.address + i as u16)
            .map_or(0.0, |v| f32::from(v) / 255.0)
    };
    let find = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k);
    let named = |k: ChannelKind, word: &str| {
        ty.channels
            .iter()
            .position(|c| c.kind == k && c.name.to_lowercase().contains(word))
    };
    let c = r.center();
    let size = r.width().max(r.height());
    let t = time as f32;
    let mut animating = false;

    // ----- Fog -----
    if let Some(i) = find(ChannelKind::Fog) {
        let out = value(i);
        let fan = find(ChannelKind::Fan).map_or(0.5, value);
        if out > 0.02 {
            animating = true;
            let n = 7;
            let spread = size * (0.9 + fan * 1.3);
            for k in 0..n {
                let seed = k as f32 * 1.7;
                let phase = (t * (0.15 + fan * 0.35) + seed * 0.37).fract();
                let dir = egui::Vec2::angled(seed * 2.4 + (t * 0.1).sin() * 0.3);
                let pos = c + dir * spread * 0.25 + Vec2::new(0.0, -phase * spread * 0.6);
                let rad = size * (0.35 + phase * 0.5);
                let alpha = (out * (1.0 - phase) * 70.0) as u8;
                let fog = if theme::is_light() {
                    Color32::from_black_alpha(alpha / 2)
                } else {
                    Color32::from_white_alpha(alpha)
                };
                p.circle_filled(pos, rad, fog);
            }
        }
    }

    // ----- Laser -----
    if let Some(i) = find(ChannelKind::Laser) {
        let v = value(i);
        // the laser's own strobe (name contains “laser”) switches dark
        let gate = named(ChannelKind::Strobe, "laser").is_none_or(|s| {
            let sv = value(s);
            sv < 0.04 || (time * f64::from(1.0 + sv * 19.0)).fract() < 0.35
        });
        if v > 0.02 && gate {
            animating = true;
            let rot = named(ChannelKind::Speed, "laser")
                .or_else(|| named(ChannelKind::Speed, "rot"))
                .map_or(0.3, value);
            let angle = t * (0.2 + rot * 3.0);
            // Pattern from the channel value: number of dots and colors change
            let pattern = (v * 7.99) as usize;
            let dots = 6 + pattern * 3;
            let cols = [
                Color32::from_rgb(0xff, 0x30, 0x30),
                Color32::from_rgb(0x30, 0xff, 0x50),
                Color32::from_rgb(0x40, 0x80, 0xff),
            ];
            let reach = size * 2.2;
            for k in 0..dots {
                let a = angle + std::f32::consts::TAU * k as f32 / dots as f32;
                let wobble = ((k * 7 + pattern) % 5) as f32 / 5.0;
                let end = c + Vec2::angled(a) * reach * (0.55 + wobble * 0.45);
                let col = cols[(k + pattern) % 3];
                p.line_segment([c, end], Stroke::new(1.0, col.gamma_multiply(0.35)));
                p.circle_filled(end, 2.0, col);
            }
        }
    }

    // ----- Pan/tilt -----
    // Where the beam points, seen from above – same model as the 3D
    // visualizer (mounting from the 3D placement, ranges from the profile).
    if let (Some(pi), Some(ti)) = (find(ChannelKind::Pan), find(ChannelKind::Tilt)) {
        let axis = |coarse: usize, fine: Option<usize>| {
            value(coarse) + fine.map_or(0.0, |i| value(i) / 255.0)
        };
        let phys = ty.physical.unwrap_or_default();
        let place = show.placement3d(id);
        let d = dmxcraft_core::venue::moving_head_direction(
            place.rotation,
            axis(pi, find(ChannelKind::PanFine)),
            axis(ti, find(ChannelKind::TiltFine)),
            phys.pan_range,
            phys.tilt_range,
        );
        // room direction as seen in the plan; length = horizontal part
        let m = show.plan_axes();
        let flat = Vec2::new(
            m[0][0] * d[0] + m[0][1] * d[2],
            m[1][0] * d[0] + m[1][1] * d[2],
        );
        let len = flat.length();
        let dir = if len > 1e-4 { flat / len } else { Vec2::ZERO };
        let end = c + dir * (size * 0.35 + len * size * 0.7);
        let s = Stroke::new(1.5, theme::accent());
        p.line_segment([c, end], s);
        p.circle_filled(end, 2.5, theme::accent());
    }
    animating
}

/// Draws a fixture in its shape – filled with the current color, with a thin
/// outline, rotated by `rot` degrees. Pixel fixtures (bars, matrix) show
/// their pixels as segments, all others one color.
/// Marker “live values set”: small triangle in the top right corner (for
/// rotated shapes at the corner that is currently top right). Lies inside
/// the fixture so closely spaced neighbors stay clear.
fn paint_live_corner(p: &egui::Painter, shape: PlanShape, r: Rect, rot: f32) {
    let len = (r.width().min(r.height()) * 0.45).clamp(5.0, 10.0);
    let color = theme::WARN;
    let rotated = rot.rem_euclid(360.0) != 0.0 && shape != PlanShape::Round;
    if !rotated {
        // round fixtures: corner of the enclosing square (lies next to the circle)
        let c = r.right_top();
        p.add(egui::Shape::convex_polygon(
            vec![c, c - Vec2::new(len, 0.0), c + Vec2::new(0.0, len)],
            color,
            Stroke::NONE,
        ));
        return;
    }
    let center = r.center();
    let (sin, cos) = rot.to_radians().sin_cos();
    let turn = |q: Pos2| {
        let d = q - center;
        center + Vec2::new(d.x * cos - d.y * sin, d.x * sin + d.y * cos)
    };
    let quad = [
        r.left_top(),
        r.right_top(),
        r.right_bottom(),
        r.left_bottom(),
    ]
    .map(turn);
    // Corner that lies furthest to the top right
    let i = (0..4)
        .max_by(|a, b| {
            let s = |q: Pos2| q.x - q.y;
            s(quad[*a]).total_cmp(&s(quad[*b]))
        })
        .unwrap_or(1);
    let c = quad[i];
    let a = quad[(i + 1) % 4];
    let b = quad[(i + 3) % 4];
    let leg = |to: Pos2| {
        let d = to - c;
        c + d / d.length().max(1.0) * len
    };
    p.add(egui::Shape::convex_polygon(
        vec![c, leg(a), leg(b)],
        color,
        Stroke::NONE,
    ));
}

fn paint_fixture(
    p: &egui::Painter,
    shape: PlanShape,
    r: Rect,
    rot: f32,
    colors: &[(Color32, f32)],
    sel: bool,
    hovered: bool,
) {
    let stroke = if sel {
        Stroke::new(2.0, theme::accent())
    } else if hovered {
        Stroke::new(1.0, theme::canvas_dim())
    } else {
        Stroke::new(1.0, theme::canvas_line())
    };
    let c = r.center();
    let (sin, cos) = rot.to_radians().sin_cos();
    // Rotate a point around the center
    let turn = |q: Pos2| -> Pos2 {
        let d = q - c;
        c + Vec2::new(d.x * cos - d.y * sin, d.x * sin + d.y * cos)
    };
    let quad = |q: Rect| -> Vec<Pos2> {
        [
            q.left_top(),
            q.right_top(),
            q.right_bottom(),
            q.left_bottom(),
        ]
        .into_iter()
        .map(turn)
        .collect()
    };
    // brightest color for single-colored shapes
    let (c0, l0) = colors
        .iter()
        .copied()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .unwrap_or((Color32::BLACK, 0.0));
    let c0 = theme::lamp(c0);
    let n = colors.len().max(1);
    match shape {
        PlanShape::Round => {
            let rad = r.width().min(r.height()) * 0.5;
            p.circle_filled(c, rad, c0);
            p.circle_stroke(c, rad, stroke);
        }
        PlanShape::Square => {
            p.add(egui::Shape::convex_polygon(quad(r), c0, stroke));
        }
        PlanShape::Matrix => {
            let side = (n as f32).sqrt().ceil().max(1.0) as usize;
            let (cw, ch) = (r.width() / side as f32, r.height() / side as f32);
            for i in 0..side * side {
                let (col, _) = colors.get(i).copied().unwrap_or((c0, l0));
                let col = theme::lamp(col);
                let cell = Rect::from_min_size(
                    Pos2::new(
                        r.left() + cw * (i % side) as f32,
                        r.top() + ch * (i / side) as f32,
                    ),
                    Vec2::new(cw, ch),
                );
                p.add(egui::Shape::convex_polygon(
                    quad(cell.shrink(0.5)),
                    col,
                    Stroke::NONE,
                ));
            }
            p.add(egui::Shape::closed_line(quad(r), stroke));
        }
        PlanShape::BarHorizontal | PlanShape::BarVertical => {
            let horizontal = shape == PlanShape::BarHorizontal;
            for (i, (col, _)) in colors.iter().enumerate() {
                let seg = if horizontal {
                    let w = r.width() / n as f32;
                    Rect::from_min_size(
                        Pos2::new(r.left() + w * i as f32, r.top()),
                        Vec2::new(w, r.height()),
                    )
                } else {
                    let h = r.height() / n as f32;
                    Rect::from_min_size(
                        Pos2::new(r.left(), r.top() + h * i as f32),
                        Vec2::new(r.width(), h),
                    )
                };
                let seg = if n > 1 { seg.shrink(0.5) } else { seg };
                p.add(egui::Shape::convex_polygon(
                    quad(seg),
                    theme::lamp(*col),
                    Stroke::NONE,
                ));
            }
            p.add(egui::Shape::closed_line(quad(r), stroke));
        }
        PlanShape::Effect => {
            let pts = [
                Pos2::new(c.x, r.top()),
                Pos2::new(r.right(), r.bottom()),
                Pos2::new(r.left(), r.bottom()),
            ]
            .into_iter()
            .map(turn)
            .collect();
            p.add(egui::Shape::convex_polygon(pts, c0, stroke));
        }
    }
}

/// Small icon of a fixture shape (for menus and buttons). `None` = automatic.
pub fn paint_shape_icon(p: &egui::Painter, shape: Option<PlanShape>, r: Rect, color: Color32) {
    let stroke = Stroke::new(1.4, color);
    let c = r.center();
    let s = r.width().min(r.height());
    match shape {
        None => {
            p.circle_stroke(c, s * 0.42, Stroke::new(1.2, color.gamma_multiply(0.6)));
            p.text(
                c,
                egui::Align2::CENTER_CENTER,
                "A",
                egui::FontId::proportional(s * 0.55),
                color,
            );
        }
        Some(PlanShape::Round) => {
            p.circle_filled(c, s * 0.42, color);
        }
        Some(PlanShape::Square) => {
            p.rect_filled(Rect::from_center_size(c, Vec2::splat(s * 0.8)), 1.5, color);
        }
        Some(PlanShape::Matrix) => {
            let cell = s * 0.8 / 3.0;
            let origin = c - Vec2::splat(s * 0.4);
            for i in 0..9 {
                let rr = Rect::from_min_size(
                    origin + Vec2::new(cell * (i % 3) as f32, cell * (i / 3) as f32),
                    Vec2::splat(cell),
                );
                p.rect_filled(rr.shrink(0.8), 0.5, color);
            }
        }
        Some(PlanShape::BarHorizontal) => {
            let bar = Rect::from_center_size(c, Vec2::new(s * 0.95, s * 0.3));
            for i in 0..4 {
                let w = bar.width() / 4.0;
                let seg = Rect::from_min_size(
                    egui::pos2(bar.left() + w * i as f32, bar.top()),
                    Vec2::new(w, bar.height()),
                );
                p.rect_filled(seg.shrink(0.6), 0.5, color);
            }
        }
        Some(PlanShape::BarVertical) => {
            let bar = Rect::from_center_size(c, Vec2::new(s * 0.3, s * 0.95));
            for i in 0..4 {
                let h = bar.height() / 4.0;
                let seg = Rect::from_min_size(
                    egui::pos2(bar.left(), bar.top() + h * i as f32),
                    Vec2::new(bar.width(), h),
                );
                p.rect_filled(seg.shrink(0.6), 0.5, color);
            }
        }
        Some(PlanShape::Effect) => {
            let pts = vec![
                egui::pos2(c.x, c.y - s * 0.42),
                egui::pos2(c.x + s * 0.45, c.y + s * 0.38),
                egui::pos2(c.x - s * 0.45, c.y + s * 0.38),
            ];
            p.add(egui::Shape::convex_polygon(pts, color, stroke));
        }
    }
}
