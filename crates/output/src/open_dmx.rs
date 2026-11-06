//! USB-DMX über FTDI-Kabel im „Open DMX“-Stil (z. B. Enttec Open DMX USB,
//! DSD TECH SH-RS09B und viele Nachbauten mit FT232R).
//!
//! Diese Kabel haben keinen eigenen Prozessor: Der PC erzeugt das DMX-Signal
//! selbst über die serielle Schnittstelle des FTDI-Chips.
//!
//! Ein DMX-Frame besteht aus
//! 1. Break (Leitung ≥ 88 µs auf 0),
//! 2. Mark after Break (≥ 8 µs auf 1),
//! 3. Startcode `0x00` und bis zu 512 Kanalbytes, je 250 kBaud, 8N2.
//!
//! Jedes Kabel bedient genau ein Universum und läuft in einem eigenen Thread
//! (ein Frame dauert ~23 ms – zwei Kabel nacheinander schafften die 40 Hz
//! nicht). Der Thread verbindet sich selbst neu, wenn das Kabel abgezogen
//! und wieder eingesteckt wird. Die Zuordnung Kabel → Universum läuft über
//! die Seriennummer des FTDI-Chips, damit sie nach einem Neustart oder
//! Umstecken gleich bleibt.

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use dmxcraft_core::{DmxFrame, UNIVERSE_SIZE};

use crate::{DmxOutput, OutputError};

/// Vendor-ID von FTDI.
pub const FTDI_VENDOR_ID: u16 = 0x0403;
/// DMX-Baudrate.
pub const DMX_BAUD: u32 = 250_000;
/// Break-Dauer (Norm: ≥ 88 µs; großzügig, weil der Schlaf auf manchen
/// Systemen ohnehin länger dauert).
const BREAK: Duration = Duration::from_micros(150);
/// Mark after Break (Norm: ≥ 8 µs).
const MAB: Duration = Duration::from_micros(20);
/// Übertragungsdauer eines vollen Frames: 513 Bytes × 11 Bit / 250 kBaud.
const FRAME_TIME: Duration = Duration::from_micros(513 * 11 * 4);
/// Wartezeit zwischen Verbindungsversuchen.
const RETRY: Duration = Duration::from_secs(1);

/// Ein erkanntes USB-Seriell-Gerät, das als DMX-Ausgang in Frage kommt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UsbDmxPort {
    /// Stabiler Schlüssel für die Zuordnung: Seriennummer, sonst Portname.
    pub key: String,
    /// Name des Ports (`COM3`, `/dev/ttyUSB0`, `/dev/cu.usbserial-…`).
    pub port_name: String,
    /// Anzeige in der Oberfläche.
    pub label: String,
    /// Ist ein FTDI-Chip (die übliche Bauart für Open-DMX-Kabel).
    pub ftdi: bool,
}

/// Listet die angeschlossenen USB-Seriell-Geräte, FTDI-Geräte zuerst.
pub fn list_ports() -> Vec<UsbDmxPort> {
    let Ok(ports) = serialport::available_ports() else {
        return Vec::new();
    };
    let mut out: Vec<UsbDmxPort> = ports
        .into_iter()
        // macOS meldet jedes Gerät als tty.* und cu.* – zum Senden nur cu.*
        .filter(|p| !cfg!(target_os = "macos") || p.port_name.starts_with("/dev/cu."))
        .filter_map(|p| match p.port_type {
            serialport::SerialPortType::UsbPort(usb) => {
                let ftdi = usb.vid == FTDI_VENDOR_ID;
                let serial = usb.serial_number.filter(|s| !s.trim().is_empty());
                let product = usb
                    .product
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "USB-Seriell".into());
                let label = match &serial {
                    Some(sn) => format!("{product} · {sn} ({})", p.port_name),
                    None => format!("{product} ({})", p.port_name),
                };
                Some(UsbDmxPort {
                    key: serial.unwrap_or_else(|| p.port_name.clone()),
                    port_name: p.port_name,
                    label,
                    ftdi,
                })
            }
            _ => None,
        })
        .collect();
    out.sort_by(|a, b| b.ftdi.cmp(&a.ftdi).then(a.port_name.cmp(&b.port_name)));
    out
}

/// Sucht den aktuellen Portnamen zu einem gespeicherten Schlüssel.
fn resolve(key: &str) -> Option<String> {
    list_ports()
        .into_iter()
        .find(|p| p.key == key || p.port_name == key)
        .map(|p| p.port_name)
}

/// Leitung, auf die ein DMX-Frame geschrieben wird (für Tests austauschbar).
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

/// Startcode + Kanaldaten eines Universums.
pub fn packet(channels: &[u8; UNIVERSE_SIZE]) -> [u8; UNIVERSE_SIZE + 1] {
    let mut buf = [0u8; UNIVERSE_SIZE + 1];
    buf[1..].copy_from_slice(channels);
    buf
}

/// Schreibt einen vollständigen DMX-Frame (Break, MAB, Startcode, Daten).
pub fn send_frame(line: &mut dyn DmxLine, channels: &[u8; UNIVERSE_SIZE]) -> io::Result<()> {
    line.set_break()?;
    std::thread::sleep(BREAK);
    line.clear_break()?;
    std::thread::sleep(MAB);
    let start = Instant::now();
    line.write_frame(&packet(channels))?;
    // Manche Treiber melden „geschrieben“, bevor das letzte Byte auf der
    // Leitung ist. Der nächste Break würde es abschneiden – also warten,
    // bis der Frame sicher übertragen ist.
    let elapsed = start.elapsed();
    if elapsed < FRAME_TIME {
        std::thread::sleep(FRAME_TIME - elapsed);
    }
    Ok(())
}

/// Gemeinsamer Zustand zwischen Ausgang und Sende-Thread eines Kabels.
struct Shared {
    data: Mutex<[u8; UNIVERSE_SIZE]>,
    stop: AtomicBool,
    connected: AtomicBool,
    error: Mutex<Option<String>>,
}

struct Worker {
    universe: usize,
    shared: Arc<Shared>,
    handle: Option<JoinHandle<()>>,
}

/// Ausgang über ein oder mehrere Open-DMX-Kabel.
pub struct OpenDmx {
    /// Je Universum (Index) der Schlüssel des zugeordneten Kabels.
    assign: Vec<Option<String>>,
    workers: Vec<Worker>,
}

impl OpenDmx {
    pub fn new(assign: Vec<Option<String>>) -> Self {
        Self {
            assign,
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

impl Drop for OpenDmx {
    fn drop(&mut self) {
        self.stop_workers();
    }
}

fn run_worker(key: String, universe: usize, shared: Arc<Shared>) {
    let set_error = |msg: String| {
        shared.connected.store(false, Ordering::Relaxed);
        if let Ok(mut e) = shared.error.lock() {
            *e = Some(msg);
        }
    };
    while !shared.stop.load(Ordering::Relaxed) {
        let Some(port_name) = resolve(&key) else {
            set_error(format!(
                "Universum {}: Kabel {key} nicht gefunden",
                universe + 1
            ));
            std::thread::sleep(RETRY);
            continue;
        };
        let port = serialport::new(&port_name, DMX_BAUD)
            .data_bits(serialport::DataBits::Eight)
            .stop_bits(serialport::StopBits::Two)
            .parity(serialport::Parity::None)
            .flow_control(serialport::FlowControl::None)
            .timeout(Duration::from_millis(200))
            .open();
        let mut port = match port {
            Ok(p) => p,
            Err(e) => {
                set_error(format!("Universum {}: {port_name}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                continue;
            }
        };
        shared.connected.store(true, Ordering::Relaxed);
        if let Ok(mut e) = shared.error.lock() {
            *e = None;
        }
        while !shared.stop.load(Ordering::Relaxed) {
            let channels = shared.data.lock().map(|d| *d).unwrap_or([0; UNIVERSE_SIZE]);
            if let Err(e) = send_frame(&mut port, &channels) {
                set_error(format!("Universum {}: {port_name}: {e}", universe + 1));
                std::thread::sleep(RETRY);
                break;
            }
        }
    }
    shared.connected.store(false, Ordering::Relaxed);
}

impl DmxOutput for OpenDmx {
    fn name(&self) -> &str {
        "USB-DMX (Open DMX)"
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
                "keinem Universum ist ein USB-DMX-Kabel zugeordnet (Einstellungen → Ausgang)"
                    .into(),
            ));
        }
        for (universe, key) in assigned {
            let shared = Arc::new(Shared {
                data: Mutex::new([0; UNIVERSE_SIZE]),
                stop: AtomicBool::new(false),
                connected: AtomicBool::new(false),
                error: Mutex::new(None),
            });
            let s = shared.clone();
            let handle = std::thread::Builder::new()
                .name(format!("open-dmx-u{}", universe + 1))
                .spawn(move || run_worker(key, universe, s))
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
                    .unwrap_or_else(|| format!("Universum {}: verbinde …", w.universe + 1))
            })
            .collect();
        (!problems.is_empty()).then(|| problems.join(" · "))
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
        assert_eq!(r.written[1], 255, "Kanal 1");
        assert_eq!(r.written[512], 7, "Kanal 512");
        // Der Frame wartet mindestens die Übertragungszeit ab
        assert!(t.elapsed() >= FRAME_TIME);
    }

    /// Ende-zu-Ende über ein echtes Terminal-Paar: Konfiguration wie im
    /// Betrieb, Bytes kommen auf der Gegenseite vollständig an.
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
        // Break gibt es auf Pseudo-Terminals nicht immer – dann nur Daten prüfen
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
        // Ein Break kann auf der Gegenseite als 0-Byte ankommen
        let frame = &got[got.len() - (UNIVERSE_SIZE + 1)..];
        assert_eq!(frame[0], 0, "Startcode (break geprüft: {ok})");
        assert_eq!(frame[1], 255);
        assert_eq!(frame[2], 128);
    }

    #[test]
    fn open_without_assignment_fails_clearly() {
        let mut o = OpenDmx::new(vec![None, None]);
        assert!(matches!(o.open(), Err(OutputError::NotFound(_))));
    }

    #[test]
    fn missing_cable_is_reported_not_fatal() {
        let mut o = OpenDmx::new(vec![Some("GIBTS-NICHT".into())]);
        o.open().unwrap();
        // Senden geht weiter, der Thread meldet das fehlende Kabel
        o.send(&DmxFrame::new(1)).unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        let mut health = None;
        while Instant::now() < deadline {
            health = o.health();
            if health
                .as_deref()
                .is_some_and(|h| h.contains("nicht gefunden"))
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(health.unwrap().contains("Universum 1"));
        o.close();
    }
}
