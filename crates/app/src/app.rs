//! App-Rahmen: Titelleiste, Layout der Ansichten, Dialoge, Ausgabe-Runtime.

use crate::i18n::t;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use dmxcraft_core::{Engine, SceneId, Show};
use dmxcraft_output::{OutputKind, OutputStatus, Runtime};
use eframe::egui;

use crate::demo;
use crate::file_dialog::{self, Purpose};
use crate::fullscreen::Fullscreen;
use crate::history::{History, Snapshot};
use crate::settings::Settings;
use crate::settings_window::{SettingsCx, SettingsWindow};
use crate::state::{AppState, Mode, ProgrammerMode};
use crate::titlebar::{self, MenuAction, TitleInfo, UpdateBadge};
use crate::ui::theme;
use crate::ui::widgets::{form, text_field, Dialog};
use crate::updater::{UpdateState, Updater};
use crate::views::control::{ProgrammerView, SceneGridView, ScenePropsView, StageView};
use crate::views::setup::SetupView;
use crate::views::windows::Windows;
use crate::views::{View, ViewCtx};

/// DMX-Wiederholrate des Ausgabe-Threads.
pub const OUTPUT_HZ: u32 = 50;

/// Ansichten unter STEUERN.
#[derive(Default)]
struct ControlViews {
    grid: SceneGridView,
    stage: StageView,
    programmer: ProgrammerView,
    props: ScenePropsView,
}

/// Dialog zum Sperren (Passwort festlegen) oder Entsperren.
struct LockDialog {
    set_password: bool,
    pw: String,
    pw2: String,
    error: Option<String>,
}

pub struct DmxCraftApp {
    engine: Arc<Mutex<Engine>>,
    runtime: Runtime,
    output_kind: OutputKind,
    updater: Updater,
    started: bool,
    logo: Option<egui::TextureHandle>,
    settings: Settings,
    settings_open: bool,
    settings_window: SettingsWindow,
    about_open: bool,
    /// Offener Systemdialog (Öffnen / Speichern unter).
    file_dialog: Option<file_dialog::Pending>,
    lock_dialog: Option<LockDialog>,
    /// Hash des Sperr-Passworts – nur im Speicher, nach dem Entsperren vergessen.
    lock_hash: Option<String>,
    state: AppState,
    control: ControlViews,
    setup: SetupView,
    windows: Windows,
    history: History,
    fullscreen: Fullscreen,
    /// Vollbild beim nächsten Frame umschalten.
    toggle_fullscreen: bool,
    /// Zuletzt erkannte USB-DMX-Kabel (für die Einstellungen).
    usb_ports: Vec<dmxcraft_output::UsbDmxPort>,
    /// Stand der Show beim letzten Laden/Speichern (für „ungespeichert“).
    saved_show: Show,
    /// Frage „Vor dem Schließen speichern?“ offen.
    close_dialog: bool,
    /// Nach erfolgreichem Speichern schließen.
    close_after_save: bool,
    /// Schließen ohne weitere Nachfrage zulassen.
    allow_close: bool,
    /// Im nächsten Frame schließen.
    request_close: bool,
}

impl DmxCraftApp {
    pub fn new(show_path: Option<String>, settings: Settings) -> Self {
        let mut message = None;
        let show = match &show_path {
            Some(p) => match Show::load(p) {
                Ok(mut s) => {
                    demo::ensure_builtin_types(&mut s);
                    message = Some((tr!("Show „{v}“ geladen", v = s.name), false));
                    s
                }
                Err(e) => {
                    message = Some((format!("{p}: {e} – Demo-Show geladen"), true));
                    demo::demo_show()
                }
            },
            None => demo::demo_show(),
        };
        let saved_show = show.clone();
        let engine = Arc::new(Mutex::new(Engine::new(show)));
        let output_kind = settings.output_kind();
        let runtime = Runtime::start(
            engine.clone(),
            output_kind.create(&settings.output_config()),
            OUTPUT_HZ,
        );
        let mut state = AppState::new(show_path.unwrap_or_default());
        state.message = message;
        Self {
            engine,
            runtime,
            output_kind,
            updater: Updater::default(),
            started: false,
            logo: None,
            settings,
            settings_open: false,
            settings_window: SettingsWindow::default(),
            about_open: false,
            file_dialog: None,
            lock_dialog: None,
            lock_hash: None,
            state,
            control: ControlViews::default(),
            setup: SetupView::default(),
            windows: Windows::default(),
            history: History::default(),
            usb_ports: Vec::new(),
            fullscreen: Fullscreen::default(),
            toggle_fullscreen: false,
            saved_show,
            close_dialog: false,
            close_after_save: false,
            allow_close: false,
            request_close: false,
        }
    }

    /// Angezeigter Name: der Dateiname der Show (ohne Endung), sonst der
    /// Name der ungespeicherten Show.
    fn display_name(&self, e: &Engine) -> String {
        std::path::Path::new(&self.state.show_path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| e.show().name.clone())
    }

    /// Ungespeicherte Änderungen an der Show?
    fn dirty(&self) -> bool {
        *Self::lock(&self.engine).show() != self.saved_show
    }

    /// Aktuellen Stand als gespeichert merken.
    fn mark_saved(&mut self) {
        self.saved_show = Self::lock(&self.engine).show().clone();
    }

    fn lock(engine: &Arc<Mutex<Engine>>) -> MutexGuard<'_, Engine> {
        // Ein vergifteter Mutex heißt: der Ausgabe-Thread ist abgestürzt.
        // Die Daten sind trotzdem konsistent genug, um weiterzuarbeiten.
        engine.lock().unwrap_or_else(|p| p.into_inner())
    }

    /// Ausgang mit den aktuellen Einstellungen neu starten (z. B. nach
    /// geänderter Kabelzuordnung).
    fn restart_output(&mut self) {
        self.output_kind = self.settings.output_kind();
        self.settings.output = self.output_kind.key().into();
        self.runtime.stop();
        self.runtime = Runtime::start(
            self.engine.clone(),
            self.output_kind.create(&self.settings.output_config()),
            OUTPUT_HZ,
        );
    }

    fn save_settings(&mut self) {
        if let Err(e) = self.settings.save() {
            self.state
                .error(tr!("Einstellungen nicht gespeichert: {e}", e = e));
        }
    }

    fn open_show(&mut self, path: &str) {
        match Show::load(path) {
            Ok(mut s) => {
                demo::ensure_builtin_types(&mut s);
                let name = s.name.clone();
                Self::lock(&self.engine).replace_show(s);
                self.history.reset(&Snapshot::of(&Self::lock(&self.engine)));
                self.mark_saved();
                self.state.reset_for_new_show();
                self.state.show_path = path.to_string();
                self.state.info(tr!("Show „{name}“ geladen", name = name));
                self.settings.remember(path);
                self.save_settings();
            }
            Err(err) => self.state.error(format!("{path}: {err}")),
        }
    }

    fn save_show(&mut self, path: &str) {
        // Name der Show = Dateiname
        if let Some(stem) = std::path::Path::new(path).file_stem() {
            Self::lock(&self.engine).show_mut().name = stem.to_string_lossy().into_owned();
        }
        let result = Self::lock(&self.engine).show().save(path);
        match result {
            Ok(()) => {
                self.mark_saved();
                self.state.show_path = path.to_string();
                self.state.info(tr!("Gespeichert: {path}", path = path));
                self.settings.remember(path);
                self.save_settings();
                if std::mem::take(&mut self.close_after_save) {
                    self.allow_close = true;
                    self.request_close = true;
                }
            }
            Err(err) => {
                self.close_after_save = false;
                self.state.error(err.to_string());
            }
        }
    }

    fn start_file_dialog(&mut self, ctx: &egui::Context, purpose: Purpose) {
        // Nur ein Dialog zur Zeit
        if self.file_dialog.is_none() {
            self.file_dialog = Some(file_dialog::Pending::start(
                purpose,
                &self.state.show_path,
                ctx,
            ));
        }
    }

    fn apply_menu(&mut self, ctx: &egui::Context, action: MenuAction) {
        match action {
            MenuAction::NewShow => {
                Self::lock(&self.engine).replace_show(demo::empty_show());
                self.history.reset(&Snapshot::of(&Self::lock(&self.engine)));
                self.mark_saved();
                self.state.reset_for_new_show();
                // neue Datei: erstes Speichern fragt nach dem Ort
                self.state.show_path.clear();
                self.state.info(t("Neue, leere Show"));
            }
            MenuAction::OpenShow => self.start_file_dialog(ctx, Purpose::OpenShow),
            MenuAction::SaveShow => {
                let p = self.state.show_path.clone();
                if p.trim().is_empty() {
                    self.start_file_dialog(ctx, Purpose::SaveShowAs);
                } else {
                    self.save_show(&p);
                }
            }
            MenuAction::SaveShowAs => self.start_file_dialog(ctx, Purpose::SaveShowAs),
            MenuAction::OpenRecent(path) => {
                if std::path::Path::new(&path).exists() {
                    self.open_show(&path);
                } else {
                    self.settings.recent.retain(|p| *p != path);
                    self.save_settings();
                    self.state.error(tr!("Nicht gefunden: {path}", path = path));
                }
            }
            MenuAction::Undo | MenuAction::Redo => {
                let mut e = Self::lock(&self.engine);
                let current = Snapshot::of(&e);
                let restored = if action == MenuAction::Undo {
                    self.history.undo(&current)
                } else {
                    self.history.redo(&current)
                };
                match restored {
                    Some(snap) => {
                        snap.restore(&mut e);
                        // Edit-Werte aus der wiederhergestellten Szene neu laden
                        self.state.edit_reload = true;
                        self.state
                            .selected
                            .retain(|id| e.show().fixture(*id).is_some());
                    }
                    None => {
                        drop(e);
                        self.state.info(t("Nichts rückgängig zu machen"));
                    }
                }
            }
            MenuAction::DmxLevels => {
                self.state.mode = Mode::Setup;
                self.state.show_dmx_levels = true;
            }
            MenuAction::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            MenuAction::ToggleFullscreen => self.toggle_fullscreen = true,
            MenuAction::Zoom(d) => {
                let z = if d == 0.0 {
                    1.0
                } else {
                    (ctx.zoom_factor() + d).clamp(0.6, 2.0)
                };
                ctx.set_zoom_factor(z);
                self.settings.ui_scale = z;
                self.save_settings();
            }
            MenuAction::ToggleBlackout => {
                let mut e = Self::lock(&self.engine);
                let bo = e.blackout();
                e.set_blackout(!bo);
            }
            MenuAction::Settings => {
                self.usb_ports = dmxcraft_output::list_ports();
                self.settings_open = true;
            }
            MenuAction::CheckUpdates => self.updater.check(ctx.clone()),
            MenuAction::InstallUpdate => self.updater.install(ctx.clone()),
            MenuAction::About => self.about_open = true,
            MenuAction::ToggleLock => {
                if self.state.locked {
                    self.lock_dialog = Some(LockDialog {
                        set_password: false,
                        pw: String::new(),
                        pw2: String::new(),
                        error: None,
                    });
                } else {
                    self.lock_dialog = Some(LockDialog {
                        set_password: true,
                        pw: String::new(),
                        pw2: String::new(),
                        error: None,
                    });
                }
            }
        }
    }

    fn lock_ui(&mut self) {
        self.state.locked = true;
        self.state.mode = Mode::Control;
        self.state.zones_open = false;
        self.settings_open = false;
        self.state.info(t("Oberfläche gesperrt"));
    }

    fn unlock_ui(&mut self) {
        self.state.locked = false;
        // Das Passwort gilt nur für eine Sperrung – beim nächsten Mal neu festlegen.
        self.lock_hash = None;
        self.state.info(t("Oberfläche entsperrt"));
    }

    fn lock_window(&mut self, ctx: &egui::Context) {
        let Some(mut d) = self.lock_dialog.take() else {
            return;
        };
        let mut open = true;
        let title = if d.set_password {
            t("Oberfläche sperren")
        } else {
            t("Oberfläche entsperren")
        };
        let action = if d.set_password {
            t("Sperren")
        } else {
            t("Entsperren")
        };
        let res = Dialog::new(title)
            .id("lock")
            .width(340.0)
            .cancel(t("Abbrechen"))
            .primary(action)
            .show(ctx, &mut open, |ui, first| {
                form(ui, "lock_form", |f| {
                    let r = f.row(t("Passwort"), |ui| {
                        ui.add(
                            text_field(&mut d.pw)
                                .password(true)
                                .desired_width(f32::INFINITY),
                        )
                    });
                    if first {
                        r.request_focus();
                    }
                    if d.set_password {
                        f.row(t("Wiederholen"), |ui| {
                            ui.add(
                                text_field(&mut d.pw2)
                                    .password(true)
                                    .desired_width(f32::INFINITY),
                            )
                        });
                    }
                    if let Some(e) = &d.error {
                        f.note(|ui| {
                            ui.label(egui::RichText::new(e).color(theme::DANGER));
                        });
                    }
                });
            });
        let confirm = res.clicked(1);
        if confirm {
            if d.set_password {
                if d.pw.chars().count() < 4 {
                    d.error = Some(t("Mindestens 4 Zeichen").into());
                } else if d.pw != d.pw2 {
                    d.error = Some(t("Passwörter stimmen nicht überein").into());
                } else {
                    self.lock_hash = Some(crate::lock::hash_password(&d.pw));
                    self.lock_ui();
                    return;
                }
            } else if self
                .lock_hash
                .as_deref()
                .is_some_and(|h| crate::lock::verify(h, &d.pw))
            {
                self.unlock_ui();
                return;
            } else {
                d.error = Some(t("Falsches Passwort").into());
                d.pw.clear();
            }
        }
        if open {
            self.lock_dialog = Some(d);
        }
    }

    fn shortcuts(&mut self, ctx: &egui::Context) -> Vec<MenuAction> {
        use egui::{Key, Modifiers};
        let typing = ctx.memory(|m| m.focused().is_some());
        let free = !self.state.locked;
        let cmd = Modifiers::COMMAND;
        let cmd_shift = Modifiers::COMMAND | Modifiers::SHIFT;
        ctx.input_mut(|i| {
            let mut out = Vec::new();
            let mut key =
                |i: &mut egui::InputState, m: Modifiers, k: Key, ok: bool, a: MenuAction| {
                    if ok && i.consume_key(m, k) {
                        out.push(a);
                    }
                };
            // Umschalt-Varianten zuerst, sonst greift Strg+S
            key(i, cmd_shift, Key::S, free, MenuAction::SaveShowAs);
            key(i, cmd_shift, Key::Z, free && !typing, MenuAction::Redo);
            key(i, cmd, Key::N, free, MenuAction::NewShow);
            key(i, cmd, Key::O, free, MenuAction::OpenShow);
            key(i, cmd, Key::S, true, MenuAction::SaveShow);
            key(i, cmd, Key::Z, free && !typing, MenuAction::Undo);
            key(i, cmd, Key::Y, free && !typing, MenuAction::Redo);
            key(i, cmd, Key::L, true, MenuAction::ToggleLock);
            key(i, cmd, Key::Comma, free, MenuAction::Settings);
            key(i, cmd, Key::D, free, MenuAction::DmxLevels);
            key(i, cmd, Key::Plus, true, MenuAction::Zoom(0.1));
            key(i, cmd, Key::Equals, true, MenuAction::Zoom(0.1));
            key(i, cmd, Key::Minus, true, MenuAction::Zoom(-0.1));
            key(i, cmd, Key::Num0, true, MenuAction::Zoom(0.0));
            key(
                i,
                Modifiers::NONE,
                Key::F11,
                true,
                MenuAction::ToggleFullscreen,
            );
            key(
                i,
                Modifiers::NONE,
                Key::B,
                !typing,
                MenuAction::ToggleBlackout,
            );
            out
        })
    }

    fn title_info(&self) -> (String, f32, (String, egui::Color32), bool) {
        let e = Self::lock(&self.engine);
        let (fps, status) = match self.runtime.status() {
            OutputStatus::Running {
                fps, note: None, ..
            } => (fps, (t("Ausgang aktiv").to_string(), theme::OK)),
            OutputStatus::Running {
                fps, note: Some(n), ..
            } => (fps, (tr!("Ausgang aktiv, aber: {n}", n = n), theme::WARN)),
            OutputStatus::Connecting => (0.0, (t("Ausgang verbindet").into(), theme::WARN)),
            OutputStatus::Error(msg) => (
                0.0,
                (tr!("Ausgang: Fehler – {msg}", msg = msg), theme::DANGER),
            ),
            OutputStatus::Stopped => (0.0, (t("Ausgang gestoppt").into(), theme::text_dim())),
        };
        (self.display_name(&e), fps, status, e.blackout())
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        // Ergebnis eines Systemdialogs
        if let Some((purpose, result)) = self.file_dialog.as_ref().and_then(|d| d.poll()) {
            self.file_dialog = None;
            match result {
                Some(path) => {
                    let path = path.to_string_lossy().into_owned();
                    match purpose {
                        Purpose::OpenShow => self.open_show(&path),
                        Purpose::SaveShowAs => self.save_show(&path),
                    }
                }
                // Speichern abgebrochen: nicht schließen
                None => self.close_after_save = false,
            }
        }

        // Vor dem Schließen: ungespeicherte Änderungen?
        if self.close_dialog {
            let mut open = true;
            let name = {
                let e = Self::lock(&self.engine);
                self.display_name(&e)
            };
            let res = Dialog::new(t("Ungespeicherte Änderungen"))
                .id("close_unsaved")
                .width(380.0)
                .cancel(t("Abbrechen"))
                .secondary(t("Nicht speichern"))
                .primary(t("Speichern"))
                .show(ctx, &mut open, |ui, _| {
                    ui.label(tr!(
                        "Möchtest du die Änderungen an „{name}“ vor dem Schließen speichern?",
                        name = name
                    ));
                });
            if res.clicked(1) {
                open = false;
                self.allow_close = true;
                self.request_close = true;
            } else if res.clicked(2) {
                open = false;
                self.close_after_save = true;
                let p = self.state.show_path.clone();
                if p.trim().is_empty() {
                    self.start_file_dialog(ctx, Purpose::SaveShowAs);
                } else {
                    self.save_show(&p);
                }
            }
            self.close_dialog = open;
        }

        // Einstellungen
        if self.settings_open {
            let mut open = true;
            let mut e = Self::lock(&self.engine);
            let ch = self.settings_window.show(
                ctx,
                &mut open,
                SettingsCx {
                    settings: &mut self.settings,
                    engine: &mut e,
                    usb_ports: &mut self.usb_ports,
                    show_path: &self.state.show_path,
                },
            );
            drop(e);
            if let Some(z) = ch.zoom {
                ctx.set_zoom_factor(z);
            }
            if ch.theme {
                theme::apply(ctx);
            }
            if ch.restart_output {
                self.restart_output();
            }
            if ch.save || !open {
                self.save_settings();
            }
            self.settings_open = open;
        }

        // Über
        if self.about_open {
            let mut open = true;
            let logo = self.logo.clone();
            Dialog::new(t("Über DMXCraft"))
                .id("about")
                .width(340.0)
                .cancel(t("Schließen"))
                .show(ctx, &mut open, |ui, _| {
                    ui.vertical_centered(|ui| {
                        if let Some(tex) = &logo {
                            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                                tex.id(),
                                [72.0, 72.0],
                            )));
                            ui.add_space(theme::SP_S);
                        }
                        ui.label(
                            egui::RichText::new(format!("DMXCraft {}", Updater::current_version()))
                                .font(theme::heading_font())
                                .size(theme::FS_TITLE),
                        );
                        ui.add_space(theme::SP_XS);
                        ui.label(
                            egui::RichText::new(t(
                                "DMX-Lichtsteuerung für Linux, Windows und macOS.",
                            ))
                            .color(theme::text_dim()),
                        );
                        ui.add_space(theme::SP_S);
                        ui.label(egui::RichText::new(t("Lizenz: MIT")).color(theme::text_dim()));
                        ui.label(
                            egui::RichText::new(t(
                                "Gerätedaten: Open Fixture Library (MIT), QLC+ (Apache 2.0)",
                            ))
                            .color(theme::text_dim()),
                        );
                        ui.add_space(theme::SP_S);
                        ui.hyperlink_to(
                            "github.com/JustinSchweiger/DMXCraft",
                            "https://github.com/JustinSchweiger/DMXCraft",
                        );
                    });
                });
            self.about_open = open;
        }
    }
}

impl eframe::App for DmxCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        // Laufende Szenen und Live-Farben flüssig anzeigen.
        ctx.request_repaint_after(Duration::from_millis(33));
        if !self.started {
            self.started = true;
            theme::apply(&ctx);
            // Zoom-Tasten selbst behandeln, damit die Größe gespeichert wird.
            ctx.options_mut(|o| o.zoom_with_keyboard = false);
            ctx.set_zoom_factor(self.settings.ui_scale.clamp(0.6, 2.0));
            self.logo = crate::icon::texture(&ctx);
            if self.settings.check_updates {
                self.updater.check(ctx.clone());
            }
        }

        // ----- Schließen: bei ungespeicherten Änderungen erst nachfragen -----
        if std::mem::take(&mut self.request_close) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if ctx.input(|i| i.viewport().close_requested()) && !self.allow_close && self.dirty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.close_dialog = true;
        }

        // ----- Titelleiste -----
        let (mut show_name, fps, status, blackout) = self.title_info();
        if self.dirty() {
            show_name.push_str(" *");
        }
        let update = match self.updater.state() {
            UpdateState::Available => UpdateBadge::Available,
            UpdateState::Installing => UpdateBadge::Busy(t("installiere Update")),
            UpdateState::Installed(v) => UpdateBadge::Installed(v),
            UpdateState::Error(e) => UpdateBadge::Error(e),
            _ => UpdateBadge::None,
        };
        let message = self
            .state
            .current_message()
            .map(|(m, e)| (m.to_string(), e));
        let info = TitleInfo {
            show_name: &show_name,
            status,
            load: self.runtime.load(),
            fps,
            locked: self.state.locked,
            output: self.output_kind,
            blackout,
            recent: &self.settings.recent,
            can_undo: self.history.can_undo(),
            can_redo: self.history.can_redo(),
            message: message.as_ref().map(|(m, e)| (m.as_str(), *e)),
            update,
        };
        let mut actions = Vec::new();
        egui::Panel::top("titlebar")
            .exact_size(titlebar::HEIGHT)
            .frame(egui::Frame::new().fill(theme::bg()))
            .show(ui, |ui| {
                actions = titlebar::show(ui, &info, &mut self.state.mode);
            });
        actions.extend(self.shortcuts(&ctx));
        for a in actions {
            self.apply_menu(&ctx, a);
        }
        if std::mem::take(&mut self.toggle_fullscreen) {
            self.fullscreen.toggle(&ctx);
        }
        self.fullscreen.update(&ctx);

        // ----- Inhalt (randlos, Bereiche nur durch Trennlinien getrennt) -----
        // Blind: Bühne und Fader zeigen die Bearbeitung, nicht den Ausgang
        let frame = {
            let e = Self::lock(&self.engine);
            if e.is_blind() {
                e.render_monitor()
            } else {
                drop(e);
                self.runtime.last_frame()
            }
        };
        let engine = self.engine.clone();
        {
            let mut e = Self::lock(&engine);
            let mut cx = ViewCtx {
                engine: &mut e,
                state: &mut self.state,
                frame: &frame,
            };
            self.windows.show(&ctx, &mut cx);
            let area = egui::Frame::new()
                .fill(theme::surface())
                .inner_margin(egui::Margin::same(6));
            match cx.state.mode {
                Mode::Setup => {
                    egui::CentralPanel::default_margins()
                        .frame(egui::Frame::new().fill(theme::surface()))
                        .show(ui, |ui| self.setup.ui(ui, &mut cx));
                }
                Mode::Control => {
                    let control = &mut self.control;
                    egui::Panel::bottom("live_bottom")
                        .resizable(true)
                        .default_size(400.0)
                        .min_size(220.0)
                        .frame(egui::Frame::new().fill(theme::surface()))
                        .show(ui, |ui| {
                            egui::Panel::right("programmer_panel")
                                .resizable(true)
                                .default_size(640.0)
                                .min_size(320.0)
                                .frame(area)
                                .show(ui, |ui| control.programmer.ui(ui, &mut cx));
                            egui::CentralPanel::default_margins()
                                .frame(area)
                                .show(ui, |ui| control.stage.ui(ui, &mut cx));
                        });
                    // Seitenleiste mit den Einstellungen der ausgewählten Szene
                    egui::Panel::right("scene_props")
                        .resizable(true)
                        .default_size(330.0)
                        .min_size(280.0)
                        .frame(area)
                        .show(ui, |ui| control.props.ui(ui, &mut cx));
                    egui::CentralPanel::default_margins()
                        .frame(area)
                        .show(ui, |ui| control.grid.ui(ui, &mut cx));
                }
            }
            edit_preview(cx.engine, cx.state);
        }
        self.dialogs(&ctx);
        self.lock_window(&ctx);
        titlebar::resize_handles(&ctx, self.fullscreen.is_on());

        // Rückgängig: Änderungen erst übernehmen, wenn Ziehen/Tippen beendet ist.
        let busy = ctx.input(|i| i.pointer.any_down()) || ctx.memory(|m| m.focused().is_some());
        self.history.track(&Self::lock(&self.engine), busy);
    }
}

/// Edit-Modus: markierte Szenen werden ausgegeben (auch wenn sie nicht laufen)
/// und ihre Geräte beim Markieren ausgewählt. Bühne und DMX-Ausgang zeigen
/// damit die Szene, die man gerade bearbeitet.
///
/// Ist genau eine Szene markiert, ist sie die Bearbeitungsszene: beim
/// Markieren werden ihre Werte in den Programmer geladen, und jede Änderung
/// am Programmer wird sofort in ihrem (ersten) Schritt gespeichert.
fn edit_preview(e: &mut Engine, st: &mut AppState) {
    let edit = st.programmer_mode.is_edit();
    e.set_blind(st.programmer_mode == ProgrammerMode::Blind);
    let preview: Vec<SceneId> = if edit {
        st.selected_scenes.iter().copied().collect()
    } else {
        Vec::new()
    };
    if e.preview() != preview.as_slice() {
        e.set_preview(preview);
    }
    let target = match (edit, st.selected_scenes.len()) {
        (true, 1) => st.selected_scenes.iter().next().copied(),
        _ => None,
    };
    // Wechsel in den Edit-Modus lädt die markierte Szene wie ein neues Markieren
    if st.programmer_mode != st.programmer_mode_seen {
        let was_edit = st.programmer_mode_seen.is_edit();
        st.programmer_mode_seen = st.programmer_mode;
        if edit && !was_edit {
            st.scene_selection_seen.clear();
            st.scene_selection_seen.insert(SceneId(u32::MAX));
        } else if !edit && was_edit {
            // Edit-Werte liegen in der Szene; der Programmer gibt sie nicht weiter aus
            e.clear_programmer();
        }
    }
    if std::mem::take(&mut st.edit_reload) && edit {
        if let Some(id) = st.selected_scenes.iter().next().copied() {
            e.clear_programmer();
            if st.selected_scenes.len() == 1 {
                if let Some(step) = e.show().scene(id).and_then(|s| s.steps.first()).cloned() {
                    for v in step.values {
                        e.set_programmer(
                            dmxcraft_core::ChannelRef {
                                fixture: v.fixture,
                                channel: v.channel,
                            },
                            v.value,
                        );
                    }
                }
            }
        }
        return;
    }
    if st.selected_scenes != st.scene_selection_seen {
        st.scene_selection_seen = st.selected_scenes.clone();
        if edit {
            if !st.selected_scenes.is_empty() {
                let show = e.show();
                st.selected = st
                    .selected_scenes
                    .iter()
                    .filter_map(|id| show.scene(*id))
                    .flat_map(|s| s.steps.iter())
                    .flat_map(|step| step.values.iter().map(|v| v.fixture))
                    .filter(|f| show.fixture(*f).is_some())
                    .collect();
            }
            // Werte der neuen Bearbeitungsszene in den Programmer laden
            e.clear_programmer();
            if let Some(step) = target
                .and_then(|id| e.show().scene(id))
                .and_then(|s| s.steps.first())
                .cloned()
            {
                for v in step.values {
                    e.set_programmer(
                        dmxcraft_core::ChannelRef {
                            fixture: v.fixture,
                            channel: v.channel,
                        },
                        v.value,
                    );
                }
            }
            return;
        }
    }
    // Programmer-Änderungen in der Bearbeitungsszene speichern
    let Some(id) = target else {
        return;
    };
    let values = crate::views::control::programmer::programmer_values(e);
    let Some(scene) = e.show_mut().scene_mut(id) else {
        return;
    };
    if scene.steps.is_empty() {
        scene.steps.push(dmxcraft_core::Step::new(Vec::new()));
    }
    let step = &mut scene.steps[0];
    let mut current = step.values.clone();
    current.sort_by_key(|c| (c.fixture, c.channel));
    if current != values {
        step.values = values;
    }
}
