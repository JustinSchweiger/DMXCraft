//! SETUP → fixture settings of the selection: quick test (light off/on,
//! center moving head), display and output limits.

use crate::i18n::t;
use dmxcraft_core::{ChannelKind, FixtureId, FixtureLimits};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, LimitPad, RangeSlider, Toggle};
use crate::views::quick::{quick_action, Layer, Quick};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct FixtureSettings;

/// Quick test via the programmer.
impl View for FixtureSettings {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        // The content must not force the panel size: if the area is smaller than
        // the content, scroll instead of locking the panel
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
        // Quick test at the top, as in the programmer: light off/on, center
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            for (q, icon, tip) in [
                (Quick::Off, Icon::LightOff, t("Light off")),
                (
                    Quick::On,
                    Icon::LightOn,
                    t("Light on (dimmer and white full)"),
                ),
                (
                    Quick::Center,
                    Icon::Center,
                    t("Center moving head (pan/tilt 50 %)"),
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
        // Mounting from the 3D placement: beam of the base pointing down = hanging
        let mount_before = hanging_of(show, first);
        let mut mount = mount_before;

        // Limits: dimmer | movement (XY) | values & switches
        let h = (ui.available_height() - 4.0).clamp(120.0, 420.0);
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                let cw = caption(ui, t("Dimmer limit")).rect.width();
                // Center the slider below the (wider) heading
                ui.horizontal(|ui| {
                    ui.add_space(((cw - crate::ui::widgets::fader::FADER_W) * 0.5).max(0.0));
                    RangeSlider {
                        min: &mut lim.dimmer_min,
                        max: &mut lim.dimmer_max,
                        height: h - 24.0,
                        enabled: has_dimmer,
                    }
                    .show(ui);
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                caption(ui, t("Movement limit"));
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
                    caption(ui, t("Invert"));
                    ui.add(Toggle::new(&mut lim.invert_pan, t("Pan")).enabled(has_pan));
                    ui.add(Toggle::new(&mut lim.invert_tilt, t("Tilt")).enabled(has_tilt));
                    ui.add(
                        Toggle::new(&mut lim.swap_pan_tilt, t("Swap")).enabled(has_pan && has_tilt),
                    );
                    ui.add_space(theme::SP_S);
                    caption(ui, t("Mounting"));
                    crate::ui::widgets::Segmented::new(&[
                        (true, t("Hanging")),
                        (false, t("Standing")),
                    ])
                    .show(ui, &mut mount);
                });
                ui.add_space(theme::SP_S);
                if ui
                    .add_enabled(!lim.is_default(), egui::Button::new(t("Reset")))
                    .on_hover_text(t("Reset all limits"))
                    .clicked()
                {
                    lim = FixtureLimits::default();
                }
            });
        });
        let limits_changed = lim != before;
        let mount_changed = mount != mount_before;
        for id in ids {
            // turn the 3D placement upside down (keeps position and direction)
            let flip = mount_changed && hanging_of(cx.engine.show(), id) != mount;
            let mut place = cx.engine.show().placement3d(id);
            if flip {
                place.rotation[0] = (place.rotation[0] + 180.0 + 180.0).rem_euclid(360.0) - 180.0;
            }
            if let Some(f) = cx.engine.show_mut().fixture_mut(id) {
                if limits_changed {
                    f.limits = lim;
                }
                if flip {
                    f.place3d = Some(place);
                }
            }
        }
    }
}

/// Is the fixture mounted hanging (base above, beam down at the center)?
fn hanging_of(show: &dmxcraft_core::Show, id: FixtureId) -> bool {
    let p = show.placement3d(id);
    dmxcraft_core::venue::moving_head_direction(p.rotation, 0.5, 0.5, 0.0, 0.0)[1] <= 0.0
}

fn caption(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.label(
        egui::RichText::new(text.to_uppercase())
            .font(theme::caption())
            .color(theme::text_dim())
            .strong(),
    )
}

/// A row “Pan from … to …” in percent.
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
