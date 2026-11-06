//! EINRICHTEN → Bühnenplan: Geräte anordnen.

use dmxcraft_core::{plan, FixtureId};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::stage_canvas::fixture_pos;
use crate::ui::widgets::{section_header, CanvasMode, StageCanvas, StageInteraction};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

pub struct StageEditorView {
    interaction: StageInteraction,
    snap: bool,
    cells: u32,
    labels: bool,
    grid: bool,
}

impl Default for StageEditorView {
    fn default() -> Self {
        Self {
            interaction: StageInteraction::default(),
            snap: true,
            cells: 24,
            labels: true,
            grid: false,
        }
    }
}

enum Arrange {
    Row,
    Grid,
    AlignH,
    AlignV,
    Reset,
}

impl View for StageEditorView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let mut arrange: Option<Arrange> = None;
        section_header(ui, "Bühnenplan", |ui| {
            ui.checkbox(&mut self.labels, "Namen");
            ui.checkbox(&mut self.grid, "Raster");
            ui.add_enabled(
                self.snap,
                egui::DragValue::new(&mut self.cells)
                    .range(4..=64)
                    .suffix(" Felder"),
            );
            ui.checkbox(&mut self.snap, "Einrasten");
        });
        ui.horizontal_wrapped(|ui| {
            let some = !cx.state.selected.is_empty();
            ui.label(egui::RichText::new("Auswahl:").color(theme::TEXT_DIM));
            if ui
                .add_enabled(some, egui::Button::new("In Reihe"))
                .clicked()
            {
                arrange = Some(Arrange::Row);
            }
            if ui
                .add_enabled(some, egui::Button::new("Als Raster"))
                .clicked()
            {
                arrange = Some(Arrange::Grid);
            }
            if ui
                .add_enabled(some, egui::Button::new("Waagerecht ausrichten"))
                .clicked()
            {
                arrange = Some(Arrange::AlignH);
            }
            if ui
                .add_enabled(some, egui::Button::new("Senkrecht ausrichten"))
                .clicked()
            {
                arrange = Some(Arrange::AlignV);
            }
            if ui
                .add_enabled(some, egui::Button::new("Position zurücksetzen"))
                .on_hover_text("Automatische Position nach Patch-Reihenfolge")
                .clicked()
            {
                arrange = Some(Arrange::Reset);
            }
            ui.separator();
            ui.label(
                egui::RichText::new(
                    "Hilfe",
                )
                .color(theme::TEXT_FAINT),
            )
            .on_hover_text("Ziehen = verschieben\nAufziehen = auswählen\nStrg/Shift = zur Auswahl hinzufügen\nRechtsklick = Menü (Form, Größe, Gruppen …)");
        });
        ui.add_space(theme::SP_XS);

        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                let show = cx.engine.show();
                let clip = cx.state.layout_clipboard.is_some();
                let mut action = None;
                let moves = StageCanvas {
                    show,
                    frame: cx.frame,
                    mode: CanvasMode::Edit,
                    snap: self.snap.then_some(self.cells),
                    show_labels: self.labels,
                    show_grid: self.grid,
                }
                .show(
                    ui,
                    &mut self.interaction,
                    &mut cx.state.selected,
                    &mut |ui, target, sel| {
                        if let Some(a) =
                            fixture_menu::menu(ui, show, sel, target, MenuPlace::StageEditor, clip)
                        {
                            action = Some(a);
                        }
                    },
                );
                for (id, p) in moves {
                    set_pos(cx, id, p);
                }
                if let Some(a) = action {
                    fixture_menu::apply(cx.engine, cx.state, a);
                }
            });

        if let Some(a) = arrange {
            let ids: Vec<FixtureId> = cx
                .engine
                .show()
                .fixtures
                .iter()
                .map(|f| f.id)
                .filter(|id| cx.state.selected.contains(id))
                .collect();
            let cur: Vec<[f32; 2]> = ids
                .iter()
                .map(|id| fixture_pos(cx.engine.show(), *id))
                .collect();
            let avg = |i: usize| cur.iter().map(|p| p[i]).sum::<f32>() / cur.len().max(1) as f32;
            let targets: Vec<Option<[f32; 2]>> = match a {
                Arrange::Row => plan::row(ids.len(), avg(1)).into_iter().map(Some).collect(),
                Arrange::Grid => plan::grid(ids.len()).into_iter().map(Some).collect(),
                Arrange::AlignH => cur.iter().map(|p| Some([p[0], avg(1)])).collect(),
                Arrange::AlignV => cur.iter().map(|p| Some([avg(0), p[1]])).collect(),
                Arrange::Reset => vec![None; ids.len()],
            };
            for (id, t) in ids.into_iter().zip(targets) {
                if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                    f.plan_pos = t;
                }
            }
        }
    }
}

fn set_pos(cx: &mut ViewCtx<'_>, id: FixtureId, p: [f32; 2]) {
    if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
        f.plan_pos = Some(p);
    }
}
