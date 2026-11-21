//! Output layer.
//!
//! Everything that sends DMX data somewhere implements [`DmxOutput`].
//! The [`Runtime`] thread clocks the engine and sends every frame to the
//! chosen backend: simulation or DMX interfaces ([`DeviceOutput`], one per
//! universe): [`open_dmx`] for Open DMX / FTDI cables, [`enttec_pro`] for
//! the Enttec DMX USB Pro and compatibles, [`vm116`] for the Velleman VM116 /
//! K8062, [`udmx`] for the uDMX, [`network`] for Art-Net and sACN.

pub mod backends;
pub mod devices;
pub mod enttec_pro;
pub mod network;
pub mod open_dmx;
pub mod runtime;
pub mod udmx;
pub mod vm116;

use dmxcraft_core::DmxFrame;
use thiserror::Error;

pub use backends::{NullOutput, RecordingOutput};
pub use devices::{list_devices, DeviceOutput, DmxDevice, Driver, NetworkConfig};
pub use runtime::{OutputStatus, Runtime};

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("Device not found: {0}")]
    NotFound(String),
    #[error("Transfer error: {0}")]
    Transfer(String),
}

/// A DMX output.
pub trait DmxOutput: Send {
    /// Display name for the UI.
    fn name(&self) -> &str;
    /// Connect (including a handshake, if any).
    fn open(&mut self) -> Result<(), OutputError>;
    /// Send a complete frame.
    fn send(&mut self, frame: &DmxFrame) -> Result<(), OutputError>;
    /// Close the connection. May be called several times.
    fn close(&mut self) {}
    /// How many universes the output can serve.
    fn max_universes(&self) -> usize {
        usize::MAX
    }
    /// Partial problems while the output is running (e.g. one of two cables is
    /// missing).
    fn health(&self) -> Option<String> {
        None
    }
}

/// Settings for the outputs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutputConfig {
    /// Per universe (index) the key of the DMX interface (see
    /// [`devices`]).
    pub open_dmx: Vec<Option<String>>,
    /// Settings of Art-Net and sACN.
    pub network: NetworkConfig,
}

/// Available outputs for the selection in the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputKind {
    Simulation,
    /// DMX interfaces (USB and network), one per universe.
    Devices,
}

impl OutputKind {
    /// Outputs offered for selection.
    pub const ALL: [OutputKind; 2] = [OutputKind::Simulation, OutputKind::Devices];

    pub fn label(self) -> &'static str {
        match self {
            OutputKind::Simulation => "Simulation (no device)",
            OutputKind::Devices => "DMX interfaces",
        }
    }

    /// Key for the settings.
    pub fn key(self) -> &'static str {
        match self {
            OutputKind::Simulation => "simulation",
            OutputKind::Devices => "open_dmx",
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
            OutputKind::Devices => Box::new(DeviceOutput::new(
                config.open_dmx.clone(),
                config.network.clone(),
            )),
        }
    }
}
