//! Sound player (SOUND tab): players with playlists and jingle pads.
//! Only the settings stored with the show live here; playback happens in the
//! UI.

use serde::{Deserialize, Serialize};

/// Number of players and pads.
pub const DECKS: usize = 2;
pub const PADS: usize = 12;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundBoard {
    pub decks: Vec<Deck>,
    pub pads: Vec<Pad>,
    /// Duration of “Crossfade”.
    pub crossfade_ms: u32,
    /// Volume of all players and pads (0–1).
    pub master: f32,
    /// Grid of the pads (columns × rows).
    pub pad_cols: u8,
    pub pad_rows: u8,
    /// Width shares of the columns at the bottom: playlists, then pads.
    pub widths: Vec<f32>,
}

/// Limits for the pad grid.
pub const PAD_GRID_MAX: u8 = 8;
/// Default layout at the bottom (playlist A, playlist B, pads).
pub const WIDTHS: [f32; 3] = [1.0, 1.0, 1.15];
/// Smallest width share of a column.
pub const MIN_WIDTH: f32 = 0.25;

impl Default for SoundBoard {
    fn default() -> Self {
        Self {
            decks: vec![Deck::default(); DECKS],
            pads: vec![Pad::default(); PADS],
            crossfade_ms: 4000,
            master: 1.0,
            pad_cols: 3,
            pad_rows: 4,
            widths: WIDTHS.to_vec(),
        }
    }
}

impl SoundBoard {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    /// Add missing players/pads (older or hand-edited shows).
    pub fn normalize(&mut self) {
        self.decks
            .resize(self.decks.len().max(DECKS), Deck::default());
        self.pad_cols = self.pad_cols.clamp(1, PAD_GRID_MAX);
        self.pad_rows = self.pad_rows.clamp(1, PAD_GRID_MAX);
        let n = self.pad_count();
        self.pads.resize(self.pads.len().max(n), Pad::default());
        // empty pads beyond the grid are dropped; used ones are kept and appear
        // again when the grid grows
        while self.pads.len() > n && self.pads.last().is_some_and(|p| p.path.is_empty()) {
            self.pads.pop();
        }
        if self.widths.len() != WIDTHS.len()
            || self.widths.iter().any(|w| !w.is_finite() || *w <= 0.0)
        {
            self.widths = WIDTHS.to_vec();
        }
        for w in &mut self.widths {
            *w = w.max(MIN_WIDTH);
        }
    }

    /// Number of visible pads.
    pub fn pad_count(&self) -> usize {
        self.pad_cols as usize * self.pad_rows as usize
    }

    /// All files in use (for the check for missing files).
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.decks
            .iter()
            .flat_map(|d| d.playlist.iter().map(|t| t.path.as_str()))
            .chain(self.pads.iter().map(|p| p.path.as_str()))
            .filter(|p| !p.is_empty())
    }

    /// Replace a path everywhere (relink); number of places.
    pub fn relink(&mut self, old: &str, new: &str) -> usize {
        let mut n = 0;
        for t in self.decks.iter_mut().flat_map(|d| d.playlist.iter_mut()) {
            if t.path == old {
                t.path = new.to_string();
                n += 1;
            }
        }
        for p in &mut self.pads {
            if p.path == old {
                p.path = new.to_string();
                n += 1;
            }
        }
        n
    }
}

/// A player with its playlist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Deck {
    pub playlist: Vec<Track>,
    /// Volume (0–1).
    pub volume: f32,
    /// Repeat the track endlessly.
    pub repeat: bool,
    /// Start the next track of the playlist after the end.
    pub auto_next: bool,
    /// Duration of “Fade out”.
    pub fade_ms: u32,
}

impl Default for Deck {
    fn default() -> Self {
        Self {
            playlist: Vec::new(),
            volume: 0.85,
            repeat: false,
            auto_next: true,
            fade_ms: 3000,
        }
    }
}

/// Track of a playlist.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    /// Length (0 = unknown).
    #[serde(default)]
    pub length_ms: u64,
    /// Own display name (empty = file name).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
}

impl Track {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            length_ms: 0,
            name: String::new(),
        }
    }

    /// File name without folder and extension.
    pub fn file_name(&self) -> String {
        file_name(&self.path)
    }

    /// Display name: own name, otherwise the file name.
    pub fn label(&self) -> String {
        if self.name.trim().is_empty() {
            self.file_name()
        } else {
            self.name.clone()
        }
    }
}

/// Jingle pad.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pad {
    /// Display name (empty = file name).
    pub name: String,
    pub path: String,
    pub color: Option<[u8; 3]>,
    /// Level (0–1).
    pub volume: f32,
    /// Repeat endlessly until pressed again.
    pub repeat: bool,
    /// Stop all other pads on start.
    pub exclusive: bool,
    /// Length (0 = unknown).
    pub length_ms: u64,
}

impl Default for Pad {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: String::new(),
            color: None,
            volume: 1.0,
            repeat: false,
            exclusive: false,
            length_ms: 0,
        }
    }
}

impl Pad {
    pub fn label(&self) -> String {
        if self.name.is_empty() {
            file_name(&self.path)
        } else {
            self.name.clone()
        }
    }
}

/// File name without folder and extension.
pub fn file_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_stem()
        .map_or_else(|| path.to_string(), |n| n.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_board_is_not_saved_and_old_shows_load() {
        let b = SoundBoard::default();
        assert!(b.is_default());
        let mut old: SoundBoard =
            serde_json::from_str(r#"{"decks":[{"playlist":[{"path":"/a.mp3"}]}]}"#).unwrap();
        old.normalize();
        assert_eq!(old.decks.len(), DECKS);
        assert_eq!(old.pads.len(), PADS);
        assert_eq!(old.widths, WIDTHS.to_vec());
        assert_eq!(old.decks[0].playlist[0].label(), "a");
        old.decks[0].playlist[0].name = "Entrance".into();
        assert_eq!(old.decks[0].playlist[0].label(), "Entrance");
        assert_eq!(old.decks[0].playlist[0].file_name(), "a");
        assert_eq!(old.relink("/a.mp3", "/b.mp3"), 1);
        assert_eq!(old.paths().collect::<Vec<_>>(), vec!["/b.mp3"]);
    }

    #[test]
    fn pad_grid_keeps_used_pads() {
        let mut b = SoundBoard::default();
        b.pads[10].path = "tusch.mp3".into();
        b.pad_cols = 2;
        b.pad_rows = 2;
        b.normalize();
        // Pad 11 empty → gone, pad 10 used → stays (hidden)
        assert_eq!(b.pad_count(), 4);
        assert_eq!(b.pads.len(), 11);
        b.pad_rows = 6;
        b.normalize();
        assert_eq!(b.pads.len(), 12);
        assert_eq!(b.pads[10].path, "tusch.mp3");
        b.pad_cols = 0;
        b.widths = vec![1.0];
        b.normalize();
        assert_eq!(b.pad_cols, 1);
        assert_eq!(b.widths, WIDTHS.to_vec());
    }
}
