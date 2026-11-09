//! Eigene Titelleiste (das Fenster hat keine Betriebssystem-Leiste):
//! Menü und Modus links, Programm- und Showname in der Mitte, Statistik und
//! Fensterknöpfe rechts. Leere Fläche: Ziehen = Fenster bewegen,
//! Doppelklick = maximieren.

use crate::i18n::t;
use eframe::egui::{self, Color32, Sense, ViewportCommand};

use crate::state::Mode;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton, Segmented};

pub const HEIGHT: f32 = 36.0;

/// Was die Titelleiste zum Zeichnen braucht.
pub struct TitleInfo<'a> {
    pub show_name: &'a str,
    /// Ausgangsstatus als Text und Farbe (für den Punkt und den Tooltip).
    pub status: (String, Color32),
    /// Auslastung des Ausgabe-Takts 0..1.
    pub load: f32,
    pub fps: f32,
    pub locked: bool,
    pub output: dmxcraft_output::OutputKind,
    pub blackout: bool,
    /// Zuletzt geöffnete Shows (neueste zuerst).
    pub recent: &'a [String],
    pub can_undo: bool,
    pub can_redo: bool,
    /// Kurzzeitige Meldung (Text, ist Fehler).
    pub message: Option<(&'a str, bool)>,
    pub update: UpdateBadge,
    /// Aktiver Zuordnungsmodus.
    pub mapping: Option<dmxcraft_core::mapping::MapKind>,
    /// Ton-Master (Lautstärke der Audioblöcke).
    pub audio: AudioInfo,
    /// Anzahl fehlender Audiodateien (rotes Ausrufezeichen).
    pub missing_audio: usize,
    /// OBS-Verbindung (nur wenn eingeschaltet): verbunden?
    pub obs: Option<bool>,
    /// 3D-Visualisierer und Profile Builder als Reiter.
    pub tools_as_tabs: bool,
}

/// Zustand des Ton-Masters für die Titelleiste.
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioInfo {
    pub master: f32,
    pub mute: bool,
    /// MIDI-Fader wartet auf Übernahme.
    pub pickup: bool,
    pub fading: bool,
    pub fade_ms: u32,
    /// Höchstlautstärke 0–1 (Ton-Master 100 % entspricht diesem Wert).
    pub limit: f32,
}

/// Update-Hinweis in der Titelleiste.
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateBadge {
    None,
    Available(String),
    Busy(&'static str),
    Installed(String),
    InstallerStarted(String),
    Error(String),
}

/// Menübefehle, die der App-Rahmen ausführt.
#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    NewShow,
    OpenShow,
    OpenRecent(String),
    SaveShow,
    SaveShowAs,
    Undo,
    Redo,
    Quit,
    ToggleFullscreen,
    Zoom(f32),
    ToggleBlackout,
    DmxLevels,
    /// 3D-Visualisierer im eigenen Fenster öffnen/schließen.
    Visualizer,
    ProfileBuilder,
    Settings,
    CheckUpdates,
    InstallUpdate,
    ToggleLock,
    /// Ton-Master setzen.
    AudioMaster(f32),
    AudioMute,
    AudioFade,
    /// Fenster „Fehlende Audiodateien“.
    MissingAudio,
    /// Einstellungen, Reiter „OBS“.
    Obs,
    /// Tastatur- bzw. MIDI-Zuordnung ein/aus.
    Mapping(dmxcraft_core::mapping::MapKind),
    About,
}

pub fn show(ui: &mut egui::Ui, info: &TitleInfo<'_>, mode: &mut Mode) -> Vec<MenuAction> {
    if info.locked && *mode == Mode::Setup {
        *mode = Mode::Control;
    }
    let mut actions = Vec::new();
    let full = ui.max_rect();
    // Hintergrundfläche zum Ziehen (zuerst anlegen, damit Knöpfe darüber liegen)
    let drag = ui.interact(
        full,
        ui.scope_id().with("titlebar_drag"),
        Sense::click_and_drag(),
    );
    if drag.drag_started() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if drag.double_clicked() {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
    }

    // Mitte: Programm- und Showname
    let painter = ui.painter();
    let center = full.center();
    let name = egui::text::LayoutJob::simple_singleline(
        "DMXCraft".into(),
        egui::FontId::proportional(theme::FS_TITLE),
        theme::text(),
    );
    let g1 = painter.layout_job(name);
    let g2 = painter.layout_no_wrap(
        format!("  ·  {}", info.show_name),
        theme::body(),
        theme::text_dim(),
    );
    let w = g1.size().x + g2.size().x;
    let x0 = center.x - w / 2.0;
    // Eine Statusmeldung rechts kann bis zur Mitte reichen – solange sie
    // steht, tritt der Titel zurück, statt sich mit ihr zu überlagern.
    if info.message.is_none() {
        painter.galley(
            egui::pos2(x0, center.y - g1.size().y / 2.0),
            g1.clone(),
            theme::text(),
        );
        painter.galley(
            egui::pos2(x0 + g1.size().x, center.y - g2.size().y / 2.0),
            g2,
            theme::text_dim(),
        );
    }

    ui.horizontal_centered(|ui| {
        ui.add_space(6.0);
        main_menu(ui, info, &mut actions);
        ui.add_space(theme::SP_XS);
        let mut tabs: Vec<(Mode, &str)> = Vec::new();
        if !info.locked {
            tabs.push((Mode::Setup, t("Einrichten")));
        }
        tabs.push((Mode::Control, t("Steuern")));
        tabs.push((Mode::Touch, t("Touch")));
        tabs.push((Mode::Sound, t("Sound")));
        if info.tools_as_tabs {
            tabs.push((Mode::Visualizer, t("3D-Visualisierer")));
            if !info.locked {
                tabs.push((Mode::Builder, t("Profile Builder")));
            }
        }
        Segmented::new(&tabs).show(ui, mode);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if ui
                .add(IconButton::new(Icon::Close).size(HEIGHT).flat().danger())
                .on_hover_text(t("Schließen"))
                .clicked()
            {
                actions.push(MenuAction::Quit);
            }
            let (icon, tip) = if maximized {
                (Icon::Restore, t("Wiederherstellen"))
            } else {
                (Icon::Maximize, t("Maximieren"))
            };
            if ui
                .add(IconButton::new(icon).size(HEIGHT).flat())
                .on_hover_text(tip)
                .clicked()
            {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
            }
            if ui
                .add(IconButton::new(Icon::Minimize).size(HEIGHT).flat())
                .on_hover_text(t("Minimieren"))
                .clicked()
            {
                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
            }
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            ui.add_space(theme::SP_M);
            if info.blackout {
                ui.label(
                    egui::RichText::new(t("BLACKOUT"))
                        .strong()
                        .color(theme::DANGER),
                );
            }
            match &info.update {
                UpdateBadge::None => {}
                UpdateBadge::Available(v) => {
                    let b =
                        egui::Button::new(tr!("Update auf v{v}", v = v)).fill(theme::accent_dim());
                    if ui
                        .add(b)
                        .on_hover_text(t("Neue Version von GitHub laden"))
                        .clicked()
                    {
                        actions.push(MenuAction::InstallUpdate);
                    }
                }
                UpdateBadge::Busy(t) => {
                    ui.label(egui::RichText::new(*t).color(theme::text_dim()));
                }
                UpdateBadge::Installed(v) => {
                    ui.label(
                        egui::RichText::new(tr!("v{v} installiert – neu starten", v = v))
                            .color(theme::OK),
                    );
                }
                UpdateBadge::InstallerStarted(v) => {
                    ui.label(
                        egui::RichText::new(tr!(
                            "Installer für v{v} gestartet – DMXCraft schließen",
                            v = v
                        ))
                        .color(theme::OK),
                    );
                }
                UpdateBadge::Error(e) => {
                    if ui
                        .small_button(t("Update-Fehler"))
                        .on_hover_text(tr!("{e}\nKlicken für neuen Versuch", e = e))
                        .clicked()
                    {
                        actions.push(MenuAction::CheckUpdates);
                    }
                }
            }
            let (icon, tip) = if info.locked {
                (Icon::Locked, t("Oberfläche entsperren"))
            } else {
                (Icon::Unlocked, t("Oberfläche sperren (Passwort)"))
            };
            if ui
                .add(IconButton::new(icon).size(28.0).flat().active(info.locked))
                .on_hover_text(tip)
                .clicked()
            {
                actions.push(MenuAction::ToggleLock);
            }
            if !info.locked {
                use dmxcraft_core::mapping::MapKind;
                for (kind, icon, tip) in [
                    (MapKind::Midi, Icon::Midi, t("MIDI-Zuordnung")),
                    (MapKind::Keyboard, Icon::Keyboard, t("Tastatur-Zuordnung")),
                ] {
                    let on = info.mapping == Some(kind);
                    let mut b = IconButton::new(icon).size(28.0).flat().active(on);
                    if on {
                        b = b.tint(crate::mapping::color(kind));
                    }
                    if ui.add(b).on_hover_text(tip).clicked() {
                        actions.push(MenuAction::Mapping(kind));
                    }
                }
            }
            ui.add_space(theme::SP_XS);
            load_meter(ui, info);
            ui.add_space(theme::SP_M);
            audio_control(ui, info.audio, &mut actions);
            if info.missing_audio > 0 {
                missing_alert(ui, info.missing_audio, &mut actions);
            }
            if let Some(ok) = info.obs {
                ui.add_space(theme::SP_S);
                let color = if ok { theme::OK } else { theme::DANGER };
                let r = ui
                    .add(
                        egui::Button::new(egui::RichText::new("OBS").color(color).strong())
                            .frame(false),
                    )
                    .on_hover_text(if ok {
                        t("OBS verbunden")
                    } else {
                        t("OBS nicht verbunden")
                    });
                if r.clicked() {
                    actions.push(MenuAction::Obs);
                }
            }
            if let Some((msg, err)) = info.message {
                ui.separator();
                let color = if err { theme::DANGER } else { theme::text() };
                ui.label(egui::RichText::new(msg).color(color));
            }
        });
    });
    actions
}

/// Rot blinkendes Ausrufezeichen: Audiodateien fehlen; Klick öffnet die Liste.
fn missing_alert(ui: &mut egui::Ui, n: usize, actions: &mut Vec<MenuAction>) {
    ui.add_space(theme::SP_S);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), Sense::click());
    let t_s = ui.input(|i| i.time);
    let on = (t_s * 2.0).fract() < 0.6;
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(100));
    crate::ui::widgets::icons::paint_alert(
        ui.painter(),
        rect.shrink(3.0),
        if on || resp.hovered() {
            theme::DANGER
        } else {
            theme::DANGER.gamma_multiply(0.35)
        },
    );
    let tip = if n == 1 {
        t("1 Audiodatei fehlt").to_string()
    } else {
        crate::tr!("{n} Audiodateien fehlen", n = n)
    };
    let resp = resp.on_hover_text(tip);
    if resp.clicked() {
        actions.push(MenuAction::MissingAudio);
    }
}

/// Ton-Master: Lautsprecher (stumm), Schieber, Prozent. Nur die Wiedergabe
/// von DMXCraft, nicht die Systemlautstärke.
fn audio_control(ui: &mut egui::Ui, a: AudioInfo, actions: &mut Vec<MenuAction>) {
    use dmxcraft_core::mapping::Control;
    // Rechts nach links: Prozent, Schieber, Symbol
    let pct = if a.mute {
        t("stumm").to_string()
    } else {
        format!("{:.0} %", a.master * a.limit * 100.0)
    };
    let color = if a.mute {
        theme::DANGER
    } else if a.master < 0.995 {
        theme::WARN
    } else {
        theme::text_dim()
    };
    let (prect, _) = ui.allocate_exact_size(egui::vec2(40.0, 18.0), Sense::hover());
    ui.painter().text(
        prect.left_center(),
        egui::Align2::LEFT_CENTER,
        pct,
        theme::heading_font(),
        color,
    );

    // Schieber
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(90.0, 18.0), Sense::click_and_drag());
    let overlay = crate::mapping::target(ui, rect, Control::AudioMaster);
    let track = egui::Rect::from_center_size(rect.center(), egui::vec2(rect.width() - 10.0, 4.0));
    if !overlay {
        if let Some(pos) = resp
            .interact_pointer_pos()
            .filter(|_| resp.dragged() || resp.clicked())
        {
            let v = ((pos.x - track.left()) / track.width()).clamp(0.0, 1.0);
            actions.push(MenuAction::AudioMaster(v));
        }
        if resp.double_clicked() {
            actions.push(MenuAction::AudioMaster(1.0));
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                actions.push(MenuAction::AudioMaster(
                    (a.master + scroll / 600.0).clamp(0.0, 1.0),
                ));
            }
        }
    }
    let p = ui.painter();
    p.rect_filled(track, 2.0, theme::raised());
    let fill_color = if a.mute {
        theme::text_dim()
    } else if a.fading {
        theme::WARN
    } else {
        theme::accent()
    };
    let x = track.left() + track.width() * a.master.clamp(0.0, 1.0);
    p.rect_filled(
        egui::Rect::from_min_max(track.min, egui::pos2(x, track.max.y)),
        2.0,
        fill_color,
    );
    p.circle_filled(
        egui::pos2(x, track.center().y),
        if resp.hovered() || resp.dragged() {
            6.0
        } else {
            5.0
        },
        theme::text(),
    );
    let mut tip = t("Ton-Master – nur die Wiedergabe von DMXCraft, nicht die Systemlautstärke.\nDoppelklick: 100 %, Mausrad: fein einstellen").to_string();
    if a.limit < 0.995 {
        tip.push_str("\n\n");
        tip.push_str(&tr!(
            "Höchstlautstärke {p} % (Einstellungen > Ton)",
            p = format!("{:.0}", a.limit * 100.0)
        ));
    }
    if a.pickup {
        tip.push_str("\n\n");
        tip.push_str(t(
            "Zurückgesetzt: ein MIDI-Fader wirkt erst wieder, wenn er den aktuellen Wert erreicht.",
        ));
    }
    resp.on_hover_text(tip);

    // Lautsprecher: Klick schaltet stumm, Rechtsklick: Ausfaden / 100 %
    let icon = if a.mute {
        Icon::SpeakerMute
    } else if a.master <= 0.0 {
        Icon::SpeakerNone
    } else if a.master < 0.5 {
        Icon::SpeakerLow
    } else {
        Icon::SpeakerHigh
    };
    let mut b = IconButton::new(icon).size(28.0).flat();
    if a.mute || a.master < 0.995 {
        b = b.active(true).tint(color);
    }
    let r = ui.add(b);
    let overlay = crate::mapping::target(ui, r.rect, Control::AudioMute);
    let r = r.on_hover_text(t("Ton stumm schalten (Rechtsklick: Ausfaden)"));
    if !overlay && r.clicked() {
        actions.push(MenuAction::AudioMute);
    }
    menu::context(&r, |ui| {
        menu::begin(ui);
        if menu::item(
            ui,
            MenuItem::new(&tr!(
                "Ton ausfaden ({s} s)",
                s = if a.fade_ms % 1000 == 0 {
                    (a.fade_ms / 1000).to_string()
                } else {
                    format!("{:.1}", a.fade_ms as f32 / 1000.0)
                }
            ))
            .enabled(!a.fading && a.master > 0.0),
        ) {
            actions.push(MenuAction::AudioFade);
        }
        if menu::item(ui, MenuItem::new(t("Auf 100 %"))) {
            actions.push(MenuAction::AudioMaster(1.0));
        }
    });
}

/// Auslastungsanzeige: Statuspunkt des Ausgangs und Balken.
fn load_meter(ui: &mut egui::Ui, info: &TitleInfo<'_>) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(96.0, 18.0), Sense::hover());
    let p = ui.painter();
    p.circle_filled(
        egui::pos2(rect.left() + 5.0, rect.center().y),
        4.0,
        info.status.1,
    );
    p.text(
        egui::pos2(rect.left() + 14.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        t("LAST"),
        theme::heading_font(),
        theme::text_dim(),
    );
    let bar = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 52.0, rect.center().y - 4.0),
        egui::pos2(rect.right(), rect.center().y + 4.0),
    );
    p.rect_filled(bar, 2.0, theme::raised());
    let l = info.load.clamp(0.0, 1.0);
    let color = if info.load > 0.85 {
        theme::DANGER
    } else if info.load > 0.6 {
        theme::WARN
    } else {
        theme::OK
    };
    if l > 0.0 {
        let fill = egui::Rect::from_min_size(
            bar.min,
            egui::vec2((bar.width() * l).max(2.0), bar.height()),
        );
        p.rect_filled(fill, 2.0, color);
    }
    resp.on_hover_text(tr!(
        "{status}\n{output} · {fps} fps · Auslastung {load} %",
        status = info.status.0,
        output = t(info.output.label()),
        fps = format!("{:.0}", info.fps),
        load = format!("{:.0}", info.load * 100.0),
    ));
}

fn main_menu(ui: &mut egui::Ui, info: &TitleInfo<'_>, actions: &mut Vec<MenuAction>) {
    let free = !info.locked;
    let resp = ui
        .add(IconButton::new(Icon::Menu).size(28.0).flat())
        .on_hover_text(t("Menü"));
    menu::dropdown(&resp, |ui| {
        menu::begin(ui);
        let soon = t("Noch nicht verfügbar");
        let mut act = |ui: &mut egui::Ui, item: MenuItem<'_>, a: MenuAction| {
            if menu::item(ui, item) {
                actions.push(a);
            }
        };
        menu::sub(ui, MenuItem::new(t("Datei")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Neu"))
                    .icon(Icon::FileNew)
                    .shortcut(&keys::new())
                    .enabled(free),
                MenuAction::NewShow,
            );
            act(
                ui,
                MenuItem::new(t("Öffnen …"))
                    .icon(Icon::FileOpen)
                    .shortcut(&keys::open())
                    .enabled(free),
                MenuAction::OpenShow,
            );
            menu::sub(
                ui,
                MenuItem::new(t("Zuletzt geöffnet"))
                    .icon(Icon::Recent)
                    .enabled(free && !info.recent.is_empty()),
                |ui| {
                    for path in info.recent {
                        let name = std::path::Path::new(path)
                            .file_stem()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| path.clone());
                        act(
                            ui,
                            MenuItem::new(&name).tooltip(path),
                            MenuAction::OpenRecent(path.clone()),
                        );
                    }
                },
            );
            menu::separator(ui);
            act(
                ui,
                MenuItem::new(t("Speichern"))
                    .icon(Icon::Save)
                    .shortcut(&keys::save()),
                MenuAction::SaveShow,
            );
            act(
                ui,
                MenuItem::new(t("Speichern unter …"))
                    .icon(Icon::SaveAs)
                    .shortcut(&keys::save_as())
                    .enabled(free),
                MenuAction::SaveShowAs,
            );
            menu::separator(ui);
            ui.add(
                MenuItem::new(t("Importieren von Daslight …"))
                    .icon(Icon::Import)
                    .enabled(false)
                    .tooltip(soon),
            );
        });
        menu::sub(ui, MenuItem::new(t("Bearbeiten")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Rückgängig"))
                    .icon(Icon::Undo)
                    .shortcut(&keys::undo())
                    .enabled(free && info.can_undo),
                MenuAction::Undo,
            );
            act(
                ui,
                MenuItem::new(t("Wiederherstellen"))
                    .icon(Icon::Redo)
                    .shortcut(&keys::redo())
                    .enabled(free && info.can_redo),
                MenuAction::Redo,
            );
            menu::separator(ui);
            let (lock_label, lock_icon) = if info.locked {
                (t("Entsperren …"), Icon::Unlocked)
            } else {
                (t("Sperren"), Icon::Locked)
            };
            act(
                ui,
                MenuItem::new(lock_label)
                    .icon(lock_icon)
                    .shortcut(&keys::lock()),
                MenuAction::ToggleLock,
            );
            act(
                ui,
                MenuItem::new(t("Einstellungen …"))
                    .icon(Icon::Settings)
                    .shortcut(&keys::settings())
                    .enabled(free),
                MenuAction::Settings,
            );
        });
        menu::sub(ui, MenuItem::new(t("Ansicht")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Vollbild"))
                    .icon(Icon::Fullscreen)
                    .shortcut("F11"),
                MenuAction::ToggleFullscreen,
            );
            menu::separator(ui);
            act(
                ui,
                MenuItem::new(t("Größer"))
                    .icon(Icon::ZoomIn)
                    .shortcut(&keys::zoom_in()),
                MenuAction::Zoom(0.1),
            );
            act(
                ui,
                MenuItem::new(t("Kleiner"))
                    .icon(Icon::ZoomOut)
                    .shortcut(&keys::zoom_out()),
                MenuAction::Zoom(-0.1),
            );
            act(
                ui,
                MenuItem::new(t("Größe wiederherstellen"))
                    .icon(Icon::ZoomReset)
                    .shortcut(&keys::zoom_reset()),
                MenuAction::Zoom(0.0),
            );
        });
        menu::sub(ui, MenuItem::new(t("Tools")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("3D-Visualisierer")).icon(Icon::Cube),
                MenuAction::Visualizer,
            );
            act(
                ui,
                MenuItem::new(t("DMX-Werte"))
                    .icon(Icon::Levels)
                    .shortcut(&keys::levels())
                    .enabled(free),
                MenuAction::DmxLevels,
            );
            act(
                ui,
                MenuItem::new(t("Profile Builder …"))
                    .icon(Icon::Builder)
                    .enabled(free),
                MenuAction::ProfileBuilder,
            );
        });
        menu::sub(ui, MenuItem::new(t("Mappings")).plain(), |ui| {
            use dmxcraft_core::mapping::MapKind;
            for (label, icon, kind) in [
                (t("Tastatur-Mapping"), Icon::Keyboard, MapKind::Keyboard),
                (t("MIDI-Mapping"), Icon::Midi, MapKind::Midi),
            ] {
                act(
                    ui,
                    MenuItem::new(label).icon(icon).enabled(free),
                    MenuAction::Mapping(kind),
                );
            }
            ui.add(
                MenuItem::new(t("Touch-Mapping"))
                    .icon(Icon::Touch)
                    .enabled(false)
                    .tooltip(soon),
            );
        });
        menu::sub(ui, MenuItem::new(t("Hilfe")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Nach Updates suchen")).icon(Icon::Update),
                MenuAction::CheckUpdates,
            );
            act(
                ui,
                MenuItem::new(t("Über DMXCraft")).icon(Icon::Info),
                MenuAction::About,
            );
        });
    });
}

/// Beschriftung der Tastenkürzel (Strg unter Windows/Linux, Cmd unter macOS).
pub mod keys {
    use crate::i18n::t;

    fn modifier() -> &'static str {
        if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            t("Strg")
        }
    }

    fn k(rest: &str) -> String {
        format!("{}+{rest}", modifier())
    }

    pub fn new() -> String {
        k("N")
    }
    pub fn open() -> String {
        k("O")
    }
    pub fn save() -> String {
        k("S")
    }
    pub fn save_as() -> String {
        format!("{}+{}+S", modifier(), t("Umschalt"))
    }
    /// Zwischenablage-Kürzel: „Strg+X“ usw.
    pub fn clip(key: &str) -> String {
        k(key)
    }
    pub fn undo() -> String {
        k("Z")
    }
    pub fn redo() -> String {
        k("Y")
    }
    pub fn lock() -> String {
        k("L")
    }
    pub fn settings() -> String {
        k(",")
    }
    pub fn zoom_in() -> String {
        k("+")
    }
    pub fn zoom_out() -> String {
        k("-")
    }
    pub fn zoom_reset() -> String {
        k("0")
    }
    pub fn levels() -> String {
        k("D")
    }
    pub fn delete() -> &'static str {
        t("Entf")
    }
}

/// Fenstergröße an den Rändern ändern (randloses Fenster).
pub fn resize_handles(ctx: &egui::Context, fullscreen: bool) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if maximized || fullscreen {
        return;
    }
    let rect = ctx.content_rect();
    let Some(p) = ctx.input(|i| i.pointer.hover_pos()) else {
        return;
    };
    const EDGE: f32 = 5.0;
    let (l, r, t, b) = (
        p.x - rect.left() < EDGE,
        rect.right() - p.x < EDGE,
        p.y - rect.top() < EDGE,
        rect.bottom() - p.y < EDGE,
    );
    use egui::ResizeDirection as D;
    let dir = match (l, r, t, b) {
        (true, _, true, _) => D::NorthWest,
        (_, true, true, _) => D::NorthEast,
        (true, _, _, true) => D::SouthWest,
        (_, true, _, true) => D::SouthEast,
        (true, _, _, _) => D::West,
        (_, true, _, _) => D::East,
        (_, _, true, _) => D::North,
        (_, _, _, true) => D::South,
        _ => return,
    };
    let icon = match dir {
        D::North | D::South => egui::CursorIcon::ResizeVertical,
        D::East | D::West => egui::CursorIcon::ResizeHorizontal,
        D::NorthWest | D::SouthEast => egui::CursorIcon::ResizeNwSe,
        D::NorthEast | D::SouthWest => egui::CursorIcon::ResizeNeSw,
    };
    ctx.set_cursor_icon(icon);
    if ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}
