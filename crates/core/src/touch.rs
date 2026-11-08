//! Touch-Oberfläche: frei angeordnete Bedienelemente auf Seiten (Raster),
//! gespeichert in der Show. Die Oberfläche läuft im Reiter „Touch“ und
//! später auf einer mobilen App.
//!
//! Schaltflächen, Fader und Drehregler lösen ein [`Control`] aus wie eine
//! Tastatur-/MIDI-Zuordnung. Farbrad und XY-Feld setzen Live-Werte der
//! hinterlegten Geräte (Farbkanäle bzw. Pan/Tilt).

use serde::{Deserialize, Serialize};

use crate::mapping::Control;
use crate::show::ChannelRef;
use crate::FixtureId;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TouchLayout {
    pub pages: Vec<TouchPage>,
    pub next_id: u32,
}

impl TouchLayout {
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    fn new_id(&mut self) -> u32 {
        self.next_id = self.next_id.max(1);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Neue Seite anhängen; gibt ihre ID zurück.
    pub fn add_page(&mut self, name: impl Into<String>) -> u32 {
        let id = self.new_id();
        self.pages.push(TouchPage {
            id,
            name: name.into(),
            items: Vec::new(),
        });
        id
    }

    pub fn page(&self, id: u32) -> Option<&TouchPage> {
        self.pages.iter().find(|p| p.id == id)
    }

    pub fn page_mut(&mut self, id: u32) -> Option<&mut TouchPage> {
        self.pages.iter_mut().find(|p| p.id == id)
    }

    /// Element auf einer Seite an der ersten freien Stelle einfügen.
    pub fn add_item(&mut self, page: u32, mut item: TouchItem) -> Option<u32> {
        let id = self.new_id();
        let p = self.page_mut(page)?;
        item.id = id;
        let (x, y) = p.free_spot(item.w, item.h);
        item.x = x;
        item.y = y;
        p.items.push(item);
        Some(id)
    }

    /// Element mit neuer ID kopieren (neben das Original).
    pub fn duplicate(&mut self, page: u32, item: u32) -> Option<u32> {
        let copy = self
            .page(page)?
            .items
            .iter()
            .find(|i| i.id == item)?
            .clone();
        self.add_item(page, copy)
    }

    /// Elemente entfernen, deren Szenen oder Geräte es nicht mehr gibt.
    pub fn retain(
        &mut self,
        scene_exists: impl Fn(crate::SceneId) -> bool,
        fixture_exists: impl Fn(FixtureId) -> bool,
    ) {
        for p in &mut self.pages {
            for i in &mut p.items {
                i.fixtures.retain(|f| fixture_exists(*f));
                i.channels.retain(|c| fixture_exists(c.fixture));
                let gone = match i.control {
                    Some(Control::Scene {
                        target: crate::mapping::SceneTarget::Scene { id },
                    }) => !scene_exists(id),
                    Some(Control::Fixture { id }) => !fixture_exists(id),
                    Some(Control::Channel { channel }) => !fixture_exists(channel.fixture),
                    _ => false,
                };
                if gone {
                    i.control = None;
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TouchPage {
    pub id: u32,
    pub name: String,
    pub items: Vec<TouchItem>,
}

impl TouchPage {
    /// Ist das Feld (Zellen) frei?
    pub fn is_free(&self, x: i32, y: i32, w: i32, h: i32, except: Option<u32>) -> bool {
        self.items
            .iter()
            .filter(|i| Some(i.id) != except)
            .all(|i| x + w <= i.x || i.x + i.w <= x || y + h <= i.y || i.y + i.h <= y)
    }

    /// Erste freie Stelle (zeilenweise, 16 Spalten breit).
    pub fn free_spot(&self, w: i32, h: i32) -> (i32, i32) {
        const COLS: i32 = 16;
        for y in 0..200 {
            for x in 0..=(COLS - w).max(0) {
                if self.is_free(x, y, w, h, None) {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }
}

/// Art eines Elements.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TouchKind {
    /// Nur Text (Überschrift).
    Label,
    #[default]
    Button,
    Fader,
    Knob,
    /// Farbrad: setzt die Farbe der hinterlegten Geräte.
    Color,
    /// XY-Feld: setzt Pan/Tilt der hinterlegten Geräte.
    Xy,
}

impl TouchKind {
    /// Standardgröße in Rasterzellen.
    pub fn default_size(self) -> (i32, i32) {
        match self {
            TouchKind::Label => (4, 1),
            TouchKind::Button => (2, 2),
            TouchKind::Fader => (1, 4),
            TouchKind::Knob => (2, 2),
            TouchKind::Color | TouchKind::Xy => (4, 4),
        }
    }

    /// Steuert dieses Element Geräte direkt (statt eines [`Control`])?
    pub fn uses_fixtures(self) -> bool {
        matches!(self, TouchKind::Color | TouchKind::Xy)
    }
}

/// Ein Element der Touch-Oberfläche. Position und Größe in Rasterzellen.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TouchItem {
    pub id: u32,
    pub kind: TouchKind,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub label: String,
    /// Hintergrundfarbe (`None` = Standard).
    pub color: Option<[u8; 3]>,
    /// Was ausgelöst wird (Schaltfläche, Fader, Drehregler).
    pub control: Option<Control>,
    /// Feste Kanäle bei [`Control::Channels`].
    pub channels: Vec<ChannelRef>,
    /// Geräte für Farbrad und XY-Feld.
    pub fixtures: Vec<FixtureId>,
    /// Schaltfläche nur aktiv, solange sie gedrückt ist.
    pub flash: bool,
    /// Textgröße (Punkte, 0 = automatisch).
    pub text_size: f32,
}

impl Default for TouchItem {
    fn default() -> Self {
        Self::new(TouchKind::Button)
    }
}

impl TouchItem {
    pub fn new(kind: TouchKind) -> Self {
        let (w, h) = kind.default_size();
        Self {
            id: 0,
            kind,
            x: 0,
            y: 0,
            w,
            h,
            label: String::new(),
            color: None,
            control: None,
            channels: Vec::new(),
            fixtures: Vec::new(),
            flash: false,
            text_size: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_find_free_spots() {
        let mut t = TouchLayout::default();
        let p = t.add_page("Seite 1");
        let a = t.add_item(p, TouchItem::new(TouchKind::Button)).unwrap();
        let b = t.add_item(p, TouchItem::new(TouchKind::Button)).unwrap();
        let page = t.page(p).unwrap();
        let (ia, ib) = (&page.items[0], &page.items[1]);
        assert_ne!(a, b);
        assert_eq!((ia.x, ia.y), (0, 0));
        assert_eq!((ib.x, ib.y), (2, 0));
        assert!(!page.is_free(1, 1, 1, 1, None));
        assert!(page.is_free(1, 1, 1, 1, Some(a)));
        let d = t.duplicate(p, a).unwrap();
        assert_eq!(t.page(p).unwrap().items.len(), 3);
        assert_ne!(d, a);
    }

    #[test]
    fn json_roundtrip_and_cleanup() {
        let mut t = TouchLayout::default();
        let p = t.add_page("A");
        let mut i = TouchItem::new(TouchKind::Button);
        i.control = Some(Control::Fixture { id: FixtureId(4) });
        i.color = Some([255, 0, 0]);
        t.add_item(p, i);
        let s = serde_json::to_string(&t).unwrap();
        let back: TouchLayout = serde_json::from_str(&s).unwrap();
        assert_eq!(back, t);
        let mut t2 = back;
        t2.retain(|_| true, |f| f != FixtureId(4));
        assert_eq!(t2.pages[0].items[0].control, None);
    }
}
