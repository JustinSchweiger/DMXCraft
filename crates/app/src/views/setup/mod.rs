//! EINRICHTEN: Patch, Bühnenplan, Szenen, Zonen & Gruppen, DMX-Monitor.

pub mod monitor;
pub mod patch;
pub mod scenes;
pub mod stage_editor;
pub mod zones;

use eframe::egui;

use crate::state::SetupTab;
use crate::ui::theme;
use crate::ui::widgets::Segmented;
use crate::views::{View, ViewCtx};

/// Rahmen für EINRICHTEN mit Unterbereichen als Karten.
#[derive(Default)]
pub struct SetupView {
    patch: patch::PatchView,
    stage: stage_editor::StageEditorView,
    scenes: scenes::ScenesView,
    zones: zones::ZonesView,
    monitor: monitor::MonitorView,
}

const TABS: [(SetupTab, &str); 5] = [
    (SetupTab::Patch, "Patch"),
    (SetupTab::Stage, "Bühnenplan"),
    (SetupTab::Scenes, "Szenen"),
    (SetupTab::Zones, "Zonen & Gruppen"),
    (SetupTab::Monitor, "DMX-Monitor"),
];

impl View for SetupView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        Segmented::new(&TABS).show(ui, &mut cx.state.setup_tab);
        ui.add_space(theme::SP_XS);
        ui.separator();
        let view: &mut dyn View = match cx.state.setup_tab {
            SetupTab::Patch => &mut self.patch,
            SetupTab::Stage => &mut self.stage,
            SetupTab::Scenes => &mut self.scenes,
            SetupTab::Zones => &mut self.zones,
            SetupTab::Monitor => &mut self.monitor,
        };
        view.ui(ui, cx);
    }
}
