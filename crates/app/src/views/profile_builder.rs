//! Tools → Profile Builder: eigene Geräte anlegen (nach dem Vorbild des
//! Fixture-Editors der Open Fixture Library).
//!
//! Reiter: Allgemein, Physisch, Kanäle (mit DMX-Bereichen), Modi, Prüfen.
//! Gespeichert wird im Konfigurationsordner ([`crate::profiles`]); die
//! Geräte erscheinen danach in der Bibliothek unter „Eigene Geräte“.
//! Export als OFL-Datei (zum Teilen oder Einreichen bei der OFL).

use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use dmxcraft_core::{Engine, FixtureType};
use dmxcraft_ofl::profile::{
    CapabilityKind, Problem, ShutterEffect, SlotKind, CATEGORIES, DMX_CONNECTORS, INTENSITY_COLORS,
    PROFILE_EXT,
};
use dmxcraft_ofl::{Capability, Library, Profile, ProfileChannel, ProfileMode};
use eframe::egui::{self, pos2, vec2, Align, Align2, Layout, Rect, Sense, Stroke, Ui};

use crate::i18n::t;
use crate::profiles::{self, UserProfile};
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::dialog::INPUT_H;
use crate::ui::widgets::{form, text_field, Dialog, Icon, IconButton, Segmented};

const SIDEBAR_W: f32 = 170.0;
const TAB_H: f32 = 34.0;
const WIDTH: f32 = 1040.0;
const HEIGHT: f32 = 620.0;
const LIST_W: f32 = 220.0;
const ROW_H: f32 = 28.0;

/// Was der Profile Builder öffnen soll.
#[derive(Clone, Debug)]
pub enum BuilderRequest {
    /// Fenster öffnen, aktuellen Entwurf behalten (Tools-Menü).
    Open,
    /// Leeres Gerät.
    New,
    /// Gespeichertes Profil bearbeiten.
    Edit(Box<Profile>),
    /// Neues Gerät nach Vorlage (Bibliothek, Duplikat).
    Template(Box<Profile>),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Tab {
    #[default]
    General,
    Physical,
    Channels,
    Modes,
    Check,
}

impl Tab {
    const ALL: [Tab; 5] = [
        Tab::General,
        Tab::Physical,
        Tab::Channels,
        Tab::Modes,
        Tab::Check,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::General => t("Allgemein"),
            Tab::Physical => t("Physisch"),
            Tab::Channels => t("Kanäle"),
            Tab::Modes => t("Modi"),
            Tab::Check => t("Prüfen"),
        }
    }

    fn icon(self) -> Icon {
        match self {
            Tab::General => Icon::Info,
            Tab::Physical => Icon::Cube,
            Tab::Channels => Icon::Sliders,
            Tab::Modes => Icon::Stack,
            Tab::Check => Icon::ListChecks,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FileJob {
    Import,
    Export,
}

#[derive(Default)]
pub struct ProfileBuilder {
    open: bool,
    tab: Tab,
    draft: Profile,
    /// Stand beim Laden/Speichern (für „ungespeichert“).
    saved: Profile,
    sel_channel: usize,
    sel_mode: usize,
    /// Wartet auf Bestätigung: ungespeicherte Änderungen verwerfen?
    pending: Option<Profile>,
    file: Option<(FileJob, Receiver<Option<PathBuf>>)>,
    /// Nach dem Anlegen eines Kanals/Modus: Namensfeld fokussieren.
    focus_name: bool,
}

impl ProfileBuilder {
    fn dirty(&self) -> bool {
        self.draft != self.saved
    }

    /// Anfrage aus Menü oder Bibliothek.
    pub fn request(&mut self, req: BuilderRequest) {
        let next = match req {
            BuilderRequest::Open => {
                if !self.open && self.draft == Profile::default() {
                    self.load(new_profile(), true);
                }
                self.open = true;
                return;
            }
            BuilderRequest::New => new_profile(),
            BuilderRequest::Edit(p) | BuilderRequest::Template(p) => *p,
        };
        self.open = true;
        if self.dirty() && self.draft != Profile::default() {
            self.pending = Some(next);
        } else {
            self.load(next, true);
        }
    }

    /// Neuen Entwurf übernehmen. `fresh` = ungespeichert (Vorlage, neu).
    fn load(&mut self, p: Profile, fresh: bool) {
        self.saved = if fresh && p.key.is_empty() {
            new_profile()
        } else {
            p.clone()
        };
        self.draft = p;
        self.tab = Tab::General;
        self.sel_channel = 0;
        self.sel_mode = 0;
    }

    pub fn show(&mut self, ctx: &egui::Context, engine: &mut Engine, st: &mut AppState) {
        if !self.open {
            return;
        }
        if st.locked {
            self.open = false;
            return;
        }
        self.poll_file(st);
        if self.pending.is_some() {
            self.confirm(ctx);
            return;
        }
        let title = if self.dirty() {
            format!("{} *", t("Profile Builder"))
        } else {
            t("Profile Builder").to_string()
        };
        let busy = self.file.is_some();
        let mut open = true;
        let res = Dialog::new(&title)
            .id("profile_builder")
            .width(WIDTH)
            .height(HEIGHT)
            .flush()
            .secondary(t("Importieren …"))
            .secondary(t("Exportieren …"))
            .cancel(t("Schließen"))
            .primary(t("Speichern"))
            .show(ctx, &mut open, |ui, _| {
                let area = Rect::from_min_size(ui.cursor().min, vec2(WIDTH, HEIGHT));
                ui.allocate_rect(area, Sense::hover());
                let side = Rect::from_min_size(area.min, vec2(SIDEBAR_W, HEIGHT));
                let problems = self.draft.problems().len();
                sidebar(ui, side, &mut self.tab, problems);
                let content = Rect::from_min_max(
                    pos2(side.right() + theme::SP_L, area.top() + theme::SP_M),
                    pos2(area.right() - theme::SP_L, area.bottom() - theme::SP_M),
                );
                let mut cui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(content)
                        .layout(Layout::top_down(Align::Min)),
                );
                cui.set_clip_rect(content);
                match self.tab {
                    Tab::Channels => self.channels_tab(&mut cui, content),
                    Tab::Modes => self.modes_tab(&mut cui, content),
                    tab => {
                        egui::ScrollArea::vertical()
                            .id_salt(("builder_scroll", tab as u8))
                            .auto_shrink([false, false])
                            .max_height(content.height())
                            .show(&mut cui, |ui| {
                                ui.set_width(content.width() - theme::SP_S);
                                match tab {
                                    Tab::General => general(ui, &mut self.draft),
                                    Tab::Physical => physical(ui, &mut self.draft),
                                    _ => check(ui, &self.draft),
                                }
                            });
                    }
                }
            });
        if !open {
            self.open = false;
        }
        if res.clicked(0) && !busy {
            self.file = Some((FileJob::Import, pick_import(ctx)));
        }
        if res.clicked(1) && !busy {
            self.file = Some((FileJob::Export, pick_export(ctx, &self.draft)));
        }
        if res.clicked(3) {
            self.save(engine, st);
        }
    }

    /// Rückfrage, bevor ein ungespeicherter Entwurf ersetzt wird.
    fn confirm(&mut self, ctx: &egui::Context) {
        let mut open = true;
        let res = Dialog::new(t("Ungespeicherte Änderungen"))
            .id("profile_builder_confirm")
            .width(380.0)
            .cancel(t("Abbrechen"))
            .primary(t("Verwerfen"))
            .show(ctx, &mut open, |ui, _| {
                ui.label(tr!(
                    "Änderungen an „{v}“ verwerfen?",
                    v = self.draft.display_name()
                ));
            });
        if res.clicked(1) {
            if let Some(p) = self.pending.take() {
                self.load(p, true);
            }
        } else if !open {
            self.pending = None;
        }
    }

    fn save(&mut self, engine: &mut Engine, st: &mut AppState) {
        let problems = self.draft.problems();
        if !problems.is_empty() {
            self.tab = Tab::Check;
            st.error(tr!(
                "Nicht gespeichert: {n} Problem(e), siehe Prüfen",
                n = problems.len()
            ));
            return;
        }
        let today = profiles::today();
        if self.draft.created.is_empty() {
            self.draft.created = today.clone();
        }
        self.draft.modified = today;
        let types = match self.draft.fixture_types() {
            Ok(t) => t,
            Err(e) => {
                st.error(e.to_string());
                return;
            }
        };
        let others: Vec<Profile> = st.user_profiles.iter().map(|u| u.profile.clone()).collect();
        let mut draft = self.draft.clone();
        if let Err(e) = profiles::save(&mut draft, &others) {
            st.error(tr!("Nicht gespeichert: {e}", e = e));
            return;
        }
        // Schlüssel steht erst jetzt fest
        let types = if self.draft.key.is_empty() {
            draft.fixture_types().unwrap_or(types)
        } else {
            types
        };
        self.draft = draft;
        self.saved = self.draft.clone();
        // Bereits gepatchte Geräte dieses Profils in der Show aktualisieren
        let mut updated = 0;
        for ty in &types {
            if engine.show().fixture_type(&ty.key).is_some() {
                engine.show_mut().upsert_fixture_type(ty.clone());
                updated += engine
                    .show()
                    .fixtures
                    .iter()
                    .filter(|f| f.type_key == ty.key)
                    .count();
            }
        }
        let entry = UserProfile {
            profile: self.draft.clone(),
            types,
        };
        match st
            .user_profiles
            .iter_mut()
            .find(|u| u.profile.key == entry.profile.key)
        {
            Some(u) => *u = entry,
            None => st.user_profiles.push(entry),
        }
        profiles::sort_user(&mut st.user_profiles);
        if updated > 0 {
            st.info(tr!(
                "„{v}“ gespeichert – {n} Gerät(e) in der Show aktualisiert",
                v = self.draft.display_name(),
                n = updated
            ));
        } else {
            st.info(tr!(
                "„{v}“ gespeichert – in der Bibliothek unter Eigene Geräte",
                v = self.draft.display_name()
            ));
        }
    }

    fn poll_file(&mut self, st: &mut AppState) {
        let Some((job, rx)) = &self.file else {
            return;
        };
        let Ok(result) = rx.try_recv() else {
            return;
        };
        let job = *job;
        self.file = None;
        let Some(path) = result else {
            return;
        };
        match job {
            FileJob::Export => match std::fs::write(&path, self.draft.to_ofl_string()) {
                Ok(()) => st.info(tr!("Exportiert: {path}", path = path.display())),
                Err(e) => st.error(format!("{}: {e}", path.display())),
            },
            FileJob::Import => match import(&path) {
                Ok(p) => {
                    st.info(tr!(
                        "„{v}“ importiert – noch nicht gespeichert",
                        v = p.display_name()
                    ));
                    if self.dirty() {
                        self.pending = Some(p);
                    } else {
                        self.load(p, true);
                    }
                }
                Err(e) => st.error(format!("{}: {e}", path.display())),
            },
        }
    }

    // ------------------------------------------------------------ Kanäle

    fn channels_tab(&mut self, ui: &mut Ui, rect: Rect) {
        let (list_rect, detail_rect) = split(rect);
        let p = &mut self.draft;
        // Liste links
        let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(list_rect));
        let mut action: Option<ListAction> = None;
        list_header(&mut lui, t("Kanal hinzufügen"), || {
            action = Some(ListAction::Add)
        });
        egui::ScrollArea::vertical()
            .id_salt("builder_channels")
            .auto_shrink([false, false])
            .show(&mut lui, |ui| {
                for (i, c) in p.channels.iter().enumerate() {
                    let kind = t(c.kind().label());
                    let name = if c.name.is_empty() { "–" } else { &c.name };
                    let r = list_row(ui, i == self.sel_channel, name, kind);
                    if r.clicked() {
                        self.sel_channel = i;
                    }
                    crate::ui::widgets::menu::context(&r, |ui| {
                        use crate::ui::widgets::menu::{self, MenuItem};
                        menu::begin(ui);
                        if menu::item(ui, MenuItem::new(t("Duplizieren")).icon(Icon::Copy)) {
                            action = Some(ListAction::Duplicate(i));
                        }
                        menu::separator(ui);
                        if menu::item(ui, MenuItem::new(t("Löschen")).icon(Icon::Trash).danger()) {
                            action = Some(ListAction::Remove(i));
                        }
                    });
                }
            });
        match action {
            Some(ListAction::Add) => {
                let name = unique(&p.available_names(), t("Kanal"));
                p.channels.push(ProfileChannel {
                    name,
                    ..ProfileChannel::default()
                });
                self.sel_channel = p.channels.len() - 1;
                self.focus_name = true;
            }
            Some(ListAction::Duplicate(i)) => {
                let mut c = p.channels[i].clone();
                c.name = unique(&p.available_names(), &c.name);
                p.channels.insert(i + 1, c);
                self.sel_channel = i + 1;
                self.focus_name = true;
            }
            Some(ListAction::Remove(i)) => {
                p.remove_channel(i);
                self.sel_channel = self.sel_channel.min(p.channels.len().saturating_sub(1));
            }
            None => {}
        }
        separator_v(ui, list_rect.right() + theme::SP_S, rect);

        let i = self.sel_channel;
        if i >= self.draft.channels.len() {
            return;
        }
        let mut dui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(detail_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        egui::ScrollArea::vertical()
            .id_salt(("builder_channel_detail", i))
            .auto_shrink([false, false])
            .show(&mut dui, |ui| {
                ui.set_width(detail_rect.width() - theme::SP_S);
                self.channel_detail(ui, i);
            });
    }

    fn channel_detail(&mut self, ui: &mut Ui, i: usize) {
        let focus = std::mem::take(&mut self.focus_name);
        let p = &mut self.draft;
        form(ui, "builder_channel", |f| {
            f.row(t("Name"), |ui| {
                let mut name = p.channels[i].name.clone();
                let r = ui.add(text_field(&mut name).desired_width(f32::INFINITY));
                if focus {
                    r.request_focus();
                    crate::ui::widgets::dialog::select_all(ui.ctx(), r.id, &name);
                }
                if name != p.channels[i].name {
                    p.rename_channel(i, &name);
                }
            });
            f.row(t("Auflösung"), |ui| {
                let mut fine = p.channels[i].fine;
                let items = [(0u8, "8 Bit"), (1, "16 Bit"), (2, "24 Bit")];
                if Segmented::new(&items).show(ui, &mut fine) {
                    p.set_fine(i, fine);
                }
            });
            f.row(t("Standardwert"), |ui| {
                ui.add(egui::DragValue::new(&mut p.channels[i].default).range(0..=255))
                    .on_hover_text(t("Wert, solange keine Szene den Kanal setzt"));
            });
            f.row(t("Erkannt als"), |ui| {
                ui.label(t(p.channels[i].kind().label()));
            });
        });
        ui.add_space(theme::SP_M);
        let ch = &mut p.channels[i];
        ui.horizontal(|ui| {
            ui.label(theme::heading_text(t("Funktionen")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus))
                    .on_hover_text(t("Bereich hinzufügen (teilt den letzten Bereich)"))
                    .clicked()
                {
                    ch.split_last();
                }
            });
        });
        ui.add_space(theme::SP_XS);
        let count = ch.capabilities.len();
        let mut remove = None;
        let name = ch.name.clone();
        for (ci, cap) in ch.capabilities.iter_mut().enumerate() {
            if capability_card(ui, (&name, i, ci), cap, count > 1) {
                remove = Some(ci);
            }
            ui.add_space(theme::SP_XS);
        }
        if let Some(ci) = remove {
            let gone = ch.capabilities.remove(ci);
            // Lücke schließen: vorheriger Bereich übernimmt das Ende
            if ci > 0 {
                ch.capabilities[ci - 1].end = gone.end;
            } else if let Some(first) = ch.capabilities.first_mut() {
                first.start = gone.start;
            }
        }
    }

    // ------------------------------------------------------------ Modi

    fn modes_tab(&mut self, ui: &mut Ui, rect: Rect) {
        let (list_rect, detail_rect) = split(rect);
        let p = &mut self.draft;
        let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(list_rect));
        let mut action: Option<ListAction> = None;
        list_header(&mut lui, t("Modus hinzufügen"), || {
            action = Some(ListAction::Add)
        });
        egui::ScrollArea::vertical()
            .id_salt("builder_modes")
            .auto_shrink([false, false])
            .show(&mut lui, |ui| {
                for (i, m) in p.modes.iter().enumerate() {
                    let n = tr!("{n} Kan.", n = m.channels.len());
                    let name = if m.name.is_empty() { "–" } else { &m.name };
                    let r = list_row(ui, i == self.sel_mode, name, &n);
                    if r.clicked() {
                        self.sel_mode = i;
                    }
                    crate::ui::widgets::menu::context(&r, |ui| {
                        use crate::ui::widgets::menu::{self, MenuItem};
                        menu::begin(ui);
                        if menu::item(ui, MenuItem::new(t("Duplizieren")).icon(Icon::Copy)) {
                            action = Some(ListAction::Duplicate(i));
                        }
                        menu::separator(ui);
                        if menu::item(ui, MenuItem::new(t("Löschen")).icon(Icon::Trash).danger()) {
                            action = Some(ListAction::Remove(i));
                        }
                    });
                }
            });
        match action {
            Some(ListAction::Add) => {
                let id = p.next_mode_id();
                // Neuer Modus enthält alle Kanäle (häufigster Fall)
                let channels = p.available_names();
                let names: Vec<String> = p.modes.iter().map(|m| m.name.clone()).collect();
                p.modes.push(ProfileMode {
                    id,
                    name: unique(&names, &tr!("{n}-Kanal", n = channels.len())),
                    short_name: String::new(),
                    channels,
                });
                self.sel_mode = p.modes.len() - 1;
                self.focus_name = true;
            }
            Some(ListAction::Duplicate(i)) => {
                let mut m = p.modes[i].clone();
                m.id = p.next_mode_id();
                let names: Vec<String> = p.modes.iter().map(|m| m.name.clone()).collect();
                m.name = unique(&names, &m.name);
                p.modes.insert(i + 1, m);
                self.sel_mode = i + 1;
                self.focus_name = true;
            }
            Some(ListAction::Remove(i)) => {
                p.modes.remove(i);
                self.sel_mode = self.sel_mode.min(p.modes.len().saturating_sub(1));
            }
            None => {}
        }
        separator_v(ui, list_rect.right() + theme::SP_S, rect);

        let i = self.sel_mode;
        if i >= self.draft.modes.len() {
            return;
        }
        let focus = std::mem::take(&mut self.focus_name);
        let mut dui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(detail_rect)
                .layout(Layout::top_down(Align::Min)),
        );
        let p = &mut self.draft;
        form(&mut dui, "builder_mode", |f| {
            f.row(t("Name"), |ui| {
                let r = ui.add(text_field(&mut p.modes[i].name).desired_width(f32::INFINITY));
                if focus {
                    r.request_focus();
                    crate::ui::widgets::dialog::select_all(ui.ctx(), r.id, &p.modes[i].name);
                }
            });
            f.row(t("Kurzname"), |ui| {
                ui.add(
                    text_field(&mut p.modes[i].short_name)
                        .hint_text(t("optional, z. B. 9ch"))
                        .desired_width(f32::INFINITY),
                );
            });
        });
        dui.add_space(theme::SP_M);
        // Zwei Spalten: Kanäle im Modus | verfügbare Kanäle
        let top = dui.cursor().top();
        let cols = Rect::from_min_max(pos2(detail_rect.left(), top), detail_rect.max);
        let gap = theme::SP_L;
        let left_w = (cols.width() - gap) * 0.58;
        let left = Rect::from_min_size(cols.min, vec2(left_w, cols.height()));
        let right = Rect::from_min_max(pos2(left.right() + gap, cols.top()), cols.max);

        let available = p.available_names();
        let mode = &mut p.modes[i];
        let mut lui = dui.new_child(egui::UiBuilder::new().max_rect(left));
        lui.horizontal(|ui| {
            ui.set_height(ROW_H);
            ui.label(theme::heading_text(t("Kanalreihenfolge")));
        });
        let mut op: Option<ModeOp> = None;
        egui::ScrollArea::vertical()
            .id_salt(("builder_mode_channels", i))
            .auto_shrink([false, false])
            .show(&mut lui, |ui| {
                for (ci, entry) in mode.channels.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.set_height(ROW_H);
                        ui.add_sized(
                            vec2(32.0, ROW_H),
                            egui::Label::new(
                                egui::RichText::new(format!("{}", ci + 1))
                                    .color(theme::text_faint()),
                            ),
                        );
                        let (text, color) = if entry.is_empty() {
                            (t("(unbenutzt)").to_string(), theme::text_faint())
                        } else if available.contains(entry) {
                            (entry.clone(), theme::text())
                        } else {
                            (entry.clone(), theme::DANGER)
                        };
                        ui.label(egui::RichText::new(text).color(color));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(IconButton::new(Icon::Close).flat().size(22.0))
                                .on_hover_text(t("Aus dem Modus entfernen"))
                                .clicked()
                            {
                                op = Some(ModeOp::Remove(ci));
                            }
                            if ui
                                .add(
                                    IconButton::new(Icon::ChevronDown)
                                        .flat()
                                        .size(22.0)
                                        .enabled(ci + 1 < mode.channels.len()),
                                )
                                .on_hover_text(t("Nach unten"))
                                .clicked()
                            {
                                op = Some(ModeOp::Down(ci));
                            }
                            if ui
                                .add(
                                    IconButton::new(Icon::ChevronUp)
                                        .flat()
                                        .size(22.0)
                                        .enabled(ci > 0),
                                )
                                .on_hover_text(t("Nach oben"))
                                .clicked()
                            {
                                op = Some(ModeOp::Up(ci));
                            }
                        });
                    });
                }
            });
        separator_v(&mut dui, left.right() + gap / 2.0, cols);
        let mut rui = dui.new_child(egui::UiBuilder::new().max_rect(right));
        rui.horizontal(|ui| {
            ui.set_height(ROW_H);
            ui.label(theme::heading_text(t("Verfügbar")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus))
                    .on_hover_text(t("Unbenutzten Kanal anhängen"))
                    .clicked()
                {
                    op = Some(ModeOp::Add(String::new()));
                }
            });
        });
        egui::ScrollArea::vertical()
            .id_salt(("builder_mode_available", i))
            .auto_shrink([false, false])
            .show(&mut rui, |ui| {
                for name in &available {
                    let used = mode.channels.contains(name);
                    let r = ui
                        .horizontal(|ui| {
                            ui.set_height(ROW_H);
                            let color = if used {
                                theme::text_faint()
                            } else {
                                theme::text()
                            };
                            ui.label(egui::RichText::new(name).color(color));
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                ui.add(IconButton::new(Icon::Plus).flat().size(22.0).enabled(!used))
                            })
                            .inner
                        })
                        .inner;
                    if r.on_hover_text(t("Anhängen")).clicked() {
                        op = Some(ModeOp::Add(name.clone()));
                    }
                }
            });
        match op {
            Some(ModeOp::Up(ci)) => mode.channels.swap(ci, ci - 1),
            Some(ModeOp::Down(ci)) => mode.channels.swap(ci, ci + 1),
            Some(ModeOp::Remove(ci)) => {
                mode.channels.remove(ci);
            }
            Some(ModeOp::Add(name)) => mode.channels.push(name),
            None => {}
        }
    }
}

enum ListAction {
    Add,
    Duplicate(usize),
    Remove(usize),
}

enum ModeOp {
    Up(usize),
    Down(usize),
    Remove(usize),
    Add(String),
}

/// Neues, leeres Gerät mit einem Dimmer-Kanal und einem Modus.
fn new_profile() -> Profile {
    let dimmer = ProfileChannel::new("Dimmer", Capability::new(CapabilityKind::Intensity));
    Profile {
        categories: vec!["Dimmer".into()],
        modes: vec![ProfileMode {
            id: 1,
            name: tr!("{n}-Kanal", n = 1),
            short_name: String::new(),
            channels: vec![dimmer.name.clone()],
        }],
        channels: vec![dimmer],
        ..Profile::default()
    }
}

/// Vorlage aus einem Bibliotheksgerät bzw. einer Geräteliste.
pub fn template(manufacturer: &str, name: &str, types: &[FixtureType]) -> Profile {
    let mut p = Profile::from_types(manufacturer, name, types);
    p.name = tr!("{v} (Kopie)", v = name);
    p
}

/// Freien Namen finden: „Kanal“, „Kanal 2“, …
fn unique(taken: &[String], base: &str) -> String {
    if !taken.iter().any(|n| n == base) {
        return base.to_string();
    }
    (2..)
        .map(|i| format!("{base} {i}"))
        .find(|n| !taken.contains(n))
        .unwrap_or_default()
}

fn split(rect: Rect) -> (Rect, Rect) {
    let list = Rect::from_min_size(rect.min, vec2(LIST_W, rect.height()));
    let detail = Rect::from_min_max(pos2(list.right() + theme::SP_L, rect.top()), rect.max);
    (list, detail)
}

fn separator_v(ui: &mut Ui, x: f32, rect: Rect) {
    ui.painter()
        .vline(x, rect.y_range(), Stroke::new(1.0, theme::border()));
}

fn list_header(ui: &mut Ui, add_tip: &str, mut on_add: impl FnMut()) {
    ui.horizontal(|ui| {
        ui.set_height(ROW_H);
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if ui
                .add(IconButton::new(Icon::Plus))
                .on_hover_text(add_tip)
                .clicked()
            {
                on_add();
            }
        });
    });
    ui.add_space(theme::SP_XS);
}

/// Zeile einer Liste: Name links, Zusatz rechts gedämpft.
fn list_row(ui: &mut Ui, selected: bool, name: &str, extra: &str) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), ROW_H), Sense::click());
    let p = ui.painter();
    if selected {
        p.rect_filled(rect, theme::R_S as f32, theme::raised());
        p.rect_filled(
            Rect::from_min_size(rect.min, vec2(3.0, rect.height())),
            0.0,
            theme::accent(),
        );
    } else if resp.hovered() {
        p.rect_filled(rect, theme::R_S as f32, theme::raised_hover());
    }
    let color = if selected {
        theme::text()
    } else {
        theme::text_dim()
    };
    let right = p.text(
        rect.right_center() - vec2(theme::SP_S, 0.0),
        Align2::RIGHT_CENTER,
        extra,
        theme::caption(),
        theme::text_faint(),
    );
    let name_rect = Rect::from_min_max(
        rect.min + vec2(theme::SP_M, 0.0),
        pos2(right.left() - theme::SP_S, rect.bottom()),
    );
    p.with_clip_rect(name_rect).text(
        name_rect.left_center(),
        Align2::LEFT_CENTER,
        name,
        theme::body(),
        color,
    );
    resp
}

/// Reiterleiste links (wie im Einstellungsfenster), Fehlerzahl am Reiter „Prüfen“.
fn sidebar(ui: &mut Ui, rect: Rect, tab: &mut Tab, problems: usize) {
    let p = ui.painter();
    p.rect_filled(rect, 0.0, theme::bg());
    p.vline(
        rect.right() - 0.5,
        rect.y_range(),
        Stroke::new(1.0, theme::border()),
    );
    for (i, tb) in Tab::ALL.into_iter().enumerate() {
        let r = Rect::from_min_size(
            pos2(rect.left(), rect.top() + theme::SP_S + i as f32 * TAB_H),
            vec2(SIDEBAR_W - 1.0, TAB_H),
        );
        let resp = ui.interact(r, ui.scope_id().with(("builder_tab", i)), Sense::click());
        let active = *tab == tb;
        let p = ui.painter();
        if active {
            p.rect_filled(r, 0.0, theme::surface());
            p.rect_filled(
                Rect::from_min_size(r.min, vec2(3.0, r.height())),
                0.0,
                theme::accent(),
            );
        } else if resp.hovered() {
            p.rect_filled(r, 0.0, theme::raised_hover());
        }
        let color = if active {
            theme::text()
        } else {
            theme::text_dim()
        };
        let icon = Rect::from_center_size(pos2(r.left() + 24.0, r.center().y), vec2(18.0, 18.0));
        tb.icon().paint(p, icon, color);
        p.text(
            pos2(r.left() + 44.0, r.center().y),
            Align2::LEFT_CENTER,
            tb.label(),
            theme::body(),
            color,
        );
        if tb == Tab::Check {
            let (text, c) = if problems == 0 {
                (String::new(), theme::OK)
            } else {
                (problems.to_string(), theme::WARN)
            };
            let dot = pos2(r.right() - 20.0, r.center().y);
            if problems == 0 {
                p.circle_filled(dot, 4.0, c);
            } else {
                p.circle_filled(dot, 9.0, c);
                p.text(
                    dot,
                    Align2::CENTER_CENTER,
                    text,
                    theme::caption(),
                    theme::bg(),
                );
            }
        }
        if resp.clicked() {
            *tab = tb;
        }
    }
}

fn section(ui: &mut Ui, title: &str) {
    ui.add_space(theme::SP_XS);
    ui.label(theme::heading_text(title));
    ui.add_space(theme::SP_XS);
}

fn hint(ui: &mut Ui, text: &str) {
    ui.label(egui::RichText::new(text).small().color(theme::text_faint()));
}

fn full_combo(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    text: &str,
    add: impl FnOnce(&mut Ui),
) {
    ui.spacing_mut().interact_size.y = INPUT_H;
    egui::ComboBox::from_id_salt(id)
        .selected_text(text)
        .width(ui.available_width())
        .height(420.0)
        .show_ui(ui, add);
}

// ------------------------------------------------------------ Allgemein

fn general(ui: &mut Ui, p: &mut Profile) {
    section(ui, t("Gerät"));
    form(ui, "builder_general", |f| {
        f.row(t("Hersteller"), |ui| {
            ui.add(
                text_field(&mut p.manufacturer)
                    .hint_text(t("z. B. Cameo"))
                    .desired_width(ui.available_width() - 32.0),
            );
            manufacturer_picker(ui, &mut p.manufacturer);
        });
        f.row(t("Modell"), |ui| {
            ui.add(text_field(&mut p.name).desired_width(f32::INFINITY));
        });
        f.row(t("Kurzname"), |ui| {
            ui.add(
                text_field(&mut p.short_name)
                    .hint_text(t("optional"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Autor"), |ui| {
            ui.add(
                text_field(&mut p.author)
                    .hint_text(t("Dein Name"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Handbuch"), |ui| {
            ui.add(
                text_field(&mut p.manual_url)
                    .hint_text("https://")
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Produktseite"), |ui| {
            ui.add(
                text_field(&mut p.product_url)
                    .hint_text("https://")
                    .desired_width(f32::INFINITY),
            );
        });
    });
    section(ui, t("Kategorien"));
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(theme::SP_XS, theme::SP_XS);
        for c in CATEGORIES {
            let on = p.categories.iter().any(|x| x == c);
            if pill(ui, category_label(c), on).clicked() {
                if on {
                    p.categories.retain(|x| x != c);
                } else {
                    p.categories.push(c.to_string());
                    // Reihenfolge wie in der OFL-Liste
                    p.categories
                        .sort_by_key(|x| CATEGORIES.iter().position(|k| k == x));
                }
            }
        }
    });
    section(ui, t("Kommentar"));
    ui.add(
        egui::TextEdit::multiline(&mut p.comment)
            .desired_rows(4)
            .desired_width(f32::INFINITY)
            .background_color(theme::bg()),
    );
    if !p.created.is_empty() {
        ui.add_space(theme::SP_S);
        hint(
            ui,
            &tr!("Angelegt {c} · geändert {m}", c = p.created, m = p.modified),
        );
    }
}

/// Auswahl aus den Herstellern der Bibliothek (Schreibweise übernehmen).
fn manufacturer_picker(ui: &mut Ui, value: &mut String) {
    let r = ui
        .add(IconButton::new(Icon::ChevronDown))
        .on_hover_text(t("Hersteller aus der Bibliothek"));
    crate::ui::widgets::menu::dropdown(&r, |ui| {
        let q = value.to_lowercase();
        let mut names: Vec<&str> = Library::embedded()
            .fixtures
            .iter()
            .map(|f| f.manufacturer.as_str())
            .filter(|m| q.is_empty() || m.to_lowercase().contains(&q))
            .collect();
        names.dedup();
        egui::ScrollArea::vertical()
            .max_height(320.0)
            .show(ui, |ui| {
                crate::ui::widgets::menu::begin(ui);
                for m in names.into_iter().take(200) {
                    if crate::ui::widgets::menu::item(
                        ui,
                        crate::ui::widgets::menu::MenuItem::new(m),
                    ) {
                        *value = m.to_string();
                    }
                }
            });
    });
}

fn pill(ui: &mut Ui, label: &str, on: bool) -> egui::Response {
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), theme::body(), theme::text());
    let size = vec2(galley.size().x + 2.0 * theme::SP_M, 26.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let p = ui.painter();
    let (fill, stroke, text) = if on {
        (theme::accent_dim(), theme::accent(), theme::text())
    } else if resp.hovered() {
        (theme::raised_hover(), theme::border(), theme::text())
    } else {
        (theme::raised(), theme::border(), theme::text_dim())
    };
    p.rect(
        rect,
        13.0,
        fill,
        Stroke::new(1.0, stroke),
        egui::StrokeKind::Inside,
    );
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        theme::body(),
        text,
    );
    resp
}

// ------------------------------------------------------------ Physisch

fn physical(ui: &mut Ui, p: &mut Profile) {
    let ph = &mut p.physical;
    section(ui, t("Gehäuse"));
    form(ui, "builder_physical", |f| {
        f.row(t("Abmessungen"), |ui| {
            for (i, l) in [t("B"), t("H"), t("T")].into_iter().enumerate() {
                ui.label(egui::RichText::new(l).color(theme::text_faint()));
                ui.add(
                    egui::DragValue::new(&mut ph.dimensions[i])
                        .range(0.0..=10_000.0)
                        .speed(1.0)
                        .suffix(" mm"),
                );
                ui.add_space(theme::SP_S);
            }
        });
        f.row(t("Gewicht"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.weight)
                    .range(0.0..=1000.0)
                    .speed(0.1)
                    .max_decimals(2)
                    .suffix(" kg"),
            );
        });
        f.row(t("Leistung"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.power)
                    .range(0.0..=100_000.0)
                    .speed(1.0)
                    .suffix(" W"),
            );
        });
        f.row(t("DMX-Anschluss"), |ui| {
            let current = if ph.dmx_connector.is_empty() {
                t("unbekannt")
            } else {
                ph.dmx_connector.as_str()
            }
            .to_string();
            full_combo(ui, "builder_connector", &current, |ui| {
                ui.selectable_value(&mut ph.dmx_connector, String::new(), t("unbekannt"));
                for c in DMX_CONNECTORS {
                    ui.selectable_value(&mut ph.dmx_connector, c.to_string(), c);
                }
            });
        });
    });
    section(ui, t("Licht"));
    form(ui, "builder_light", |f| {
        f.row(t("Leuchtmittel"), |ui| {
            ui.add(
                text_field(&mut ph.bulb)
                    .hint_text(t("z. B. LED"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Farbtemperatur"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.color_temperature)
                    .range(0.0..=20_000.0)
                    .speed(10.0)
                    .suffix(" K"),
            );
        });
        f.row(t("Lichtstrom"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.lumens)
                    .range(0.0..=1_000_000.0)
                    .speed(10.0)
                    .suffix(" lm"),
            );
        });
        f.row(t("Optik"), |ui| {
            ui.add(
                text_field(&mut ph.lens)
                    .hint_text(t("optional, z. B. Fresnel"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Abstrahlwinkel"), |ui| {
            ui.label(egui::RichText::new(t("von")).color(theme::text_faint()));
            ui.add(
                egui::DragValue::new(&mut ph.beam_angle[0])
                    .range(0.0..=360.0)
                    .speed(0.5)
                    .suffix(" °"),
            );
            ui.add_space(theme::SP_S);
            ui.label(egui::RichText::new(t("bis")).color(theme::text_faint()));
            ui.add(
                egui::DragValue::new(&mut ph.beam_angle[1])
                    .range(0.0..=360.0)
                    .speed(0.5)
                    .suffix(" °"),
            );
        });
    });
    ui.add_space(theme::SP_S);
    hint(
        ui,
        t("Unbekannte Werte auf 0 lassen – sie werden nicht exportiert. Abstrahlwinkel und Abmessungen nutzt der 3D-Visualisierer."),
    );
}

// ------------------------------------------------------------ Funktionen

/// Karte für einen DMX-Bereich. Gibt `true` zurück, wenn gelöscht werden soll.
fn capability_card(
    ui: &mut Ui,
    id: (&str, usize, usize),
    cap: &mut Capability,
    removable: bool,
) -> bool {
    let mut remove = false;
    let salt = (id.1, id.2);
    egui::Frame::new()
        .fill(theme::raised())
        .stroke(Stroke::new(1.0, theme::border()))
        .corner_radius(theme::R_S)
        .inner_margin(egui::Margin::same(theme::SP_S as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.set_height(INPUT_H);
                ui.add(egui::DragValue::new(&mut cap.start).range(0..=255));
                ui.label(egui::RichText::new("–").color(theme::text_faint()));
                ui.add(egui::DragValue::new(&mut cap.end).range(0..=255));
                ui.add_space(theme::SP_S);
                let mut kind = cap.kind;
                ui.scope(|ui| {
                    ui.set_width(200.0);
                    full_combo(ui, ("builder_cap_kind", salt), kind_label(kind), |ui| {
                        for k in CapabilityKind::ALL {
                            ui.selectable_value(&mut kind, k, kind_label(k));
                        }
                    });
                });
                if kind != cap.kind {
                    cap.set_kind(kind);
                }
                ui.add_space(theme::SP_S);
                ui.label(
                    egui::RichText::new(t(cap.channel_kind().label()))
                        .small()
                        .color(theme::text_faint()),
                )
                .on_hover_text(t("So verwendet DMXCraft diesen Bereich"));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add(
                            IconButton::new(Icon::Trash)
                                .flat()
                                .danger()
                                .enabled(removable),
                        )
                        .on_hover_text(t("Bereich löschen"))
                        .clicked()
                    {
                        remove = true;
                    }
                });
            });
            ui.add_space(theme::SP_XS);
            ui.horizontal(|ui| {
                ui.set_height(INPUT_H);
                capability_fields(ui, salt, cap);
                ui.add(
                    text_field(&mut cap.comment)
                        .hint_text(t("Kommentar"))
                        .desired_width(ui.available_width()),
                );
            });
        });
    remove
}

/// Felder, die vom Typ abhängen (Farbe, Effekt, Slot, Start-/Endwert).
fn capability_fields(ui: &mut Ui, salt: (usize, usize), cap: &mut Capability) {
    use CapabilityKind as C;
    let small = |ui: &mut Ui, w: f32, add: &mut dyn FnMut(&mut Ui)| {
        ui.scope(|ui| {
            ui.set_width(w);
            add(ui);
        });
    };
    match cap.kind {
        C::ColorIntensity => {
            small(ui, 140.0, &mut |ui| {
                full_combo(
                    ui,
                    ("builder_cap_color", salt),
                    color_label(&cap.color),
                    |ui| {
                        for c in INTENSITY_COLORS {
                            ui.selectable_value(&mut cap.color, c.to_string(), color_label(c));
                        }
                    },
                );
            });
        }
        C::ShutterStrobe => {
            small(ui, 140.0, &mut |ui| {
                full_combo(
                    ui,
                    ("builder_cap_shutter", salt),
                    shutter_label(cap.shutter),
                    |ui| {
                        for s in ShutterEffect::ALL {
                            ui.selectable_value(&mut cap.shutter, s, shutter_label(s));
                        }
                    },
                );
            });
        }
        C::WheelSlot => {
            ui.label(egui::RichText::new(t("Slot")).color(theme::text_dim()));
            ui.add(egui::DragValue::new(&mut cap.slot).range(1..=64));
            small(ui, 100.0, &mut |ui| {
                full_combo(
                    ui,
                    ("builder_cap_slot", salt),
                    slot_label(cap.slot_kind),
                    |ui| {
                        for s in SlotKind::ALL {
                            ui.selectable_value(&mut cap.slot_kind, s, slot_label(s));
                        }
                    },
                );
            });
            if cap.slot_kind != SlotKind::Open {
                ui.add(
                    text_field(&mut cap.label)
                        .hint_text(t("Name"))
                        .desired_width(110.0),
                );
            }
            if cap.slot_kind == SlotKind::Color {
                hex_field(ui, &mut cap.hex);
            }
        }
        C::ColorPreset => {
            ui.add(
                text_field(&mut cap.label)
                    .hint_text(t("Farbname"))
                    .desired_width(110.0),
            );
            hex_field(ui, &mut cap.hex);
        }
        C::Effect => {
            ui.add(
                text_field(&mut cap.label)
                    .hint_text(t("Effektname"))
                    .desired_width(130.0),
            );
        }
        _ => {}
    }
    if let Some(prop) = cap.kind.property() {
        let (label, example) = property_label(prop);
        ui.label(egui::RichText::new(label).color(theme::text_dim()))
            .on_hover_text(example);
        ui.add(
            text_field(&mut cap.from)
                .hint_text(t("von"))
                .desired_width(80.0),
        )
        .on_hover_text(example);
        ui.add(
            text_field(&mut cap.to)
                .hint_text(t("bis"))
                .desired_width(80.0),
        )
        .on_hover_text(example);
    }
}

/// Farbfeld (#rrggbb) mit Farbwähler.
fn hex_field(ui: &mut Ui, hex: &mut String) {
    let mut c = egui::Color32::from_hex(hex).unwrap_or(egui::Color32::WHITE);
    let before = c;
    let r =
        egui::color_picker::color_edit_button_srgba(ui, &mut c, egui::color_picker::Alpha::Opaque);
    if c != before || (r.changed() && hex.is_empty()) {
        *hex = c.to_hex()[..7].to_string();
    }
    if !hex.is_empty()
        && ui
            .add(IconButton::new(Icon::Close).flat().size(20.0))
            .on_hover_text(t("Farbe entfernen"))
            .clicked()
    {
        hex.clear();
    }
}

// ------------------------------------------------------------ Prüfen

fn check(ui: &mut Ui, p: &Profile) {
    let problems = p.problems();
    section(ui, t("Probleme"));
    if problems.is_empty() {
        ui.horizontal(|ui| {
            Icon::ListChecks.paint(
                ui.painter(),
                Rect::from_min_size(ui.cursor().min, vec2(18.0, 18.0)),
                theme::OK,
            );
            ui.add_space(24.0);
            ui.label(egui::RichText::new(t("Bereit zum Speichern")).color(theme::OK));
        });
    }
    for pr in &problems {
        ui.horizontal(|ui| {
            let (rect, _) = ui.allocate_exact_size(vec2(10.0, 18.0), Sense::hover());
            ui.painter().circle_filled(rect.center(), 4.0, theme::WARN);
            ui.label(problem_text(pr));
        });
    }
    ui.add_space(theme::SP_M);
    section(ui, t("So erkennt DMXCraft die Kanäle"));
    for m in &p.modes {
        ui.label(
            egui::RichText::new(tr!(
                "{mode} · {n} Kan.",
                mode = m.name,
                n = m.channels.len()
            ))
            .strong(),
        );
        egui::Grid::new(("builder_check_mode", m.id))
            .num_columns(3)
            .spacing(vec2(theme::SP_L, 2.0))
            .striped(true)
            .show(ui, |ui| {
                for (i, e) in m.channels.iter().enumerate() {
                    ui.label(egui::RichText::new((i + 1).to_string()).color(theme::text_faint()));
                    if e.is_empty() {
                        ui.label(egui::RichText::new(t("(unbenutzt)")).color(theme::text_faint()));
                    } else {
                        ui.label(e);
                    }
                    ui.label(egui::RichText::new(t(p.kind_of(e).label())).color(theme::text_dim()));
                    ui.end_row();
                }
            });
        ui.add_space(theme::SP_S);
    }
    section(ui, t("OFL-Datei"));
    let json = p.to_ofl_string();
    ui.horizontal_top(|ui| {
        let w = ui.available_width() - 40.0;
        let mut text: &str = &json;
        ui.add(
            egui::TextEdit::multiline(&mut text)
                .code_editor()
                .desired_width(w)
                .desired_rows(json.lines().count().min(40)),
        );
        if ui
            .add(IconButton::new(Icon::Copy))
            .on_hover_text(t("Kopieren"))
            .clicked()
        {
            ui.ctx().copy_text(json.clone());
        }
    });
    hint(
        ui,
        t("Diese Datei lässt sich mit anderen teilen oder auf open-fixture-library.org einreichen."),
    );
}

fn problem_text(p: &Problem) -> String {
    match p {
        Problem::NoManufacturer => t("Hersteller fehlt").into(),
        Problem::NoName => t("Modellname fehlt").into(),
        Problem::NoCategory => t("Mindestens eine Kategorie wählen").into(),
        Problem::NoChannels => t("Keine Kanäle").into(),
        Problem::NoModes => t("Kein Modus").into(),
        Problem::EmptyChannelName => t("Kanal ohne Namen").into(),
        Problem::DuplicateChannel(c) => tr!("Kanalname „{c}“ doppelt", c = c),
        Problem::NoCapabilities(c) => tr!("„{c}“ hat keine Funktion", c = c),
        Problem::Gap(c, v) => tr!(
            "„{c}“: Wert {v} ist keinem Bereich zugeordnet",
            c = c,
            v = v
        ),
        Problem::Overlap(c, v) => tr!(
            "„{c}“: Bereiche überschneiden sich oder sind vertauscht (bei {v})",
            c = c,
            v = v
        ),
        Problem::MissingValue(c, v) => tr!(
            "„{c}“: Bereich ab {v} braucht einen Wert (z. B. Winkel oder Geschwindigkeit)",
            c = c,
            v = v
        ),
        Problem::EmptyModeName => t("Modus ohne Namen").into(),
        Problem::DuplicateMode(m) => tr!("Modusname „{m}“ doppelt", m = m),
        Problem::EmptyMode(m) => tr!("Modus „{m}“ hat keine Kanäle", m = m),
        Problem::TooManyChannels(m) => tr!("Modus „{m}“ hat mehr als 512 Kanäle", m = m),
        Problem::UnknownChannel(m, c) => {
            tr!("Modus „{m}“: Kanal „{c}“ gibt es nicht", m = m, c = c)
        }
        Problem::DuplicateInMode(m, c) => {
            tr!("Modus „{m}“: Kanal „{c}“ mehrfach", m = m, c = c)
        }
    }
}

// ------------------------------------------------------------ Dateien

fn pick_import(ctx: &egui::Context) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Gerät in den Profile Builder laden"))
            .add_filter(t("Gerätedateien"), &[PROFILE_EXT, "json", "qxf"])
            .pick_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

fn pick_export(ctx: &egui::Context, p: &Profile) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    let name = format!("{}.json", dmxcraft_ofl::profile::file_slug(&p.name));
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Als OFL-Datei exportieren"))
            .add_filter(t("Open Fixture Library (JSON)"), &["json"])
            .set_file_name(name)
            .save_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

/// Lädt eine Gerätedatei als (ungespeicherten) Entwurf.
fn import(path: &std::path::Path) -> Result<Profile, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let ext = path
        .extension()
        .map(|x| x.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    let mut p = if ext == PROFILE_EXT {
        Profile::from_json(&text).map_err(|e| e.to_string())?
    } else if ext == "qxf" {
        let f = dmxcraft_ofl::convert_qxf(&text).map_err(|e| e.to_string())?;
        Profile::from_types(&f.manufacturer, &f.name, &f.modes)
    } else {
        // OFL-Ordnerstruktur: <hersteller>/<gerät>.json – Namen aus der Bibliothek
        let man_key = path
            .parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let man = Library::embedded()
            .fixtures
            .iter()
            .find(|f| f.manufacturer_key == man_key)
            .map_or(String::new(), |f| f.manufacturer.clone());
        Profile::from_ofl(&man, &text).map_err(|e| e.to_string())?
    };
    // Importierte Geräte werden immer neu angelegt
    p.key.clear();
    Ok(p)
}

// ------------------------------------------------------------ Texte

fn kind_label(k: CapabilityKind) -> &'static str {
    use CapabilityKind as C;
    match k {
        C::NoFunction => t("Keine Funktion"),
        C::Intensity => t("Helligkeit"),
        C::ColorIntensity => t("Farbanteil"),
        C::ColorPreset => t("Farbvorgabe"),
        C::ColorTemperature => t("Farbtemperatur"),
        C::ShutterStrobe => t("Shutter / Strobe"),
        C::StrobeSpeed => t("Strobe-Geschwindigkeit"),
        C::Pan => t("Pan"),
        C::PanContinuous => t("Pan endlos"),
        C::Tilt => t("Tilt"),
        C::TiltContinuous => t("Tilt endlos"),
        C::PanTiltSpeed => t("Pan/Tilt-Geschwindigkeit"),
        C::WheelSlot => t("Rad-Slot"),
        C::WheelRotation => t("Raddrehung"),
        C::Effect => t("Effekt"),
        C::EffectSpeed => t("Effekt-Geschwindigkeit"),
        C::Prism => t("Prisma"),
        C::PrismRotation => t("Prismendrehung"),
        C::Focus => t("Fokus"),
        C::Zoom => t("Zoom"),
        C::Iris => t("Iris"),
        C::Frost => t("Frost"),
        C::Fog => t("Nebel"),
        C::FogOutput => t("Nebelmenge"),
        C::Rotation => t("Drehung"),
        C::Speed => t("Geschwindigkeit"),
        C::Maintenance => t("Wartung"),
        C::Generic => t("Generisch"),
    }
}

fn property_label(prop: &str) -> (&'static str, &'static str) {
    match prop {
        "brightness" => (t("Helligkeit"), t("z. B. off, dark, bright oder 0%, 100%")),
        "colorTemperature" => (t("Farbtemperatur"), t("z. B. warm, cold oder 3200K, 5600K")),
        "angle" => (t("Winkel"), t("z. B. 0deg, 540deg oder narrow, wide")),
        "speed" => (
            t("Geschwindigkeit"),
            t("z. B. slow, fast, stop oder 1Hz, 20Hz"),
        ),
        "distance" => (t("Entfernung"), t("z. B. near, far oder 1m, 10m")),
        "openPercent" => (t("Öffnung"), t("z. B. closed, open oder 0%, 100%")),
        "frostIntensity" => (t("Stärke"), t("z. B. 0%, 100%")),
        _ => (t("Menge"), t("z. B. weak, strong oder 0%, 100%")),
    }
}

fn shutter_label(s: ShutterEffect) -> &'static str {
    match s {
        ShutterEffect::Open => t("Offen"),
        ShutterEffect::Closed => t("Geschlossen"),
        ShutterEffect::Strobe => t("Strobe"),
        ShutterEffect::Pulse => t("Puls"),
        ShutterEffect::RampUp => t("Rampe auf"),
        ShutterEffect::RampDown => t("Rampe ab"),
        ShutterEffect::RampUpDown => t("Rampe auf und ab"),
        ShutterEffect::Lightning => t("Blitz"),
        ShutterEffect::Spikes => t("Spitzen"),
        ShutterEffect::Burst => t("Burst"),
    }
}

fn slot_label(s: SlotKind) -> &'static str {
    match s {
        SlotKind::Open => t("Offen"),
        SlotKind::Color => t("Farbe"),
        SlotKind::Gobo => t("Gobo"),
        SlotKind::Prism => t("Prisma"),
        SlotKind::Frost => t("Frost"),
    }
}

fn color_label(c: &str) -> &'static str {
    match c {
        "Red" => t("Rot"),
        "Green" => t("Grün"),
        "Blue" => t("Blau"),
        "White" => t("Weiß"),
        "Warm White" => t("Warmweiß"),
        "Cold White" => t("Kaltweiß"),
        "Amber" => t("Amber"),
        "UV" => t("UV"),
        "Cyan" => t("Cyan"),
        "Magenta" => t("Magenta"),
        "Yellow" => t("Gelb"),
        "Lime" => t("Limette"),
        "Indigo" => t("Indigo"),
        _ => t("Farbe wählen"),
    }
}

fn category_label(c: &str) -> &'static str {
    match c {
        "Barrel Scanner" => t("Barrel-Scanner"),
        "Blinder" => t("Blinder"),
        "Color Changer" => t("Farbwechsler"),
        "Dimmer" => t("Dimmer"),
        "Effect" => t("Effekt"),
        "Fan" => t("Ventilator"),
        "Flower" => t("Flower"),
        "Hazer" => t("Hazer"),
        "Laser" => t("Laser"),
        "Matrix" => t("Matrix"),
        "Moving Head" => t("Moving Head"),
        "Pixel Bar" => t("Pixel-Bar"),
        "Scanner" => t("Scanner"),
        "Smoke" => t("Nebelmaschine"),
        "Stand" => t("Stativ"),
        "Strobe" => t("Strobe"),
        _ => t("Sonstiges"),
    }
}
