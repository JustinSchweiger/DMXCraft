//! EINRICHTEN → Geräteeinstellungen der Auswahl: Schnelltest (Licht aus/an,
//! Moving Head zentrieren), Darstellung und Ausgangsgrenzen.

use crate::i18n::t;
use dmxcraft_core::{
    ChannelKind, ChannelRef, Engine, FixtureId, FixtureLimits, PlanShape, PLAN_SIZE_RANGE,
};
use eframe::egui;

use crate::ui::theme;
use crate::ui::widgets::{
    section_header, Icon, IconButton, LimitPad, RangeSlider, ShapeButton, Toggle,
};
use crate::views::{View, ViewCtx};

#[derive(Default)]
pub struct FixtureSettings;

/// Schnelltest über den Programmer.
#[derive(Clone, Copy)]
enum Quick {
    Off,
    On,
    Center,
}

impl View for FixtureSettings {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let ids: Vec<FixtureId> = cx.state.selected.iter().copied().collect();
        let title = match ids.len() {
            0 => t("Gerät").to_string(),
            1 => cx
                .engine
                .show()
                .fixture(ids[0])
                .map_or(t("Gerät").into(), |f| f.name.clone()),
            n => tr!("{n} Geräte", n = n),
        };
        let mut quick = None;
        let some = !ids.is_empty();
        section_header(ui, &title, |ui| {
            if ui
                .add(IconButton::new(Icon::Center).enabled(some))
                .on_hover_text(t("Moving Head zentrieren (Pan/Tilt 50 %)"))
                .clicked()
            {
                quick = Some(Quick::Center);
            }
            if ui
                .add(IconButton::new(Icon::LightOn).enabled(some))
                .on_hover_text(t("Licht an (Dimmer und Weiß voll)"))
                .clicked()
            {
                quick = Some(Quick::On);
            }
            if ui
                .add(IconButton::new(Icon::LightOff).enabled(some))
                .on_hover_text(t("Licht aus"))
                .clicked()
            {
                quick = Some(Quick::Off);
            }
        });
        if let Some(q) = quick {
            quick_action(cx.engine, &ids, q);
        }
        let Some(first) = ids.first().copied() else {
            ui.label(
                egui::RichText::new(t("Geräte im Bühnenplan oder im Patch auswählen."))
                    .color(theme::text_dim()),
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
        let has_dimmer =
            has(ChannelKind::Dimmer) || has(ChannelKind::Red) || has(ChannelKind::White);
        let has_pan = has(ChannelKind::Pan);
        let has_tilt = has(ChannelKind::Tilt);
        let mut lim = show.fixture(first).map(|f| f.limits).unwrap_or_default();
        let shape = show.fixture(first).and_then(|f| f.plan_shape);
        let mut size = show.plan_size(first);
        let before = lim;
        let mut new_shape: Option<Option<PlanShape>> = None;
        let mut size_changed = false;

        // Darstellung
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            if ui
                .add(ShapeButton {
                    shape: None,
                    selected: shape.is_none(),
                })
                .clicked()
            {
                new_shape = Some(None);
            }
            for s in PlanShape::ALL {
                if ui
                    .add(ShapeButton {
                        shape: Some(s),
                        selected: shape == Some(s),
                    })
                    .clicked()
                {
                    new_shape = Some(Some(s));
                }
            }
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            ui.add_space(theme::SP_S);
            ui.spacing_mut().slider_width = (ui.available_width() - 60.0).max(60.0);
            size_changed = ui
                .add(
                    egui::Slider::new(&mut size, PLAN_SIZE_RANGE)
                        .fixed_decimals(1)
                        .suffix("x"),
                )
                .on_hover_text(t("Größe im Bühnenplan"))
                .changed();
        });
        ui.add_space(theme::SP_XS);
        ui.separator();

        // Grenzen: Dimmer | Bewegung (XY) | Werte & Schalter
        let h = ui.available_height().clamp(150.0, 320.0);
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                caption(ui, t("Dimmer"));
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
                caption(ui, t("Bewegung"));
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
        if ids.len() > 1 {
            ui.label(
                egui::RichText::new(t("Änderungen gelten für alle ausgewählten Geräte."))
                    .color(theme::text_faint()),
            );
        }

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

/// Setzt Programmer-Werte für einen schnellen Test der Auswahl.
fn quick_action(e: &mut Engine, ids: &[FixtureId], q: Quick) {
    for id in ids {
        let Some(ty) = e
            .show()
            .fixture(*id)
            .and_then(|f| e.show().type_of(f))
            .cloned()
        else {
            continue;
        };
        let has_white = ty.has_kind(ChannelKind::White);
        for (i, def) in ty.channels.iter().enumerate() {
            let ch = ChannelRef {
                fixture: *id,
                channel: i as u16,
            };
            let value = match (q, def.kind) {
                (Quick::Off, ChannelKind::Dimmer) => Some(0),
                (Quick::Off, k) if k.is_color_intensity() => Some(0),
                (Quick::On, ChannelKind::Dimmer) => Some(255),
                (Quick::On, ChannelKind::White) => Some(255),
                (Quick::On, ChannelKind::Red | ChannelKind::Green | ChannelKind::Blue)
                    if !has_white =>
                {
                    Some(255)
                }
                (Quick::Center, ChannelKind::Pan | ChannelKind::Tilt) => Some(128),
                (Quick::Center, ChannelKind::PanFine | ChannelKind::TiltFine) => Some(0),
                _ => None,
            };
            if let Some(v) = value {
                e.set_programmer(ch, v);
            }
        }
    }
}
