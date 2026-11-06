//! EINRICHTEN → Geräteeinstellungen der Auswahl: Darstellung im Bühnenplan
//! und Ausgangsgrenzen (Dimmer-Limit, Pan/Tilt-Bereich, Invertieren).

use dmxcraft_core::{ChannelKind, FixtureId, FixtureLimits, PlanShape, PLAN_SIZE_RANGE};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::section_header;
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct FixtureSettings;

impl View for FixtureSettings {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let ids: Vec<FixtureId> = cx.state.selected.iter().copied().collect();
        let title = match ids.len() {
            0 => "Gerät".to_string(),
            1 => cx
                .engine
                .show()
                .fixture(ids[0])
                .map_or("Gerät".into(), |f| f.name.clone()),
            n => format!("{n} Geräte"),
        };
        section_header(ui, &title, |_| {});
        let Some(first) = ids.first().copied() else {
            ui.label(
                egui::RichText::new("Geräte im Bühnenplan oder im Patch auswählen.")
                    .color(theme::TEXT_DIM),
            );
            return;
        };
        let show = cx.engine.show();
        let has = |k: ChannelKind| {
            ids.iter().any(|id| {
                show.fixture(*id)
                    .and_then(|f| show.type_of(f))
                    .is_some_and(|t| t.has_kind(k))
            })
        };
        let (has_dimmer, has_pan, has_tilt) = (
            has(ChannelKind::Dimmer) || has(ChannelKind::Red),
            has(ChannelKind::Pan),
            has(ChannelKind::Tilt),
        );
        let mut lim = show.fixture(first).map(|f| f.limits).unwrap_or_default();
        let shape = show.fixture(first).and_then(|f| f.plan_shape);
        let mut size = show.plan_size(first);
        let before = lim;
        let mut new_shape: Option<Option<PlanShape>> = None;
        let mut size_changed = false;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                // Darstellung
                caption(ui, "Darstellung");
                egui::ComboBox::from_id_salt("fs_shape")
                    .width(ui.available_width())
                    .selected_text(shape.map_or("Form: automatisch", |s| s.label()))
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(shape.is_none(), "Automatisch (nach Typ)")
                            .clicked()
                        {
                            new_shape = Some(None);
                        }
                        for s in PlanShape::ALL {
                            if ui.selectable_label(shape == Some(s), s.label()).clicked() {
                                new_shape = Some(Some(s));
                            }
                        }
                    });
                ui.spacing_mut().slider_width = ui.available_width() - 70.0;
                size_changed = ui
                    .add(
                        egui::Slider::new(&mut size, PLAN_SIZE_RANGE)
                            .fixed_decimals(1)
                            .suffix("x"),
                    )
                    .on_hover_text("Größe im Bühnenplan")
                    .changed();

                // Dimmer
                ui.add_space(theme::SP_S);
                caption(ui, "Dimmer-Limit");
                ui.add_enabled_ui(has_dimmer, |ui| {
                    let mut pct = (f32::from(lim.dimmer_max) / 255.0 * 100.0).round();
                    if ui
                        .add(
                            egui::Slider::new(&mut pct, 0.0..=100.0)
                                .integer()
                                .suffix("%"),
                        )
                        .on_hover_text("Höchste Helligkeit dieses Geräts")
                        .changed()
                    {
                        lim.dimmer_max = (pct / 100.0 * 255.0).round() as u8;
                    }
                });

                // Bewegung
                ui.add_space(theme::SP_S);
                caption(ui, "Bewegungsbereich");
                ui.add_enabled_ui(has_pan || has_tilt, |ui| {
                    range(ui, "Pan", &mut lim.pan_min, &mut lim.pan_max, has_pan);
                    range(ui, "Tilt", &mut lim.tilt_min, &mut lim.tilt_max, has_tilt);
                    ui.add_space(theme::SP_XS);
                    ui.add_enabled(
                        has_pan,
                        egui::Checkbox::new(&mut lim.invert_pan, "Pan invertieren"),
                    );
                    ui.add_enabled(
                        has_tilt,
                        egui::Checkbox::new(&mut lim.invert_tilt, "Tilt invertieren"),
                    );
                    ui.add_enabled(
                        has_pan && has_tilt,
                        egui::Checkbox::new(&mut lim.swap_pan_tilt, "Pan und Tilt tauschen"),
                    );
                });
                if !(has_pan || has_tilt) {
                    ui.label(egui::RichText::new("Keine Pan/Tilt-Kanäle").color(theme::TEXT_FAINT));
                }

                ui.add_space(theme::SP_S);
                if ui
                    .add_enabled(!lim.is_default(), egui::Button::new("Grenzen zurücksetzen"))
                    .clicked()
                {
                    lim = FixtureLimits::default();
                }
                if ids.len() > 1 {
                    ui.label(
                        egui::RichText::new("Änderungen gelten für alle ausgewählten Geräte.")
                            .color(theme::TEXT_FAINT),
                    );
                }
            });

        let limits_changed = lim != before;
        for id in ids {
            if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                if limits_changed {
                    f.limits = lim;
                }
                if let Some(s) = new_shape {
                    f.plan_shape = s;
                }
                if size_changed {
                    f.plan_size = Some(size);
                }
            }
        }
    }
}

fn caption(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text.to_uppercase())
            .font(theme::caption())
            .color(theme::TEXT_DIM)
            .strong(),
    );
}

/// Bereich als zwei Werte in Prozent (min/max).
fn range(ui: &mut egui::Ui, label: &str, min: &mut u8, max: &mut u8, enabled: bool) {
    ui.add_enabled_ui(enabled, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(label).color(theme::TEXT_DIM));
            let mut lo = f32::from(*min) / 255.0 * 100.0;
            let mut hi = f32::from(*max) / 255.0 * 100.0;
            let a = ui.add(
                egui::DragValue::new(&mut lo)
                    .range(0.0..=100.0)
                    .speed(0.5)
                    .fixed_decimals(0)
                    .prefix("von ")
                    .suffix("%"),
            );
            let b = ui.add(
                egui::DragValue::new(&mut hi)
                    .range(0.0..=100.0)
                    .speed(0.5)
                    .fixed_decimals(0)
                    .prefix("bis ")
                    .suffix("%"),
            );
            if a.changed() || b.changed() {
                *min = (lo / 100.0 * 255.0).round() as u8;
                *max = (hi / 100.0 * 255.0).round() as u8;
            }
        });
    });
}
