//! Crash handling: a panic is written to `crash.log` in the settings folder
//! (the Windows build has no console, so it would vanish otherwise). If the
//! program itself (main thread) goes down with unsaved changes, the show is
//! saved as recovery copy right away, and the next start says what happened.
//!
//! Panics in background threads (output, MCP, decoders) are only logged:
//! those threads catch them and keep running.

use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};

use dmxcraft_core::Engine;

/// Log files larger than this are started anew.
const MAX_LOG: u64 = 256 * 1024;

struct Armed {
    engine: Arc<Mutex<Engine>>,
    /// Recovery copy and its info file (see `autosave`).
    files: Option<(PathBuf, PathBuf)>,
    dirty: AtomicBool,
    path: Mutex<String>,
}

static ARMED: OnceLock<Armed> = OnceLock::new();

fn dir() -> Option<PathBuf> {
    crate::settings::Settings::path().and_then(|p| p.parent().map(|d| d.to_path_buf()))
}

pub fn log_file() -> Option<PathBuf> {
    dir().map(|d| d.join("crash.log"))
}

fn marker_file() -> Option<PathBuf> {
    dir().map(|d| d.join("crash.pending"))
}

/// Install the panic hook (at program start).
pub fn install() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let name = thread.name().unwrap_or("unnamed").to_string();
        let main = name == "main";
        let msg = info
            .payload()
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "(no message)".into());
        let place = info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();
        let trace = std::backtrace::Backtrace::force_capture();
        write_log(&format!(
            "=== {} · DMXCraft {} · thread {name}\n{msg}\nat {place}\n{trace}\n",
            timestamp(),
            env!("CARGO_PKG_VERSION"),
        ));
        if main {
            emergency_save();
            if let Some(m) = marker_file() {
                let _ = std::fs::write(m, msg);
            }
        }
        default(info);
    }));
}

/// Make the show reachable for the emergency save.
pub fn arm(engine: Arc<Mutex<Engine>>, files: Option<(PathBuf, PathBuf)>) {
    let _ = ARMED.set(Armed {
        engine,
        files,
        dirty: AtomicBool::new(false),
        path: Mutex::new(String::new()),
    });
}

/// Current state for the emergency save (called every frame, cheap).
pub fn update(dirty: bool, path: &str) {
    let Some(a) = ARMED.get() else {
        return;
    };
    a.dirty.store(dirty, Ordering::Relaxed);
    if let Ok(mut p) = a.path.try_lock() {
        if *p != path {
            *p = path.to_string();
        }
    }
}

/// Was the last run ended by an error? (Returns its message once.)
pub fn take_last_crash() -> Option<String> {
    let m = marker_file()?;
    let msg = std::fs::read_to_string(&m).ok()?;
    let _ = std::fs::remove_file(m);
    Some(msg)
}

fn emergency_save() {
    let Some(a) = ARMED.get() else {
        return;
    };
    let Some((show_file, info_file)) = &a.files else {
        return;
    };
    if !a.dirty.load(Ordering::Relaxed) {
        return;
    }
    // try_lock: the panicking thread may hold the engine itself
    let show = match a.engine.try_lock() {
        Ok(e) => e.show().clone(),
        Err(std::sync::TryLockError::Poisoned(p)) => p.into_inner().show().clone(),
        Err(std::sync::TryLockError::WouldBlock) => return,
    };
    if show.save(show_file).is_ok() {
        let info = crate::autosave::RecoveryInfo {
            path: a.path.try_lock().map(|p| p.clone()).unwrap_or_default(),
            name: show.name.clone(),
            saved_at: crate::autosave::now_secs(),
        };
        if let Ok(json) = serde_json::to_string_pretty(&info) {
            let _ = std::fs::write(info_file, json);
        }
    }
}

fn write_log(text: &str) {
    let Some(path) = log_file() else {
        return;
    };
    if let Some(d) = path.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    let fresh = std::fs::metadata(&path).is_ok_and(|m| m.len() > MAX_LOG);
    let file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(!fresh)
        .truncate(fresh)
        .open(&path);
    if let Ok(mut f) = file {
        let _ = f.write_all(text.as_bytes());
    }
}

fn timestamp() -> String {
    let secs = crate::autosave::now_secs();
    // UTC, without a date library: days since 1970 → date
    let (days, rest) = (secs / 86_400, secs % 86_400);
    let (y, m, d) = civil(days as i64);
    format!(
        "{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02} UTC",
        rest / 3600,
        rest / 60 % 60,
        rest % 60
    )
}

/// Days since 1970-01-01 → (year, month, day) (H. Hinnant's algorithm).
fn civil(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

#[cfg(test)]
mod tests {
    #[test]
    fn dates() {
        assert_eq!(super::civil(0), (1970, 1, 1));
        assert_eq!(super::civil(20_000), (2024, 10, 4));
    }
}
