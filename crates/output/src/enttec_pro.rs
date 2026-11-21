//! Enttec DMX USB Pro and compatible widgets (DMXking ultraDMX Micro /
//! ultraDMX Pro and many clones).
//!
//! The widget has its own processor: the PC sends the universe as one
//! message over the (FTDI) serial port and the widget generates the DMX
//! signal and repeats the last frame by itself.
//!
//! Message format (Enttec DMX USB Pro API):
//! `0x7E, label, length LSB, length MSB, data …, 0xE7`. Label 6 ("Output Only
//! Send DMX Packet Request") carries the start code and up to 512 channels.

use std::io::Write;
use std::sync::Arc;
use std::time::{Duration, Instant};

use dmxcraft_core::UNIVERSE_SIZE;

use crate::devices::Shared;
use crate::open_dmx::{open_port, resolve};

/// Prefix of the assignment key, separates the Pro from Open DMX cables.
pub const KEY_PREFIX: &str = "pro:";

const START: u8 = 0x7E;
const END: u8 = 0xE7;
/// "Output Only Send DMX Packet Request".
const LABEL_SEND_DMX: u8 = 6;
/// Length of a message with a full universe.
pub const MESSAGE_LEN: usize = 4 + 1 + UNIVERSE_SIZE + 1;

/// Time per frame (~40 Hz); the widget repeats the last frame by itself.
const FRAME: Duration = Duration::from_millis(25);
/// Wait time between connection attempts.
const RETRY: Duration = Duration::from_secs(1);

/// Product or manufacturer string of a Pro-compatible widget.
pub fn looks_like_pro(product: &str, manufacturer: &str) -> bool {
    let p = product.to_ascii_uppercase();
    let m = manufacturer.to_ascii_uppercase();
    (p.contains("DMX") && p.contains("PRO")) || p.contains("ULTRADMX") || m.contains("DMXKING")
}

/// Message with start code and the 512 channels of a universe.
pub fn message(channels: &[u8; UNIVERSE_SIZE]) -> [u8; MESSAGE_LEN] {
    let len = (UNIVERSE_SIZE + 1) as u16;
    let mut m = [0u8; MESSAGE_LEN];
    m[0] = START;
    m[1] = LABEL_SEND_DMX;
    m[2..4].copy_from_slice(&len.to_le_bytes());
    m[4] = 0; // start code
    m[5..5 + UNIVERSE_SIZE].copy_from_slice(channels);
    m[MESSAGE_LEN - 1] = END;
    m
}

/// Send thread of one widget (one universe).
pub(crate) fn run_worker(key: String, universe: usize, shared: Arc<Shared>) {
    let id = key.trim_start_matches(KEY_PREFIX).to_string();
    while !shared.stopped() {
        let Some(port_name) = resolve(&id) else {
            shared.set_error(format!(
                "Universe {}: DMX USB Pro {id} not found",
                universe + 1
            ));
            std::thread::sleep(RETRY);
            continue;
        };
        let mut port = match open_port(&port_name) {
            Ok(p) => p,
            Err(e) => {
                shared.set_error(format!("Universe {}: {port_name}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        shared.set_connected();
        while !shared.stopped() {
            let start = Instant::now();
            let msg = message(&shared.channels());
            if let Err(e) = port.write_all(&msg).and_then(|()| port.flush()) {
                shared.set_error(format!("Universe {}: {port_name}: {e}", universe + 1));
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
    fn message_layout() {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 255;
        ch[511] = 7;
        let m = message(&ch);
        assert_eq!(m.len(), 518);
        // start delimiter, label 6, length 513 (0x0201) LSB first, start code
        assert_eq!(m[..5], [0x7E, 6, 0x01, 0x02, 0]);
        assert_eq!(m[5], 255, "channel 1");
        assert_eq!(m[516], 7, "channel 512");
        assert_eq!(m[517], 0xE7);
    }

    #[test]
    fn recognizes_pro_widgets() {
        assert!(looks_like_pro("DMX USB PRO", "ENTTEC"));
        assert!(looks_like_pro("ultraDMX Micro", ""));
        assert!(looks_like_pro("", "DMXking.com"));
        assert!(!looks_like_pro("FT232R USB UART", "FTDI"));
        assert!(!looks_like_pro("Open DMX USB", "ENTTEC"));
    }
}
