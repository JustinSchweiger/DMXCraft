//! EINRICHTEN → Bühnenplan: Geräte anordnen.

use crate::i18n::t;
use dmxcraft_core::{plan, FixtureId};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::stage_canvas::fixture_pos;
use crate::ui::widgets::{CanvasMode, Icon, IconButton, StageCanvas, StageInteraction};
use crate::views::fixture_menu::{self, MenuPlace};
use crate::views::{View, ViewCtx};

pub struct StageEditorView {
    interaction: StageInteraction,
    snap: bool,
    cells: u32,
    grid: bool,
}

impl Default for StageEditorView {
    fn default() -> Self {
        Self {
            interaction: StageInteraction::default(),
            snap: true,
            cells: 24,
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
        // Werkzeuge senkrecht links: Anordnen, Ansicht (Einrasten, Raster,
        // Namen), Zoom, Hilfe
        egui::Panel::left("stage_tools")
            .resizable(false)
            .exact_size(34.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: 6,
                ..Default::default()
            }))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let some = !cx.state.selected.is_empty();
                for (icon, hint, a) in [
                    (Icon::ArrangeRow, t("In Reihe verteilen"), Arrange::Row),
                    (Icon::ArrangeGrid, t("Als Raster anordnen"), Arrange::Grid),
                    (
                        Icon::AlignH,
                        t("Waagerecht ausrichten (gleiche Höhe)"),
                        Arrange::AlignH,
                    ),
                    (
                        Icon::AlignV,
                        t("Senkrecht ausrichten (untereinander)"),
                        Arrange::AlignV,
                    ),
                    (
                        Icon::Reset,
                        t("Position zurücksetzen (nach Patch-Reihenfolge)"),
                        Arrange::Reset,
                    ),
                ] {
                    if ui
                        .add(IconButton::new(icon).size(26.0).enabled(some))
                        .on_hover_text(hint)
                        .clicked()
                    {
                        arrange = Some(a);
                    }
                }
                tool_sep(ui);
                for (on, icon, hint) in [
                    (&mut self.snap, Icon::Magnet, t("Einrasten")),
                    (&mut self.grid, Icon::GridFour, t("Raster")),
                ] {
                    if ui
                        .add(IconButton::new(icon).size(26.0).active(*on))
                        .on_hover_text(hint)
                        .clicked()
                    {
                        *on = !*on;
                    }
                }
                let mut cells = self.cells;
                if crate::ui::widgets::drag::drag_number(
                    ui,
                    "snap_cells",
                    &mut cells,
                    4..=64,
                    self.snap,
                    egui::vec2(26.0, 22.0),
                )
                .on_hover_text(t("Felder zum Einrasten"))
                .changed()
                {
                    self.cells = cells;
                }
                tool_sep(ui);
                let st = &mut self.interaction;
                if ui
                    .add(IconButton::new(Icon::ZoomFit).size(26.0))
                    .on_hover_text(t("Alle Geräte einpassen"))
                    .clicked()
                {
                    st.fit_request = true;
                }
                if ui
                    .add(IconButton::new(Icon::ZoomIn).size(26.0))
                    .on_hover_text(tr!("Vergrößern ({pct} %)", pct = (st.zoom * 100.0).round()))
                    .clicked()
                {
                    st.zoom_by(1.25);
                }
                if ui
                    .add(IconButton::new(Icon::ZoomOut).size(26.0))
                    .on_hover_text(tr!("Verkleinern ({pct} %)", pct = (st.zoom * 100.0).round()))
                    .clicked()
                {
                    st.zoom_by(0.8);
                }
                tool_sep(ui);
                ui.add(IconButton::new(Icon::Info).size(26.0).flat()).on_hover_text(
                    t("Ziehen = verschieben\nAufziehen = auswählen\nStrg/Shift = zur Auswahl hinzufügen\nMittlere Maustaste = Ansicht ziehen\nMausrad = zoomen\nRechtsklick = Menü (Form, Größe, Gruppen …)"),
                );
            });

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
                    show_grid: self.grid,
                    auto_fit: false,
                    order: &[],
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

/// Kurze waagerechte Trennlinie in der Werkzeugleiste.
fn tool_sep(ui: &mut egui::Ui) {
    let (r, _) = ui.allocate_exact_size(egui::vec2(26.0, 7.0), egui::Sense::hover());
    ui.painter().line_segment(
        [
            egui::pos2(r.left() + 4.0, r.center().y),
            egui::pos2(r.right() - 4.0, r.center().y),
        ],
        egui::Stroke::new(1.0, theme::border()),
    );
}
