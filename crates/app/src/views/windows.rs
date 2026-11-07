//! Fenster für Bereiche, die (noch) keine eigene Seite haben:
//! Bänke & Gruppen, Profile Builder.

use crate::i18n::t;
use eframe::egui;

use crate::ui::widgets::Dialog;
use crate::views::profile_builder::ProfileBuilder;
use crate::views::setup::zones::ZonesView;
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct Windows {
    zones: ZonesView,
    builder: ProfileBuilder,
}

impl Windows {
    pub fn show(&mut self, ctx: &egui::Context, cx: &mut ViewCtx<'_>) {
        if cx.state.zones_open {
            let mut open = true;
            Dialog::new(t("Bänke & Gruppen"))
                .id("zones")
                .resizable([860.0, 460.0])
                .cancel(t("Schließen"))
                .show(ctx, &mut open, |ui, _| self.zones.ui(ui, cx));
            cx.state.zones_open = open;
        }
        if let Some(req) = cx.state.builder.take() {
            self.builder.request(req);
        }
        self.builder.show(ctx, cx.engine, cx.state);
    }
}
