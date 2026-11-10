//! System dialogs for opening and saving (Explorer, Finder, GTK/KDE portal).
//!
//! The dialog runs in its own thread so the UI (and the DMX output display)
//! keeps drawing; `poll` collects the result.

use crate::i18n::t;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use eframe::egui;

/// File extension of shows (content is JSON). Older `.json` shows can still
/// be opened.
pub const SHOW_EXT: &str = "dmxcraft";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    OpenShow,
    SaveShowAs,
}

pub struct Pending {
    purpose: Purpose,
    rx: Receiver<Option<PathBuf>>,
}

impl Pending {
    /// Opens the dialog. `current` is the current show file (start folder, name).
    pub fn start(purpose: Purpose, current: &str, ctx: &egui::Context) -> Self {
        let (tx, rx) = channel();
        let current = PathBuf::from(current);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new().add_filter(t("DMXCraft show"), &[SHOW_EXT]);
            if purpose == Purpose::OpenShow {
                dialog = dialog.add_filter(t("Older show (JSON)"), &["json"]);
            }
            if let Some(dir) = current.parent().filter(|d| d.is_dir()) {
                dialog = dialog.set_directory(dir);
            }
            let result = match purpose {
                Purpose::OpenShow => dialog.set_title(t("Open show")).pick_file(),
                Purpose::SaveShowAs => {
                    let name = current
                        .file_stem()
                        .map(|n| format!("{}.{SHOW_EXT}", n.to_string_lossy()))
                        .unwrap_or_else(|| format!("Show.{SHOW_EXT}"));
                    dialog
                        .set_title(t("Save show as"))
                        .set_file_name(name)
                        .save_file()
                        .map(with_show_extension)
                }
            };
            let _ = tx.send(result);
            ctx.request_repaint();
        });
        Self { purpose, rx }
    }

    /// `Some` once the dialog has been closed (inner value `None` = cancelled).
    pub fn poll(&self) -> Option<(Purpose, Option<PathBuf>)> {
        self.rx.try_recv().ok().map(|r| (self.purpose, r))
    }
}

/// Save as: always with `.dmxcraft` (even if `.json` was chosen).
fn with_show_extension(p: PathBuf) -> PathBuf {
    if p.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case(SHOW_EXT))
    {
        p
    } else {
        p.with_extension(SHOW_EXT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_as_gets_show_extension() {
        assert_eq!(
            with_show_extension("a/b".into()),
            PathBuf::from("a/b.dmxcraft")
        );
        assert_eq!(
            with_show_extension("a/b.json".into()),
            PathBuf::from("a/b.dmxcraft")
        );
        assert_eq!(
            with_show_extension("a/b.DMXCraft".into()),
            PathBuf::from("a/b.DMXCraft")
        );
    }
}
