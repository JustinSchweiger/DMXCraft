//! DMX interfaces, one per universe.
//!
//! [`DeviceOutput`] runs one send thread per assigned universe. Which driver
//! a thread uses comes from the stored assignment key (see [`Driver`]):
//!
//! | key                 | driver                                   |
//! |---------------------|------------------------------------------|
//! | `<serial>` / port   | Open DMX / FTDI cable ([`crate::open_dmx`]) |
//! | `pro:<serial>`      | Enttec DMX USB Pro and compatibles ([`crate::enttec_pro`]) |
//! | `vm116:<serial>`    | Velleman VM116 / K8062 ([`crate::vm116`]) |
//! | `udmx:<serial>`     | uDMX ([`crate::udmx`])                   |
//! | `artnet`            | Art-Net ([`crate::network`])             |
//! | `sacn`              | sACN / E1.31 ([`crate::network`])        |
//!
//! Every thread reconnects by itself when its device is unplugged and
//! plugged in again, and reports problems through [`DmxOutput::health`].

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use dmxcraft_core::{DmxFrame, UNIVERSE_SIZE};

use crate::{DmxOutput, OutputError};

/// A DMX interface that can be assigned to a universe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DmxDevice {
    /// Stable key for the assignment (see the module docs).
    pub key: String,
    /// System name of the device (`COM3`, `/dev/ttyUSB0`, HID path, …).
    pub port_name: String,
    /// Display in the UI.
    pub label: String,
    /// USB serial device with an FTDI chip.
    pub ftdi: bool,
    /// How DMX is sent to the device.
    pub driver: Driver,
    /// Recognized as a DMX interface of this kind (counted as “found” and
    /// listed first). A serial device is listed once per serial driver, only
    /// one of them is recognized.
    pub recognized: bool,
}

/// Kind of DMX interface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Driver {
    /// Serial cable without processor (Open DMX / FTDI).
    OpenDmx,
    /// Enttec DMX USB Pro protocol (also DMXking ultraDMX and many clones).
    EnttecPro,
    /// Velleman VM116 / K8062 (USB HID).
    Vm116,
    /// uDMX (Anyma and clones, USB vendor requests).
    Udmx,
    /// Art-Net over the network.
    ArtNet,
    /// sACN / E1.31 over the network.
    Sacn,
}

/// Key of the Art-Net output.
pub const ARTNET_KEY: &str = "artnet";
/// Key of the sACN output.
pub const SACN_KEY: &str = "sacn";

impl Driver {
    /// Driver for a stored assignment key.
    pub fn for_key(key: &str) -> Driver {
        if key == ARTNET_KEY {
            Driver::ArtNet
        } else if key == SACN_KEY {
            Driver::Sacn
        } else if key.starts_with(crate::vm116::KEY_PREFIX) {
            Driver::Vm116
        } else if key.starts_with(crate::udmx::KEY_PREFIX) {
            Driver::Udmx
        } else if key.starts_with(crate::enttec_pro::KEY_PREFIX) {
            Driver::EnttecPro
        } else {
            Driver::OpenDmx
        }
    }

    /// Network output: may serve several universes at once.
    pub fn is_network(self) -> bool {
        matches!(self, Driver::ArtNet | Driver::Sacn)
    }

    /// Talks to a USB serial port.
    pub fn is_serial(self) -> bool {
        matches!(self, Driver::OpenDmx | Driver::EnttecPro)
    }
}

/// Lists the available DMX interfaces: USB devices (recognized ones first)
/// followed by the network outputs.
pub fn list_devices() -> Vec<DmxDevice> {
    let mut out = crate::open_dmx::list_serial_devices();
    out.extend(crate::vm116::list());
    out.extend(crate::udmx::list());
    out.sort_by(|a, b| {
        b.recognized
            .cmp(&a.recognized)
            .then(a.label.cmp(&b.label))
            .then(a.port_name.cmp(&b.port_name))
    });
    out.extend(crate::network::list());
    out
}

/// Shared state between the output and the send thread of a universe.
pub(crate) struct Shared {
    pub(crate) data: Mutex<[u8; UNIVERSE_SIZE]>,
    pub(crate) stop: AtomicBool,
    pub(crate) connected: AtomicBool,
    error: Mutex<Option<String>>,
}

impl Shared {
    pub(crate) fn new() -> Self {
        Self {
            data: Mutex::new([0; UNIVERSE_SIZE]),
            stop: AtomicBool::new(false),
            connected: AtomicBool::new(false),
            error: Mutex::new(None),
        }
    }

    pub(crate) fn stopped(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    /// Current channel values of the universe.
    pub(crate) fn channels(&self) -> [u8; UNIVERSE_SIZE] {
        self.data.lock().map(|d| *d).unwrap_or([0; UNIVERSE_SIZE])
    }

    /// Not connected, with the reason.
    pub(crate) fn set_error(&self, msg: String) {
        self.connected.store(false, Ordering::Relaxed);
        if let Ok(mut e) = self.error.lock() {
            *e = Some(msg);
        }
    }

    pub(crate) fn set_connected(&self) {
        self.connected.store(true, Ordering::Relaxed);
        if let Ok(mut e) = self.error.lock() {
            *e = None;
        }
    }
}

/// Settings of the network outputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NetworkConfig {
    /// Destination of Art-Net packets (broadcast or the address of a node).
    pub artnet_target: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            artnet_target: crate::network::ARTNET_BROADCAST.into(),
        }
    }
}

struct Worker {
    universe: usize,
    shared: Arc<Shared>,
    handle: Option<JoinHandle<()>>,
}

/// Output via DMX interfaces, one per universe.
pub struct DeviceOutput {
    /// Per universe (index) the key of the assigned interface.
    assign: Vec<Option<String>>,
    network: NetworkConfig,
    workers: Vec<Worker>,
}

impl DeviceOutput {
    pub fn new(assign: Vec<Option<String>>, network: NetworkConfig) -> Self {
        Self {
            assign,
            network,
            workers: Vec::new(),
        }
    }

    fn stop_workers(&mut self) {
        for w in &self.workers {
            w.shared.stop.store(true, Ordering::Relaxed);
        }
        for w in &mut self.workers {
            if let Some(h) = w.handle.take() {
                let _ = h.join();
            }
        }
        self.workers.clear();
    }
}

impl Drop for DeviceOutput {
    fn drop(&mut self) {
        self.stop_workers();
    }
}

fn run_worker(key: String, universe: usize, network: NetworkConfig, shared: Arc<Shared>) {
    let s = shared.clone();
    match Driver::for_key(&key) {
        Driver::OpenDmx => crate::open_dmx::run_worker(key, universe, s),
        Driver::EnttecPro => crate::enttec_pro::run_worker(key, universe, s),
        Driver::Vm116 => crate::vm116::run_worker(key, universe, s),
        Driver::Udmx => crate::udmx::run_worker(key, universe, s),
        Driver::ArtNet => crate::network::run_artnet(network.artnet_target, universe, s),
        Driver::Sacn => crate::network::run_sacn(universe, s),
    }
    shared.connected.store(false, Ordering::Relaxed);
}

impl DmxOutput for DeviceOutput {
    fn name(&self) -> &str {
        "DMX interfaces"
    }

    fn open(&mut self) -> Result<(), OutputError> {
        self.stop_workers();
        let assigned: Vec<(usize, String)> = self
            .assign
            .iter()
            .enumerate()
            .filter_map(|(u, k)| k.clone().map(|k| (u, k)))
            .collect();
        if assigned.is_empty() {
            return Err(OutputError::NotFound(
                "no DMX interface is assigned to any universe (Settings → Output)".into(),
            ));
        }
        for (universe, key) in assigned {
            let shared = Arc::new(Shared::new());
            let s = shared.clone();
            let network = self.network.clone();
            let handle = std::thread::Builder::new()
                .name(format!("dmx-out-u{}", universe + 1))
                .spawn(move || run_worker(key, universe, network, s))
                .map_err(|e| OutputError::Transfer(e.to_string()))?;
            self.workers.push(Worker {
                universe,
                shared,
                handle: Some(handle),
            });
        }
        Ok(())
    }

    fn send(&mut self, frame: &DmxFrame) -> Result<(), OutputError> {
        for w in &self.workers {
            if let (Some(src), Ok(mut dst)) = (frame.universe(w.universe), w.shared.data.lock()) {
                dst.copy_from_slice(src);
            }
        }
        Ok(())
    }

    fn close(&mut self) {
        self.stop_workers();
    }

    fn max_universes(&self) -> usize {
        self.assign.len()
    }

    fn health(&self) -> Option<String> {
        let problems: Vec<String> = self
            .workers
            .iter()
            .filter(|w| !w.shared.connected.load(Ordering::Relaxed))
            .map(|w| {
                w.shared
                    .error
                    .lock()
                    .ok()
                    .and_then(|e| e.clone())
                    .unwrap_or_else(|| format!("Universe {}: connecting …", w.universe + 1))
            })
            .collect();
        (!problems.is_empty()).then(|| problems.join(" · "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    /// Waits until the health report contains all `needles`.
    fn health_with(o: &DeviceOutput, needles: &[&str]) -> String {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let health = o.health().unwrap_or_default();
            if needles.iter().all(|n| health.contains(n)) || Instant::now() > deadline {
                return health;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    #[test]
    fn driver_follows_the_key() {
        assert_eq!(Driver::for_key("A6XYZ123"), Driver::OpenDmx);
        assert_eq!(Driver::for_key("/dev/ttyUSB0"), Driver::OpenDmx);
        assert_eq!(Driver::for_key("pro:EN123"), Driver::EnttecPro);
        assert_eq!(Driver::for_key("vm116:1234"), Driver::Vm116);
        assert_eq!(Driver::for_key("udmx:1-2.3"), Driver::Udmx);
        assert_eq!(Driver::for_key("artnet"), Driver::ArtNet);
        assert_eq!(Driver::for_key("sacn"), Driver::Sacn);
        assert!(Driver::ArtNet.is_network() && !Driver::Udmx.is_network());
    }

    #[test]
    fn network_outputs_are_always_listed() {
        let list = list_devices();
        assert!(list.iter().any(|d| d.key == ARTNET_KEY));
        assert!(list.iter().any(|d| d.key == SACN_KEY));
    }

    #[test]
    fn open_without_assignment_fails_clearly() {
        let mut o = DeviceOutput::new(vec![None, None], NetworkConfig::default());
        assert!(matches!(o.open(), Err(OutputError::NotFound(_))));
    }

    #[test]
    fn missing_devices_are_reported_not_fatal() {
        let mut o = DeviceOutput::new(
            vec![
                Some("DOES-NOT-EXIST".into()),
                Some("vm116:DOES-NOT-EXIST".into()),
                Some("pro:DOES-NOT-EXIST".into()),
                Some("udmx:DOES-NOT-EXIST".into()),
            ],
            NetworkConfig::default(),
        );
        o.open().unwrap();
        // Sending continues, the threads report the missing devices
        o.send(&DmxFrame::new(4)).unwrap();
        let needles = [
            "Universe 1: cable DOES-NOT-EXIST not found",
            "Universe 2: VM116",
            "Universe 3: DMX USB Pro DOES-NOT-EXIST not found",
            "Universe 4: uDMX",
        ];
        let health = health_with(&o, &needles);
        for needle in needles {
            assert!(health.contains(needle), "{needle} missing in {health}");
        }
        o.close();
    }
}
