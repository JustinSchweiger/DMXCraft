//! App-Rahmen: Titelleiste, Layout der Ansichten, Dialoge, Ausgabe-Runtime.

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use dmxcraft_core::{Engine, Show};
use dmxcraft_output::{OutputKind, OutputStatus, Runtime};
use eframe::egui;

use crate::demo;
use crate::settings::Settings;
use crate::state::{AppState, Mode};
use crate::titlebar::{self, MenuAction, TitleInfo, UpdateBadge};
use crate::ui::theme;
use crate::ui::widgets::dialog::INPUT_H;
use crate::ui::widgets::{form, text_field, Dialog, Toggle};
use crate::updater::{UpdateState, Updater};
use crate::views::control::{ProgrammerView, SceneGridView, StageView};
use crate::views::setup::SetupView;
use crate::views::windows::Windows;
use crate::views::{View, ViewCtx};

/// DMX-Wiederholrate des Ausgabe-Threads.
pub const OUTPUT_HZ: u32 = 40;

/// Ansichten unter STEUERN.
#[derive(Default)]
struct ControlViews {
    grid: SceneGridView,
    stage: StageView,
    programmer: ProgrammerView,
}

/// Kleiner Dialog für Dateipfade (Öffnen / Speichern unter).
#[derive(Clone, Copy, PartialEq, Eq)]
enum PathDialog {
    Open,
    SaveAs,
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
    about_open: bool,
    path_dialog: Option<(PathDialog, String)>,
    lock_dialog: Option<LockDialog>,
    state: AppState,
    control: ControlViews,
    setup: SetupView,
    windows: Windows,
}

impl DmxCraftApp {
    pub fn new(show_path: Option<String>, settings: Settings) -> Self {
        let mut message = None;
        let show = match &show_path {
            Some(p) => match Show::load(p) {
                Ok(mut s) => {
                    demo::ensure_builtin_types(&mut s);
                    message = Some((format!("Show „{}“ geladen", s.name), false));
                    s
                }
                Err(e) => {
                    message = Some((format!("{p}: {e} – Demo-Show geladen"), true));
                    demo::demo_show()
                }
            },
            None => demo::demo_show(),
        };
        let engine = Arc::new(Mutex::new(Engine::new(show)));
        let output_kind = OutputKind::from_key(&settings.output);
        let runtime = Runtime::start(engine.clone(), output_kind.create(), OUTPUT_HZ);
        let mut state = AppState::new(show_path.unwrap_or_else(|| "show.json".into()));
        state.message = message;
        // Gesperrt bleibt gesperrt – auch nach einem Neustart
        state.locked = settings.locked && settings.lock_hash.is_some();
        Self {
            engine,
            runtime,
            output_kind,
            updater: Updater::default(),
            started: false,
            logo: None,
            settings,
            settings_open: false,
            about_open: false,
            path_dialog: None,
            lock_dialog: None,
            state,
            control: ControlViews::default(),
            setup: SetupView::default(),
            windows: Windows::default(),
        }
    }

    fn lock(engine: &Arc<Mutex<Engine>>) -> MutexGuard<'_, Engine> {
        // Ein vergifteter Mutex heißt: der Ausgabe-Thread ist abgestürzt.
        // Die Daten sind trotzdem konsistent genug, um weiterzuarbeiten.
        engine.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn switch_output(&mut self, kind: OutputKind) {
        if kind == self.output_kind {
            return;
        }
        self.runtime.stop();
        self.runtime = Runtime::start(self.engine.clone(), kind.create(), OUTPUT_HZ);
        self.output_kind = kind;
        self.settings.output = kind.key().into();
        self.save_settings();
    }

    fn save_settings(&mut self) {
        if let Err(e) = self.settings.save() {
            self.state
                .error(format!("Einstellungen nicht gespeichert: {e}"));
        }
    }

    fn open_show(&mut self, path: &str) {
        match Show::load(path) {
            Ok(mut s) => {
                demo::ensure_builtin_types(&mut s);
                let name = s.name.clone();
                Self::lock(&self.engine).replace_show(s);
                self.state.reset_for_new_show();
                self.state.show_path = path.to_string();
                self.state.info(format!("Show „{name}“ geladen"));
                self.settings.last_show = Some(path.to_string());
                self.save_settings();
            }
            Err(err) => self.state.error(format!("{path}: {err}")),
        }
    }

    fn save_show(&mut self, path: &str) {
        let result = Self::lock(&self.engine).show().save(path);
        match result {
            Ok(()) => {
                self.state.show_path = path.to_string();
                self.state.info(format!("Gespeichert: {path}"));
                self.settings.last_show = Some(path.to_string());
                self.save_settings();
            }
            Err(err) => self.state.error(err.to_string()),
        }
    }

    fn apply_menu(&mut self, ctx: &egui::Context, action: MenuAction) {
        match action {
            MenuAction::NewShow => {
                Self::lock(&self.engine).replace_show(demo::empty_show());
                self.state.reset_for_new_show();
                self.state.info("Neue, leere Show");
            }
            MenuAction::DemoShow => {
                Self::lock(&self.engine).replace_show(demo::demo_show());
                self.state.reset_for_new_show();
                self.state.info("Demo-Show geladen");
            }
            MenuAction::OpenShow => {
                self.path_dialog = Some((PathDialog::Open, self.state.show_path.clone()))
            }
            MenuAction::SaveShow => {
                let p = self.state.show_path.clone();
                self.save_show(&p);
            }
            MenuAction::SaveShowAs => {
                self.path_dialog = Some((PathDialog::SaveAs, self.state.show_path.clone()))
            }
            MenuAction::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            MenuAction::SetMode(m) => self.state.mode = m,
            MenuAction::SceneEditor => self.state.scene_editor_open = true,
            MenuAction::Zones => self.state.zones_open = true,
            MenuAction::ToggleFullscreen => {
                let fs = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fs));
            }
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
            MenuAction::ReleaseAll => Self::lock(&self.engine).release_all(),
            MenuAction::ToggleBlackout => {
                let mut e = Self::lock(&self.engine);
                let bo = e.blackout();
                e.set_blackout(!bo);
            }
            MenuAction::ClearProgrammer => Self::lock(&self.engine).clear_programmer(),
            MenuAction::Output(k) => self.switch_output(k),
            MenuAction::Settings => self.settings_open = true,
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
                } else if self.settings.lock_hash.is_some() {
                    self.lock_ui();
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
        self.state.scene_editor_open = false;
        self.state.zones_open = false;
        self.settings_open = false;
        self.path_dialog = None;
        self.settings.locked = true;
        self.save_settings();
        self.state.info("Oberfläche gesperrt");
    }

    fn unlock_ui(&mut self) {
        self.state.locked = false;
        self.settings.locked = false;
        self.save_settings();
        self.state.info("Oberfläche entsperrt");
    }

    fn lock_window(&mut self, ctx: &egui::Context) {
        let Some(mut d) = self.lock_dialog.take() else {
            return;
        };
        let mut open = true;
        let title = if d.set_password {
            "Oberfläche sperren"
        } else {
            "Oberfläche entsperren"
        };
        let action = if d.set_password {
            "Sperren"
        } else {
            "Entsperren"
        };
        let res = Dialog::new(title)
            .id("lock")
            .width(340.0)
            .cancel("Abbrechen")
            .primary(action)
            .show(ctx, &mut open, |ui, first| {
                form(ui, "lock_form", |f| {
                    let r = f.row("Passwort", |ui| {
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
                        f.row("Wiederholen", |ui| {
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
                    d.error = Some("Mindestens 4 Zeichen".into());
                } else if d.pw != d.pw2 {
                    d.error = Some("Passwörter stimmen nicht überein".into());
                } else {
                    self.settings.lock_hash = Some(crate::lock::hash_password(&d.pw));
                    self.lock_ui();
                    return;
                }
            } else if self
                .settings
                .lock_hash
                .as_deref()
                .is_some_and(|h| crate::lock::verify(h, &d.pw))
            {
                self.unlock_ui();
                return;
            } else {
                d.error = Some("Falsches Passwort".into());
                d.pw.clear();
            }
        }
        if open {
            self.lock_dialog = Some(d);
        }
    }

    fn shortcuts(&mut self, ctx: &egui::Context) -> Vec<MenuAction> {
        let typing = ctx.memory(|m| m.focused().is_some());
        ctx.input_mut(|i| {
            let mut out = Vec::new();
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::S) {
                out.push(MenuAction::SaveShow);
            }
            if !self.state.locked && i.consume_key(egui::Modifiers::COMMAND, egui::Key::O) {
                out.push(MenuAction::OpenShow);
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::F11) {
                out.push(MenuAction::ToggleFullscreen);
            }
            if !typing && i.consume_key(egui::Modifiers::NONE, egui::Key::B) {
                out.push(MenuAction::ToggleBlackout);
            }
            out
        })
    }

    fn title_info(&self) -> (String, f32, (String, egui::Color32), bool) {
        let e = Self::lock(&self.engine);
        let (fps, status) = match self.runtime.status() {
            OutputStatus::Running { fps, .. } => (fps, ("Ausgang aktiv".to_string(), theme::OK)),
            OutputStatus::Connecting => (0.0, ("Ausgang verbindet".into(), theme::WARN)),
            OutputStatus::Error(msg) => (0.0, (format!("Ausgang: Fehler – {msg}"), theme::DANGER)),
            OutputStatus::Stopped => (0.0, ("Ausgang gestoppt".into(), theme::TEXT_DIM)),
        };
        (e.show().name.clone(), fps, status, e.blackout())
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        // Pfad-Dialog
        if let Some((kind, mut path)) = self.path_dialog.take() {
            let mut open = true;
            let title = match kind {
                PathDialog::Open => "Show öffnen",
                PathDialog::SaveAs => "Show speichern unter",
            };
            let action = if kind == PathDialog::Open {
                "Öffnen"
            } else {
                "Speichern"
            };
            let res = Dialog::new(title)
                .id("path")
                .width(460.0)
                .cancel("Abbrechen")
                .primary(action)
                .show(ctx, &mut open, |ui, first| {
                    form(ui, "path_form", |f| {
                        let r = f.row("Show-Datei", |ui| {
                            ui.add(text_field(&mut path).desired_width(f32::INFINITY))
                                .on_hover_text("Pfad der Show-Datei (.json)")
                        });
                        if first {
                            r.request_focus();
                        }
                    });
                });
            let confirm = res.clicked(1);
            if confirm {
                match kind {
                    PathDialog::Open => self.open_show(&path),
                    PathDialog::SaveAs => self.save_show(&path),
                }
            } else if open {
                self.path_dialog = Some((kind, path));
            }
        }

        // Einstellungen
        if self.settings_open {
            let mut open = true;
            let mut changed = false;
            let mut new_output = self.output_kind;
            Dialog::new("Einstellungen")
                .id("settings")
                .width(440.0)
                .cancel("Schließen")
                .show(ctx, &mut open, |ui, _| {
                    let section = |ui: &mut egui::Ui, title: &str| {
                        ui.add_space(theme::SP_XS);
                        ui.label(theme::heading_text(title));
                        ui.add_space(theme::SP_XS);
                    };
                    section(ui, "Show");
                    form(ui, "settings_show", |f| {
                        let mut e = Self::lock(&self.engine);
                        f.row("Name", |ui| {
                            ui.add(text_field(&mut e.show_mut().name).desired_width(f32::INFINITY));
                        });
                    });
                    ui.add_space(theme::SP_S);
                    section(ui, "Ausgang");
                    form(ui, "settings_output", |f| {
                        f.row("Gerät", |ui| {
                            ui.spacing_mut().interact_size.y = INPUT_H;
                            egui::ComboBox::from_id_salt("settings_output")
                                .selected_text(new_output.label())
                                .width(ui.available_width())
                                .show_ui(ui, |ui| {
                                    for k in OutputKind::ALL {
                                        ui.selectable_value(&mut new_output, k, k.label());
                                    }
                                });
                        });
                    });
                    ui.add_space(theme::SP_S);
                    section(ui, "Darstellung");
                    form(ui, "settings_view", |f| {
                        f.row("Skalierung", |ui| {
                            let mut scale = self.settings.ui_scale;
                            ui.spacing_mut().slider_width = ui.available_width() - 64.0;
                            if ui
                                .add(egui::Slider::new(&mut scale, 0.6..=2.0).fixed_decimals(2))
                                .changed()
                            {
                                self.settings.ui_scale = scale;
                                ctx.set_zoom_factor(scale);
                                changed = true;
                            }
                        });
                    });
                    ui.add_space(theme::SP_S);
                    section(ui, "Start");
                    changed |= ui
                        .add(Toggle::new(
                            &mut self.settings.open_last_show,
                            "Zuletzt verwendete Show öffnen",
                        ))
                        .changed();
                    changed |= ui
                        .add(Toggle::new(
                            &mut self.settings.check_updates,
                            "Beim Start nach Updates suchen",
                        ))
                        .changed();
                    ui.add_space(theme::SP_S);
                    section(ui, "Sperre");
                    form(ui, "settings_lock", |f| {
                        f.row("Passwort", |ui| {
                            if self.settings.lock_hash.is_some() {
                                if ui
                                    .button("Entfernen")
                                    .on_hover_text(
                                        "Beim nächsten Sperren wird ein neues festgelegt",
                                    )
                                    .clicked()
                                {
                                    self.settings.lock_hash = None;
                                    changed = true;
                                }
                            } else {
                                ui.label(
                                    egui::RichText::new("nicht festgelegt").color(theme::TEXT_DIM),
                                )
                                .on_hover_text("Wird beim ersten Sperren festgelegt");
                            }
                        });
                    });
                    if let Some(p) = Settings::path() {
                        ui.add_space(theme::SP_M);
                        ui.label(
                            egui::RichText::new(format!("Gespeichert in {}", p.display()))
                                .small()
                                .color(theme::TEXT_FAINT),
                        );
                    }
                });
            if new_output != self.output_kind {
                self.switch_output(new_output);
            }
            if changed {
                self.save_settings();
            }
            self.settings_open = open;
        }

        // Über
        if self.about_open {
            let mut open = true;
            let logo = self.logo.clone();
            Dialog::new("Über DmxCraft")
                .id("about")
                .width(340.0)
                .cancel("Schließen")
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
                            egui::RichText::new(format!("DmxCraft {}", Updater::current_version()))
                                .font(theme::heading_font())
                                .size(theme::FS_TITLE),
                        );
                        ui.add_space(theme::SP_XS);
                        ui.label(
                            egui::RichText::new("DMX-Lichtsteuerung für Linux, Windows und macOS.")
                                .color(theme::TEXT_DIM),
                        );
                        ui.add_space(theme::SP_S);
                        ui.label(egui::RichText::new("Lizenz: MIT").color(theme::TEXT_DIM));
                        ui.label(
                            egui::RichText::new("Gerätedaten: Open Fixture Library (MIT)")
                                .color(theme::TEXT_DIM),
                        );
                        ui.add_space(theme::SP_S);
                        ui.hyperlink_to(
                            "github.com/JustinSchweiger/DmxCraft",
                            "https://github.com/JustinSchweiger/DmxCraft",
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
            ctx.set_zoom_factor(self.settings.ui_scale.clamp(0.6, 2.0));
            self.logo = crate::icon::texture(&ctx);
            if self.settings.check_updates {
                self.updater.check(ctx.clone());
            }
        }

        // ----- Titelleiste -----
        let (show_name, fps, status, blackout) = self.title_info();
        let update = match self.updater.state() {
            UpdateState::Available => UpdateBadge::Available,
            UpdateState::Installing => UpdateBadge::Busy("installiere Update"),
            UpdateState::Installed(v) => UpdateBadge::Installed(v),
            UpdateState::Error(e) => UpdateBadge::Error(e),
            _ => UpdateBadge::None,
        };
        let message = self
            .state
            .current_message()
            .map(|(m, e)| (m.to_string(), e));
        let info = TitleInfo {
            logo: self.logo.as_ref().map(|t| t.id()),
            show_name: &show_name,
            status,
            load: self.runtime.load(),
            fps,
            locked: self.state.locked,
            output: self.output_kind,
            blackout,
            message: message.as_ref().map(|(m, e)| (m.as_str(), *e)),
            update,
        };
        let mut actions = Vec::new();
        egui::Panel::top("titlebar")
            .exact_size(titlebar::HEIGHT)
            .frame(egui::Frame::new().fill(theme::BG))
            .show(ui, |ui| {
                actions = titlebar::show(ui, &info, &mut self.state.mode);
            });
        actions.extend(self.shortcuts(&ctx));
        for a in actions {
            self.apply_menu(&ctx, a);
        }

        // ----- Inhalt (randlos, Bereiche nur durch Trennlinien getrennt) -----
        let frame = self.runtime.last_frame();
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
                .fill(theme::SURFACE)
                .inner_margin(egui::Margin::same(6));
            match cx.state.mode {
                Mode::Setup => {
                    egui::CentralPanel::default_margins()
                        .frame(egui::Frame::new().fill(theme::SURFACE))
                        .show(ui, |ui| self.setup.ui(ui, &mut cx));
                }
                Mode::Control => {
                    let control = &mut self.control;
                    egui::Panel::bottom("live_bottom")
                        .resizable(true)
                        .default_size(400.0)
                        .min_size(220.0)
                        .frame(egui::Frame::new().fill(theme::SURFACE))
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
                    egui::CentralPanel::default_margins()
                        .frame(area)
                        .show(ui, |ui| control.grid.ui(ui, &mut cx));
                }
            }
        }
        self.dialogs(&ctx);
        self.lock_window(&ctx);
        titlebar::resize_handles(&ctx);
    }
}
