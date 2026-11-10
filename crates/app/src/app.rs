//! App frame: title bar, layout of the views, dialogs, output runtime.

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

/// DMX refresh rate of the output thread.
pub const OUTPUT_HZ: u32 = 50;

/// Views in CONTROL.
#[derive(Default)]
struct ControlViews {
    grid: SceneGridView,
    stage: StageView,
    programmer: ProgrammerView,
    props: ScenePropsView,
    touch: crate::views::touch::TouchView,
}

/// Dialog for locking (set a password) or unlocking.
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
    /// Open system dialog (Open / Save as).
    file_dialog: Option<file_dialog::Pending>,
    lock_dialog: Option<LockDialog>,
    /// Hash of the lock password – kept in memory only, forgotten after unlocking.
    lock_hash: Option<String>,
    state: AppState,
    control: ControlViews,
    setup: SetupView,
    sound: crate::views::sound::SoundView,
    windows: Windows,
    history: History,
    /// MCP server for AI assistants.
    mcp: crate::mcp::McpLink,
    remote: crate::remote::RemoteLink,
    mapper: crate::mapping::Mapper,
    /// 3D visualizer.
    viz: crate::viz::VizView,
    /// Profile Builder (separate window).
    builder: crate::views::profile_builder::ProfileBuilder,
    fullscreen: Fullscreen,
    /// Toggle fullscreen in the next frame.
    toggle_fullscreen: bool,
    /// Last detected USB DMX cables (for the settings).
    usb_ports: Vec<dmxcraft_output::UsbDmxPort>,
    /// State of the show at the last load/save (for “unsaved”).
    saved_show: Show,
    /// Question “Save before closing?” open.
    close_dialog: bool,
    /// Close after saving successfully.
    close_after_save: bool,
    /// Allow closing without asking again.
    allow_close: bool,
    /// Close in the next frame.
    request_close: bool,
}

/// Fill in missing data of the fixture types (library and custom profiles).
fn fill_types(s: &mut Show) {
    let own: Vec<dmxcraft_core::FixtureType> = crate::profiles::load_user()
        .into_iter()
        .flat_map(|u| u.types)
        .collect();
    dmxcraft_ofl::fill_details(s, &own);
}

impl DmxCraftApp {
    pub fn new(show_path: Option<String>, settings: Settings) -> Self {
        let mut message = None;
        let show = match &show_path {
            Some(p) => match Show::load(p) {
                Ok(mut s) => {
                    demo::ensure_builtin_types(&mut s);
                    fill_types(&mut s);
                    message = Some((tr!("Show “{v}” loaded", v = s.name), false));
                    s
                }
                Err(e) => {
                    message = Some((format!("{p}: {e} – demo show loaded"), true));
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
        if settings.start_touch {
            state.mode = Mode::Touch;
        }
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
            sound: Default::default(),
            windows: Windows::default(),
            history: History::default(),
            mcp: crate::mcp::McpLink::default(),
            remote: crate::remote::RemoteLink::default(),
            mapper: crate::mapping::Mapper::default(),
            viz: crate::viz::VizView::default(),
            builder: Default::default(),
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

    /// Displayed name: the file name of the show (without extension), otherwise
    /// the name of the unsaved show.
    fn display_name(&self, e: &Engine) -> String {
        std::path::Path::new(&self.state.show_path)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| e.show().name.clone())
    }

    /// Unsaved changes to the show?
    fn dirty(&self) -> bool {
        *Self::lock(&self.engine).show() != self.saved_show
    }

    /// Remember the current state as saved.
    fn mark_saved(&mut self) {
        self.saved_show = Self::lock(&self.engine).show().clone();
    }

    fn lock(engine: &Arc<Mutex<Engine>>) -> MutexGuard<'_, Engine> {
        // A poisoned mutex means: the output thread crashed.
        // The data is still consistent enough to keep working.
        engine.lock().unwrap_or_else(|p| p.into_inner())
    }

    /// Restart the output with the current settings (e.g. after the cable
    /// assignment changed).
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
            self.state.error(tr!("Settings not saved: {e}", e = e));
        }
    }

    fn open_show(&mut self, path: &str) {
        match Show::load(path) {
            Ok(mut s) => {
                demo::ensure_builtin_types(&mut s);
                fill_types(&mut s);
                let name = s.name.clone();
                Self::lock(&self.engine).replace_show(s);
                self.history.reset(&Snapshot::of(&Self::lock(&self.engine)));
                self.mark_saved();
                self.state.reset_for_new_show();
                self.state.show_path = path.to_string();
                self.state.info(tr!("Show “{name}” loaded", name = name));
                self.settings.remember(path);
                self.save_settings();
            }
            Err(err) => self.state.error(format!("{path}: {err}")),
        }
    }

    fn save_show(&mut self, path: &str) {
        // Name of the show = file name
        if let Some(stem) = std::path::Path::new(path).file_stem() {
            Self::lock(&self.engine).show_mut().name = stem.to_string_lossy().into_owned();
        }
        let result = Self::lock(&self.engine).show().save(path);
        match result {
            Ok(()) => {
                self.mark_saved();
                self.state.show_path = path.to_string();
                self.state.info(tr!("Saved: {path}", path = path));
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
        // Only one dialog at a time
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
                // new file: the first save asks for the location
                self.state.show_path.clear();
                self.state.info(t("New, empty show"));
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
                    self.state.error(tr!("Not found: {path}", path = path));
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
                        // Reload the edit values from the restored scene
                        self.state.edit_reload = true;
                        self.state
                            .selected
                            .retain(|id| e.show().fixture(*id).is_some());
                    }
                    None => {
                        drop(e);
                        self.state.info(t("Nothing to undo"));
                    }
                }
            }
            MenuAction::Visualizer => {
                if self.settings.tools_as_tabs && !self.state.viz_window {
                    self.state.mode = Mode::Visualizer;
                } else {
                    self.state.viz_window = !self.state.viz_window;
                }
            }
            MenuAction::ProfileBuilder => {
                self.state.builder = Some(crate::views::profile_builder::BuilderRequest::Open);
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
            MenuAction::CheckUpdates => self.updater.check(ctx.clone(), true),
            MenuAction::InstallUpdate => self.updater.install(ctx.clone()),
            MenuAction::About => self.about_open = true,
            MenuAction::Mapping(kind) => {
                let st = &mut self.state;
                st.mapping = if st.mapping == Some(kind) {
                    None
                } else {
                    Some(kind)
                };
                st.map_sel = None;
                st.map_learn = false;
            }
            MenuAction::AudioMaster(v) => self.state.audio.set_master(v),
            MenuAction::AudioMute => self.state.audio.mute = !self.state.audio.mute,
            MenuAction::MissingAudio => self.state.missing.open = true,
            MenuAction::Obs => {
                self.settings_window.tab = crate::settings_window::Tab::Obs;
                self.settings_open = true;
            }
            MenuAction::AudioFade => {
                let e = Self::lock(&self.engine);
                self.state.audio.start_fade(&e);
            }
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
        self.state.info(t("Interface locked"));
    }

    fn unlock_ui(&mut self) {
        self.state.locked = false;
        // The password is only valid for one lock – set a new one next time.
        self.lock_hash = None;
        self.state.info(t("Interface unlocked"));
    }

    fn lock_window(&mut self, ctx: &egui::Context) {
        let Some(mut d) = self.lock_dialog.take() else {
            return;
        };
        let mut open = true;
        let title = if d.set_password {
            t("Lock interface")
        } else {
            t("Unlock interface")
        };
        let action = if d.set_password {
            t("Lock")
        } else {
            t("Unlock")
        };
        let res = Dialog::new(title)
            .id("lock")
            .width(340.0)
            .cancel(t("Cancel"))
            .primary(action)
            .show(ctx, &mut open, |ui, first| {
                form(ui, "lock_form", |f| {
                    let r = f.row(t("Password"), |ui| {
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
                        f.row(t("Repeat"), |ui| {
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
                    d.error = Some(t("At least 4 characters").into());
                } else if d.pw != d.pw2 {
                    d.error = Some(t("Passwords do not match").into());
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
                d.error = Some(t("Wrong password").into());
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
        // Ctrl+D duplicates in the patch or in the timeline, otherwise DMX values
        let duplicate = (self.state.mode == Mode::Setup && !self.state.selected.is_empty())
            || (self.state.focus == crate::state::Focus::Timeline
                && !self.state.timeline_sel.is_empty());
        ctx.input_mut(|i| {
            let mut out = Vec::new();
            let mut key =
                |i: &mut egui::InputState, m: Modifiers, k: Key, ok: bool, a: MenuAction| {
                    if ok && i.consume_key(m, k) {
                        out.push(a);
                    }
                };
            // Shift variants first, otherwise Ctrl+S takes over
            key(i, cmd_shift, Key::S, free, MenuAction::SaveShowAs);
            key(i, cmd_shift, Key::Z, free && !typing, MenuAction::Redo);
            key(i, cmd, Key::N, free, MenuAction::NewShow);
            key(i, cmd, Key::O, free, MenuAction::OpenShow);
            key(i, cmd, Key::S, true, MenuAction::SaveShow);
            key(i, cmd, Key::Z, free && !typing, MenuAction::Undo);
            key(i, cmd, Key::Y, free && !typing, MenuAction::Redo);
            key(i, cmd, Key::L, true, MenuAction::ToggleLock);
            key(i, cmd, Key::Comma, free, MenuAction::Settings);
            key(i, cmd, Key::D, free && !duplicate, MenuAction::DmxLevels);
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
            } => (fps, (t("Output active").to_string(), theme::OK)),
            OutputStatus::Running {
                fps, note: Some(n), ..
            } => (fps, (tr!("Output active, but: {n}", n = n), theme::WARN)),
            OutputStatus::Connecting => (0.0, (t("Output connecting").into(), theme::WARN)),
            OutputStatus::Error(msg) => (
                0.0,
                (tr!("Output: error – {msg}", msg = msg), theme::DANGER),
            ),
            OutputStatus::Stopped => (0.0, (t("Output stopped").into(), theme::text_dim())),
        };
        (self.display_name(&e), fps, status, e.blackout())
    }

    fn dialogs(&mut self, ctx: &egui::Context) {
        // Result of a system dialog
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
                // Saving cancelled: do not close
                None => self.close_after_save = false,
            }
        }

        // Before closing: unsaved changes?
        if self.close_dialog {
            let mut open = true;
            let name = {
                let e = Self::lock(&self.engine);
                self.display_name(&e)
            };
            let res = Dialog::new(t("Unsaved changes"))
                .id("close_unsaved")
                .width(380.0)
                .cancel(t("Cancel"))
                .secondary(t("Don't save"))
                .primary(t("Save"))
                .show(ctx, &mut open, |ui, _| {
                    ui.label(tr!(
                        "Do you want to save the changes to “{name}” before closing?",
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

        // Settings
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
                    mcp: &self.mcp.status,
                    remote: &self.remote,
                    midi_inputs: &self.mapper.midi.input_names,
                    midi_outputs: &self.mapper.midi.output_names,
                    obs: &self.state.obs,
                },
            );
            drop(e);
            if ch.mcp_retry {
                self.mcp.retry();
            }
            if ch.remote_retry {
                self.remote.retry();
            }
            if let Some((out, model)) = ch.midi_clear {
                self.mapper.clear_now(&out, model);
            }
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

        // About
        if self.about_open {
            let mut open = true;
            let logo = self.logo.clone();
            Dialog::new(t("About DMXCraft"))
                .id("about")
                .width(340.0)
                .cancel(t("Close"))
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
                                "DMX lighting control for Linux, Windows and macOS.",
                            ))
                            .color(theme::text_dim()),
                        );
                        ui.add_space(theme::SP_S);
                        ui.label(
                            egui::RichText::new("© 2026 Justin Schweiger").color(theme::text_dim()),
                        );
                        ui.label(
                            egui::RichText::new(t("License: GPL-3.0, without any warranty"))
                                .color(theme::text_dim()),
                        );
                        ui.label(
                            egui::RichText::new(t(
                                "Fixture data: Open Fixture Library (MIT), QLC+ (Apache 2.0)",
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
    fn ui(&mut self, ui: &mut egui::Ui, frame_: &mut eframe::Frame) {
        let rs = frame_.wgpu_render_state().cloned();
        let ctx = ui.ctx().clone();
        // Show running scenes and live colors smoothly.
        ctx.request_repaint_after(Duration::from_millis(33));
        if !self.started {
            self.started = true;
            theme::apply(&ctx);
            // Handle the zoom keys ourselves so the size is saved.
            ctx.options_mut(|o| o.zoom_with_keyboard = false);
            ctx.set_zoom_factor(self.settings.ui_scale.clamp(0.6, 2.0));
            self.logo = crate::icon::texture(&ctx);
            if self.settings.check_updates {
                self.updater.check(ctx.clone(), false);
            }
        }
        if let Some((msg, err)) = self.updater.take_notice() {
            if err {
                self.state.error(msg);
            } else {
                self.state.info(msg);
            }
        }

        // ----- AI assistant (MCP) -----
        let events = self.mcp.update(
            &ctx,
            &self.engine,
            &self.settings,
            &self.state.show_path,
            self.state.locked,
        );
        for ev in events {
            match ev {
                dmxcraft_mcp::AppEvent::ToolCalled(name) => {
                    self.state.info(tr!("MCP: {name}", name = name));
                }
                dmxcraft_mcp::AppEvent::Saved(path) => {
                    self.mark_saved();
                    self.state.show_path = path.clone();
                    self.settings.remember(&path);
                    self.save_settings();
                }
                dmxcraft_mcp::AppEvent::SelectScene(id) => {
                    self.state.selected_scenes = [id].into();
                    self.state.focus = crate::state::Focus::Scenes;
                    self.state.mode = Mode::Control;
                }
                dmxcraft_mcp::AppEvent::ProfilesChanged => {
                    self.state.user_profiles = crate::profiles::load_user();
                }
            }
        }

        // ----- Closing: ask first when there are unsaved changes -----
        if std::mem::take(&mut self.request_close) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if ctx.input(|i| i.viewport().close_requested()) && !self.allow_close && self.dirty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.close_dialog = true;
        }

        // ----- Keyboard/MIDI mapping: overlay for this frame -----
        if self.state.locked {
            self.state.mapping = None;
            self.state.map_learn = false;
        }
        if self.mapper.devices != self.settings.midi_devices {
            self.mapper.devices = self.settings.midi_devices.clone();
        }
        self.mapper
            .begin_frame(&ctx, Self::lock(&self.engine).show(), &self.state);

        // ----- Title bar -----
        let (mut show_name, fps, status, blackout) = self.title_info();
        if self.dirty() {
            show_name.push_str(" *");
        }
        let update = match self.updater.state() {
            UpdateState::Available(r) => UpdateBadge::Available(r.version),
            UpdateState::Installing => UpdateBadge::Busy(t("installing update")),
            UpdateState::Installed(v) => UpdateBadge::Installed(v),
            UpdateState::InstallerStarted(v) => UpdateBadge::InstallerStarted(v),
            UpdateState::Error(e) => UpdateBadge::Error(e),
            _ => UpdateBadge::None,
        };
        let message = self
            .state
            .current_message()
            .map(|(m, e)| (m.to_string(), e));
        let obs_enabled = Self::lock(&self.engine).show().obs.enabled;
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
            mapping: self.state.mapping,
            tools_as_tabs: self.settings.tools_as_tabs,
            audio: titlebar::AudioInfo {
                master: self.state.audio.master,
                mute: self.state.audio.mute,
                pickup: self.state.audio.pickup,
                fading: self.state.audio.fading(),
                fade_ms: self.state.audio.fade_ms_for(&Self::lock(&self.engine)),
                limit: self.state.audio.limit,
            },
            missing_audio: self.state.missing.files.len(),
            obs: obs_enabled.then(|| self.state.obs.status() == crate::obs::Status::Connected),
        };
        let mut actions = Vec::new();
        egui::Panel::top("titlebar")
            .exact_size(titlebar::HEIGHT)
            .frame(egui::Frame::new().fill(theme::bg()))
            .show(ui, |ui| {
                actions = titlebar::show(ui, &info, &mut self.state.mode);
            });
        if !self.state.map_learn {
            actions.extend(self.shortcuts(&ctx));
        }
        for a in actions {
            self.apply_menu(&ctx, a);
        }
        if std::mem::take(&mut self.toggle_fullscreen) {
            self.fullscreen.toggle(&ctx);
        }
        self.fullscreen.update(&ctx);

        // ----- Content (borderless, areas separated only by divider lines) -----
        // Blind: stage and faders show the edit, not the output
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
            self.state.wrap_scenes = self.settings.wrap_scenes;
            self.state.fader_ranges = self.settings.fader_ranges;
            self.state.viz_quality = crate::viz::Quality::from_key(self.settings.viz_quality);
            let mut cx = ViewCtx {
                engine: &mut e,
                state: &mut self.state,
                frame: &frame,
            };
            self.windows.show(&ctx, &mut cx);
            cx.state.missing.check(cx.engine.show(), false);
            cx.state.missing.publish(&ctx);
            crate::missing_audio::window(&ctx, &mut cx);
            let area = egui::Frame::new()
                .fill(theme::surface())
                .inner_margin(egui::Margin::same(6));
            // Tool tabs only with the setting (Builder not locked)
            let tool_tab = match cx.state.mode {
                Mode::Visualizer => true,
                Mode::Builder => !cx.state.locked,
                _ => false,
            };
            if matches!(cx.state.mode, Mode::Visualizer | Mode::Builder)
                && !(self.settings.tools_as_tabs && tool_tab)
            {
                cx.state.mode = Mode::Control;
            }
            match cx.state.mode {
                Mode::Visualizer => {
                    egui::CentralPanel::default_margins()
                        .frame(area)
                        .show(ui, |ui| {
                            if cx.state.viz_window {
                                if moved_out(ui, t("The 3D visualizer is open in its own window."))
                                {
                                    cx.state.viz_window = false;
                                }
                            } else {
                                self.viz.ui(ui, &mut cx, rs.as_ref(), false);
                            }
                        });
                }
                Mode::Builder => {
                    if self.builder.is_open() {
                        egui::CentralPanel::default_margins()
                            .frame(area)
                            .show(ui, |ui| {
                                if moved_out(
                                    ui,
                                    t("The Profile Builder is open in its own window."),
                                ) {
                                    self.builder.close();
                                }
                            });
                    } else {
                        self.builder.embedded(ui, cx.engine, cx.state);
                    }
                }
                Mode::Sound => {
                    egui::CentralPanel::default_margins()
                        .frame(egui::Frame::new().fill(theme::surface()))
                        .show(ui, |ui| self.sound.ui(ui, &mut cx));
                }
                Mode::Setup => {
                    egui::CentralPanel::default_margins()
                        .frame(egui::Frame::new().fill(theme::surface()))
                        .show(ui, |ui| self.setup.ui(ui, &mut cx));
                }
                Mode::Control | Mode::Touch => {
                    let touch_mode = cx.state.mode == Mode::Touch;
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
                    if touch_mode {
                        egui::CentralPanel::default_margins()
                            .frame(area)
                            .show(ui, |ui| control.touch.ui(ui, &mut cx));
                    } else {
                        // Sidebar with the settings of the selected scene
                        egui::Panel::right("scene_props")
                            .resizable(true)
                            .default_size(360.0)
                            .min_size(340.0)
                            .frame(area)
                            .show(ui, |ui| control.props.ui(ui, &mut cx));
                        egui::CentralPanel::default_margins()
                            .frame(area)
                            .show(ui, |ui| control.grid.ui(ui, &mut cx));
                    }
                }
            }
            crate::mapping::window::show(&ctx, cx.engine, cx.state, &mut self.mapper);
            // Evaluate the touch remote (web app) before the mappings
            self.remote
                .update(&ctx, cx.engine, cx.state, &self.settings);
            // Audio triggers of running SUPER SCENES (before the mapping)
            let mut triggers = std::mem::take(&mut cx.state.triggers);
            triggers.update(&ctx, cx.engine, cx.state);
            cx.state.triggers = triggers;
            self.mapper.end_frame(&ctx, cx.engine, cx.state);
            {
                let board = {
                    let mut b = cx.engine.show().sound.clone();
                    b.normalize();
                    b
                };
                crate::views::sound::poll(&ctx, &board, &mut cx.state.sound);
                if cx.state.sound.update(&board, cx.state.audio.gain()) {
                    ctx.request_repaint_after(std::time::Duration::from_millis(50));
                }
            }
            // Alt + click: element to the touch surface or as trigger target
            for p in crate::mapping::take_touch_picks(&ctx) {
                if cx.state.locked {
                    continue;
                }
                if let Some(target) = cx.state.trigger_pick.take() {
                    crate::views::control::audio_trigger_ui::apply_trigger_pick(
                        cx.engine, cx.state, target, p,
                    );
                } else {
                    crate::views::touch::apply_pick(cx.engine, cx.state, p);
                }
            }
            edit_preview(cx.engine, cx.state);
            cx.engine.set_timeline_cursor(cx.state.timeline_cursor);
            let scrubbing = cx.state.audio.scrubbing;
            cx.state.obs.update(cx.engine, scrubbing, &ctx);
            cx.state.audio.update(cx.engine, &self.settings);
            if cx.state.audio.fading() {
                ctx.request_repaint();
            }
        }
        self.state.viewport_busy = false;
        // Requests of the AI assistant to the visualizer (camera, image)
        let jobs = self.mcp.take_viz_jobs();
        if !jobs.is_empty() {
            let mut e = Self::lock(&self.engine);
            let mut cx = ViewCtx {
                engine: &mut e,
                state: &mut self.state,
                frame: &frame,
            };
            for job in jobs {
                let r = self.viz.mcp(&job.action, &job.args, &mut cx, rs.as_ref());
                let _ = job.reply.send(r);
            }
        }
        // 3D visualizer in its own window (e.g. on the second screen)
        if self.state.viz_window {
            let engine = self.engine.clone();
            let viz = &mut self.viz;
            let state = &mut self.state;
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("dmxcraft_viz"),
                egui::ViewportBuilder::default()
                    .with_title("DMXCraft 3D")
                    .with_inner_size([1280.0, 760.0])
                    .with_min_inner_size([480.0, 320.0]),
                |vctx, _class| {
                    egui::CentralPanel::default()
                        .frame(
                            egui::Frame::new()
                                .fill(theme::surface())
                                .inner_margin(egui::Margin::same(6)),
                        )
                        .show(vctx, |ui| {
                            let mut e = Self::lock(&engine);
                            let mut cx = ViewCtx {
                                engine: &mut e,
                                state,
                                frame: &frame,
                            };
                            viz.ui(ui, &mut cx, rs.as_ref(), true);
                        });
                    viewport_input(vctx, state, true);
                    if vctx.input(|i| i.viewport().close_requested()) {
                        state.viz_window = false;
                    }
                },
            );
        }
        // Profile Builder in its own window
        if let Some(req) = self.state.builder.take() {
            // as a tab: open there, unless the window is already open
            if self.settings.tools_as_tabs && !self.builder.is_open() {
                self.builder.request(req, false);
                self.state.mode = Mode::Builder;
            } else {
                self.builder.request(req, true);
            }
        }
        if self.state.locked {
            self.builder.close();
        }
        if self.builder.is_open() {
            let engine = self.engine.clone();
            let builder = &mut self.builder;
            let state = &mut self.state;
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("dmxcraft_profile_builder"),
                egui::ViewportBuilder::default()
                    .with_title(builder.title())
                    .with_inner_size([1240.0, 800.0])
                    .with_min_inner_size([900.0, 560.0]),
                |vctx, _class| {
                    let mut e = Self::lock(&engine);
                    builder.window(vctx, &mut e, state);
                    viewport_input(vctx, state, false);
                    if vctx.input(|i| i.viewport().close_requested()) {
                        builder.close();
                    }
                },
            );
        }
        if self.state.viz_quality.key() != self.settings.viz_quality {
            self.settings.viz_quality = self.state.viz_quality.key();
            self.save_settings();
        }
        if let Some(undo) = self.state.history_request.take() {
            self.apply_menu(
                &ctx,
                if undo {
                    MenuAction::Undo
                } else {
                    MenuAction::Redo
                },
            );
        }
        self.state.can_undo = self.history.can_undo();
        self.state.can_redo = self.history.can_redo();
        self.dialogs(&ctx);
        self.lock_window(&ctx);
        titlebar::resize_handles(&ctx, self.fullscreen.is_on());

        // Undo: only take over changes once dragging/typing has finished.
        let busy = ctx.input(|i| i.pointer.any_down())
            || ctx.memory(|m| m.focused().is_some())
            || self.state.viewport_busy;
        self.history.track(&Self::lock(&self.engine), busy);
    }
}

/// Input of a separate window: dragging/typing there counts as “busy” for
/// the history (otherwise every intermediate state of a drag would become
/// its own undo step), and Ctrl+Z / Ctrl+Y / Ctrl+Shift+Z work there too.
fn viewport_input(ui: &mut egui::Ui, state: &mut AppState, undo_keys: bool) {
    use egui::{Key, Modifiers};
    let typing = ui.memory(|m| m.focused().is_some());
    state.viewport_busy |= typing || ui.input(|i| i.pointer.any_down());
    if !undo_keys || typing || state.locked {
        return;
    }
    let cmd = Modifiers::COMMAND;
    let request = ui.input_mut(|i| {
        if i.consume_key(cmd | Modifiers::SHIFT, Key::Z) || i.consume_key(cmd, Key::Y) {
            Some(false)
        } else if i.consume_key(cmd, Key::Z) {
            Some(true)
        } else {
            None
        }
    });
    if request.is_some() {
        state.history_request = request;
    }
}

/// Edit mode: selected scenes are output (even when they are not running)
/// and their fixtures are selected when the scene is selected. Stage and
/// DMX output therefore show the scene being edited.
///
/// If exactly one scene is selected, it is the edit scene: when it is
/// selected its values are loaded into the programmer, and every change in
/// the programmer is saved immediately in its (first) step.
fn edit_preview(e: &mut Engine, st: &mut AppState) {
    st.sync_selection_order(e.show());
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
    // keep the edited step within the scene
    let steps = target
        .and_then(|id| e.show().scene(id))
        .map_or(1, |s| s.steps.len().max(1));
    st.edit_step = st.edit_step.min(steps - 1);
    e.set_preview_step(st.edit_step);
    // Switching to edit mode loads the selected scene like a new selection
    if st.programmer_mode != st.programmer_mode_seen {
        let was_edit = st.programmer_mode_seen.is_edit();
        st.programmer_mode_seen = st.programmer_mode;
        if edit && !was_edit {
            st.scene_selection_seen.clear();
            st.scene_selection_seen.insert(SceneId(u32::MAX));
        } else if !edit && was_edit {
            // Edit values live in the scene; the programmer does not output them as well
            e.clear_programmer();
        }
    }
    // another step chosen: load its values
    if st.edit_step != st.edit_step_seen {
        st.edit_step_seen = st.edit_step;
        st.edit_reload = true;
    }
    if std::mem::take(&mut st.edit_reload) && edit {
        if !st.selected_scenes.is_empty() {
            e.clear_programmer();
            if let Some(id) = target {
                load_step(e, id, st.edit_step);
            }
        }
        return;
    }
    if st.selected_scenes != st.scene_selection_seen {
        st.scene_selection_seen = st.selected_scenes.clone();
        // new scene: start at the first step, no effect as FX target
        st.edit_step = 0;
        st.fx_target = st
            .fx_target
            .filter(|(s, _)| st.selected_scenes.len() == 1 && st.selected_scenes.contains(s));
        st.edit_step_seen = 0;
        e.set_preview_step(0);
        if edit {
            if !st.selected_scenes.is_empty() {
                let show = e.show();
                // Effect fixtures first (their order matters), then the others
                let mut ids: Vec<dmxcraft_core::FixtureId> = Vec::new();
                for s in st.selected_scenes.iter().filter_map(|id| show.scene(*id)) {
                    let fx = s.effects.iter().flat_map(|e| e.fixtures.iter().copied());
                    let steps = s
                        .steps
                        .iter()
                        .flat_map(|step| step.values.iter().map(|v| v.fixture));
                    for f in fx.chain(steps) {
                        if show.fixture(f).is_some() && !ids.contains(&f) {
                            ids.push(f);
                        }
                    }
                }
                st.select_ordered(&ids);
            }
            // Load the values of the new edit scene into the programmer
            e.clear_programmer();
            if let Some(id) = target {
                load_step(e, id, 0);
            }
            return;
        }
    }
    // Save programmer changes in the edited step
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
    let i = st.edit_step.min(scene.steps.len() - 1);
    let step = &mut scene.steps[i];
    let mut current = step.values.clone();
    current.sort_by_key(|c| (c.fixture, c.channel));
    if current != values {
        step.values = values;
    }
}

/// Load the values of a step of the scene into the programmer.
fn load_step(e: &mut Engine, id: SceneId, i: usize) {
    let Some(step) = e
        .show()
        .scene(id)
        .and_then(|s| s.steps.get(i).or(s.steps.first()))
        .cloned()
    else {
        return;
    };
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

/// Placeholder in the tab while the tool runs in its own window. `true` if
/// it should be brought back into the tab.
fn moved_out(ui: &mut egui::Ui, text: &str) -> bool {
    let mut back = false;
    ui.vertical_centered(|ui| {
        ui.add_space(ui.available_height() * 0.38);
        ui.label(
            egui::RichText::new(text)
                .color(theme::text_dim())
                .size(theme::FS_TITLE),
        );
        ui.add_space(theme::SP_M);
        back = ui.button(t("Show here")).clicked();
    });
    back
}
