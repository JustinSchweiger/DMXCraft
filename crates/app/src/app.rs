use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use eframe::egui;
use lightcraft_core::{Engine, FixtureId, SceneId, Show, ZoneId};
use lightcraft_output::{OutputKind, OutputStatus, Runtime};

use crate::demo;
use crate::views;

/// DMX-Wiederholrate des Ausgabe-Threads.
pub const OUTPUT_HZ: u32 = 40;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Live,
    Programmer,
    Scenes,
    Monitor,
}

impl Tab {
    const ALL: [Tab; 4] = [Tab::Live, Tab::Programmer, Tab::Scenes, Tab::Monitor];
    fn label(self) -> &'static str {
        match self {
            Tab::Live => "Live",
            Tab::Programmer => "Programmer",
            Tab::Scenes => "Szenen",
            Tab::Monitor => "DMX-Monitor",
        }
    }
}

/// Formular-Zustand für „Geräte hinzufügen“.
pub struct PatchForm {
    pub type_key: String,
    pub name: String,
    pub count: u16,
    /// 1-basiert in der Oberfläche.
    pub universe: u8,
    pub address: u16,
}

/// Zustand der Oberfläche (nicht Teil der Show-Datei).
pub struct UiState {
    pub tab: Tab,
    pub show_path: String,
    pub message: Option<(String, bool)>,
    pub patch: PatchForm,
    pub selected: BTreeSet<FixtureId>,
    pub new_scene_name: String,
    pub new_scene_zone: Option<ZoneId>,
    pub append_target: Option<SceneId>,
    pub edit_scene: Option<SceneId>,
    pub new_zone_name: String,
    pub monitor_universe: usize,
    pub library: views::library::LibraryState,
}

impl UiState {
    pub fn info(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), false));
    }
    pub fn error(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), true));
    }
}

pub struct LightCraftApp {
    engine: Arc<Mutex<Engine>>,
    runtime: Runtime,
    output_kind: OutputKind,
    pub ui: UiState,
}

impl LightCraftApp {
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
        Self {
            engine,
            runtime,
            output_kind,
            ui: UiState {
                tab: Tab::Live,
                show_path: show_path.unwrap_or_else(|| "show.json".into()),
                message,
                patch: PatchForm {
                    type_key: first_type,
                    name: "PAR".into(),
                    count: 1,
                    universe: 1,
                    address: 1,
                },
                selected: BTreeSet::new(),
                new_scene_name: "Neue Szene".into(),
                new_scene_zone: None,
                append_target: None,
                edit_scene: None,
                new_zone_name: "Neue Zone".into(),
                monitor_universe: 0,
                library: Default::default(),
            },
        }
    }

    fn engine(&self) -> MutexGuard<'_, Engine> {
        // Ein vergifteter Mutex heißt: der Ausgabe-Thread ist abgestürzt.
        // Die Daten sind trotzdem konsistent genug, um weiterzuarbeiten.
        self.engine.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn switch_output(&mut self, kind: OutputKind) {
        if kind == self.output_kind {
            return;
        }
        self.runtime.stop();
        self.runtime = Runtime::start(self.engine.clone(), kind.create(), OUTPUT_HZ);
        self.output_kind = kind;
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        let mut new_kind = self.output_kind;
        let status = self.runtime.status();
        let engine = self.engine.clone();
        let mut e = engine.lock().unwrap_or_else(|p| p.into_inner());
        let st = &mut self.ui;

        ui.horizontal_wrapped(|ui| {
            ui.strong("LightCraft");
            ui.separator();

            // Datei
            if ui.button("Neu").clicked() {
                e.replace_show(demo::empty_show());
                st.selected.clear();
                st.edit_scene = None;
                st.info("Neue, leere Show");
            }
            if ui.button("Demo").clicked() {
                e.replace_show(demo::demo_show());
                st.selected.clear();
                st.edit_scene = None;
                st.info("Demo-Show geladen");
            }
            ui.add(egui::TextEdit::singleline(&mut st.show_path).desired_width(180.0));
            if ui.button("Öffnen").clicked() {
                match Show::load(&st.show_path) {
                    Ok(mut s) => {
                        demo::ensure_builtin_types(&mut s);
                        let name = s.name.clone();
                        e.replace_show(s);
                        st.selected.clear();
                        st.edit_scene = None;
                        st.info(format!("Show „{name}“ geladen"));
                    }
                    Err(err) => st.error(err.to_string()),
                }
            }
            if ui.button("Speichern").clicked() {
                match e.show().save(&st.show_path) {
                    Ok(()) => st.info(format!("Gespeichert: {}", st.show_path)),
                    Err(err) => st.error(err.to_string()),
                }
            }
            ui.separator();

            // Ausgang
            egui::ComboBox::from_id_salt("output_kind")
                .selected_text(self.output_kind.label())
                .show_ui(ui, |ui| {
                    for k in OutputKind::ALL {
                        ui.selectable_value(&mut new_kind, k, k.label());
                    }
                });
            let (text, color) = match &status {
                OutputStatus::Running { fps, .. } => (
                    format!("Ausgang aktiv · {fps:.0} fps"),
                    egui::Color32::from_rgb(80, 200, 120),
                ),
                OutputStatus::Connecting => ("verbinde…".into(), egui::Color32::YELLOW),
                OutputStatus::Error(_) => (
                    "Ausgang: Fehler".into(),
                    egui::Color32::from_rgb(230, 80, 80),
                ),
                OutputStatus::Stopped => ("gestoppt".into(), egui::Color32::GRAY),
            };
            let r = ui.colored_label(color, text);
            if let OutputStatus::Error(msg) = &status {
                r.on_hover_text(msg);
            }
            ui.separator();

            // Master, Speed, Blackout
            let mut master = e.master() * 100.0;
            if ui
                .add(
                    egui::Slider::new(&mut master, 0.0..=100.0)
                        .text("Master %")
                        .integer(),
                )
                .changed()
            {
                e.set_master(master / 100.0);
            }
            let mut speed = e.speed();
            if ui
                .add(
                    egui::Slider::new(&mut speed, 0.1..=4.0)
                        .text("Speed")
                        .logarithmic(true)
                        .fixed_decimals(2),
                )
                .changed()
            {
                e.set_speed(speed);
            }
            let bo = e.blackout();
            let bo_btn =
                egui::Button::new(if bo { "BLACKOUT AN" } else { "Blackout" }).fill(if bo {
                    egui::Color32::from_rgb(170, 30, 30)
                } else {
                    ui.visuals().widgets.inactive.weak_bg_fill
                });
            if ui.add(bo_btn).clicked() {
                e.set_blackout(!bo);
            }
            if ui.button("Alle Szenen aus").clicked() {
                e.release_all();
            }
        });
        drop(e);
        if new_kind != self.output_kind {
            self.switch_output(new_kind);
        }
    }
}

impl eframe::App for LightCraftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Laufende Szenen/Monitor flüssig anzeigen.
        ui.ctx().request_repaint_after(Duration::from_millis(33));

        egui::Panel::top("top_bar").show(ui, |ui| {
            ui.add_space(4.0);
            self.top_bar(ui);
            ui.add_space(4.0);
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                match &self.ui.message {
                    Some((msg, true)) => {
                        ui.colored_label(egui::Color32::from_rgb(230, 80, 80), msg);
                    }
                    Some((msg, false)) => {
                        ui.label(msg);
                    }
                    None => {
                        ui.weak("Bereit");
                    }
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let e = self.engine();
                    ui.weak(format!(
                        "{} Geräte · {} Szenen · {} aktiv",
                        e.show().fixtures.len(),
                        e.show().scenes.len(),
                        e.active_scenes().len()
                    ));
                });
            });
        });

        let frame = self.runtime.last_frame();
        let engine = self.engine.clone();

        egui::Panel::left("patch")
            .default_size(340.0)
            .show(ui, |ui| {
                let mut e = engine.lock().unwrap_or_else(|p| p.into_inner());
                views::patch::ui(ui, &mut e, &mut self.ui);
            });

        {
            let mut e = engine.lock().unwrap_or_else(|p| p.into_inner());
            views::library::window(ui.ctx(), &mut e, &mut self.ui);
        }

        egui::CentralPanel::default_margins().show(ui, |ui| {
            ui.horizontal(|ui| {
                for t in Tab::ALL {
                    ui.selectable_value(&mut self.ui.tab, t, t.label());
                }
            });
            ui.separator();
            let mut e = engine.lock().unwrap_or_else(|p| p.into_inner());
            match self.ui.tab {
                Tab::Live => views::live::ui(ui, &mut e, &mut self.ui),
                Tab::Programmer => views::programmer::ui(ui, &mut e, &mut self.ui, &frame),
                Tab::Scenes => views::scenes::ui(ui, &mut e, &mut self.ui),
                Tab::Monitor => views::monitor::ui(ui, &e, &mut self.ui, &frame),
            }
        });
    }
}
