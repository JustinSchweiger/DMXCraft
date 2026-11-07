//! EINRICHTEN: eine Seite mit Patch-Raster (oben), Bühnenplan mit
//! Geräteeinstellungen (unten) und Bibliothek (links). Alle Bereiche sind
//! an ihren Trennlinien in der Größe veränderbar.

pub mod fixture_settings;
pub mod library_panel;
pub mod patch_grid;
pub mod stage_editor;
pub mod zones;

use eframe::egui;

use crate::ui::theme;
use crate::views::fixture_menu::FixtureAction;
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
        // Entf löscht die ausgewählten Geräte (ohne Rückfrage, Strg+Z holt sie zurück)
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        if !typing
            && !cx.state.selected.is_empty()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
        {
            crate::views::fixture_menu::apply(cx.engine, cx.state, FixtureAction::Delete);
        }
        // Zwischenablage (Strg+X/C/V kommen von egui als Ereignisse), Strg+D, Strg+A
        if !typing && !cx.state.locked {
            let (cut, copy, paste, dup, all) = ui.input(|i| {
                let ev = |f: fn(&egui::Event) -> bool| i.events.iter().any(f);
                let key = |k| i.modifiers.command && i.key_pressed(k);
                (
                    ev(|e| matches!(e, egui::Event::Cut)) || key(egui::Key::X),
                    ev(|e| matches!(e, egui::Event::Copy)) || key(egui::Key::C),
                    ev(|e| matches!(e, egui::Event::Paste(_))) || key(egui::Key::V),
                    key(egui::Key::D),
                    key(egui::Key::A),
                )
            });
            let some = !cx.state.selected.is_empty();
            let action = if cut && some {
                Some(FixtureAction::Cut)
            } else if copy && some {
                Some(FixtureAction::Copy)
            } else if paste && !cx.state.fixture_clipboard.is_empty() {
                Some(FixtureAction::Paste)
            } else if dup && some {
                Some(FixtureAction::Duplicate)
            } else if all {
                Some(FixtureAction::SelectAll)
            } else {
                None
            };
            if let Some(a) = action {
                if matches!(a, FixtureAction::Cut | FixtureAction::Copy) {
                    // egui meldet Strg+V nur, wenn die System-Zwischenablage Text enthält
                    ui.ctx().copy_text("DMXCraft".into());
                }
                crate::views::fixture_menu::apply(cx.engine, cx.state, a);
            }
        }
        let area = egui::Frame::new()
            .fill(theme::surface())
            .inner_margin(egui::Margin::same(6));
        egui::Panel::left("setup_library")
            .resizable(true)
            .default_size(340.0)
            .min_size(240.0)
            .frame(area)
            .show(ui, |ui| self.library.ui(ui, cx));
        egui::Panel::bottom("setup_stage")
            .resizable(true)
            .default_size(ui.available_height() * 0.48)
            .min_size(200.0)
            .frame(egui::Frame::new().fill(theme::surface()))
            .show(ui, |ui| {
                egui::Panel::right("setup_fixture_settings")
                    .resizable(true)
                    .default_size(560.0)
                    .min_size(320.0)
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
