//! STEUERN → Bühnenplan: Gruppen und Geräteauswahl (Anordnen unter EINRICHTEN).

use crate::i18n::t;
use std::collections::BTreeSet;

use dmxcraft_core::{Engine, FixtureId};
use eframe::egui;

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{section_header, CanvasMode, StageCanvas, StageInteraction};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct StageView {
    interaction: StageInteraction,
}

impl View for StageView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        section_header(ui, t("Bühne"), |ui| {
            selection_tools(ui, cx.engine, cx.state)
        });
        group_bar(ui, cx.engine, cx.state);
        ui.add_space(theme::SP_XS);
        let show = cx.engine.show();
        let canvas = StageCanvas {
            show,
            frame: cx.frame,
            mode: CanvasMode::Select,
            snap: None,
            show_labels: false,
            show_grid: false,
            auto_fit: true,
        };
        let mut action = None;
        let clip = cx.state.layout_clipboard.is_some();
        let locked = cx.state.locked;
        // Im Auswahlmodus entstehen keine Verschiebungen.
        let _ = canvas.show(
            ui,
            &mut self.interaction,
            &mut cx.state.selected,
            &mut |ui, target, sel| {
                if locked {
                    ui.label(t("Oberfläche ist gesperrt"));
                    return;
                }
                if let Some(a) = fixture_menu::menu(ui, show, sel, target, MenuPlace::Stage, clip) {
                    action = Some(a);
                }
            },
        );
        if let Some(a) = action {
            fixture_menu::apply(cx.engine, cx.state, a);
        }
    }
}

fn selection_tools(ui: &mut egui::Ui, e: &Engine, st: &mut AppState) {
    // rechts-nach-links: zuletzt hinzugefügt steht ganz links
    if ui
        .button(t("Jedes 2."))
        .on_hover_text(t("Jedes zweite Gerät der Auswahl"))
        .clicked()
    {
        st.selected = st.selected.iter().copied().step_by(2).collect();
    }
    if ui.button(t("Umkehren")).clicked() {
        st.selected = e
            .show()
            .fixtures
            .iter()
            .map(|f| f.id)
            .filter(|id| !st.selected.contains(id))
            .collect();
    }
    if ui.button(t("Keine")).clicked() {
        st.selected.clear();
    }
    ui.label(
        egui::RichText::new(tr!("{n} ausgewählt", n = st.selected.len())).color(theme::text_dim()),
    );
}

fn group_bar(ui: &mut egui::Ui, e: &Engine, st: &mut AppState) {
    ui.horizontal_wrapped(|ui| {
        let all: BTreeSet<FixtureId> = e.show().fixtures.iter().map(|f| f.id).collect();
        if ui
            .selectable_label(!all.is_empty() && st.selected == all, t("Alle"))
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
                egui::RichText::new(t("Gruppen legst du unter EINRICHTEN → Zonen & Gruppen an"))
                    .color(theme::text_faint()),
            );
        }
    });
}
