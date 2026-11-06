//! Kern von DmxCraft: Datenmodell und Lichtengine, komplett ohne I/O.
//!
//! * [`fixture`] – Gerätetypen und deren Kanäle
//! * [`show`]    – die Show-Datei: Patch, Zonen, Szenen (JSON)
//! * [`engine`]  – Wiedergabe: Szenen, Überblendungen, HTP/LTP-Merge, Master
//! * [`dmx`]     – DMX-Frame (mehrere Universen à 512 Kanäle)

pub mod dmx;
pub mod engine;
pub mod fixture;
pub mod library;
pub mod plan;
pub mod show;

pub use dmx::{DmxFrame, UNIVERSE_SIZE};
pub use engine::{Engine, SceneProgress};
pub use fixture::{ChannelDef, ChannelKind, FixtureType};
pub use show::{
    ChannelRef, ChannelValue, Fixture, FixtureId, Group, GroupId, LoopMode, PatchError, Scene,
    SceneId, Show, ShowError, Step, Zone, ZoneId,
};
