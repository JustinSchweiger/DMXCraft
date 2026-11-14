//! The show: everything that is saved. The file is JSON and contains a copy
//! of the fixture types used so it loads on every computer.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dmx::UNIVERSE_SIZE;
use crate::fixture::{ChannelDef, FixtureType};

pub const SHOW_FORMAT_VERSION: u32 = 1;

/// Fixed number of DMX universes of every show.
pub const UNIVERSES: u8 = 8;

macro_rules! id_type {
    ($name:ident) => {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub u32);
    };
}

id_type!(FixtureId);
id_type!(SceneId);
id_type!(ZoneId);
id_type!(GroupId);

/// A patched fixture.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: String,
    /// Key of the fixture type ([`FixtureType::key`]).
    pub type_key: String,
    /// 0-based (universe 1 in the UI = 0 here).
    pub universe: u8,
    /// Start address, 1-based (1..=512).
    pub address: u16,
    /// Position in the stage plan (0.0..=1.0 on both axes). `None` = automatic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_pos: Option<[f32; 2]>,
    /// Display in the stage plan. `None` = automatic by fixture type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_shape: Option<PlanShape>,
    /// Size factor in the stage plan (1.0 = normal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_size: Option<f32>,
    /// Rotation in the stage plan in degrees (clockwise).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_rotation: Option<f32>,
    /// Output limits (dimmer limit, pan/tilt range, invert).
    #[serde(default, skip_serializing_if = "FixtureLimits::is_default")]
    pub limits: FixtureLimits,
    /// Placement in the 3D visualizer. `None` = automatic from the stage plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place3d: Option<crate::venue::Placement>,
}

/// Limits applied to a fixture on output – independent of scenes and
/// programmer (e.g. to tame fixtures that are too bright or keep moving
/// heads away from the audience).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FixtureLimits {
    /// Lowest dimmer value once the fixture is on (0 = no limit).
    /// A dimmer value of 0 stays 0 so blackout and “off” work.
    pub dimmer_min: u8,
    /// Highest dimmer value (255 = no limit).
    pub dimmer_max: u8,
    pub pan_min: u8,
    pub pan_max: u8,
    pub tilt_min: u8,
    pub tilt_max: u8,
    pub invert_pan: bool,
    pub invert_tilt: bool,
    /// Swap pan and tilt (e.g. for fixtures mounted sideways).
    pub swap_pan_tilt: bool,
}

impl Default for FixtureLimits {
    fn default() -> Self {
        Self {
            dimmer_min: 0,
            dimmer_max: 255,
            pan_min: 0,
            pan_max: 255,
            tilt_min: 0,
            tilt_max: 255,
            invert_pan: false,
            invert_tilt: false,
            swap_pan_tilt: false,
        }
    }
}

impl FixtureLimits {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    pub fn affects_dimmer(&self) -> bool {
        self.dimmer_min != 0 || self.dimmer_max != 255
    }

    /// Map a dimmer value into the allowed range (0 stays 0).
    pub fn map_dimmer(&self, v: u8) -> u8 {
        if v == 0 {
            return 0;
        }
        let (lo, hi) = (
            u32::from(self.dimmer_min.min(self.dimmer_max)),
            u32::from(self.dimmer_max.max(self.dimmer_min)),
        );
        (lo + u32::from(v) * (hi - lo) / 255) as u8
    }

    pub fn affects_position(&self) -> bool {
        self.pan_min != 0
            || self.pan_max != 255
            || self.tilt_min != 0
            || self.tilt_max != 255
            || self.invert_pan
            || self.invert_tilt
            || self.swap_pan_tilt
    }

    /// Maps a 16-bit value (0..=65535) into the allowed range.
    pub fn map16(value: u16, min: u8, max: u8, invert: bool) -> u16 {
        let v = if invert { u16::MAX - value } else { value };
        let (lo, hi) = (u32::from(min) << 8, (u32::from(max) << 8) | 0xFF);
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        (lo + u32::from(v) * (hi - lo) / u32::from(u16::MAX)) as u16
    }
}

/// Shape of a fixture in the stage plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanShape {
    /// Round spotlight (PAR, spot, moving head). Older shows with “moving_head”
    /// are read as round.
    #[serde(alias = "moving_head")]
    Round,
    /// Square fixture (flood, panel).
    Square,
    /// Square pixel grid (matrix panel).
    Matrix,
    /// Horizontal bar (LED bar, pixel bar).
    BarHorizontal,
    /// Vertical bar.
    BarVertical,
    /// Effect / laser / other.
    Effect,
}

impl PlanShape {
    pub const ALL: [PlanShape; 6] = [
        PlanShape::Round,
        PlanShape::Square,
        PlanShape::Matrix,
        PlanShape::BarHorizontal,
        PlanShape::BarVertical,
        PlanShape::Effect,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PlanShape::Round => "Round (PAR/spot)",
            PlanShape::Square => "Square (flood/panel)",
            PlanShape::Matrix => "Pixel grid (matrix)",
            PlanShape::BarHorizontal => "Bar, horizontal",
            PlanShape::BarVertical => "Bar, vertical",
            PlanShape::Effect => "Effect/laser",
        }
    }

    /// Matching shape for a fixture type.
    pub fn guess(ty: &FixtureType) -> PlanShape {
        use crate::fixture::ChannelKind as K;
        let reds = ty.channels.iter().filter(|c| c.kind == K::Red).count();
        let moving = ty.has_kind(K::Pan) || ty.has_kind(K::Tilt);
        if moving {
            PlanShape::Round
        } else if reds >= 9 && (reds as f32).sqrt().fract() == 0.0 {
            PlanShape::Matrix
        } else if reds > 1 {
            PlanShape::BarHorizontal
        } else if reds == 1 || ty.has_kind(K::Dimmer) {
            PlanShape::Round
        } else {
            PlanShape::Effect
        }
    }
}

/// Allowed range for [`Fixture::plan_size`].
pub const PLAN_SIZE_RANGE: std::ops::RangeInclusive<f32> = 0.5..=4.0;

/// Reference to a channel of a fixture (channel index 0-based in the fixture
/// type).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChannelRef {
    pub fixture: FixtureId,
    pub channel: u16,
}

impl ChannelRef {
    /// Channel number of the virtual dimmer (fixtures with color channels but
    /// without their own dimmer). It is stored and merged like a channel but not
    /// output: it scales the fixture's color channels.
    pub const VIRTUAL_DIMMER: u16 = u16::MAX;

    pub fn virtual_dimmer(fixture: FixtureId) -> Self {
        Self {
            fixture,
            channel: Self::VIRTUAL_DIMMER,
        }
    }

    pub fn is_virtual(self) -> bool {
        self.channel == Self::VIRTUAL_DIMMER
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelValue {
    pub fixture: FixtureId,
    pub channel: u16,
    pub value: u8,
}

impl ChannelValue {
    pub fn channel_ref(&self) -> ChannelRef {
        ChannelRef {
            fixture: self.fixture,
            channel: self.channel,
        }
    }
}

/// A step of a scene: channel values, fade time and hold time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub values: Vec<ChannelValue>,
    /// Fade time from the previous step in milliseconds.
    pub fade_ms: u32,
    /// Hold time after the fade in milliseconds.
    pub hold_ms: u32,
}

impl Step {
    pub fn new(values: Vec<ChannelValue>) -> Self {
        Self {
            values,
            fade_ms: 0,
            hold_ms: 1000,
        }
    }

    pub fn duration_ms(&self) -> u64 {
        u64::from(self.fade_ms) + u64::from(self.hold_ms)
    }

    pub fn value_map(&self) -> HashMap<ChannelRef, u8> {
        self.values
            .iter()
            .map(|v| (v.channel_ref(), v.value))
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoopMode {
    /// Start over after the last step.
    #[default]
    Loop,
    /// Stay at the last step.
    Once,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub id: SceneId,
    pub name: String,
    /// Scenes of the same zone exclude each other.
    /// `None` = freely combinable.
    pub zone: Option<ZoneId>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub loop_mode: LoopMode,
    /// Own color of the scene; `None` = color of the bank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
    /// Playback settings (dimmer, speed, fade in/out …).
    #[serde(default, skip_serializing_if = "SceneSettings::is_default")]
    pub settings: SceneSettings,
    /// Effects; a scene with effects is an FX scene.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    /// Hidden in the scene overview (the bank menu shows it again).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
}

/// Kind of an effect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectKind {
    Steps,
    ColorFx,
    ChaserFx,
    MoveFx,
    ValueFx,
    CurveFx,
    Mappings,
    ColorMappings,
    /// Timeline with scenes and audio (no further effect possible).
    SuperScene,
}

impl EffectKind {
    pub const ALL: [EffectKind; 9] = [
        EffectKind::Steps,
        EffectKind::ColorFx,
        EffectKind::ChaserFx,
        EffectKind::MoveFx,
        EffectKind::ValueFx,
        EffectKind::CurveFx,
        EffectKind::Mappings,
        EffectKind::ColorMappings,
        EffectKind::SuperScene,
    ];

    /// The effect stands alone in a scene (STEPS, SUPER SCENE).
    pub fn exclusive(self) -> bool {
        matches!(self, EffectKind::Steps | EffectKind::SuperScene)
    }

    /// Display name (technical term, not translated).
    /// The effect moves assigned channels (“features”).
    pub fn uses_features(self) -> bool {
        matches!(
            self,
            EffectKind::ChaserFx | EffectKind::ValueFx | EffectKind::CurveFx | EffectKind::Mappings
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            EffectKind::Steps => "STEPS",
            EffectKind::ColorFx => "COLOR FX",
            EffectKind::ChaserFx => "CHASER FX",
            EffectKind::MoveFx => "MOVE FX",
            EffectKind::ValueFx => "VALUE FX",
            EffectKind::CurveFx => "CURVE FX",
            EffectKind::Mappings => "MAPPINGS",
            EffectKind::ColorMappings => "COLOR MAPPINGS",
            EffectKind::SuperScene => "SUPER SCENE",
        }
    }
}

/// An effect of an FX scene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    pub id: u32,
    pub kind: EffectKind,
    /// Collapsed in the sidebar.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub collapsed: bool,
    /// Fixtures the effect acts on (order = beam order).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixtures: Vec<FixtureId>,
    /// Settings of a COLOR FX or (with gray values) a VALUE FX.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::fx::ColorFx>,
    /// Channels moved by the effect (CHASER, VALUE, CURVE, MAPPINGS).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<crate::fx::feature::Feature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chaser: Option<crate::fx::feature::ChaserFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<crate::fx::feature::CurveFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<crate::fx::motion::MoveFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<crate::fx::mapping::MappingFx>,
    /// Timeline of a SUPER SCENE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<crate::timeline::Timeline>,
}

impl Effect {
    pub fn new(id: u32, kind: EffectKind) -> Self {
        Self {
            id,
            kind,
            collapsed: false,
            fixtures: Vec::new(),
            color: match kind {
                EffectKind::ColorFx => Some(crate::fx::ColorFx::default()),
                EffectKind::ValueFx => Some(crate::fx::ColorFx::values()),
                _ => None,
            },
            features: if kind.uses_features() {
                vec![crate::fx::feature::Feature::new(
                    crate::fixture::ChannelKind::Dimmer,
                )]
            } else {
                Vec::new()
            },
            chaser: (kind == EffectKind::ChaserFx).then(Default::default),
            curve: (kind == EffectKind::CurveFx).then(Default::default),
            motion: (kind == EffectKind::MoveFx).then(Default::default),
            mapping: match kind {
                EffectKind::Mappings => Some(crate::fx::mapping::MappingFx::new(
                    crate::fx::mapping::MapPreset::Rainbow,
                    false,
                )),
                EffectKind::ColorMappings => Some(crate::fx::mapping::MappingFx::new(
                    crate::fx::mapping::MapPreset::Rainbow,
                    true,
                )),
                _ => None,
            },
            timeline: (kind == EffectKind::SuperScene).then(Default::default),
        }
    }

    /// Fill in missing settings (older shows/imports).
    pub fn ensure_settings(&mut self) {
        let fresh = Effect::new(self.id, self.kind);
        if self.color.is_none() {
            self.color = fresh.color;
        }
        if self.chaser.is_none() {
            self.chaser = fresh.chaser;
        }
        if self.curve.is_none() {
            self.curve = fresh.curve;
        }
        if self.motion.is_none() {
            self.motion = fresh.motion;
        }
        if self.mapping.is_none() {
            self.mapping = fresh.mapping;
        }
        if self.timeline.is_none() {
            self.timeline = fresh.timeline;
        }
    }
}

/// Kind of a scene (for display and sidebar).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    Static,
    Fx,
    Super,
}

/// Reference to banks for release and release protection.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankScope {
    /// No scenes.
    Off,
    /// All scenes.
    All,
    /// Scenes of the same bank.
    #[default]
    Bank,
    /// Scenes outside the own bank.
    OutsideBank,
    /// Scenes of specific banks.
    Banks(Vec<ZoneId>),
}

impl BankScope {
    /// Does the reference hit a scene of bank `other`, seen from `own`?
    pub fn matches(&self, own: Option<ZoneId>, other: Option<ZoneId>) -> bool {
        match self {
            BankScope::Off => false,
            BankScope::All => true,
            BankScope::Bank => own.is_some() && own == other,
            BankScope::OutsideBank => own.is_none() || own != other,
            BankScope::Banks(list) => other.is_some_and(|z| list.contains(&z)),
        }
    }
}

/// Settings of a scene in the “Properties” sidebar.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SceneSettings {
    /// Scene dimmer for the intensity (0–255, default full).
    pub dimmer: u8,
    /// Playback speed in percent (100 = normal).
    pub speed_pct: u16,
    /// Phase in degrees (for effects).
    pub phase_deg: u16,
    /// Size in percent (for effects).
    pub size_pct: u8,
    /// Fade-in time on start.
    pub fade_in_ms: u32,
    /// Fade-out time on stop.
    pub fade_out_ms: u32,
    /// Tempo of the step crossfades in tenths (10 = 1.0 as programmed, 20 =
    /// twice as fast; 1..=40).
    pub fade_speed: u8,
    /// Play the steps backwards.
    pub reverse: bool,
    /// Priority: the higher priority wins on overlaps, even if a lower one was
    /// started later.
    pub priority: i8,
    /// Cycles when not endless (`Scene::loop_mode == Once`).
    pub repeat_count: u16,
    /// At the end (after the cycles) start these scenes.
    pub jump_to_scenes: Vec<SceneId>,
    /// At the end (after the cycles) release.
    pub release_at_end: bool,
    /// Which running scenes are released on start.
    pub release_mode: BankScope,
    /// Which scenes cannot release this scene.
    pub protect: BankScope,
    /// Flash: only runs while the scene is pressed.
    pub flash: bool,
    /// Phase offset on start/stop (for effects).
    pub phase_in_ms: u32,
    pub phase_out_ms: u32,
    /// SUPER SCENE: duration of “Fade out sound” in ms (0 = program setting).
    pub audio_fade_ms: u32,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            dimmer: 255,
            speed_pct: 100,
            phase_deg: 0,
            size_pct: 100,
            fade_in_ms: 0,
            fade_out_ms: 0,
            fade_speed: 10,
            reverse: false,
            priority: 0,
            repeat_count: 1,
            jump_to_scenes: Vec::new(),
            release_at_end: false,
            release_mode: BankScope::Bank,
            protect: BankScope::Off,
            flash: false,
            phase_in_ms: 0,
            phase_out_ms: 0,
            audio_fade_ms: 0,
        }
    }
}

impl SceneSettings {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Scene {
    pub fn kind(&self) -> SceneKind {
        if self.effects.is_empty() {
            SceneKind::Static
        } else if self.timeline().is_some() {
            SceneKind::Super
        } else {
            SceneKind::Fx
        }
    }

    /// Timeline, if it is a SUPER SCENE.
    pub fn timeline(&self) -> Option<&crate::timeline::Timeline> {
        self.effects.iter().find_map(|e| e.timeline.as_ref())
    }

    pub fn timeline_mut(&mut self) -> Option<&mut crate::timeline::Timeline> {
        self.effects.iter_mut().find_map(|e| e.timeline.as_mut())
    }

    /// SUPER SCENE: set the only step to the length of the timeline so
    /// repeat/release at the end work as for other scenes. A SUPER SCENE
    /// has no values of its own (its light comes only from the timeline
    /// blocks), so the step is emptied.
    pub fn sync_timeline_length(&mut self) {
        let Some(len) = self.timeline().map(|t| t.length_ms()) else {
            return;
        };
        if self.steps.is_empty() {
            self.steps.push(Step::new(Vec::new()));
        }
        self.steps.truncate(1);
        self.steps[0].values.clear();
        self.steps[0].fade_ms = 0;
        self.steps[0].hold_ms = len.clamp(100, u64::from(u32::MAX)) as u32;
    }

    /// Scene with a STEPS effect (sequence of steps; no further effect
    /// possible).
    pub fn has_steps(&self) -> bool {
        self.effects.iter().any(|e| e.kind == EffectKind::Steps)
    }

    /// Insert a new step after `after` (copy of that step, hold time 1 s);
    /// returns its index.
    pub fn insert_step(&mut self, after: usize) -> usize {
        let mut st = self
            .steps
            .get(after)
            .cloned()
            .unwrap_or_else(|| Step::new(Vec::new()));
        st.hold_ms = 1000;
        st.fade_ms = 0;
        let at = (after + 1).min(self.steps.len());
        self.steps.insert(at, st);
        at
    }

    /// Delete a step (the last one always stays).
    pub fn remove_step(&mut self, i: usize) -> bool {
        if self.steps.len() <= 1 || i >= self.steps.len() {
            return false;
        }
        self.steps.remove(i);
        true
    }

    /// Append an effect; returns its ID.
    pub fn add_effect(&mut self, kind: EffectKind) -> u32 {
        let id = self.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1;
        self.effects.push(Effect::new(id, kind));
        id
    }

    /// Total duration of one cycle of all steps.
    pub fn duration_ms(&self) -> u64 {
        self.steps.iter().map(Step::duration_ms).sum()
    }

    /// Set the total duration: steps are stretched or compressed
    /// proportionally (a scene with one step gets this hold time).
    pub fn set_duration_ms(&mut self, ms: u64) {
        let old = self.duration_ms();
        if self.steps.is_empty() {
            return;
        }
        if old == 0 {
            let each = (ms / self.steps.len() as u64) as u32;
            for s in &mut self.steps {
                s.fade_ms = 0;
                s.hold_ms = each;
            }
            return;
        }
        let k = ms as f64 / old as f64;
        for s in &mut self.steps {
            s.fade_ms = (f64::from(s.fade_ms) * k).round() as u32;
            s.hold_ms = (f64::from(s.hold_ms) * k).round() as u32;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub id: ZoneId,
    pub name: String,
    /// Accent color of the zone in the UI (RGB).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
    /// Hidden in CONTROL (scenes keep running anyway).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
    /// Compact view in CONTROL: low scene cards with the name only.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub compact: bool,
}

/// A named fixture selection (e.g. “Moving heads”, “PARs truss”).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub fixtures: Vec<FixtureId>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatchError {
    #[error("Unknown fixture type “{0}”")]
    UnknownType(String),
    #[error("Universe {0} does not exist (available: {1})")]
    BadUniverse(u8, u8),
    #[error("Address {address} + {channels} channels does not fit into a universe")]
    OutOfRange { address: u16, channels: u16 },
    #[error("Overlaps with “{0}”")]
    Overlap(String),
}

#[derive(Debug, Error)]
pub enum ShowError {
    #[error("Could not read/write file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid show format: {0}")]
    Format(#[from] serde_json::Error),
    #[error("Show version {0} is not supported (maximum {SHOW_FORMAT_VERSION})")]
    Version(u32),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show {
    pub format_version: u32,
    pub name: String,
    /// Number of DMX universes – always [`UNIVERSES`] (older files are adjusted
    /// on load).
    pub universes: u8,
    pub fixture_types: Vec<FixtureType>,
    pub fixtures: Vec<Fixture>,
    pub zones: Vec<Zone>,
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub groups: Vec<Group>,
    /// Venue for the 3D visualizer.
    #[serde(default)]
    pub venue: crate::venue::Venue,
    /// Keyboard and MIDI mappings.
    #[serde(default, skip_serializing_if = "crate::mapping::Mappings::is_empty")]
    pub mappings: crate::mapping::Mappings,
    /// Touch surface (pages with freely arranged controls).
    #[serde(default, skip_serializing_if = "crate::touch::TouchLayout::is_empty")]
    pub touch: crate::touch::TouchLayout,
    /// Connection to OBS (video blocks, OBS actions).
    #[serde(default, skip_serializing_if = "crate::obs::ObsSettings::is_default")]
    pub obs: crate::obs::ObsSettings,
    /// SOUND tab: players, playlists, jingle pads.
    #[serde(default, skip_serializing_if = "crate::sound::SoundBoard::is_default")]
    pub sound: crate::sound::SoundBoard,
    /// SCENE GROUPS (scenes with offsets, used as blocks in SUPER SCENES).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scene_groups: Vec<crate::scene_group::SceneGroup>,
    /// LIVE TRACKING: points that moving heads follow.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trackers: Vec<crate::tracking::Tracker>,
}

impl Default for Show {
    fn default() -> Self {
        Self::new("New show", 2)
    }
}

impl Show {
    /// New show. `_universes` is ignored; every show has [`UNIVERSES`] universes.
    pub fn new(name: impl Into<String>, _universes: u8) -> Self {
        Self {
            format_version: SHOW_FORMAT_VERSION,
            name: name.into(),
            universes: UNIVERSES,
            fixture_types: Vec::new(),
            fixtures: Vec::new(),
            zones: Vec::new(),
            scenes: Vec::new(),
            groups: Vec::new(),
            venue: crate::venue::Venue::default(),
            mappings: Default::default(),
            touch: Default::default(),
            obs: Default::default(),
            sound: Default::default(),
            scene_groups: Vec::new(),
            trackers: Vec::new(),
        }
    }

    /// Older shows: scenes with several steps get the STEPS effect so their
    /// steps can be edited.
    pub fn normalize(&mut self) {
        for tl in self.scenes.iter_mut().filter_map(|s| s.timeline_mut()) {
            tl.normalize();
        }
        for fx in self.scenes.iter_mut().flat_map(|s| s.effects.iter_mut()) {
            fx.ensure_settings();
        }
        for sc in &mut self.scenes {
            // SUPER SCENES: drop values of their own (older shows could store
            // the programmer of the previously edited scene there)
            sc.sync_timeline_length();
            if sc.steps.len() > 1 && !sc.has_steps() {
                sc.effects.insert(
                    0,
                    Effect::new(
                        sc.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1,
                        EffectKind::Steps,
                    ),
                );
            }
        }
    }

    // ---------- Load / save ----------

    pub fn to_json(&self) -> Result<String, ShowError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, ShowError> {
        let mut show: Show = serde_json::from_str(json)?;
        if show.format_version > SHOW_FORMAT_VERSION {
            return Err(ShowError::Version(show.format_version));
        }
        show.universes = UNIVERSES;
        show.normalize();
        show.refine_channel_kinds();
        Ok(show)
    }

    /// Writes the show. Goes through a temporary file next to it, so a crash
    /// or a full disk while writing never leaves a half-written show.
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ShowError> {
        let path = path.as_ref();
        let mut tmp = path.as_os_str().to_owned();
        tmp.push(".tmp");
        let tmp = std::path::PathBuf::from(tmp);
        {
            // on disk before the rename, so a power cut cannot leave an empty file
            use std::io::Write;
            let mut f = std::fs::File::create(&tmp)?;
            f.write_all(self.to_json()?.as_bytes())?;
            f.sync_all()?;
        }
        if let Err(e) = std::fs::rename(&tmp, path) {
            let _ = std::fs::remove_file(&tmp);
            return Err(e.into());
        }
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ShowError> {
        Self::from_json(&std::fs::read_to_string(path)?)
    }

    /// Refine the channel kinds of the stored fixture types based on the
    /// channel names (older shows do not know e.g. laser or fog yet).
    /// Returns the number of changed channels.
    pub fn refine_channel_kinds(&mut self) -> usize {
        let mut n = 0;
        for ty in &mut self.fixture_types {
            for c in &mut ty.channels {
                let k = c.kind.refine(&c.name);
                if k != c.kind {
                    c.kind = k;
                    n += 1;
                }
            }
        }
        n
    }

    // ---------- Fixture types ----------

    pub fn fixture_type(&self, key: &str) -> Option<&FixtureType> {
        self.fixture_types.iter().find(|t| t.key == key)
    }

    /// Adds a fixture type or replaces it (same key).
    pub fn upsert_fixture_type(&mut self, ty: FixtureType) {
        match self.fixture_types.iter_mut().find(|t| t.key == ty.key) {
            Some(existing) => *existing = ty,
            None => self.fixture_types.push(ty),
        }
    }

    // ---------- Patch ----------

    pub fn fixture(&self, id: FixtureId) -> Option<&Fixture> {
        self.fixtures.iter().find(|f| f.id == id)
    }

    pub fn fixture_mut(&mut self, id: FixtureId) -> Option<&mut Fixture> {
        self.fixtures.iter_mut().find(|f| f.id == id)
    }

    pub fn type_of(&self, fixture: &Fixture) -> Option<&FixtureType> {
        self.fixture_type(&fixture.type_key)
    }

    /// Virtual dimmer of a fixture, if it needs one.
    pub fn virtual_dimmer(&self, id: FixtureId) -> Option<ChannelRef> {
        let f = self.fixture(id)?;
        self.type_of(f)?
            .needs_virtual_dimmer()
            .then(|| ChannelRef::virtual_dimmer(id))
    }

    pub fn channel_def(&self, ch: ChannelRef) -> Option<&ChannelDef> {
        let f = self.fixture(ch.fixture)?;
        self.type_of(f)?.channels.get(usize::from(ch.channel))
    }

    fn next_fixture_id(&self) -> FixtureId {
        FixtureId(self.fixtures.iter().map(|f| f.id.0).max().unwrap_or(0) + 1)
    }

    /// Checks whether a fixture fits at this position. `ignore` allows
    /// repatching an existing fixture.
    pub fn check_placement(
        &self,
        type_key: &str,
        universe: u8,
        address: u16,
        ignore: Option<FixtureId>,
    ) -> Result<(), PatchError> {
        let ty = self
            .fixture_type(type_key)
            .ok_or_else(|| PatchError::UnknownType(type_key.into()))?;
        if universe >= self.universes {
            return Err(PatchError::BadUniverse(universe + 1, self.universes));
        }
        let channels = ty.channel_count();
        let end = u32::from(address) + u32::from(channels) - 1;
        if address == 0 || channels == 0 || end > UNIVERSE_SIZE as u32 {
            return Err(PatchError::OutOfRange { address, channels });
        }
        let start = u32::from(address);
        for other in &self.fixtures {
            if Some(other.id) == ignore || other.universe != universe {
                continue;
            }
            let Some(other_ty) = self.type_of(other) else {
                continue;
            };
            let o_start = u32::from(other.address);
            let o_end = o_start + u32::from(other_ty.channel_count()) - 1;
            if start <= o_end && o_start <= end {
                return Err(PatchError::Overlap(other.name.clone()));
            }
        }
        Ok(())
    }

    pub fn add_fixture(
        &mut self,
        name: impl Into<String>,
        type_key: &str,
        universe: u8,
        address: u16,
    ) -> Result<FixtureId, PatchError> {
        self.check_placement(type_key, universe, address, None)?;
        let id = self.next_fixture_id();
        self.fixtures.push(Fixture {
            id,
            name: name.into(),
            type_key: type_key.into(),
            universe,
            address,
            plan_pos: None,
            plan_shape: None,
            plan_size: None,
            plan_rotation: None,
            limits: FixtureLimits::default(),
            place3d: None,
        });
        Ok(id)
    }

    pub fn repatch_fixture(
        &mut self,
        id: FixtureId,
        universe: u8,
        address: u16,
    ) -> Result<(), PatchError> {
        let type_key = match self.fixture(id) {
            Some(f) => f.type_key.clone(),
            None => return Ok(()),
        };
        self.check_placement(&type_key, universe, address, Some(id))?;
        if let Some(f) = self.fixture_mut(id) {
            f.universe = universe;
            f.address = address;
        }
        Ok(())
    }

    /// Removes a fixture and all scene values that concern it.
    pub fn remove_fixture(&mut self, id: FixtureId) {
        self.fixtures.retain(|f| f.id != id);
        for g in &mut self.groups {
            g.fixtures.retain(|f| *f != id);
        }
        for t in &mut self.trackers {
            t.fixtures.retain(|f| *f != id);
        }
        for scene in &mut self.scenes {
            for step in &mut scene.steps {
                step.values.retain(|v| v.fixture != id);
            }
        }
    }

    /// Shape of a fixture in the stage plan (stored or estimated by type).
    pub fn plan_shape(&self, id: FixtureId) -> PlanShape {
        let Some(f) = self.fixture(id) else {
            return PlanShape::Round;
        };
        f.plan_shape
            .or_else(|| self.type_of(f).map(PlanShape::guess))
            .unwrap_or(PlanShape::Round)
    }

    /// Rotation of a fixture in the stage plan (degrees, 0..360).
    pub fn plan_rotation(&self, id: FixtureId) -> f32 {
        self.fixture(id)
            .and_then(|f| f.plan_rotation)
            .unwrap_or(0.0)
            .rem_euclid(360.0)
    }

    /// Size factor of a fixture in the stage plan.
    pub fn plan_size(&self, id: FixtureId) -> f32 {
        self.fixture(id)
            .and_then(|f| f.plan_size)
            .unwrap_or(1.0)
            .clamp(*PLAN_SIZE_RANGE.start(), *PLAN_SIZE_RANGE.end())
    }

    /// Channel count of a patched fixture.
    pub fn fixture_channels(&self, id: FixtureId) -> u16 {
        self.fixture(id)
            .and_then(|f| self.type_of(f))
            .map_or(0, |t| t.channel_count())
    }

    /// Usage of a universe: index = address − 1 → fixture (if occupied).
    pub fn address_map(&self, universe: u8) -> Vec<Option<FixtureId>> {
        let mut map = vec![None; UNIVERSE_SIZE];
        for f in self.fixtures.iter().filter(|f| f.universe == universe) {
            let start = usize::from(f.address).saturating_sub(1);
            let end = (start + usize::from(self.fixture_channels(f.id))).min(UNIVERSE_SIZE);
            for slot in &mut map[start..end] {
                slot.get_or_insert(f.id);
            }
        }
        map
    }

    /// Patch copies of fixtures: each at the first free address from `from`
    /// (in the following universes if needed), in the given order. Names are
    /// counted on (“PAR 4” → “PAR 5”), the stage position slightly offset.
    /// Returns the new IDs.
    pub fn paste_fixtures(&mut self, src: &[Fixture], universe: u8, from: u16) -> Vec<FixtureId> {
        let mut out = Vec::new();
        let (mut u, mut a) = (universe, from.max(1));
        for f in src {
            let Some(ch) = self.fixture_type(&f.type_key).map(|t| t.channel_count()) else {
                continue;
            };
            // find the first free position from (u, a)
            let mut place = None;
            while u < self.universes && place.is_none() {
                let mut addr = a;
                while u32::from(addr) + u32::from(ch) - 1 <= UNIVERSE_SIZE as u32 {
                    if self.check_placement(&f.type_key, u, addr, None).is_ok() {
                        place = Some((u, addr));
                        break;
                    }
                    addr += 1;
                }
                if place.is_none() {
                    u += 1;
                    a = 1;
                }
            }
            let Some((pu, pa)) = place else {
                break;
            };
            let id = self.next_fixture_id();
            let name = self.next_fixture_name(&f.name);
            let from = if self.fixture(f.id).is_some() {
                self.plan_position(f.id)
            } else {
                f.plan_pos
                    .unwrap_or_else(|| crate::plan::default_position(self.fixtures.len()))
            };
            let pos = self.free_plan_position(from);
            self.fixtures.push(Fixture {
                id,
                name,
                universe: pu,
                address: pa,
                plan_pos: Some(pos),
                ..f.clone()
            });
            out.push(id);
            u = pu;
            a = pa + ch;
        }
        out
    }

    /// Programmer pan/tilt (16 bit) that point a moving head at `target`
    /// (room coordinates, m), with its output limits and inversion taken
    /// into account. `None` if the fixture has no pan/tilt or the point is
    /// out of reach.
    pub fn aim_moving_head(&self, id: FixtureId, target: [f32; 3]) -> Option<(u16, u16)> {
        self.aim_moving_head_near(id, target, None)
    }

    /// Like [`Show::aim_moving_head`]; of the possible pan/tilt pairs the one
    /// closest to `near` (so a following head does not swing around).
    pub fn aim_moving_head_near(
        &self,
        id: FixtureId,
        target: [f32; 3],
        near: Option<(u16, u16)>,
    ) -> Option<(u16, u16)> {
        let f = self.fixture(id)?;
        let ty = self.type_of(f)?;
        if !ty.has_kind(crate::ChannelKind::Pan) || !ty.has_kind(crate::ChannelKind::Tilt) {
            return None;
        }
        let ph = ty.physical.unwrap_or_default();
        let p = self.placement3d(id);
        // the head turns about 33 cm below the mounting point (as drawn)
        let k = if ph.size_mm[1] > 100.0 {
            (ph.size_mm[1] / 420.0).clamp(0.6, 1.6)
        } else {
            1.0
        };
        let r = crate::venue::rotation(p.rotation);
        let origin = [0, 1, 2].map(|i| p.position[i] - r[1][i] * 0.33 * k);
        let lim = f.limits;
        // output value back to the programmer value (inverse of map16)
        let inv = |out: f32, min: u8, max: u8, invert: bool| -> Option<u16> {
            let o = out * 65535.0;
            let (lo, hi) = (f32::from(min) * 256.0, f32::from(max) * 256.0 + 255.0);
            let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
            if o < lo - 1.0 || o > hi + 1.0 || hi - lo < 1.0 {
                return None;
            }
            let v = ((o - lo) / (hi - lo) * 65535.0).round().clamp(0.0, 65535.0) as u16;
            Some(if invert { u16::MAX - v } else { v })
        };
        let mut all =
            crate::venue::moving_head_aim(p.rotation, origin, target, ph.pan_range, ph.tilt_range)
                .into_iter()
                .filter_map(|(pan, tilt)| {
                    let po = inv(pan, lim.pan_min, lim.pan_max, lim.invert_pan)?;
                    let to = inv(tilt, lim.tilt_min, lim.tilt_max, lim.invert_tilt)?;
                    Some(if lim.swap_pan_tilt {
                        (to, po)
                    } else {
                        (po, to)
                    })
                });
        let Some((np, nt)) = near else {
            return all.next();
        };
        all.min_by_key(|&(p, t)| {
            let dp = i64::from(p) - i64::from(np);
            let dt = i64::from(t) - i64::from(nt);
            dp * dp + dt * dt
        })
    }

    /// Position in the stage plan (own or automatic).
    /// Placement of a fixture in 3D space: stored or derived from the stage plan
    /// (below the ceiling, pointing down).
    pub fn placement3d(&self, id: FixtureId) -> crate::venue::Placement {
        if let Some(p) = self.fixture(id).and_then(|f| f.place3d) {
            return p;
        }
        let [w, h, d] = self.venue.size;
        let [px, py] = self.plan_position(id);
        crate::venue::Placement {
            position: [
                (px - 0.5) * w * 0.8,
                (h - 0.35).max(1.5),
                (0.5 - py) * d * 0.8,
            ],
            rotation: [0.0; 3],
        }
    }

    /// How directions in the room (x, z) appear in the stage plan (x, y):
    /// `plan = [m[0][0]·x + m[0][1]·z, m[1][0]·x + m[1][1]·z]`. Fitted to the
    /// moving heads that have both a 3D placement and a plan position – the
    /// plan may be drawn rotated or mirrored (e.g. seen from the stage).
    /// Without enough of them, or if they do not match: room x = right,
    /// room z = up.
    pub fn plan_axes(&self) -> [[f32; 2]; 2] {
        let pairs: Vec<([f32; 2], [f32; 2])> = self
            .fixtures
            .iter()
            // only moving heads: other fixtures are often drawn schematically
            .filter(|f| {
                self.type_of(f)
                    .is_some_and(|t| t.has_kind(crate::fixture::ChannelKind::Pan))
            })
            .filter_map(|f| {
                let p3 = f.place3d?;
                let pp = f.plan_pos?;
                Some(([p3.position[0], p3.position[2]], pp))
            })
            .collect();
        let default = [[1.0, 0.0], [0.0, -1.0]];
        if pairs.len() < 3 {
            return default;
        }
        let n = pairs.len() as f32;
        type Pair = ([f32; 2], [f32; 2]);
        let mean = |f: fn(&Pair) -> [f32; 2]| {
            let s = pairs
                .iter()
                .map(f)
                .fold([0.0, 0.0], |a, v| [a[0] + v[0], a[1] + v[1]]);
            [s[0] / n, s[1] / n]
        };
        let (rm, pm) = (mean(|p| p.0), mean(|p| p.1));
        // Best rotation (and best rotation of the mirrored room): angle from
        // the summed dot and cross products of the centered point pairs
        let fit = |mirror: f32| {
            let (mut dot, mut cross, mut spread) = (0.0f32, 0.0f32, 0.0f32);
            for (r, p) in &pairs {
                let a = [r[0] - rm[0], mirror * (r[1] - rm[1])];
                let b = [p[0] - pm[0], p[1] - pm[1]];
                dot += a[0] * b[0] + a[1] * b[1];
                cross += a[0] * b[1] - a[1] * b[0];
                spread += (a[0] * a[0] + a[1] * a[1]).sqrt() * (b[0] * b[0] + b[1] * b[1]).sqrt();
            }
            let angle = cross.atan2(dot);
            // quality: share of the spread that the rotation explains
            let q = if spread > 1e-6 {
                dot.hypot(cross) / spread
            } else {
                0.0
            };
            (angle, q)
        };
        let (a1, q1) = fit(1.0);
        let (a2, q2) = fit(-1.0);
        let (angle, q, mirror) = if q2 > q1 {
            (a2, q2, -1.0)
        } else {
            (a1, q1, 1.0)
        };
        if q < 0.8 {
            return default;
        }
        let (s, c) = angle.sin_cos();
        // plan = Rot(angle) · [x, mirror·z]
        [[c, -s * mirror], [s, c * mirror]]
    }

    pub fn plan_position(&self, id: FixtureId) -> [f32; 2] {
        let idx = self.fixtures.iter().position(|f| f.id == id).unwrap_or(0);
        self.fixture(id)
            .and_then(|f| f.plan_pos)
            .unwrap_or_else(|| crate::plan::default_position(idx))
    }

    /// Next free spot in the stage plan next to `from` (searched in rings
    /// outwards) so pasted fixtures do not lie on top of each other.
    pub fn free_plan_position(&self, from: [f32; 2]) -> [f32; 2] {
        const STEP: f32 = 0.06;
        const MIN_DIST: f32 = 0.035;
        let taken: Vec<[f32; 2]> = self
            .fixtures
            .iter()
            .map(|f| self.plan_position(f.id))
            .collect();
        let free = |p: [f32; 2]| {
            taken
                .iter()
                .all(|q| (q[0] - p[0]).hypot(q[1] - p[1]) >= MIN_DIST)
        };
        let inside = |p: [f32; 2]| (0.02..=0.98).contains(&p[0]) && (0.02..=0.98).contains(&p[1]);
        for ring in 1..20i32 {
            // first to the right, then to the left, then below, finally above
            let mut cands = Vec::new();
            for dy in -ring..=ring {
                for dx in -ring..=ring {
                    if dx.abs().max(dy.abs()) == ring {
                        cands.push((dx, dy));
                    }
                }
            }
            cands.sort_by_key(|(dx, dy)| (dy.abs(), -dx.signum(), -*dy, *dx));
            for (dx, dy) in cands {
                let p = [from[0] + dx as f32 * STEP, from[1] + dy as f32 * STEP];
                if inside(p) && free(p) {
                    return p;
                }
            }
        }
        from
    }

    /// Free name: if `name` ends in a number it is counted up to the first free
    /// name, otherwise “name 2”, “name 3” ….
    fn next_fixture_name(&self, name: &str) -> String {
        let taken = |n: &str| self.fixtures.iter().any(|f| f.name == n);
        if !taken(name) {
            return name.to_string();
        }
        let digits = name.len() - name.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        let (stem, mut n) = if digits > 0 {
            let (s, d) = name.split_at(name.len() - digits);
            (s.to_string(), d.parse::<u32>().unwrap_or(1))
        } else {
            (format!("{name} "), 1)
        };
        loop {
            n += 1;
            let cand = format!("{stem}{n}");
            if !taken(&cand) {
                return cand;
            }
        }
    }

    /// First free address in a universe where `channels` channels fit.
    pub fn next_free_address(&self, universe: u8, channels: u16) -> Option<u16> {
        if channels == 0 || usize::from(channels) > UNIVERSE_SIZE {
            return None;
        }
        let mut used = [false; UNIVERSE_SIZE];
        for f in self.fixtures.iter().filter(|f| f.universe == universe) {
            if let Some(ty) = self.type_of(f) {
                let start = usize::from(f.address).saturating_sub(1);
                let end = (start + usize::from(ty.channel_count())).min(UNIVERSE_SIZE);
                used[start..end].fill(true);
            }
        }
        let n = usize::from(channels);
        (0..=UNIVERSE_SIZE - n)
            .find(|&s| used[s..s + n].iter().all(|u| !u))
            .map(|s| s as u16 + 1)
    }

    // ---------- Zones & scenes ----------

    pub fn add_zone(&mut self, name: impl Into<String>) -> ZoneId {
        let id = ZoneId(self.zones.iter().map(|z| z.id.0).max().unwrap_or(0) + 1);
        self.zones.push(Zone {
            id,
            name: name.into(),
            color: None,
            hidden: false,
            compact: false,
        });
        id
    }

    pub fn zone_mut(&mut self, id: ZoneId) -> Option<&mut Zone> {
        self.zones.iter_mut().find(|z| z.id == id)
    }

    /// Copies a bank together with its scenes; the copy comes right after it.
    pub fn duplicate_zone(&mut self, id: ZoneId, name: impl Into<String>) -> Option<ZoneId> {
        let pos = self.zones.iter().position(|z| z.id == id)?;
        let new_id = ZoneId(self.zones.iter().map(|z| z.id.0).max().unwrap_or(0) + 1);
        let mut zone = self.zones[pos].clone();
        zone.id = new_id;
        zone.name = name.into();
        zone.hidden = false;
        self.zones.insert(pos + 1, zone);
        let copies: Vec<Scene> = self
            .scenes
            .iter()
            .filter(|s| s.zone == Some(id))
            .cloned()
            .collect();
        for mut sc in copies {
            sc.id = SceneId(self.scenes.iter().map(|s| s.id.0).max().unwrap_or(0) + 1);
            sc.zone = Some(new_id);
            self.scenes.push(sc);
        }
        Some(new_id)
    }

    /// Deletes a bank together with its scenes.
    /// Move a bank to position `to` (index in the bank list, before the
    /// bank currently there; `zones.len()` = at the end).
    pub fn move_zone(&mut self, id: ZoneId, to: usize) {
        let Some(from) = self.zones.iter().position(|z| z.id == id) else {
            return;
        };
        let z = self.zones.remove(from);
        let to = if to > from { to - 1 } else { to };
        self.zones.insert(to.min(self.zones.len()), z);
    }

    pub fn remove_zone_with_scenes(&mut self, id: ZoneId) {
        self.zones.retain(|z| z.id != id);
        let gone: Vec<SceneId> = self
            .scenes
            .iter()
            .filter(|s| s.zone == Some(id))
            .map(|s| s.id)
            .collect();
        self.scenes.retain(|s| s.zone != Some(id));
        for g in &mut self.scene_groups {
            g.members.retain(|m| !gone.contains(&m.scene));
        }
    }

    pub fn zone(&self, id: ZoneId) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == id)
    }

    // ---------- Groups ----------

    pub fn add_group(&mut self, name: impl Into<String>, fixtures: Vec<FixtureId>) -> GroupId {
        let id = GroupId(self.groups.iter().map(|g| g.id.0).max().unwrap_or(0) + 1);
        self.groups.push(Group {
            id,
            name: name.into(),
            fixtures,
        });
        id
    }

    pub fn remove_group(&mut self, id: GroupId) {
        self.groups.retain(|g| g.id != id);
    }

    /// Move scenes without a bank into a bank (the first bank, otherwise a new
    /// one named `name`). Returns `true` if something changed.
    pub fn adopt_free_scenes(&mut self, name: impl Into<String>) -> bool {
        if !self.scenes.iter().any(|s| s.zone.is_none()) {
            return false;
        }
        let zone = match self.zones.first() {
            Some(z) => z.id,
            None => self.add_zone(name),
        };
        for s in self.scenes.iter_mut().filter(|s| s.zone.is_none()) {
            s.zone = Some(zone);
        }
        true
    }

    /// Move scenes into a bank, before `before` (otherwise to the end of the
    /// bank). The order of the moved scenes is kept.
    pub fn move_scenes(&mut self, ids: &[SceneId], zone: ZoneId, before: Option<SceneId>) {
        let moving = |id: SceneId| ids.contains(&id);
        // The target must not be one of the moved scenes: the next other one in
        // the bank
        let before = before.and_then(|b| {
            let start = self.scenes.iter().position(|s| s.id == b)?;
            self.scenes[start..]
                .iter()
                .find(|s| s.zone == Some(zone) && !moving(s.id))
                .map(|s| s.id)
        });
        let mut taken = Vec::new();
        self.scenes.retain(|s| {
            if moving(s.id) {
                taken.push(s.clone());
                false
            } else {
                true
            }
        });
        for s in &mut taken {
            s.zone = Some(zone);
        }
        let at = match before.and_then(|b| self.scenes.iter().position(|s| s.id == b)) {
            Some(i) => i,
            None => self
                .scenes
                .iter()
                .rposition(|s| s.zone == Some(zone))
                .map_or(self.scenes.len(), |i| i + 1),
        };
        self.scenes.splice(at..at, taken);
    }

    pub fn remove_zone(&mut self, id: ZoneId) {
        self.zones.retain(|z| z.id != id);
        for s in &mut self.scenes {
            if s.zone == Some(id) {
                s.zone = None;
            }
        }
    }

    pub fn scene(&self, id: SceneId) -> Option<&Scene> {
        self.scenes.iter().find(|s| s.id == id)
    }

    pub fn scene_mut(&mut self, id: SceneId) -> Option<&mut Scene> {
        self.scenes.iter_mut().find(|s| s.id == id)
    }

    pub fn add_scene(
        &mut self,
        name: impl Into<String>,
        zone: Option<ZoneId>,
        steps: Vec<Step>,
    ) -> SceneId {
        let id = SceneId(self.scenes.iter().map(|s| s.id.0).max().unwrap_or(0) + 1);
        self.scenes.push(Scene {
            id,
            name: name.into(),
            zone,
            steps,
            loop_mode: LoopMode::Loop,
            color: None,
            settings: SceneSettings::default(),
            effects: Vec::new(),
            hidden: false,
        });
        id
    }

    /// New SUPER SCENE after the first scene from `ids`: the scenes lie one
    /// after another on track 1.
    pub fn create_super_scene(
        &mut self,
        ids: &[SceneId],
        name: impl Into<String>,
    ) -> Option<SceneId> {
        let first = *ids.first()?;
        let pos = self.scenes.iter().position(|s| s.id == first)?;
        let zone = self.scenes[pos].zone;
        let id = self.add_scene(name, zone, vec![Step::new(Vec::new())]);
        let parts: Vec<Scene> = ids.iter().filter_map(|i| self.scene(*i).cloned()).collect();
        let sc = self.scene_mut(id)?;
        sc.add_effect(EffectKind::SuperScene);
        if let Some(tl) = sc.timeline_mut() {
            let mut at = 0;
            for p in &parts {
                tl.add_scene_clip(0, p, at);
                at += p.duration_ms().max(100);
            }
        }
        sc.sync_timeline_length();
        // right after the first scene
        let new = self.scenes.pop()?;
        self.scenes.insert(pos + 1, new);
        Some(id)
    }

    /// Copies a scene; the copy comes right after the original.
    pub fn duplicate_scene(&mut self, id: SceneId, name: impl Into<String>) -> Option<SceneId> {
        let pos = self.scenes.iter().position(|s| s.id == id)?;
        let mut copy = self.scenes[pos].clone();
        copy.id = SceneId(self.scenes.iter().map(|s| s.id.0).max().unwrap_or(0) + 1);
        copy.name = name.into();
        let new_id = copy.id;
        self.scenes.insert(pos + 1, copy);
        Some(new_id)
    }

    pub fn remove_scene(&mut self, id: SceneId) {
        self.scenes.retain(|s| s.id != id);
        for g in &mut self.scene_groups {
            g.members.retain(|m| m.scene != id);
        }
        for tl in self.scenes.iter_mut().filter_map(|s| s.timeline_mut()) {
            tl.forget_scene(id);
        }
    }
}

#[cfg(test)]
mod zone_tests {
    use super::*;

    #[test]
    fn move_scenes_between_banks() {
        let mut s = Show::new("T", 8);
        let z1 = s.add_zone("1");
        let z2 = s.add_zone("2");
        let a = s.add_scene("A", Some(z1), vec![]);
        let b = s.add_scene("B", Some(z1), vec![]);
        let c = s.add_scene("C", Some(z2), vec![]);
        let order = |s: &Show, z| -> Vec<SceneId> {
            s.scenes
                .iter()
                .filter(|x| x.zone == Some(z))
                .map(|x| x.id)
                .collect()
        };
        s.move_scenes(&[a], z2, Some(c));
        assert_eq!(order(&s, z2), vec![a, c]);
        assert_eq!(order(&s, z1), vec![b]);
        s.move_scenes(&[a, c], z1, None);
        assert_eq!(order(&s, z1), vec![b, a, c]);
        // inserting before itself = before the next scene not moved
        s.move_scenes(&[a], z1, Some(a));
        assert_eq!(order(&s, z1), vec![b, a, c]);
        s.move_scenes(&[c], z1, Some(b));
        assert_eq!(order(&s, z1), vec![c, b, a]);
    }

    #[test]
    fn free_scenes_get_a_bank() {
        let mut s = Show::new("T", 8);
        let a = s.add_scene("A", None, vec![]);
        assert!(s.adopt_free_scenes("Bank 1"));
        let z = s.scene(a).unwrap().zone.unwrap();
        assert_eq!(s.zone(z).unwrap().name, "Bank 1");
        assert!(!s.adopt_free_scenes("x"));
        let b = s.add_scene("B", None, vec![]);
        s.adopt_free_scenes("x");
        assert_eq!(s.scene(b).unwrap().zone, Some(z));
        assert_eq!(s.zones.len(), 1);
    }

    #[test]
    fn duplicate_zone_copies_scenes_behind_original() {
        let mut s = Show::new("T", 8);
        let a = s.add_zone("A");
        let b = s.add_zone("B");
        s.add_scene("x", Some(a), vec![]);
        s.add_scene("y", Some(a), vec![]);
        s.add_scene("z", Some(b), vec![]);
        let c = s.duplicate_zone(a, "A (copy)").unwrap();
        assert_eq!(s.zones.iter().map(|z| z.id).collect::<Vec<_>>(), [a, c, b]);
        let copied: Vec<_> = s.scenes.iter().filter(|x| x.zone == Some(c)).collect();
        assert_eq!(copied.len(), 2);
        // new, unique IDs
        let mut ids: Vec<_> = s.scenes.iter().map(|x| x.id).collect();
        ids.dedup();
        assert_eq!(ids.len(), 5);
        let x = s.scenes[0].id;
        let dup = s.duplicate_scene(x, "x2").unwrap();
        assert_eq!(s.scenes[1].id, dup);
        assert_eq!(s.scenes[1].zone, Some(a));
        s.remove_scene(dup);
        s.remove_zone_with_scenes(a);
        assert_eq!(s.scenes.len(), 3);
        // Show format: hidden/color only saved when set
        let json = s.to_json().unwrap();
        assert!(!json.contains("hidden"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_scene_has_no_values_of_its_own() {
        let mut s = Show::new("t", 1);
        let cv = |v| ChannelValue {
            fixture: FixtureId(1),
            channel: 0,
            value: v,
        };
        // scene recorded with values, then turned into a SUPER SCENE
        let id = s.add_scene("S", None, vec![Step::new(vec![cv(255)])]);
        let sc = s.scene_mut(id).unwrap();
        sc.add_effect(EffectKind::SuperScene);
        sc.sync_timeline_length();
        assert!(s.scene(id).unwrap().steps[0].values.is_empty());
        // older shows: stray values are dropped when the show is loaded
        s.scene_mut(id).unwrap().steps[0].values.push(cv(100));
        s.normalize();
        assert!(s.scene(id).unwrap().steps[0].values.is_empty());
        // normal scenes keep their values
        let plain = s.add_scene("P", None, vec![Step::new(vec![cv(7)])]);
        s.normalize();
        assert_eq!(s.scene(plain).unwrap().steps[0].values, vec![cv(7)]);
    }

    #[test]
    fn plan_axes_follow_a_mirrored_plan() {
        let mut s = Show::new("t", 1);
        s.fixture_types = crate::library::builtin_fixture_types();
        // plan drawn mirrored in x: room x grows to the left of the plan
        for (i, (x, z)) in [(-3.0, 0.0), (2.0, 0.0), (-1.0, 3.0), (1.0, 3.0)]
            .into_iter()
            .enumerate()
        {
            let id = s
                .add_fixture("F", "generic/moving-head-spot-11ch", 0, 1 + 11 * i as u16)
                .unwrap();
            let f = s.fixture_mut(id).unwrap();
            f.place3d = Some(crate::venue::Placement {
                position: [x, 5.0, z],
                rotation: [0.0; 3],
            });
            f.plan_pos = Some([0.5 - x * 0.1, 0.5 - z * 0.1]);
        }
        let m = s.plan_axes();
        let map = |d: [f32; 2]| {
            [
                m[0][0] * d[0] + m[0][1] * d[1],
                m[1][0] * d[0] + m[1][1] * d[1],
            ]
        };
        let x = map([1.0, 0.0]);
        let z = map([0.0, 1.0]);
        assert!(x[0] < -0.99 && x[1].abs() < 0.01, "{x:?}");
        assert!(z[1] < -0.99 && z[0].abs() < 0.01, "{z:?}");
        // without both positions: default orientation
        assert_eq!(Show::new("u", 1).plan_axes(), [[1.0, 0.0], [0.0, -1.0]]);
    }
    use crate::library::builtin_fixture_types;

    #[test]
    fn move_zone_reorders_banks() {
        let mut s = Show::new("t", 1);
        let a = s.add_zone("A");
        let b = s.add_zone("B");
        let c = s.add_zone("C");
        let order = |s: &Show| s.zones.iter().map(|z| z.id).collect::<Vec<_>>();
        s.move_zone(c, 0);
        assert_eq!(order(&s), vec![c, a, b]);
        s.move_zone(c, 3);
        assert_eq!(order(&s), vec![a, b, c]);
        s.move_zone(a, 2);
        assert_eq!(order(&s), vec![b, a, c]);
    }

    #[test]
    fn aim_moving_head_respects_limits() {
        let mut s = Show::new("t", 1);
        s.fixture_types = builtin_fixture_types();
        let id = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        let f = s.fixture_mut(id).unwrap();
        f.place3d = Some(crate::venue::Placement {
            position: [0.0, 4.0, 0.0],
            rotation: [0.0, -90.0, 0.0],
        });
        f.limits.pan_min = 30;
        f.limits.pan_max = 220;
        f.limits.invert_tilt = true;
        let target = [2.0, 0.0, 3.0];
        let (p, t) = s.aim_moving_head(id, target).unwrap();
        let lim = s.fixture(id).unwrap().limits;
        let po = FixtureLimits::map16(p, lim.pan_min, lim.pan_max, lim.invert_pan);
        let to = FixtureLimits::map16(t, lim.tilt_min, lim.tilt_max, lim.invert_tilt);
        let ph = s
            .type_of(s.fixture(id).unwrap())
            .unwrap()
            .physical
            .unwrap_or_default();
        let d = crate::venue::moving_head_direction(
            [0.0, -90.0, 0.0],
            f32::from(po) / 65535.0,
            f32::from(to) / 65535.0,
            ph.pan_range,
            ph.tilt_range,
        );
        // head center 33 cm below the mounting point
        let w = [2.0f32, -3.67, 3.0];
        let n = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        let dot: f32 = (0..3).map(|i| d[i] * w[i] / n).sum();
        assert!(dot > 0.999, "{dot}");
    }

    #[test]
    fn steps_insert_remove_and_normalize() {
        let mut s = Show::new("T", 1);
        let id = s.add_scene(
            "A",
            None,
            vec![Step::new(Vec::new()), Step::new(Vec::new())],
        );
        s.normalize();
        let sc = s.scene_mut(id).unwrap();
        assert!(sc.has_steps());
        sc.steps[0].hold_ms = 300;
        sc.steps[0].fade_ms = 200;
        let at = sc.insert_step(0);
        assert_eq!(at, 1);
        assert_eq!(sc.steps.len(), 3);
        assert_eq!((sc.steps[1].hold_ms, sc.steps[1].fade_ms), (1000, 0));
        assert!(sc.remove_step(1));
        assert!(sc.remove_step(0));
        assert!(!sc.remove_step(0), "the last step stays");
    }

    fn show() -> Show {
        let mut s = Show::new("Test", 2);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        s
    }

    #[test]
    fn paste_fixtures_finds_free_addresses_and_names() {
        let mut s = show();
        let a = s
            .add_fixture("PAR 1", "generic/rgbw-par-6ch", 0, 1)
            .unwrap();
        let b = s
            .add_fixture("PAR 2", "generic/rgbw-par-6ch", 0, 13)
            .unwrap();
        let src: Vec<Fixture> = [a, b]
            .iter()
            .map(|i| s.fixture(*i).unwrap().clone())
            .collect();
        let ids = s.paste_fixtures(&src, 0, 1);
        let got: Vec<(String, u16)> = ids
            .iter()
            .map(|i| {
                let f = s.fixture(*i).unwrap();
                (f.name.clone(), f.address)
            })
            .collect();
        // 1..6 occupied, 7..12 free, 13..18 occupied, then 19
        assert_eq!(got, vec![("PAR 3".into(), 7), ("PAR 4".into(), 19)]);
        // full universe: continue in the next one
        let ids = s.paste_fixtures(&src, 0, 510);
        assert_eq!(s.fixture(ids[0]).unwrap().universe, 1);
        // no two fixtures at the same position in the stage plan
        let pos: Vec<[f32; 2]> = s.fixtures.iter().map(|f| s.plan_position(f.id)).collect();
        for (i, a) in pos.iter().enumerate() {
            for b in &pos[i + 1..] {
                assert!((a[0] - b[0]).hypot(a[1] - b[1]) > 0.03, "{a:?} {b:?}");
            }
        }
    }

    #[test]
    fn patch_rejects_overlap_and_range() {
        let mut s = show();
        s.add_fixture("PAR 1", "generic/rgbw-par-6ch", 0, 1)
            .unwrap();
        assert_eq!(
            s.add_fixture("PAR 2", "generic/rgbw-par-6ch", 0, 6),
            Err(PatchError::Overlap("PAR 1".into()))
        );
        assert!(s.add_fixture("PAR 2", "generic/rgbw-par-6ch", 0, 7).is_ok());
        assert!(s.add_fixture("PAR 3", "generic/rgbw-par-6ch", 1, 1).is_ok());
        assert!(matches!(
            s.add_fixture("X", "generic/rgbw-par-6ch", 0, 508),
            Err(PatchError::OutOfRange { .. })
        ));
        assert!(s.add_fixture("X", "generic/rgbw-par-6ch", 0, 507).is_ok());
        assert!(s.add_fixture("X", "generic/dimmer-1ch", 7, 1).is_ok());
        assert!(matches!(
            s.add_fixture("X", "generic/dimmer-1ch", 8, 1),
            Err(PatchError::BadUniverse(9, 8))
        ));
        assert!(matches!(
            s.add_fixture("X", "nope", 0, 100),
            Err(PatchError::UnknownType(_))
        ));
    }

    #[test]
    fn next_free_address_skips_used() {
        let mut s = show();
        s.add_fixture("A", "generic/rgbw-par-6ch", 0, 1).unwrap();
        s.add_fixture("B", "generic/rgbw-par-6ch", 0, 10).unwrap();
        assert_eq!(s.next_free_address(0, 3), Some(7));
        assert_eq!(s.next_free_address(0, 4), Some(16));
        assert_eq!(s.next_free_address(1, 512), Some(1));
    }

    #[test]
    fn plan_shape_is_guessed_from_type() {
        let mut s = show();
        let par = s.add_fixture("P", "generic/rgbw-par-6ch", 0, 1).unwrap();
        let mh = s
            .add_fixture("M", "generic/moving-head-spot-11ch", 0, 20)
            .unwrap();
        assert_eq!(s.plan_shape(par), PlanShape::Round);
        assert_eq!(s.plan_shape(mh), PlanShape::Round);
        s.fixture_mut(par).unwrap().plan_shape = Some(PlanShape::BarVertical);
        assert_eq!(s.plan_shape(par), PlanShape::BarVertical);
        assert_eq!(s.plan_size(par), 1.0);
        s.fixture_mut(par).unwrap().plan_size = Some(99.0);
        assert_eq!(s.plan_size(par), 4.0);
    }

    #[test]
    fn address_map_marks_ranges() {
        let mut s = show();
        let a = s.add_fixture("A", "generic/rgb-3ch", 0, 5).unwrap();
        let map = s.address_map(0);
        assert_eq!(map[3], None);
        assert_eq!(&map[4..7], &[Some(a), Some(a), Some(a)]);
        assert_eq!(map[7], None);
        assert!(s.address_map(1).iter().all(Option::is_none));
        assert_eq!(s.fixture_channels(a), 3);
    }

    #[test]
    fn repatch_ignores_itself() {
        let mut s = show();
        let a = s.add_fixture("A", "generic/rgbw-par-6ch", 0, 1).unwrap();
        assert!(s.repatch_fixture(a, 0, 3).is_ok());
        assert_eq!(s.fixture(a).unwrap().address, 3);
    }

    #[test]
    fn json_roundtrip() {
        let mut s = show();
        let f = s.add_fixture("A", "generic/rgb-3ch", 0, 1).unwrap();
        let z = s.add_zone("Colors");
        s.add_scene(
            "Red",
            Some(z),
            vec![Step::new(vec![ChannelValue {
                fixture: f,
                channel: 0,
                value: 255,
            }])],
        );
        let json = s.to_json().unwrap();
        assert_eq!(Show::from_json(&json).unwrap(), s);
    }

    #[test]
    fn old_show_files_without_new_fields_load() {
        let json = r#"{"format_version":1,"name":"Alt","universes":1,"fixture_types":[],
            "fixtures":[{"id":1,"name":"A","type_key":"x","universe":0,"address":1}],
            "zones":[{"id":1,"name":"Z"}],"scenes":[]}"#;
        let s = Show::from_json(json).unwrap();
        assert!(s.groups.is_empty());
        assert_eq!(s.fixtures[0].plan_pos, None);
        assert_eq!(s.zones[0].color, None);
    }

    #[test]
    fn groups_follow_fixture_removal() {
        let mut s = show();
        let a = s.add_fixture("A", "generic/rgb-3ch", 0, 1).unwrap();
        let b = s.add_fixture("B", "generic/rgb-3ch", 0, 4).unwrap();
        let g = s.add_group("All", vec![a, b]);
        s.remove_fixture(a);
        assert_eq!(
            s.groups.iter().find(|x| x.id == g).unwrap().fixtures,
            vec![b]
        );
    }

    #[test]
    fn removing_fixture_cleans_scenes() {
        let mut s = show();
        let f = s.add_fixture("A", "generic/rgb-3ch", 0, 1).unwrap();
        let id = s.add_scene(
            "Red",
            None,
            vec![Step::new(vec![ChannelValue {
                fixture: f,
                channel: 0,
                value: 255,
            }])],
        );
        s.remove_fixture(f);
        assert!(s.scene(id).unwrap().steps[0].values.is_empty());
    }
}
