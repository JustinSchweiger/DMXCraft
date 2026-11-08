//! Gemeinsamer UI-Zustand, den mehrere Ansichten brauchen. Ansichtsspezifischer
//! Zustand liegt in den jeweiligen View-Typen.

use std::collections::BTreeSet;

use dmxcraft_core::{FixtureId, FixtureType, SceneId};

/// Hauptbereich der Oberfläche.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Setup,
    Control,
    /// Touch-Oberfläche (frei angeordnete Bedienelemente).
    Touch,
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
    /// Reihenfolge der Auswahl (zuerst gewählte zuerst) – bestimmt die
    /// Nummerierung auf der Bühne und die Beam-Reihenfolge neuer Effekte.
    pub selection_order: Vec<FixtureId>,
    /// Markierte Szenen in STEUERN (für Löschen u. a.).
    pub selected_scenes: BTreeSet<SceneId>,
    /// Zuordnungsmodus (Überlagerung und Fenster) – Tastatur oder MIDI.
    pub mapping: Option<dmxcraft_core::mapping::MapKind>,
    /// Gewählte Zuordnung im Zuordnungsfenster.
    pub map_sel: Option<u32>,
    /// Wartet auf die nächste Taste bzw. MIDI-Nachricht.
    pub map_learn: bool,
    /// Im Szenenraster angezeigte Bank (`None` = alle).
    pub shown_bank: Option<dmxcraft_core::ZoneId>,
    /// Anfrage an das Szenenraster, eine Bank anzuzeigen.
    pub request_bank: Option<Option<dmxcraft_core::ZoneId>>,
    /// Touch: angezeigte Seite, Bearbeiten-Modus, gewähltes Element.
    pub touch_page: Option<u32>,
    pub touch_edit: bool,
    pub touch_sel: Option<u32>,
    /// Touch: ausgelöste Schaltflächen/Fader (Zuordnung, gedrückt, Wert,
    /// Regler) – verarbeitet der Zuordnungs-Rahmen am Ende des Bilds.
    pub touch_fire: Vec<(dmxcraft_core::mapping::Mapping, bool, u8, bool)>,
    /// Zuletzt gesetzte Werte von Touch-Fadern/-Reglern ohne lesbaren
    /// Zustand (Touch-Modus und Fernbedienung teilen sie).
    pub touch_values: std::collections::HashMap<u32, f32>,
    /// Zuletzt verarbeitete Szenen-Markierung (Edit-Modus: Geräte mitwählen).
    pub scene_selection_seen: BTreeSet<SceneId>,
    /// Zuletzt verarbeiteter Programmer-Modus.
    pub programmer_mode_seen: ProgrammerMode,
    /// Edit-Werte neu aus der Szene laden (nach Rückgängig/Wiederholen).
    pub edit_reload: bool,
    /// Bearbeiteter Schritt der markierten Szene (Edit-Modus).
    pub edit_step: usize,
    pub edit_step_seen: usize,
    /// Zuletzt angeklickter Effekt (Szene, Effekt): Ziel des FX-Knopfs der Bühne.
    pub fx_target: Option<(SceneId, u32)>,
    /// Zeitleiste (SUPER SCENE): ausgewählte Blöcke und Abspielposition (ms).
    pub timeline_sel: BTreeSet<u32>,
    /// Zuletzt angeklickte Szene: Anfang einer Umschalt-Auswahl.
    pub scene_anchor: Option<SceneId>,
    /// Szenen am unteren Rand in weitere Spalten umbrechen (Einstellung).
    pub wrap_scenes: bool,
    /// Wertebereiche auf den Fader-Schienen (Einstellung).
    pub fader_ranges: bool,
    /// Qualität des 3D-Visualisierers (Einstellung).
    pub viz_quality: crate::viz::Quality,
    /// 3D-Visualisierer im eigenen Fenster.
    pub viz_window: bool,
    /// Verlauf: gibt es Schritte zurück/vor? (vom App-Rahmen je Bild gesetzt)
    pub can_undo: bool,
    pub can_redo: bool,
    /// Ansicht bittet um Rückgängig (`true`) bzw. Wiederholen (`false`).
    pub history_request: Option<bool>,
    /// In einem eigenen Fenster (3D, Profile Builder) wird gerade gezogen
    /// oder getippt – Rückgängig übernimmt erst danach einen Schritt.
    pub viewport_busy: bool,
    pub timeline_cursor: f64,
    /// Audiowiedergabe und Wellenformen.
    pub audio: crate::audio::AudioPlayer,
    /// Worauf die Entf-Taste unter STEUERN wirkt (zuletzt angeklickt).
    pub focus: Focus,
    /// Betriebsmodus des Programmers.
    pub programmer_mode: ProgrammerMode,
    pub patch: PatchForm,
    /// Per Datei importierte Gerätetypen (noch nicht gepatcht).
    pub imported_types: Vec<FixtureType>,
    /// Fenster „Zonen & Gruppen“ geöffnet.
    pub zones_open: bool,
    /// Eigene Geräteprofile (Profile Builder) mit ihren Gerätetypen.
    pub user_profiles: Vec<crate::profiles::UserProfile>,
    /// Profile Builder öffnen (Menü, Bibliothek).
    pub builder: Option<crate::views::profile_builder::BuilderRequest>,
    /// Gerät, das im Patch-Raster umbenannt werden soll.
    pub rename_request: Option<FixtureId>,
    /// Kopiertes Bühnenplan-Layout (Positionen relativ zum ersten Gerät).
    pub layout_clipboard: Option<Vec<[f32; 2]>>,
    /// Ausgeschnittene/kopierte Geräte (Patch: Strg+X/C/V).
    pub fixture_clipboard: Vec<dmxcraft_core::Fixture>,
    /// Zuletzt angeklickte Zelle im Patch (Universum, Adresse): Ziel für Einfügen.
    pub patch_cursor: Option<(u8, u16)>,
    /// Gerät, das der DMX-Patch beim nächsten Öffnen zeigen soll.
    pub monitor_focus: Option<FixtureId>,
    /// Patch-Raster auf „Ausgang“ umschalten (Tools → DMX-Werte).
    pub show_dmx_levels: bool,
}

impl AppState {
    /// Auswahlreihenfolge an die Auswahl anpassen: Abgewählte fallen heraus,
    /// neu Gewählte kommen hinten dazu (mehrere auf einmal in Patch-Reihenfolge).
    pub fn sync_selection_order(&mut self, show: &dmxcraft_core::Show) {
        let sel = &self.selected;
        self.selection_order.retain(|f| sel.contains(f));
        if self.selection_order.len() == sel.len() {
            return;
        }
        let known: BTreeSet<FixtureId> = self.selection_order.iter().copied().collect();
        let mut new: Vec<FixtureId> = sel.difference(&known).copied().collect();
        let pos = |f: &FixtureId| {
            show.fixtures
                .iter()
                .position(|x| x.id == *f)
                .unwrap_or(usize::MAX)
        };
        new.sort_by_key(pos);
        self.selection_order.extend(new);
    }

    /// Auswahl in fester Reihenfolge setzen.
    pub fn select_ordered(&mut self, ids: &[FixtureId]) {
        self.selected = ids.iter().copied().collect();
        self.selection_order = ids.to_vec();
    }

    pub fn new(show_path: String) -> Self {
        Self {
            mode: Mode::Control,
            locked: false,
            show_path,
            message: None,
            message_time: None,
            selected: BTreeSet::new(),
            selection_order: Vec::new(),
            selected_scenes: BTreeSet::new(),
            mapping: None,
            map_sel: None,
            map_learn: false,
            shown_bank: None,
            request_bank: None,
            touch_page: None,
            touch_edit: false,
            touch_sel: None,
            touch_fire: Vec::new(),
            touch_values: Default::default(),
            programmer_mode: ProgrammerMode::default(),
            focus: Focus::default(),
            scene_selection_seen: BTreeSet::new(),
            programmer_mode_seen: ProgrammerMode::default(),
            edit_reload: false,
            edit_step: 0,
            edit_step_seen: 0,
            fx_target: None,
            timeline_sel: BTreeSet::new(),
            scene_anchor: None,
            wrap_scenes: false,
            fader_ranges: true,
            viz_quality: crate::viz::Quality::default(),
            viz_window: false,
            can_undo: false,
            can_redo: false,
            history_request: None,
            viewport_busy: false,
            timeline_cursor: 0.0,
            audio: Default::default(),
            patch: PatchForm {
                name: String::new(),
                count: 1,
                universe: 1,
                address: 1,
            },
            imported_types: Vec::new(),
            zones_open: false,
            user_profiles: crate::profiles::load_user(),
            builder: None,
            rename_request: None,
            layout_clipboard: None,
            fixture_clipboard: Vec::new(),
            patch_cursor: None,
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
    /// Wie Edit, aber der Ausgang bleibt auf dem Stand beim Umschalten;
    /// Bühne und Fader zeigen die Bearbeitung.
    Blind,
    /// Live-Betrieb (Start-Modus).
    #[default]
    Live,
}

impl ProgrammerMode {
    /// Edit und Blind Edit bearbeiten die markierte Szene.
    pub fn is_edit(self) -> bool {
        matches!(self, ProgrammerMode::Edit | ProgrammerMode::Blind)
    }
}

/// Drag-&-Drop-Inhalt: Gerätetyp aus der Bibliothek, der ins Patch gezogen wird.
#[derive(Clone)]
pub struct LibraryDrag(pub FixtureType);

/// Zuletzt angeklicktes Element unter STEUERN (Ziel der Entf-Taste).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Focus {
    /// Markierte Szenen.
    #[default]
    Scenes,
    /// Eine Bank (Reiter der Bankleiste).
    Bank(dmxcraft_core::ZoneId),
    /// Ein Effekt einer Szene (Seitenleiste).
    Effect(SceneId, u32),
    /// Zeitleiste einer SUPER SCENE (Blöcke).
    Timeline,
}
