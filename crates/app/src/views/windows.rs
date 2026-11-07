//! Fenster für Bereiche, die (noch) keine eigene Seite haben:
//! Szenen-Editor und Zonen & Gruppen.

use crate::i18n::t;
use eframe::egui;

use crate::ui::widgets::Dialog;
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
            Dialog::new(t("Szenen bearbeiten"))
                .id("scene_editor")
                .resizable([860.0, 560.0])
                .cancel(t("Schließen"))
                .show(ctx, &mut open, |ui, _| self.scenes.ui(ui, cx));
            cx.state.scene_editor_open = open;
        }
        if cx.state.zones_open {
            let mut open = true;
            Dialog::new(t("Bänke & Gruppen"))
                .id("zones")
                .resizable([860.0, 460.0])
                .cancel(t("Schließen"))
                .show(ctx, &mut open, |ui, _| self.zones.ui(ui, cx));
            cx.state.zones_open = open;
        }
    }
}
