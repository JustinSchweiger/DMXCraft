//! LIVE TRACKING: moving heads follow points in the room. Each tracker has a
//! target point (meters, like the 3D venue) and its fixtures; while it
//! follows, every fixture gets its own pan/tilt so all beams meet in the
//! point. It lies above scenes, programmer and live values. Besides pan/tilt
//! a target can also bring its own light (dimmer, color, zoom, shutter open)
//! – otherwise that comes from the scenes or the programmer.

use serde::{Deserialize, Serialize};

use crate::show::{FixtureId, Show};

/// A point that moving heads follow.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tracker {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub fixtures: Vec<FixtureId>,
    /// Target in the room (m): x (left/right), y (height), z (front/back).
    pub target: [f32; 3],
    /// The target brings its own light (dimmer, color, zoom, shutter open).
    #[serde(default = "yes")]
    pub light: bool,
    /// Dimmer 0–1.
    #[serde(default = "one")]
    pub dimmer: f32,
    #[serde(default = "white")]
    pub color: [u8; 3],
    /// Zoom / focus 0–1.
    #[serde(default = "half")]
    pub zoom: f32,
    /// The target sets the zoom (independent of its own light).
    #[serde(default = "yes")]
    pub own_zoom: bool,
    /// Color cells (rings, segments) the color goes to – bit n = cell n;
    /// the others stay dark.
    #[serde(default = "all_cells")]
    pub cells: u32,
    /// Following now (live state, not saved).
    #[serde(skip)]
    pub active: bool,
}

/// Equality of the saved settings: following on/off is a live state, not a
/// change of the show (no “unsaved changes”, no undo step).
impl PartialEq for Tracker {
    fn eq(&self, o: &Self) -> bool {
        self.id == o.id
            && self.name == o.name
            && self.fixtures == o.fixtures
            && self.target == o.target
            && self.light == o.light
            && self.dimmer == o.dimmer
            && self.color == o.color
            && self.zoom == o.zoom
            && self.own_zoom == o.own_zoom
            && self.cells == o.cells
    }
}

fn yes() -> bool {
    true
}
fn one() -> f32 {
    1.0
}
fn half() -> f32 {
    0.5
}
fn all_cells() -> u32 {
    u32::MAX
}
fn white() -> [u8; 3] {
    [255, 255, 255]
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
            light: true,
            dimmer: 1.0,
            color: white(),
            zoom: 0.5,
            own_zoom: true,
            cells: all_cells(),
            active: false,
        });
        id
    }

    pub fn remove_tracker(&mut self, id: u32) {
        self.trackers.retain(|t| t.id != id);
    }

    /// Number of color cells (rings, segments) of the tracker's fixtures:
    /// how many red channels the fixture with the most has.
    pub fn tracker_cells(&self, id: u32) -> usize {
        let Some(t) = self.tracker(id) else {
            return 0;
        };
        t.fixtures
            .iter()
            .filter_map(|f| self.fixture(*f).and_then(|x| self.type_of(x)))
            .map(|ty| {
                ty.channels
                    .iter()
                    .filter(|c| c.kind == crate::ChannelKind::Red)
                    .count()
            })
            .max()
            .unwrap_or(0)
    }

    /// Room extent (x and z, m) the targets can be moved in.
    pub fn tracking_area(&self) -> ([f32; 2], [f32; 2]) {
        let s = self.venue.size;
        let (w, d) = (s[0].max(2.0), s[2].max(2.0));
        ([-w / 2.0, w / 2.0], [-d / 2.0, d / 2.0])
    }
}
