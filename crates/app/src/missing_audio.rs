//! Fehlende Audiodateien der SUPER SCENES: regelmäßige Prüfung, Fenster zum
//! neu Verknüpfen (einzeln oder per Ordnersuche). Beim Verknüpfen ändert sich
//! nur der Pfad – Schnitt, Versatz, Länge und Korrekturen bleiben.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use dmxcraft_core::timeline::ClipContent;
use dmxcraft_core::{SceneId, Show};
use eframe::egui;

use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::Dialog;
use crate::views::ViewCtx;

/// Prüfabstand (Dateien können während des Betriebs verschwinden).
const CHECK_EVERY: Duration = Duration::from_secs(3);

/// Ergebnis der letzten Prüfung.
#[derive(Default)]
pub struct MissingAudio {
    /// Fehlende Pfade mit den SUPER SCENES, die sie verwenden.
    pub files: BTreeMap<String, Vec<SceneId>>,
    checked: Option<Instant>,
    /// Fenster offen.
    pub open: bool,
}

impl MissingAudio {
    /// Pfade prüfen (höchstens alle paar Sekunden, `force` sofort).
    pub fn check(&mut self, show: &Show, force: bool) {
        if !force && self.checked.is_some_and(|c| c.elapsed() < CHECK_EVERY) {
            return;
        }
        self.checked = Some(Instant::now());
        self.files = scan(show);
        if self.files.is_empty() {
            self.open = false;
        }
    }

    pub fn any(&self) -> bool {
        !self.files.is_empty()
    }

    /// Betroffene SUPER SCENES für die Szenenkarten bereitlegen (Kontext).
    pub fn publish(&self, ctx: &egui::Context) {
        let ids: Vec<SceneId> = self.files.values().flatten().copied().collect();
        ctx.data_mut(|d| d.insert_temp(missing_id(), ids));
    }
}

fn missing_id() -> egui::Id {
    egui::Id::unique("missing_audio_scenes")
}

/// Fehlt einer SUPER SCENE eine Audiodatei? (für die Szenenkarten)
pub fn scene_missing(ctx: &egui::Context, id: SceneId) -> bool {
    ctx.data(|d| {
        d.get_temp::<Vec<SceneId>>(missing_id())
            .is_some_and(|v| v.contains(&id))
    })
}

/// Alle Audiopfade der Show, die es nicht (mehr) gibt.
fn scan(show: &Show) -> BTreeMap<String, Vec<SceneId>> {
    let mut out: BTreeMap<String, Vec<SceneId>> = BTreeMap::new();
    let mut exists: HashMap<&str, bool> = HashMap::new();
    for sc in &show.scenes {
        let Some(tl) = sc.timeline() else {
            continue;
        };
        for c in tl.tracks.iter().flat_map(|t| &t.clips) {
            if let ClipContent::Audio { path, .. } | ClipContent::Video { path, .. } = &c.content {
                let ok = *exists
                    .entry(path.as_str())
                    .or_insert_with(|| Path::new(path).is_file());
                if !ok {
                    let v = out.entry(path.clone()).or_default();
                    if !v.contains(&sc.id) {
                        v.push(sc.id);
                    }
                }
            }
        }
    }
    // Reiter SOUND (ohne Szene)
    for path in show.sound.paths() {
        if !*exists
            .entry(path)
            .or_insert_with(|| Path::new(path).is_file())
        {
            out.entry(path.to_string()).or_default();
        }
    }
    out
}

/// Alle Blöcke mit Pfad `old` auf `new` umstellen; Anzahl der Blöcke.
pub fn relink(show: &mut Show, old: &str, new: &str) -> usize {
    let mut n = 0;
    for sc in &mut show.scenes {
        let Some(tl) = sc.timeline_mut() else {
            continue;
        };
        for c in tl.tracks.iter_mut().flat_map(|t| &mut t.clips) {
            if let ClipContent::Audio { path, .. } | ClipContent::Video { path, .. } =
                &mut c.content
            {
                if path == old {
                    *path = new.to_string();
                    n += 1;
                }
            }
        }
    }
    n += show.sound.relink(old, new);
    n
}

/// Dateien eines Ordners (rekursiv, begrenzt) nach Dateiname (klein).
fn index_folder(dir: &Path) -> HashMap<String, PathBuf> {
    let mut out = HashMap::new();
    let mut stack = vec![(dir.to_path_buf(), 0u8)];
    let mut seen = 0usize;
    while let Some((d, depth)) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&d) else {
            continue;
        };
        for entry in rd.flatten() {
            seen += 1;
            if seen > 200_000 {
                return out;
            }
            let p = entry.path();
            if p.is_dir() {
                if depth < 8 {
                    stack.push((p, depth + 1));
                }
            } else if let Some(name) = p.file_name() {
                out.entry(name.to_string_lossy().to_lowercase())
                    .or_insert(p);
            }
        }
    }
    out
}

/// Dateiname eines (auch fremden, z. B. Windows-)Pfads.
fn file_name(path: &str) -> String {
    path.rsplit(['/', '\\']).next().unwrap_or(path).to_string()
}

const AUDIO_EXT: [&str; 11] = [
    "mp3", "wav", "flac", "ogg", "m4a", "aac", "mp4", "mov", "m4v", "mkv", "webm",
];

/// Fenster „Fehlende Audiodateien“.
pub fn window(ctx: &egui::Context, cx: &mut ViewCtx<'_>) {
    if !cx.state.missing.open {
        return;
    }
    let mut open = true;
    let files: Vec<(String, Vec<SceneId>)> = cx
        .state
        .missing
        .files
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let mut relinks: Vec<(String, String)> = Vec::new();
    let mut search = false;
    Dialog::new(t("Fehlende Audiodateien"))
        .id("missing_audio")
        .resizable([720.0, 420.0])
        .cancel(t("Schließen"))
        .show(ctx, &mut open, |ui, _| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(if files.len() == 1 {
                        t("1 Datei nicht gefunden. Schnitt und Versatz bleiben beim neuen Verknüpfen erhalten.").to_string()
                    } else {
                        crate::tr!(
                            "{n} Dateien nicht gefunden. Schnitt und Versatz bleiben beim neuen Verknüpfen erhalten.",
                            n = files.len()
                        )
                    })
                    .color(theme::text_dim()),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(t("In Ordner suchen …")).clicked() {
                        search = true;
                    }
                });
            });
            ui.add_space(theme::SP_S);
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (path, scenes) in &files {
                        egui::Frame::new()
                            .fill(theme::raised())
                            .corner_radius(theme::R_S)
                            .inner_margin(egui::Margin::same(8))
                            .show(ui, |ui| {
                                ui.set_min_width(ui.available_width());
                                ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new(file_name(path))
                                                .color(theme::DANGER)
                                                .strong(),
                                        );
                                        ui.add(
                                            egui::Label::new(
                                                egui::RichText::new(path)
                                                    .font(theme::caption())
                                                    .color(theme::text_faint()),
                                            )
                                            .truncate(),
                                        );
                                        let names: Vec<String> = scenes
                                            .iter()
                                            .filter_map(|id| cx.engine.show().scene(*id))
                                            .map(|s| s.name.clone())
                                            .collect();
                                        ui.label(
                                            egui::RichText::new(crate::tr!(
                                                "Verwendet in: {scenes}",
                                                scenes = names.join(", ")
                                            ))
                                            .font(theme::caption())
                                            .color(theme::text_dim()),
                                        );
                                    });
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            if ui.button(t("Datei wählen …")).clicked() {
                                                if let Some(p) = rfd::FileDialog::new()
                                                    .add_filter(t("Audio / Video"), &AUDIO_EXT)
                                                    .pick_file()
                                                {
                                                    relinks.push((
                                                        path.clone(),
                                                        p.to_string_lossy().into_owned(),
                                                    ));
                                                }
                                            }
                                        },
                                    );
                                });
                            });
                        ui.add_space(theme::SP_XS);
                    }
                });
        });
    if search {
        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
            let index = index_folder(&dir);
            for (path, _) in &files {
                if let Some(p) = index.get(&file_name(path).to_lowercase()) {
                    relinks.push((path.clone(), p.to_string_lossy().into_owned()));
                }
            }
            if relinks.is_empty() {
                cx.state
                    .info(t("Im Ordner keine passenden Dateien gefunden"));
            }
        }
    }
    if !relinks.is_empty() {
        let mut n = 0;
        for (old, new) in &relinks {
            n += relink(cx.engine.show_mut(), old, new);
        }
        cx.state
            .info(crate::tr!("{n} Audio-Blöcke neu verknüpft", n = n));
        cx.state.missing.check(cx.engine.show(), true);
    }
    cx.state.missing.open = open && cx.state.missing.any();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_missing_and_relinks_keeping_cut() {
        let mut show = Show::default();
        let a = show.add_scene("A", None, vec![dmxcraft_core::Step::new(Vec::new())]);
        let id = show.create_super_scene(&[a], "Super").unwrap();
        let tl = show.scene_mut(id).unwrap().timeline_mut().unwrap();
        let cid = tl.add_clip(
            1,
            ClipContent::Audio {
                path: "/nicht/da/song.mp3".into(),
                offset_ms: 1234,
                length_ms: 60_000,
                grid: None,
                segment_grids: Vec::new(),
                triggers: Vec::new(),
            },
            5_000,
            20_000,
        );
        let files = scan(&show);
        assert_eq!(files.get("/nicht/da/song.mp3"), Some(&vec![id]));
        // neue Datei (existiert)
        let new = std::env::temp_dir().join("dmxcraft_relink.mp3");
        std::fs::write(&new, b"x").unwrap();
        let n = relink(&mut show, "/nicht/da/song.mp3", &new.to_string_lossy());
        assert_eq!(n, 1);
        assert!(scan(&show).is_empty());
        let (_, c) = show
            .scene(id)
            .unwrap()
            .timeline()
            .unwrap()
            .clip(cid)
            .unwrap();
        assert_eq!((c.start_ms, c.duration_ms), (5_000, 20_000));
        assert!(matches!(
            c.content,
            ClipContent::Audio {
                offset_ms: 1234,
                ..
            }
        ));
        let _ = std::fs::remove_file(new);
    }

    #[test]
    fn folder_search_matches_file_names() {
        let dir = std::env::temp_dir().join("dmxcraft_search/unter");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("Song.MP3"), b"x").unwrap();
        let index = index_folder(dir.parent().unwrap());
        assert!(index.contains_key(&file_name("C:\\Musik\\song.mp3").to_lowercase()));
        let _ = std::fs::remove_dir_all(dir.parent().unwrap());
    }
}
