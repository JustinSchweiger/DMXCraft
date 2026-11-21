//! Velleman VM116 / K8062 USB DMX interface.
//!
//! Unlike the Open DMX cables this interface has its own microcontroller and
//! shows up as a USB HID device (VID `0x10CF`, PID `0x8062`), so no driver is
//! needed on Windows, macOS or Linux. The PC sends the universe in 8-byte
//! reports; the interface generates the DMX signal itself.
//!
//! Report types (first byte), as used by the Open Lighting Architecture
//! (`plugins/usbdmx/VellemanK8062.cpp`) and Velleman's own driver:
//!
//! * `4` start of frame: `[4, n, 6 slots]` – `n` = number of leading zero
//!   slots *including the start code*, followed by the next 6 slots,
//! * `5` compressed:     `[5, n, 6 slots]` – `n` zero slots, then 6 slots,
//! * `2` data:           `[2, 7 slots]`,
//! * `3` single slot:    `[3, slot]` – used for the last few slots.
//!
//! Firmware upgrades with 64-byte reports exist; they also understand the
//! standard 8-byte reports used here.

use std::sync::Arc;
use std::time::{Duration, Instant};

use dmxcraft_core::UNIVERSE_SIZE;

use crate::devices::{DmxDevice, Driver, Shared};

/// USB vendor ID of Velleman.
pub const VM116_VENDOR_ID: u16 = 0x10CF;
/// USB product ID of the VM116 / K8062.
pub const VM116_PRODUCT_ID: u16 = 0x8062;
/// Prefix of the assignment key, separates the VM116 from serial cables.
pub const KEY_PREFIX: &str = "vm116:";

/// Size of one report (without the HID report ID).
pub const REPORT_SIZE: usize = 8;
/// Slots in a data report.
const DATA_SLOTS: usize = REPORT_SIZE - 1;
/// Slots after a zero count.
const COMPRESSED_SLOTS: usize = REPORT_SIZE - 2;
/// Longest run of zeros in one report (OLA: more upsets the firmware).
const MAX_ZEROS: usize = 100;

const MSG_DATA: u8 = 2;
const MSG_SINGLE: u8 = 3;
const MSG_START: u8 = 4;
const MSG_COMPRESSED: u8 = 5;

/// Shortest time per frame (~44 Hz), in case the driver accepts the reports
/// faster than the interface sends them.
const MIN_FRAME: Duration = Duration::from_micros(22_700);
/// Wait time between connection attempts.
const RETRY: Duration = Duration::from_secs(1);

/// Leading zero slots of `data` that can be skipped while still leaving
/// `COMPRESSED_SLOTS` slots for the report.
fn leading_zeros(data: &[u8]) -> usize {
    let mut n = 0;
    while n < MAX_ZEROS && n + COMPRESSED_SLOTS < data.len() && data[n] == 0 {
        n += 1;
    }
    n
}

/// Converts a universe into the reports of one frame.
pub fn encode_frame(channels: &[u8; UNIVERSE_SIZE]) -> Vec<[u8; REPORT_SIZE]> {
    let data = &channels[..];
    let size = data.len();
    let mut out = Vec::with_capacity(80);

    let zeros = leading_zeros(data);
    let mut r = [0u8; REPORT_SIZE];
    r[0] = MSG_START;
    r[1] = (zeros + 1) as u8; // + start code
    r[2..].copy_from_slice(&data[zeros..zeros + COMPRESSED_SLOTS]);
    out.push(r);
    let mut i = zeros + COMPRESSED_SLOTS;

    while i + DATA_SLOTS < size {
        let zeros = leading_zeros(&data[i..]);
        let mut r = [0u8; REPORT_SIZE];
        if zeros > 0 {
            r[0] = MSG_COMPRESSED;
            r[1] = zeros as u8;
            r[2..].copy_from_slice(&data[i + zeros..i + zeros + COMPRESSED_SLOTS]);
            i += zeros + COMPRESSED_SLOTS;
        } else {
            r[0] = MSG_DATA;
            r[1..].copy_from_slice(&data[i..i + DATA_SLOTS]);
            i += DATA_SLOTS;
        }
        out.push(r);
    }
    for &v in &data[i..] {
        let mut r = [0u8; REPORT_SIZE];
        r[0] = MSG_SINGLE;
        r[1] = v;
        out.push(r);
    }
    out
}

/// Assignment key of a device: serial number if it has one, otherwise the
/// system path.
fn key_of(dev: &hidapi::DeviceInfo) -> (String, bool) {
    match dev.serial_number().map(str::trim).filter(|s| !s.is_empty()) {
        Some(sn) => (format!("{KEY_PREFIX}{sn}"), true),
        None => (
            format!("{KEY_PREFIX}{}", dev.path().to_string_lossy()),
            false,
        ),
    }
}

fn devices(api: &hidapi::HidApi) -> impl Iterator<Item = &hidapi::DeviceInfo> {
    api.device_list()
        .filter(|d| d.vendor_id() == VM116_VENDOR_ID && d.product_id() == VM116_PRODUCT_ID)
}

/// Lists the connected VM116 / K8062 interfaces.
pub fn list() -> Vec<DmxDevice> {
    let Ok(api) = hidapi::HidApi::new() else {
        return Vec::new();
    };
    let mut seen = Vec::new();
    devices(&api)
        .filter_map(|d| {
            let path = d.path().to_string_lossy().into_owned();
            // Some systems list a device once per HID collection
            if seen.contains(&path) {
                return None;
            }
            seen.push(path.clone());
            let (key, has_serial) = key_of(d);
            let label = if has_serial {
                format!(
                    "Velleman VM116 / K8062 · {}",
                    key.trim_start_matches(KEY_PREFIX)
                )
            } else {
                "Velleman VM116 / K8062".to_string()
            };
            Some(DmxDevice {
                key,
                port_name: path,
                label,
                ftdi: false,
                driver: Driver::Vm116,
                recognized: true,
            })
        })
        .collect()
}

/// Opens the device for a key. Without a serial number the path can change
/// after replugging; then the only connected VM116 without a serial number
/// is taken.
fn open(api: &hidapi::HidApi, key: &str) -> Result<hidapi::HidDevice, String> {
    let all: Vec<&hidapi::DeviceInfo> = devices(api).collect();
    let dev = all
        .iter()
        .copied()
        .find(|d| key_of(d).0 == key)
        .or_else(|| {
            let no_serial: Vec<_> = all.iter().copied().filter(|d| !key_of(d).1).collect();
            (no_serial.len() == 1).then(|| no_serial[0])
        });
    let Some(dev) = dev else {
        return Err("not found".into());
    };
    dev.open_device(api).map_err(|e| e.to_string())
}

/// Send thread of one VM116 (one universe).
pub(crate) fn run_worker(key: String, universe: usize, shared: Arc<Shared>) {
    let name = key.trim_start_matches(KEY_PREFIX).to_string();
    let label = if name.len() > 24 { "VM116" } else { &name };
    let mut api: Option<hidapi::HidApi> = None;
    while !shared.stopped() {
        let api = match &mut api {
            Some(a) => {
                let _ = a.refresh_devices();
                a
            }
            None => match hidapi::HidApi::new() {
                Ok(a) => api.insert(a),
                Err(e) => {
                    shared.set_error(format!("Universe {}: USB HID: {e}", universe + 1));
                    std::thread::sleep(RETRY);
                    continue;
                }
            },
        };
        let device = match open(api, &key) {
            Ok(d) => d,
            Err(e) => {
                shared.set_error(format!("Universe {}: VM116 {label}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        shared.set_connected();
        let mut buf = [0u8; REPORT_SIZE + 1]; // report ID 0 + report
        'frames: while !shared.stopped() {
            let start = Instant::now();
            let channels = shared.channels();
            for report in encode_frame(&channels) {
                buf[1..].copy_from_slice(&report);
                if let Err(e) = device.write(&buf) {
                    shared.set_error(format!("Universe {}: VM116 {label}: {e}", universe + 1));
                    std::thread::sleep(RETRY);
                    break 'frames;
                }
            }
            let elapsed = start.elapsed();
            if elapsed < MIN_FRAME {
                std::thread::sleep(MIN_FRAME - elapsed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rebuilds the universe from the reports the way the interface reads
    /// them.
    fn decode(reports: &[[u8; REPORT_SIZE]]) -> Vec<u8> {
        let mut out = Vec::new();
        for (n, r) in reports.iter().enumerate() {
            match r[0] {
                MSG_START => {
                    assert_eq!(n, 0, "start only at the beginning");
                    assert!(r[1] >= 1);
                    out.extend(std::iter::repeat_n(0, usize::from(r[1]) - 1));
                    out.extend_from_slice(&r[2..]);
                }
                MSG_COMPRESSED => {
                    assert!(r[1] >= 1 && usize::from(r[1]) <= MAX_ZEROS);
                    out.extend(std::iter::repeat_n(0, usize::from(r[1])));
                    out.extend_from_slice(&r[2..]);
                }
                MSG_DATA => out.extend_from_slice(&r[1..]),
                MSG_SINGLE => out.push(r[1]),
                t => panic!("unknown report type {t}"),
            }
        }
        out
    }

    fn roundtrip(ch: [u8; UNIVERSE_SIZE]) -> usize {
        let reports = encode_frame(&ch);
        assert_eq!(decode(&reports), ch.to_vec());
        assert!(reports[1..].iter().all(|r| r[0] != MSG_START));
        reports.len()
    }

    #[test]
    fn all_zero_frame_is_compressed() {
        let n = roundtrip([0; UNIVERSE_SIZE]);
        assert!(n < 15, "{n} reports for an empty universe");
    }

    #[test]
    fn full_frame_uses_data_reports() {
        let n = roundtrip([255; UNIVERSE_SIZE]);
        // 6 in the start report, 72 × 7 in data reports, 2 single
        assert_eq!(n, 1 + 72 + 2);
    }

    #[test]
    fn mixed_frames_roundtrip() {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 1;
        roundtrip(ch);
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[511] = 9;
        roundtrip(ch);
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[300] = 200;
        ch[301] = 0;
        ch[302] = 7;
        roundtrip(ch);
        // pseudo-random pattern with runs of zeros of every length
        let mut x = 12345u32;
        for _ in 0..200 {
            let mut ch = [0u8; UNIVERSE_SIZE];
            for c in ch.iter_mut() {
                x = x.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                *c = if (x >> 16).is_multiple_of(3) {
                    (x >> 8) as u8
                } else {
                    0
                };
            }
            roundtrip(ch);
        }
    }

    #[test]
    fn start_report_counts_the_start_code() {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[3] = 50;
        let r = encode_frame(&ch);
        // 3 zero channels + start code, then channel 4
        assert_eq!(r[0][..3], [MSG_START, 4, 50]);
    }
}
