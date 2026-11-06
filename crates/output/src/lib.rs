//! Ausgabeschicht.
//!
//! Alles, was DMX-Daten irgendwohin schickt, implementiert [`DmxOutput`].
//! Der [`Runtime`]-Thread taktet die Engine und schickt jeden Frame an das
//! gewählte Backend. Die USB-Anbindung der DVC Gold kommt später als
//! weiteres Backend in [`dvc_gold`] – an Engine und Oberfläche ändert sich
//! dafür nichts.

pub mod backends;
pub mod dvc_gold;
pub mod runtime;

use dmxcraft_core::DmxFrame;
use thiserror::Error;

pub use backends::{NullOutput, RecordingOutput};
pub use dvc_gold::DvcGoldUsb;
pub use runtime::{OutputStatus, Runtime};

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Gerät nicht gefunden: {0}")]
    NotFound(String),
    #[error("Noch nicht implementiert: {0}")]
    NotImplemented(&'static str),
    #[error("Übertragungsfehler: {0}")]
    Transfer(String),
}

/// Ein DMX-Ausgang.
pub trait DmxOutput: Send {
    /// Anzeigename für die Oberfläche.
    fn name(&self) -> &str;
    /// Verbindung aufbauen (inkl. eventuellem Handshake).
    fn open(&mut self) -> Result<(), OutputError>;
    /// Einen vollständigen Frame senden.
    fn send(&mut self, frame: &DmxFrame) -> Result<(), OutputError>;
    /// Verbindung schließen. Darf mehrfach aufgerufen werden.
    fn close(&mut self) {}
    /// Wie viele Universen der Ausgang bedienen kann.
    fn max_universes(&self) -> usize {
        usize::MAX
    }
}

/// Verfügbare Ausgänge für die Auswahl in der Oberfläche.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputKind {
    Simulation,
    DvcGoldUsb,
}

impl OutputKind {
    pub const ALL: [OutputKind; 2] = [OutputKind::Simulation, OutputKind::DvcGoldUsb];

    pub fn label(self) -> &'static str {
        match self {
            OutputKind::Simulation => "Simulation (kein Gerät)",
            OutputKind::DvcGoldUsb => "Daslight DVC Gold (USB)",
        }
    }

    pub fn create(self) -> Box<dyn DmxOutput> {
        match self {
            OutputKind::Simulation => Box::new(NullOutput::default()),
            OutputKind::DvcGoldUsb => Box::new(DvcGoldUsb::new()),
        }
    }
}
