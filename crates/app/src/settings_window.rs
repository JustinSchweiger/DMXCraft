//! Einstellungsfenster: Reiter links, Inhalt rechts.
//!
//! Das Fenster ändert die [`Settings`] direkt und meldet über
//! [`SettingsChanges`], was die App danach tun muss (speichern, Ausgang
//! neu starten, Zoom setzen).

use dmxcraft_core::{Engine, UNIVERSES};
use dmxcraft_output::UsbDmxPort;
use eframe::egui::{self, pos2, vec2, Align2, Sense, Stroke, Ui};

use crate::settings::Settings;
use crate::ui::theme;
use crate::ui::widgets::dialog::INPUT_H;
use crate::ui::widgets::{form, text_field, Dialog, Icon, IconButton, Toggle};

const SIDEBAR_W: f32 = 170.0;
const TAB_H: f32 = 34.0;
const WIDTH: f32 = 760.0;
const HEIGHT: f32 = 470.0;

/// Stufen der Darstellungsgröße in Prozent.
pub const SCALES: [u32; 9] = [60, 75, 90, 100, 110, 125, 150, 175, 200];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    General,
    Outputs,
    Midi,
    Project,
    Lock,
}

impl Tab {
    const ALL: [Tab; 5] = [
        Tab::General,
        Tab::Outputs,
        Tab::Midi,
        Tab::Project,
        Tab::Lock,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::General => "Allgemein",
            Tab::Outputs => "Ausgänge",
            Tab::Midi => "MIDI",
            Tab::Project => "Projekt",
            Tab::Lock => "Sperre",
        }
    }

    fn icon(self) -> Icon {
        match self {
            Tab::General => Icon::Settings,
            Tab::Outputs => Icon::Levels,
            Tab::Midi => Icon::Midi,
            Tab::Project => Icon::FileOpen,
            Tab::Lock => Icon::Locked,
        }
    }
}

/// Was nach einem Frame des Fensters zu tun ist.
#[derive(Default)]
pub struct SettingsChanges {
    pub save: bool,
    pub restart_output: bool,
    pub zoom: Option<f32>,
}

#[derive(Default)]
pub struct SettingsWindow {
    pub tab: Tab,
}

/// Was das Fenster aus der App braucht.
pub struct SettingsCx<'a> {
    pub settings: &'a mut Settings,
    pub engine: &'a mut Engine,
    pub usb_ports: &'a mut Vec<UsbDmxPort>,
    pub show_path: &'a str,
}

impl SettingsWindow {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        open: &mut bool,
        cx: SettingsCx<'_>,
    ) -> SettingsChanges {
        let mut ch = SettingsChanges::default();
        let tab = &mut self.tab;
        Dialog::new("Einstellungen")
            .id("settings")
            .width(WIDTH)
            .height(HEIGHT)
            .flush()
            .cancel("Schließen")
            .show(ctx, open, |ui, _| {
                // Feste Aufteilung: Reiterleiste links, Inhalt rechts
                let area = egui::Rect::from_min_size(ui.cursor().min, vec2(WIDTH, HEIGHT));
                ui.allocate_rect(area, Sense::hover());
                let side = egui::Rect::from_min_size(area.min, vec2(SIDEBAR_W, HEIGHT));
                let content = egui::Rect::from_min_max(
                    pos2(side.right() + theme::SP_L, area.top() + theme::SP_M),
                    pos2(area.right() - theme::SP_L, area.bottom() - theme::SP_M),
                );
                sidebar(ui, side, tab);
                let mut cui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content)
                        .layout(egui::Layout::top_down(egui::Align::Min)),
                );
                egui::ScrollArea::vertical()
                    .id_salt(("settings_scroll", *tab as u8))
                    .auto_shrink([false, false])
                    .max_height(content.height())
                    .show(&mut cui, |ui| {
                        ui.set_width(content.width());
                        match *tab {
                            Tab::General => general(ui, cx.settings, &mut ch),
                            Tab::Outputs => {
                                outputs(ui, cx.settings, cx.engine, cx.usb_ports, &mut ch)
                            }
                            Tab::Midi => midi(ui),
                            Tab::Project => project(ui, cx.engine, cx.show_path),
                            Tab::Lock => lock(ui, cx.settings, &mut ch),
                        }
                    });
            });
        ch
    }
}

/// Reiterleiste links.
fn sidebar(ui: &mut Ui, rect: egui::Rect, tab: &mut Tab) {
    let p = ui.painter();
    p.rect_filled(rect, 0.0, theme::BG);
    p.vline(
        rect.right() - 0.5,
        rect.y_range(),
        Stroke::new(1.0, theme::BORDER),
    );
    for (i, t) in Tab::ALL.into_iter().enumerate() {
        let r = egui::Rect::from_min_size(
            pos2(rect.left(), rect.top() + theme::SP_S + i as f32 * TAB_H),
            vec2(SIDEBAR_W - 1.0, TAB_H),
        );
        let resp = ui.interact(r, ui.scope_id().with(("settings_tab", i)), Sense::click());
        let active = *tab == t;
        let p = ui.painter();
        if active {
            p.rect_filled(r, 0.0, theme::SURFACE);
            p.rect_filled(
                egui::Rect::from_min_size(r.min, vec2(3.0, r.height())),
                0.0,
                theme::ACCENT,
            );
        } else if resp.hovered() {
            p.rect_filled(r, 0.0, theme::RAISED_HOVER);
        }
        let color = if active { theme::TEXT } else { theme::TEXT_DIM };
        let icon = egui::Rect::from_center_size(
            pos2(r.left() + 24.0, r.center().y),
            egui::Vec2::splat(18.0),
        );
        t.icon().paint(p, icon, color);
        p.text(
            pos2(r.left() + 44.0, r.center().y),
            Align2::LEFT_CENTER,
            t.label(),
            theme::body(),
            color,
        );
        if resp.clicked() {
            *tab = t;
        }
    }
}

fn section(ui: &mut Ui, title: &str) {
    ui.add_space(theme::SP_XS);
    ui.label(theme::heading_text(title));
    ui.add_space(theme::SP_XS);
}

fn hint(ui: &mut Ui, text: &str) {
    ui.label(egui::RichText::new(text).small().color(theme::TEXT_FAINT));
}

fn full_combo(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    text: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui),
) {
    ui.spacing_mut().interact_size.y = INPUT_H;
    egui::ComboBox::from_id_salt(id)
        .selected_text(text)
        .width(ui.available_width())
        .show_ui(ui, add);
}

fn general(ui: &mut Ui, s: &mut Settings, ch: &mut SettingsChanges) {
    section(ui, "Optionen");
    form(ui, "settings_general", |f| {
        f.row("Sprache", |ui| {
            full_combo(ui, "settings_lang", "Deutsch", |ui| {
                ui.selectable_value(&mut s.language, "de".to_string(), "Deutsch");
                ui.add_enabled(false, egui::Button::selectable(false, "English"))
                    .on_disabled_hover_text("Noch nicht verfügbar");
            });
        });
        f.row("Darstellungsgröße", |ui| {
            let current = (s.ui_scale * 100.0).round() as u32;
            let mut picked = current;
            full_combo(ui, "settings_scale", format!("{current} %"), |ui| {
                for pct in SCALES {
                    ui.selectable_value(&mut picked, pct, format!("{pct} %"));
                }
            });
            if picked != current {
                s.ui_scale = picked as f32 / 100.0;
                ch.zoom = Some(s.ui_scale);
                ch.save = true;
            }
        });
        f.row("Instanzname", |ui| {
            let r = ui
                .add(text_field(&mut s.instance_name).desired_width(f32::INFINITY))
                .on_hover_text("Name dieses Rechners, z. B. für Fernsteuerung im Netz");
            if r.lost_focus() {
                ch.save = true;
            }
        });
    });
    ui.add_space(theme::SP_M);
    section(ui, "Startup");
    ch.save |= ui
        .add(Toggle::new(
            &mut s.open_last_show,
            "Letzte Datei automatisch laden",
        ))
        .changed();
    ch.save |= ui
        .add(Toggle::new(&mut s.start_touch, "Im Touch-Modus starten"))
        .on_hover_text("Der Touch-Modus folgt – die Einstellung wird schon gespeichert")
        .changed();
    ch.save |= ui
        .add(Toggle::new(
            &mut s.check_updates,
            "Beim Start nach Updates suchen",
        ))
        .changed();
    ui.add_space(theme::SP_L);
    if let Some(p) = Settings::path() {
        hint(ui, &format!("Gespeichert in {}", p.display()));
    }
}

fn outputs(
    ui: &mut Ui,
    s: &mut Settings,
    e: &Engine,
    ports: &mut Vec<UsbDmxPort>,
    ch: &mut SettingsChanges,
) {
    ui.horizontal(|ui| {
        ui.label(theme::heading_text("Universen")).on_hover_text(
            "Ein USB-DMX-Kabel (Open DMX / FTDI) je Universum. Die Zuordnung hängt an der \
             Seriennummer des Kabels und bleibt beim Umstecken erhalten.",
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(IconButton::new(Icon::Reset).flat())
                .on_hover_text("Geräte neu suchen")
                .clicked()
            {
                *ports = dmxcraft_output::list_ports();
            }
            let n = ports.iter().filter(|p| p.ftdi).count();
            ui.label(
                egui::RichText::new(match n {
                    0 => "kein USB-DMX-Kabel gefunden".to_string(),
                    1 => "1 Kabel gefunden".to_string(),
                    n => format!("{n} Kabel gefunden"),
                })
                .color(theme::TEXT_DIM),
            );
        });
    });
    ui.add_space(theme::SP_XS);

    // Pro Universum: Gerät, Zustand, Anzahl gepatchter Geräte
    let show = e.show();
    egui::Grid::new("settings_universes")
        .num_columns(4)
        .min_col_width(0.0)
        .min_row_height(INPUT_H)
        .spacing(vec2(theme::SP_M, theme::SP_S))
        .show(ui, |ui| {
            for u in 0..usize::from(UNIVERSES) {
                let current = s.open_dmx.get(u).filter(|k| !k.is_empty()).cloned();
                let port = current
                    .as_ref()
                    .and_then(|k| ports.iter().find(|p| &p.key == k));
                // Zustandspunkt
                let (dot, tip) = match (&current, port) {
                    (None, _) => (
                        theme::TEXT_FAINT,
                        "Kein Gerät – wird nicht ausgegeben".to_string(),
                    ),
                    (Some(_), Some(_)) => (theme::OK, "Gerät angeschlossen".to_string()),
                    (Some(k), None) => (theme::WARN, format!("{k} ist nicht angeschlossen")),
                };
                let (r, resp) = ui.allocate_exact_size(vec2(10.0, INPUT_H), Sense::hover());
                ui.painter().circle_filled(r.center(), 4.0, dot);
                resp.on_hover_text(tip);

                ui.label(format!("Universum {}", u + 1));

                let text = match (&current, port) {
                    (None, _) => "– kein Gerät –".to_string(),
                    (Some(_), Some(p)) => p.label.clone(),
                    (Some(k), None) => format!("{k} (nicht angeschlossen)"),
                };
                let mut picked = current.clone();
                ui.allocate_ui(vec2(360.0, INPUT_H), |ui| {
                    full_combo(ui, ("settings_universe", u), text, |ui| {
                        ui.selectable_value(&mut picked, None, "– kein Gerät –");
                        for p in ports.iter() {
                            let label = if p.ftdi {
                                p.label.clone()
                            } else {
                                format!("{} (kein FTDI)", p.label)
                            };
                            ui.selectable_value(&mut picked, Some(p.key.clone()), label);
                        }
                    });
                });
                if picked != current {
                    s.assign_open_dmx(u, picked);
                    ch.save = true;
                    ch.restart_output = true;
                }

                let n = show
                    .fixtures
                    .iter()
                    .filter(|f| usize::from(f.universe) == u)
                    .count();
                ui.label(
                    egui::RichText::new(match n {
                        0 => String::new(),
                        1 => "1 Gerät".to_string(),
                        n => format!("{n} Geräte"),
                    })
                    .color(theme::TEXT_DIM),
                );
                ui.end_row();
            }
        });
}

fn midi(ui: &mut Ui) {
    section(ui, "MIDI-Eingang");
    form(ui, "settings_midi", |f| {
        f.row("Gerät", |ui| {
            ui.add_enabled_ui(false, |ui| {
                full_combo(ui, "settings_midi_dev", "– kein Gerät –", |_| {});
            })
            .response
            .on_disabled_hover_text(
                "Hier wählst du später den MIDI-Controller für das MIDI-Mapping",
            );
        });
    });
}

fn project(ui: &mut Ui, e: &mut Engine, show_path: &str) {
    section(ui, "Show");
    form(ui, "settings_project", |f| {
        f.row("Name", |ui| {
            ui.add(text_field(&mut e.show_mut().name).desired_width(f32::INFINITY));
        });
        f.row("Datei", |ui| {
            let text = if show_path.is_empty() {
                "noch nicht gespeichert"
            } else {
                show_path
            };
            ui.add(egui::Label::new(egui::RichText::new(text).color(theme::TEXT_DIM)).truncate())
                .on_hover_text(text);
        });
        let show = e.show();
        f.row("Inhalt", |ui| {
            ui.label(
                egui::RichText::new(format!(
                    "{} Geräte · {} Szenen · {} Zonen · {} Gruppen",
                    show.fixtures.len(),
                    show.scenes.len(),
                    show.zones.len(),
                    show.groups.len()
                ))
                .color(theme::TEXT_DIM),
            );
        });
    });
}

fn lock(ui: &mut Ui, s: &mut Settings, ch: &mut SettingsChanges) {
    ui.label(theme::heading_text("Oberfläche sperren"))
        .on_hover_text(
            "Gesperrt (Bearbeiten → Sperren, Strg+L) bleibt nur STEUERN bedienbar: Patch, Szenen \
         und Einstellungen lassen sich nicht ändern. Die Sperre bleibt nach einem Neustart.",
        );
    ui.add_space(theme::SP_XS);
    form(ui, "settings_lock", |f| {
        f.row("Passwort", |ui| {
            if s.lock_hash.is_some() {
                ui.label("festgelegt");
                if ui
                    .button("Entfernen")
                    .on_hover_text("Beim nächsten Sperren wird ein neues festgelegt")
                    .clicked()
                {
                    s.lock_hash = None;
                    ch.save = true;
                }
            } else {
                ui.label(egui::RichText::new("nicht festgelegt").color(theme::TEXT_DIM));
            }
        });
    });
}
