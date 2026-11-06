//! App-Rahmen: Titelleiste, Layout der Ansichten, Dialoge, Ausgabe-Runtime.

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use dmxcraft_core::{Engine, Show};
use dmxcraft_output::{OutputKind, OutputStatus, Runtime};
use eframe::egui;

use crate::demo;
use crate::settings::Settings;
use crate::state::{AppState, Mode};
use crate::titlebar::{self, MenuAction, TitleInfo};
use crate::ui::theme;
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
            MenuAction::About => self.about_open = true,
        }
    }

    fn shortcuts(&mut self, ctx: &egui::Context) -> Vec<MenuAction> {
        let typing = ctx.memory(|m| m.focused().is_some());
        ctx.input_mut(|i| {
            let mut out = Vec::new();
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::S) {
                out.push(MenuAction::SaveShow);
            }
            if i.consume_key(egui::Modifiers::COMMAND, egui::Key::O) {
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

    fn title_info(&self) -> (String, String, (String, egui::Color32), bool) {
        let e = Self::lock(&self.engine);
        let stats = format!(
            "{} Geräte · {} Szenen · {} aktiv",
            e.show().fixtures.len(),
            e.show().scenes.len(),
            e.active_scenes().len()
        );
        let status = match self.runtime.status() {
            OutputStatus::Running { fps, .. } => (format!("Ausgang {fps:.0} fps"), theme::OK),
            OutputStatus::Connecting => ("Ausgang verbindet".into(), theme::WARN),
            OutputStatus::Error(_) => ("Ausgang: Fehler".into(), theme::DANGER),
            OutputStatus::Stopped => ("Ausgang gestoppt".into(), theme::TEXT_DIM),
        };
        (e.show().name.clone(), stats, status, e.blackout())
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            match &self.state.message {
                Some((msg, true)) => {
                    ui.label(egui::RichText::new(msg).color(theme::DANGER));
                }
                Some((msg, false)) => {
                    ui.label(msg);
                }
                None => {
                    ui.label(egui::RichText::new("Bereit").color(theme::TEXT_FAINT));
                }
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                update_widget(ui, &self.updater);
                ui.label(
                    egui::RichText::new(format!("DmxCraft v{}", Updater::current_version()))
                        .color(theme::TEXT_FAINT),
                );
            });
        });
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        // Pfad-Dialog
        if let Some((kind, mut path)) = self.path_dialog.take() {
            let mut open = true;
            let mut confirm = false;
            let title = match kind {
                PathDialog::Open => "Show öffnen",
                PathDialog::SaveAs => "Show speichern unter",
            };
            egui::Window::new(title)
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("Pfad der Show-Datei (.json):");
                    let r = ui.add(egui::TextEdit::singleline(&mut path).desired_width(420.0));
                    if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        confirm = true;
                    }
                    ui.horizontal(|ui| {
                        let label = if kind == PathDialog::Open {
                            "Öffnen"
                        } else {
                            "Speichern"
                        };
                        if ui.button(label).clicked() {
                            confirm = true;
                        }
                    });
                });
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
            egui::Window::new("Einstellungen")
                .open(&mut open)
                .collapsible(false)
                .default_width(420.0)
                .show(ctx, |ui| {
                    ui.heading("Show");
                    let mut e = Self::lock(&self.engine);
                    egui::Grid::new("settings_show")
                        .num_columns(2)
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut e.show_mut().name);
                            ui.end_row();
                            ui.label("DMX-Universen");
                            let mut u = e.show().universes;
                            let max_used = e
                                .show()
                                .fixtures
                                .iter()
                                .map(|f| f.universe + 1)
                                .max()
                                .unwrap_or(1);
                            if ui
                                .add(egui::DragValue::new(&mut u).range(max_used.max(1)..=8))
                                .on_hover_text(
                                    "Kann nicht unter das höchste belegte Universum gesetzt werden",
                                )
                                .changed()
                            {
                                e.show_mut().universes = u;
                            }
                            ui.end_row();
                        });
                    drop(e);
                    ui.separator();
                    ui.heading("Ausgang");
                    egui::ComboBox::from_id_salt("settings_output")
                        .selected_text(new_output.label())
                        .show_ui(ui, |ui| {
                            for k in OutputKind::ALL {
                                ui.selectable_value(&mut new_output, k, k.label());
                            }
                        });
                    ui.separator();
                    ui.heading("Darstellung");
                    let mut scale = self.settings.ui_scale;
                    if ui
                        .add(
                            egui::Slider::new(&mut scale, 0.6..=2.0)
                                .text("Skalierung")
                                .fixed_decimals(2),
                        )
                        .changed()
                    {
                        self.settings.ui_scale = scale;
                        ctx.set_zoom_factor(scale);
                        changed = true;
                    }
                    ui.separator();
                    ui.heading("Start");
                    changed |= ui
                        .checkbox(
                            &mut self.settings.open_last_show,
                            "Zuletzt verwendete Show öffnen",
                        )
                        .changed();
                    changed |= ui
                        .checkbox(
                            &mut self.settings.check_updates,
                            "Beim Start nach Updates suchen",
                        )
                        .changed();
                    ui.separator();
                    if let Some(p) = Settings::path() {
                        ui.label(
                            egui::RichText::new(format!("Gespeichert in {}", p.display()))
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
            egui::Window::new("Über DmxCraft")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if let Some(tex) = &self.logo {
                        ui.add(egui::Image::new(egui::load::SizedTexture::new(
                            tex.id(),
                            [64.0, 64.0],
                        )));
                    }
                    ui.heading(format!("DmxCraft {}", Updater::current_version()));
                    ui.label("DMX-Lichtsteuerung für Linux, Windows und macOS.");
                    ui.label("Lizenz: MIT");
                    ui.label("Gerätedaten: Open Fixture Library (MIT)");
                    ui.hyperlink_to(
                        "github.com/JustinSchweiger/DmxCraft",
                        "https://github.com/JustinSchweiger/DmxCraft",
                    );
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
        let (show_name, stats, status, blackout) = self.title_info();
        let info = TitleInfo {
            logo: self.logo.as_ref().map(|t| t.id()),
            show_name: &show_name,
            status,
            stats,
            output: self.output_kind,
            blackout,
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

        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(8, 3)),
            )
            .show(ui, |ui| self.status_bar(ui));

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
        titlebar::resize_handles(&ctx);
    }
}

fn update_widget(ui: &mut egui::Ui, updater: &Updater) {
    let ctx = ui.ctx().clone();
    match updater.state() {
        UpdateState::Idle | UpdateState::UpToDate => {
            if ui
                .small_button("Updates")
                .on_hover_text("Nach neuer Version suchen")
                .clicked()
            {
                updater.check(ctx);
            }
        }
        UpdateState::Checking => {
            ui.label(egui::RichText::new("suche Updates").color(theme::TEXT_DIM));
        }
        UpdateState::Available => {
            let b = egui::Button::new("Update installieren").fill(theme::ACCENT_DIM);
            if ui
                .add(b)
                .on_hover_text("Neue Version von GitHub laden")
                .clicked()
            {
                updater.install(ctx);
            }
        }
        UpdateState::Installing => {
            ui.label(egui::RichText::new("installiere Update").color(theme::TEXT_DIM));
        }
        UpdateState::Installed(v) => {
            ui.label(
                egui::RichText::new(format!("v{v} installiert – bitte neu starten"))
                    .color(theme::OK),
            );
        }
        UpdateState::NotInstalled => {
            ui.label(egui::RichText::new("(Entwickler-Build)").color(theme::TEXT_FAINT))
                .on_hover_text(
                    "Updates gibt es nur für Installationen über die offiziellen Installer.",
                );
        }
        UpdateState::Error(e) => {
            if ui
                .small_button("Update-Fehler")
                .on_hover_text(format!("{e}\nKlicken für neuen Versuch"))
                .clicked()
            {
                updater.check(ctx);
            }
        }
    }
}
