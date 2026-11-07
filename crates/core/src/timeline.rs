//! Zeitleiste einer SUPER SCENE: Spuren mit Blöcken (Szenen oder Audio),
//! wie in einem Schnittprogramm. Die Engine spielt die Blöcke der Szenen
//! ab; Audio gibt die Oberfläche wieder.

use serde::{Deserialize, Serialize};

use crate::show::SceneId;

/// Raster der Zeitleiste.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GridMode {
    #[default]
    Time,
    Beat,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Timeline {
    pub tracks: Vec<Track>,
    /// Laufende Nummer für Spur- und Block-IDs.
    pub next_id: u32,
    /// Längenänderung wiederholt die Szene (Schleife) …
    pub loop_resize: bool,
    /// … oder dehnt/staucht sie (Geschwindigkeit).
    pub stretch_resize: bool,
    /// Blöcke dürfen sich auf einer Spur überlappen.
    pub allow_overlap: bool,
    pub auto_scroll: bool,
    pub snap: bool,
    pub grid: GridMode,
    pub bpm: f32,
    pub offset_ms: u32,
}

impl Default for Timeline {
    fn default() -> Self {
        let mut t = Self {
            tracks: Vec::new(),
            next_id: 1,
            loop_resize: true,
            stretch_resize: false,
            allow_overlap: true,
            auto_scroll: true,
            snap: true,
            grid: GridMode::Time,
            bpm: 120.0,
            offset_ms: 0,
        };
        for _ in 0..4 {
            t.add_track(None);
        }
        t
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: u32,
    /// Stumm (Auge aus).
    #[serde(default)]
    pub muted: bool,
    /// Gesperrt (Schloss).
    #[serde(default)]
    pub locked: bool,
    /// Unterspuren (Dimmer/Lautstärke) sichtbar.
    #[serde(default)]
    pub expanded: bool,
    #[serde(default)]
    pub clips: Vec<Clip>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClipContent {
    Scene(SceneId),
    /// Audiodatei; `offset_ms` = Beginn innerhalb der Datei, `length_ms` =
    /// Länge der Datei (0 = unbekannt).
    Audio {
        path: String,
        #[serde(default)]
        offset_ms: u64,
        #[serde(default)]
        length_ms: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Clip {
    pub id: u32,
    pub start_ms: u64,
    pub duration_ms: u64,
    #[serde(default)]
    pub fade_in_ms: u32,
    #[serde(default)]
    pub fade_out_ms: u32,
    /// Szene wiederholt sich im Block (sonst bleibt der letzte Stand).
    #[serde(default = "yes")]
    pub looped: bool,
    /// Länge an das Tempo (ganze Schläge) anpassen.
    #[serde(default)]
    pub tempo_sync: bool,
    /// Abspielgeschwindigkeit der Szene (Dehnen).
    #[serde(default = "one")]
    pub speed: f32,
    /// Dimmer (Szene) bzw. Lautstärke (Audio), 0..=1.
    #[serde(default = "one")]
    pub level: f32,
    pub content: ClipContent,
}

fn yes() -> bool {
    true
}

fn one() -> f32 {
    1.0
}

impl Clip {
    pub fn end_ms(&self) -> u64 {
        self.start_ms + self.duration_ms
    }

    pub fn scene(&self) -> Option<SceneId> {
        match self.content {
            ClipContent::Scene(id) => Some(id),
            ClipContent::Audio { .. } => None,
        }
    }

    /// Pegel durch Ein-/Ausblenden zum Zeitpunkt `t` (0..=1, ohne `level`).
    pub fn fade_at(&self, t: u64) -> f32 {
        let since = t.saturating_sub(self.start_ms) as f32;
        let until = self.end_ms().saturating_sub(t) as f32;
        let a = if self.fade_in_ms > 0 {
            (since / self.fade_in_ms as f32).min(1.0)
        } else {
            1.0
        };
        let b = if self.fade_out_ms > 0 {
            (until / self.fade_out_ms as f32).min(1.0)
        } else {
            1.0
        };
        a.min(b)
    }
}

impl Timeline {
    fn next(&mut self) -> u32 {
        let id = self.next_id.max(1);
        self.next_id = id + 1;
        id
    }

    /// Spur einfügen (vor `at`, sonst am Ende); gibt ihre ID zurück.
    pub fn add_track(&mut self, at: Option<usize>) -> u32 {
        let id = self.next();
        let t = Track {
            id,
            muted: false,
            locked: false,
            expanded: false,
            clips: Vec::new(),
        };
        match at {
            Some(i) if i <= self.tracks.len() => self.tracks.insert(i, t),
            _ => self.tracks.push(t),
        }
        id
    }

    /// Ende des letzten Blocks.
    pub fn length_ms(&self) -> u64 {
        self.tracks
            .iter()
            .flat_map(|t| t.clips.iter())
            .map(Clip::end_ms)
            .max()
            .unwrap_or(0)
    }

    pub fn clip(&self, id: u32) -> Option<(usize, &Clip)> {
        self.tracks
            .iter()
            .enumerate()
            .find_map(|(i, t)| t.clips.iter().find(|c| c.id == id).map(|c| (i, c)))
    }

    pub fn clip_mut(&mut self, id: u32) -> Option<&mut Clip> {
        self.tracks
            .iter_mut()
            .flat_map(|t| t.clips.iter_mut())
            .find(|c| c.id == id)
    }

    /// Block anlegen; gibt seine ID zurück.
    /// Szenenblock einfügen: Länge = ein Durchlauf der Szene, Ein-/Ausblendzeit
    /// aus den Szeneneinstellungen.
    pub fn add_scene_clip(
        &mut self,
        track: usize,
        scene: &crate::show::Scene,
        start_ms: u64,
    ) -> u32 {
        let dur = scene.duration_ms().max(100);
        let id = self.add_clip(track, ClipContent::Scene(scene.id), start_ms, dur);
        if let Some(c) = self.clip_mut(id) {
            let max = u32::try_from(c.duration_ms).unwrap_or(u32::MAX);
            c.fade_in_ms = scene.settings.fade_in_ms.min(max);
            c.fade_out_ms = scene.settings.fade_out_ms.min(max);
        }
        id
    }

    pub fn add_clip(
        &mut self,
        track: usize,
        content: ClipContent,
        start_ms: u64,
        duration_ms: u64,
    ) -> u32 {
        while self.tracks.len() <= track {
            self.add_track(None);
        }
        let id = self.next();
        self.tracks[track].clips.push(Clip {
            id,
            start_ms,
            duration_ms: duration_ms.max(40),
            fade_in_ms: 0,
            fade_out_ms: 0,
            looped: true,
            tempo_sync: false,
            speed: 1.0,
            level: 1.0,
            content,
        });
        id
    }

    pub fn remove_clip(&mut self, id: u32) -> Option<Clip> {
        for t in &mut self.tracks {
            if let Some(i) = t.clips.iter().position(|c| c.id == id) {
                return Some(t.clips.remove(i));
            }
        }
        None
    }

    /// Block auf eine andere Spur legen.
    pub fn move_clip_to_track(&mut self, id: u32, track: usize) {
        if track >= self.tracks.len() {
            return;
        }
        if let Some(c) = self.remove_clip(id) {
            self.tracks[track].clips.push(c);
        }
    }

    /// Block an der Stelle `at` teilen; gibt die ID des zweiten Teils zurück.
    pub fn split_clip(&mut self, id: u32, at: u64) -> Option<u32> {
        let (track, c) = self.clip(id).map(|(t, c)| (t, c.clone()))?;
        if at <= c.start_ms + 40 || at + 40 >= c.end_ms() {
            return None;
        }
        let first = at - c.start_ms;
        let new_id = self.next();
        let mut second = c.clone();
        second.id = new_id;
        second.start_ms = at;
        second.duration_ms = c.duration_ms - first;
        second.fade_in_ms = 0;
        if let ClipContent::Audio { offset_ms, .. } = &mut second.content {
            *offset_ms += first;
        }
        if let Some(a) = self.clip_mut(id) {
            a.duration_ms = first;
            a.fade_out_ms = 0;
        }
        self.tracks[track].clips.push(second);
        Some(new_id)
    }

    /// Würde der Block (Spur, Bereich) einen anderen überlappen?
    pub fn overlaps(&self, track: usize, start: u64, end: u64, except: u32) -> bool {
        self.tracks.get(track).is_some_and(|t| {
            t.clips
                .iter()
                .any(|c| c.id != except && c.start_ms < end && start < c.end_ms())
        })
    }

    /// Blöcke einfügen (Spur und Zeit relativ zueinander wie in `clips`):
    /// die Gruppe beginnt auf Spur `track` an der ersten freien Stelle ab
    /// `at`, ohne andere Blöcke zu überdecken. Liefert die neuen IDs.
    pub fn paste_group(&mut self, clips: &[(usize, Clip)], track: usize, at: u64) -> Vec<u32> {
        let (Some(first), Some(base)) = (
            clips.iter().map(|(_, c)| c.start_ms).min(),
            clips.iter().map(|(t, _)| *t).min(),
        ) else {
            return Vec::new();
        };
        let parts: Vec<(usize, u64, &Clip)> = clips
            .iter()
            .map(|(t, c)| (track + (t - base), c.start_ms - first, c))
            .collect();
        // so lange nach hinten schieben, bis nichts mehr kollidiert
        let mut shift = at;
        'search: for _ in 0..10_000 {
            for &(ti, rel, c) in &parts {
                let (s, e) = (shift + rel, shift + rel + c.duration_ms);
                if let Some(hit) = self.tracks.get(ti).and_then(|t| {
                    t.clips
                        .iter()
                        .filter(|o| o.start_ms < e && s < o.end_ms())
                        .map(Clip::end_ms)
                        .max()
                }) {
                    shift = hit.saturating_sub(rel).max(shift + 1);
                    continue 'search;
                }
            }
            break;
        }
        parts
            .into_iter()
            .map(|(ti, rel, c)| {
                let start = shift + rel;
                let id = self.add_clip(ti, c.content.clone(), start, c.duration_ms);
                if let Some(n) = self.clip_mut(id) {
                    *n = Clip {
                        id,
                        start_ms: start,
                        ..c.clone()
                    };
                }
                id
            })
            .collect()
    }

    /// Länge eines Schlags in ms.
    pub fn beat_ms(&self) -> f64 {
        60_000.0 / f64::from(self.bpm.clamp(20.0, 400.0))
    }

    /// Blöcke, die eine Szene enthalten (für Aufräumen beim Löschen).
    pub fn forget_scene(&mut self, id: SceneId) {
        for t in &mut self.tracks {
            t.clips.retain(|c| c.scene() != Some(id));
        }
    }

    /// Aktive Szenenblöcke zum Zeitpunkt `t` in Spurreihenfolge.
    pub fn active_at(&self, t: u64) -> impl Iterator<Item = (usize, &Clip)> {
        self.tracks
            .iter()
            .enumerate()
            .filter(|(_, tr)| !tr.muted)
            .flat_map(move |(i, tr)| {
                tr.clips
                    .iter()
                    .filter(move |c| c.start_ms <= t && t < c.end_ms())
                    .map(move |c| (i, c))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_scene_from_scenes_keeps_fades() {
        let mut show = crate::show::Show::default();
        let a = show.add_scene("A", None, vec![crate::show::Step::new(Vec::new())]);
        let b = show.add_scene("B", None, vec![crate::show::Step::new(Vec::new())]);
        show.scene_mut(a).unwrap().settings.fade_in_ms = 400;
        show.scene_mut(a).unwrap().settings.fade_out_ms = 600;
        let s = show.create_super_scene(&[a, b], "S").unwrap();
        assert_eq!(show.scenes[1].id, s);
        let sc = show.scene(s).unwrap();
        let tl = sc.timeline().unwrap();
        let clips = &tl.tracks[0].clips;
        assert_eq!(clips.len(), 2);
        assert_eq!((clips[0].fade_in_ms, clips[0].fade_out_ms), (400, 600));
        assert_eq!(clips[1].start_ms, clips[0].end_ms());
        assert_eq!(sc.duration_ms(), tl.length_ms());
    }

    #[test]
    fn paste_finds_free_place_and_keeps_gaps() {
        let mut t = Timeline::default();
        let a = t.add_clip(1, ClipContent::Scene(SceneId(1)), 0, 1000);
        let b = t.add_clip(2, ClipContent::Scene(SceneId(2)), 1500, 500);
        let group: Vec<(usize, Clip)> = [a, b]
            .iter()
            .map(|id| {
                let (tr, c) = t.clip(*id).unwrap();
                (tr, c.clone())
            })
            .collect();
        // dreimal an derselben Stelle auf Spur 3 einfügen: hintereinander
        let mut starts = Vec::new();
        for _ in 0..3 {
            let ids = t.paste_group(&group[..1], 3, 200);
            starts.push(t.clip(ids[0]).unwrap().1.start_ms);
        }
        assert_eq!(starts, vec![200, 1200, 2200]);
        // Gruppe über zwei Spuren: Abstand und Spurversatz bleiben
        let ids = t.paste_group(&group, 0, 0);
        let (ta, ca) = t.clip(ids[0]).map(|(x, c)| (x, c.clone())).unwrap();
        let (tb, cb) = t.clip(ids[1]).map(|(x, c)| (x, c.clone())).unwrap();
        assert_eq!((ta, tb), (0, 1));
        assert_eq!(cb.start_ms - ca.start_ms, 1500);
        // Spur 1 war bei 0..1000 belegt -> Gruppe muss so weit nach hinten,
        // dass der zweite Block (Spur 1, +1500) frei liegt: erster Start 0 geht
        assert_eq!(ca.start_ms, 0);
    }

    #[test]
    fn split_and_length() {
        let mut t = Timeline::default();
        let a = t.add_clip(0, ClipContent::Scene(SceneId(1)), 1000, 5000);
        assert_eq!(t.length_ms(), 6000);
        let b = t.split_clip(a, 3000).unwrap();
        assert_eq!(t.clip(a).unwrap().1.duration_ms, 2000);
        assert_eq!(t.clip(b).unwrap().1.start_ms, 3000);
        assert_eq!(t.clip(b).unwrap().1.duration_ms, 3000);
        assert!(t.overlaps(0, 2500, 2600, 0));
        assert!(!t.overlaps(1, 2500, 2600, 0));
    }

    #[test]
    fn fades() {
        let mut t = Timeline::default();
        let a = t.add_clip(0, ClipContent::Scene(SceneId(1)), 0, 1000);
        let c = t.clip_mut(a).unwrap();
        c.fade_in_ms = 200;
        c.fade_out_ms = 200;
        assert!((c.fade_at(100) - 0.5).abs() < 1e-3);
        assert!((c.fade_at(500) - 1.0).abs() < 1e-3);
        assert!((c.fade_at(900) - 0.5).abs() < 1e-3);
    }
}
