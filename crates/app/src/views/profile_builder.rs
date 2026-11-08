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
use crate::ui::widgets::dialog::FooterButton;
use crate::ui::widgets::dialog::INPUT_H;
use crate::ui::widgets::fader::{kind_color, paint_symbol};
use crate::ui::widgets::{form, text_field, Dialog, Icon, IconButton, Segmented};

const SIDEBAR_W: f32 = 170.0;
const TAB_H: f32 = 34.0;
const FOOT_H: f32 = 52.0;
/// Formulare (Allgemein, Physisch) nicht über die ganze Fensterbreite ziehen.
const MAX_FORM_W: f32 = 900.0;
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
    /// Ausgewählter Bereich im ausgewählten Kanal.
    sel_range: usize,
    /// Ziehen an einer Bereichsgrenze: noch nicht verbrauchte Pixel.
    drag_acc: f32,
    sel_mode: usize,
    /// Wartet auf Bestätigung: ungespeicherte Änderungen verwerfen?
    pending: Option<Profile>,
    file: Option<(FileJob, Receiver<Option<PathBuf>>)>,
    /// Nach dem Anlegen eines Kanals/Modus: Namensfeld fokussieren.
    focus_name: bool,
    /// Fenster nach vorn holen (beim nächsten Zeichnen).
    raise: bool,
}

impl ProfileBuilder {
    fn dirty(&self) -> bool {
        self.draft != self.saved
    }

    /// Anfrage aus Menü oder Bibliothek.
    pub fn request(&mut self, req: BuilderRequest) {
        self.raise = true;
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
        self.sel_range = 0;
        self.sel_mode = 0;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    /// Fenstertitel (mit * bei ungespeicherten Änderungen).
    pub fn title(&self) -> String {
        let name = self.draft.display_name();
        let base = if name.is_empty() {
            format!("DMXCraft – {}", t("Profile Builder"))
        } else {
            format!("DMXCraft – {} – {name}", t("Profile Builder"))
        };
        if self.dirty() {
            format!("{base} *")
        } else {
            base
        }
    }

    /// Inhalt des eigenen Fensters: Reiter links, Inhalt, Fußzeile.
    pub fn window(&mut self, root: &mut Ui, engine: &mut Engine, st: &mut AppState) {
        let ctx = &root.ctx().clone();
        self.poll_file(st);
        if std::mem::take(&mut self.raise) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
        }
        let busy = self.file.is_some();
        let mut clicked = None;
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme::surface()))
            .show(root, |ui| {
                let area = ui.max_rect();
                ui.allocate_rect(area, Sense::hover());
                let foot = Rect::from_min_max(pos2(area.left(), area.bottom() - FOOT_H), area.max);
                let side = Rect::from_min_max(area.min, pos2(area.left() + SIDEBAR_W, foot.top()));
                let problems = self.draft.problems().len();
                sidebar(ui, side, &mut self.tab, problems);
                let content = Rect::from_min_max(
                    pos2(side.right() + theme::SP_L, area.top() + theme::SP_M),
                    pos2(area.right() - theme::SP_L, foot.top() - theme::SP_M),
                );
                // Fußzeile
                ui.painter().rect_filled(foot, 0.0, theme::bg());
                ui.painter().hline(
                    foot.x_range(),
                    foot.top() + 0.5,
                    Stroke::new(1.0, theme::border()),
                );
                let mut row = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(foot.shrink2(vec2(theme::SP_M, 0.0)))
                        .layout(Layout::right_to_left(Align::Center)),
                );
                row.spacing_mut().item_spacing = vec2(theme::SP_S, 0.0);
                let buttons = [
                    (t("Speichern"), true, 3),
                    (t("Schließen"), false, 2),
                    (t("Exportieren …"), false, 1),
                    (t("Importieren …"), false, 0),
                ];
                for (label, primary, i) in buttons {
                    let enabled = i >= 2 || !busy;
                    if row
                        .add(FooterButton::new(label).primary(primary).enabled(enabled))
                        .clicked()
                    {
                        clicked = Some(i);
                    }
                }
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
                                ui.set_width((content.width() - theme::SP_S).min(MAX_FORM_W));
                                match tab {
                                    Tab::General => general(ui, &mut self.draft),
                                    Tab::Physical => physical(ui, &mut self.draft),
                                    _ => check(ui, &self.draft),
                                }
                            });
                    }
                }
            });
        // Strg+S speichert
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            clicked = Some(3);
        }
        match clicked {
            Some(0) => self.file = Some((FileJob::Import, pick_import(ctx))),
            Some(1) => self.file = Some((FileJob::Export, pick_export(ctx, &self.draft))),
            Some(2) => self.open = false,
            Some(3) => self.save(engine, st),
            _ => {}
        }
        if self.pending.is_some() {
            self.confirm(ctx);
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
        // drei Spalten: Kanäle | Bereiche | Eigenschaften
        let gap = theme::SP_L;
        let list_w = (rect.width() * 0.3).clamp(240.0, 380.0);
        let props_w = (rect.width() * 0.34).clamp(320.0, 480.0);
        let list = Rect::from_min_size(rect.min, vec2(list_w, rect.height()));
        let props = Rect::from_min_max(pos2(rect.right() - props_w, rect.top()), rect.max);
        let ranges = Rect::from_min_max(
            pos2(list.right() + gap, rect.top()),
            pos2(props.left() - gap, rect.bottom()),
        );
        self.channel_list(ui, list);
        separator_v(ui, list.right() + gap / 2.0, rect);
        if self.sel_channel >= self.draft.channels.len() {
            return;
        }
        separator_v(ui, props.left() - gap / 2.0, rect);
        self.range_stack(ui, ranges);
        self.channel_props(ui, props);
    }

    /// Linke Spalte: Kanäle als Karten mit Symbol, Name und Art.
    fn channel_list(&mut self, ui: &mut Ui, rect: Rect) {
        let p = &mut self.draft;
        let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        let mut action: Option<ListAction> = None;
        lui.horizontal(|ui| {
            ui.set_height(ROW_H);
            ui.label(theme::heading_text(t("Kanäle")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus))
                    .on_hover_text(t("Kanal hinzufügen"))
                    .clicked()
                {
                    action = Some(ListAction::Add);
                }
            });
        });
        lui.add_space(theme::SP_XS);
        egui::ScrollArea::vertical()
            .id_salt("builder_channels")
            .auto_shrink([false, false])
            .show(&mut lui, |ui| {
                ui.spacing_mut().item_spacing.y = theme::SP_XS;
                for (i, c) in p.channels.iter().enumerate() {
                    let kind = c.kind();
                    let sub = if c.fine > 0 {
                        format!("{} · {} Bit", t(kind.label()), 8 * (u32::from(c.fine) + 1))
                    } else {
                        t(kind.label()).to_string()
                    };
                    let name = if c.name.is_empty() { "–" } else { &c.name };
                    let r = card_row(ui, i == self.sel_channel, i + 1, kind, name, &sub);
                    if r.clicked() && self.sel_channel != i {
                        self.sel_channel = i;
                        self.sel_range = 0;
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
        self.apply_channel_action(action);
    }

    fn apply_channel_action(&mut self, action: Option<ListAction>) {
        let p = &mut self.draft;
        match action {
            Some(ListAction::Add) => {
                let name = unique(&p.available_names(), t("Kanal"));
                p.channels.push(ProfileChannel {
                    name,
                    ..ProfileChannel::default()
                });
                self.sel_channel = p.channels.len() - 1;
                self.sel_range = 0;
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
                self.sel_range = 0;
            }
            None => {}
        }
    }

    /// Mittlere Spalte: Bereiche als Stapel von 0 (oben) bis 255 (unten).
    /// Die Grenzen lassen sich ziehen oder mit den Pfeilen um 1 verschieben.
    fn range_stack(&mut self, ui: &mut Ui, rect: Rect) {
        let i = self.sel_channel;
        let mut rui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        let ch = &mut self.draft.channels[i];
        ch.normalize_ranges();
        let mut add = false;
        rui.horizontal(|ui| {
            ui.set_height(ROW_H);
            ui.label(theme::heading_text(t("Bereiche")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus).enabled(ch.capabilities.len() < 256))
                    .on_hover_text(t("Bereich hinzufügen"))
                    .clicked()
                {
                    add = true;
                }
            });
        });
        rui.add_space(theme::SP_XS);
        if add {
            ch.add_range();
            self.sel_range = ch.capabilities.len() - 1;
        }
        self.sel_range = self.sel_range.min(ch.capabilities.len() - 1);
        let avail_h = rui.available_height() - theme::SP_S;
        let px_per_value = (avail_h / 256.0).max(1.0);
        let mut op: Option<(usize, i32)> = None;
        let mut select = None;
        egui::ScrollArea::vertical()
            .id_salt(("builder_ranges", i))
            .auto_shrink([false, false])
            .show(&mut rui, |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let full_w = ui.available_width();
                let label_w = 36.0;
                let mut blocks = Vec::new();
                for (ci, cap) in ch.capabilities.iter().enumerate() {
                    let width = f32::from(cap.end - cap.start) + 1.0;
                    let h = (width * px_per_value).max(RANGE_MIN_H);
                    let (row, _) = ui.allocate_exact_size(vec2(full_w, h), Sense::hover());
                    let block = Rect::from_min_max(pos2(row.left() + label_w, row.top()), row.max);
                    let resp =
                        ui.interact(block, ui.scope_id().with(("range", ci)), Sense::click());
                    if resp.clicked() {
                        select = Some(ci);
                    }
                    paint_range_block(ui, block, cap, ci == self.sel_range, resp.hovered());
                    // Wertgrenzen links
                    let p = ui.painter();
                    p.text(
                        pos2(row.left() + label_w - 6.0, row.top() + 2.0),
                        Align2::RIGHT_TOP,
                        cap.start.to_string(),
                        theme::caption(),
                        theme::text_dim(),
                    );
                    if cap.end > cap.start {
                        p.text(
                            pos2(row.left() + label_w - 6.0, row.bottom() - 2.0),
                            Align2::RIGHT_BOTTOM,
                            cap.end.to_string(),
                            theme::caption(),
                            theme::text_faint(),
                        );
                    }
                    blocks.push(block);
                }
                // Griffe an den Grenzen (nicht nach dem letzten Bereich)
                for (ci, block) in blocks
                    .iter()
                    .enumerate()
                    .take(blocks.len().saturating_sub(1))
                {
                    let handle = Rect::from_center_size(
                        pos2(block.center().x, block.bottom()),
                        vec2(26.0, 30.0),
                    );
                    let id = ui.scope_id().with(("range_handle", ci));
                    let resp = ui.interact(handle, id, Sense::click_and_drag());
                    paint_handle(ui, handle, resp.hovered() || resp.dragged());
                    if resp.hovered() || resp.dragged() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                    }
                    if resp.drag_started() {
                        self.drag_acc = 0.0;
                    }
                    if resp.dragged() {
                        self.drag_acc += resp.drag_delta().y;
                        let steps = (self.drag_acc / px_per_value).trunc();
                        if steps != 0.0 {
                            self.drag_acc -= steps * px_per_value;
                            op = Some((ci, steps as i32));
                        }
                    } else if resp.clicked() {
                        let up = resp
                            .interact_pointer_pos()
                            .is_some_and(|p| p.y < handle.center().y);
                        op = Some((ci, if up { -1 } else { 1 }));
                    }
                    resp.on_hover_text(t("Grenze ziehen oder oben/unten klicken (±1)"));
                }
            });
        if let Some(ci) = select {
            self.sel_range = ci;
        }
        if let Some((ci, delta)) = op {
            let end = (i32::from(ch.capabilities[ci].end) + delta).clamp(0, 254) as u8;
            ch.set_range_end(ci, end);
        }
    }

    /// Rechte Spalte: Kanal- und Bereichseigenschaften, Aktionen.
    fn channel_props(&mut self, ui: &mut Ui, rect: Rect) {
        let i = self.sel_channel;
        let focus = std::mem::take(&mut self.focus_name);
        let mut pui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect)
                .layout(Layout::top_down(Align::Min)),
        );
        let mut channel_action = None;
        let mut remove_range = false;
        egui::ScrollArea::vertical()
            .id_salt("builder_channel_props")
            .auto_shrink([false, false])
            .show(&mut pui, |ui| {
                ui.set_width(rect.width() - theme::SP_S);
                let p = &mut self.draft;
                ui.horizontal(|ui| {
                    ui.set_height(ROW_H);
                    ui.label(theme::heading_text(t("Kanal")));
                });
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
                        let k = p.channels[i].kind();
                        let (r, _) = ui.allocate_exact_size(vec2(20.0, 20.0), Sense::hover());
                        paint_symbol(ui.painter(), r, k, kind_color(k));
                        ui.label(t(k.label()));
                    });
                });
                ui.add_space(theme::SP_L);
                let ch = &mut p.channels[i];
                let ci = self.sel_range.min(ch.capabilities.len() - 1);
                let count = ch.capabilities.len();
                let last = ci + 1 == count;
                ui.horizontal(|ui| {
                    ui.set_height(ROW_H);
                    ui.label(theme::heading_text(&tr!("Bereich {n}", n = ci + 1)));
                });
                let mut new_end = None;
                let cap = &mut ch.capabilities[ci];
                form(ui, "builder_range", |f| {
                    f.row(t("Werte"), |ui| {
                        ui.label(
                            egui::RichText::new(cap.start.to_string()).color(theme::text_dim()),
                        );
                        ui.label(egui::RichText::new("–").color(theme::text_faint()));
                        let mut end = cap.end;
                        let r = ui
                            .add(egui::DragValue::new(&mut end).range(cap.start..=255))
                            .on_hover_text(if last {
                                t("Ende – kleiner als 255 legt für den Rest einen neuen Bereich an")
                            } else {
                                t("Ende – die folgenden Bereiche rücken automatisch nach")
                            });
                        if end != cap.end && (r.changed() || r.lost_focus()) {
                            new_end = Some(end);
                        }
                    });
                    f.row(t("Typ"), |ui| {
                        let mut kind = cap.kind;
                        full_combo(ui, ("builder_cap_kind", i, ci), kind_label(kind), |ui| {
                            for k in CapabilityKind::ALL {
                                ui.selectable_value(&mut kind, k, kind_label(k));
                            }
                        });
                        if kind != cap.kind {
                            cap.set_kind(kind);
                        }
                    });
                    f.row(t("Kommentar"), |ui| {
                        ui.add(text_field(&mut cap.comment).desired_width(f32::INFINITY));
                    });
                });
                ui.add_space(theme::SP_XS);
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(theme::SP_S, theme::SP_S);
                    capability_fields(ui, (i, ci), cap);
                });
                if let Some(end) = new_end {
                    ch.set_range_end(ci, end);
                }
                ui.add_space(theme::SP_L);
                if wide_button(ui, t("Bereich löschen"), false, count > 1).clicked() {
                    remove_range = true;
                }
                if wide_button(ui, t("Kanal duplizieren"), false, true).clicked() {
                    channel_action = Some(ListAction::Duplicate(i));
                }
                if wide_button(ui, t("Kanal löschen"), true, true).clicked() {
                    channel_action = Some(ListAction::Remove(i));
                }
            });
        if remove_range {
            let ch = &mut self.draft.channels[i];
            ch.remove_range(self.sel_range);
            self.sel_range = self.sel_range.min(ch.capabilities.len() - 1);
        }
        self.apply_channel_action(channel_action);
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

/// Mindesthöhe eines Bereichs im Stapel.
const RANGE_MIN_H: f32 = 46.0;

/// Kanal-Karte: Nummer, Symbol der Kanalart, Name und Art darunter.
fn card_row(
    ui: &mut Ui,
    selected: bool,
    number: usize,
    kind: dmxcraft_core::ChannelKind,
    name: &str,
    sub: &str,
) -> egui::Response {
    let (row, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 48.0), Sense::click());
    let p = ui.painter();
    p.text(
        pos2(row.left() + 14.0, row.center().y),
        Align2::CENTER_CENTER,
        number.to_string(),
        theme::caption(),
        theme::text_faint(),
    );
    let card = Rect::from_min_max(pos2(row.left() + 28.0, row.top()), row.max);
    let (fill, stroke) = if selected {
        (theme::raised(), Stroke::new(1.5, theme::accent()))
    } else if resp.hovered() {
        (theme::raised_hover(), Stroke::new(1.0, theme::border()))
    } else {
        (theme::raised(), Stroke::new(1.0, theme::border()))
    };
    p.rect(
        card,
        theme::R_S as f32,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    let icon = Rect::from_center_size(pos2(card.left() + 22.0, card.center().y), vec2(24.0, 24.0));
    paint_symbol(p, icon, kind, kind_color(kind));
    let text_x = card.left() + 44.0;
    let clip = Rect::from_min_max(
        pos2(text_x, card.top()),
        pos2(card.right() - 8.0, card.bottom()),
    );
    let pc = p.with_clip_rect(clip);
    pc.text(
        pos2(text_x, card.center().y - 8.0),
        Align2::LEFT_CENTER,
        name,
        theme::body(),
        theme::text(),
    );
    pc.text(
        pos2(text_x, card.center().y + 9.0),
        Align2::LEFT_CENTER,
        sub,
        theme::caption(),
        theme::text_dim(),
    );
    resp
}

/// Ein Bereich im Stapel: Symbol, Typ und Details.
fn paint_range_block(ui: &Ui, r: Rect, cap: &Capability, selected: bool, hovered: bool) {
    let p = ui.painter();
    let (fill, stroke) = if selected {
        (theme::raised(), Stroke::new(1.5, theme::accent()))
    } else if hovered {
        (theme::raised_hover(), Stroke::new(1.0, theme::border()))
    } else {
        (theme::raised(), Stroke::new(1.0, theme::border()))
    };
    p.rect(
        r.shrink2(vec2(0.0, 1.0)),
        theme::R_S as f32,
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    let k = cap.channel_kind();
    let icon = Rect::from_center_size(pos2(r.left() + 22.0, r.center().y), vec2(22.0, 22.0));
    paint_symbol(p, icon, k, kind_color(k));
    let mut title = kind_label(cap.kind).to_string();
    let detail = match cap.kind {
        CapabilityKind::ColorIntensity => color_label(&cap.color).to_string(),
        CapabilityKind::ShutterStrobe => shutter_label(cap.shutter).to_string(),
        CapabilityKind::WheelSlot => {
            let name = if cap.label.trim().is_empty() {
                slot_label(cap.slot_kind).to_string()
            } else {
                cap.label.trim().to_string()
            };
            format!("{} {} · {name}", t("Slot"), cap.slot)
        }
        _ => cap.label.trim().to_string(),
    };
    if !detail.is_empty() {
        title = format!("{title} – {detail}");
    }
    let sub = match (cap.from.trim(), cap.to.trim()) {
        ("", "") => cap.comment.trim().to_string(),
        (a, b) if a == b || b.is_empty() => a.to_string(),
        (a, "") => a.to_string(),
        (a, b) => format!("{a} – {b}"),
    };
    let x = r.left() + 42.0;
    let clip = Rect::from_min_max(pos2(x, r.top()), pos2(r.right() - 8.0, r.bottom()));
    let pc = p.with_clip_rect(clip);
    let (ty, sy) = if sub.is_empty() || r.height() < 40.0 {
        (r.center().y, None)
    } else {
        (r.center().y - 8.0, Some(r.center().y + 9.0))
    };
    pc.text(
        pos2(x, ty),
        Align2::LEFT_CENTER,
        title,
        theme::body(),
        theme::text(),
    );
    if let Some(sy) = sy {
        pc.text(
            pos2(x, sy),
            Align2::LEFT_CENTER,
            sub,
            theme::caption(),
            theme::text_dim(),
        );
    }
}

/// Griff an einer Bereichsgrenze: Pfeil hoch/runter.
fn paint_handle(ui: &Ui, r: Rect, active: bool) {
    let p = ui.painter();
    let (fill, stroke) = if active {
        (theme::raised_hover(), theme::accent())
    } else {
        (theme::surface(), theme::border())
    };
    p.rect(
        r,
        4.0,
        fill,
        Stroke::new(1.0, stroke),
        egui::StrokeKind::Inside,
    );
    let color = if active {
        theme::text()
    } else {
        theme::text_dim()
    };
    let half = |top: bool| {
        let c = if top {
            pos2(r.center().x, r.top() + r.height() * 0.28)
        } else {
            pos2(r.center().x, r.bottom() - r.height() * 0.28)
        };
        Rect::from_center_size(c, vec2(12.0, 12.0))
    };
    Icon::ChevronUp.paint(p, half(true), color);
    Icon::ChevronDown.paint(p, half(false), color);
}

/// Breiter Knopf (Aktionen in der Eigenschaftsspalte).
fn wide_button(ui: &mut Ui, label: &str, danger: bool, enabled: bool) -> egui::Response {
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 32.0), sense);
    let hovered = resp.hovered() && enabled;
    let fill = if hovered {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    let text = match (enabled, danger) {
        (false, _) => theme::text_faint(),
        (true, true) => theme::DANGER,
        (true, false) => theme::text(),
    };
    ui.painter().rect(
        r,
        16.0,
        fill,
        Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        r.center(),
        Align2::CENTER_CENTER,
        label,
        theme::body(),
        text,
    );
    ui.add_space(theme::SP_XS);
    resp
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
