//! Ansichten. Jede Ansicht ist ein Typ mit eigenem Zustand, der [`View`]
//! implementiert; gemeinsame Daten kommen über [`ViewCtx`].

pub mod control;
pub mod fixture_menu;
pub mod profile_builder;
pub mod quick;
pub mod setup;
pub mod windows;

use dmxcraft_core::{DmxFrame, Engine};
use eframe::egui::Ui;

use crate::state::AppState;

/// Alles, was eine Ansicht zum Zeichnen braucht.
pub struct ViewCtx<'a> {
    pub engine: &'a mut Engine,
    pub state: &'a mut AppState,
    /// Zuletzt ausgegebener DMX-Frame (für Live-Anzeigen).
    pub frame: &'a DmxFrame,
}

pub trait View {
    fn ui(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>);
}
