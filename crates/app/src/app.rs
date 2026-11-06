//! App-Rahmen: Kopfzeile, Modus, Layout der Ansichten, Ausgabe-Runtime.

use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use dmxcraft_core::{Engine, Show};
use dmxcraft_output::{OutputKind, OutputStatus, Runtime};
use eframe::egui;

use crate::demo;
use crate::state::{AppState, Mode};
use crate::ui::theme;
use crate::ui::widgets::Segmented;
use crate::updater::{UpdateState, Updater};
use crate::views::control::{ProgrammerView, SceneGridView, StageView};
use crate::views::library::LibraryWindow;
use crate::views::setup::SetupView;
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

pub struct DmxCraftApp {
    engine: Arc<Mutex<Engine>>,
    runtime: Runtime,
    output_kind: OutputKind,
    updater: Updater,
    started: bool,
    logo: Option<egui::TextureHandle>,
    state: AppState,
    control: ControlViews,
    setup: SetupView,
    library: LibraryWindow,
}

impl DmxCraftApp {
    pub fn new(show_path: Option<String>) -> Self {
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
        let first_type = show
            .fixture_types
            .first()
            .map(|t| t.key.clone())
            .unwrap_or_default();
        let engine = Arc::new(Mutex::new(Engine::new(show)));
        let output_kind = OutputKind::Simulation;
        let runtime = Runtime::start(engine.clone(), output_kind.create(), OUTPUT_HZ);
        let mut state = AppState::new(show_path.unwrap_or_else(|| "show.json".into()), first_type);
        state.message = message;
        Self {
            engine,
            runtime,
            output_kind,
            updater: Updater::default(),
            started: false,
            logo: None,
            state,
            control: ControlViews::default(),
            setup: SetupView::default(),
            library: LibraryWindow::default(),
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
    }

    fn header(&mut self, ui: &mut egui::Ui) {
        let mut new_kind = self.output_kind;
        let status = self.runtime.status();
        let engine = self.engine.clone();
        let mut e = Self::lock(&engine);
        let st = &mut self.state;

        ui.horizontal(|ui| {
            ui.set_min_height(36.0);
            if let Some(tex) = &self.logo {
                ui.add(egui::Image::new(egui::load::SizedTexture::new(
                    tex.id(),
                    [28.0, 28.0],
                )));
            }
            ui.label(
                egui::RichText::new("DmxCraft")
                    .strong()
                    .size(theme::FS_TITLE + 1.0),
            );
            ui.add_space(theme::SP_S);
            Segmented::new(&[(Mode::Setup, "Einrichten"), (Mode::Control, "Steuern")])
                .show(ui, &mut st.mode);
            ui.add_space(theme::SP_S);
            ui.add(
                egui::TextEdit::singleline(&mut e.show_mut().name)
                    .desired_width(160.0)
                    .hint_text("Showname"),
            )
            .on_hover_text("Name der Show");
            file_menu(ui, &mut e, st);

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let bo = e.blackout();
                let bo_btn =
                    egui::Button::new(egui::RichText::new("Blackout").strong().color(if bo {
                        egui::Color32::WHITE
                    } else {
                        theme::TEXT
                    }))
                    .fill(if bo { theme::DANGER } else { theme::RAISED })
                    .min_size(egui::vec2(92.0, 28.0));
                if ui.add(bo_btn).clicked() {
                    e.set_blackout(!bo);
                }
                if ui
                    .button("Alle aus")
                    .on_hover_text("Alle Szenen stoppen")
                    .clicked()
                {
                    e.release_all();
                }
                let mut speed = e.speed();
                if ui
                    .add(
                        egui::Slider::new(&mut speed, 0.1..=4.0)
                            .logarithmic(true)
                            .fixed_decimals(2)
                            .suffix("x"),
                    )
                    .on_hover_text("Wiedergabegeschwindigkeit")
                    .changed()
                {
                    e.set_speed(speed);
                }
                ui.label(egui::RichText::new("Speed").color(theme::TEXT_DIM));
                let mut master = e.master() * 100.0;
                if ui
                    .add(
                        egui::Slider::new(&mut master, 0.0..=100.0)
                            .integer()
                            .suffix("%"),
                    )
                    .changed()
                {
                    e.set_master(master / 100.0);
                }
                ui.label(egui::RichText::new("Master").color(theme::TEXT_DIM));
                ui.separator();
                let (text, color) = match &status {
                    OutputStatus::Running { fps, .. } => (format!("{fps:.0} fps"), theme::OK),
                    OutputStatus::Connecting => ("verbinde".into(), theme::WARN),
                    OutputStatus::Error(_) => ("Fehler".into(), theme::DANGER),
                    OutputStatus::Stopped => ("gestoppt".into(), theme::TEXT_DIM),
                };
                let r = ui.label(egui::RichText::new(text).color(color));
                if let OutputStatus::Error(msg) = &status {
                    r.on_hover_text(msg);
                }
                egui::ComboBox::from_id_salt("output_kind")
                    .selected_text(self.output_kind.label())
                    .show_ui(ui, |ui| {
                        for k in OutputKind::ALL {
                            ui.selectable_value(&mut new_kind, k, k.label());
                        }
                    });
            });
        });
        drop(e);
        if new_kind != self.output_kind {
            self.switch_output(new_kind);
        }
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
                ui.separator();
                let e = Self::lock(&self.engine);
                ui.label(
                    egui::RichText::new(format!(
                        "{} Geräte · {} Szenen · {} aktiv",
                        e.show().fixtures.len(),
                        e.show().scenes.len(),
                        e.active_scenes().len()
                    ))
                    .color(theme::TEXT_DIM),
                );
            });
        });
    }
}

fn file_menu(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
    ui.menu_button("Datei", |ui| {
        if ui.button("Neue Show").clicked() {
            e.replace_show(demo::empty_show());
            st.reset_for_new_show();
            st.info("Neue, leere Show");
            ui.close();
        }
        if ui.button("Demo-Show laden").clicked() {
            e.replace_show(demo::demo_show());
            st.reset_for_new_show();
            st.info("Demo-Show geladen");
            ui.close();
        }
        ui.separator();
        ui.label(egui::RichText::new("Show-Datei").color(theme::TEXT_DIM));
        ui.add(egui::TextEdit::singleline(&mut st.show_path).desired_width(260.0));
        ui.horizontal(|ui| {
            if ui.button("Öffnen").clicked() {
                match Show::load(&st.show_path) {
                    Ok(mut s) => {
                        demo::ensure_builtin_types(&mut s);
                        let name = s.name.clone();
                        e.replace_show(s);
                        st.reset_for_new_show();
                        st.info(format!("Show „{name}“ geladen"));
                    }
                    Err(err) => st.error(err.to_string()),
                }
                ui.close();
            }
            if ui.button("Speichern").clicked() {
                match e.show().save(&st.show_path) {
                    Ok(()) => st.info(format!("Gespeichert: {}", st.show_path)),
                    Err(err) => st.error(err.to_string()),
                }
                ui.close();
            }
        });
    });
}

impl eframe::App for DmxCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Laufende Szenen und Live-Farben flüssig anzeigen.
        ui.ctx().request_repaint_after(Duration::from_millis(33));
        if !self.started {
            self.started = true;
            theme::apply(ui.ctx());
            self.logo = crate::icon::texture(ui.ctx());
            self.updater.check(ui.ctx().clone());
        }

        egui::Panel::top("header")
            .frame(
                egui::Frame::new()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(10, 6)),
            )
            .show(ui, |ui| self.header(ui));
        egui::Panel::bottom("status")
            .frame(
                egui::Frame::new()
                    .fill(theme::BG)
                    .inner_margin(egui::Margin::symmetric(10, 4)),
            )
            .show(ui, |ui| self.status_bar(ui));

        let frame = self.runtime.last_frame();
        let engine = self.engine.clone();
        let mut e = Self::lock(&engine);
        self.library.show(ui.ctx(), &mut e, &mut self.state);
        let mut cx = ViewCtx {
            engine: &mut e,
            state: &mut self.state,
            frame: &frame,
        };
        let pad = egui::Frame::new()
            .fill(theme::SURFACE)
            .inner_margin(theme::SP_S);

        match cx.state.mode {
            Mode::Setup => {
                egui::CentralPanel::default_margins()
                    .frame(pad)
                    .show(ui, |ui| self.setup.ui(ui, &mut cx));
            }
            Mode::Control => {
                let control = &mut self.control;
                egui::Panel::bottom("live_bottom")
                    .resizable(true)
                    .default_size(380.0)
                    .min_size(220.0)
                    .frame(pad)
                    .show(ui, |ui| {
                        egui::Panel::right("programmer_panel")
                            .resizable(true)
                            .default_size(620.0)
                            .min_size(320.0)
                            .frame(egui::Frame::new().inner_margin(egui::Margin {
                                left: 10,
                                ..Default::default()
                            }))
                            .show(ui, |ui| control.programmer.ui(ui, &mut cx));
                        egui::CentralPanel::default_margins()
                            .frame(egui::Frame::new())
                            .show(ui, |ui| control.stage.ui(ui, &mut cx));
                    });
                egui::CentralPanel::default_margins()
                    .frame(pad)
                    .show(ui, |ui| control.grid.ui(ui, &mut cx));
            }
        }
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
