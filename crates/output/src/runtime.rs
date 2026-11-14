//! Real-time thread: clocks the engine, renders frames and sends them.

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use dmxcraft_core::{DmxFrame, Engine};

use crate::DmxOutput;

/// How long to wait after an error before reconnecting.
const RECONNECT_DELAY: Duration = Duration::from_secs(2);

#[derive(Clone, Debug, PartialEq)]
pub enum OutputStatus {
    Connecting,
    /// `note`: partial problem of the output, e.g. a missing cable.
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

/// Owns the output thread. It is shut down cleanly on drop.
pub struct Runtime {
    engine: Arc<Mutex<Engine>>,
    last_frame: Arc<Mutex<DmxFrame>>,
    status: Arc<Mutex<OutputStatus>>,
    /// Load of the output clock in per mille (compute time / tick duration,
    /// smoothed).
    load: Arc<AtomicU32>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Runtime {
    /// Starts the thread with `hz` frames per second (DMX usually: 25–44).
    pub fn start(engine: Arc<Mutex<Engine>>, mut output: Box<dyn DmxOutput>, hz: u32) -> Self {
        let hz = hz.clamp(1, 60);
        let period = Duration::from_secs_f64(1.0 / f64::from(hz));
        let universes = usize::from(
            engine
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .show()
                .universes,
        );
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
                    *t_status.lock().unwrap_or_else(|p| p.into_inner()) = s;
                };
                let mut connected = false;
                let mut retry_at = Instant::now();
                let mut last_tick = Instant::now();
                let mut next = Instant::now();
                let mut fps_count = 0u32;
                let mut fps_since = Instant::now();
                let mut frame = DmxFrame::new(universes);
                let mut engine_failed = false;

                while !t_stop.load(Ordering::Relaxed) {
                    let now = Instant::now();
                    if next > now {
                        std::thread::sleep((next - now).min(Duration::from_millis(50)));
                        continue;
                    }
                    next += period;
                    if next < now {
                        next = now + period; // do not catch up after stalls
                    }

                    // Always clock the engine, even without a device – the UI then shows the
                    // values in the monitor.
                    let work_start = Instant::now();
                    let dt = now - last_tick;
                    last_tick = now;
                    {
                        // A panic elsewhere while holding the engine must not
                        // stop the light: keep clocking with the engine as it is
                        let mut e = t_engine.lock().unwrap_or_else(|p| p.into_inner());
                        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            e.tick(dt);
                            e.render(&mut frame);
                        }));
                        if ok.is_err() && !engine_failed {
                            engine_failed = true;
                            eprintln!("engine panicked during tick; output keeps running");
                        }
                    }
                    t_frame
                        .lock()
                        .unwrap_or_else(|p| p.into_inner())
                        .clone_from(&frame);

                    // Compute time of the engine relative to the tick duration
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
            .expect("Could not start the output thread");

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

    /// Last computed frame (for the DMX monitor).
    pub fn last_frame(&self) -> DmxFrame {
        self.last_frame
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clone()
    }

    pub fn status(&self) -> OutputStatus {
        self.status
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .clone()
    }

    /// Load of the output clock, 0.0..=1.0 (above 1.0 = the clock is not kept).
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

/// Moving average of the load (per mille).
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
        // wait generously on slow CI machines (macOS)
        let t0 = std::time::Instant::now();
        while frames.lock().unwrap().len() < 3 && t0.elapsed() < Duration::from_secs(3) {
            std::thread::sleep(Duration::from_millis(20));
        }
        rt.stop();
        let frames = frames.lock().unwrap();
        assert!(frames.len() >= 3, "only {} frames", frames.len());
        assert_eq!(frames.last().unwrap().get(0, 5), Some(222));
        assert_eq!(rt.status(), OutputStatus::Stopped);
        assert!(rt.load() < 1.0, "Load {} with a tiny show", rt.load());
    }

    #[test]
    fn output_keeps_running_after_a_panic_elsewhere() {
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
                value: 99,
            }])],
        );
        let engine = Arc::new(Mutex::new(Engine::new(show)));
        // another thread panics while holding the engine: the lock is poisoned
        let e2 = engine.clone();
        let _ = std::thread::spawn(move || {
            let _guard = e2.lock().unwrap();
            panic!("test panic");
        })
        .join();
        assert!(engine.is_poisoned());
        engine
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .activate(id);
        let rec = RecordingOutput::default();
        let frames = rec.frames.clone();
        let mut rt = Runtime::start(engine, Box::new(rec), 50);
        let t0 = std::time::Instant::now();
        while frames.lock().unwrap().len() < 3 && t0.elapsed() < Duration::from_secs(3) {
            std::thread::sleep(Duration::from_millis(20));
        }
        rt.stop();
        let frames = frames.lock().unwrap();
        assert_eq!(frames.last().unwrap().get(0, 5), Some(99));
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
