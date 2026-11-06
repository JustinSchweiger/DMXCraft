//! Systemdialoge zum Öffnen und Speichern (Explorer, Finder, GTK/KDE-Portal).
//!
//! Der Dialog läuft in einem eigenen Thread, damit die Oberfläche (und die
//! DMX-Ausgabe-Anzeige) weiterzeichnet; das Ergebnis holt `poll` ab.

use crate::i18n::t;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use eframe::egui;

/// Dateiendung der Shows (Inhalt ist JSON). Ältere `.json`-Shows lassen sich
/// weiter öffnen.
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
    /// Öffnet den Dialog. `current` ist die aktuelle Show-Datei (Startordner, Name).
    pub fn start(purpose: Purpose, current: &str, ctx: &egui::Context) -> Self {
        let (tx, rx) = channel();
        let current = PathBuf::from(current);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new().add_filter(t("DMXCraft-Show"), &[SHOW_EXT]);
            if purpose == Purpose::OpenShow {
                dialog = dialog.add_filter(t("Ältere Show (JSON)"), &["json"]);
            }
            if let Some(dir) = current.parent().filter(|d| d.is_dir()) {
                dialog = dialog.set_directory(dir);
            }
            let result = match purpose {
                Purpose::OpenShow => dialog.set_title(t("Show öffnen")).pick_file(),
                Purpose::SaveShowAs => {
                    let name = current
                        .file_stem()
                        .map(|n| format!("{}.{SHOW_EXT}", n.to_string_lossy()))
                        .unwrap_or_else(|| format!("Show.{SHOW_EXT}"));
                    dialog
                        .set_title(t("Show speichern unter"))
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

    /// `Some` sobald der Dialog geschlossen wurde (innerer Wert `None` = abgebrochen).
    pub fn poll(&self) -> Option<(Purpose, Option<PathBuf>)> {
        self.rx.try_recv().ok().map(|r| (self.purpose, r))
    }
}

/// Speichern unter: immer mit `.dmxcraft` (auch wenn `.json` gewählt war).
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
