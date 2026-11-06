//! Gemeinsamer UI-Zustand, den mehrere Ansichten brauchen. Ansichtsspezifischer
//! Zustand liegt in den jeweiligen View-Typen.

use std::collections::BTreeSet;

use dmxcraft_core::{FixtureId, SceneId, ZoneId};

/// Hauptbereich der Oberfläche.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Setup,
    Control,
}

/// Unterbereiche von EINRICHTEN.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetupTab {
    Patch,
    Stage,
    Scenes,
    Zones,
    Monitor,
}

/// Formular „Geräte hinzufügen“ (geteilt zwischen Patch und Gerätebibliothek).
pub struct PatchForm {
    pub type_key: String,
    pub name: String,
    pub count: u16,
    /// 1-basiert in der Oberfläche.
    pub universe: u8,
    pub address: u16,
}

pub struct AppState {
    pub mode: Mode,
    pub setup_tab: SetupTab,
    pub show_path: String,
    pub message: Option<(String, bool)>,
    pub selected: BTreeSet<FixtureId>,
    pub patch: PatchForm,
    pub library_open: bool,
    pub edit_scene: Option<SceneId>,
    pub append_target: Option<SceneId>,
    pub new_scene_name: String,
    pub new_scene_zone: Option<ZoneId>,
}

impl AppState {
    pub fn new(show_path: String, first_type: String) -> Self {
        Self {
            mode: Mode::Control,
            setup_tab: SetupTab::Patch,
            show_path,
            message: None,
            selected: BTreeSet::new(),
            patch: PatchForm {
                type_key: first_type,
                name: "PAR".into(),
                count: 1,
                universe: 1,
                address: 1,
            },
            library_open: false,
            edit_scene: None,
            append_target: None,
            new_scene_name: "Neue Szene".into(),
            new_scene_zone: None,
        }
    }

    pub fn info(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), false));
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), true));
    }

    /// Nach dem Laden/Ersetzen einer Show.
    pub fn reset_for_new_show(&mut self) {
        self.selected.clear();
        self.edit_scene = None;
        self.append_target = None;
    }

    /// Szene im Szenen-Editor öffnen.
    pub fn open_scene_editor(&mut self, id: SceneId) {
        self.edit_scene = Some(id);
        self.mode = Mode::Setup;
        self.setup_tab = SetupTab::Scenes;
    }
}
