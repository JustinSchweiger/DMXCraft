//! EINRICHTEN → Seitenleiste „Bibliothek“: Gerätetypen suchen (eingebaut,
//! in der Show, Open Fixture Library) und patchen.

use crate::i18n::t;
use std::collections::BTreeMap;

use dmxcraft_core::library::builtin_fixture_types;
use dmxcraft_core::{Engine, FixtureType};
use dmxcraft_ofl::{convert_fixture, convert_qxf, Library, LibraryFixture, Source};
use eframe::egui;

use crate::state::{AppState, LibraryDrag};
use crate::ui::theme;
use crate::ui::widgets::text_field;
use crate::views::{View, ViewCtx};

/// Höchstens so viele Hersteller werden bei leerer Suche aufgeklappt angeboten.
const MAX_FIXTURES_PER_SEARCH: usize = 400;

#[derive(Default)]
pub struct LibraryPanel {
    query: String,
    /// Gewählter Gerätetyp (noch nicht unbedingt in der Show).
    chosen: Option<FixtureType>,
    /// Laufender Systemdialog zum Importieren einer Gerätedatei.
    import_dialog: Option<std::sync::mpsc::Receiver<Option<std::path::PathBuf>>>,
}

impl View for LibraryPanel {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        drag_ghost(ui.ctx());
        // Ergebnis des Systemdialogs
        if let Some(rx) = &self.import_dialog {
            if let Ok(result) = rx.try_recv() {
                self.import_dialog = None;
                if let Some(path) = result {
                    self.import(cx.state, &path);
                }
            }
        }
        ui.horizontal(|ui| {
            let busy = self.import_dialog.is_some();
            let r = ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let r = ui
                    .add(crate::ui::widgets::IconButton::new(crate::ui::widgets::Icon::Import).enabled(!busy))
                    .on_hover_text(t(
                        "Eigene Gerätedatei importieren (Open Fixture Library .json oder QLC+ .qxf)",
                    ));
                ui.add(
                    text_field(&mut self.query)
                        .hint_text(t("Geräte suchen (Hersteller, Modell, Kategorie)"))
                        .desired_width(f32::INFINITY),
                );
                r
            });
            if r.inner.clicked() {
                let (tx, rx) = std::sync::mpsc::channel();
                let ctx = ui.ctx().clone();
                std::thread::spawn(move || {
                    let path = rfd::FileDialog::new()
                        .set_title(t("Gerätedatei importieren"))
                        .add_filter(t("Gerätedateien"), &["json", "qxf"])
                        .pick_file();
                    let _ = tx.send(path);
                    ctx.request_repaint();
                });
                self.import_dialog = Some(rx);
            }
        });
        ui.add_space(theme::SP_XS);

        // Patch-Formular unten fest, Liste füllt den Rest
        egui::Panel::bottom("library_patch_form")
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(0, 6)))
            .show(ui, |ui| self.patch_form(ui, cx.engine, cx.state));

        egui::CentralPanel::default_margins()
            .frame(
                egui::Frame::new()
                    .fill(theme::bg())
                    .inner_margin(egui::Margin::same(4)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        self.tree(ui, cx.engine, &cx.state.imported_types);
                    });
            });
    }
}

impl LibraryPanel {
    fn tree(&mut self, ui: &mut egui::Ui, e: &Engine, imported: &[FixtureType]) {
        let searching = !self.query.trim().is_empty();
        let words: Vec<String> = self
            .query
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();
        let matches = |hay: &str| words.iter().all(|w| hay.to_lowercase().contains(w));

        // Eingebaute und bereits in der Show vorhandene Typen
        let builtin = builtin_fixture_types();
        let in_show: Vec<FixtureType> = e
            .show()
            .fixture_types
            .iter()
            .filter(|t| !t.key.starts_with("generic/"))
            .cloned()
            .collect();
        for (title, types) in [
            (t("_Generisch"), builtin),
            (t("_Importiert"), imported.to_vec()),
            (t("_In dieser Show"), in_show),
        ] {
            let list: Vec<FixtureType> = types
                .into_iter()
                .filter(|t| !searching || matches(&t.display_name()))
                .collect();
            if list.is_empty() {
                continue;
            }
            self.group(ui, title, searching, |me, ui| {
                for t in list {
                    me.type_row(ui, &t.display_name(), &t, None);
                }
            });
        }

        // Open Fixture Library, nach Hersteller gruppiert
        let lib = Library::embedded();
        let mut by_man: BTreeMap<String, Vec<&LibraryFixture>> = BTreeMap::new();
        let mut count = 0;
        for f in &lib.fixtures {
            if searching {
                let hay = format!("{} {} {}", f.manufacturer, f.name, f.categories.join(" "));
                if !matches(&hay) || count >= MAX_FIXTURES_PER_SEARCH {
                    continue;
                }
                count += 1;
            }
            by_man
                .entry(f.manufacturer.to_uppercase())
                .or_default()
                .push(f);
        }
        for (man, fixtures) in by_man {
            self.group(ui, &man, searching, |me, ui| {
                for f in fixtures {
                    // Herkunft anhängen, damit gleiche Geräte aus OFL und QLC+
                    // unterscheidbar sind
                    let tag = (f.source == Source::Qlc).then(|| f.source.label());
                    if f.modes.len() == 1 {
                        let label =
                            tr!("{v} ({n} Kan.)", v = f.name, n = f.modes[0].channels.len());
                        me.type_row(ui, &label, &f.modes[0], tag);
                    } else {
                        let id =
                            ui.make_persistent_id((f.source.label(), &f.manufacturer_key, &f.key));
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ui.ctx(),
                            id,
                            false,
                        )
                        .show_header(ui, |ui| {
                            // Kopfzeile ziehen = erster Modus
                            let r = ui.add(egui::Label::new(&f.name).sense(egui::Sense::drag()));
                            if let Some(m) = f.modes.first() {
                                r.dnd_set_drag_payload(LibraryDrag(m.clone()));
                            }
                            if let Some(tag) = tag {
                                source_tag(ui, tag);
                            }
                        })
                        .body(|ui| {
                            for m in &f.modes {
                                let label =
                                    tr!("{mode} ({n} Kan.)", mode = m.mode, n = m.channels.len());
                                me.type_row(ui, &label, m, None);
                            }
                        });
                    }
                }
            });
        }
        if searching && count >= MAX_FIXTURES_PER_SEARCH {
            ui.label(
                egui::RichText::new(t("Weitere Treffer – Suche eingrenzen"))
                    .color(theme::text_faint()),
            );
        }
    }

    fn group(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        open: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) {
        let header = egui::CollapsingHeader::new(
            egui::RichText::new(title)
                .strong()
                .size(theme::FS_CAPTION + 1.0),
        )
        .id_salt(("lib_group", title));
        let header = if open {
            header.open(Some(true))
        } else {
            header
        };
        header.show(ui, |ui| body(self, ui));
    }

    /// Eine Zeile pro Gerätetyp; `tag` = Herkunft (z. B. „QLC+“), klein dahinter.
    fn type_row(&mut self, ui: &mut egui::Ui, label: &str, ty: &FixtureType, tag: Option<&str>) {
        let selected = self.chosen.as_ref().is_some_and(|c| c.key == ty.key);
        let r = ui
            .horizontal(|ui| {
                let r = ui.selectable_label(selected, label);
                if let Some(tag) = tag {
                    source_tag(ui, tag);
                }
                r
            })
            .inner;
        let r = r.on_hover_text(
            ty.channels
                .iter()
                .enumerate()
                .map(|(i, c)| format!("{:>3}  {}", i + 1, c.name))
                .collect::<Vec<_>>()
                .join("\n"),
        );
        if r.clicked() {
            self.chosen = Some(ty.clone());
        }
        // direkt ins Patch ziehen
        let r = r.interact(egui::Sense::click_and_drag());
        r.dnd_set_drag_payload(LibraryDrag(ty.clone()));
    }

    fn patch_form(&mut self, ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
        let Some(ty) = self.chosen.clone() else {
            return;
        };
        ui.label(egui::RichText::new(ty.display_name()).strong());
        let universes = e.show().universes;
        let channels = ty.channel_count();
        egui::Grid::new("lib_patch_grid")
            .num_columns(2)
            .spacing([8.0, 4.0])
            .show(ui, |ui| {
                ui.label(t("Name"));
                if st.patch.name.is_empty() {
                    st.patch.name = ty.model.clone();
                }
                ui.text_edit_singleline(&mut st.patch.name);
                ui.end_row();
                ui.label(t("Universum"));
                ui.add(egui::DragValue::new(&mut st.patch.universe).range(1..=universes));
                ui.end_row();
                ui.label(t("Startadresse"));
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut st.patch.address).range(1..=512));
                    if ui
                        .small_button("frei")
                        .on_hover_text(t("Erste passende freie Adresse"))
                        .clicked()
                    {
                        match e.show().next_free_address(st.patch.universe - 1, channels) {
                            Some(a) => st.patch.address = a,
                            None => st.error(t("Kein Platz mehr in diesem Universum")),
                        }
                    }
                });
                ui.end_row();
                ui.label(t("Anzahl"));
                ui.add(egui::DragValue::new(&mut st.patch.count).range(1..=128));
                ui.end_row();
            });
        let total = u32::from(channels) * u32::from(st.patch.count);
        ui.label(
            egui::RichText::new(tr!(
                "{channels} Kanäle je Gerät · zusammen {total}",
                total = total,
                channels = channels
            ))
            .color(theme::text_faint()),
        );
        let b = egui::Button::new(egui::RichText::new(t("Patchen")).strong())
            .fill(theme::accent_dim())
            .min_size(egui::vec2(ui.available_width(), 30.0));
        if ui.add(b).clicked() {
            patch(e, st, &ty);
        }
    }

    fn import(&mut self, st: &mut AppState, path: &std::path::Path) {
        let json = match std::fs::read_to_string(path) {
            Ok(j) => j,
            Err(err) => {
                st.error(format!("{}: {err}", path.display()));
                return;
            }
        };
        let key = path
            .file_stem()
            .map_or("geraet".into(), |s| s.to_string_lossy().to_string());
        let man = path
            .parent()
            .and_then(|p| p.file_name())
            .map_or("eigene".into(), |s| s.to_string_lossy().to_string());
        // QLC+-Dateien (.qxf) sind XML, alles andere wird als OFL-JSON gelesen
        let is_qxf = path
            .extension()
            .is_some_and(|x| x.eq_ignore_ascii_case("qxf"));
        let converted = if is_qxf {
            convert_qxf(&json)
        } else {
            convert_fixture(&man, &man, &key, &json)
        };
        match converted {
            Ok(f) => {
                st.info(tr!("„{v}“ geladen – Modus wählen und patchen", v = f.name));
                self.query = f.name.clone();
                self.chosen = f.modes.first().cloned();
                st.imported_types.extend(f.modes);
            }
            Err(err) => st.error(err.to_string()),
        }
    }
}

/// Kleiner grauer Hinweis auf die Herkunft eines Geräts.
fn source_tag(ui: &mut egui::Ui, tag: &str) {
    ui.label(egui::RichText::new(tag).small().color(theme::text_faint()));
}

/// Fügt `count` Geräte des Typs ab der Startadresse hinzu.
fn patch(e: &mut Engine, st: &mut AppState, ty: &FixtureType) {
    e.show_mut().upsert_fixture_type(ty.clone());
    let universe = st.patch.universe.saturating_sub(1);
    let count = st.patch.count.max(1);
    let mut address = st.patch.address;
    let mut added = Vec::new();
    for i in 0..count {
        let name = if count == 1 {
            st.patch.name.clone()
        } else {
            format!("{} {}", st.patch.name, i + 1)
        };
        match e.show_mut().add_fixture(name, &ty.key, universe, address) {
            Ok(id) => {
                added.push(id);
                address = address.saturating_add(ty.channel_count());
            }
            Err(err) => {
                st.error(tr!("Gerät {i_nr}: {err}", err = err, i_nr = i + 1));
                break;
            }
        }
    }
    if !added.is_empty() {
        st.patch.address = address.min(512);
        st.selected = added.iter().copied().collect();
        if added.len() == usize::from(count) {
            st.info(tr!("{n} Gerät(e) gepatcht", n = added.len()));
        }
    }
}

/// Beschriftung am Mauszeiger, solange ein Gerät aus der Bibliothek gezogen wird.
fn drag_ghost(ctx: &egui::Context) {
    let Some(d) = egui::DragAndDrop::payload::<LibraryDrag>(ctx) else {
        return;
    };
    let Some(pos) = ctx.pointer_interact_pos() else {
        return;
    };
    ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
    egui::Area::new(egui::Id::unique("library_drag_ghost"))
        .order(egui::Order::Tooltip)
        .fixed_pos(pos + egui::vec2(14.0, 6.0))
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .fill(theme::raised())
                .stroke(egui::Stroke::new(1.0, theme::accent()))
                .corner_radius(theme::R_S)
                .inner_margin(egui::Margin::symmetric(10, 6))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(tr!(
                            "{v} ({n} Kan.)",
                            v = d.0.model,
                            n = d.0.channel_count()
                        ))
                        .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
}
