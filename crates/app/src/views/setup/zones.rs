//! EINRICHTEN → Zonen & Gruppen.

use crate::i18n::t;
use dmxcraft_core::{FixtureId, GroupId, ZoneId};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::{section_header, text_field};
use crate::views::{View, ViewCtx};

pub struct ZonesView {
    new_zone: String,
    new_group: String,
}

impl Default for ZonesView {
    fn default() -> Self {
        Self {
            new_zone: t("New bank").into(),
            new_group: String::new(),
        }
    }
}

impl View for ZonesView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let mut remove_zone: Option<ZoneId> = None;
        let mut remove_group: Option<GroupId> = None;
        let mut move_zone: Option<(usize, usize)> = None;
        let e = &mut *cx.engine;
        let selected: Vec<FixtureId> = cx.state.selected.iter().copied().collect();

        ui.columns(2, |cols| {
            // ---------- Zonen ----------
            let ui = &mut cols[0];
            section_header(ui, t("Banks"), |_| {});
            ui.label(
                egui::RichText::new(t("Each bank is a column under CONTROL. Scenes in the same bank are mutually exclusive."))
                    .color(theme::text_dim()),
            );
            ui.add_space(theme::SP_S);
            let n = e.show().zones.len();
            let ids: Vec<ZoneId> = e.show().zones.iter().map(|z| z.id).collect();
            for (i, id) in ids.iter().enumerate() {
                let fallback = theme::ZONE_PALETTE[i % theme::ZONE_PALETTE.len()];
                let count = e.show().scenes.iter().filter(|s| s.zone == Some(*id)).count();
                let Some(z) = e.show_mut().zones.iter_mut().find(|z| z.id == *id) else {
                    continue;
                };
                ui.horizontal(|ui| {
                    let mut c = z.color.unwrap_or(fallback);
                    if ui.color_edit_button_srgb(&mut c).changed() {
                        z.color = Some(c);
                    }
                    ui.add(text_field(&mut z.name).desired_width(180.0));
                    ui.label(egui::RichText::new(tr!("{count} scenes", count = count)).color(theme::text_dim()));
                    if ui.add_enabled(i > 0, egui::Button::new(t("Move left"))).clicked() {
                        move_zone = Some((i, i - 1));
                    }
                    if ui.add_enabled(i + 1 < n, egui::Button::new(t("Move right"))).clicked() {
                        move_zone = Some((i, i + 1));
                    }
                    if ui.button(t("Delete")).on_hover_text(t("Scenes become “free”")).clicked() {
                        remove_zone = Some(*id);
                    }
                });
            }
            ui.add_space(theme::SP_S);
            ui.horizontal(|ui| {
                ui.add(text_field(&mut self.new_zone).desired_width(180.0));
                if ui.button(t("Add bank")).clicked() && !self.new_zone.trim().is_empty() {
                    e.show_mut().add_zone(self.new_zone.trim().to_string());
                }
            });

            // ---------- Gruppen ----------
            let ui = &mut cols[1];
            section_header(ui, t("Groups"), |_| {});
            ui.label(
                egui::RichText::new(t("Groups select several fixtures with one click (CONTROL, above the stage plan)."))
                    .color(theme::text_dim()),
            );
            ui.add_space(theme::SP_S);
            let names: Vec<(FixtureId, String)> =
                e.show().fixtures.iter().map(|f| (f.id, f.name.clone())).collect();
            for g in &mut e.show_mut().groups {
                ui.horizontal(|ui| {
                    ui.add(text_field(&mut g.name).desired_width(160.0));
                    let members: Vec<String> = g
                        .fixtures
                        .iter()
                        .filter_map(|id| names.iter().find(|(i, _)| i == id).map(|(_, n)| n.clone()))
                        .collect();
                    ui.label(egui::RichText::new(tr!("{n} fixtures", n = members.len())).color(theme::text_dim()))
                        .on_hover_text(members.join("\n"));
                    if ui
                        .add_enabled(!selected.is_empty(), egui::Button::new(t("Set to selection")))
                        .clicked()
                    {
                        g.fixtures = selected.clone();
                    }
                    if ui.button(t("Delete")).clicked() {
                        remove_group = Some(g.id);
                    }
                });
            }
            ui.add_space(theme::SP_S);
            ui.horizontal(|ui| {
                ui.add(text_field(&mut self.new_group).hint_text(t("Name")).desired_width(160.0));
                if ui
                    .add_enabled(!selected.is_empty(), egui::Button::new(t("Group from selection")))
                    .on_hover_text(t("Select fixtures in the patch or stage plan"))
                    .clicked()
                {
                    let name = if self.new_group.trim().is_empty() {
                        tr!("Group {n}", n = e.show().groups.len() + 1)
                    } else {
                        self.new_group.trim().to_string()
                    };
                    e.show_mut().add_group(name, selected.clone());
                    self.new_group.clear();
                }
            });
        });

        if let Some((a, b)) = move_zone {
            e.show_mut().zones.swap(a, b);
        }
        if let Some(z) = remove_zone {
            e.show_mut().remove_zone(z);
        }
        if let Some(g) = remove_group {
            e.show_mut().remove_group(g);
        }
    }
}
