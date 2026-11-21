//! uDMX (Anyma and clones).
//!
//! A small open-hardware interface with an AVR running V-USB. It uses the
//! shared V-USB IDs (VID `0x16C0`, PID `0x05DC`) and is recognized by its
//! product string "uDMX". The universe is sent as a USB vendor request:
//! `bRequest` 2 ("set channel range"), `wValue` = number of channels,
//! `wIndex` = first channel (0-based), data = the channel values.
//!
//! The device talks plain USB (no HID, no serial port), accessed through
//! `nusb`. On Windows it needs the WinUSB driver (install it once with
//! Zadig); on Linux a udev rule for access; macOS needs nothing.

use std::sync::Arc;
use std::time::{Duration, Instant};

use dmxcraft_core::UNIVERSE_SIZE;
use nusb::transfer::{ControlOut, ControlType, Recipient};
use nusb::MaybeFuture;

use crate::devices::{DmxDevice, Driver, Shared};

/// Shared V-USB vendor ID (Van Ooijen Technische Informatica).
pub const UDMX_VENDOR_ID: u16 = 0x16C0;
/// Shared V-USB product ID ("libusb device"), told apart by the product string.
pub const UDMX_PRODUCT_ID: u16 = 0x05DC;
/// Prefix of the assignment key.
pub const KEY_PREFIX: &str = "udmx:";

/// "Set channel range".
const CMD_SET_CHANNEL_RANGE: u8 = 2;
const TIMEOUT: Duration = Duration::from_millis(500);
/// Time per frame (~40 Hz); the device repeats the last values by itself.
const FRAME: Duration = Duration::from_millis(25);
/// Wait time between connection attempts.
const RETRY: Duration = Duration::from_secs(1);

/// The vendor request that sends a whole universe.
pub fn request(channels: &[u8; UNIVERSE_SIZE]) -> ControlOut<'_> {
    ControlOut {
        control_type: ControlType::Vendor,
        recipient: Recipient::Device,
        request: CMD_SET_CHANNEL_RANGE,
        value: UNIVERSE_SIZE as u16,
        index: 0,
        data: channels,
    }
}

fn is_udmx(d: &nusb::DeviceInfo) -> bool {
    d.vendor_id() == UDMX_VENDOR_ID
        && d.product_id() == UDMX_PRODUCT_ID
        && d.product_string()
            .is_some_and(|p| p.to_ascii_lowercase().contains("udmx"))
}

/// Assignment key: serial number if the device has one, otherwise the USB
/// port it is plugged into.
fn key_of(d: &nusb::DeviceInfo) -> (String, bool) {
    match d.serial_number().map(str::trim).filter(|s| !s.is_empty()) {
        Some(sn) => (format!("{KEY_PREFIX}{sn}"), true),
        None => {
            let chain: Vec<String> = d.port_chain().iter().map(u8::to_string).collect();
            (
                format!("{KEY_PREFIX}{}-{}", d.bus_id(), chain.join(".")),
                false,
            )
        }
    }
}

fn devices() -> Vec<nusb::DeviceInfo> {
    nusb::list_devices()
        .wait()
        .map(|it| it.filter(is_udmx).collect())
        .unwrap_or_default()
}

/// Lists the connected uDMX interfaces.
pub fn list() -> Vec<DmxDevice> {
    devices()
        .iter()
        .map(|d| {
            let (key, _) = key_of(d);
            let id = key.trim_start_matches(KEY_PREFIX).to_string();
            DmxDevice {
                label: format!("uDMX · {id}"),
                port_name: id,
                key,
                ftdi: false,
                driver: Driver::Udmx,
                recognized: true,
            }
        })
        .collect()
}

/// Finds the device for a key. Without a serial number the key is the USB
/// port; if the device moved to another port and it is the only uDMX without
/// a serial number, it is taken.
fn find(key: &str) -> Option<nusb::DeviceInfo> {
    let all = devices();
    if let Some(d) = all.iter().find(|d| key_of(d).0 == key) {
        return Some(d.clone());
    }
    let no_serial: Vec<_> = all.into_iter().filter(|d| !key_of(d).1).collect();
    (no_serial.len() == 1).then(|| no_serial.into_iter().next())?
}

fn open(info: &nusb::DeviceInfo) -> Result<nusb::Interface, String> {
    let hint = if cfg!(windows) {
        " (install the WinUSB driver with Zadig)"
    } else {
        ""
    };
    let device = info.open().wait().map_err(|e| format!("{e}{hint}"))?;
    device
        .claim_interface(0)
        .wait()
        .map_err(|e| format!("{e}{hint}"))
}

/// Send thread of one uDMX (one universe).
pub(crate) fn run_worker(key: String, universe: usize, shared: Arc<Shared>) {
    let id = key.trim_start_matches(KEY_PREFIX).to_string();
    while !shared.stopped() {
        let Some(info) = find(&key) else {
            shared.set_error(format!("Universe {}: uDMX {id} not found", universe + 1));
            std::thread::sleep(RETRY);
            continue;
        };
        let iface = match open(&info) {
            Ok(i) => i,
            Err(e) => {
                shared.set_error(format!("Universe {}: uDMX {id}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        shared.set_connected();
        while !shared.stopped() {
            let start = Instant::now();
            let channels = shared.channels();
            if let Err(e) = iface.control_out(request(&channels), TIMEOUT).wait() {
                shared.set_error(format!("Universe {}: uDMX {id}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                break;
            }
            let elapsed = start.elapsed();
            if elapsed < FRAME {
                std::thread::sleep(FRAME - elapsed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_sends_the_whole_universe() {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 1;
        ch[511] = 2;
        let r = request(&ch);
        assert_eq!(r.control_type, ControlType::Vendor);
        assert_eq!(r.recipient, Recipient::Device);
        assert_eq!((r.request, r.value, r.index), (2, 512, 0));
        assert_eq!(r.data.len(), 512);
        assert_eq!((r.data[0], r.data[511]), (1, 2));
    }
}
