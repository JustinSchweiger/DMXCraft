use eframe::egui::{self, Ui};

use crate::ui::theme;

/// Sektionskopf: Titel in Großbuchstaben links, Aktionen rechts.
pub fn section_header(ui: &mut Ui, title: &str, right: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(24.0);
        ui.label(theme::heading_text(title));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), right);
    });
}
