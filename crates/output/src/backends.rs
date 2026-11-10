use std::sync::{Arc, Mutex};

use dmxcraft_core::DmxFrame;

use crate::{DmxOutput, OutputError};

/// Discards all frames. For working without a device; the UI still shows
/// the values in the DMX monitor.
#[derive(Default)]
pub struct NullOutput {
    pub frames_sent: u64,
}

impl DmxOutput for NullOutput {
    fn name(&self) -> &str {
        "Simulation"
    }
    fn open(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
    fn send(&mut self, _frame: &DmxFrame) -> Result<(), OutputError> {
        self.frames_sent += 1;
        Ok(())
    }
}

/// Stores sent frames – for tests and debugging.
#[derive(Clone, Default)]
pub struct RecordingOutput {
    pub frames: Arc<Mutex<Vec<DmxFrame>>>,
}

impl DmxOutput for RecordingOutput {
    fn name(&self) -> &str {
        "Aufzeichnung"
    }
    fn open(&mut self) -> Result<(), OutputError> {
        Ok(())
    }
    fn send(&mut self, frame: &DmxFrame) -> Result<(), OutputError> {
        self.frames
            .lock()
            .map_err(|_| OutputError::Transfer("Mutex vergiftet".into()))?
            .push(frame.clone());
        Ok(())
    }
}
