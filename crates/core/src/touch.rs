//! Touch surface: freely arranged controls on pages (grid), stored in the
//! show. The surface runs in the “Touch” tab and in the web app of the
//! touch remote.
//!
//! Buttons, faders and knobs trigger a [`Control`] like a keyboard/MIDI
//! mapping. Color wheel and XY pad set live values of the assigned fixtures
//! (color channels or pan/tilt).

use serde::{Deserialize, Serialize};

use crate::mapping::Control;
use crate::show::ChannelRef;
use crate::venue::model::Blob;
use crate::FixtureId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TouchLayout {
    pub pages: Vec<TouchPage>,
    pub next_id: u32,
    /// Edge length of a grid cell (pixels at 100 %).
    pub cell: f32,
    /// Snap to the grid when moving and resizing.
    pub snap: bool,
    /// Show grid cells.
    pub show_grid: bool,
    /// Images for backgrounds and buttons.
    pub images: Vec<TouchImage>,
}

impl Default for TouchLayout {
    fn default() -> Self {
        Self {
            pages: Vec::new(),
            next_id: 1,
            cell: 72.0,
            snap: true,
            show_grid: true,
            images: Vec::new(),
        }
    }
}

/// An image of the touch surface (PNG/JPEG).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TouchImage {
    pub id: u32,
    pub name: String,
    pub data: Blob,
}

impl TouchLayout {
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// Add an image; returns its ID.
    pub fn add_image(&mut self, name: impl Into<String>, data: Vec<u8>) -> u32 {
        let id = self.new_id();
        self.images.push(TouchImage {
            id,
            name: name.into(),
            data: Blob(data.into()),
        });
        id
    }

    pub fn image(&self, id: u32) -> Option<&TouchImage> {
        self.images.iter().find(|i| i.id == id)
    }

    /// Remove images no longer used.
    pub fn prune_images(&mut self) {
        let used: Vec<u32> = self
            .pages
            .iter()
            .flat_map(|p| {
                p.background
                    .into_iter()
                    .chain(p.items.iter().filter_map(|i| i.image))
            })
            .collect();
        self.images.retain(|i| used.contains(&i.id));
    }

    fn new_id(&mut self) -> u32 {
        self.next_id = self.next_id.max(1);
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Append a new page; returns its ID.
    pub fn add_page(&mut self, name: impl Into<String>) -> u32 {
        let id = self.new_id();
        self.pages.push(TouchPage {
            id,
            name: name.into(),
            items: Vec::new(),
            background: None,
            background_color: None,
        });
        id
    }

    pub fn page(&self, id: u32) -> Option<&TouchPage> {
        self.pages.iter().find(|p| p.id == id)
    }

    pub fn page_mut(&mut self, id: u32) -> Option<&mut TouchPage> {
        self.pages.iter_mut().find(|p| p.id == id)
    }

    /// Insert an element on a page at the first free position.
    pub fn add_item(&mut self, page: u32, mut item: TouchItem) -> Option<u32> {
        self.page(page)?;
        let id = self.new_id();
        let p = self.page_mut(page)?;
        item.id = id;
        let (x, y) = p.free_spot(item.w, item.h);
        item.x = x;
        item.y = y;
        p.items.push(item);
        Some(id)
    }

    /// Copy an element with a new ID (next to the original).
    pub fn duplicate(&mut self, page: u32, item: u32) -> Option<u32> {
        let copy = self
            .page(page)?
            .items
            .iter()
            .find(|i| i.id == item)?
            .clone();
        self.add_item(page, copy)
    }

    /// Remove elements whose scenes or fixtures no longer exist.
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
    /// Background image ([`TouchLayout::images`]).
    #[serde(default)]
    pub background: Option<u32>,
    #[serde(default)]
    pub background_color: Option<[u8; 3]>,
}

impl TouchPage {
    /// Is the area (in cells) free?
    pub fn is_free(&self, x: f32, y: f32, w: f32, h: f32, except: Option<u32>) -> bool {
        const EPS: f32 = 0.001;
        self.items.iter().filter(|i| Some(i.id) != except).all(|i| {
            x + w <= i.x + EPS || i.x + i.w <= x + EPS || y + h <= i.y + EPS || i.y + i.h <= y + EPS
        })
    }

    /// First free position (row by row, 16 columns wide, whole cells).
    pub fn free_spot(&self, w: f32, h: f32) -> (f32, f32) {
        const COLS: f32 = 16.0;
        for y in 0..200 {
            let mut x = 0.0;
            while x + w <= COLS.max(w) {
                if self.is_free(x, y as f32, w, h, None) {
                    return (x, y as f32);
                }
                x += 1.0;
            }
        }
        (0.0, 0.0)
    }
}

/// Kind of an element.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TouchKind {
    /// Text only (heading).
    Label,
    #[default]
    Button,
    Fader,
    Knob,
    /// Color wheel: sets the color of the assigned fixtures.
    Color,
    /// XY pad: sets pan/tilt of the assigned fixtures.
    Xy,
}

impl TouchKind {
    /// Default size in grid cells.
    pub fn default_size(self) -> (f32, f32) {
        match self {
            TouchKind::Label => (4.0, 1.0),
            TouchKind::Button => (2.0, 2.0),
            TouchKind::Fader => (1.0, 4.0),
            TouchKind::Knob => (2.0, 2.0),
            TouchKind::Color | TouchKind::Xy => (4.0, 4.0),
        }
    }

    /// Does this element control fixtures directly (instead of a [`Control`])?
    pub fn uses_fixtures(self) -> bool {
        matches!(self, TouchKind::Color | TouchKind::Xy)
    }
}

/// An element of the touch surface. Position and size in grid cells.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TouchItem {
    pub id: u32,
    pub kind: TouchKind,
    /// Position and size in grid cells (fractions too without snapping).
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub label: String,
    /// Background color (`None` = default).
    pub color: Option<[u8; 3]>,
    /// Text color (`None` = matching the background).
    pub text_color: Option<[u8; 3]>,
    /// Background image ([`TouchLayout::images`]).
    pub image: Option<u32>,
    /// Icon from the icon library (name).
    pub icon: Option<String>,
    /// What is triggered (button, fader, knob).
    pub control: Option<Control>,
    /// Fixed channels for [`Control::Channels`].
    pub channels: Vec<ChannelRef>,
    /// Fixtures for color wheel and XY pad.
    pub fixtures: Vec<FixtureId>,
    /// Button only active while it is pressed.
    pub flash: bool,
    /// Text size (points, 0 = automatic).
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
            x: 0.0,
            y: 0.0,
            w,
            h,
            label: String::new(),
            color: None,
            text_color: None,
            image: None,
            icon: None,
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
        let p = t.add_page("Page 1");
        let a = t.add_item(p, TouchItem::new(TouchKind::Button)).unwrap();
        let b = t.add_item(p, TouchItem::new(TouchKind::Button)).unwrap();
        let page = t.page(p).unwrap();
        let (ia, ib) = (&page.items[0], &page.items[1]);
        assert_ne!(a, b);
        assert_eq!((ia.x, ia.y), (0.0, 0.0));
        assert_eq!((ib.x, ib.y), (2.0, 0.0));
        assert!(!page.is_free(1.0, 1.0, 1.0, 1.0, None));
        assert!(page.is_free(1.0, 1.0, 1.0, 1.0, Some(a)));
        assert!(page.is_free(4.0, 0.0, 0.5, 0.5, None));
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

    #[test]
    fn images_are_pruned_and_old_integer_positions_load() {
        let mut t = TouchLayout::default();
        let p = t.add_page("A");
        let img = t.add_image("bg.png", vec![1, 2, 3]);
        t.add_image("unused.png", vec![4]);
        t.page_mut(p).unwrap().background = Some(img);
        t.prune_images();
        assert_eq!(t.images.len(), 1);
        // older files: whole numbers
        let json = r#"{"pages":[{"id":1,"name":"A","items":[{"id":2,"kind":"button","x":1,"y":2,"w":2,"h":2}]}]}"#;
        let old: TouchLayout = serde_json::from_str(json).unwrap();
        assert_eq!(old.pages[0].items[0].x, 1.0);
        assert!(old.snap);
    }
}
