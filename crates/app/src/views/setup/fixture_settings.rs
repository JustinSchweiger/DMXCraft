//! EINRICHTEN → Geräteeinstellungen der Auswahl: Schnelltest (Licht aus/an,
//! Moving Head zentrieren), Darstellung und Ausgangsgrenzen.

use crate::i18n::t;
use dmxcraft_core::{ChannelKind, FixtureId, FixtureLimits};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, LimitPad, RangeSlider, Toggle};
use crate::views::quick::{quick_action, Layer, Quick};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct FixtureSettings;

/// Schnelltest über den Programmer.
impl View for FixtureSettings {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        // Der Inhalt darf die Panelgröße nicht erzwingen: ist der Bereich
        // kleiner als der Inhalt, wird gescrollt statt das Panel zu sperren
        egui::ScrollArea::both()
            .id_salt("fixture_settings_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| body(ui, cx));
    }
}

fn body(ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
    {
        let ids: Vec<FixtureId> = cx.state.selected.iter().copied().collect();
        let mut quick = None;
        let some = !ids.is_empty();
        // Schnelltest oben, wie im Programmer: Licht aus/an, zentrieren
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            for (q, icon, tip) in [
                (Quick::Off, Icon::LightOff, t("Licht aus")),
                (
                    Quick::On,
                    Icon::LightOn,
                    t("Licht an (Dimmer und Weiß voll)"),
                ),
                (
                    Quick::Center,
                    Icon::Center,
                    t("Moving Head zentrieren (Pan/Tilt 50 %)"),
                ),
            ] {
                if ui
                    .add(IconButton::new(icon).size(34.0).flat().enabled(some))
                    .on_hover_text(tip)
                    .clicked()
                {
                    quick = Some(q);
                }
            }
        });
        ui.add_space(theme::SP_S);
        if let Some(q) = quick {
            quick_action(cx.engine, Layer::Programmer, &ids, q);
        }

        let Some(first) = ids.first().copied() else {
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
        let has_dimmer =
            has(ChannelKind::Dimmer) || has(ChannelKind::Red) || has(ChannelKind::White);
        let has_pan = has(ChannelKind::Pan);
        let has_tilt = has(ChannelKind::Tilt);
        let mut lim = show.fixture(first).map(|f| f.limits).unwrap_or_default();
        let before = lim;

        // Grenzen: Dimmer | Bewegung (XY) | Werte & Schalter
        let h = (ui.available_height() - 4.0).clamp(120.0, 420.0);
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                caption(ui, t("Dimmer Limit"));
                RangeSlider {
                    min: &mut lim.dimmer_min,
                    max: &mut lim.dimmer_max,
                    height: h - 24.0,
                    enabled: has_dimmer,
                }
                .show(ui);
            });
            ui.separator();
            ui.vertical(|ui| {
                caption(ui, t("Bewegung Limit"));
                let side = (h - 24.0).min(ui.available_width() - 150.0).max(110.0);
                LimitPad {
                    pan: (&mut lim.pan_min, &mut lim.pan_max),
                    tilt: (&mut lim.tilt_min, &mut lim.tilt_max),
                    size: egui::vec2(side, side),
                    enabled: has_pan || has_tilt,
                }
                .show(ui);
            });
            ui.vertical(|ui| {
                ui.add_space(theme::SP_L);
                ui.add_enabled_ui(has_pan || has_tilt, |ui| {
                    egui::Grid::new("fs_ranges")
                        .num_columns(3)
                        .spacing([6.0, 4.0])
                        .show(ui, |ui| {
                            range_row(ui, t("Pan"), &mut lim.pan_min, &mut lim.pan_max, has_pan);
                            range_row(
                                ui,
                                t("Tilt"),
                                &mut lim.tilt_min,
                                &mut lim.tilt_max,
                                has_tilt,
                            );
                        });
                    ui.add_space(theme::SP_S);
                    caption(ui, t("Invertieren"));
                    ui.add(Toggle::new(&mut lim.invert_pan, t("Pan")).enabled(has_pan));
                    ui.add(Toggle::new(&mut lim.invert_tilt, t("Tilt")).enabled(has_tilt));
                    ui.add(
                        Toggle::new(&mut lim.swap_pan_tilt, t("Tauschen"))
                            .enabled(has_pan && has_tilt),
                    );
                });
                ui.add_space(theme::SP_S);
                if ui
                    .add_enabled(!lim.is_default(), egui::Button::new(t("Zurücksetzen")))
                    .on_hover_text(t("Alle Grenzen zurücksetzen"))
                    .clicked()
                {
                    lim = FixtureLimits::default();
                }
            });
        });
        let limits_changed = lim != before;
        for id in ids {
            if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                if limits_changed {
                    f.limits = lim;
                }
            }
        }
    }
}

fn caption(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text.to_uppercase())
            .font(theme::caption())
            .color(theme::text_dim())
            .strong(),
    );
}

/// Eine Zeile „Pan  von … bis …“ in Prozent.
fn range_row(ui: &mut egui::Ui, label: &str, min: &mut u8, max: &mut u8, enabled: bool) {
    ui.label(egui::RichText::new(label).color(if enabled {
        theme::text_dim()
    } else {
        theme::text_faint()
    }));
    let mut lo = f32::from(*min) / 255.0 * 100.0;
    let mut hi = f32::from(*max) / 255.0 * 100.0;
    let a = ui.add_enabled(
        enabled,
        egui::DragValue::new(&mut lo)
            .range(0.0..=100.0)
            .speed(0.5)
            .fixed_decimals(0)
            .suffix("%"),
    );
    let b = ui.add_enabled(
        enabled,
        egui::DragValue::new(&mut hi)
            .range(0.0..=100.0)
            .speed(0.5)
            .fixed_decimals(0)
            .suffix("%"),
    );
    if a.changed() || b.changed() {
        *min = (lo.min(hi) / 100.0 * 255.0).round() as u8;
        *max = (hi.max(lo) / 100.0 * 255.0).round() as u8;
    }
    ui.end_row();
}
