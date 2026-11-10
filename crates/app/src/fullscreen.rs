//! Toggle fullscreen without disturbing the layout.
//!
//! While the window changes its size (fullscreen on/off, the macOS
//! animation, the intermediate steps below) egui would squeeze the resizable
//! areas into every intermediate size and keep them there. So the content is
//! not laid out while the window [`settling`](Fullscreen::settling): until
//! the size has been stable for a few frames (or after a timeout).
//!
//! Windows: from a maximized, borderless window Windows does not fully take
//! over the fullscreen size (a black strip the height of the taskbar stays).
//! So the maximization is undone before going fullscreen and restored
//! afterwards, each with a few frames in between.

use std::time::{Duration, Instant};

use eframe::egui::{self, Vec2, ViewportCommand};

/// Frames between the Windows steps.
const SETTLE: u8 = 3;
/// Frames the size has to stay the same before the layout runs again.
const STABLE_FRAMES: u8 = 4;
/// Longest wait for the window (macOS animation takes about a second).
const MAX_WAIT: Duration = Duration::from_millis(1500);

#[derive(Default)]
pub struct Fullscreen {
    on: bool,
    /// Maximized before fullscreen (maximize again when leaving).
    was_maximized: bool,
    /// Pending step and remaining frames until then.
    pending: Option<(Step, u8)>,
    /// Window is changing size.
    moving: Option<Moving>,
}

#[derive(Clone, Copy)]
struct Moving {
    since: Instant,
    /// Size before the change.
    start: Vec2,
    last: Vec2,
    /// Frames with an unchanged size after it has changed at least once.
    stable: u8,
    changed: bool,
}

impl Moving {
    fn new(ctx: &egui::Context) -> Self {
        let size = ctx.content_rect().size();
        Self {
            since: Instant::now(),
            start: size,
            last: size,
            stable: 0,
            changed: false,
        }
    }
}

#[derive(Clone, Copy)]
enum Step {
    Enter,
    Maximize,
}

impl Fullscreen {
    pub fn is_on(&self) -> bool {
        self.on
    }

    pub fn toggle(&mut self, ctx: &egui::Context) {
        if self.pending.is_some() {
            return;
        }
        let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
        self.moving = Some(Moving::new(ctx));
        if self.on {
            self.on = false;
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
            if self.was_maximized && cfg!(windows) {
                self.pending = Some((Step::Maximize, SETTLE));
            }
        } else {
            self.on = true;
            self.was_maximized = maximized;
            if self.was_maximized && cfg!(windows) {
                ctx.send_viewport_cmd(ViewportCommand::Maximized(false));
                self.pending = Some((Step::Enter, SETTLE));
            } else {
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true));
            }
        }
        ctx.request_repaint();
    }

    /// Call every frame: executes pending steps.
    pub fn update(&mut self, ctx: &egui::Context) {
        let Some((step, frames)) = self.pending else {
            // Take over the state when the window changed from outside (e.g. the
            // green button on macOS, or the remembered state at startup)
            if let Some(fs) = ctx.input(|i| i.viewport().fullscreen) {
                if fs != self.on && self.moving.is_none() {
                    self.on = fs;
                    self.moving = Some(Moving::new(ctx));
                }
            }
            return;
        };
        if frames > 0 {
            self.pending = Some((step, frames - 1));
            ctx.request_repaint();
            return;
        }
        self.pending = None;
        match step {
            Step::Enter => ctx.send_viewport_cmd(ViewportCommand::Fullscreen(true)),
            Step::Maximize => ctx.send_viewport_cmd(ViewportCommand::Maximized(true)),
        }
        ctx.request_repaint();
    }

    /// Is the window still changing size? Then the content should not be laid
    /// out in this frame.
    pub fn settling(&mut self, ctx: &egui::Context) -> bool {
        let Some(mut m) = self.moving else {
            return false;
        };
        ctx.request_repaint();
        let size = ctx.content_rect().size();
        let reached = ctx
            .input(|i| i.viewport().fullscreen)
            .is_none_or(|fs| fs == self.on);
        m.changed |= size != m.start;
        m.stable = if size == m.last && m.changed && reached && self.pending.is_none() {
            m.stable.saturating_add(1)
        } else {
            0
        };
        m.last = size;
        if m.stable >= STABLE_FRAMES || m.since.elapsed() > MAX_WAIT {
            self.moving = None;
            return false;
        }
        self.moving = Some(m);
        true
    }
}
