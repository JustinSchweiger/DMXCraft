//! MIDI inputs and outputs (midir: WinMM, CoreMIDI, ALSA). All inputs are
//! opened; new devices are detected every few seconds. Incoming messages
//! land in a queue that the app frame collects every frame.

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, Instant};

use dmxcraft_core::mapping::MidiMessage;

/// A received message.
#[derive(Clone, Debug, PartialEq)]
pub struct MidiEvent {
    pub device: String,
    /// Channel 1–16.
    pub channel: u8,
    pub message: MidiMessage,
    pub number: u8,
    /// Velocity or controller value 0–127 (note off = 0).
    pub value: u8,
}

/// Raw bytes → event (only Note On/Off and Control Change).
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
    /// Names of the outputs (for selection lists).
    pub output_names: Vec<String>,
    pub input_names: Vec<String>,
    last_scan: Option<Instant>,
    /// Outputs connected since the last collection (send feedback again).
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
    /// Look for devices (at most every 3 s) and open new inputs.
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

    /// Collect received messages.
    pub fn drain(&mut self) -> Vec<MidiEvent> {
        self.rx.try_iter().collect()
    }

    /// Send a message to an output device (opens it if needed).
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
