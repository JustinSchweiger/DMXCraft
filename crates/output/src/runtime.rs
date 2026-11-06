//! Echtzeit-Thread: taktet die Engine, rendert Frames und sendet sie.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use dmxcraft_core::{DmxFrame, Engine};

use crate::DmxOutput;

/// Wie lange nach einem Fehler gewartet wird, bevor neu verbunden wird.
const RECONNECT_DELAY: Duration = Duration::from_secs(2);

#[derive(Clone, Debug, PartialEq)]
pub enum OutputStatus {
    Connecting,
    /// `note`: Teilproblem des Ausgangs, z. B. ein fehlendes Kabel.
    Running {
        output: String,
        fps: f32,
        note: Option<String>,
    },
    Error(String),
    Stopped,
}

impl OutputStatus {
    pub fn is_running(&self) -> bool {
        matches!(self, OutputStatus::Running { .. })
    }
}

/// Besitzt den Ausgabe-Thread. Beim Drop wird er sauber beendet.
pub struct Runtime {
    engine: Arc<Mutex<Engine>>,
    last_frame: Arc<Mutex<DmxFrame>>,
    status: Arc<Mutex<OutputStatus>>,
    /// Auslastung des Ausgabe-Takts in Promille (Rechenzeit / Taktdauer, geglättet).
    load: Arc<AtomicU32>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Runtime {
    /// Startet den Thread mit `hz` Frames pro Sekunde (DMX üblich: 25–44).
    pub fn start(engine: Arc<Mutex<Engine>>, mut output: Box<dyn DmxOutput>, hz: u32) -> Self {
        let hz = hz.clamp(1, 60);
        let period = Duration::from_secs_f64(1.0 / f64::from(hz));
        let universes = engine.lock().map_or(1, |e| usize::from(e.show().universes));
        let last_frame = Arc::new(Mutex::new(DmxFrame::new(universes)));
        let status = Arc::new(Mutex::new(OutputStatus::Connecting));
        let stop = Arc::new(AtomicBool::new(false));
        let load = Arc::new(AtomicU32::new(0));

        let (t_engine, t_frame, t_status, t_stop, t_load) = (
            engine.clone(),
            last_frame.clone(),
            status.clone(),
            stop.clone(),
            load.clone(),
        );

        let handle = std::thread::Builder::new()
            .name("dmxcraft-output".into())
            .spawn(move || {
                let set_status = |s: OutputStatus| {
                    if let Ok(mut g) = t_status.lock() {
                        *g = s;
                    }
                };
                let mut connected = false;
                let mut retry_at = Instant::now();
                let mut last_tick = Instant::now();
                let mut next = Instant::now();
                let mut fps_count = 0u32;
                let mut fps_since = Instant::now();
                let mut frame = DmxFrame::new(universes);

                while !t_stop.load(Ordering::Relaxed) {
                    let now = Instant::now();
                    if next > now {
                        std::thread::sleep((next - now).min(Duration::from_millis(50)));
                        continue;
                    }
                    next += period;
                    if next < now {
                        next = now + period; // nach Hängern nicht nachholen
                    }

                    // Engine immer takten, auch ohne Gerät – die Oberfläche
                    // zeigt die Werte dann im Monitor.
                    let work_start = Instant::now();
                    let dt = now - last_tick;
                    last_tick = now;
                    if let Ok(mut e) = t_engine.lock() {
                        e.tick(dt);
                        e.render(&mut frame);
                    }
                    if let Ok(mut lf) = t_frame.lock() {
                        lf.clone_from(&frame);
                    }

                    // Rechenzeit der Engine relativ zur Taktdauer
                    record_load(&t_load, work_start.elapsed(), period);

                    if !connected {
                        if now < retry_at {
                            continue;
                        }
                        match output.open() {
                            Ok(()) => {
                                connected = true;
                                fps_count = 0;
                                fps_since = now;
                                set_status(OutputStatus::Running {
                                    output: output.name().into(),
                                    fps: 0.0,
                                    note: output.health(),
                                });
                            }
                            Err(err) => {
                                set_status(OutputStatus::Error(err.to_string()));
                                retry_at = now + RECONNECT_DELAY;
                                continue;
                            }
                        }
                    }

                    match output.send(&frame) {
                        Ok(()) => {
                            fps_count += 1;
                            let el = fps_since.elapsed();
                            if el >= Duration::from_secs(1) {
                                set_status(OutputStatus::Running {
                                    output: output.name().into(),
                                    fps: fps_count as f32 / el.as_secs_f32(),
                                    note: output.health(),
                                });
                                fps_count = 0;
                                fps_since = Instant::now();
                            }
                        }
                        Err(err) => {
                            output.close();
                            connected = false;
                            set_status(OutputStatus::Error(err.to_string()));
                            retry_at = now + RECONNECT_DELAY;
                        }
                    }
                }
                output.close();
                set_status(OutputStatus::Stopped);
            })
            .expect("Ausgabe-Thread konnte nicht gestartet werden");

        Self {
            engine,
            last_frame,
            status,
            load,
            stop,
            handle: Some(handle),
        }
    }

    pub fn engine(&self) -> &Arc<Mutex<Engine>> {
        &self.engine
    }

    /// Zuletzt berechneter Frame (für den DMX-Monitor).
    pub fn last_frame(&self) -> DmxFrame {
        self.last_frame
            .lock()
            .map(|f| f.clone())
            .unwrap_or_else(|_| DmxFrame::new(1))
    }

    pub fn status(&self) -> OutputStatus {
        self.status
            .lock()
            .map(|s| s.clone())
            .unwrap_or(OutputStatus::Error("Status nicht lesbar".into()))
    }

    /// Auslastung des Ausgabe-Takts, 0.0..=1.0 (über 1.0 = Takt wird nicht gehalten).
    pub fn load(&self) -> f32 {
        self.load.load(Ordering::Relaxed) as f32 / 1000.0
    }

    pub fn stop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

/// Gleitender Mittelwert der Auslastung (Promille).
fn record_load(load: &AtomicU32, work: Duration, period: Duration) {
    let sample = (work.as_secs_f64() / period.as_secs_f64() * 1000.0).min(5000.0) as u32;
    let old = load.load(Ordering::Relaxed);
    let new = (old * 9 + sample) / 10;
    load.store(new, Ordering::Relaxed);
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OutputError, RecordingOutput};
    use dmxcraft_core::library::builtin_fixture_types;
    use dmxcraft_core::{ChannelValue, Show, Step};

    #[test]
    fn runtime_sends_rendered_frames() {
        let mut show = Show::new("T", 1);
        for t in builtin_fixture_types() {
            show.upsert_fixture_type(t);
        }
        let f = show.add_fixture("D", "generic/dimmer-1ch", 0, 5).unwrap();
        let id = show.add_scene(
            "An",
            None,
            vec![Step::new(vec![ChannelValue {
                fixture: f,
                channel: 0,
                value: 222,
            }])],
        );
        let mut engine = Engine::new(show);
        engine.activate(id);
        let engine = Arc::new(Mutex::new(engine));
        let rec = RecordingOutput::default();
        let frames = rec.frames.clone();
        let mut rt = Runtime::start(engine, Box::new(rec), 50);
        std::thread::sleep(Duration::from_millis(200));
        rt.stop();
        let frames = frames.lock().unwrap();
        assert!(frames.len() >= 3, "nur {} Frames", frames.len());
        assert_eq!(frames.last().unwrap().get(0, 5), Some(222));
        assert_eq!(rt.status(), OutputStatus::Stopped);
        assert!(
            rt.load() < 1.0,
            "Auslastung {} bei winziger Show",
            rt.load()
        );
    }

    struct Failing;
    impl DmxOutput for Failing {
        fn name(&self) -> &str {
            "Failing"
        }
        fn open(&mut self) -> Result<(), OutputError> {
            Err(OutputError::NotFound("weg".into()))
        }
        fn send(&mut self, _: &DmxFrame) -> Result<(), OutputError> {
            unreachable!()
        }
    }

    #[test]
    fn engine_keeps_running_without_device() {
        let engine = Arc::new(Mutex::new(Engine::new(Show::new("T", 1))));
        let rt = Runtime::start(engine, Box::new(Failing), 50);
        std::thread::sleep(Duration::from_millis(100));
        assert!(matches!(rt.status(), OutputStatus::Error(_)));
    }
}
