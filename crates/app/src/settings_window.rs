//! Settings window: tabs on the left, content on the right.
//!
//! The window changes the [`Settings`] directly and reports via
//! [`SettingsChanges`] what the app has to do afterwards (save, restart the
//! output, set the zoom).

use crate::i18n::t;
use crate::midi_device::{DeviceModel, MidiDevice};
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

/// Steps of the display size in percent.
pub const SCALES: [u32; 9] = [60, 75, 90, 100, 110, 125, 150, 175, 200];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tab {
    #[default]
    General,
    Customize,
    Startup,
    Outputs,
    Midi,
    Audio,
    Obs,
    Remote,
    Ai,
    Project,
}

impl Tab {
    const ALL: [Tab; 10] = [
        Tab::General,
        Tab::Customize,
        Tab::Startup,
        Tab::Outputs,
        Tab::Midi,
        Tab::Audio,
        Tab::Obs,
        Tab::Remote,
        Tab::Ai,
        Tab::Project,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::General => t("General"),
            Tab::Customize => t("Customize"),
            Tab::Startup => t("Start"),
            Tab::Outputs => t("Outputs"),
            Tab::Midi => t("MIDI"),
            Tab::Audio => t("Audio"),
            Tab::Obs => t("OBS"),
            Tab::Remote => t("Remote"),
            Tab::Ai => t("MCP"),
            Tab::Project => t("Project"),
        }
    }

    fn icon(self) -> Icon {
        match self {
            Tab::General => Icon::Settings,
            Tab::Customize => Icon::Sliders,
            Tab::Startup => Icon::Play,
            Tab::Outputs => Icon::Levels,
            Tab::Midi => Icon::Midi,
            Tab::Audio => Icon::SpeakerHigh,
            Tab::Obs => Icon::Camera,
            Tab::Remote => Icon::Remote,
            Tab::Ai => Icon::Robot,
            Tab::Project => Icon::FileOpen,
        }
    }
}

/// What to do after a frame of the window.
#[derive(Default)]
pub struct SettingsChanges {
    pub save: bool,
    pub restart_output: bool,
    pub zoom: Option<f32>,
    /// Theme changed – set the egui style again.
    pub theme: bool,
    /// Restart the MCP server.
    pub mcp_retry: bool,
    /// Restart the remote.
    pub remote_retry: bool,
    /// Clear the LEDs of this output now.
    pub midi_clear: Option<(String, DeviceModel)>,
}

#[derive(Default)]
pub struct SettingsWindow {
    pub tab: Tab,
}

/// What the window needs from the app.
pub struct SettingsCx<'a> {
    pub settings: &'a mut Settings,
    pub engine: &'a mut Engine,
    pub usb_ports: &'a mut Vec<UsbDmxPort>,
    pub show_path: &'a str,
    pub mcp: &'a crate::mcp::McpStatus,
    pub remote: &'a crate::remote::RemoteLink,
    /// Detected MIDI inputs and outputs.
    pub midi_inputs: &'a [String],
    pub midi_outputs: &'a [String],
    pub obs: &'a crate::obs::Obs,
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
        Dialog::new(t("Settings"))
            .id("settings")
            .width(WIDTH)
            .height(HEIGHT)
            .flush()
            .cancel(t("Close"))
            .show(ctx, open, |ui, _| {
                // Fixed layout: tab bar on the left, content on the right
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
                            Tab::Customize => customize(ui, cx.settings, &mut ch),
                            Tab::Startup => startup(ui, cx.settings, &mut ch),
                            Tab::Outputs => {
                                outputs(ui, cx.settings, cx.engine, cx.usb_ports, &mut ch)
                            }
                            Tab::Midi => {
                                midi(ui, cx.settings, cx.midi_inputs, cx.midi_outputs, &mut ch)
                            }
                            Tab::Audio => audio(ui, cx.settings, &mut ch),
                            Tab::Obs => obs(ui, cx.engine, cx.obs),
                            Tab::Remote => remote(ui, cx.settings, cx.remote, &mut ch),
                            Tab::Ai => ai(ui, cx.settings, cx.mcp, &mut ch),
                            Tab::Project => project(ui, cx.engine, cx.show_path),
                        }
                    });
            });
        ch
    }
}

/// Tab bar on the left.
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

/// AI assistant: MCP server on/off, port and instructions for connecting.
fn ai(ui: &mut Ui, s: &mut Settings, status: &crate::mcp::McpStatus, ch: &mut SettingsChanges) {
    use crate::mcp::McpStatus;
    section(ui, t("MCP"));
    ch.save |= ui
        .add(Toggle::new(&mut s.mcp_enabled, t("Enable MCP server")))
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
                McpStatus::Off => (t("off").to_string(), theme::text_dim()),
                McpStatus::Running(p) => {
                    (tr!("running – http://127.0.0.1:{p}/mcp", p = p), theme::OK)
                }
                McpStatus::Error(e) => (tr!("Error: {e}", e = e), theme::DANGER),
            };
            ui.label(egui::RichText::new(text).color(color));
            if matches!(status, McpStatus::Error(_)) && ui.button(t("Try again")).clicked() {
                ch.mcp_retry = true;
            }
        });
    });
    ui.add_space(theme::SP_M);
    let url = format!("http://127.0.0.1:{}/mcp", s.mcp_port);
    let (exe, args) = crate::mcp::bridge_command(s.mcp_port);
    let snippets = [
        (
            t("Claude Code (terminal)"),
            format!("claude mcp add --transport http dmxcraft {url}"),
        ),
        (
            t("Claude Desktop (claude_desktop_config.json)"),
            serde_json::to_string_pretty(&serde_json::json!({
                "mcpServers": { "dmxcraft": { "command": exe, "args": args } }
            }))
            .unwrap_or_default(),
        ),
        (t("Other clients (Streamable HTTP)"), url.clone()),
    ];
    for (title, code) in snippets {
        section(ui, title);
        ui.horizontal(|ui| {
            let w = ui.available_width() - 40.0;
            // read-only, but selectable and copyable
            let mut text: &str = &code;
            ui.add(
                egui::TextEdit::multiline(&mut text)
                    .code_editor()
                    .desired_width(w)
                    .desired_rows(code.lines().count().max(1)),
            );
            if ui
                .add(IconButton::new(Icon::Copy))
                .on_hover_text(t("Copy"))
                .clicked()
            {
                ui.ctx().copy_text(code.clone());
            }
        });
        ui.add_space(theme::SP_S);
    }
    hint(
        ui,
        t("Claude Desktop starts DMXCraft with the argument “mcp” as a bridge; DMXCraft itself must be running with the server switched on. Changes made by the AI can be undone with Ctrl+Z; while DMXCraft is locked the AI may only read."),
    );
}

/// Sound: maximum volume, reset at startup, fade-out.
fn audio(ui: &mut Ui, s: &mut Settings, ch: &mut SettingsChanges) {
    section(ui, t("Audio"));
    form(ui, "settings_audio_limit", |f| {
        f.row(t("Maximum volume"), |ui| {
            let r = ui.add(
                egui::DragValue::new(&mut s.audio_limit_pct)
                    .range(1..=100)
                    .speed(0.5)
                    .suffix(" %"),
            );
            if r.lost_focus() || r.drag_stopped() {
                ch.save = true;
            }
        });
    });
    hint(
        ui,
        t("Sound master 100 % (and a MIDI fader at the top) equals this volume – e.g. 30 % for testing without changing the system volume."),
    );
    ui.add_space(theme::SP_S);
    ch.save |= ui
        .add(Toggle::new(
            &mut s.audio_reset_on_start,
            t("Starting a SUPER SCENE resets the sound master to the maximum volume"),
        ))
        .changed();
    hint(
        ui,
        t("A MIDI fader on the sound master only takes effect again once it reaches the current value – so it never jumps."),
    );
    ui.add_space(theme::SP_S);
    form(ui, "settings_audio", |f| {
        f.row(t("Fade out (default)"), |ui| {
            let mut secs = s.audio_fade_ms as f32 / 1000.0;
            let r = ui.add(
                egui::DragValue::new(&mut secs)
                    .range(0.5..=60.0)
                    .speed(0.1)
                    .fixed_decimals(1)
                    .suffix(" s"),
            );
            s.audio_fade_ms = (secs * 1000.0).round() as u32;
            if r.lost_focus() || r.drag_stopped() {
                ch.save = true;
            }
        });
    });
    ch.save |= ui
        .add(Toggle::new(
            &mut s.audio_fade_stop,
            t("Stop SUPER SCENES with sound after fading out"),
        ))
        .changed();
    ui.add_space(theme::SP_M);
    hint(
        ui,
        t("Only affects playback of the audio clips in SUPER SCENES, not the system volume. Sound master, mute and fade out are in the title bar and available as targets in MIDI mapping."),
    );
}

/// Touch remote: server on/off, port, PIN, addresses and QR code.
fn remote(
    ui: &mut Ui,
    s: &mut Settings,
    link: &crate::remote::RemoteLink,
    ch: &mut SettingsChanges,
) {
    use crate::remote::RemoteStatus;
    section(ui, t("Touch remote"));
    ch.save |= ui
        .add(Toggle::new(
            &mut s.remote_enabled,
            t("Enable remote control on the local network"),
        ))
        .changed();
    ui.add_space(theme::SP_S);
    form(ui, "settings_remote", |f| {
        f.row(t("Port"), |ui| {
            let mut port = s.remote_port;
            let r = ui.add(egui::DragValue::new(&mut port).range(1024..=65535).speed(1));
            if port != s.remote_port {
                s.remote_port = port;
            }
            if r.lost_focus() || r.drag_stopped() {
                ch.save = true;
            }
        });
        f.row(t("PIN"), |ui| {
            let r = ui.add(
                egui::TextEdit::singleline(&mut s.remote_pin)
                    .hint_text(t("no PIN"))
                    .char_limit(12)
                    .desired_width(ui.available_width()),
            );
            if r.changed() {
                s.remote_pin.retain(|c| !c.is_whitespace());
            }
            if r.lost_focus() {
                ch.save = true;
            }
        });
        f.row(t("Status"), |ui| {
            let (text, color) = match &link.status {
                RemoteStatus::Off => (t("off").to_string(), theme::text_dim()),
                RemoteStatus::Running(_) if link.clients > 0 => (
                    tr!("running – {n} device(s) connected", n = link.clients),
                    theme::OK,
                ),
                RemoteStatus::Running(_) => {
                    (t("running – no device connected").to_string(), theme::OK)
                }
                RemoteStatus::Error(e) => (tr!("Error: {e}", e = e), theme::DANGER),
            };
            ui.label(egui::RichText::new(text).color(color));
            if matches!(link.status, RemoteStatus::Error(_)) && ui.button(t("Try again")).clicked()
            {
                ch.remote_retry = true;
            }
        });
    });
    if matches!(link.status, RemoteStatus::Running(_)) {
        ui.add_space(theme::SP_M);
        section(ui, t("Connect"));
        ui.horizontal_top(|ui| {
            if let Some(url) = link.addresses.first() {
                qr_code(ui, url, 180.0);
                ui.add_space(theme::SP_M);
            }
            ui.vertical(|ui| {
                ui.label(t("Scan with the camera of your phone or tablet, or open an address in the browser:"));
                ui.add_space(theme::SP_S);
                for url in &link.addresses {
                    ui.horizontal(|ui| {
                        ui.hyperlink_to(egui::RichText::new(url).monospace(), url);
                        if ui
                            .add(IconButton::new(Icon::Copy))
                            .on_hover_text(t("Copy"))
                            .clicked()
                        {
                            ui.ctx().copy_text(url.clone());
                        }
                    });
                }
                if link.addresses.is_empty() {
                    ui.label(
                        egui::RichText::new(t("No network address found."))
                            .color(theme::WARN),
                    );
                }
            });
        });
    }
    ui.add_space(theme::SP_M);
    hint(
        ui,
        t("The remote shows the touch mode pages in the browser (iPhone, iPad, Android). Device and computer must be on the same network; the firewall may ask on first start. Use “Add to Home Screen” to run it full screen like an app. Without a PIN, anyone on the network can control it."),
    );
}

/// Draw the QR code of an address (black on white, with quiet zone).
fn qr_code(ui: &mut Ui, text: &str, size: f32) {
    let (rect, _) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    let Ok(code) = qrcode::QrCode::new(text.as_bytes()) else {
        return;
    };
    let n = code.width();
    let p = ui.painter();
    p.rect_filled(rect, theme::R_S, egui::Color32::WHITE);
    let cell = size / (n + 8) as f32;
    let origin = rect.min + vec2(cell * 4.0, cell * 4.0);
    for y in 0..n {
        for x in 0..n {
            if code[(x, y)] == qrcode::Color::Dark {
                p.rect_filled(
                    egui::Rect::from_min_size(
                        origin + vec2(x as f32 * cell, y as f32 * cell),
                        vec2(cell + 0.3, cell + 0.3),
                    ),
                    0.0,
                    egui::Color32::BLACK,
                );
            }
        }
    }
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
    section(ui, t("Options"));
    form(ui, "settings_general", |f| {
        f.row(t("Language"), |ui| {
            let current = Lang::from_key(&s.language);
            let mut picked = current;
            // Language names in the respective language so they can always be found
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
        f.row(t("Instance name"), |ui| {
            let r = ui
                .add(text_field(&mut s.instance_name).desired_width(f32::INFINITY))
                .on_hover_text(t(
                    "Name of this computer, e.g. for remote control over the network",
                ));
            if r.lost_focus() {
                ch.save = true;
            }
        });
    });
    ui.add_space(theme::SP_L);
    if let Some(p) = Settings::path() {
        hint(ui, &tr!("Stored in {v}", v = p.display()));
    }
}

/// Customize: appearance and display.
fn customize(ui: &mut Ui, s: &mut Settings, ch: &mut SettingsChanges) {
    section(ui, t("Appearance"));
    form(ui, "settings_customize", |f| {
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
        f.row(t("Display size"), |ui| {
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
    });
    ui.add_space(theme::SP_M);
    section(ui, t("Display"));
    ch.save |= ui
        .add(Toggle::new(
            &mut s.wrap_scenes,
            t("Wrap scenes into further columns at the bottom"),
        ))
        .on_hover_text(t("Off: all scenes of a bank stacked (with scroll bar)"))
        .changed();
    ch.save |= ui
        .add(Toggle::new(
            &mut s.fader_ranges,
            t("Show value ranges on the fader tracks"),
        ))
        .on_hover_text(t("Off: plain tracks; the label still appears while moving"))
        .changed();
    ch.save |= ui
        .add(Toggle::new(
            &mut s.tools_as_tabs,
            t("3D visualizer and Profile Builder as tabs in the title bar"),
        ))
        .on_hover_text(t(
            "In addition to Setup, Control and Touch; the button in the tab still opens them in their own window",
        ))
        .changed();
}

/// Startup: what happens when the program starts.
fn startup(ui: &mut Ui, s: &mut Settings, ch: &mut SettingsChanges) {
    section(ui, t("On startup"));
    ch.save |= ui
        .add(Toggle::new(
            &mut s.open_last_show,
            t("Load last file automatically"),
        ))
        .changed();
    ch.save |= ui
        .add(Toggle::new(&mut s.start_touch, t("Start in touch mode")))
        .on_hover_text(t("Touch mode is coming – the setting is already saved"))
        .changed();
    ch.save |= ui
        .add(Toggle::new(
            &mut s.check_updates,
            t("Check for updates on startup"),
        ))
        .changed();
}

fn outputs(
    ui: &mut Ui,
    s: &mut Settings,
    e: &Engine,
    ports: &mut Vec<UsbDmxPort>,
    ch: &mut SettingsChanges,
) {
    ui.horizontal(|ui| {
        ui.label(theme::heading_text(t("Universes"))).on_hover_text(
            t("One USB DMX cable (Open DMX / FTDI) per universe. The assignment follows the cable's serial number and survives replugging."),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(IconButton::new(Icon::Reset).flat())
                .on_hover_text(t("Search for devices again"))
                .clicked()
            {
                *ports = dmxcraft_output::list_ports();
            }
            let n = ports.iter().filter(|p| p.ftdi).count();
            ui.label(
                egui::RichText::new(match n {
                    0 => t("no USB DMX cable found").to_string(),
                    1 => t("1 cable found").to_string(),
                    n => tr!("{n} cables found", n = n),
                })
                .color(theme::text_dim()),
            );
        });
    });
    ui.add_space(theme::SP_XS);

    // Per universe: device, state, number of patched fixtures
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
                // Status dot
                let (dot, tip) = match (&current, port) {
                    (None, _) => (theme::text_faint(), t("No device – not output").to_string()),
                    (Some(_), Some(_)) => (theme::OK, t("Device connected").to_string()),
                    (Some(k), None) => (theme::WARN, tr!("{k} is not connected", k = k)),
                };
                let (r, resp) = ui.allocate_exact_size(vec2(10.0, INPUT_H), Sense::hover());
                ui.painter().circle_filled(r.center(), 4.0, dot);
                resp.on_hover_text(tip);

                ui.label(tr!("Universe {u_nr}", u_nr = u + 1));

                let text = match (&current, port) {
                    (None, _) => t("– no device –").to_string(),
                    (Some(_), Some(p)) => p.label.clone(),
                    (Some(k), None) => tr!("{k} (not connected)", k = k),
                };
                let mut picked = current.clone();
                ui.allocate_ui(vec2(360.0, INPUT_H), |ui| {
                    full_combo(ui, ("settings_universe", u), text, |ui| {
                        ui.selectable_value(&mut picked, None, t("– no device –"));
                        for p in ports.iter() {
                            let label = if p.ftdi {
                                p.label.clone()
                            } else {
                                tr!("{label} (not FTDI)", label = p.label)
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
                        1 => t("1 fixture").to_string(),
                        n => tr!("{n} fixtures", n = n),
                    })
                    .color(theme::text_dim()),
                );
                ui.end_row();
            }
        });
}

fn midi(
    ui: &mut Ui,
    s: &mut Settings,
    inputs: &[String],
    outputs: &[String],
    ch: &mut SettingsChanges,
) {
    ui.horizontal(|ui| {
        ui.label(theme::heading_text(t("MIDI controllers")));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let free: Vec<&String> = inputs
                .iter()
                .filter(|i| !s.midi_devices.iter().any(|d| &d.input == *i))
                .collect();
            let add = ui
                .add(IconButton::new(Icon::Plus).flat().enabled(!free.is_empty()))
                .on_hover_text(t("Add device"));
            if add.clicked() {
                egui::Popup::toggle_id(ui.ctx(), add.id);
            }
            crate::ui::widgets::menu::dropdown(&add, |ui| {
                for i in free {
                    if ui.button(i.as_str()).clicked() {
                        s.midi_devices.push(MidiDevice::new(i, outputs));
                        ch.save = true;
                        ui.close();
                    }
                }
            });
        });
    });
    hint(
        ui,
        t("All connected MIDI inputs are opened for mapping. Devices set up here get their type (button colours) and the output for LED feedback."),
    );
    ui.add_space(theme::SP_S);
    if s.midi_devices.is_empty() {
        hint(
            ui,
            if inputs.is_empty() {
                t("No MIDI device found.")
            } else {
                t("No device set up yet – add one with “+”.")
            },
        );
        return;
    }
    let mut remove = None;
    for (i, d) in s.midi_devices.iter_mut().enumerate() {
        let before = d.clone();
        let connected = inputs.contains(&d.input);
        egui::Frame::new()
            .fill(theme::raised())
            .stroke(egui::Stroke::new(1.0, theme::border()))
            .corner_radius(theme::R_M)
            .inner_margin(theme::SP_M)
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.horizontal(|ui| {
                    let (r, dot) = ui.allocate_exact_size(vec2(10.0, 18.0), Sense::hover());
                    let c = if connected {
                        theme::OK
                    } else {
                        theme::text_faint()
                    };
                    ui.painter().circle_filled(r.center(), 4.0, c);
                    dot.on_hover_text(if connected {
                        t("Connected")
                    } else {
                        t("Not connected")
                    });
                    ui.label(egui::RichText::new(&d.input).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(IconButton::new(Icon::Trash).flat().danger())
                            .on_hover_text(t("Remove device"))
                            .clicked()
                        {
                            remove = Some(i);
                        }
                    });
                });
                ui.add_space(theme::SP_XS);
                form(ui, &format!("settings_midi_dev_{i}"), |f| {
                    f.row(t("Type"), |ui| {
                        full_combo(ui, ("midi_model", i), d.model.label(), |ui| {
                            for m in DeviceModel::ALL {
                                ui.selectable_value(&mut d.model, m, m.label());
                            }
                        });
                    });
                    f.row(t("LED output"), |ui| {
                        let shown = if d.output.is_empty() {
                            t("– no output –").to_string()
                        } else {
                            d.output.clone()
                        };
                        full_combo(ui, ("midi_out", i), shown, |ui| {
                            ui.selectable_value(&mut d.output, String::new(), t("– no output –"));
                            for o in outputs {
                                ui.selectable_value(&mut d.output, o.clone(), o);
                            }
                        });
                    });
                    f.row(t("Clear LEDs"), |ui| {
                        ui.add(Toggle::new(&mut d.clear_on_connect, t("on connect")));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add_enabled(
                                    !d.output.is_empty(),
                                    egui::Button::new(t("Clear now")),
                                )
                                .clicked()
                            {
                                ch.midi_clear = Some((d.output.clone(), d.model));
                            }
                        });
                    });
                });
            });
        ui.add_space(theme::SP_S);
        if *d != before {
            ch.save = true;
        }
    }
    if let Some(i) = remove {
        s.midi_devices.remove(i);
        ch.save = true;
    }
}

/// OBS connection (belongs to the show, saved with it).
fn obs(ui: &mut Ui, e: &mut Engine, link: &crate::obs::Obs) {
    use crate::obs::Status;
    let mut cfg = e.show().obs.clone();
    section(ui, t("OBS"));
    ui.add(Toggle::new(&mut cfg.enabled, t("Connect to OBS")));
    ui.add_space(theme::SP_S);
    form(ui, "settings_obs", |f| {
        f.row(t("Address"), |ui| {
            ui.add(egui::TextEdit::singleline(&mut cfg.host).desired_width(220.0));
        });
        f.row(t("Port"), |ui| {
            ui.add(
                egui::DragValue::new(&mut cfg.port)
                    .range(1..=65535)
                    .speed(1),
            );
        });
        f.row(t("Password"), |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut cfg.password)
                    .password(true)
                    .desired_width(220.0),
            );
        });
        f.row(t("Media source for videos"), |ui| {
            ui.add(egui::TextEdit::singleline(&mut cfg.video_source).desired_width(220.0));
        });
        f.row(t("Status"), |ui| {
            let (text, color) = match link.status() {
                Status::Off => (t("Not connected").to_string(), theme::text_dim()),
                Status::Connecting => (t("Connecting …").to_string(), theme::WARN),
                Status::Connected => (
                    tr!(
                        "Connected – {scenes} scenes, {inputs} sources",
                        scenes = link.scenes().len(),
                        inputs = link.inputs().len()
                    ),
                    theme::OK,
                ),
                Status::Failed(e) => (tr!("Error: {e}", e = e), theme::DANGER),
            };
            ui.label(egui::RichText::new(text).color(color));
        });
        if let (crate::obs::Status::Connected, Some(err)) = (link.status(), link.last_error()) {
            f.row(t("Last message"), |ui| {
                ui.label(egui::RichText::new(err).color(theme::text_faint()));
            });
        }
    });
    if cfg != e.show().obs {
        e.show_mut().obs = cfg;
    }
}

fn project(ui: &mut Ui, e: &mut Engine, show_path: &str) {
    section(ui, t("Show"));
    form(ui, "settings_project", |f| {
        f.row(t("File"), |ui| {
            let text = if show_path.is_empty() {
                t("not saved yet")
            } else {
                show_path
            };
            ui.add(egui::Label::new(egui::RichText::new(text).color(theme::text_dim())).truncate())
                .on_hover_text(text);
        });
        let show = e.show();
        f.row(t("Content"), |ui| {
            ui.label(
                egui::RichText::new(tr!(
                    "{n} fixtures · {n1} scenes · {n2} banks · {n3} groups",
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
