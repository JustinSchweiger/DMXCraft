//! Einstellungsfenster: Reiter links, Inhalt rechts.
//!
//! Das Fenster ändert die [`Settings`] direkt und meldet über
//! [`SettingsChanges`], was die App danach tun muss (speichern, Ausgang
//! neu starten, Zoom setzen).

use crate::i18n::t;
use dmxcraft_core::{Engine, UNIVERSES};
use dmxcraft_output::UsbDmxPort;
use eframe::egui::{self, pos2, vec2, Align2, Sense, Stroke, Ui};

use crate::i18n::{self, Lang};
use crate::settings::Settings;
use crate::ui::theme::{self, Theme};
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
    Ai,
    Project,
}

impl Tab {
    const ALL: [Tab; 5] = [Tab::General, Tab::Outputs, Tab::Midi, Tab::Ai, Tab::Project];

    fn label(self) -> &'static str {
        match self {
            Tab::General => t("Allgemein"),
            Tab::Outputs => t("Ausgänge"),
            Tab::Midi => t("MIDI"),
            Tab::Ai => t("KI-Assistent"),
            Tab::Project => t("Projekt"),
        }
    }

    fn icon(self) -> Icon {
        match self {
            Tab::General => Icon::Settings,
            Tab::Outputs => Icon::Levels,
            Tab::Midi => Icon::Midi,
            Tab::Ai => Icon::Robot,
            Tab::Project => Icon::FileOpen,
        }
    }
}

/// Was nach einem Frame des Fensters zu tun ist.
#[derive(Default)]
pub struct SettingsChanges {
    pub save: bool,
    pub restart_output: bool,
    pub zoom: Option<f32>,
    /// Theme gewechselt – egui-Stil neu setzen.
    pub theme: bool,
    /// MCP-Server neu starten.
    pub mcp_retry: bool,
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
    pub mcp: &'a crate::mcp::McpStatus,
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
        Dialog::new(t("Einstellungen"))
            .id("settings")
            .width(WIDTH)
            .height(HEIGHT)
            .flush()
            .cancel(t("Schließen"))
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
                            Tab::Ai => ai(ui, cx.settings, cx.mcp, &mut ch),
                            Tab::Project => project(ui, cx.engine, cx.show_path),
                        }
                    });
            });
        ch
    }
}

/// Reiterleiste links.
fn sidebar(ui: &mut Ui, rect: egui::Rect, tab: &mut Tab) {
    let p = ui.painter();
    p.rect_filled(rect, 0.0, theme::bg());
    p.vline(
        rect.right() - 0.5,
        rect.y_range(),
        Stroke::new(1.0, theme::border()),
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
            p.rect_filled(r, 0.0, theme::surface());
            p.rect_filled(
                egui::Rect::from_min_size(r.min, vec2(3.0, r.height())),
                0.0,
                theme::accent(),
            );
        } else if resp.hovered() {
            p.rect_filled(r, 0.0, theme::raised_hover());
        }
        let color = if active {
            theme::text()
        } else {
            theme::text_dim()
        };
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

/// KI-Assistent: MCP-Server ein/aus, Port und Anleitungen zum Verbinden.
fn ai(ui: &mut Ui, s: &mut Settings, status: &crate::mcp::McpStatus, ch: &mut SettingsChanges) {
    use crate::mcp::McpStatus;
    section(ui, t("KI-Assistent (MCP)"));
    hint(
        ui,
        t("Über das Model Context Protocol kann ein KI-Assistent wie Claude DMXCraft bedienen: Geräte patchen, Szenen, Effekte und SUPER SCENES anlegen, Szenen starten … Der Server ist nur auf diesem Rechner erreichbar."),
    );
    ui.add_space(theme::SP_S);
    ch.save |= ui
        .add(Toggle::new(&mut s.mcp_enabled, t("MCP-Server einschalten")))
        .changed();
    ui.add_space(theme::SP_S);
    form(ui, "settings_mcp", |f| {
        f.row(t("Port"), |ui| {
            let mut port = s.mcp_port;
            let r = ui.add(egui::DragValue::new(&mut port).range(1024..=65535).speed(1));
            if port != s.mcp_port {
                s.mcp_port = port;
            }
            if r.lost_focus() || r.drag_stopped() {
                ch.save = true;
            }
        });
        f.row(t("Status"), |ui| {
            let (text, color) = match status {
                McpStatus::Off => (t("aus").to_string(), theme::text_dim()),
                McpStatus::Running(p) => {
                    (tr!("läuft – http://127.0.0.1:{p}/mcp", p = p), theme::OK)
                }
                McpStatus::Error(e) => (tr!("Fehler: {e}", e = e), theme::DANGER),
            };
            ui.label(egui::RichText::new(text).color(color));
            if matches!(status, McpStatus::Error(_)) && ui.button(t("Erneut versuchen")).clicked() {
                ch.mcp_retry = true;
            }
        });
    });
    ui.add_space(theme::SP_M);
    let url = format!("http://127.0.0.1:{}/mcp", s.mcp_port);
    let exe = crate::mcp::bridge_command();
    let snippets = [
        (
            t("Claude Code (Terminal)"),
            format!("claude mcp add --transport http dmxcraft {url}"),
        ),
        (
            t("Claude Desktop (claude_desktop_config.json)"),
            serde_json::to_string_pretty(&serde_json::json!({
                "mcpServers": { "dmxcraft": { "command": exe, "args": ["mcp"] } }
            }))
            .unwrap_or_default(),
        ),
        (t("Andere Clients (Streamable HTTP)"), url.clone()),
    ];
    for (title, code) in snippets {
        section(ui, title);
        ui.horizontal(|ui| {
            let w = ui.available_width() - 40.0;
            let mut text = code.clone();
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .code_editor()
                    .desired_width(w)
                    .desired_rows(code.lines().count().max(1)),
            );
            if ui
                .add(IconButton::new(Icon::Copy))
                .on_hover_text(t("Kopieren"))
                .clicked()
            {
                ui.ctx().copy_text(code.clone());
            }
        });
        ui.add_space(theme::SP_S);
    }
    hint(
        ui,
        t("Claude Desktop startet DMXCraft dafür mit dem Argument „mcp“ als Brücke; DMXCraft selbst muss laufen und der Server eingeschaltet sein. Änderungen der KI lassen sich mit Strg+Z rückgängig machen; ist DMXCraft gesperrt, darf die KI nur lesen."),
    );
}

fn hint(ui: &mut Ui, text: &str) {
    ui.label(egui::RichText::new(text).small().color(theme::text_faint()));
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
    section(ui, t("Optionen"));
    form(ui, "settings_general", |f| {
        f.row(t("Sprache"), |ui| {
            let current = Lang::from_key(&s.language);
            let mut picked = current;
            // Sprachnamen in der jeweiligen Sprache, damit man sie immer findet
            full_combo(ui, "settings_lang", current.native_name(), |ui| {
                for l in Lang::ALL {
                    ui.selectable_value(&mut picked, l, l.native_name());
                }
            });
            if picked != current {
                s.language = picked.key().into();
                i18n::set_lang(picked);
                ch.save = true;
            }
        });
        f.row(t("Theme"), |ui| {
            let current = Theme::from_key(&s.theme);
            let mut picked = current;
            full_combo(ui, "settings_theme", t(current.label()), |ui| {
                for th in Theme::ALL {
                    ui.selectable_value(&mut picked, th, t(th.label()));
                }
            });
            if picked != current {
                s.theme = picked.key().into();
                theme::set_theme(picked);
                ch.theme = true;
                ch.save = true;
            }
        });
        f.row(t("Darstellungsgröße"), |ui| {
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
        f.row(t("Instanzname"), |ui| {
            let r = ui
                .add(text_field(&mut s.instance_name).desired_width(f32::INFINITY))
                .on_hover_text(t("Name dieses Rechners, z. B. für Fernsteuerung im Netz"));
            if r.lost_focus() {
                ch.save = true;
            }
        });
    });
    ui.add_space(theme::SP_S);
    ch.save |= ui
        .add(Toggle::new(
            &mut s.wrap_scenes,
            t("Szenen am unteren Rand in weitere Spalten umbrechen"),
        ))
        .on_hover_text(t(
            "Aus: alle Szenen einer Bank untereinander (mit Bildlaufleiste)",
        ))
        .changed();
    ui.add_space(theme::SP_M);
    section(ui, t("Startup"));
    ch.save |= ui
        .add(Toggle::new(
            &mut s.open_last_show,
            t("Letzte Datei automatisch laden"),
        ))
        .changed();
    ch.save |= ui
        .add(Toggle::new(&mut s.start_touch, t("Im Touch-Modus starten")))
        .on_hover_text(t(
            "Der Touch-Modus folgt – die Einstellung wird schon gespeichert",
        ))
        .changed();
    ch.save |= ui
        .add(Toggle::new(
            &mut s.check_updates,
            t("Beim Start nach Updates suchen"),
        ))
        .changed();
    ui.add_space(theme::SP_L);
    if let Some(p) = Settings::path() {
        hint(ui, &tr!("Gespeichert in {v}", v = p.display()));
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
        ui.label(theme::heading_text(t("Universen"))).on_hover_text(
            "Ein USB-DMX-Kabel (Open DMX / FTDI) je Universum. Die Zuordnung hängt an der \
             Seriennummer des Kabels und bleibt beim Umstecken erhalten.",
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(IconButton::new(Icon::Reset).flat())
                .on_hover_text(t("Geräte neu suchen"))
                .clicked()
            {
                *ports = dmxcraft_output::list_ports();
            }
            let n = ports.iter().filter(|p| p.ftdi).count();
            ui.label(
                egui::RichText::new(match n {
                    0 => t("kein USB-DMX-Kabel gefunden").to_string(),
                    1 => t("1 Kabel gefunden").to_string(),
                    n => tr!("{n} Kabel gefunden", n = n),
                })
                .color(theme::text_dim()),
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
                        theme::text_faint(),
                        t("Kein Gerät – wird nicht ausgegeben").to_string(),
                    ),
                    (Some(_), Some(_)) => (theme::OK, t("Gerät angeschlossen").to_string()),
                    (Some(k), None) => (theme::WARN, tr!("{k} ist nicht angeschlossen", k = k)),
                };
                let (r, resp) = ui.allocate_exact_size(vec2(10.0, INPUT_H), Sense::hover());
                ui.painter().circle_filled(r.center(), 4.0, dot);
                resp.on_hover_text(tip);

                ui.label(tr!("Universum {u_nr}", u_nr = u + 1));

                let text = match (&current, port) {
                    (None, _) => t("– kein Gerät –").to_string(),
                    (Some(_), Some(p)) => p.label.clone(),
                    (Some(k), None) => tr!("{k} (nicht angeschlossen)", k = k),
                };
                let mut picked = current.clone();
                ui.allocate_ui(vec2(360.0, INPUT_H), |ui| {
                    full_combo(ui, ("settings_universe", u), text, |ui| {
                        ui.selectable_value(&mut picked, None, t("– kein Gerät –"));
                        for p in ports.iter() {
                            let label = if p.ftdi {
                                p.label.clone()
                            } else {
                                tr!("{label} (kein FTDI)", label = p.label)
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
                        1 => t("1 Gerät").to_string(),
                        n => tr!("{n} Geräte", n = n),
                    })
                    .color(theme::text_dim()),
                );
                ui.end_row();
            }
        });
}

fn midi(ui: &mut Ui) {
    section(ui, t("MIDI-Eingang"));
    form(ui, "settings_midi", |f| {
        f.row(t("Gerät"), |ui| {
            ui.add_enabled_ui(false, |ui| {
                full_combo(ui, "settings_midi_dev", t("– kein Gerät –"), |_| {});
            })
            .response
            .on_disabled_hover_text(t(
                "Hier wählst du später den MIDI-Controller für das MIDI-Mapping",
            ));
        });
    });
}

fn project(ui: &mut Ui, e: &mut Engine, show_path: &str) {
    section(ui, t("Show"));
    form(ui, "settings_project", |f| {
        f.row(t("Datei"), |ui| {
            let text = if show_path.is_empty() {
                t("noch nicht gespeichert")
            } else {
                show_path
            };
            ui.add(egui::Label::new(egui::RichText::new(text).color(theme::text_dim())).truncate())
                .on_hover_text(text);
        });
        let show = e.show();
        f.row(t("Inhalt"), |ui| {
            ui.label(
                egui::RichText::new(tr!(
                    "{n} Geräte · {n1} Szenen · {n2} Zonen · {n3} Gruppen",
                    n = show.fixtures.len(),
                    n1 = show.scenes.len(),
                    n2 = show.zones.len(),
                    n3 = show.groups.len()
                ))
                .color(theme::text_dim()),
            );
        });
    });
}
