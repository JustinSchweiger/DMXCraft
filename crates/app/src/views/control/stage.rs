//! STEUERN → Bühnenplan: Gruppen und Geräteauswahl (Anordnen unter EINRICHTEN).

use std::collections::BTreeSet;

use dmxcraft_core::{Engine, FixtureId};
use eframe::egui;

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{section_header, CanvasMode, StageCanvas, StageInteraction};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct StageView {
    interaction: StageInteraction,
}

impl View for StageView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        section_header(ui, "Bühne", |ui| selection_tools(ui, cx.engine, cx.state));
        group_bar(ui, cx.engine, cx.state);
        ui.add_space(theme::SP_XS);
        let canvas = StageCanvas {
            show: cx.engine.show(),
            frame: cx.frame,
            mode: CanvasMode::Select,
            snap: None,
            show_labels: false,
        };
        // Im Auswahlmodus entstehen keine Verschiebungen.
        let _ = canvas.show(ui, &mut self.interaction, &mut cx.state.selected);
    }
}

fn selection_tools(ui: &mut egui::Ui, e: &Engine, st: &mut AppState) {
    // rechts-nach-links: zuletzt hinzugefügt steht ganz links
    if ui
        .button("Jedes 2.")
        .on_hover_text("Jedes zweite Gerät der Auswahl")
        .clicked()
    {
        st.selected = st.selected.iter().copied().step_by(2).collect();
    }
    if ui.button("Umkehren").clicked() {
        st.selected = e
            .show()
            .fixtures
            .iter()
            .map(|f| f.id)
            .filter(|id| !st.selected.contains(id))
            .collect();
    }
    if ui.button("Keine").clicked() {
        st.selected.clear();
    }
    ui.label(
        egui::RichText::new(format!("{} ausgewählt", st.selected.len())).color(theme::TEXT_DIM),
    );
}

fn group_bar(ui: &mut egui::Ui, e: &Engine, st: &mut AppState) {
    ui.horizontal_wrapped(|ui| {
        let all: BTreeSet<FixtureId> = e.show().fixtures.iter().map(|f| f.id).collect();
        if ui
            .selectable_label(!all.is_empty() && st.selected == all, "Alle")
            .clicked()
        {
            st.selected = all;
        }
        for g in &e.show().groups {
            let set: BTreeSet<FixtureId> = g.fixtures.iter().copied().collect();
            if ui
                .selectable_label(!set.is_empty() && st.selected == set, &g.name)
                .clicked()
            {
                st.selected = set;
            }
        }
        if e.show().groups.is_empty() {
            ui.label(
                egui::RichText::new("Gruppen legst du unter EINRICHTEN → Zonen & Gruppen an")
                    .color(theme::TEXT_FAINT),
            );
        }
    });
}
