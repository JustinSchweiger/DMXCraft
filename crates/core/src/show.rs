//! Die Show: alles, was gespeichert wird. Die Datei ist JSON und enthält die
//! verwendeten Gerätetypen als Kopie, damit sie auf jedem Rechner lädt.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dmx::UNIVERSE_SIZE;
use crate::fixture::{ChannelDef, FixtureType};

pub const SHOW_FORMAT_VERSION: u32 = 1;

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

/// Ein gepatchtes Gerät.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: String,
    /// Schlüssel des Gerätetyps ([`FixtureType::key`]).
    pub type_key: String,
    /// 0-basiert (Universum 1 in der Oberfläche = 0 hier).
    pub universe: u8,
    /// Startadresse, 1-basiert (1..=512).
    pub address: u16,
}

/// Verweis auf einen Kanal eines Geräts (Kanalindex 0-basiert im Gerätetyp).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChannelRef {
    pub fixture: FixtureId,
    pub channel: u16,
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

/// Ein Schritt einer Szene: Kanalwerte, Überblendzeit und Haltezeit.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub values: Vec<ChannelValue>,
    /// Überblendzeit vom vorherigen Schritt in Millisekunden.
    pub fade_ms: u32,
    /// Haltezeit nach der Überblendung in Millisekunden.
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
    /// Nach dem letzten Schritt wieder von vorn.
    #[default]
    Loop,
    /// Nach dem letzten Schritt stehen bleiben.
    Once,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Scene {
    pub id: SceneId,
    pub name: String,
    /// Szenen derselben Zone schließen sich gegenseitig aus.
    /// `None` = frei kombinierbar.
    pub zone: Option<ZoneId>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub loop_mode: LoopMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub id: ZoneId,
    pub name: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatchError {
    #[error("Unbekannter Gerätetyp „{0}“")]
    UnknownType(String),
    #[error("Universum {0} existiert nicht (vorhanden: {1})")]
    BadUniverse(u8, u8),
    #[error("Adresse {address} + {channels} Kanäle passt nicht in ein Universum")]
    OutOfRange { address: u16, channels: u16 },
    #[error("Überschneidung mit „{0}“")]
    Overlap(String),
}

#[derive(Debug, Error)]
pub enum ShowError {
    #[error("Datei konnte nicht gelesen/geschrieben werden: {0}")]
    Io(#[from] std::io::Error),
    #[error("Ungültiges Show-Format: {0}")]
    Format(#[from] serde_json::Error),
    #[error("Show-Version {0} wird nicht unterstützt (maximal {SHOW_FORMAT_VERSION})")]
    Version(u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Show {
    pub format_version: u32,
    pub name: String,
    /// Anzahl DMX-Universen (DVC Gold: 2).
    pub universes: u8,
    pub fixture_types: Vec<FixtureType>,
    pub fixtures: Vec<Fixture>,
    pub zones: Vec<Zone>,
    pub scenes: Vec<Scene>,
}

impl Default for Show {
    fn default() -> Self {
        Self::new("Neue Show", 2)
    }
}

impl Show {
    pub fn new(name: impl Into<String>, universes: u8) -> Self {
        Self {
            format_version: SHOW_FORMAT_VERSION,
            name: name.into(),
            universes: universes.max(1),
            fixture_types: Vec::new(),
            fixtures: Vec::new(),
            zones: Vec::new(),
            scenes: Vec::new(),
        }
    }

    // ---------- Laden / Speichern ----------

    pub fn to_json(&self) -> Result<String, ShowError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, ShowError> {
        let show: Show = serde_json::from_str(json)?;
        if show.format_version > SHOW_FORMAT_VERSION {
            return Err(ShowError::Version(show.format_version));
        }
        Ok(show)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ShowError> {
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ShowError> {
        Self::from_json(&std::fs::read_to_string(path)?)
    }

    // ---------- Gerätetypen ----------

    pub fn fixture_type(&self, key: &str) -> Option<&FixtureType> {
        self.fixture_types.iter().find(|t| t.key == key)
    }

    /// Fügt einen Gerätetyp hinzu oder ersetzt ihn (gleicher Schlüssel).
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

    pub fn channel_def(&self, ch: ChannelRef) -> Option<&ChannelDef> {
        let f = self.fixture(ch.fixture)?;
        self.type_of(f)?.channels.get(usize::from(ch.channel))
    }

    fn next_fixture_id(&self) -> FixtureId {
        FixtureId(self.fixtures.iter().map(|f| f.id.0).max().unwrap_or(0) + 1)
    }

    /// Prüft, ob ein Gerät an diese Stelle passt. `ignore` erlaubt das
    /// Umpatchen eines vorhandenen Geräts.
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

    /// Entfernt ein Gerät und alle Szenenwerte, die es betreffen.
    pub fn remove_fixture(&mut self, id: FixtureId) {
        self.fixtures.retain(|f| f.id != id);
        for scene in &mut self.scenes {
            for step in &mut scene.steps {
                step.values.retain(|v| v.fixture != id);
            }
        }
    }

    /// Erste freie Adresse in einem Universum, an die `channels` Kanäle passen.
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

    // ---------- Zonen & Szenen ----------

    pub fn add_zone(&mut self, name: impl Into<String>) -> ZoneId {
        let id = ZoneId(self.zones.iter().map(|z| z.id.0).max().unwrap_or(0) + 1);
        self.zones.push(Zone {
            id,
            name: name.into(),
        });
        id
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
        });
        id
    }

    pub fn remove_scene(&mut self, id: SceneId) {
        self.scenes.retain(|s| s.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::builtin_fixture_types;

    fn show() -> Show {
        let mut s = Show::new("Test", 2);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        s
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
        assert!(matches!(
            s.add_fixture("X", "generic/dimmer-1ch", 2, 1),
            Err(PatchError::BadUniverse(3, 2))
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
        let z = s.add_zone("Farben");
        s.add_scene(
            "Rot",
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
    fn removing_fixture_cleans_scenes() {
        let mut s = show();
        let f = s.add_fixture("A", "generic/rgb-3ch", 0, 1).unwrap();
        let id = s.add_scene(
            "Rot",
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
