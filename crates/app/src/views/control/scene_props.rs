//! STEUERN → Seitenleiste rechts neben den Szenen: Einstellungen der
//! ausgewählten Szene.
//!
//! Ganz rechts eine schmale Leiste mit Reiter-Symbolen, links daneben der
//! Inhalt. Oben steht in jedem Reiter Name und Farbe der Szene mit dem
//! Play/Pause-Knopf.

use crate::i18n::t;

use dmxcraft_core::{Engine, SceneId};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, Vec2};

use crate::ui::theme;
use crate::ui::widgets::knob::Knob;
use crate::ui::widgets::time_field::time_field;
use crate::ui::widgets::{form, Icon, IconButton};
use crate::views::{View, ViewCtx};

/// Breite der Reiterleiste.
const TABS_W: f32 = 40.0;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Properties,
    Second,
    Third,
}

#[derive(Default)]
pub struct ScenePropsView {
    tab: Tab,
}

impl View for ScenePropsView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let full = ui.available_rect_before_wrap();
        let tabs = Rect::from_min_max(pos2(full.right() - TABS_W, full.top()), full.max);
        let body = Rect::from_min_max(full.min, pos2(tabs.left() - theme::SP_S, full.bottom()));
        ui.allocate_rect(full, Sense::hover());

        // Reiterleiste
        let p = ui.painter();
        p.line_segment(
            [tabs.left_top(), tabs.left_bottom()],
            Stroke::new(1.0, theme::border()),
        );
        let mut tab_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(tabs.shrink2(Vec2::new(3.0, 0.0)))
                .layout(egui::Layout::top_down(egui::Align::Center)),
        );
        for (tab, icon, tip) in [
            (Tab::Properties, Icon::Settings, t("Eigenschaften")),
            (Tab::Second, Icon::Levels, t("Folgt später")),
            (Tab::Third, Icon::Builder, t("Folgt später")),
        ] {
            if tab_ui
                .add(
                    IconButton::new(icon)
                        .size(32.0)
                        .flat()
                        .active(self.tab == tab),
                )
                .on_hover_text(tip)
                .clicked()
            {
                self.tab = tab;
            }
        }

        let mut ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(body)
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let ui = &mut ui;
        let Some(id) = single_scene(cx) else {
            ui.label(
                egui::RichText::new(t("Eine Szene auswählen, um ihre Einstellungen zu sehen."))
                    .color(theme::text_faint()),
            );
            return;
        };
        header(ui, cx.engine, id);
        ui.add_space(theme::SP_S);
        let locked = cx.state.locked;
        egui::ScrollArea::vertical()
            .id_salt("scene_props")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.tab {
                Tab::Properties => properties(ui, cx.engine, id, locked),
                Tab::Second | Tab::Third => {
                    ui.label(egui::RichText::new(t("Folgt später")).color(theme::text_faint()));
                }
            });
    }
}

/// Genau eine markierte Szene.
fn single_scene(cx: &ViewCtx<'_>) -> Option<SceneId> {
    let sel = &cx.state.selected_scenes;
    (sel.len() == 1)
        .then(|| sel.iter().next().copied())
        .flatten()
        .filter(|id| cx.engine.show().scene(*id).is_some())
}

/// Name und Farbe der Szene, Play/Pause rechts, darunter eine Trennlinie.
fn header(ui: &mut egui::Ui, e: &mut Engine, id: SceneId) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let name = scene.name.clone();
    let active = e.is_active(id);
    let paused = e.progress(id).is_some_and(|p| p.paused);
    ui.horizontal(|ui| {
        ui.set_min_height(34.0);
        let (sw, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover());
        ui.painter().rect_filled(sw, theme::R_S, color);
        ui.label(egui::RichText::new(&name).color(color).strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let icon = if active && !paused {
                Icon::Pause
            } else {
                Icon::Play
            };
            let tip = if active && !paused {
                t("Pause")
            } else {
                t("Abspielen")
            };
            if ui
                .add(IconButton::new(icon).size(30.0).active(active))
                .on_hover_text(tip)
                .clicked()
            {
                if active {
                    e.toggle_pause(id);
                } else {
                    e.activate(id);
                }
            }
        });
    });
    let (line, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(line, 0.0, theme::border());
}

fn properties(ui: &mut egui::Ui, e: &mut Engine, id: SceneId, locked: bool) {
    let Some(mut st) = e.show().scene(id).map(|s| s.settings.clone()) else {
        return;
    };
    let mut duration = e.show().scene(id).map_or(0, |s| s.duration_ms()) as u32;
    let duration_before = duration;
    let on = !locked;

    // Regler
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let mut dim = f32::from(st.dimmer) / 255.0;
        let dim_text = format!("{:.0} %", dim * 100.0);
        if Knob::new(&mut dim, t("Dimmer"), dim_text)
            .enabled(on)
            .show(ui)
        {
            st.dimmer = (dim * 255.0).round() as u8;
        }
        // Speed logarithmisch: 10 % … 100 % (Mitte) … 1000 %
        let mut sp = ((f32::from(st.speed_pct) / 10.0).log10() / 2.0).clamp(0.0, 1.0);
        if Knob::new(&mut sp, t("Speed"), format!("{} %", st.speed_pct))
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.speed_pct = (10.0 * 100f32.powf(sp)).round() as u16;
        }
        let mut ph = f32::from(st.phase_deg) / 360.0;
        if Knob::new(&mut ph, t("Phase"), format!("{}°", st.phase_deg))
            .default_value(0.0)
            .enabled(on)
            .show(ui)
        {
            st.phase_deg = (ph * 360.0).round() as u16;
        }
        let mut sz = f32::from(st.size_pct) / 200.0;
        if Knob::new(&mut sz, t("Size"), format!("{} %", st.size_pct))
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.size_pct = (sz * 200.0).round() as u8;
        }
    });
    ui.add_space(theme::SP_S);

    // Position in der Szene, zum Anfang / Ende springen
    let pos = e.scene_position(id);
    ui.horizontal(|ui| {
        if ui
            .add(IconButton::new(Icon::Prev).enabled(pos.is_some()))
            .on_hover_text(t("Zum Anfang"))
            .clicked()
        {
            e.seek(id, 0.0);
        }
        if ui
            .add(IconButton::new(Icon::Next).enabled(pos.is_some()))
            .on_hover_text(t("Zum Ende"))
            .clicked()
        {
            e.seek(id, 1.0);
        }
        let mut v = pos.unwrap_or(0.0);
        if bar_slider(ui, "scene_pos", &mut v, pos.is_some(), None) {
            e.seek(id, v);
        }
    });
    if pos.is_some() {
        ui.ctx().request_repaint();
    }
    ui.add_space(theme::SP_S);

    form(ui, "scene_props_form", |f| {
        f.row(t("Dauer"), |ui| {
            time_field(ui, "duration", &mut duration, on)
        });
        f.row(t("Fade in"), |ui| {
            time_field(ui, "fade_in", &mut st.fade_in_ms, on)
        });
        f.row(t("Fade out"), |ui| {
            time_field(ui, "fade_out", &mut st.fade_out_ms, on)
        });
        f.row(t("Fade Speed"), |ui| {
            let mut v = f32::from(st.fade_speed_pct) / 100.0;
            let text = format!("{} %", st.fade_speed_pct);
            if bar_slider(ui, "fade_speed", &mut v, on, Some(&text)) {
                st.fade_speed_pct = (v * 100.0).round() as u8;
            }
        });
        f.row(t("Phase in"), |ui| {
            time_field(ui, "phase_in", &mut st.phase_in_ms, on)
        });
        f.row(t("Phase out"), |ui| {
            time_field(ui, "phase_out", &mut st.phase_out_ms, on)
        });
    });

    // Änderungen übernehmen (nur wenn sich etwas geändert hat → kein „*“)
    if let Some(scene) = e.show().scene(id) {
        let changed = scene.settings != st || duration != duration_before;
        if changed {
            if let Some(scene) = e.show_mut().scene_mut(id) {
                scene.settings = st;
                if duration != duration_before {
                    scene.set_duration_ms(u64::from(duration));
                }
            }
        }
    }
}

/// Waagerechter Balkenregler 0..=1, optional mit Text in der Mitte.
fn bar_slider(ui: &mut egui::Ui, id: &str, v: &mut f32, enabled: bool, text: Option<&str>) -> bool {
    let w = ui.available_width().max(60.0);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::hover());
    let sense = if enabled {
        Sense::click_and_drag()
    } else {
        Sense::hover()
    };
    let resp = ui.interact(rect, ui.make_persistent_id(("bar_slider", id)), sense);
    let mut changed = false;
    if resp.dragged() || resp.clicked() {
        if let Some(p) = resp.interact_pointer_pos() {
            let nv = ((p.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            if nv != *v {
                *v = nv;
                changed = true;
            }
        }
    }
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S, theme::bg());
    let fill = Rect::from_min_size(rect.min, Vec2::new(rect.width() * *v, rect.height()));
    let color: Color32 = if enabled {
        theme::accent_dim()
    } else {
        theme::raised()
    };
    p.rect_filled(fill, theme::R_S, color);
    p.rect_stroke(
        rect,
        theme::R_S,
        Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
    );
    if let Some(text) = text {
        p.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            theme::caption(),
            theme::text(),
        );
    }
    changed
}
