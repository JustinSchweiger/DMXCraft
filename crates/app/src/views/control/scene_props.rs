//! STEUERN → Seitenleiste rechts neben den Szenen: Einstellungen der
//! ausgewählten Szene.
//!
//! Ganz rechts eine schmale Leiste mit Reiter-Symbolen, links daneben der
//! Inhalt. Oben steht in jedem Reiter Name und Farbe der Szene mit dem
//! Play/Pause-Knopf.

use crate::i18n::t;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use dmxcraft_core::{BankScope, Effect, EffectKind, Engine, LoopMode, SceneId, ZoneId};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, Vec2};

use crate::ui::theme;
use crate::ui::widgets::knob::Knob;
use crate::ui::widgets::time_field::time_field;
use crate::ui::widgets::{Dialog, Icon, IconButton, Toggle};
use crate::views::{View, ViewCtx};

/// Breite der Reiterleiste.
const TABS_W: f32 = 40.0;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    Properties,
    Effects,
    Third,
}

#[derive(Default)]
pub struct ScenePropsView {
    tab: Tab,
    /// Ausgewählter Effekt (Szene, Effekt-ID).
    selected_fx: Option<(SceneId, u32)>,
    /// Kachelauswahl zum Hinzufügen eines Effekts offen.
    adding: bool,
    /// Laufender Import/Export-Dialog.
    fx_dialog: Option<FxDialog>,
    /// Auswahlfenster für Szenen/Bänke.
    picker: Option<Picker>,
}

/// Import/Export von Effekten über den Systemdialog (eigener Thread).
struct FxDialog {
    export: bool,
    scene: SceneId,
    rx: Receiver<Option<PathBuf>>,
}

/// Dateiendung exportierter Effekte (Inhalt JSON).
const FX_EXT: &str = "dmxfx";

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
            (Tab::Properties, Icon::Sliders, t("Eigenschaften")),
            (Tab::Effects, Icon::Fx, t("Effekte")),
            (Tab::Third, Icon::Gear, t("Erweiterte Eigenschaften")),
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
            return;
        };
        header(ui, cx.engine, id);
        ui.add_space(theme::SP_S);
        let locked = cx.state.locked;
        picker_window(ui.ctx(), cx.engine, &mut self.picker);
        self.poll_dialog(cx);
        if self.tab == Tab::Effects {
            self.effects_tab(ui, cx, id, locked);
            return;
        }
        egui::ScrollArea::vertical()
            .id_salt("scene_props")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.tab {
                Tab::Properties => properties(ui, cx.engine, id, locked),
                Tab::Third => advanced(ui, cx.engine, id, locked, &mut self.picker),
                Tab::Effects => {}
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
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let mut st = scene.settings.clone();
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let mut duration = scene.duration_ms() as u32;
    let duration_before = duration;
    let on = !locked;

    // Regler in der Szenenfarbe
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let mut dim = f32::from(st.dimmer) / 255.0;
        let dim_text = format!("{:.0} %", dim * 100.0);
        if Knob::new(&mut dim, t("Dimmer"), dim_text)
            .color(color)
            .enabled(on)
            .show(ui)
        {
            st.dimmer = (dim * 255.0).round() as u8;
        }
        // Speed logarithmisch: 10 % … 100 % (Mitte) … 1000 %
        let mut sp = ((f32::from(st.speed_pct) / 10.0).log10() / 2.0).clamp(0.0, 1.0);
        if Knob::new(&mut sp, t("Speed"), format!("{} %", st.speed_pct))
            .color(color)
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.speed_pct = (10.0 * 100f32.powf(sp)).round() as u16;
        }
        let mut ph = f32::from(st.phase_deg) / 360.0;
        if Knob::new(&mut ph, t("Phase"), format!("{}°", st.phase_deg))
            .color(color)
            .default_value(0.0)
            .enabled(on)
            .show(ui)
        {
            st.phase_deg = (ph * 360.0).round() as u16;
        }
        let mut sz = f32::from(st.size_pct) / 200.0;
        if Knob::new(&mut sz, t("Size"), format!("{} %", st.size_pct))
            .color(color)
            .default_value(0.5)
            .enabled(on)
            .show(ui)
        {
            st.size_pct = (sz * 200.0).round() as u8;
        }
    });
    ui.add_space(theme::SP_S);

    // Zeitleiste: Anfang/Ende, Richtung, Position mit verstrichener/Gesamtzeit
    let pos = e.scene_position(id);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
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
        ui.add_space(theme::SP_XS);
        for (rev, icon, tip) in [
            (true, Icon::PlayBack, t("Rückwärts abspielen")),
            (false, Icon::Play, t("Vorwärts abspielen")),
        ] {
            if ui
                .add(
                    IconButton::new(icon)
                        .active(st.reverse == rev)
                        .tint(color)
                        .enabled(on),
                )
                .on_hover_text(tip)
                .clicked()
            {
                st.reverse = rev;
            }
        }
        ui.add_space(theme::SP_XS);
        let mut v = pos.unwrap_or(0.0);
        if bar_slider(ui, "scene_pos", &mut v, pos.is_some(), None, color) {
            e.seek(id, v);
        }
    });
    let total = u64::from(duration);
    let elapsed = (f64::from(pos.unwrap_or(0.0)) * total as f64) as u64;
    // links die verstrichene Zeit (Szenenfarbe), rechts die Gesamtdauer
    let hours = total >= 3_600_000;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(time_text(elapsed, hours))
                .font(egui::FontId::proportional(theme::FS_BODY + 1.0))
                .strong()
                .color(color),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(time_text(total, hours))
                    .font(egui::FontId::proportional(theme::FS_BODY + 1.0))
                    .color(theme::text()),
            );
        });
    });
    if pos.is_some() {
        ui.ctx().request_repaint();
    }
    ui.add_space(theme::SP_S);

    // Felder rechtsbündig, Bezeichnung links
    let row = |ui: &mut egui::Ui, label: &str, add: &mut dyn FnMut(&mut egui::Ui)| {
        ui.horizontal(|ui| {
            ui.set_min_height(32.0);
            ui.label(egui::RichText::new(label).color(theme::text_dim()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                add(ui)
            });
        });
    };
    row(ui, t("Dauer"), &mut |ui| {
        time_field(ui, "duration", &mut duration, on, true, color);
    });
    row(ui, t("Fade in"), &mut |ui| {
        time_field(ui, "fade_in", &mut st.fade_in_ms, on, false, color);
    });
    row(ui, t("Fade out"), &mut |ui| {
        time_field(ui, "fade_out", &mut st.fade_out_ms, on, false, color);
    });
    row(ui, t("Fade Speed"), &mut |ui| {
        // 0.1 … 4.0 in Zehnteln, Standard 1.0
        ui.set_max_width(150.0);
        let mut v = f32::from(st.fade_speed.clamp(1, 40) - 1) / 39.0;
        let text = format!("{:.1}", f32::from(st.fade_speed) / 10.0);
        if bar_slider(ui, "fade_speed", &mut v, on, Some(&text), color) {
            st.fade_speed = (v * 39.0).round() as u8 + 1;
        }
    });
    row(ui, t("Phase in"), &mut |ui| {
        time_field(ui, "phase_in", &mut st.phase_in_ms, on, false, color);
    });
    row(ui, t("Phase out"), &mut |ui| {
        time_field(ui, "phase_out", &mut st.phase_out_ms, on, false, color);
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

/// Zeit als Text wie im Zeitfeld.
fn time_text(ms: u64, hours: bool) -> String {
    crate::ui::widgets::time_field::format(ms, hours)
}

/// Waagerechter Balkenregler 0..=1, optional mit Text in der Mitte.
fn bar_slider(
    ui: &mut egui::Ui,
    id: &str,
    v: &mut f32,
    enabled: bool,
    text: Option<&str>,
    accent: Color32,
) -> bool {
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
        accent.gamma_multiply(0.45)
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

impl ScenePropsView {
    /// Reiter „Effekte“: bei STATIC-Szenen die Kacheln zur Auswahl, bei
    /// FX-Szenen die Liste der Effekte und unten die Werkzeugleiste.
    fn effects_tab(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: SceneId, locked: bool) {
        let Some(scene) = cx.engine.show().scene(id) else {
            return;
        };
        let effects = scene.effects.clone();
        if effects.is_empty() || self.adding {
            let mut pick = None;
            egui::ScrollArea::vertical()
                .id_salt("fx_tiles")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    if self.adding && !effects.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label(theme::heading_text(t("Effekt hinzufügen")));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add(IconButton::new(Icon::Close).flat())
                                        .on_hover_text(t("Abbrechen"))
                                        .clicked()
                                    {
                                        self.adding = false;
                                    }
                                },
                            );
                        });
                        ui.add_space(theme::SP_S);
                    }
                    pick = effect_tiles(ui, !locked);
                });
            if let Some(kind) = pick {
                if let Some(scene) = cx.engine.show_mut().scene_mut(id) {
                    let fx = scene.add_effect(kind);
                    self.selected_fx = Some((id, fx));
                }
                self.adding = false;
            }
            return;
        }

        // Liste der Effekte, darunter die Werkzeugleiste
        const BAR_H: f32 = 40.0;
        let list_h = (ui.available_height() - BAR_H).max(60.0);
        let selected = self.selected_fx.filter(|(s, _)| *s == id).map(|(_, f)| f);
        let mut toggle = None;
        let mut select = None;
        ui.allocate_ui(Vec2::new(ui.available_width(), list_h), |ui| {
            egui::ScrollArea::vertical()
                .id_salt("fx_list")
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for fx in &effects {
                        let (head, chevron) = effect_header(ui, fx, selected == Some(fx.id));
                        if chevron.clicked() {
                            toggle = Some(fx.id);
                        }
                        if head.clicked() {
                            select = Some(fx.id);
                        }
                        if !fx.collapsed {
                            egui::Frame::new()
                                .fill(theme::raised())
                                .inner_margin(egui::Margin::same(8))
                                .show(ui, |ui| {
                                    ui.set_min_width(ui.available_width());
                                    ui.label(
                                        egui::RichText::new(t("Einstellungen folgen"))
                                            .color(theme::text_faint()),
                                    );
                                });
                        }
                        ui.add_space(2.0);
                    }
                });
        });
        if let Some(f) = select {
            self.selected_fx = Some((id, f));
            cx.state.focus = crate::state::Focus::Effect(id, f);
        }
        // Entf löscht den angeklickten Effekt (nicht die Szene)
        let typing = ui.ctx().memory(|m| m.focused().is_some());
        let mut delete = false;
        if let (false, false, crate::state::Focus::Effect(s, f)) = (typing, locked, cx.state.focus)
        {
            if s == id
                && selected == Some(f)
                && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
            {
                delete = true;
            }
        }
        if let (Some(f), false) = (toggle, locked) {
            if let Some(fx) = cx
                .engine
                .show_mut()
                .scene_mut(id)
                .and_then(|s| s.effects.iter_mut().find(|e| e.id == f))
            {
                fx.collapsed = !fx.collapsed;
            }
        }

        // Werkzeugleiste
        let (line, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), Sense::hover());
        ui.painter().rect_filled(line, 0.0, theme::border());
        ui.horizontal(|ui| {
            ui.set_min_height(BAR_H - 1.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let on = !locked;
                if ui
                    .add(
                        IconButton::new(Icon::Save)
                            .flat()
                            .enabled(on && self.fx_dialog.is_none()),
                    )
                    .on_hover_text(t("Effekte exportieren"))
                    .clicked()
                {
                    self.start_dialog(ui.ctx(), id, true);
                }
                if ui
                    .add(
                        IconButton::new(Icon::Import)
                            .flat()
                            .enabled(on && self.fx_dialog.is_none()),
                    )
                    .on_hover_text(t("Effekte importieren"))
                    .clicked()
                {
                    self.start_dialog(ui.ctx(), id, false);
                }
                let clicked_delete = ui
                    .add(
                        IconButton::new(Icon::Trash)
                            .size(30.0)
                            .flat()
                            .enabled(on && selected.is_some()),
                    )
                    .on_hover_text(t("Ausgewählten Effekt löschen"))
                    .clicked();
                if clicked_delete || delete {
                    if let (Some(f), Some(scene)) = (selected, cx.engine.show_mut().scene_mut(id)) {
                        // keine Effekte mehr -> wieder STATIC
                        scene.effects.retain(|e| e.id != f);
                        self.selected_fx = scene.effects.last().map(|e| (id, e.id));
                        cx.state.focus = crate::state::Focus::Scenes;
                    }
                }
                if ui
                    .add(IconButton::new(Icon::Plus).size(30.0).flat().enabled(on))
                    .on_hover_text(t("Effekt hinzufügen"))
                    .clicked()
                {
                    self.adding = true;
                }
            });
        });
    }

    fn start_dialog(&mut self, ctx: &egui::Context, scene: SceneId, export: bool) {
        let (tx, rx) = channel();
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let dialog = rfd::FileDialog::new().add_filter(t("DMXCraft-Effekte"), &[FX_EXT]);
            let result = if export {
                dialog
                    .set_title(t("Effekte exportieren"))
                    .set_file_name(format!("Effekte.{FX_EXT}"))
                    .save_file()
                    .map(|p| p.with_extension(FX_EXT))
            } else {
                dialog.set_title(t("Effekte importieren")).pick_file()
            };
            let _ = tx.send(result);
            ctx.request_repaint();
        });
        self.fx_dialog = Some(FxDialog { export, scene, rx });
    }

    /// Ergebnis eines Import/Export-Dialogs verarbeiten.
    fn poll_dialog(&mut self, cx: &mut ViewCtx<'_>) {
        let Some(d) = &self.fx_dialog else {
            return;
        };
        let Ok(result) = d.rx.try_recv() else {
            return;
        };
        let (export, scene) = (d.export, d.scene);
        self.fx_dialog = None;
        let Some(path) = result else {
            return;
        };
        if export {
            // ausgewählter Effekt, sonst alle Effekte der Szene
            let Some(s) = cx.engine.show().scene(scene) else {
                return;
            };
            let sel = self
                .selected_fx
                .filter(|(sc, _)| *sc == scene)
                .map(|(_, f)| f);
            let list: Vec<&Effect> = match sel {
                Some(f) => s.effects.iter().filter(|e| e.id == f).collect(),
                None => s.effects.iter().collect(),
            };
            let result = serde_json::to_string_pretty(&list)
                .map_err(|e| e.to_string())
                .and_then(|json| std::fs::write(&path, json).map_err(|e| e.to_string()));
            match result {
                Ok(()) => cx
                    .state
                    .info(tr!("Effekte exportiert: {path}", path = path.display())),
                Err(e) => cx.state.error(e),
            }
        } else {
            let read = std::fs::read_to_string(&path)
                .map_err(|e| e.to_string())
                .and_then(|s| serde_json::from_str::<Vec<Effect>>(&s).map_err(|e| e.to_string()));
            match read {
                Ok(list) => {
                    if let Some(s) = cx.engine.show_mut().scene_mut(scene) {
                        let n = list.len();
                        for fx in list {
                            let id = s.add_effect(fx.kind);
                            if let Some(new) = s.effects.iter_mut().find(|e| e.id == id) {
                                new.collapsed = fx.collapsed;
                            }
                        }
                        cx.state.info(tr!("{n} Effekt(e) importiert", n = n));
                    }
                }
                Err(e) => cx.state.error(e),
            }
        }
    }
}

/// Kacheln der Effektarten in zwei Spalten; gibt die angeklickte zurück.
fn effect_tiles(ui: &mut egui::Ui, enabled: bool) -> Option<EffectKind> {
    let gap = theme::SP_S;
    let w = ((ui.available_width() - gap) / 2.0).min(120.0);
    let h = w * 0.72;
    let mut pick = None;
    for row in EffectKind::ALL.chunks(2) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            let total = w * row.len() as f32 + gap * (row.len() as f32 - 1.0);
            ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
            for kind in row {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());
                let hot = enabled && resp.hovered();
                let p = ui.painter();
                p.rect_filled(
                    rect,
                    theme::R_S,
                    if hot {
                        theme::raised_hover()
                    } else {
                        theme::bg()
                    },
                );
                p.rect_stroke(
                    rect,
                    theme::R_S,
                    Stroke::new(
                        if hot { 2.0 } else { 1.5 },
                        if hot {
                            theme::accent()
                        } else {
                            theme::border()
                        },
                    ),
                    egui::StrokeKind::Inside,
                );
                let galley = p.layout(
                    kind.label().to_string(),
                    theme::heading_font(),
                    if enabled {
                        theme::text()
                    } else {
                        theme::text_faint()
                    },
                    w - 12.0,
                );
                let pos = rect.center() - galley.size() * 0.5;
                p.galley(pos, galley, theme::text());
                if enabled && resp.clicked() {
                    pick = Some(*kind);
                }
            }
        });
        ui.add_space(gap);
    }
    pick
}

/// Kopfzeile eines Effekts: Zuklapp-Pfeil, Name; Antwort für Auswahl und Pfeil.
fn effect_header(
    ui: &mut egui::Ui,
    fx: &Effect,
    selected: bool,
) -> (egui::Response, egui::Response) {
    let (rect, head) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::click());
    let arrow_rect = Rect::from_min_size(rect.min, Vec2::new(28.0, rect.height()));
    let chevron = ui.interact(
        arrow_rect,
        ui.make_persistent_id(("fx_chevron", fx.id)),
        Sense::click(),
    );
    let p = ui.painter();
    let fill = if selected {
        theme::accent_dim()
    } else if head.hovered() {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S, fill);
    let icon = if fx.collapsed {
        Icon::ChevronRight
    } else {
        Icon::ChevronDown
    };
    icon.paint(
        p,
        Rect::from_center_size(arrow_rect.center(), Vec2::splat(12.0)),
        theme::text_dim(),
    );
    p.text(
        pos2(arrow_rect.right(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        fx.kind.label(),
        theme::heading_font(),
        if selected {
            theme::text()
        } else {
            theme::text_dim()
        },
    );
    (head, chevron)
}

/// Wofür das Auswahlfenster geöffnet ist.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PickTarget {
    /// „Springe zu“: Szenen.
    Jump,
    /// Freigabe-Modus „Bänke“.
    Release,
    /// Freigabeschutz „Bänke“.
    Protect,
}

/// Offenes Auswahlfenster (Szenen bzw. Bänke, Mehrfachauswahl).
pub(super) struct Picker {
    scene: SceneId,
    target: PickTarget,
    chosen: std::collections::BTreeSet<u32>,
}

/// Reiter „Erweiterte Eigenschaften“: Priorität, Wiederholung, Ende,
/// Freigabe, Freigabeschutz und Flash.
fn advanced(
    ui: &mut egui::Ui,
    e: &mut Engine,
    id: SceneId,
    locked: bool,
    picker: &mut Option<Picker>,
) {
    let Some(scene) = e.show().scene(id) else {
        return;
    };
    let mut st = scene.settings.clone();
    let mut loop_mode = scene.loop_mode;
    let color = scene
        .color
        .map_or_else(|| theme::zone_color(e.show(), scene.zone), theme::rgb);
    let names: std::collections::HashMap<u32, String> = e
        .show()
        .scenes
        .iter()
        .map(|s| (s.id.0, s.name.clone()))
        .collect();
    let on = !locked;
    let once = loop_mode == LoopMode::Once;
    let mut open = None;

    ui.add_enabled_ui(on, |ui| {
        row(ui, t("Szenen-Priorität"), |ui| {
            ui.add(egui::DragValue::new(&mut st.priority).range(-10..=10))
                .on_hover_text(t("Höhere Priorität gewinnt bei Überschneidungen"));
        });
        row(ui, t("Wiederholung"), |ui| {
            if once {
                ui.add(
                    egui::DragValue::new(&mut st.repeat_count)
                        .range(1..=999)
                        .suffix("×"),
                );
            }
            for (mode, icon, tip) in [
                (LoopMode::Once, Icon::RepeatOnce, t("X-mal abspielen")),
                (LoopMode::Loop, Icon::Repeat, t("Endlosschleife")),
            ] {
                if ui
                    .add(IconButton::new(icon).active(loop_mode == mode).tint(color))
                    .on_hover_text(tip)
                    .clicked()
                {
                    loop_mode = mode;
                }
            }
        });
        ui.add_enabled_ui(once, |ui| {
            row(ui, t("Springe zu"), |ui| {
                let text = match st.jump_to_scenes.as_slice() {
                    [] => t("Keine").to_string(),
                    [one] => names.get(&one.0).cloned().unwrap_or_default(),
                    many => tr!("{n} Szenen", n = many.len()),
                };
                if ui.button(text).clicked() {
                    open = Some(PickTarget::Jump);
                }
            });
            row(ui, t("Am Ende freigeben"), |ui| {
                ui.add(Toggle::new(&mut st.release_at_end, ""));
            });
        });
        ui.add_space(theme::SP_S);
        if scope_row(
            ui,
            t("Freigabe-Modus"),
            ("release_mode", id.0),
            &mut st.release_mode,
        ) {
            open = Some(PickTarget::Release);
        }
        if scope_row(
            ui,
            t("Vor Freigabe schützen"),
            ("protect", id.0),
            &mut st.protect,
        ) {
            open = Some(PickTarget::Protect);
        }
        ui.add_space(theme::SP_S);
        row(ui, t("Flash-Modus"), |ui| {
            ui.add(Toggle::new(&mut st.flash, ""))
                .on_hover_text(t("Läuft nur, solange die Szene gedrückt wird"));
        });
    });

    if let Some(target) = open {
        let chosen = match target {
            PickTarget::Jump => st.jump_to_scenes.iter().map(|s| s.0).collect(),
            PickTarget::Release => banks_of(&st.release_mode),
            PickTarget::Protect => banks_of(&st.protect),
        };
        *picker = Some(Picker {
            scene: id,
            target,
            chosen,
        });
    }

    if let Some(scene) = e.show().scene(id) {
        if scene.settings != st || scene.loop_mode != loop_mode {
            if let Some(scene) = e.show_mut().scene_mut(id) {
                scene.settings = st;
                scene.loop_mode = loop_mode;
            }
        }
    }
}

fn banks_of(scope: &BankScope) -> std::collections::BTreeSet<u32> {
    match scope {
        BankScope::Banks(list) => list.iter().map(|z| z.0).collect(),
        _ => Default::default(),
    }
}

/// Eine Bank im Auswahlfenster: ID, Name, Farbe und ihre Szenen.
type PickColumn = (ZoneId, String, Color32, Vec<(SceneId, String, Color32)>);

/// Auswahlfenster: alle Bänke als Spalten mit ihren Szenen. Je nach Zweck
/// werden Szenen oder Bänke (Kopf) ausgewählt, mehrere möglich.
fn picker_window(ctx: &egui::Context, e: &mut Engine, picker: &mut Option<Picker>) {
    let Some(p) = picker.as_mut() else {
        return;
    };
    let pick_banks = p.target != PickTarget::Jump;
    let title = if pick_banks {
        t("Bänke auswählen")
    } else {
        t("Szenen auswählen")
    };
    let show = e.show();
    let columns: Vec<PickColumn> = show
        .zones
        .iter()
        .map(|z| {
            let zc = theme::zone_color(show, Some(z.id));
            let scenes = show
                .scenes
                .iter()
                .filter(|s| s.zone == Some(z.id) && s.id != p.scene)
                .map(|s| (s.id, s.name.clone(), s.color.map_or(zc, theme::rgb)))
                .collect();
            (z.id, z.name.clone(), zc, scenes)
        })
        .collect();
    let mut open = true;
    let res = Dialog::new(title)
        .id("scene_bank_picker")
        .resizable([760.0, 460.0])
        .cancel(t("Abbrechen"))
        .primary(t("Übernehmen"))
        .show(ctx, &mut open, |ui, _| {
            ui.label(
                egui::RichText::new(if pick_banks {
                    t("Bänke anklicken, um sie aus- oder abzuwählen.")
                } else {
                    t("Szenen anklicken, um sie aus- oder abzuwählen.")
                })
                .color(theme::text_dim()),
            );
            ui.add_space(theme::SP_S);
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        for (zid, zname, zc, scenes) in &columns {
                            ui.vertical(|ui| {
                                ui.set_width(150.0);
                                let on = pick_banks && p.chosen.contains(&zid.0);
                                if pick_tile(ui, zname, *zc, on, pick_banks, true) {
                                    toggle(&mut p.chosen, zid.0);
                                }
                                for (sid, name, c) in scenes {
                                    let on = !pick_banks && p.chosen.contains(&sid.0);
                                    if pick_tile(ui, name, *c, on, !pick_banks, false) {
                                        toggle(&mut p.chosen, sid.0);
                                    }
                                }
                            });
                        }
                    });
                });
        });
    if res.clicked(1) {
        let (scene, target, chosen) = (p.scene, p.target, p.chosen.clone());
        if let Some(s) = e.show_mut().scene_mut(scene) {
            match target {
                PickTarget::Jump => {
                    s.settings.jump_to_scenes = chosen.into_iter().map(SceneId).collect();
                }
                PickTarget::Release => {
                    s.settings.release_mode =
                        BankScope::Banks(chosen.into_iter().map(ZoneId).collect());
                }
                PickTarget::Protect => {
                    s.settings.protect = BankScope::Banks(chosen.into_iter().map(ZoneId).collect());
                }
            }
        }
        open = false;
    }
    if !open {
        *picker = None;
    }
}

fn toggle(set: &mut std::collections::BTreeSet<u32>, v: u32) {
    if !set.remove(&v) {
        set.insert(v);
    }
}

/// Kachel im Auswahlfenster; `true` beim Anklicken.
fn pick_tile(
    ui: &mut egui::Ui,
    name: &str,
    color: Color32,
    on: bool,
    enabled: bool,
    head: bool,
) -> bool {
    let h = if head { 30.0 } else { 34.0 };
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(150.0, h), Sense::click());
    let p = ui.painter();
    let fill = if on {
        color.gamma_multiply(0.25)
    } else if enabled && resp.hovered() {
        theme::raised_hover()
    } else if head {
        theme::bg()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S, fill);
    let stroke = if on {
        Stroke::new(1.5, color)
    } else {
        Stroke::new(1.0, theme::border())
    };
    p.rect_stroke(rect, theme::R_S, stroke, egui::StrokeKind::Inside);
    if head {
        p.line_segment(
            [
                rect.left_top() + Vec2::new(6.0, 2.0),
                rect.right_top() + Vec2::new(-6.0, 2.0),
            ],
            Stroke::new(2.0, color),
        );
    }
    p.text(
        pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        if head {
            theme::heading_font()
        } else {
            theme::body()
        },
        if enabled || head {
            color
        } else {
            color.gamma_multiply(0.5)
        },
    );
    ui.add_space(4.0);
    enabled && resp.clicked()
}

/// Zeile: Bezeichnung links, Bedienelement rechtsbündig.
fn row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(32.0);
        ui.label(egui::RichText::new(label).color(theme::text_dim()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), add);
    });
}

/// Bank-Bezug als kurze Auswahl rechts; bei „Bänke“ darunter ein Knopf für
/// das Auswahlfenster. `true` = Fenster öffnen.
fn scope_row(ui: &mut egui::Ui, label: &str, id: (&str, u32), scope: &mut BankScope) -> bool {
    let names = [t("Aus"), t("Alle"), t("Bank"), t("Außerhalb"), t("Bänke")];
    let index = match scope {
        BankScope::Off => 0,
        BankScope::All => 1,
        BankScope::Bank => 2,
        BankScope::OutsideBank => 3,
        BankScope::Banks(_) => 4,
    };
    let mut picked = index;
    let mut open = false;
    row(ui, label, |ui| {
        egui::ComboBox::from_id_salt(id)
            .selected_text(names[index])
            .width(110.0)
            .show_ui(ui, |ui| {
                for (i, n) in names.iter().enumerate() {
                    ui.selectable_value(&mut picked, i, *n);
                }
            });
    });
    if picked != index {
        *scope = match picked {
            0 => BankScope::Off,
            1 => BankScope::All,
            2 => BankScope::Bank,
            3 => BankScope::OutsideBank,
            _ => {
                open = true;
                BankScope::Banks(Vec::new())
            }
        };
    }
    if let BankScope::Banks(list) = scope {
        row(ui, "", |ui| {
            let text = match list.len() {
                0 => t("Keine Bank").to_string(),
                1 => t("1 Bank").to_string(),
                n => tr!("{n} Bänke", n = n),
            };
            if ui
                .button(text)
                .on_hover_text(t("Bänke auswählen"))
                .clicked()
            {
                open = true;
            }
        });
    }
    open
}
