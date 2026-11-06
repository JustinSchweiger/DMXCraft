//! Fenster für Bereiche, die (noch) keine eigene Seite haben:
//! Szenen-Editor und Zonen & Gruppen.

use eframe::egui;

use crate::views::setup::{scenes::ScenesView, zones::ZonesView};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct Windows {
    scenes: ScenesView,
    zones: ZonesView,
}

impl Windows {
    pub fn show(&mut self, ctx: &egui::Context, cx: &mut ViewCtx<'_>) {
        if cx.state.scene_editor_open {
            let mut open = true;
            egui::Window::new("Szenen bearbeiten")
                .open(&mut open)
                .default_size([860.0, 520.0])
                .show(ctx, |ui| self.scenes.ui(ui, cx));
            cx.state.scene_editor_open = open;
        }
        if cx.state.zones_open {
            let mut open = true;
            egui::Window::new("Zonen & Gruppen")
                .open(&mut open)
                .default_size([860.0, 420.0])
                .show(ctx, |ui| self.zones.ui(ui, cx));
            cx.state.zones_open = open;
        }
    }
}
