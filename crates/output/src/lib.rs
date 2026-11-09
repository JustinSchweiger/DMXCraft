//! Ausgabeschicht.
//!
//! Alles, was DMX-Daten irgendwohin schickt, implementiert [`DmxOutput`].
//! Der [`Runtime`]-Thread taktet die Engine und schickt jeden Frame an das
//! gewählte Backend: Simulation oder USB-DMX-Kabel ([`open_dmx`]).

pub mod backends;
pub mod open_dmx;
pub mod runtime;

use dmxcraft_core::DmxFrame;
use thiserror::Error;

pub use backends::{NullOutput, RecordingOutput};
pub use open_dmx::{list_ports, OpenDmx, UsbDmxPort};
pub use runtime::{OutputStatus, Runtime};

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Gerät nicht gefunden: {0}")]
    NotFound(String),
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
    /// Teilprobleme bei laufendem Ausgang (z. B. ein Kabel von zweien fehlt).
    fn health(&self) -> Option<String> {
        None
    }
}

/// Einstellungen für die Ausgänge.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutputConfig {
    /// USB-DMX: je Universum (Index) der Schlüssel des Kabels.
    pub open_dmx: Vec<Option<String>>,
}

/// Verfügbare Ausgänge für die Auswahl in der Oberfläche.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputKind {
    Simulation,
    OpenDmx,
}

impl OutputKind {
    /// Zur Auswahl angebotene Ausgänge.
    pub const ALL: [OutputKind; 2] = [OutputKind::Simulation, OutputKind::OpenDmx];

    pub fn label(self) -> &'static str {
        match self {
            OutputKind::Simulation => "Simulation (kein Gerät)",
            OutputKind::OpenDmx => "USB-DMX-Kabel (Open DMX / FTDI)",
        }
    }

    /// Schlüssel für die Einstellungen.
    pub fn key(self) -> &'static str {
        match self {
            OutputKind::Simulation => "simulation",
            OutputKind::OpenDmx => "open_dmx",
        }
    }

    pub fn from_key(key: &str) -> OutputKind {
        OutputKind::ALL
            .into_iter()
            .find(|k| k.key() == key)
            .unwrap_or(OutputKind::Simulation)
    }

    pub fn create(self, config: &OutputConfig) -> Box<dyn DmxOutput> {
        match self {
            OutputKind::Simulation => Box::new(NullOutput::default()),
            OutputKind::OpenDmx => Box::new(OpenDmx::new(config.open_dmx.clone())),
        }
    }
}
