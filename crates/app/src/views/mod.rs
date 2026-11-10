//! Views. Every view is a type with its own state that implements [`View`];
//! shared data comes via [`ViewCtx`].

pub mod control;
pub mod fixture_menu;
pub mod profile_builder;
pub mod quick;
pub mod setup;
pub mod sound;
pub mod touch;
pub mod windows;

use dmxcraft_core::{DmxFrame, Engine};
use eframe::egui::Ui;

use crate::state::AppState;

/// Everything a view needs for drawing.
pub struct ViewCtx<'a> {
    pub engine: &'a mut Engine,
    pub state: &'a mut AppState,
    /// Last output DMX frame (for live displays).
    pub frame: &'a DmxFrame,
}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>);
}
