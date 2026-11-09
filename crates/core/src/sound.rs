//! Sound-Player (Reiter SOUND): Player mit Playlisten und Jingle-Pads.
//! Hier nur die mit der Show gespeicherten Einstellungen; abgespielt wird in
//! der Oberfläche.

use serde::{Deserialize, Serialize};

/// Anzahl der Player und Pads.
pub const DECKS: usize = 2;
pub const PADS: usize = 12;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SoundBoard {
    pub decks: Vec<Deck>,
    pub pads: Vec<Pad>,
    /// Dauer von „Überblenden“.
    pub crossfade_ms: u32,
    /// Lautstärke aller Player und Pads (0–1).
    pub master: f32,
    /// Raster der Pads (Spalten × Zeilen).
    pub pad_cols: u8,
    pub pad_rows: u8,
    /// Breitenanteile der Spalten unten: Playlisten, dann Pads.
    pub widths: Vec<f32>,
}

/// Grenzen für das Pad-Raster.
pub const PAD_GRID_MAX: u8 = 8;
/// Standardaufteilung unten (Playlist A, Playlist B, Pads).
pub const WIDTHS: [f32; 3] = [1.0, 1.0, 1.15];
/// Kleinster Breitenanteil einer Spalte.
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

    /// Fehlende Player/Pads ergänzen (ältere oder von Hand bearbeitete Shows).
    pub fn normalize(&mut self) {
        self.decks
            .resize(self.decks.len().max(DECKS), Deck::default());
        self.pad_cols = self.pad_cols.clamp(1, PAD_GRID_MAX);
        self.pad_rows = self.pad_rows.clamp(1, PAD_GRID_MAX);
        let n = self.pad_count();
        self.pads.resize(self.pads.len().max(n), Pad::default());
        // leere Pads hinter dem Raster fallen weg; belegte bleiben erhalten
        // und erscheinen wieder, wenn das Raster größer wird
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

    /// Anzahl der sichtbaren Pads.
    pub fn pad_count(&self) -> usize {
        self.pad_cols as usize * self.pad_rows as usize
    }

    /// Alle verwendeten Dateien (für die Prüfung auf fehlende Dateien).
    pub fn paths(&self) -> impl Iterator<Item = &str> {
        self.decks
            .iter()
            .flat_map(|d| d.playlist.iter().map(|t| t.path.as_str()))
            .chain(self.pads.iter().map(|p| p.path.as_str()))
            .filter(|p| !p.is_empty())
    }

    /// Pfad überall ersetzen (Neu verknüpfen); Anzahl der Stellen.
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

/// Ein Player mit seiner Playlist.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Deck {
    pub playlist: Vec<Track>,
    /// Lautstärke (0–1).
    pub volume: f32,
    /// Titel endlos wiederholen.
    pub repeat: bool,
    /// Nach dem Ende den nächsten Titel der Playlist starten.
    pub auto_next: bool,
    /// Dauer von „Ausfaden“.
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

/// Titel einer Playlist.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    pub path: String,
    /// Länge (0 = unbekannt).
    #[serde(default)]
    pub length_ms: u64,
    /// Eigener Anzeigename (leer = Dateiname).
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

    /// Dateiname ohne Ordner und Endung.
    pub fn file_name(&self) -> String {
        file_name(&self.path)
    }

    /// Anzeigename: eigener Name, sonst Dateiname.
    pub fn label(&self) -> String {
        if self.name.trim().is_empty() {
            self.file_name()
        } else {
            self.name.clone()
        }
    }
}

/// Jingle-Pad.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Pad {
    /// Anzeigename (leer = Dateiname).
    pub name: String,
    pub path: String,
    pub color: Option<[u8; 3]>,
    /// Pegel (0–1).
    pub volume: f32,
    /// Endlos wiederholen, bis erneut gedrückt.
    pub repeat: bool,
    /// Beim Start alle anderen Pads stoppen.
    pub exclusive: bool,
    /// Länge (0 = unbekannt).
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

/// Dateiname ohne Ordner und Endung.
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
        old.decks[0].playlist[0].name = "Einmarsch".into();
        assert_eq!(old.decks[0].playlist[0].label(), "Einmarsch");
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
        // Pad 11 leer → weg, Pad 10 belegt → bleibt (versteckt)
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
