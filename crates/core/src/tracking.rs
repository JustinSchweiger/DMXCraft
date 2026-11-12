//! LIVE TRACKING: moving heads follow points in the room. Each tracker has a
//! target point (meters, like the 3D venue) and its fixtures; while it
//! follows, every fixture gets its own pan/tilt so all beams meet in the
//! point. It lies above scenes, programmer and live values and only takes
//! pan/tilt – color, dimmer, focus … come from the scenes or the programmer.

use serde::{Deserialize, Serialize};

use crate::show::{FixtureId, Show};

/// A point that moving heads follow.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tracker {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub fixtures: Vec<FixtureId>,
    /// Target in the room (m): x (left/right), y (height), z (front/back).
    pub target: [f32; 3],
    /// Following now (live state, not saved).
    #[serde(skip)]
    pub active: bool,
}

/// Default height of the target: chest height of a person.
pub const DEFAULT_HEIGHT: f32 = 1.3;

impl Show {
    pub fn tracker(&self, id: u32) -> Option<&Tracker> {
        self.trackers.iter().find(|t| t.id == id)
    }

    pub fn tracker_mut(&mut self, id: u32) -> Option<&mut Tracker> {
        self.trackers.iter_mut().find(|t| t.id == id)
    }

    /// New tracker in the middle of the room with these fixtures.
    pub fn add_tracker(&mut self, name: impl Into<String>, fixtures: Vec<FixtureId>) -> u32 {
        let id = self.trackers.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        self.trackers.push(Tracker {
            id,
            name: name.into(),
            fixtures,
            target: [0.0, DEFAULT_HEIGHT, 0.0],
            active: false,
        });
        id
    }

    pub fn remove_tracker(&mut self, id: u32) {
        self.trackers.retain(|t| t.id != id);
    }

    /// Room extent (x and z, m) the targets can be moved in.
    pub fn tracking_area(&self) -> ([f32; 2], [f32; 2]) {
        let s = self.venue.size;
        let (w, d) = (s[0].max(2.0), s[2].max(2.0));
        ([-w / 2.0, w / 2.0], [-d / 2.0, d / 2.0])
    }
}
