//! Gemeinsamer UI-Zustand, den mehrere Ansichten brauchen. Ansichtsspezifischer
//! Zustand liegt in den jeweiligen View-Typen.

use std::collections::BTreeSet;

use dmxcraft_core::{FixtureId, FixtureType, SceneId};

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
    /// Markierte Szenen in STEUERN (für Löschen u. a.).
    pub selected_scenes: BTreeSet<SceneId>,
    /// Zuletzt verarbeitete Szenen-Markierung (Edit-Modus: Geräte mitwählen).
    pub scene_selection_seen: BTreeSet<SceneId>,
    /// Zuletzt verarbeiteter Programmer-Modus.
    pub programmer_mode_seen: ProgrammerMode,
    /// Edit-Werte neu aus der Szene laden (nach Rückgängig/Wiederholen).
    pub edit_reload: bool,
    /// Betriebsmodus des Programmers.
    pub programmer_mode: ProgrammerMode,
    pub patch: PatchForm,
    /// Per Datei importierte Gerätetypen (noch nicht gepatcht).
    pub imported_types: Vec<FixtureType>,
    /// Fenster „Zonen & Gruppen“ geöffnet.
    pub zones_open: bool,
    /// Gerät, das im Patch-Raster umbenannt werden soll.
    pub rename_request: Option<FixtureId>,
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
            selected_scenes: BTreeSet::new(),
            programmer_mode: ProgrammerMode::default(),
            scene_selection_seen: BTreeSet::new(),
            programmer_mode_seen: ProgrammerMode::default(),
            edit_reload: false,
            patch: PatchForm {
                name: String::new(),
                count: 1,
                universe: 1,
                address: 1,
            },
            imported_types: Vec::new(),
            zones_open: false,
            rename_request: None,
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
    }
}

/// Betriebsmodus des Programmers (wie sich Änderungen an Geräten auswirken).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgrammerMode {
    /// Werte bearbeiten und sofort ausgeben.
    Edit,
    /// Werte bearbeiten, ohne dass sie ausgegeben werden.
    Blind,
    /// Live-Betrieb (Start-Modus).
    #[default]
    Live,
}
