//! Gemeinsamer UI-Zustand, den mehrere Ansichten brauchen. Ansichtsspezifischer
//! Zustand liegt in den jeweiligen View-Typen.

use std::collections::BTreeSet;

use dmxcraft_core::{FixtureId, FixtureType, SceneId, ZoneId};

/// Hauptbereich der Oberfläche.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Setup,
    Control,
}

/// Formular „Geräte hinzufügen“ (geteilt zwischen Patch und Gerätebibliothek).
pub struct PatchForm {
    pub name: String,
    pub count: u16,
    /// 1-basiert in der Oberfläche.
    pub universe: u8,
    pub address: u16,
}

pub struct AppState {
    pub mode: Mode,
    /// Oberfläche gesperrt: nur Steuern, keine Änderungen an Patch und Szenen.
    pub locked: bool,
    pub show_path: String,
    pub message: Option<(String, bool)>,
    /// Zeitpunkt der letzten Meldung (Meldungen blenden nach einigen Sekunden aus).
    pub message_time: Option<std::time::Instant>,
    pub selected: BTreeSet<FixtureId>,
    pub patch: PatchForm,
    /// Per Datei importierte Gerätetypen (noch nicht gepatcht).
    pub imported_types: Vec<FixtureType>,
    /// Szenen-Editor als Fenster geöffnet.
    pub scene_editor_open: bool,
    /// Fenster „Zonen & Gruppen“ geöffnet.
    pub zones_open: bool,
    /// Gerät, das im Patch-Raster umbenannt werden soll.
    pub rename_request: Option<FixtureId>,
    pub edit_scene: Option<SceneId>,
    pub append_target: Option<SceneId>,
    pub new_scene_name: String,
    pub new_scene_zone: Option<ZoneId>,
    /// Kopiertes Bühnenplan-Layout (Positionen relativ zum ersten Gerät).
    pub layout_clipboard: Option<Vec<[f32; 2]>>,
    /// Gerät, das der DMX-Patch beim nächsten Öffnen zeigen soll.
    pub monitor_focus: Option<FixtureId>,
    /// Patch-Raster auf „Ausgang“ umschalten (Tools → DMX-Werte).
    pub show_dmx_levels: bool,
}

impl AppState {
    pub fn new(show_path: String) -> Self {
        Self {
            mode: Mode::Control,
            locked: false,
            show_path,
            message: None,
            message_time: None,
            selected: BTreeSet::new(),
            patch: PatchForm {
                name: String::new(),
                count: 1,
                universe: 1,
                address: 1,
            },
            imported_types: Vec::new(),
            scene_editor_open: false,
            zones_open: false,
            rename_request: None,
            edit_scene: None,
            append_target: None,
            new_scene_name: "Neue Szene".into(),
            new_scene_zone: None,
            layout_clipboard: None,
            monitor_focus: None,
            show_dmx_levels: false,
        }
    }

    pub fn info(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), false));
        self.message_time = Some(std::time::Instant::now());
    }

    pub fn error(&mut self, msg: impl Into<String>) {
        self.message = Some((msg.into(), true));
        self.message_time = Some(std::time::Instant::now());
    }

    /// Aktuell anzuzeigende Meldung (Hinweise 5 s, Fehler 10 s).
    pub fn current_message(&self) -> Option<(&str, bool)> {
        let (msg, err) = self.message.as_ref()?;
        let age = self.message_time?.elapsed().as_secs_f32();
        (age < if *err { 10.0 } else { 5.0 }).then_some((msg.as_str(), *err))
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
        self.scene_editor_open = true;
    }
}
