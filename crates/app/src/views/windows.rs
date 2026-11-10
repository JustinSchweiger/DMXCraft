//! Fenster für Bereiche, die (noch) keine eigene Seite haben:
//! Bänke & Gruppen.

use crate::i18n::t;
use eframe::egui;

use crate::ui::widgets::Dialog;
use crate::views::setup::zones::ZonesView;
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct Windows {
    zones: ZonesView,
}

impl Windows {
    pub fn show(&mut self, ctx: &egui::Context, cx: &mut ViewCtx<'_>) {
        if cx.state.zones_open {
            let mut open = true;
            Dialog::new(t("Banks & groups"))
                .id("zones")
                .resizable([860.0, 460.0])
                .cancel(t("Close"))
                .show(ctx, &mut open, |ui, _| self.zones.ui(ui, cx));
            cx.state.zones_open = open;
        }
    }
}
