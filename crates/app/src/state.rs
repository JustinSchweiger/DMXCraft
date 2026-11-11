//! Shared UI state needed by several views. View-specific state lives in
//! the respective view types.

use std::collections::BTreeSet;

use dmxcraft_core::{FixtureId, FixtureType, SceneId};

/// Main area of the interface.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    Setup,
    Control,
    /// Touch surface (freely arranged controls).
    Touch,
    /// 3D visualizer as a tab.
    Visualizer,
    /// Profile Builder as a tab.
    Builder,
    /// Sound player with playlists and jingle pads.
    Sound,
}

impl Mode {
    /// Tabs in the title bar, in their order.
    pub const TABS: [Mode; 6] = [
        Mode::Setup,
        Mode::Control,
        Mode::Touch,
        Mode::Sound,
        Mode::Visualizer,
        Mode::Builder,
    ];

    /// Key in the settings (hidden tabs).
    pub fn key(self) -> &'static str {
        match self {
            Mode::Setup => "setup",
            Mode::Control => "control",
            Mode::Touch => "touch",
            Mode::Sound => "sound",
            Mode::Visualizer => "visualizer",
            Mode::Builder => "builder",
        }
    }

    /// Tab label (English source, translated via `i18n::t`).
    pub fn label(self) -> &'static str {
        match self {
            Mode::Setup => "Setup",
            Mode::Control => "Control",
            Mode::Touch => "Touch",
            Mode::Sound => "Sound",
            Mode::Visualizer => "3D visualizer",
            Mode::Builder => "Profile builder",
        }
    }

    /// Not available while the program is locked.
    pub fn needs_unlock(self) -> bool {
        matches!(self, Mode::Setup | Mode::Builder)
    }
}

/// “Add fixtures” form (shared between patch and fixture library).
pub struct PatchForm {
    pub name: String,
    pub count: u16,
    /// 1-based in the interface.
    pub universe: u8,
    pub address: u16,
}

pub struct AppState {
    pub mode: Mode,
    /// Interface locked: control only, no changes to patch and scenes.
    pub locked: bool,
    pub show_path: String,
    pub message: Option<(String, bool)>,
    /// Time of the last message (messages fade out after a few seconds).
    pub message_time: Option<std::time::Instant>,
    pub selected: BTreeSet<FixtureId>,
    /// Order of selection (first selected first) – determines the numbering on
    /// the stage and the beam order of new effects.
    pub selection_order: Vec<FixtureId>,
    /// Selected scenes in CONTROL (for deleting etc.).
    pub selected_scenes: BTreeSet<SceneId>,
    /// Mapping mode (overlay and window) – keyboard or MIDI.
    pub mapping: Option<dmxcraft_core::mapping::MapKind>,
    /// Selected mapping in the mapping window.
    pub map_sel: Option<u32>,
    /// Waiting for the next key or MIDI message.
    pub map_learn: bool,
    /// Bank shown in the scene grid (`None` = all).
    pub shown_bank: Option<dmxcraft_core::ZoneId>,
    /// Request to the scene grid to show a bank.
    pub request_bank: Option<Option<dmxcraft_core::ZoneId>>,
    /// Touch: displayed page, edit mode, selected element.
    pub touch_page: Option<u32>,
    pub touch_edit: bool,
    pub touch_sel: Option<u32>,
    /// All selected elements (multi-selection, contains `touch_sel`).
    pub touch_multi: Vec<u32>,
    /// Touch: triggered buttons/faders (mapping, pressed, value, knob) –
    /// processed by the mapping frame at the end of the frame.
    pub touch_fire: Vec<(dmxcraft_core::mapping::Mapping, bool, u8, bool)>,
    /// Last set values of touch faders/knobs without a readable state (touch
    /// mode and remote share them).
    pub touch_values: std::collections::HashMap<u32, f32>,
    /// Last processed scene selection (edit mode: select fixtures too).
    pub scene_selection_seen: BTreeSet<SceneId>,
    /// Last processed programmer mode.
    pub programmer_mode_seen: ProgrammerMode,
    /// Reload the edit values from the scene (after undo/redo).
    pub edit_reload: bool,
    /// Programmer values at the last edit sync: only a change against these
    /// is written into the scene, so changes from elsewhere (MCP, remote)
    /// are not overwritten by stale programmer values.
    pub edit_programmer: Option<Vec<dmxcraft_core::ChannelValue>>,
    /// Edited step of the selected scene (edit mode).
    pub edit_step: usize,
    pub edit_step_seen: usize,
    /// Last clicked effect (scene, effect): target of the stage's FX button.
    pub fx_target: Option<(SceneId, u32)>,
    /// Timeline (SUPER SCENE): selected blocks and play position (ms).
    pub timeline_sel: BTreeSet<u32>,
    /// Last clicked scene: start of a Shift selection.
    pub scene_anchor: Option<SceneId>,
    /// Wrap scenes at the bottom into more columns (setting).
    pub wrap_scenes: bool,
    /// Value ranges on the fader tracks (setting).
    pub fader_ranges: bool,
    /// Quality of the 3D visualizer (setting).
    pub viz_quality: crate::viz::Quality,
    /// 3D visualizer in its own window.
    pub viz_window: bool,
    /// History: are there steps back/forward? (set by the app frame every frame)
    pub can_undo: bool,
    pub can_redo: bool,
    /// A view asks for undo (`true`) or redo (`false`).
    pub history_request: Option<bool>,
    /// Something is being dragged or typed in a separate window (3D, Profile
    /// Builder) – undo only takes a step afterwards.
    pub viewport_busy: bool,
    pub timeline_cursor: f64,
    /// Audio playback and waveforms.
    pub audio: crate::audio::AudioPlayer,
    /// Connection to OBS (video blocks, OBS actions).
    pub obs: crate::obs::Obs,
    /// Playback of the SOUND tab.
    pub sound: crate::sound::SoundPlayer,
    /// Audio triggers of the SUPER SCENES (envelopes, events, pressed targets).
    pub triggers: crate::audio_triggers::TriggerEngine,
    /// The target of an audio trigger is being chosen (scene, block, trigger):
    /// the next Alt + click goes to it instead of to the touch surface.
    pub trigger_pick: Option<(dmxcraft_core::SceneId, u32, u32)>,
    /// MOVE FX point being edited (scene, effect, pan/tilt): its beams stay
    /// there. Set by the effect editor every frame, applied in the next one.
    pub move_hold: Option<(dmxcraft_core::SceneId, u32, [f32; 2])>,
    /// SCENE GROUP shown in the sidebar (instead of the scene).
    pub selected_group: Option<u32>,
    /// Missing audio files of the SUPER SCENES (check and window).
    pub missing: crate::missing_audio::MissingAudio,
    /// What the Delete key acts on in CONTROL (last clicked).
    pub focus: Focus,
    /// Operating mode of the programmer.
    pub programmer_mode: ProgrammerMode,
    pub patch: PatchForm,
    /// Fixture types imported from a file (not patched yet).
    pub imported_types: Vec<FixtureType>,
    /// “Zones & groups” window open.
    pub zones_open: bool,
    /// Custom fixture profiles (Profile Builder) with their fixture types.
    pub user_profiles: Vec<crate::profiles::UserProfile>,
    /// Open the Profile Builder (menu, library).
    pub builder: Option<crate::views::profile_builder::BuilderRequest>,
    /// Fixture to be renamed in the patch grid.
    pub rename_request: Option<FixtureId>,
    /// Copied stage plan layout (positions relative to the first fixture).
    pub layout_clipboard: Option<Vec<[f32; 2]>>,
    /// Cut/copied fixtures (patch: Ctrl+X/C/V).
    pub fixture_clipboard: Vec<dmxcraft_core::Fixture>,
    /// Copied look of a fixture (stage view: copy/paste look).
    pub look_clipboard: Option<dmxcraft_core::look::Look>,
    /// Last clicked cell in the patch (universe, address): target for pasting.
    pub patch_cursor: Option<(u8, u16)>,
    /// Fixture the DMX patch should show the next time it opens.
    pub monitor_focus: Option<FixtureId>,
    /// Switch the patch grid to “Output” (Tools → DMX values).
    pub show_dmx_levels: bool,
}

impl AppState {
    /// Adapt the selection order to the selection: deselected ones drop out,
    /// newly selected ones are appended (several at once in patch order).
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

    /// Set the selection in a fixed order.
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
            touch_multi: Vec::new(),
            touch_fire: Vec::new(),
            touch_values: Default::default(),
            programmer_mode: ProgrammerMode::default(),
            focus: Focus::default(),
            scene_selection_seen: BTreeSet::new(),
            programmer_mode_seen: ProgrammerMode::default(),
            edit_reload: false,
            edit_programmer: None,
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
            obs: Default::default(),
            sound: Default::default(),
            triggers: Default::default(),
            trigger_pick: None,
            move_hold: None,
            selected_group: None,
            missing: Default::default(),
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
            look_clipboard: None,
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

    /// Message to show right now (hints 5 s, errors 10 s).
    pub fn current_message(&self) -> Option<(&str, bool)> {
        let (msg, err) = self.message.as_ref()?;
        let age = self.message_time?.elapsed().as_secs_f32();
        (age < if *err { 10.0 } else { 5.0 }).then_some((msg.as_str(), *err))
    }

    /// After loading/replacing a show.
    pub fn reset_for_new_show(&mut self) {
        self.selected.clear();
        self.sound.stop_all();
    }
}

/// Operating mode of the programmer (how changes to fixtures take effect).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProgrammerMode {
    /// Edit values and output them immediately.
    Edit,
    /// Like Edit, but the output stays as it was when switching; stage and
    /// faders show the edit.
    Blind,
    /// Live operation (start mode).
    #[default]
    Live,
}

impl ProgrammerMode {
    /// Edit and Blind Edit edit the selected scene.
    pub fn is_edit(self) -> bool {
        matches!(self, ProgrammerMode::Edit | ProgrammerMode::Blind)
    }
}

/// Drag & drop payload: fixture type from the library dragged into the patch.
#[derive(Clone)]
pub struct LibraryDrag(pub FixtureType);

/// Last clicked element in CONTROL (target of the Delete key).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Focus {
    /// Selected scenes.
    #[default]
    Scenes,
    /// A bank (tab of the bank bar).
    Bank(dmxcraft_core::ZoneId),
    /// An effect of a scene (sidebar).
    Effect(SceneId, u32),
    /// Timeline of a SUPER SCENE (blocks).
    Timeline,
}
