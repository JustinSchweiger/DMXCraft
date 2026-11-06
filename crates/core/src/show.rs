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
id_type!(GroupId);

/// Ein gepatchtes Gerät.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    pub id: FixtureId,
    pub name: String,
    /// Schlüssel des Gerätetyps ([`FixtureType::key`]).
    pub type_key: String,
    /// 0-basiert (Universum 1 in der Oberfläche = 0 hier).
    pub universe: u8,
    /// Startadresse, 1-basiert (1..=512).
    pub address: u16,
    /// Position im Bühnenplan (0.0..=1.0 in beiden Achsen). `None` = automatisch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_pos: Option<[f32; 2]>,
    /// Darstellung im Bühnenplan. `None` = automatisch nach Gerätetyp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_shape: Option<PlanShape>,
    /// Größenfaktor im Bühnenplan (1.0 = normal).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_size: Option<f32>,
    /// Ausgangsgrenzen (Dimmer-Limit, Pan/Tilt-Bereich, Invertieren).
    #[serde(default, skip_serializing_if = "FixtureLimits::is_default")]
    pub limits: FixtureLimits,
}

/// Grenzen, die beim Ausgeben auf ein Gerät angewendet werden – unabhängig
/// von Szenen und Programmer (z. B. um zu helle Geräte zu zähmen oder Moving
/// Heads vom Publikum fernzuhalten).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct FixtureLimits {
    /// Niedrigster Dimmerwert, sobald das Gerät an ist (0 = keine Begrenzung).
    /// Ein Dimmerwert von 0 bleibt 0, damit Blackout und „aus“ funktionieren.
    pub dimmer_min: u8,
    /// Höchster Dimmerwert (255 = keine Begrenzung).
    pub dimmer_max: u8,
    pub pan_min: u8,
    pub pan_max: u8,
    pub tilt_min: u8,
    pub tilt_max: u8,
    pub invert_pan: bool,
    pub invert_tilt: bool,
    /// Pan und Tilt vertauschen (z. B. für seitlich montierte Geräte).
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

    /// Dimmerwert in den erlaubten Bereich abbilden (0 bleibt 0).
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

    /// Bildet einen 16-Bit-Wert (0..=65535) auf den erlaubten Bereich ab.
    pub fn map16(value: u16, min: u8, max: u8, invert: bool) -> u16 {
        let v = if invert { u16::MAX - value } else { value };
        let (lo, hi) = (u32::from(min) << 8, (u32::from(max) << 8) | 0xFF);
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        (lo + u32::from(v) * (hi - lo) / u32::from(u16::MAX)) as u16
    }
}

/// Form eines Geräts im Bühnenplan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanShape {
    /// Runder Scheinwerfer (PAR, Spot).
    Round,
    /// Quadratisches Gerät (Fluter, Panel).
    Square,
    /// Rund mit mehreren LEDs als Segmente (z. B. Moving Head mit LED-Ring).
    MovingHead,
    /// Quadratisches Pixel-Raster (Matrix-Panel).
    Matrix,
    /// Waagerechte Leiste (LED-Bar, Pixel-Leiste).
    BarHorizontal,
    /// Senkrechte Leiste.
    BarVertical,
    /// Effekt / Laser / Sonstiges.
    Effect,
}

impl PlanShape {
    pub const ALL: [PlanShape; 7] = [
        PlanShape::Round,
        PlanShape::Square,
        PlanShape::MovingHead,
        PlanShape::Matrix,
        PlanShape::BarHorizontal,
        PlanShape::BarVertical,
        PlanShape::Effect,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PlanShape::Round => "Rund (PAR/Spot)",
            PlanShape::Square => "Quadrat (Fluter/Panel)",
            PlanShape::MovingHead => "Rund mit LED-Segmenten",
            PlanShape::Matrix => "Pixel-Raster (Matrix)",
            PlanShape::BarHorizontal => "Leiste waagerecht",
            PlanShape::BarVertical => "Leiste senkrecht",
            PlanShape::Effect => "Effekt/Laser",
        }
    }

    /// Passende Form für einen Gerätetyp.
    pub fn guess(ty: &FixtureType) -> PlanShape {
        use crate::fixture::ChannelKind as K;
        let reds = ty.channels.iter().filter(|c| c.kind == K::Red).count();
        let moving = ty.has_kind(K::Pan) || ty.has_kind(K::Tilt);
        if moving && reds > 1 {
            PlanShape::MovingHead
        } else if moving {
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

/// Erlaubter Bereich für [`Fixture::plan_size`].
pub const PLAN_SIZE_RANGE: std::ops::RangeInclusive<f32> = 0.5..=4.0;

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
    /// Akzentfarbe der Zone in der Oberfläche (RGB).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
}

/// Eine benannte Geräteauswahl (z. B. „Moving Heads“, „PARs Traverse“).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub fixtures: Vec<FixtureId>,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Show {
    pub format_version: u32,
    pub name: String,
    /// Anzahl DMX-Universen (DVC Gold: 2).
    pub universes: u8,
    pub fixture_types: Vec<FixtureType>,
    pub fixtures: Vec<Fixture>,
    pub zones: Vec<Zone>,
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub groups: Vec<Group>,
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
            groups: Vec::new(),
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
            plan_pos: None,
            plan_shape: None,
            plan_size: None,
            limits: FixtureLimits::default(),
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
        for g in &mut self.groups {
            g.fixtures.retain(|f| *f != id);
        }
        for scene in &mut self.scenes {
            for step in &mut scene.steps {
                step.values.retain(|v| v.fixture != id);
            }
        }
    }

    /// Form eines Geräts im Bühnenplan (gespeichert oder nach Typ geschätzt).
    pub fn plan_shape(&self, id: FixtureId) -> PlanShape {
        let Some(f) = self.fixture(id) else {
            return PlanShape::Round;
        };
        f.plan_shape
            .or_else(|| self.type_of(f).map(PlanShape::guess))
            .unwrap_or(PlanShape::Round)
    }

    /// Größenfaktor eines Geräts im Bühnenplan.
    pub fn plan_size(&self, id: FixtureId) -> f32 {
        self.fixture(id)
            .and_then(|f| f.plan_size)
            .unwrap_or(1.0)
            .clamp(*PLAN_SIZE_RANGE.start(), *PLAN_SIZE_RANGE.end())
    }

    /// Kanalanzahl eines gepatchten Geräts.
    pub fn fixture_channels(&self, id: FixtureId) -> u16 {
        self.fixture(id)
            .and_then(|f| self.type_of(f))
            .map_or(0, |t| t.channel_count())
    }

    /// Belegung eines Universums: Index = Adresse − 1 → Gerät (falls belegt).
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
            color: None,
        });
        id
    }

    pub fn zone(&self, id: ZoneId) -> Option<&Zone> {
        self.zones.iter().find(|z| z.id == id)
    }

    // ---------- Gruppen ----------

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
        let g = s.add_group("Alle", vec![a, b]);
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
