//! Kern von DMXCraft: Datenmodell und Lichtengine, komplett ohne I/O.
//!
//! * [`fixture`] – Gerätetypen und deren Kanäle
//! * [`show`]    – die Show-Datei: Patch, Zonen, Szenen (JSON)
//! * [`engine`]  – Wiedergabe: Szenen, Überblendungen, HTP/LTP-Merge, Master
//! * [`dmx`]     – DMX-Frame (mehrere Universen à 512 Kanäle)

pub mod dip;
pub mod dmx;
pub mod engine;
pub mod fixture;
pub mod library;
pub mod plan;
pub mod show;

pub use dmx::{DmxFrame, UNIVERSE_SIZE};
pub use engine::{Engine, FadeState, SceneProgress};
pub use fixture::{ChannelDef, ChannelKind, FixtureType};
pub use show::{
    ChannelRef, ChannelValue, Effect, EffectKind, Fixture, FixtureId, FixtureLimits, Group,
    GroupId, LoopMode, PatchError, PlanShape, Scene, SceneId, SceneKind, SceneSettings, Show,
    ShowError, Step, Zone, ZoneId, PLAN_SIZE_RANGE, UNIVERSES,
};
