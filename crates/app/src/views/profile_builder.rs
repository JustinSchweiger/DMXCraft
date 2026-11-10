//! Tools → Profile Builder: create your own fixtures (modeled on the fixture
//! editor of the Open Fixture Library).
//!
//! Tabs: General, Physical, Channels (with DMX ranges), Modes, Check.
//! Saved in the config folder ([`crate::profiles`]); the fixtures then
//! appear in the library under “My fixtures”.
//! Export as an OFL file (to share or submit to the OFL).

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
/// Do not stretch forms (General, Physical) across the whole window width.
const MAX_FORM_W: f32 = 900.0;
const LIST_W: f32 = 220.0;
const ROW_H: f32 = 28.0;

/// What the Profile Builder should open.
#[derive(Clone, Debug)]
pub enum BuilderRequest {
    /// Open the window, keep the current draft (Tools menu).
    Open,
    /// Empty fixture.
    New,
    /// Edit a saved profile.
    Edit(Box<Profile>),
    /// New fixture from a template (library, duplicate).
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
            Tab::General => t("General"),
            Tab::Physical => t("Physical"),
            Tab::Channels => t("Channels"),
            Tab::Modes => t("Modes"),
            Tab::Check => t("Check"),
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
    /// State at load/save (for “unsaved”).
    saved: Profile,
    sel_channel: usize,
    /// Selected range in the selected channel.
    sel_range: usize,
    /// Dragging a range border: pixels not consumed yet.
    drag_acc: f32,
    sel_mode: usize,
    /// Waiting for confirmation: discard unsaved changes?
    pending: Option<Profile>,
    file: Option<(FileJob, Receiver<Option<PathBuf>>)>,
    /// After creating a channel/mode: focus the name field.
    focus_name: bool,
    /// Bring the window to the front (at the next draw).
    raise: bool,
}

impl ProfileBuilder {
    fn dirty(&self) -> bool {
        self.draft != self.saved
    }

    /// Request from the menu or the library. `window`: open in its own window
    /// (otherwise only load the draft – for the tab).
    pub fn request(&mut self, req: BuilderRequest, window: bool) {
        self.raise = window;
        let next = match req {
            BuilderRequest::Open => {
                if self.draft == Profile::default() {
                    self.load(new_profile(), true);
                }
                self.open |= window;
                return;
            }
            BuilderRequest::New => new_profile(),
            BuilderRequest::Edit(p) | BuilderRequest::Template(p) => *p,
        };
        self.open |= window;
        if self.dirty() && self.draft != Profile::default() {
            self.pending = Some(next);
        } else {
            self.load(next, true);
        }
    }

    /// Take over a new draft. `fresh` = unsaved (template, new).
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

    /// Open in its own window (from the tab).
    pub fn open_window(&mut self) {
        if self.draft == Profile::default() {
            self.load(new_profile(), true);
        }
        self.open = true;
        self.raise = true;
    }

    /// Window title (with * for unsaved changes).
    pub fn title(&self) -> String {
        let name = self.draft.display_name();
        let base = if name.is_empty() {
            format!("DMXCraft – {}", t("Profile builder"))
        } else {
            format!("DMXCraft – {} – {name}", t("Profile builder"))
        };
        if self.dirty() {
            format!("{base} *")
        } else {
            base
        }
    }

    /// Content of the separate window: tabs on the left, content, footer.
    pub fn window(&mut self, root: &mut Ui, engine: &mut Engine, st: &mut AppState) {
        self.body(root, engine, st, false);
    }

    /// Content in the title bar tab (instead of “Close”: “Open in window”).
    pub fn embedded(&mut self, root: &mut Ui, engine: &mut Engine, st: &mut AppState) {
        if self.draft == Profile::default() {
            self.load(new_profile(), true);
        }
        self.body(root, engine, st, true);
    }

    fn body(&mut self, root: &mut Ui, engine: &mut Engine, st: &mut AppState, embedded: bool) {
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
                // Footer
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
                    (t("Save"), true, 3),
                    (
                        if embedded {
                            t("Open in window")
                        } else {
                            t("Close")
                        },
                        false,
                        2,
                    ),
                    (t("Export …"), false, 1),
                    (t("Import …"), false, 0),
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
        // Ctrl+S saves
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::COMMAND, egui::Key::S)) {
            clicked = Some(3);
        }
        match clicked {
            Some(0) => self.file = Some((FileJob::Import, pick_import(ctx))),
            Some(1) => self.file = Some((FileJob::Export, pick_export(ctx, &self.draft))),
            Some(2) => {
                if embedded {
                    self.open_window();
                } else {
                    self.open = false;
                }
            }
            Some(3) => self.save(engine, st),
            _ => {}
        }
        if self.pending.is_some() {
            self.confirm(ctx);
        }
    }

    /// Ask before an unsaved draft is replaced.
    fn confirm(&mut self, ctx: &egui::Context) {
        let mut open = true;
        let res = Dialog::new(t("Unsaved changes"))
            .id("profile_builder_confirm")
            .width(380.0)
            .cancel(t("Cancel"))
            .primary(t("Discard"))
            .show(ctx, &mut open, |ui, _| {
                ui.label(tr!(
                    "Discard changes to “{v}”?",
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
                "Not saved: {n} problem(s), see Check",
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
            st.error(tr!("Not saved: {e}", e = e));
            return;
        }
        // The key is only fixed now
        let types = if self.draft.key.is_empty() {
            draft.fixture_types().unwrap_or(types)
        } else {
            types
        };
        self.draft = draft;
        self.saved = self.draft.clone();
        // Update already patched fixtures of this profile in the show
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
                "“{v}” saved – {n} fixture(s) in the show updated",
                v = self.draft.display_name(),
                n = updated
            ));
        } else {
            st.info(tr!(
                "“{v}” saved – find it in the library under My fixtures",
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
                Ok(()) => st.info(tr!("Exported: {path}", path = path.display())),
                Err(e) => st.error(format!("{}: {e}", path.display())),
            },
            FileJob::Import => match import(&path) {
                Ok(p) => {
                    st.info(tr!("“{v}” imported – not saved yet", v = p.display_name()));
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

    // ------------------------------------------------------------ Channels

    fn channels_tab(&mut self, ui: &mut Ui, rect: Rect) {
        // three columns: channels | ranges | properties
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

    /// Left column: channels as cards with icon, name and kind.
    fn channel_list(&mut self, ui: &mut Ui, rect: Rect) {
        let p = &mut self.draft;
        let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(rect));
        let mut action: Option<ListAction> = None;
        lui.horizontal(|ui| {
            ui.set_height(ROW_H);
            ui.label(theme::heading_text(t("Channels")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus))
                    .on_hover_text(t("Add channel"))
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
                        if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
                            action = Some(ListAction::Duplicate(i));
                        }
                        menu::separator(ui);
                        if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash).danger()) {
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
                let name = unique(&p.available_names(), t("Channel"));
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

    /// Middle column: ranges as a stack from 0 (top) to 255 (bottom).
    /// The borders can be dragged or moved by 1 with the arrows.
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
            ui.label(theme::heading_text(t("Ranges")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus).enabled(ch.capabilities.len() < 256))
                    .on_hover_text(t("Add range"))
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
                    // Value borders on the left
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
                // Handles at the borders (not after the last range)
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
                    resp.on_hover_text(t("Drag the boundary or click top/bottom (±1)"));
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

    /// Right column: channel and range properties, actions.
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
                    ui.label(theme::heading_text(t("Channel")));
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
                    f.row(t("Resolution"), |ui| {
                        let mut fine = p.channels[i].fine;
                        let items = [(0u8, "8 Bit"), (1, "16 Bit"), (2, "24 Bit")];
                        if Segmented::new(&items).show(ui, &mut fine) {
                            p.set_fine(i, fine);
                        }
                    });
                    f.row(t("Default value"), |ui| {
                        ui.add(egui::DragValue::new(&mut p.channels[i].default).range(0..=255))
                            .on_hover_text(t("Value while no scene sets the channel"));
                    });
                    f.row(t("Detected as"), |ui| {
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
                    ui.label(theme::heading_text(&tr!("Range {n}", n = ci + 1)));
                });
                let mut new_end = None;
                let cap = &mut ch.capabilities[ci];
                form(ui, "builder_range", |f| {
                    f.row(t("Values"), |ui| {
                        ui.label(
                            egui::RichText::new(cap.start.to_string()).color(theme::text_dim()),
                        );
                        ui.label(egui::RichText::new("–").color(theme::text_faint()));
                        let mut end = cap.end;
                        let r = ui
                            .add(egui::DragValue::new(&mut end).range(cap.start..=255))
                            .on_hover_text(if last {
                                t("End – less than 255 creates a new range for the rest")
                            } else {
                                t("End – the following ranges move along automatically")
                            });
                        if end != cap.end && (r.changed() || r.lost_focus()) {
                            new_end = Some(end);
                        }
                    });
                    f.row(t("Type"), |ui| {
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
                    f.row(t("Comment"), |ui| {
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
                if wide_button(ui, t("Delete range"), false, count > 1).clicked() {
                    remove_range = true;
                }
                if wide_button(ui, t("Duplicate channel"), false, true).clicked() {
                    channel_action = Some(ListAction::Duplicate(i));
                }
                if wide_button(ui, t("Delete channel"), true, true).clicked() {
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

    // ------------------------------------------------------------ Modes

    fn modes_tab(&mut self, ui: &mut Ui, rect: Rect) {
        let (list_rect, detail_rect) = split(rect);
        let p = &mut self.draft;
        let mut lui = ui.new_child(egui::UiBuilder::new().max_rect(list_rect));
        let mut action: Option<ListAction> = None;
        list_header(&mut lui, t("Add mode"), || action = Some(ListAction::Add));
        egui::ScrollArea::vertical()
            .id_salt("builder_modes")
            .auto_shrink([false, false])
            .show(&mut lui, |ui| {
                for (i, m) in p.modes.iter().enumerate() {
                    let n = tr!("{n} ch.", n = m.channels.len());
                    let name = if m.name.is_empty() { "–" } else { &m.name };
                    let r = list_row(ui, i == self.sel_mode, name, &n);
                    if r.clicked() {
                        self.sel_mode = i;
                    }
                    crate::ui::widgets::menu::context(&r, |ui| {
                        use crate::ui::widgets::menu::{self, MenuItem};
                        menu::begin(ui);
                        if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
                            action = Some(ListAction::Duplicate(i));
                        }
                        menu::separator(ui);
                        if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash).danger()) {
                            action = Some(ListAction::Remove(i));
                        }
                    });
                }
            });
        match action {
            Some(ListAction::Add) => {
                let id = p.next_mode_id();
                // A new mode contains all channels (the most common case)
                let channels = p.available_names();
                let names: Vec<String> = p.modes.iter().map(|m| m.name.clone()).collect();
                p.modes.push(ProfileMode {
                    id,
                    name: unique(&names, &tr!("{n}-channel", n = channels.len())),
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
            f.row(t("Short name"), |ui| {
                ui.add(
                    text_field(&mut p.modes[i].short_name)
                        .hint_text(t("optional, e.g. 9ch"))
                        .desired_width(f32::INFINITY),
                );
            });
        });
        dui.add_space(theme::SP_M);
        // Two columns: channels in the mode | available channels
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
            ui.label(theme::heading_text(t("Channel order")));
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
                            (t("(unused)").to_string(), theme::text_faint())
                        } else if available.contains(entry) {
                            (entry.clone(), theme::text())
                        } else {
                            (entry.clone(), theme::DANGER)
                        };
                        ui.label(egui::RichText::new(text).color(color));
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            if ui
                                .add(IconButton::new(Icon::Close).flat().size(22.0))
                                .on_hover_text(t("Remove from mode"))
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
                                .on_hover_text(t("Move down"))
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
                                .on_hover_text(t("Move up"))
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
            ui.label(theme::heading_text(t("Available")));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus))
                    .on_hover_text(t("Append unused channel"))
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
                    if r.on_hover_text(t("Append")).clicked() {
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

/// New, empty fixture with a dimmer channel and one mode.
fn new_profile() -> Profile {
    let dimmer = ProfileChannel::new("Dimmer", Capability::new(CapabilityKind::Intensity));
    Profile {
        categories: vec!["Dimmer".into()],
        modes: vec![ProfileMode {
            id: 1,
            name: tr!("{n}-channel", n = 1),
            short_name: String::new(),
            channels: vec![dimmer.name.clone()],
        }],
        channels: vec![dimmer],
        ..Profile::default()
    }
}

/// Template from a library fixture or a fixture list.
pub fn template(manufacturer: &str, name: &str, types: &[FixtureType]) -> Profile {
    let mut p = Profile::from_types(manufacturer, name, types);
    p.name = tr!("{v} (copy)", v = name);
    p
}

/// Find a free name: “Channel”, “Channel 2”, …
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

/// Row of a list: name on the left, extra info muted on the right.
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

/// Tab bar on the left (as in the settings window), error count on the
/// “Check” tab.
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

// ------------------------------------------------------------ General

fn general(ui: &mut Ui, p: &mut Profile) {
    section(ui, t("Device"));
    form(ui, "builder_general", |f| {
        f.row(t("Manufacturer"), |ui| {
            ui.add(
                text_field(&mut p.manufacturer)
                    .hint_text(t("e.g. Cameo"))
                    .desired_width(ui.available_width() - 32.0),
            );
            manufacturer_picker(ui, &mut p.manufacturer);
        });
        f.row(t("Model"), |ui| {
            ui.add(text_field(&mut p.name).desired_width(f32::INFINITY));
        });
        f.row(t("Short name"), |ui| {
            ui.add(
                text_field(&mut p.short_name)
                    .hint_text(t("optional"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Author"), |ui| {
            ui.add(
                text_field(&mut p.author)
                    .hint_text(t("Your name"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Manual"), |ui| {
            ui.add(
                text_field(&mut p.manual_url)
                    .hint_text("https://")
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Product page"), |ui| {
            ui.add(
                text_field(&mut p.product_url)
                    .hint_text("https://")
                    .desired_width(f32::INFINITY),
            );
        });
    });
    section(ui, t("Categories"));
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(theme::SP_XS, theme::SP_XS);
        for c in CATEGORIES {
            let on = p.categories.iter().any(|x| x == c);
            if pill(ui, category_label(c), on).clicked() {
                if on {
                    p.categories.retain(|x| x != c);
                } else {
                    p.categories.push(c.to_string());
                    // Order as in the OFL list
                    p.categories
                        .sort_by_key(|x| CATEGORIES.iter().position(|k| k == x));
                }
            }
        }
    });
    section(ui, t("Comment"));
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
            &tr!("Created {c} · modified {m}", c = p.created, m = p.modified),
        );
    }
}

/// Selection from the manufacturers of the library (take over the spelling).
fn manufacturer_picker(ui: &mut Ui, value: &mut String) {
    let r = ui
        .add(IconButton::new(Icon::ChevronDown))
        .on_hover_text(t("Manufacturer from the library"));
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

// ------------------------------------------------------------ Physical

fn physical(ui: &mut Ui, p: &mut Profile) {
    let ph = &mut p.physical;
    section(ui, t("Housing"));
    form(ui, "builder_physical", |f| {
        f.row(t("Dimensions"), |ui| {
            for (i, l) in [t("W"), t("H"), t("D")].into_iter().enumerate() {
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
        f.row(t("Weight"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.weight)
                    .range(0.0..=1000.0)
                    .speed(0.1)
                    .max_decimals(2)
                    .suffix(" kg"),
            );
        });
        f.row(t("Power"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.power)
                    .range(0.0..=100_000.0)
                    .speed(1.0)
                    .suffix(" W"),
            );
        });
        f.row(t("DMX connector"), |ui| {
            let current = if ph.dmx_connector.is_empty() {
                t("unknown")
            } else {
                ph.dmx_connector.as_str()
            }
            .to_string();
            full_combo(ui, "builder_connector", &current, |ui| {
                ui.selectable_value(&mut ph.dmx_connector, String::new(), t("unknown"));
                for c in DMX_CONNECTORS {
                    ui.selectable_value(&mut ph.dmx_connector, c.to_string(), c);
                }
            });
        });
    });
    section(ui, t("Lighting"));
    form(ui, "builder_light", |f| {
        f.row(t("Light source"), |ui| {
            ui.add(
                text_field(&mut ph.bulb)
                    .hint_text(t("e.g. LED"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Color temperature"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.color_temperature)
                    .range(0.0..=20_000.0)
                    .speed(10.0)
                    .suffix(" K"),
            );
        });
        f.row(t("Luminous flux"), |ui| {
            ui.add(
                egui::DragValue::new(&mut ph.lumens)
                    .range(0.0..=1_000_000.0)
                    .speed(10.0)
                    .suffix(" lm"),
            );
        });
        f.row(t("Lens"), |ui| {
            ui.add(
                text_field(&mut ph.lens)
                    .hint_text(t("optional, e.g. Fresnel"))
                    .desired_width(f32::INFINITY),
            );
        });
        f.row(t("Beam angle"), |ui| {
            ui.label(egui::RichText::new(t("from")).color(theme::text_faint()));
            ui.add(
                egui::DragValue::new(&mut ph.beam_angle[0])
                    .range(0.0..=360.0)
                    .speed(0.5)
                    .suffix(" °"),
            );
            ui.add_space(theme::SP_S);
            ui.label(egui::RichText::new(t("to")).color(theme::text_faint()));
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
        t("Leave unknown values at 0 – they are not exported. The 3D visualizer uses the beam angle and dimensions."),
    );
}

// ------------------------------------------------------------ Functions

/// Minimum height of a range in the stack.
const RANGE_MIN_H: f32 = 46.0;

/// Channel card: number, icon of the channel kind, name and kind below.
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

/// A range in the stack: icon, type and details.
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

/// Handle at a range border: arrow up/down.
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

/// Wide button (actions in the properties column).
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

/// Fields that depend on the type (color, effect, slot, start/end value).
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
                    .hint_text(t("Color name"))
                    .desired_width(110.0),
            );
            hex_field(ui, &mut cap.hex);
        }
        C::Effect => {
            ui.add(
                text_field(&mut cap.label)
                    .hint_text(t("Effect name"))
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
                .hint_text(t("from"))
                .desired_width(80.0),
        )
        .on_hover_text(example);
        ui.add(
            text_field(&mut cap.to)
                .hint_text(t("to"))
                .desired_width(80.0),
        )
        .on_hover_text(example);
    }
}

/// Color field (#rrggbb) with color picker.
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
            .on_hover_text(t("Remove color"))
            .clicked()
    {
        hex.clear();
    }
}

// ------------------------------------------------------------ Check

fn check(ui: &mut Ui, p: &Profile) {
    let problems = p.problems();
    section(ui, t("Problems"));
    if problems.is_empty() {
        ui.horizontal(|ui| {
            Icon::ListChecks.paint(
                ui.painter(),
                Rect::from_min_size(ui.cursor().min, vec2(18.0, 18.0)),
                theme::OK,
            );
            ui.add_space(24.0);
            ui.label(egui::RichText::new(t("Ready to save")).color(theme::OK));
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
    section(ui, t("How DMXCraft detects the channels"));
    for m in &p.modes {
        ui.label(
            egui::RichText::new(tr!("{mode} · {n} ch.", mode = m.name, n = m.channels.len()))
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
                        ui.label(egui::RichText::new(t("(unused)")).color(theme::text_faint()));
                    } else {
                        ui.label(e);
                    }
                    ui.label(egui::RichText::new(t(p.kind_of(e).label())).color(theme::text_dim()));
                    ui.end_row();
                }
            });
        ui.add_space(theme::SP_S);
    }
    section(ui, t("OFL file"));
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
            .on_hover_text(t("Copy"))
            .clicked()
        {
            ui.ctx().copy_text(json.clone());
        }
    });
    hint(
        ui,
        t("You can share this file or submit it to open-fixture-library.org."),
    );
}

fn problem_text(p: &Problem) -> String {
    match p {
        Problem::NoManufacturer => t("Manufacturer missing").into(),
        Problem::NoName => t("Model name missing").into(),
        Problem::NoCategory => t("Choose at least one category").into(),
        Problem::NoChannels => t("No channels").into(),
        Problem::NoModes => t("No mode").into(),
        Problem::EmptyChannelName => t("Channel without a name").into(),
        Problem::DuplicateChannel(c) => tr!("Channel name “{c}” used twice", c = c),
        Problem::NoCapabilities(c) => tr!("“{c}” has no capability", c = c),
        Problem::Gap(c, v) => tr!("“{c}”: value {v} is not in any range", c = c, v = v),
        Problem::Overlap(c, v) => tr!(
            "“{c}”: ranges overlap or are reversed (at {v})",
            c = c,
            v = v
        ),
        Problem::MissingValue(c, v) => tr!(
            "“{c}”: range from {v} needs a value (e.g. angle or speed)",
            c = c,
            v = v
        ),
        Problem::EmptyModeName => t("Mode without a name").into(),
        Problem::DuplicateMode(m) => tr!("Mode name “{m}” used twice", m = m),
        Problem::EmptyMode(m) => tr!("Mode “{m}” has no channels", m = m),
        Problem::TooManyChannels(m) => tr!("Mode “{m}” has more than 512 channels", m = m),
        Problem::UnknownChannel(m, c) => {
            tr!("Mode “{m}”: channel “{c}” does not exist", m = m, c = c)
        }
        Problem::DuplicateInMode(m, c) => {
            tr!(
                "Mode “{m}”: channel “{c}” used more than once",
                m = m,
                c = c
            )
        }
    }
}

// ------------------------------------------------------------ Files

fn pick_import(ctx: &egui::Context) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Load fixture into the Profile Builder"))
            .add_filter(t("Fixture files"), &[PROFILE_EXT, "json", "qxf"])
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
            .set_title(t("Export as OFL file"))
            .add_filter(t("Open Fixture Library (JSON)"), &["json"])
            .set_file_name(name)
            .save_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

/// Loads a fixture file as an (unsaved) draft.
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
        // OFL folder structure: <manufacturer>/<fixture>.json – names from the
        // library
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
    // Imported fixtures are always created anew
    p.key.clear();
    Ok(p)
}

// ------------------------------------------------------------ Texts

fn kind_label(k: CapabilityKind) -> &'static str {
    use CapabilityKind as C;
    match k {
        C::NoFunction => t("No function"),
        C::Intensity => t("Intensity"),
        C::ColorIntensity => t("Color intensity"),
        C::ColorPreset => t("Color preset"),
        C::ColorTemperature => t("Color temperature"),
        C::ShutterStrobe => t("Shutter / strobe"),
        C::StrobeSpeed => t("Strobe speed"),
        C::Pan => t("Pan"),
        C::PanContinuous => t("Pan continuous"),
        C::Tilt => t("Tilt"),
        C::TiltContinuous => t("Tilt continuous"),
        C::PanTiltSpeed => t("Pan/tilt speed"),
        C::WheelSlot => t("Wheel slot"),
        C::WheelRotation => t("Wheel rotation"),
        C::Effect => t("Effect"),
        C::EffectSpeed => t("Effect speed"),
        C::Prism => t("Prism"),
        C::PrismRotation => t("Prism rotation"),
        C::Focus => t("Focus"),
        C::Zoom => t("Zoom"),
        C::Iris => t("Iris"),
        C::Frost => t("Frost"),
        C::Fog => t("Fog"),
        C::FogOutput => t("Fog output"),
        C::Rotation => t("Rotation"),
        C::Speed => t("Speed"),
        C::Maintenance => t("Maintenance"),
        C::Generic => t("Generic"),
    }
}

fn property_label(prop: &str) -> (&'static str, &'static str) {
    match prop {
        "brightness" => (t("Intensity"), t("e.g. off, dark, bright or 0%, 100%")),
        "colorTemperature" => (t("Color temperature"), t("e.g. warm, cold or 3200K, 5600K")),
        "angle" => (t("Angle"), t("e.g. 0deg, 540deg or narrow, wide")),
        "speed" => (t("Speed"), t("e.g. slow, fast, stop or 1Hz, 20Hz")),
        "distance" => (t("Distance"), t("e.g. near, far or 1m, 10m")),
        "openPercent" => (t("Opening"), t("e.g. closed, open or 0%, 100%")),
        "frostIntensity" => (t("Strength"), t("e.g. 0%, 100%")),
        _ => (t("Amount"), t("e.g. weak, strong or 0%, 100%")),
    }
}

fn shutter_label(s: ShutterEffect) -> &'static str {
    match s {
        ShutterEffect::Open => t("Open"),
        ShutterEffect::Closed => t("Closed"),
        ShutterEffect::Strobe => t("Strobe"),
        ShutterEffect::Pulse => t("Pulse"),
        ShutterEffect::RampUp => t("Ramp up"),
        ShutterEffect::RampDown => t("Ramp down"),
        ShutterEffect::RampUpDown => t("Ramp up and down"),
        ShutterEffect::Lightning => t("Lightning"),
        ShutterEffect::Spikes => t("Spikes"),
        ShutterEffect::Burst => t("Burst"),
    }
}

fn slot_label(s: SlotKind) -> &'static str {
    match s {
        SlotKind::Open => t("Open"),
        SlotKind::Color => t("Color"),
        SlotKind::Gobo => t("Gobo"),
        SlotKind::Prism => t("Prism"),
        SlotKind::Frost => t("Frost"),
    }
}

fn color_label(c: &str) -> &'static str {
    match c {
        "Red" => t("Red"),
        "Green" => t("Green"),
        "Blue" => t("Blue"),
        "White" => t("White"),
        "Warm White" => t("Warm white"),
        "Cold White" => t("Cold white"),
        "Amber" => "Amber",
        "UV" => t("UV"),
        "Cyan" => t("Cyan"),
        "Magenta" => t("Magenta"),
        "Yellow" => t("Yellow"),
        "Lime" => t("Lime"),
        "Indigo" => t("Indigo"),
        _ => t("Choose color"),
    }
}

fn category_label(c: &str) -> &'static str {
    match c {
        "Barrel Scanner" => t("Barrel scanner"),
        "Blinder" => t("Blinder"),
        "Color Changer" => t("Color changer"),
        "Dimmer" => t("Dimmer"),
        "Effect" => t("Effect"),
        "Fan" => t("Fan"),
        "Flower" => t("Flower"),
        "Hazer" => t("Hazer"),
        "Laser" => t("Laser"),
        "Matrix" => t("Matrix"),
        "Moving Head" => t("Moving head"),
        "Pixel Bar" => t("Pixel bar"),
        "Scanner" => t("Scanner"),
        "Smoke" => t("Smoke machine"),
        "Stand" => t("Stand"),
        "Strobe" => t("Strobe"),
        _ => t("Other"),
    }
}
