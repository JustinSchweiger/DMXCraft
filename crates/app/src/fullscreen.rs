//! Vollbild umschalten.
//!
//! Aus einem maximierten, randlosen Fenster heraus übernimmt Windows die
//! Vollbildgröße nicht ganz: unten bleibt ein schwarzer Streifen in Höhe der
//! Taskleiste. Daher wird die Maximierung vor dem Vollbild aufgehoben (und
//! danach wiederhergestellt) – jeweils mit ein paar Frames Abstand, damit das
//! System den Zwischenzustand übernommen hat.

use eframe::egui::{self, ViewportCommand};

/// Frames zwischen den Schritten.
const SETTLE: u8 = 3;

#[derive(Default)]
pub struct Fullscreen {
    on: bool,
    /// Vor dem Vollbild maximiert (beim Verlassen wieder maximieren).
    was_maximized: bool,
    /// Ausstehender Schritt und verbleibende Frames bis dahin.
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

    /// Jeden Frame aufrufen: führt ausstehende Schritte aus.
    pub fn update(&mut self, ctx: &egui::Context) {
        let Some((step, frames)) = self.pending else {
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
