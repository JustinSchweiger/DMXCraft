//! Fenster „Gerätebibliothek“: Open Fixture Library durchsuchen und
//! Gerätetypen in die Show übernehmen.

use dmxcraft_core::Engine;
use dmxcraft_ofl::{convert_fixture, Library, LibraryFixture};
use eframe::egui;

use crate::state::AppState as UiState;

const MAX_RESULTS: usize = 300;

/// Fenster „Gerätebibliothek“. Geöffnet wird es über `AppState::library_open`.
#[derive(Default)]
pub struct LibraryWindow {
    query: String,
    /// (Herstellerschlüssel, Geräteschlüssel)
    selected: Option<(String, String)>,
    import_path: String,
}

impl LibraryWindow {
    pub fn show(&mut self, ctx: &egui::Context, e: &mut Engine, st: &mut UiState) {
        if !st.library_open {
            return;
        }
        let mut open = true;
        egui::Window::new("Gerätebibliothek (Open Fixture Library)")
            .open(&mut open)
            .default_size([760.0, 520.0])
            .show(ctx, |ui| self.contents(ui, e, st));
        st.library_open = open;
    }

    fn contents(&mut self, ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
        let lib = Library::embedded();
        ui.horizontal(|ui| {
            ui.label("Suche");
            ui.add(
                egui::TextEdit::singleline(&mut self.query)
                    .hint_text("Hersteller, Modell oder Kategorie, z. B. „cameo par“")
                    .desired_width(360.0),
            );
            ui.weak(format!(
                "{} Geräte · OFL-Stand {}",
                lib.fixtures.len(),
                lib.source_date
            ));
        });
        ui.separator();

        let results = lib.search(&self.query, MAX_RESULTS);
        let selected: Option<&LibraryFixture> = self.selected.as_ref().and_then(|(m, k)| {
            lib.fixtures
                .iter()
                .find(|f| &f.manufacturer_key == m && &f.key == k)
        });

        ui.columns(2, |cols| {
            egui::ScrollArea::vertical()
                .id_salt("lib_results")
                .auto_shrink([false, false])
                .max_height(380.0)
                .show(&mut cols[0], |ui| {
                    if results.is_empty() {
                        ui.weak("Keine Treffer");
                    }
                    for f in &results {
                        let is_sel = selected.is_some_and(|s| std::ptr::eq(s, *f));
                        let label = format!("{} – {}", f.manufacturer, f.name);
                        if ui.selectable_label(is_sel, label).clicked() {
                            self.selected = Some((f.manufacturer_key.clone(), f.key.clone()));
                        }
                    }
                    if results.len() == MAX_RESULTS {
                        ui.weak("… Suche weiter eingrenzen");
                    }
                });

            let ui = &mut cols[1];
            match selected {
                None => {
                    ui.weak("Links ein Gerät auswählen.");
                }
                Some(f) => {
                    ui.heading(&f.name);
                    ui.label(&f.manufacturer);
                    if !f.categories.is_empty() {
                        ui.weak(f.categories.join(", "));
                    }
                    ui.add_space(6.0);
                    ui.strong("Modus übernehmen:");
                    egui::ScrollArea::vertical()
                        .id_salt("lib_modes")
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for mode in &f.modes {
                                ui.horizontal(|ui| {
                                    if ui.button("Übernehmen").clicked() {
                                        take_type(e, st, mode.clone(), &f.name);
                                    }
                                    ui.label(format!(
                                        "{} ({} Kanäle)",
                                        mode.mode,
                                        mode.channels.len()
                                    ))
                                    .on_hover_text(
                                        mode.channels
                                            .iter()
                                            .enumerate()
                                            .map(|(i, c)| format!("{:>3}  {}", i + 1, c.name))
                                            .collect::<Vec<_>>()
                                            .join("\n"),
                                    );
                                });
                            }
                        });
                }
            }
        });

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Eigene OFL-Datei importieren:");
            ui.add(
                egui::TextEdit::singleline(&mut self.import_path)
                    .hint_text("/pfad/zu/hersteller/geraet.json")
                    .desired_width(320.0),
            );
            if ui.button("Importieren").clicked() {
                import_file(e, st, &self.import_path);
            }
        });
        ui.weak("Gerätedaten: Open Fixture Library, MIT-Lizenz – open-fixture-library.org");
    }
}

fn take_type(e: &mut Engine, st: &mut UiState, ty: dmxcraft_core::FixtureType, name: &str) {
    let key = ty.key.clone();
    let channels = ty.channel_count();
    let display = ty.display_name();
    e.show_mut().upsert_fixture_type(ty);
    st.patch.type_key = key;
    st.patch.name = name.to_string();
    let universe = st.patch.universe.saturating_sub(1);
    if let Some(a) = e.show().next_free_address(universe, channels) {
        st.patch.address = a;
    }
    st.info(format!(
        "„{display}“ übernommen – links im Patch hinzufügen"
    ));
}

fn import_file(e: &mut Engine, st: &mut UiState, path: &str) {
    let path = std::path::PathBuf::from(path.trim());
    let json = match std::fs::read_to_string(&path) {
        Ok(j) => j,
        Err(err) => {
            st.error(format!("{}: {err}", path.display()));
            return;
        }
    };
    // OFL-Ablage: <hersteller>/<gerät>.json
    let fixture_key = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "geraet".into());
    let man_key = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "eigene".into());
    match convert_fixture(&man_key, &man_key, &fixture_key, &json) {
        Ok(f) => {
            let n = f.modes.len();
            let first = f.modes[0].clone();
            for m in f.modes.iter().skip(1) {
                e.show_mut().upsert_fixture_type(m.clone());
            }
            take_type(e, st, first, &f.name);
            st.info(format!("„{}“ mit {n} Modus/Modi importiert", f.name));
        }
        Err(err) => st.error(err.to_string()),
    }
}
