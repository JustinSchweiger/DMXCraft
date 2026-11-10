//! Core of DMXCraft: data model and lighting engine, entirely without I/O.
//!
//! * [`fixture`] – fixture types and their channels
//! * [`show`]    – the show file: patch, zones, scenes (JSON)
//! * [`engine`]  – playback: scenes, crossfades, HTP/LTP merge, master
//! * [`dmx`]     – DMX frame (several universes of 512 channels each)

pub mod audio;
pub mod dip;
pub mod dmx;
pub mod engine;
pub mod fixture;
pub mod fx;
pub mod library;
pub mod look;
pub mod mapping;
pub mod obs;
pub mod plan;
pub mod show;
pub mod sound;
pub mod timeline;
pub mod touch;
pub mod venue;

pub use dmx::{DmxFrame, UNIVERSE_SIZE};
pub use engine::{Engine, FadeState, SceneProgress};
pub use fixture::{ChannelDef, ChannelKind, ChannelRange, FixtureType, RangeEffect};
pub use show::{
    BankScope, ChannelRef, ChannelValue, Effect, EffectKind, Fixture, FixtureId, FixtureLimits,
    Group, GroupId, LoopMode, PatchError, PlanShape, Scene, SceneId, SceneKind, SceneSettings,
    Show, ShowError, Step, Zone, ZoneId, PLAN_SIZE_RANGE, UNIVERSES,
};
