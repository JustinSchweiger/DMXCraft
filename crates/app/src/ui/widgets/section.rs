use eframe::egui::{self, Ui};

use crate::ui::theme;

/// Sektionskopf: Titel in Großbuchstaben links, Aktionen rechts.
pub fn section_header(ui: &mut Ui, title: &str, right: impl FnOnce(&mut Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(24.0);
        if !title.is_empty() {
            ui.label(theme::heading_text(title));
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), right);
    });
}

/// Aufklappbare Sektion: Pfeil und Titel (GROSS) als Kopf, Zustand bleibt gemerkt.
pub fn fold_section<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    title: &str,
    default_open: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let id = ui.make_persistent_id(id);
    let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
        ui.ctx(),
        id,
        default_open,
    );
    let open = state.is_open();
    let r = ui
        .horizontal(|ui| {
            ui.set_min_height(24.0);
            let icon = if open {
                crate::ui::widgets::Icon::ChevronDown
            } else {
                crate::ui::widgets::Icon::ChevronRight
            };
            let a = ui.add(crate::ui::widgets::IconButton::new(icon).size(18.0).flat());
            let b =
                ui.add(egui::Label::new(theme::heading_text(title)).sense(egui::Sense::click()));
            a.clicked() || b.clicked()
        })
        .inner;
    if r {
        state.toggle(ui);
    }
    state.store(ui.ctx());
    let res = state.show_body_unindented(ui, body).map(|r| r.inner);
    if res.is_some() {
        ui.add_space(theme::SP_S);
    }
    res
}
