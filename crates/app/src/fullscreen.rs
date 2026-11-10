//! Toggle fullscreen.
//!
//! From a maximized, borderless window Windows does not fully take over the
//! fullscreen size: a black strip the height of the taskbar stays at the
//! bottom. So the maximization is undone before going fullscreen (and
//! restored afterwards) – each with a few frames in between so the system
//! has taken over the intermediate state.

use eframe::egui::{self, ViewportCommand};

/// Frames between the steps.
const SETTLE: u8 = 3;

#[derive(Default)]
pub struct Fullscreen {
    on: bool,
    /// Maximized before fullscreen (maximize again when leaving).
    was_maximized: bool,
    /// Pending step and remaining frames until then.
    pending: Option<(Step, u8)>,
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
        if self.on {
            self.on = false;
            ctx.send_viewport_cmd(ViewportCommand::Fullscreen(false));
            if self.was_maximized {
                self.pending = Some((Step::Maximize, SETTLE));
            }
        } else {
            self.on = true;
            self.was_maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
            if self.was_maximized {
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
            // Take over the state when the window went fullscreen from outside (e.g.
            // at startup from the remembered window size)
            if let Some(fs) = ctx.input(|i| i.viewport().fullscreen) {
                self.on = fs;
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
}
