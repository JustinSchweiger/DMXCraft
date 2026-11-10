//! Auto-save and crash recovery.
//!
//! * **Recovery:** while the show has unsaved changes, a copy is written to
//!   the settings folder (`recovery/`) every few seconds. A clean exit, saving
//!   or discarding the changes removes it again. If a copy is still there at
//!   the next start, DMXCraft was not closed properly and offers to restore
//!   it. A lock file makes sure only the first running instance does this.
//! * **Auto-save:** every n minutes the show is saved to its file (only when
//!   it has one and there are unsaved changes; setting `autosave_minutes`).

use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use dmxcraft_core::Show;
use serde::{Deserialize, Serialize};

/// How often the recovery copy is updated.
const RECOVERY_EVERY: Duration = Duration::from_secs(10);

/// What is known about the recovery copy.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RecoveryInfo {
    /// File of the show (empty = never saved).
    pub path: String,
    /// Name of the show.
    pub name: String,
    /// Time of the copy (seconds since 1970).
    pub saved_at: u64,
}

impl RecoveryInfo {
    /// Age of the copy in minutes.
    pub fn minutes_ago(&self) -> u64 {
        now_secs().saturating_sub(self.saved_at) / 60
    }
}

pub struct AutoSave {
    /// Folder of the recovery copy; None = recovery off (no settings folder
    /// or another instance is running).
    dir: Option<PathBuf>,
    /// Held while running: only one instance writes recovery copies.
    _lock: Option<File>,
    last_recovery: Instant,
    /// Show of the last recovery copy (do not write the same again).
    written: Option<Show>,
    last_autosave: Instant,
}

impl AutoSave {
    pub fn start() -> Self {
        let dir = crate::settings::Settings::path()
            .and_then(|p| p.parent().map(|d| d.join("recovery")))
            .filter(|d| std::fs::create_dir_all(d).is_ok());
        let lock = dir.as_ref().and_then(|d| {
            let f = File::create(d.join("instance.lock")).ok()?;
            f.try_lock().ok().map(|()| f)
        });
        Self {
            dir: dir.filter(|_| lock.is_some()),
            _lock: lock,
            last_recovery: Instant::now(),
            written: None,
            last_autosave: Instant::now(),
        }
    }

    fn show_file(&self) -> Option<PathBuf> {
        self.dir.as_ref().map(|d| d.join("recovery.dmxcraft"))
    }

    fn info_file(&self) -> Option<PathBuf> {
        self.dir.as_ref().map(|d| d.join("recovery.json"))
    }

    /// Recovery copy left over from a crash, if any.
    pub fn pending(&self) -> Option<RecoveryInfo> {
        let show = self.show_file()?;
        if !show.exists() {
            return None;
        }
        let info = std::fs::read_to_string(self.info_file()?).ok();
        Some(
            info.and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default(),
        )
    }

    /// Load the recovery copy.
    pub fn load(&self) -> Option<Show> {
        Show::load(self.show_file()?).ok()
    }

    /// Remove the recovery copy (saved, discarded or closed normally).
    pub fn clear(&mut self) {
        self.written = None;
        for f in [self.show_file(), self.info_file()].into_iter().flatten() {
            let _ = std::fs::remove_file(f);
        }
    }

    /// Move the recovery copy aside (`recovery-<time>.dmxcraft`), so it is
    /// neither offered again nor overwritten. Returns the new file.
    pub fn keep_aside(&mut self) -> Option<PathBuf> {
        let (dir, file) = (self.dir.clone()?, self.show_file()?);
        let to = dir.join(format!("recovery-{}.dmxcraft", now_secs()));
        std::fs::rename(file, &to).ok()?;
        if let Some(info) = self.info_file() {
            let _ = std::fs::remove_file(info);
        }
        Some(to)
    }

    /// Call regularly: writes the recovery copy when the show has unsaved
    /// changes (`dirty`), removes it otherwise.
    pub fn recovery_tick(&mut self, show: &Show, dirty: bool, path: &str) {
        if self.dir.is_none() || self.last_recovery.elapsed() < RECOVERY_EVERY {
            return;
        }
        self.last_recovery = Instant::now();
        if !dirty {
            if self.written.is_some() {
                self.clear();
            }
            return;
        }
        if self.written.as_ref() == Some(show) {
            return;
        }
        self.written = Some(show.clone());
        let (Some(file), Some(info_file)) = (self.show_file(), self.info_file()) else {
            return;
        };
        let info = RecoveryInfo {
            path: path.to_string(),
            name: show.name.clone(),
            saved_at: now_secs(),
        };
        // Write in the background: serializing a large show takes a moment
        let show = show.clone();
        std::thread::spawn(move || {
            if show.save(&file).is_ok() {
                if let Ok(json) = serde_json::to_string_pretty(&info) {
                    let _ = std::fs::write(info_file, json);
                }
            }
        });
    }

    /// Is it time to save the show to its file? `minutes` = 0: off.
    pub fn autosave_due(&mut self, minutes: u32, dirty: bool, has_path: bool) -> bool {
        if minutes == 0 || !dirty || !has_path {
            self.last_autosave = Instant::now();
            return false;
        }
        if self.last_autosave.elapsed() < Duration::from_secs(u64::from(minutes) * 60) {
            return false;
        }
        self.last_autosave = Instant::now();
        true
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn off() -> AutoSave {
        AutoSave {
            dir: None,
            _lock: None,
            last_recovery: Instant::now(),
            written: None,
            last_autosave: Instant::now(),
        }
    }

    #[test]
    fn autosave_only_when_due_dirty_and_saved_before() {
        let mut a = off();
        assert!(!a.autosave_due(1, true, true), "not yet due");
        a.last_autosave = Instant::now() - Duration::from_secs(61);
        assert!(!a.autosave_due(0, true, true), "off");
        a.last_autosave = Instant::now() - Duration::from_secs(61);
        assert!(!a.autosave_due(1, false, true), "nothing changed");
        a.last_autosave = Instant::now() - Duration::from_secs(61);
        assert!(!a.autosave_due(1, true, false), "never saved: no file");
        a.last_autosave = Instant::now() - Duration::from_secs(61);
        assert!(a.autosave_due(1, true, true));
        assert!(!a.autosave_due(1, true, true), "interval starts again");
    }

    #[test]
    fn without_folder_nothing_pending() {
        let mut a = off();
        assert!(a.pending().is_none());
        a.recovery_tick(&Show::new("t", 1), true, "");
        a.clear();
    }
}
