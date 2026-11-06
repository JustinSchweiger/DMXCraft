//! Systemdialoge zum Öffnen und Speichern (Explorer, Finder, GTK/KDE-Portal).
//!
//! Der Dialog läuft in einem eigenen Thread, damit die Oberfläche (und die
//! DMX-Ausgabe-Anzeige) weiterzeichnet; das Ergebnis holt `poll` ab.

use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};

use eframe::egui;

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
            let mut dialog = rfd::FileDialog::new().add_filter("DMXCraft-Show", &["json"]);
            if let Some(dir) = current.parent().filter(|d| d.is_dir()) {
                dialog = dialog.set_directory(dir);
            }
            let result = match purpose {
                Purpose::OpenShow => dialog.set_title("Show öffnen").pick_file(),
                Purpose::SaveShowAs => {
                    let name = current
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_else(|| "Show.json".into());
                    dialog
                        .set_title("Show speichern unter")
                        .set_file_name(name)
                        .save_file()
                        .map(with_json_extension)
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

fn with_json_extension(p: PathBuf) -> PathBuf {
    if p.extension().is_some() {
        p
    } else {
        p.with_extension("json")
    }
}
