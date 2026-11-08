//! Die Show: alles, was gespeichert wird. Die Datei ist JSON und enthält die
//! verwendeten Gerätetypen als Kopie, damit sie auf jedem Rechner lädt.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::dmx::UNIVERSE_SIZE;
use crate::fixture::{ChannelDef, FixtureType};

pub const SHOW_FORMAT_VERSION: u32 = 1;

/// Feste Anzahl DMX-Universen jeder Show.
pub const UNIVERSES: u8 = 8;

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
    /// Drehung im Bühnenplan in Grad (im Uhrzeigersinn).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan_rotation: Option<f32>,
    /// Ausgangsgrenzen (Dimmer-Limit, Pan/Tilt-Bereich, Invertieren).
    #[serde(default, skip_serializing_if = "FixtureLimits::is_default")]
    pub limits: FixtureLimits,
    /// Lage im 3D-Visualisierer. `None` = automatisch aus dem Bühnenplan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub place3d: Option<crate::venue::Placement>,
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
    /// Runder Scheinwerfer (PAR, Spot, Moving Head). Ältere Shows mit
    /// „moving_head“ werden als rund gelesen.
    #[serde(alias = "moving_head")]
    Round,
    /// Quadratisches Gerät (Fluter, Panel).
    Square,
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
    pub const ALL: [PlanShape; 6] = [
        PlanShape::Round,
        PlanShape::Square,
        PlanShape::Matrix,
        PlanShape::BarHorizontal,
        PlanShape::BarVertical,
        PlanShape::Effect,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PlanShape::Round => "Rund (PAR/Spot)",
            PlanShape::Square => "Quadrat (Fluter/Panel)",
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
        if moving {
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

impl ChannelRef {
    /// Kanalnummer des virtuellen Dimmers (Geräte mit Farbkanälen, aber ohne
    /// eigenen Dimmer). Er wird wie ein Kanal gespeichert und gemischt, aber
    /// nicht ausgegeben: Er skaliert die Farbkanäle des Geräts.
    pub const VIRTUAL_DIMMER: u16 = u16::MAX;

    pub fn virtual_dimmer(fixture: FixtureId) -> Self {
        Self {
            fixture,
            channel: Self::VIRTUAL_DIMMER,
        }
    }

    pub fn is_virtual(self) -> bool {
        self.channel == Self::VIRTUAL_DIMMER
    }
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub id: SceneId,
    pub name: String,
    /// Szenen derselben Zone schließen sich gegenseitig aus.
    /// `None` = frei kombinierbar.
    pub zone: Option<ZoneId>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub loop_mode: LoopMode,
    /// Eigene Farbe der Szene; `None` = Farbe der Bank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
    /// Wiedergabe-Einstellungen (Dimmer, Speed, Ein-/Ausblenden …).
    #[serde(default, skip_serializing_if = "SceneSettings::is_default")]
    pub settings: SceneSettings,
    /// Effekte; eine Szene mit Effekten ist eine FX-Szene.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub effects: Vec<Effect>,
    /// In der Szenenübersicht ausgeblendet (Bank-Menü blendet wieder ein).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
}

/// Art eines Effekts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectKind {
    Steps,
    ColorFx,
    ChaserFx,
    MoveFx,
    ValueFx,
    CurveFx,
    Mappings,
    ColorMappings,
    /// Zeitleiste mit Szenen und Audio (kein weiterer Effekt möglich).
    SuperScene,
}

impl EffectKind {
    pub const ALL: [EffectKind; 9] = [
        EffectKind::Steps,
        EffectKind::ColorFx,
        EffectKind::ChaserFx,
        EffectKind::MoveFx,
        EffectKind::ValueFx,
        EffectKind::CurveFx,
        EffectKind::Mappings,
        EffectKind::ColorMappings,
        EffectKind::SuperScene,
    ];

    /// Effekt steht allein in einer Szene (STEPS, SUPER SCENE).
    pub fn exclusive(self) -> bool {
        matches!(self, EffectKind::Steps | EffectKind::SuperScene)
    }

    /// Anzeigename (Fachbegriff, nicht übersetzt).
    /// Effekt bewegt zugeordnete Kanäle („Features“).
    pub fn uses_features(self) -> bool {
        matches!(
            self,
            EffectKind::ChaserFx | EffectKind::ValueFx | EffectKind::CurveFx | EffectKind::Mappings
        )
    }

    pub fn label(self) -> &'static str {
        match self {
            EffectKind::Steps => "STEPS",
            EffectKind::ColorFx => "COLOR FX",
            EffectKind::ChaserFx => "CHASER FX",
            EffectKind::MoveFx => "MOVE FX",
            EffectKind::ValueFx => "VALUE FX",
            EffectKind::CurveFx => "CURVE FX",
            EffectKind::Mappings => "MAPPINGS",
            EffectKind::ColorMappings => "COLOR MAPPINGS",
            EffectKind::SuperScene => "SUPER SCENE",
        }
    }
}

/// Ein Effekt einer FX-Szene.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effect {
    pub id: u32,
    pub kind: EffectKind,
    /// In der Seitenleiste zugeklappt.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub collapsed: bool,
    /// Geräte, auf die der Effekt wirkt (Reihenfolge = Beam-Reihenfolge).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixtures: Vec<FixtureId>,
    /// Einstellungen eines COLOR FX bzw. (mit Grauwerten) eines VALUE FX.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::fx::ColorFx>,
    /// Vom Effekt bewegte Kanäle (CHASER, VALUE, CURVE, MAPPINGS).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<crate::fx::feature::Feature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chaser: Option<crate::fx::feature::ChaserFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<crate::fx::feature::CurveFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub motion: Option<crate::fx::motion::MoveFx>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<crate::fx::mapping::MappingFx>,
    /// Zeitleiste einer SUPER SCENE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeline: Option<crate::timeline::Timeline>,
}

impl Effect {
    pub fn new(id: u32, kind: EffectKind) -> Self {
        Self {
            id,
            kind,
            collapsed: false,
            fixtures: Vec::new(),
            color: match kind {
                EffectKind::ColorFx => Some(crate::fx::ColorFx::default()),
                EffectKind::ValueFx => Some(crate::fx::ColorFx::values()),
                _ => None,
            },
            features: if kind.uses_features() {
                vec![crate::fx::feature::Feature::new(
                    crate::fixture::ChannelKind::Dimmer,
                )]
            } else {
                Vec::new()
            },
            chaser: (kind == EffectKind::ChaserFx).then(Default::default),
            curve: (kind == EffectKind::CurveFx).then(Default::default),
            motion: (kind == EffectKind::MoveFx).then(Default::default),
            mapping: match kind {
                EffectKind::Mappings => Some(crate::fx::mapping::MappingFx::new(
                    crate::fx::mapping::MapPreset::Rainbow,
                    false,
                )),
                EffectKind::ColorMappings => Some(crate::fx::mapping::MappingFx::new(
                    crate::fx::mapping::MapPreset::Rainbow,
                    true,
                )),
                _ => None,
            },
            timeline: (kind == EffectKind::SuperScene).then(Default::default),
        }
    }

    /// Fehlende Einstellungen ergänzen (ältere Shows/Importe).
    pub fn ensure_settings(&mut self) {
        let fresh = Effect::new(self.id, self.kind);
        if self.color.is_none() {
            self.color = fresh.color;
        }
        if self.chaser.is_none() {
            self.chaser = fresh.chaser;
        }
        if self.curve.is_none() {
            self.curve = fresh.curve;
        }
        if self.motion.is_none() {
            self.motion = fresh.motion;
        }
        if self.mapping.is_none() {
            self.mapping = fresh.mapping;
        }
        if self.timeline.is_none() {
            self.timeline = fresh.timeline;
        }
    }
}

/// Art einer Szene (für Anzeige und Seitenleiste).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    Static,
    Fx,
    Super,
}

/// Bezug auf Bänke für Freigabe und Freigabeschutz.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BankScope {
    /// Keine Szenen.
    Off,
    /// Alle Szenen.
    All,
    /// Szenen derselben Bank.
    #[default]
    Bank,
    /// Szenen außerhalb der eigenen Bank.
    OutsideBank,
    /// Szenen bestimmter Bänke.
    Banks(Vec<ZoneId>),
}

impl BankScope {
    /// Trifft der Bezug eine Szene der Bank `other`, gesehen von `own`?
    pub fn matches(&self, own: Option<ZoneId>, other: Option<ZoneId>) -> bool {
        match self {
            BankScope::Off => false,
            BankScope::All => true,
            BankScope::Bank => own.is_some() && own == other,
            BankScope::OutsideBank => own.is_none() || own != other,
            BankScope::Banks(list) => other.is_some_and(|z| list.contains(&z)),
        }
    }
}

/// Einstellungen einer Szene in der Seitenleiste „Eigenschaften“.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SceneSettings {
    /// Szenen-Dimmer für die Intensität (0–255, Standard voll).
    pub dimmer: u8,
    /// Ablaufgeschwindigkeit in Prozent (100 = normal).
    pub speed_pct: u16,
    /// Phase in Grad (für Effekte).
    pub phase_deg: u16,
    /// Größe in Prozent (für Effekte).
    pub size_pct: u8,
    /// Einblendzeit beim Start.
    pub fade_in_ms: u32,
    /// Ausblendzeit beim Stoppen.
    pub fade_out_ms: u32,
    /// Tempo der Schritt-Überblendungen in Zehnteln (10 = 1.0 wie
    /// programmiert, 20 = doppelt so schnell; 1..=40).
    pub fade_speed: u8,
    /// Schritte rückwärts abspielen.
    pub reverse: bool,
    /// Priorität: höhere Priorität gewinnt bei Überschneidungen, auch wenn
    /// eine niedrigere später gestartet wurde.
    pub priority: i8,
    /// Durchläufe, wenn nicht endlos (`Scene::loop_mode == Once`).
    pub repeat_count: u16,
    /// Am Ende (nach den Durchläufen) diese Szenen starten.
    pub jump_to_scenes: Vec<SceneId>,
    /// Am Ende (nach den Durchläufen) freigeben.
    pub release_at_end: bool,
    /// Welche laufenden Szenen beim Start freigegeben werden.
    pub release_mode: BankScope,
    /// Von welchen Szenen diese Szene nicht freigegeben werden kann.
    pub protect: BankScope,
    /// Flash: läuft nur, solange die Szene gedrückt wird.
    pub flash: bool,
    /// Phasen-Versatz beim Start/Stoppen (für Effekte).
    pub phase_in_ms: u32,
    pub phase_out_ms: u32,
    /// SUPER SCENE: Dauer von „Ton ausfaden“ in ms (0 = Programmeinstellung).
    pub audio_fade_ms: u32,
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self {
            dimmer: 255,
            speed_pct: 100,
            phase_deg: 0,
            size_pct: 100,
            fade_in_ms: 0,
            fade_out_ms: 0,
            fade_speed: 10,
            reverse: false,
            priority: 0,
            repeat_count: 1,
            jump_to_scenes: Vec::new(),
            release_at_end: false,
            release_mode: BankScope::Bank,
            protect: BankScope::Off,
            flash: false,
            phase_in_ms: 0,
            phase_out_ms: 0,
            audio_fade_ms: 0,
        }
    }
}

impl SceneSettings {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
}

impl Scene {
    pub fn kind(&self) -> SceneKind {
        if self.effects.is_empty() {
            SceneKind::Static
        } else if self.timeline().is_some() {
            SceneKind::Super
        } else {
            SceneKind::Fx
        }
    }

    /// Zeitleiste, falls es eine SUPER SCENE ist.
    pub fn timeline(&self) -> Option<&crate::timeline::Timeline> {
        self.effects.iter().find_map(|e| e.timeline.as_ref())
    }

    pub fn timeline_mut(&mut self) -> Option<&mut crate::timeline::Timeline> {
        self.effects.iter_mut().find_map(|e| e.timeline.as_mut())
    }

    /// SUPER SCENE: einzigen Schritt auf die Länge der Zeitleiste setzen,
    /// damit Wiederholen/Freigeben am Ende wie bei anderen Szenen wirken.
    pub fn sync_timeline_length(&mut self) {
        let Some(len) = self.timeline().map(|t| t.length_ms()) else {
            return;
        };
        if self.steps.is_empty() {
            self.steps.push(Step::new(Vec::new()));
        }
        self.steps.truncate(1);
        self.steps[0].fade_ms = 0;
        self.steps[0].hold_ms = len.clamp(100, u64::from(u32::MAX)) as u32;
    }

    /// Szene mit STEPS-Effekt (Schrittfolge; kein weiterer Effekt möglich).
    pub fn has_steps(&self) -> bool {
        self.effects.iter().any(|e| e.kind == EffectKind::Steps)
    }

    /// Neuen Schritt nach `after` einfügen (Kopie dieses Schritts, Haltezeit
    /// 1 s); gibt seinen Index zurück.
    pub fn insert_step(&mut self, after: usize) -> usize {
        let mut st = self
            .steps
            .get(after)
            .cloned()
            .unwrap_or_else(|| Step::new(Vec::new()));
        st.hold_ms = 1000;
        st.fade_ms = 0;
        let at = (after + 1).min(self.steps.len());
        self.steps.insert(at, st);
        at
    }

    /// Schritt löschen (der letzte bleibt immer erhalten).
    pub fn remove_step(&mut self, i: usize) -> bool {
        if self.steps.len() <= 1 || i >= self.steps.len() {
            return false;
        }
        self.steps.remove(i);
        true
    }

    /// Effekt anhängen; gibt seine ID zurück.
    pub fn add_effect(&mut self, kind: EffectKind) -> u32 {
        let id = self.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1;
        self.effects.push(Effect::new(id, kind));
        id
    }

    /// Gesamtdauer eines Durchlaufs aller Schritte.
    pub fn duration_ms(&self) -> u64 {
        self.steps.iter().map(Step::duration_ms).sum()
    }

    /// Gesamtdauer setzen: Schritte werden anteilig gestreckt bzw. gestaucht
    /// (eine Szene mit einem Schritt bekommt diese Haltezeit).
    pub fn set_duration_ms(&mut self, ms: u64) {
        let old = self.duration_ms();
        if self.steps.is_empty() {
            return;
        }
        if old == 0 {
            let each = (ms / self.steps.len() as u64) as u32;
            for s in &mut self.steps {
                s.fade_ms = 0;
                s.hold_ms = each;
            }
            return;
        }
        let k = ms as f64 / old as f64;
        for s in &mut self.steps {
            s.fade_ms = (f64::from(s.fade_ms) * k).round() as u32;
            s.hold_ms = (f64::from(s.hold_ms) * k).round() as u32;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Zone {
    pub id: ZoneId,
    pub name: String,
    /// Akzentfarbe der Zone in der Oberfläche (RGB).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
    /// In STEUERN ausgeblendet (Szenen laufen trotzdem weiter).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
    /// Kompaktansicht in STEUERN: niedrige Szenenkarten nur mit Namen.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub compact: bool,
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
    /// Anzahl DMX-Universen – immer [`UNIVERSES`] (ältere Dateien werden beim Laden angepasst).
    pub universes: u8,
    pub fixture_types: Vec<FixtureType>,
    pub fixtures: Vec<Fixture>,
    pub zones: Vec<Zone>,
    pub scenes: Vec<Scene>,
    #[serde(default)]
    pub groups: Vec<Group>,
    /// Veranstaltungsort für den 3D-Visualisierer.
    #[serde(default)]
    pub venue: crate::venue::Venue,
    /// Tastatur- und MIDI-Zuordnungen.
    #[serde(default, skip_serializing_if = "crate::mapping::Mappings::is_empty")]
    pub mappings: crate::mapping::Mappings,
    /// Touch-Oberfläche (Seiten mit frei angeordneten Bedienelementen).
    #[serde(default, skip_serializing_if = "crate::touch::TouchLayout::is_empty")]
    pub touch: crate::touch::TouchLayout,
}

impl Default for Show {
    fn default() -> Self {
        Self::new("Neue Show", 2)
    }
}

impl Show {
    /// Neue Show. `_universes` wird ignoriert; jede Show hat [`UNIVERSES`] Universen.
    pub fn new(name: impl Into<String>, _universes: u8) -> Self {
        Self {
            format_version: SHOW_FORMAT_VERSION,
            name: name.into(),
            universes: UNIVERSES,
            fixture_types: Vec::new(),
            fixtures: Vec::new(),
            zones: Vec::new(),
            scenes: Vec::new(),
            groups: Vec::new(),
            venue: crate::venue::Venue::default(),
            mappings: Default::default(),
            touch: Default::default(),
        }
    }

    /// Ältere Shows: Szenen mit mehreren Schritten bekommen den
    /// STEPS-Effekt, damit ihre Schritte bearbeitbar sind.
    pub fn normalize(&mut self) {
        for fx in self.scenes.iter_mut().flat_map(|s| s.effects.iter_mut()) {
            fx.ensure_settings();
        }
        for sc in &mut self.scenes {
            if sc.steps.len() > 1 && !sc.has_steps() {
                sc.effects.insert(
                    0,
                    Effect::new(
                        sc.effects.iter().map(|e| e.id).max().unwrap_or(0) + 1,
                        EffectKind::Steps,
                    ),
                );
            }
        }
    }

    // ---------- Laden / Speichern ----------

    pub fn to_json(&self) -> Result<String, ShowError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, ShowError> {
        let mut show: Show = serde_json::from_str(json)?;
        if show.format_version > SHOW_FORMAT_VERSION {
            return Err(ShowError::Version(show.format_version));
        }
        show.universes = UNIVERSES;
        show.normalize();
        show.refine_channel_kinds();
        Ok(show)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), ShowError> {
        std::fs::write(path, self.to_json()?)?;
        Ok(())
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ShowError> {
        Self::from_json(&std::fs::read_to_string(path)?)
    }

    /// Kanalarten der gespeicherten Gerätetypen anhand der Kanalnamen
    /// verfeinern (ältere Shows kennen z. B. Laser oder Nebel noch nicht).
    /// Gibt die Anzahl geänderter Kanäle zurück.
    pub fn refine_channel_kinds(&mut self) -> usize {
        let mut n = 0;
        for ty in &mut self.fixture_types {
            for c in &mut ty.channels {
                let k = c.kind.refine(&c.name);
                if k != c.kind {
                    c.kind = k;
                    n += 1;
                }
            }
        }
        n
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

    /// Virtueller Dimmer eines Geräts, falls es einen braucht.
    pub fn virtual_dimmer(&self, id: FixtureId) -> Option<ChannelRef> {
        let f = self.fixture(id)?;
        self.type_of(f)?
            .needs_virtual_dimmer()
            .then(|| ChannelRef::virtual_dimmer(id))
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
            plan_rotation: None,
            limits: FixtureLimits::default(),
            place3d: None,
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

    /// Drehung eines Geräts im Bühnenplan (Grad, 0..360).
    pub fn plan_rotation(&self, id: FixtureId) -> f32 {
        self.fixture(id)
            .and_then(|f| f.plan_rotation)
            .unwrap_or(0.0)
            .rem_euclid(360.0)
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

    /// Kopien von Geräten patchen: jede an die erste freie Adresse ab
    /// `from` (wenn nötig in den folgenden Universen), in der gegebenen
    /// Reihenfolge. Namen werden weitergezählt („PAR 4“ → „PAR 5“), die
    /// Bühnenposition leicht versetzt. Liefert die neuen IDs.
    pub fn paste_fixtures(&mut self, src: &[Fixture], universe: u8, from: u16) -> Vec<FixtureId> {
        let mut out = Vec::new();
        let (mut u, mut a) = (universe, from.max(1));
        for f in src {
            let Some(ch) = self.fixture_type(&f.type_key).map(|t| t.channel_count()) else {
                continue;
            };
            // erste freie Stelle ab (u, a) suchen
            let mut place = None;
            while u < self.universes && place.is_none() {
                let mut addr = a;
                while u32::from(addr) + u32::from(ch) - 1 <= UNIVERSE_SIZE as u32 {
                    if self.check_placement(&f.type_key, u, addr, None).is_ok() {
                        place = Some((u, addr));
                        break;
                    }
                    addr += 1;
                }
                if place.is_none() {
                    u += 1;
                    a = 1;
                }
            }
            let Some((pu, pa)) = place else {
                break;
            };
            let id = self.next_fixture_id();
            let name = self.next_fixture_name(&f.name);
            let from = if self.fixture(f.id).is_some() {
                self.plan_position(f.id)
            } else {
                f.plan_pos
                    .unwrap_or_else(|| crate::plan::default_position(self.fixtures.len()))
            };
            let pos = self.free_plan_position(from);
            self.fixtures.push(Fixture {
                id,
                name,
                universe: pu,
                address: pa,
                plan_pos: Some(pos),
                ..f.clone()
            });
            out.push(id);
            u = pu;
            a = pa + ch;
        }
        out
    }

    /// Position im Bühnenplan (eigene oder automatische).
    /// Lage eines Geräts im 3D-Raum: gespeichert oder aus dem Bühnenplan
    /// abgeleitet (unter der Decke, nach unten strahlend).
    pub fn placement3d(&self, id: FixtureId) -> crate::venue::Placement {
        if let Some(p) = self.fixture(id).and_then(|f| f.place3d) {
            return p;
        }
        let [w, h, d] = self.venue.size;
        let [px, py] = self.plan_position(id);
        crate::venue::Placement {
            position: [
                (px - 0.5) * w * 0.8,
                (h - 0.35).max(1.5),
                (0.5 - py) * d * 0.8,
            ],
            rotation: [0.0; 3],
        }
    }

    pub fn plan_position(&self, id: FixtureId) -> [f32; 2] {
        let idx = self.fixtures.iter().position(|f| f.id == id).unwrap_or(0);
        self.fixture(id)
            .and_then(|f| f.plan_pos)
            .unwrap_or_else(|| crate::plan::default_position(idx))
    }

    /// Nächster freier Platz im Bühnenplan neben `from` (ringförmig nach
    /// außen gesucht), damit eingefügte Geräte nicht übereinander liegen.
    pub fn free_plan_position(&self, from: [f32; 2]) -> [f32; 2] {
        const STEP: f32 = 0.06;
        const MIN_DIST: f32 = 0.035;
        let taken: Vec<[f32; 2]> = self
            .fixtures
            .iter()
            .map(|f| self.plan_position(f.id))
            .collect();
        let free = |p: [f32; 2]| {
            taken
                .iter()
                .all(|q| (q[0] - p[0]).hypot(q[1] - p[1]) >= MIN_DIST)
        };
        let inside = |p: [f32; 2]| (0.02..=0.98).contains(&p[0]) && (0.02..=0.98).contains(&p[1]);
        for ring in 1..20i32 {
            // zuerst rechts daneben, dann links, dann darunter, zuletzt darüber
            let mut cands = Vec::new();
            for dy in -ring..=ring {
                for dx in -ring..=ring {
                    if dx.abs().max(dy.abs()) == ring {
                        cands.push((dx, dy));
                    }
                }
            }
            cands.sort_by_key(|(dx, dy)| (dy.abs(), -dx.signum(), -*dy, *dx));
            for (dx, dy) in cands {
                let p = [from[0] + dx as f32 * STEP, from[1] + dy as f32 * STEP];
                if inside(p) && free(p) {
                    return p;
                }
            }
        }
        from
    }

    /// Freier Name: endet `name` auf eine Zahl, wird bis zum ersten freien
    /// Namen weitergezählt, sonst „name 2“, „name 3“ ….
    fn next_fixture_name(&self, name: &str) -> String {
        let taken = |n: &str| self.fixtures.iter().any(|f| f.name == n);
        if !taken(name) {
            return name.to_string();
        }
        let digits = name.len() - name.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        let (stem, mut n) = if digits > 0 {
            let (s, d) = name.split_at(name.len() - digits);
            (s.to_string(), d.parse::<u32>().unwrap_or(1))
        } else {
            (format!("{name} "), 1)
        };
        loop {
            n += 1;
            let cand = format!("{stem}{n}");
            if !taken(&cand) {
                return cand;
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
            color: None,
            hidden: false,
            compact: false,
        });
        id
    }

    pub fn zone_mut(&mut self, id: ZoneId) -> Option<&mut Zone> {
        self.zones.iter_mut().find(|z| z.id == id)
    }

    /// Kopiert eine Bank samt ihrer Szenen; die Kopie steht direkt dahinter.
    pub fn duplicate_zone(&mut self, id: ZoneId, name: impl Into<String>) -> Option<ZoneId> {
        let pos = self.zones.iter().position(|z| z.id == id)?;
        let new_id = ZoneId(self.zones.iter().map(|z| z.id.0).max().unwrap_or(0) + 1);
        let mut zone = self.zones[pos].clone();
        zone.id = new_id;
        zone.name = name.into();
        zone.hidden = false;
        self.zones.insert(pos + 1, zone);
        let copies: Vec<Scene> = self
            .scenes
            .iter()
            .filter(|s| s.zone == Some(id))
            .cloned()
            .collect();
        for mut sc in copies {
            sc.id = SceneId(self.scenes.iter().map(|s| s.id.0).max().unwrap_or(0) + 1);
            sc.zone = Some(new_id);
            self.scenes.push(sc);
        }
        Some(new_id)
    }

    /// Löscht eine Bank mitsamt ihren Szenen.
    pub fn remove_zone_with_scenes(&mut self, id: ZoneId) {
        self.zones.retain(|z| z.id != id);
        self.scenes.retain(|s| s.zone != Some(id));
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

    /// Szenen ohne Bank in eine Bank übernehmen (erste Bank, sonst neue mit
    /// `name`). Gibt `true` zurück, wenn etwas geändert wurde.
    pub fn adopt_free_scenes(&mut self, name: impl Into<String>) -> bool {
        if !self.scenes.iter().any(|s| s.zone.is_none()) {
            return false;
        }
        let zone = match self.zones.first() {
            Some(z) => z.id,
            None => self.add_zone(name),
        };
        for s in self.scenes.iter_mut().filter(|s| s.zone.is_none()) {
            s.zone = Some(zone);
        }
        true
    }

    /// Szenen in eine Bank verschieben, vor `before` (sonst ans Ende der
    /// Bank). Die Reihenfolge der verschobenen Szenen bleibt erhalten.
    pub fn move_scenes(&mut self, ids: &[SceneId], zone: ZoneId, before: Option<SceneId>) {
        let moving = |id: SceneId| ids.contains(&id);
        // Ziel darf keine der verschobenen Szenen sein: nächste andere in der Bank
        let before = before.and_then(|b| {
            let start = self.scenes.iter().position(|s| s.id == b)?;
            self.scenes[start..]
                .iter()
                .find(|s| s.zone == Some(zone) && !moving(s.id))
                .map(|s| s.id)
        });
        let mut taken = Vec::new();
        self.scenes.retain(|s| {
            if moving(s.id) {
                taken.push(s.clone());
                false
            } else {
                true
            }
        });
        for s in &mut taken {
            s.zone = Some(zone);
        }
        let at = match before.and_then(|b| self.scenes.iter().position(|s| s.id == b)) {
            Some(i) => i,
            None => self
                .scenes
                .iter()
                .rposition(|s| s.zone == Some(zone))
                .map_or(self.scenes.len(), |i| i + 1),
        };
        self.scenes.splice(at..at, taken);
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
            color: None,
            settings: SceneSettings::default(),
            effects: Vec::new(),
            hidden: false,
        });
        id
    }

    /// Neue SUPER SCENE hinter der ersten Szene aus `ids`: die Szenen liegen
    /// nacheinander auf Spur 1.
    pub fn create_super_scene(
        &mut self,
        ids: &[SceneId],
        name: impl Into<String>,
    ) -> Option<SceneId> {
        let first = *ids.first()?;
        let pos = self.scenes.iter().position(|s| s.id == first)?;
        let zone = self.scenes[pos].zone;
        let id = self.add_scene(name, zone, vec![Step::new(Vec::new())]);
        let parts: Vec<Scene> = ids.iter().filter_map(|i| self.scene(*i).cloned()).collect();
        let sc = self.scene_mut(id)?;
        sc.add_effect(EffectKind::SuperScene);
        if let Some(tl) = sc.timeline_mut() {
            let mut at = 0;
            for p in &parts {
                tl.add_scene_clip(0, p, at);
                at += p.duration_ms().max(100);
            }
        }
        sc.sync_timeline_length();
        // direkt hinter die erste Szene
        let new = self.scenes.pop()?;
        self.scenes.insert(pos + 1, new);
        Some(id)
    }

    /// Kopiert eine Szene; die Kopie steht direkt hinter dem Original.
    pub fn duplicate_scene(&mut self, id: SceneId, name: impl Into<String>) -> Option<SceneId> {
        let pos = self.scenes.iter().position(|s| s.id == id)?;
        let mut copy = self.scenes[pos].clone();
        copy.id = SceneId(self.scenes.iter().map(|s| s.id.0).max().unwrap_or(0) + 1);
        copy.name = name.into();
        let new_id = copy.id;
        self.scenes.insert(pos + 1, copy);
        Some(new_id)
    }

    pub fn remove_scene(&mut self, id: SceneId) {
        self.scenes.retain(|s| s.id != id);
        for tl in self.scenes.iter_mut().filter_map(|s| s.timeline_mut()) {
            tl.forget_scene(id);
        }
    }
}

#[cfg(test)]
mod zone_tests {
    use super::*;

    #[test]
    fn move_scenes_between_banks() {
        let mut s = Show::new("T", 8);
        let z1 = s.add_zone("1");
        let z2 = s.add_zone("2");
        let a = s.add_scene("A", Some(z1), vec![]);
        let b = s.add_scene("B", Some(z1), vec![]);
        let c = s.add_scene("C", Some(z2), vec![]);
        let order = |s: &Show, z| -> Vec<SceneId> {
            s.scenes
                .iter()
                .filter(|x| x.zone == Some(z))
                .map(|x| x.id)
                .collect()
        };
        s.move_scenes(&[a], z2, Some(c));
        assert_eq!(order(&s, z2), vec![a, c]);
        assert_eq!(order(&s, z1), vec![b]);
        s.move_scenes(&[a, c], z1, None);
        assert_eq!(order(&s, z1), vec![b, a, c]);
        // vor sich selbst einfügen = vor die nächste nicht verschobene Szene
        s.move_scenes(&[a], z1, Some(a));
        assert_eq!(order(&s, z1), vec![b, a, c]);
        s.move_scenes(&[c], z1, Some(b));
        assert_eq!(order(&s, z1), vec![c, b, a]);
    }

    #[test]
    fn free_scenes_get_a_bank() {
        let mut s = Show::new("T", 8);
        let a = s.add_scene("A", None, vec![]);
        assert!(s.adopt_free_scenes("Bank 1"));
        let z = s.scene(a).unwrap().zone.unwrap();
        assert_eq!(s.zone(z).unwrap().name, "Bank 1");
        assert!(!s.adopt_free_scenes("x"));
        let b = s.add_scene("B", None, vec![]);
        s.adopt_free_scenes("x");
        assert_eq!(s.scene(b).unwrap().zone, Some(z));
        assert_eq!(s.zones.len(), 1);
    }

    #[test]
    fn duplicate_zone_copies_scenes_behind_original() {
        let mut s = Show::new("T", 8);
        let a = s.add_zone("A");
        let b = s.add_zone("B");
        s.add_scene("x", Some(a), vec![]);
        s.add_scene("y", Some(a), vec![]);
        s.add_scene("z", Some(b), vec![]);
        let c = s.duplicate_zone(a, "A (Kopie)").unwrap();
        assert_eq!(s.zones.iter().map(|z| z.id).collect::<Vec<_>>(), [a, c, b]);
        let copied: Vec<_> = s.scenes.iter().filter(|x| x.zone == Some(c)).collect();
        assert_eq!(copied.len(), 2);
        // neue, eindeutige IDs
        let mut ids: Vec<_> = s.scenes.iter().map(|x| x.id).collect();
        ids.dedup();
        assert_eq!(ids.len(), 5);
        let x = s.scenes[0].id;
        let dup = s.duplicate_scene(x, "x2").unwrap();
        assert_eq!(s.scenes[1].id, dup);
        assert_eq!(s.scenes[1].zone, Some(a));
        s.remove_scene(dup);
        s.remove_zone_with_scenes(a);
        assert_eq!(s.scenes.len(), 3);
        // Show-Format: ausgeblendet/Farbe nur gespeichert, wenn gesetzt
        let json = s.to_json().unwrap();
        assert!(!json.contains("hidden"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::builtin_fixture_types;

    #[test]
    fn steps_insert_remove_and_normalize() {
        let mut s = Show::new("T", 1);
        let id = s.add_scene(
            "A",
            None,
            vec![Step::new(Vec::new()), Step::new(Vec::new())],
        );
        s.normalize();
        let sc = s.scene_mut(id).unwrap();
        assert!(sc.has_steps());
        sc.steps[0].hold_ms = 300;
        sc.steps[0].fade_ms = 200;
        let at = sc.insert_step(0);
        assert_eq!(at, 1);
        assert_eq!(sc.steps.len(), 3);
        assert_eq!((sc.steps[1].hold_ms, sc.steps[1].fade_ms), (1000, 0));
        assert!(sc.remove_step(1));
        assert!(sc.remove_step(0));
        assert!(!sc.remove_step(0), "der letzte Schritt bleibt");
    }

    fn show() -> Show {
        let mut s = Show::new("Test", 2);
        for t in builtin_fixture_types() {
            s.upsert_fixture_type(t);
        }
        s
    }

    #[test]
    fn paste_fixtures_finds_free_addresses_and_names() {
        let mut s = show();
        let a = s
            .add_fixture("PAR 1", "generic/rgbw-par-6ch", 0, 1)
            .unwrap();
        let b = s
            .add_fixture("PAR 2", "generic/rgbw-par-6ch", 0, 13)
            .unwrap();
        let src: Vec<Fixture> = [a, b]
            .iter()
            .map(|i| s.fixture(*i).unwrap().clone())
            .collect();
        let ids = s.paste_fixtures(&src, 0, 1);
        let got: Vec<(String, u16)> = ids
            .iter()
            .map(|i| {
                let f = s.fixture(*i).unwrap();
                (f.name.clone(), f.address)
            })
            .collect();
        // 1..6 belegt, 7..12 frei, 13..18 belegt, danach 19
        assert_eq!(got, vec![("PAR 3".into(), 7), ("PAR 4".into(), 19)]);
        // volles Universum: weiter im nächsten
        let ids = s.paste_fixtures(&src, 0, 510);
        assert_eq!(s.fixture(ids[0]).unwrap().universe, 1);
        // keine zwei Geräte an derselben Stelle im Bühnenplan
        let pos: Vec<[f32; 2]> = s.fixtures.iter().map(|f| s.plan_position(f.id)).collect();
        for (i, a) in pos.iter().enumerate() {
            for b in &pos[i + 1..] {
                assert!((a[0] - b[0]).hypot(a[1] - b[1]) > 0.03, "{a:?} {b:?}");
            }
        }
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
        assert!(s.add_fixture("X", "generic/dimmer-1ch", 7, 1).is_ok());
        assert!(matches!(
            s.add_fixture("X", "generic/dimmer-1ch", 8, 1),
            Err(PatchError::BadUniverse(9, 8))
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
