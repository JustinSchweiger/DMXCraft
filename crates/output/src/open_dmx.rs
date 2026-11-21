//! USB DMX via FTDI cables in “Open DMX” style (e.g. Enttec Open DMX USB,
//! DSD TECH SH-RS09B and many clones with FT232R).
//!
//! These cables have no processor of their own: the PC generates the DMX
//! signal itself through the serial interface of the FTDI chip.
//!
//! A DMX frame consists of
//! 1. break (line ≥ 88 µs at 0),
//! 2. mark after break (≥ 8 µs at 1),
//! 3. start code `0x00` and up to 512 channel bytes, each 250 kBaud, 8N2.
//!
//! Every cable serves exactly one universe and runs in its own thread (a
//! frame takes ~23 ms – two cables one after the other could not reach
//! 40 Hz). The thread reconnects by itself when the cable is unplugged and
//! plugged in again. The cable → universe assignment uses the serial number
//! of the FTDI chip so it stays the same after a restart or replugging.
//!
//! The threads are run by [`crate::DeviceOutput`].

use std::io::{self, Write};
use std::sync::Arc;
use std::time::{Duration, Instant};

use dmxcraft_core::UNIVERSE_SIZE;

use crate::devices::{DmxDevice, Driver, Shared};

/// Vendor ID of FTDI.
pub const FTDI_VENDOR_ID: u16 = 0x0403;
/// DMX baud rate.
pub const DMX_BAUD: u32 = 250_000;
/// Break duration (standard: ≥ 88 µs; generous because sleeping takes longer
/// on some systems anyway).
const BREAK: Duration = Duration::from_micros(150);
/// Mark after break (standard: ≥ 8 µs).
const MAB: Duration = Duration::from_micros(20);
/// Transfer time of a full frame: 513 bytes × 11 bits / 250 kBaud.
const FRAME_TIME: Duration = Duration::from_micros(513 * 11 * 4);
/// Wait time between connection attempts.
const RETRY: Duration = Duration::from_secs(1);

/// Lists the connected USB serial devices, each once per serial driver
/// (Open DMX and DMX USB Pro use the same FTDI chip; the product string only
/// sometimes tells them apart).
pub(crate) fn list_serial_devices() -> Vec<DmxDevice> {
    let Ok(ports) = serialport::available_ports() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for p in ports
        .into_iter()
        // macOS reports every device as tty.* and cu.* – only cu.* for sending
        .filter(|p| !cfg!(target_os = "macos") || p.port_name.starts_with("/dev/cu."))
    {
        let serialport::SerialPortType::UsbPort(usb) = p.port_type else {
            continue;
        };
        let ftdi = usb.vid == FTDI_VENDOR_ID;
        let serial = usb.serial_number.filter(|s| !s.trim().is_empty());
        let product = usb.product.filter(|s| !s.trim().is_empty());
        let pro = crate::enttec_pro::looks_like_pro(
            product.as_deref().unwrap_or_default(),
            usb.manufacturer.as_deref().unwrap_or_default(),
        );
        let product = product.unwrap_or_else(|| "USB serial".into());
        let label = match &serial {
            Some(sn) => format!("{product} · {sn} ({})", p.port_name),
            None => format!("{product} ({})", p.port_name),
        };
        let key = serial.unwrap_or_else(|| p.port_name.clone());
        out.push(DmxDevice {
            key: key.clone(),
            port_name: p.port_name.clone(),
            label: format!("{label} · Open DMX"),
            ftdi,
            driver: Driver::OpenDmx,
            recognized: ftdi && !pro,
        });
        out.push(DmxDevice {
            key: format!("{}{key}", crate::enttec_pro::KEY_PREFIX),
            port_name: p.port_name,
            label: format!("{label} · DMX USB Pro"),
            ftdi,
            driver: Driver::EnttecPro,
            recognized: pro,
        });
    }
    out
}

/// Finds the current port name for a stored key (serial number or port name,
/// without driver prefix).
pub(crate) fn resolve(key: &str) -> Option<String> {
    list_serial_devices()
        .into_iter()
        .find(|p| p.driver == Driver::OpenDmx && (p.key == key || p.port_name == key))
        .map(|p| p.port_name)
}

/// Line a DMX frame is written to (replaceable for tests).
pub trait DmxLine {
    fn set_break(&mut self) -> io::Result<()>;
    fn clear_break(&mut self) -> io::Result<()>;
    fn write_frame(&mut self, data: &[u8]) -> io::Result<()>;
}

impl DmxLine for Box<dyn serialport::SerialPort> {
    fn set_break(&mut self) -> io::Result<()> {
        serialport::SerialPort::set_break(self.as_ref()).map_err(io::Error::from)
    }
    fn clear_break(&mut self) -> io::Result<()> {
        serialport::SerialPort::clear_break(self.as_ref()).map_err(io::Error::from)
    }
    fn write_frame(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_all(data)?;
        self.flush()
    }
}

/// Start code + channel data of a universe.
pub fn packet(channels: &[u8; UNIVERSE_SIZE]) -> [u8; UNIVERSE_SIZE + 1] {
    let mut buf = [0u8; UNIVERSE_SIZE + 1];
    buf[1..].copy_from_slice(channels);
    buf
}

/// Writes a complete DMX frame (break, MAB, start code, data).
pub fn send_frame(line: &mut dyn DmxLine, channels: &[u8; UNIVERSE_SIZE]) -> io::Result<()> {
    line.set_break()?;
    std::thread::sleep(BREAK);
    line.clear_break()?;
    std::thread::sleep(MAB);
    let start = Instant::now();
    line.write_frame(&packet(channels))?;
    // Some drivers report “written” before the last byte is on the line. The
    // next break would cut it off – so wait until the frame has surely been
    // transmitted.
    let elapsed = start.elapsed();
    if elapsed < FRAME_TIME {
        std::thread::sleep(FRAME_TIME - elapsed);
    }
    Ok(())
}

/// Opens a serial port at 250 kBaud, 8N2 (DMX line settings; also used for
/// the DMX USB Pro, where the setting does not matter or matches).
pub(crate) fn open_port(port_name: &str) -> serialport::Result<Box<dyn serialport::SerialPort>> {
    serialport::new(port_name, DMX_BAUD)
        .data_bits(serialport::DataBits::Eight)
        .stop_bits(serialport::StopBits::Two)
        .parity(serialport::Parity::None)
        .flow_control(serialport::FlowControl::None)
        .timeout(Duration::from_millis(200))
        .open()
}

/// Send thread of an Open DMX cable.
pub(crate) fn run_worker(key: String, universe: usize, shared: Arc<Shared>) {
    let set_error = |msg: String| shared.set_error(msg);
    while !shared.stopped() {
        let Some(port_name) = resolve(&key) else {
            set_error(format!("Universe {}: cable {key} not found", universe + 1));
            std::thread::sleep(RETRY);
            continue;
        };
        let mut port = match open_port(&port_name) {
            Ok(p) => p,
            Err(e) => {
                set_error(format!("Universe {}: {port_name}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        shared.set_connected();
        while !shared.stopped() {
            let channels = shared.channels();
            if let Err(e) = send_frame(&mut port, &channels) {
                set_error(format!("Universe {}: {port_name}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Recorder {
        events: Vec<String>,
        written: Vec<u8>,
    }

    impl DmxLine for Recorder {
        fn set_break(&mut self) -> io::Result<()> {
            self.events.push("break".into());
            Ok(())
        }
        fn clear_break(&mut self) -> io::Result<()> {
            self.events.push("mark".into());
            Ok(())
        }
        fn write_frame(&mut self, data: &[u8]) -> io::Result<()> {
            self.events.push(format!("data {}", data.len()));
            self.written.extend_from_slice(data);
            Ok(())
        }
    }

    #[test]
    fn frame_is_break_mark_startcode_and_512_channels() {
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 255;
        ch[511] = 7;
        let mut r = Recorder::default();
        let t = Instant::now();
        send_frame(&mut r, &ch).unwrap();
        assert_eq!(r.events, ["break", "mark", "data 513"]);
        assert_eq!(r.written[0], 0, "Startcode");
        assert_eq!(r.written[1], 255, "Channel 1");
        assert_eq!(r.written[512], 7, "Channel 512");
        // The frame waits at least for the transfer time
        assert!(t.elapsed() >= FRAME_TIME);
    }

    /// End to end over a real terminal pair: configuration as in operation,
    /// the bytes arrive completely on the other side.
    #[cfg(target_os = "linux")]
    #[test]
    fn frame_over_pseudo_terminal() {
        use serialport::SerialPort;
        use std::io::Read;

        let (master, mut slave) = serialport::TTYPort::pair().expect("pty");
        let mut port: Box<dyn SerialPort> = Box::new(master);
        port.set_baud_rate(DMX_BAUD).unwrap();
        port.set_stop_bits(serialport::StopBits::Two).unwrap();
        let mut ch = [0u8; UNIVERSE_SIZE];
        ch[0] = 255;
        ch[1] = 128;
        // Break is not always available on pseudo terminals – then only check the
        // data
        let ok = match send_frame(&mut port, &ch) {
            Ok(()) => true,
            Err(_) => {
                port.write_frame(&packet(&ch)).unwrap();
                false
            }
        };
        slave.set_timeout(Duration::from_millis(500)).unwrap();
        let mut got = Vec::new();
        let mut buf = [0u8; 1024];
        while got.len() < UNIVERSE_SIZE + 1 {
            match slave.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => got.extend_from_slice(&buf[..n]),
            }
        }
        // A break can arrive as a 0 byte on the other side
        let frame = &got[got.len() - (UNIVERSE_SIZE + 1)..];
        assert_eq!(frame[0], 0, "Start code (break checked: {ok})");
        assert_eq!(frame[1], 255);
        assert_eq!(frame[2], 128);
    }
}
