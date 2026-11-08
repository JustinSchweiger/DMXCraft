//! MIDI-Ein- und -Ausgänge (midir: WinMM, CoreMIDI, ALSA). Alle Eingänge
//! werden geöffnet; neue Geräte werden alle paar Sekunden erkannt. Eingehende
//! Nachrichten landen in einer Warteschlange, die der App-Rahmen je Bild
//! abholt.

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use dmxcraft_core::mapping::MidiMessage;

/// Eine eingegangene Nachricht.
#[derive(Clone, Debug, PartialEq)]
pub struct MidiEvent {
    pub device: String,
    /// Kanal 1–16.
    pub channel: u8,
    pub message: MidiMessage,
    pub number: u8,
    /// Anschlag bzw. Reglerwert 0–127 (Note Off = 0).
    pub value: u8,
}

/// Rohbytes → Ereignis (nur Note On/Off und Control Change).
pub fn parse(device: &str, bytes: &[u8]) -> Option<MidiEvent> {
    let (&status, rest) = bytes.split_first()?;
    let channel = (status & 0x0f) + 1;
    let (message, value) = match status & 0xf0 {
        0x90 => (MidiMessage::Note, *rest.get(1)?),
        0x80 => (MidiMessage::Note, 0),
        0xb0 => (MidiMessage::Cc, *rest.get(1)?),
        _ => return None,
    };
    Some(MidiEvent {
        device: device.to_string(),
        channel,
        message,
        number: *rest.first()? & 0x7f,
        value: value & 0x7f,
    })
}

pub struct MidiHub {
    tx: Sender<MidiEvent>,
    rx: Receiver<MidiEvent>,
    inputs: HashMap<String, midir::MidiInputConnection<()>>,
    outputs: HashMap<String, midir::MidiOutputConnection>,
    /// Namen der Ausgänge (für Auswahllisten).
    pub output_names: Vec<String>,
    pub input_names: Vec<String>,
    last_scan: Option<Instant>,
    /// Seit dem letzten Abholen neu verbundene Ausgänge (Rückmeldung neu senden).
    pub new_outputs: bool,
}

impl Default for MidiHub {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            rx,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            output_names: Vec::new(),
            input_names: Vec::new(),
            last_scan: None,
            new_outputs: false,
        }
    }
}

impl MidiHub {
    /// Geräte suchen (höchstens alle 3 s) und neue Eingänge öffnen.
    pub fn scan(&mut self) {
        if self
            .last_scan
            .is_some_and(|t| t.elapsed() < Duration::from_secs(3))
        {
            return;
        }
        self.last_scan = Some(Instant::now());
        if let Ok(probe) = midir::MidiInput::new("DMXCraft") {
            let ports = probe.ports();
            let names: Vec<String> = ports
                .iter()
                .filter_map(|p| probe.port_name(p).ok())
                .collect();
            self.inputs.retain(|n, _| names.contains(n));
            for name in &names {
                if self.inputs.contains_key(name) || name.contains("DMXCraft") {
                    continue;
                }
                let Ok(input) = midir::MidiInput::new("DMXCraft") else {
                    continue;
                };
                let Some(port) = input
                    .ports()
                    .into_iter()
                    .find(|p| input.port_name(p).ok().as_ref() == Some(name))
                else {
                    continue;
                };
                let tx = self.tx.clone();
                let dev = name.clone();
                if let Ok(conn) = input.connect(
                    &port,
                    "dmxcraft-in",
                    move |_, bytes, _| {
                        if let Some(e) = parse(&dev, bytes) {
                            let _ = tx.send(e);
                        }
                    },
                    (),
                ) {
                    self.inputs.insert(name.clone(), conn);
                }
            }
            self.input_names = names;
        }
        if let Ok(probe) = midir::MidiOutput::new("DMXCraft") {
            let names: Vec<String> = probe
                .ports()
                .iter()
                .filter_map(|p| probe.port_name(p).ok())
                .filter(|n| !n.contains("DMXCraft"))
                .collect();
            self.outputs.retain(|n, _| names.contains(n));
            self.output_names = names;
        }
    }

    /// Eingegangene Nachrichten abholen.
    pub fn drain(&mut self) -> Vec<MidiEvent> {
        self.rx.try_iter().collect()
    }

    /// Nachricht an ein Ausgabegerät senden (öffnet es bei Bedarf).
    pub fn send(&mut self, device: &str, bytes: &[u8]) {
        if !self.outputs.contains_key(device) {
            let Ok(out) = midir::MidiOutput::new("DMXCraft") else {
                return;
            };
            let Some(port) = out
                .ports()
                .into_iter()
                .find(|p| out.port_name(p).ok().as_deref() == Some(device))
            else {
                return;
            };
            match out.connect(&port, "dmxcraft-out") {
                Ok(c) => {
                    self.outputs.insert(device.to_string(), c);
                    self.new_outputs = true;
                }
                Err(_) => return,
            }
        }
        if let Some(c) = self.outputs.get_mut(device) {
            if c.send(bytes).is_err() {
                self.outputs.remove(device);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_note_and_cc() {
        let e = parse("APC", &[0x90, 56, 127]).unwrap();
        assert_eq!((e.channel, e.number, e.value), (1, 56, 127));
        assert_eq!(e.message, MidiMessage::Note);
        let off = parse("APC", &[0x81, 56, 64]).unwrap();
        assert_eq!((off.channel, off.value), (2, 0));
        let cc = parse("APC", &[0xb0, 48, 100]).unwrap();
        assert_eq!(cc.message, MidiMessage::Cc);
        assert!(parse("APC", &[0xf8]).is_none());
    }
}
