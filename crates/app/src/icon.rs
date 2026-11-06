//! App-Logo: Fenster-Icon und Textur für die Kopfzeile.

use std::sync::Arc;

use eframe::egui;

pub const ICON_PNG: &[u8] = include_bytes!("../assets/icon-256.png");

pub fn window_icon() -> Arc<egui::IconData> {
    Arc::new(eframe::icon_data::from_png_bytes(ICON_PNG).unwrap_or_default())
}

pub fn texture(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let icon = eframe::icon_data::from_png_bytes(ICON_PNG).ok()?;
    let img = egui::ColorImage::from_rgba_unmultiplied(
        [icon.width as usize, icon.height as usize],
        &icon.rgba,
    );
    Some(ctx.load_texture("dmxcraft-logo", img, egui::TextureOptions::LINEAR))
}
