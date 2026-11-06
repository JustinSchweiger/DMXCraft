//! EINRICHTEN: eine Seite mit Patch-Raster (oben), Bühnenplan mit
//! Geräteeinstellungen (unten) und Bibliothek (rechts).

pub mod fixture_settings;
pub mod library_panel;
pub mod patch_grid;
pub mod scenes;
pub mod stage_editor;
pub mod zones;

use eframe::egui;

use crate::ui::theme;
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct SetupView {
    grid: patch_grid::PatchGrid,
    stage: stage_editor::StageEditorView,
    settings: fixture_settings::FixtureSettings,
    library: library_panel::LibraryPanel,
}

impl View for SetupView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let area = egui::Frame::new()
            .fill(theme::SURFACE)
            .inner_margin(egui::Margin::same(6));
        egui::Panel::right("setup_library")
            .resizable(true)
            .default_size(340.0)
            .min_size(260.0)
            .frame(area)
            .show(ui, |ui| self.library.ui(ui, cx));
        egui::Panel::bottom("setup_stage")
            .resizable(true)
            .default_size(ui.available_height() * 0.48)
            .min_size(200.0)
            .frame(egui::Frame::new().fill(theme::SURFACE))
            .show(ui, |ui| {
                egui::Panel::right("setup_fixture_settings")
                    .resizable(false)
                    .exact_size(250.0)
                    .frame(area)
                    .show(ui, |ui| self.settings.ui(ui, cx));
                egui::CentralPanel::default_margins()
                    .frame(area)
                    .show(ui, |ui| self.stage.ui(ui, cx));
            });
        egui::CentralPanel::default_margins()
            .frame(area)
            .show(ui, |ui| self.grid.ui(ui, cx));
    }
}
