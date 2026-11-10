//! SETUP → “Library” sidebar: search fixture types (built-in, in the show,
//! Open Fixture Library) and patch them.

use crate::i18n::t;
use std::collections::BTreeMap;

use dmxcraft_core::library::builtin_fixture_types;
use dmxcraft_core::{Engine, FixtureType};
use dmxcraft_ofl::{convert_fixture, convert_qxf, Library, Source};
use eframe::egui;

use crate::profiles::{self, UserProfile};
use crate::state::{AppState, LibraryDrag};
use crate::ui::theme;
use crate::ui::widgets::text_field;
use crate::views::profile_builder::{template, BuilderRequest};
use crate::views::{View, ViewCtx};

/// At most this many manufacturers are offered expanded for an empty search.
const MAX_FIXTURES_PER_SEARCH: usize = 400;

#[derive(Default)]
pub struct LibraryPanel {
    query: String,
    /// Selected fixture type (not necessarily in the show yet).
    chosen: Option<FixtureType>,
    /// Running system dialog for importing a fixture file.
    import_dialog: Option<std::sync::mpsc::Receiver<Option<std::path::PathBuf>>>,
    /// Action from a context menu (executed after drawing).
    action: Option<LibAction>,
    /// Search result for `query`: groups of the library index with the
    /// matching fixtures, and whether the result was cut off.
    search: Option<(String, Vec<SearchGroup>, bool)>,
    /// Height of a closed group header incl. spacing (measured), for
    /// skipping headers outside the visible area.
    header_h: f32,
}

/// Search hits of one manufacturer: index into `LibIndex::groups`, fixtures.
type SearchGroup = (usize, Vec<usize>);

/// Library grouped by manufacturer, built once (the library is embedded).
struct LibIndex {
    /// Manufacturer (upper case) and the indices of its fixtures.
    groups: Vec<(String, Vec<usize>)>,
    /// Search text per fixture (lower case): manufacturer, name, categories.
    hay: Vec<String>,
}

fn lib_index() -> &'static LibIndex {
    static INDEX: std::sync::OnceLock<LibIndex> = std::sync::OnceLock::new();
    INDEX.get_or_init(|| {
        let lib = Library::embedded();
        let mut by_man: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, f) in lib.fixtures.iter().enumerate() {
            by_man
                .entry(f.manufacturer.to_uppercase())
                .or_default()
                .push(i);
        }
        LibIndex {
            groups: by_man.into_iter().collect(),
            hay: lib
                .fixtures
                .iter()
                .map(|f| {
                    format!("{} {} {}", f.manufacturer, f.name, f.categories.join(" "))
                        .to_lowercase()
                })
                .collect(),
        }
    })
}

/// Built-in fixture types (built once).
fn builtin_types() -> &'static [FixtureType] {
    static TYPES: std::sync::OnceLock<Vec<FixtureType>> = std::sync::OnceLock::new();
    TYPES.get_or_init(builtin_fixture_types)
}

enum LibAction {
    Builder(BuilderRequest),
    Delete(String),
}

impl View for LibraryPanel {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        drag_ghost(ui.ctx());
        // Result of the system dialog
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
                if ui
                    .add(crate::ui::widgets::IconButton::new(
                        crate::ui::widgets::Icon::Builder,
                    ))
                    .on_hover_text(t("Create a new fixture in the Profile Builder"))
                    .clicked()
                {
                    self.action = Some(LibAction::Builder(BuilderRequest::New));
                }
                let r = ui
                    .add(
                        crate::ui::widgets::IconButton::new(crate::ui::widgets::Icon::Import)
                            .enabled(!busy),
                    )
                    .on_hover_text(t(
                        "Import your own fixture file (Open Fixture Library .json or QLC+ .qxf)",
                    ));
                ui.add(
                    text_field(&mut self.query)
                        .hint_text(t("Search fixtures (manufacturer, model, category)"))
                        .desired_width(f32::INFINITY),
                );
                r
            });
            if r.inner.clicked() {
                let (tx, rx) = std::sync::mpsc::channel();
                let ctx = ui.ctx().clone();
                std::thread::spawn(move || {
                    let path = rfd::FileDialog::new()
                        .set_title(t("Import fixture file"))
                        .add_filter(t("Fixture files"), &["json", "qxf"])
                        .pick_file();
                    let _ = tx.send(path);
                    ctx.request_repaint();
                });
                self.import_dialog = Some(rx);
            }
        });
        ui.add_space(theme::SP_XS);

        // Patch form fixed at the bottom, the list fills the rest
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
                        self.tree(
                            ui,
                            cx.engine,
                            &cx.state.imported_types,
                            &cx.state.user_profiles,
                        );
                    });
            });
        match self.action.take() {
            Some(LibAction::Builder(req)) => cx.state.builder = Some(req),
            Some(LibAction::Delete(key)) => {
                if let Err(e) = profiles::delete(&key) {
                    cx.state.error(e);
                } else if let Some(i) = cx
                    .state
                    .user_profiles
                    .iter()
                    .position(|u| u.profile.key == key)
                {
                    let u = cx.state.user_profiles.remove(i);
                    if self
                        .chosen
                        .as_ref()
                        .is_some_and(|c| u.types.iter().any(|t| t.key == c.key))
                    {
                        self.chosen = None;
                    }
                    cx.state.info(tr!(
                        "“{v}” removed from the library",
                        v = u.profile.display_name()
                    ));
                }
            }
            None => {}
        }
    }
}

impl LibraryPanel {
    fn tree(
        &mut self,
        ui: &mut egui::Ui,
        e: &Engine,
        imported: &[FixtureType],
        user: &[UserProfile],
    ) {
        let searching = !self.query.trim().is_empty();
        let words: Vec<String> = self
            .query
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .collect();
        let matches = |hay: &str| words.iter().all(|w| hay.to_lowercase().contains(w));

        // My fixtures (Profile Builder)
        let own: Vec<&UserProfile> = user
            .iter()
            .filter(|u| !searching || matches(&u.profile.display_name()))
            .collect();
        if !own.is_empty() {
            self.group(ui, t("_My fixtures"), searching, |me, ui| {
                for u in own {
                    me.user_row(ui, u);
                }
            });
        }

        // Built-in types and types already in the show
        let in_show: Vec<&FixtureType> = e
            .show()
            .fixture_types
            .iter()
            .filter(|t| !t.key.starts_with("generic/"))
            .collect();
        for (title, types) in [
            (t("_Generic"), builtin_types().iter().collect::<Vec<_>>()),
            (t("_Imported"), imported.iter().collect()),
            (t("_In this show"), in_show),
        ] {
            let list: Vec<&FixtureType> = types
                .into_iter()
                .filter(|t| !searching || matches(&t.display_name()))
                .collect();
            if list.is_empty() {
                continue;
            }
            self.group(ui, title, searching, |me, ui| {
                for ty in list {
                    let r = me.type_row(ui, &ty.display_name(), ty, None);
                    me.template_menu(&r, &ty.manufacturer, &ty.model, std::slice::from_ref(ty));
                }
            });
        }

        // Open Fixture Library, grouped by manufacturer. The search runs only
        // when the query changes.
        let lib = Library::embedded();
        let index = lib_index();
        if searching {
            let query = self.query.trim().to_string();
            if self.search.as_ref().is_none_or(|(q, _, _)| *q != query) {
                let mut groups = Vec::new();
                let mut count = 0;
                for (g, (_, fixtures)) in index.groups.iter().enumerate() {
                    let hits: Vec<usize> = fixtures
                        .iter()
                        .copied()
                        .filter(|&i| words.iter().all(|w| index.hay[i].contains(w.as_str())))
                        .take(MAX_FIXTURES_PER_SEARCH.saturating_sub(count))
                        .collect();
                    count += hits.len();
                    if !hits.is_empty() {
                        groups.push((g, hits));
                    }
                }
                self.search = Some((query, groups, count >= MAX_FIXTURES_PER_SEARCH));
            }
        } else {
            self.search = None;
        }
        let truncated = self.search.as_ref().is_some_and(|(_, _, cut)| *cut);
        // taken out while drawing (the rows need `&mut self`)
        let search = self.search.take();
        let shown: &[SearchGroup] = search.as_ref().map_or(&[], |(_, g, _)| g);
        let groups: Box<dyn Iterator<Item = (&str, &[usize])>> = if searching {
            Box::new(
                shown
                    .iter()
                    .map(|(g, hits)| (index.groups[*g].0.as_str(), hits.as_slice())),
            )
        } else {
            Box::new(
                index
                    .groups
                    .iter()
                    .map(|(name, all)| (name.as_str(), all.as_slice())),
            )
        };
        for (man, fixtures) in groups {
            self.group(ui, man, searching, |me, ui| {
                for f in fixtures.iter().map(|&i| &lib.fixtures[i]) {
                    // Append the origin so identical fixtures from OFL and QLC+ can be told
                    // apart
                    let tag = (f.source == Source::Qlc).then(|| f.source.label());
                    if f.modes.len() == 1 {
                        let label = tr!("{v} ({n} ch.)", v = f.name, n = f.modes[0].channels.len());
                        let r = me.type_row(ui, &label, &f.modes[0], tag);
                        me.template_menu(&r, &f.manufacturer, &f.name, &f.modes);
                    } else {
                        let id =
                            ui.make_persistent_id((f.source.label(), &f.manufacturer_key, &f.key));
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ui.ctx(),
                            id,
                            false,
                        )
                        .show_header(ui, |ui| {
                            // Dragging the header = first mode
                            let r = ui.add(egui::Label::new(&f.name).sense(egui::Sense::drag()));
                            if let Some(m) = f.modes.first() {
                                if r.drag_started() || r.hovered() {
                                    r.dnd_set_drag_payload(LibraryDrag(m.clone()));
                                }
                            }
                            me.template_menu(&r, &f.manufacturer, &f.name, &f.modes);
                            if let Some(tag) = tag {
                                source_tag(ui, tag);
                            }
                        })
                        .body(|ui| {
                            for m in &f.modes {
                                let label =
                                    tr!("{mode} ({n} ch.)", mode = m.mode, n = m.channels.len());
                                let r = me.type_row(ui, &label, m, None);
                                me.template_menu(&r, &f.manufacturer, &f.name, &f.modes);
                            }
                        });
                    }
                }
            });
        }
        self.search = search;
        if searching && truncated {
            ui.label(
                egui::RichText::new(t("More results – narrow your search"))
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
        // The library has hundreds of manufacturers: closed headers outside the
        // visible area only take up their space.
        let closed = !open
            && !egui::collapsing_header::CollapsingState::load(
                ui.ctx(),
                ui.make_persistent_id(("lib_group", title)),
            )
            .is_some_and(|s| s.is_open());
        if closed && self.header_h > 0.0 {
            let rect = egui::Rect::from_min_size(
                ui.cursor().min,
                egui::vec2(ui.available_width(), self.header_h),
            );
            if !ui.is_rect_visible(rect) {
                ui.add_space(self.header_h);
                return;
            }
        }
        let top = ui.cursor().min.y;
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
        let r = header.show(ui, |ui| body(self, ui));
        if closed && r.body_returned.is_none() {
            self.header_h = ui.cursor().min.y - top;
        }
    }

    /// Context menu “As template …” for fixtures not built by the user.
    fn template_menu(
        &mut self,
        r: &egui::Response,
        manufacturer: &str,
        name: &str,
        modes: &[FixtureType],
    ) {
        use crate::ui::widgets::menu::{self, MenuItem};
        menu::context(r, |ui| {
            menu::begin(ui);
            if menu::item(
                ui,
                MenuItem::new(t("Use as template in the Profile Builder …"))
                    .icon(crate::ui::widgets::Icon::Builder),
            ) {
                let p = template(manufacturer, name, modes);
                self.action = Some(LibAction::Builder(BuilderRequest::Template(Box::new(p))));
            }
        });
    }

    /// Own fixture: like a library fixture, context menu for editing.
    fn user_row(&mut self, ui: &mut egui::Ui, u: &UserProfile) {
        let name = u.profile.display_name();
        let menu = |me: &mut Self, r: &egui::Response| {
            use crate::ui::widgets::menu::{self, MenuItem};
            use crate::ui::widgets::Icon;
            menu::context(r, |ui| {
                menu::begin(ui);
                menu::title(ui, &name);
                if menu::item(ui, MenuItem::new(t("Edit …")).icon(Icon::Edit)) {
                    me.action = Some(LibAction::Builder(BuilderRequest::Edit(Box::new(
                        u.profile.clone(),
                    ))));
                }
                if menu::item(ui, MenuItem::new(t("Duplicate …")).icon(Icon::Copy)) {
                    let mut p = u.profile.clone();
                    p.key.clear();
                    p.created.clear();
                    p.name = tr!("{v} (copy)", v = p.name);
                    me.action = Some(LibAction::Builder(BuilderRequest::Template(Box::new(p))));
                }
                menu::separator(ui);
                if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash).danger()) {
                    me.action = Some(LibAction::Delete(u.profile.key.clone()));
                }
            });
        };
        match u.types.as_slice() {
            [] => {
                let r = ui.add(
                    egui::Label::new(egui::RichText::new(&name).color(theme::DANGER))
                        .sense(egui::Sense::click()),
                );
                let r = r.on_hover_text(t(
                    "Profile has errors – open it in the Profile Builder and check",
                ));
                menu(self, &r);
            }
            [ty] => {
                let label = tr!("{v} ({n} ch.)", v = name, n = ty.channels.len());
                let r = self.type_row(ui, &label, ty, None);
                menu(self, &r);
            }
            types => {
                let id = ui.make_persistent_id(("user_fixture", &u.profile.key));
                egui::collapsing_header::CollapsingState::load_with_default_open(
                    ui.ctx(),
                    id,
                    false,
                )
                .show_header(ui, |ui| {
                    let r = ui.add(egui::Label::new(&name).sense(egui::Sense::click_and_drag()));
                    r.dnd_set_drag_payload(LibraryDrag(types[0].clone()));
                    menu(self, &r);
                })
                .body(|ui| {
                    for ty in types {
                        let label = tr!("{mode} ({n} ch.)", mode = ty.mode, n = ty.channels.len());
                        let r = self.type_row(ui, &label, ty, None);
                        menu(self, &r);
                    }
                });
            }
        }
    }

    /// One row per fixture type; `tag` = origin (e.g. “QLC+”), small after it.
    fn type_row(
        &mut self,
        ui: &mut egui::Ui,
        label: &str,
        ty: &FixtureType,
        tag: Option<&str>,
    ) -> egui::Response {
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
        // channel list only built when hovered
        let r = r.on_hover_ui(|ui| {
            ui.label(
                ty.channels
                    .iter()
                    .enumerate()
                    .map(|(i, c)| format!("{:>3}  {}", i + 1, c.name))
                    .collect::<Vec<_>>()
                    .join("\n"),
            );
        });
        if r.clicked() {
            self.chosen = Some(ty.clone());
        }
        // drag directly into the patch
        let r = r.interact(egui::Sense::click_and_drag());
        if r.drag_started() {
            r.dnd_set_drag_payload(LibraryDrag(ty.clone()));
        }
        r
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
                ui.label(t("Universe"));
                ui.add(egui::DragValue::new(&mut st.patch.universe).range(1..=universes));
                ui.end_row();
                ui.label(t("Start address"));
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut st.patch.address).range(1..=512));
                    if ui
                        .small_button(t("free"))
                        .on_hover_text(t("First matching free address"))
                        .clicked()
                    {
                        match e.show().next_free_address(st.patch.universe - 1, channels) {
                            Some(a) => st.patch.address = a,
                            None => st.error(t("No more room in this universe")),
                        }
                    }
                });
                ui.end_row();
                ui.label(t("Count"));
                ui.add(egui::DragValue::new(&mut st.patch.count).range(1..=128));
                ui.end_row();
            });
        let total = u32::from(channels) * u32::from(st.patch.count);
        ui.label(
            egui::RichText::new(tr!(
                "{channels} channels per fixture · {total} in total",
                total = total,
                channels = channels
            ))
            .color(theme::text_faint()),
        );
        let b = egui::Button::new(egui::RichText::new(t("Patch")).strong())
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
            .map_or("fixture".into(), |s| s.to_string_lossy().to_string());
        let man = path
            .parent()
            .and_then(|p| p.file_name())
            .map_or("custom".into(), |s| s.to_string_lossy().to_string());
        // QLC+ files (.qxf) are XML, everything else is read as OFL JSON
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
                st.info(tr!("“{v}” loaded – choose a mode and patch", v = f.name));
                self.query = f.name.clone();
                self.chosen = f.modes.first().cloned();
                st.imported_types.extend(f.modes);
            }
            Err(err) => st.error(err.to_string()),
        }
    }
}

/// Small gray hint at the origin of a fixture.
fn source_tag(ui: &mut egui::Ui, tag: &str) {
    ui.label(egui::RichText::new(tag).small().color(theme::text_faint()));
}

/// Adds `count` fixtures of the type from the start address.
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
                st.error(tr!("Fixture {i_nr}: {err}", err = err, i_nr = i + 1));
                break;
            }
        }
    }
    if !added.is_empty() {
        st.patch.address = address.min(512);
        st.selected = added.iter().copied().collect();
        if added.len() == usize::from(count) {
            st.info(tr!("{n} fixture(s) patched", n = added.len()));
        }
    }
}

/// Label at the mouse pointer while a fixture is dragged from the library.
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
                            "{v} ({n} ch.)",
                            v = d.0.model,
                            n = d.0.channel_count()
                        ))
                        .wrap_mode(egui::TextWrapMode::Extend),
                    );
                });
        });
}
