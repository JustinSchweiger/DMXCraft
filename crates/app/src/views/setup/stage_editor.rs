//! EINRICHTEN → Bühnenplan: Geräte anordnen.

use crate::i18n::t;
use dmxcraft_core::{plan, FixtureId};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::stage_canvas::{fixture_pos, ZOOM_MAX, ZOOM_MIN};
use crate::ui::widgets::{
    section_header, CanvasMode, Icon, IconButton, StageCanvas, StageInteraction, Toggle,
};
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

/// Zoom-Leiste unten rechts im Bühnenplan: Einpassen, −, Regler, +.
fn zoom_bar(ui: &mut egui::Ui, area: egui::Rect, st: &mut StageInteraction) {
    let size = egui::vec2(210.0, 32.0);
    let bar = egui::Rect::from_min_size(area.right_bottom() - size - egui::vec2(8.0, 8.0), size);
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(bar)
            .layout(egui::Layout::left_to_right(egui::Align::Center)),
    );
    egui::Frame::new()
        .fill(theme::raised())
        .stroke(egui::Stroke::new(1.0, theme::border()))
        .corner_radius(theme::R_M)
        .inner_margin(egui::Margin::symmetric(4, 2))
        .show(&mut child, |ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            if ui
                .add(IconButton::new(Icon::ZoomFit).flat())
                .on_hover_text(t("Alle Geräte einpassen"))
                .clicked()
            {
                st.fit_request = true;
            }
            if ui
                .add(IconButton::new(Icon::ZoomOut).flat())
                .on_hover_text(t("Verkleinern"))
                .clicked()
            {
                st.zoom_by(0.8);
            }
            // Logarithmisch: gleiche Wege für Vergrößern und Verkleinern
            let mut z = st.zoom;
            ui.spacing_mut().slider_width = 100.0;
            if ui
                .add(
                    egui::Slider::new(&mut z, ZOOM_MIN..=ZOOM_MAX)
                        .logarithmic(true)
                        .show_value(false),
                )
                .on_hover_text(tr!("Zoom {pct} %", pct = (st.zoom * 100.0).round()))
                .changed()
            {
                st.zoom = z;
            }
            if ui
                .add(IconButton::new(Icon::ZoomIn).flat())
                .on_hover_text(t("Vergrößern"))
                .clicked()
            {
                st.zoom_by(1.25);
            }
        });
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
        section_header(ui, t("Bühnenplan"), |ui| {
            ui.label(egui::RichText::new(t("Hilfe")).color(theme::text_faint())).on_hover_text(
                t("Ziehen = verschieben\nAufziehen = auswählen\nStrg/Shift = zur Auswahl hinzufügen\nMittlere Maustaste = Ansicht ziehen\nMausrad = zoomen\nRechtsklick = Menü (Form, Größe, Gruppen …)"),
            );
            ui.separator();
            ui.add(Toggle::new(&mut self.labels, t("Namen")));
            ui.add(Toggle::new(&mut self.grid, t("Raster")));
            ui.add_enabled(
                self.snap,
                egui::DragValue::new(&mut self.cells)
                    .range(4..=64)
                    .suffix(t(" Felder")),
            );
            ui.add(Toggle::new(&mut self.snap, t("Einrasten")));
        });

        // Anordnen: Symbol-Knöpfe senkrecht links
        egui::Panel::left("stage_tools")
            .resizable(false)
            .exact_size(34.0)
            .frame(egui::Frame::new().inner_margin(egui::Margin {
                right: 6,
                ..Default::default()
            }))
            .show(ui, |ui| {
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
                        .add(IconButton::new(icon).size(28.0).enabled(some))
                        .on_hover_text(hint)
                        .clicked()
                    {
                        arrange = Some(a);
                    }
                }
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
                    show_labels: self.labels,
                    show_grid: self.grid,
                    auto_fit: false,
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
                let area = ui.min_rect();
                zoom_bar(ui, area, &mut self.interaction);
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
