//! Custom title bar (the window has no operating system bar): menu and mode
//! on the left, program and show name in the middle, statistics and window
//! buttons on the right. Empty area: drag = move window, double-click =
//! maximize.

use crate::i18n::t;
use eframe::egui::{self, Color32, Sense, ViewportCommand};

use crate::state::Mode;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton, Segmented};

pub const HEIGHT: f32 = 36.0;

/// macOS: the window keeps its native buttons (traffic lights) at the top
/// left, the own window buttons and resize edges are not needed.
pub const NATIVE_FRAME: bool = cfg!(target_os = "macos");
/// macOS: room for the traffic lights left of the menu.
const TRAFFIC_LIGHTS_W: f32 = 72.0;

/// What the title bar needs for drawing.
pub struct TitleInfo<'a> {
    pub show_name: &'a str,
    /// Output status as text and color (for the dot and the tooltip).
    pub status: (String, Color32),
    /// Load of the output clock 0..1.
    pub load: f32,
    pub fps: f32,
    pub locked: bool,
    pub output: dmxcraft_output::OutputKind,
    pub blackout: bool,
    /// Recently opened shows (newest first).
    pub recent: &'a [String],
    pub can_undo: bool,
    pub can_redo: bool,
    /// Short-lived message (text, is error).
    pub message: Option<(&'a str, bool)>,
    pub update: UpdateBadge,
    /// Active mapping mode.
    pub mapping: Option<dmxcraft_core::mapping::MapKind>,
    /// Sound master (volume of the audio blocks).
    pub audio: AudioInfo,
    /// Number of missing audio files (red exclamation mark).
    pub missing_audio: usize,
    /// OBS connection (only when enabled): connected?
    pub obs: Option<bool>,
    /// Tabs shown in the title bar (in this order).
    pub tabs: &'a [Mode],
}

/// State of the sound master for the title bar.
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioInfo {
    pub master: f32,
    pub mute: bool,
    /// MIDI fader waiting to catch up.
    pub pickup: bool,
    pub fading: bool,
    pub fade_ms: u32,
    /// Maximum volume 0–1 (sound master 100 % corresponds to this value).
    pub limit: f32,
}

/// Update notice in the title bar.
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateBadge {
    None,
    Available(String),
    Busy(&'static str),
    Installed(String),
    InstallerStarted(String),
    Error(String),
}

/// Menu commands executed by the app frame.
#[derive(Clone, Debug, PartialEq)]
pub enum MenuAction {
    NewShow,
    OpenShow,
    OpenRecent(String),
    SaveShow,
    SaveShowAs,
    Undo,
    Redo,
    Quit,
    ToggleFullscreen,
    Zoom(f32),
    ToggleBlackout,
    DmxLevels,
    /// Open/close the 3D visualizer in its own window.
    Visualizer,
    /// Show (true) or hide a tab in the title bar.
    ShowTab(Mode, bool),
    ProfileBuilder,
    Settings,
    CheckUpdates,
    InstallUpdate,
    ToggleLock,
    /// Set the sound master.
    AudioMaster(f32),
    AudioMute,
    AudioFade,
    /// “Missing audio files” window.
    MissingAudio,
    /// Settings, “OBS” tab.
    Obs,
    /// Keyboard or MIDI mapping on/off.
    Mapping(dmxcraft_core::mapping::MapKind),
    About,
}

pub fn show(ui: &mut egui::Ui, info: &TitleInfo<'_>, mode: &mut Mode) -> Vec<MenuAction> {
    if info.locked && *mode == Mode::Setup {
        *mode = Mode::Control;
    }
    let mut actions = Vec::new();
    let full = ui.max_rect();
    // Background area for dragging (created first so buttons lie on top)
    let drag = ui.interact(
        full,
        ui.scope_id().with("titlebar_drag"),
        Sense::click_and_drag(),
    );
    if drag.drag_started() {
        ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
    }
    let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
    if drag.double_clicked() {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
    }

    // Middle: program and show name
    let painter = ui.painter();
    let center = full.center();
    let name = egui::text::LayoutJob::simple_singleline(
        "DMXCraft".into(),
        egui::FontId::proportional(theme::FS_TITLE),
        theme::text(),
    );
    let g1 = painter.layout_job(name);
    let g2 = painter.layout_no_wrap(
        format!("  ·  {}", info.show_name),
        theme::body(),
        theme::text_dim(),
    );
    let w = g1.size().x + g2.size().x;
    let x0 = center.x - w / 2.0;
    // A status message on the right can reach the middle – while it is shown,
    // the title steps back instead of overlapping it.
    if info.message.is_none() {
        painter.galley(
            egui::pos2(x0, center.y - g1.size().y / 2.0),
            g1.clone(),
            theme::text(),
        );
        painter.galley(
            egui::pos2(x0 + g1.size().x, center.y - g2.size().y / 2.0),
            g2,
            theme::text_dim(),
        );
    }

    ui.horizontal_centered(|ui| {
        ui.add_space(6.0);
        // the traffic lights are hidden in native fullscreen
        if NATIVE_FRAME && !ui.input(|i| i.viewport().fullscreen.unwrap_or(false)) {
            ui.add_space(TRAFFIC_LIGHTS_W);
        }
        main_menu(ui, info, &mut actions);
        ui.add_space(theme::SP_XS);
        let tabs: Vec<(Mode, &str)> = info
            .tabs
            .iter()
            .filter(|m| !(info.locked && m.needs_unlock()))
            .map(|&m| (m, t(m.label())))
            .collect();
        let (_, items) = Segmented::new(&tabs).show_items(ui, mode);
        // Right-click on a tab: hide it (shown again via View → Tabs)
        let last = info.tabs.len() <= 1;
        for (m, r) in items {
            menu::context(&r, |ui| {
                menu::begin(ui);
                if menu::item(
                    ui,
                    MenuItem::new(&tr!("Hide “{tab}”", tab = t(m.label())))
                        .icon(Icon::EyeOff)
                        .enabled(!last),
                ) {
                    actions.push(MenuAction::ShowTab(m, false));
                }
            });
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if !NATIVE_FRAME {
                window_buttons(ui, maximized, &mut actions);
            }
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            ui.add_space(theme::SP_M);
            if info.blackout {
                ui.label(
                    egui::RichText::new(t("BLACKOUT"))
                        .strong()
                        .color(theme::DANGER),
                );
            }
            match &info.update {
                UpdateBadge::None => {}
                UpdateBadge::Available(v) => {
                    let b =
                        egui::Button::new(tr!("Update to v{v}", v = v)).fill(theme::accent_dim());
                    if ui
                        .add(b)
                        .on_hover_text(t("Download new version from GitHub"))
                        .clicked()
                    {
                        actions.push(MenuAction::InstallUpdate);
                    }
                }
                UpdateBadge::Busy(t) => {
                    ui.label(egui::RichText::new(*t).color(theme::text_dim()));
                }
                UpdateBadge::Installed(v) => {
                    ui.label(
                        egui::RichText::new(tr!("v{v} installed – restart", v = v))
                            .color(theme::OK),
                    );
                }
                UpdateBadge::InstallerStarted(v) => {
                    ui.label(
                        egui::RichText::new(tr!(
                            "Installer for v{v} started – close DMXCraft",
                            v = v
                        ))
                        .color(theme::OK),
                    );
                }
                UpdateBadge::Error(e) => {
                    if ui
                        .small_button(t("Update error"))
                        .on_hover_text(tr!("{e}\nClick to try again", e = e))
                        .clicked()
                    {
                        actions.push(MenuAction::CheckUpdates);
                    }
                }
            }
            let (icon, tip) = if info.locked {
                (Icon::Locked, t("Unlock interface"))
            } else {
                (Icon::Unlocked, t("Lock interface (password)"))
            };
            if ui
                .add(IconButton::new(icon).size(28.0).flat().active(info.locked))
                .on_hover_text(tip)
                .clicked()
            {
                actions.push(MenuAction::ToggleLock);
            }
            if !info.locked {
                use dmxcraft_core::mapping::MapKind;
                for (kind, icon, tip) in [
                    (MapKind::Midi, Icon::Midi, t("MIDI mapping")),
                    (MapKind::Keyboard, Icon::Keyboard, t("Keyboard mapping")),
                ] {
                    let on = info.mapping == Some(kind);
                    let mut b = IconButton::new(icon).size(28.0).flat().active(on);
                    if on {
                        b = b.tint(crate::mapping::color(kind));
                    }
                    if ui.add(b).on_hover_text(tip).clicked() {
                        actions.push(MenuAction::Mapping(kind));
                    }
                }
            }
            ui.add_space(theme::SP_XS);
            load_meter(ui, info);
            ui.add_space(theme::SP_M);
            audio_control(ui, info.audio, &mut actions);
            if info.missing_audio > 0 {
                missing_alert(ui, info.missing_audio, &mut actions);
            }
            if let Some(ok) = info.obs {
                ui.add_space(theme::SP_S);
                let color = if ok { theme::OK } else { theme::DANGER };
                let r = ui
                    .add(
                        egui::Button::new(egui::RichText::new("OBS").color(color).strong())
                            .frame(false),
                    )
                    .on_hover_text(if ok {
                        t("OBS connected")
                    } else {
                        t("OBS not connected")
                    });
                if r.clicked() {
                    actions.push(MenuAction::Obs);
                }
            }
            if let Some((msg, err)) = info.message {
                ui.separator();
                let color = if err { theme::DANGER } else { theme::text() };
                ui.label(egui::RichText::new(msg).color(color));
            }
        });
    });
    actions
}

/// Red blinking exclamation mark: audio files missing; click opens the list.
fn missing_alert(ui: &mut egui::Ui, n: usize, actions: &mut Vec<MenuAction>) {
    ui.add_space(theme::SP_S);
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), Sense::click());
    let t_s = ui.input(|i| i.time);
    let on = (t_s * 2.0).fract() < 0.6;
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(100));
    crate::ui::widgets::icons::paint_alert(
        ui.painter(),
        rect.shrink(3.0),
        if on || resp.hovered() {
            theme::DANGER
        } else {
            theme::DANGER.gamma_multiply(0.35)
        },
    );
    let tip = if n == 1 {
        t("1 audio file is missing").to_string()
    } else {
        crate::tr!("{n} audio files are missing", n = n)
    };
    let resp = resp.on_hover_text(tip);
    if resp.clicked() {
        actions.push(MenuAction::MissingAudio);
    }
}

/// Sound master: speaker (mute), slider, percent. Only DMXCraft's playback,
/// not the system volume.
fn audio_control(ui: &mut egui::Ui, a: AudioInfo, actions: &mut Vec<MenuAction>) {
    use dmxcraft_core::mapping::Control;
    // Right to left: percent, slider, icon
    let pct = if a.mute {
        t("muted").to_string()
    } else {
        format!("{:.0} %", a.master * a.limit * 100.0)
    };
    let color = if a.mute {
        theme::DANGER
    } else if a.master < 0.995 {
        theme::WARN
    } else {
        theme::text_dim()
    };
    let (prect, _) = ui.allocate_exact_size(egui::vec2(40.0, 18.0), Sense::hover());
    ui.painter().text(
        prect.left_center(),
        egui::Align2::LEFT_CENTER,
        pct,
        theme::heading_font(),
        color,
    );

    // Slider
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(90.0, 18.0), Sense::click_and_drag());
    let overlay = crate::mapping::target(ui, rect, Control::AudioMaster);
    let track = egui::Rect::from_center_size(rect.center(), egui::vec2(rect.width() - 10.0, 4.0));
    if !overlay {
        if let Some(pos) = resp
            .interact_pointer_pos()
            .filter(|_| resp.dragged() || resp.clicked())
        {
            let v = ((pos.x - track.left()) / track.width()).clamp(0.0, 1.0);
            actions.push(MenuAction::AudioMaster(v));
        }
        if resp.double_clicked() {
            actions.push(MenuAction::AudioMaster(1.0));
        }
        if resp.hovered() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);
            if scroll != 0.0 {
                actions.push(MenuAction::AudioMaster(
                    (a.master + scroll / 600.0).clamp(0.0, 1.0),
                ));
            }
        }
    }
    let p = ui.painter();
    p.rect_filled(track, 2.0, theme::raised());
    let fill_color = if a.mute {
        theme::text_dim()
    } else if a.fading {
        theme::WARN
    } else {
        theme::accent()
    };
    let x = track.left() + track.width() * a.master.clamp(0.0, 1.0);
    p.rect_filled(
        egui::Rect::from_min_max(track.min, egui::pos2(x, track.max.y)),
        2.0,
        fill_color,
    );
    p.circle_filled(
        egui::pos2(x, track.center().y),
        if resp.hovered() || resp.dragged() {
            6.0
        } else {
            5.0
        },
        theme::text(),
    );
    let mut tip = t("Sound master – only DMXCraft playback, not the system volume.\nDouble-click: 100 %, mouse wheel: fine adjustment").to_string();
    if a.limit < 0.995 {
        tip.push_str("\n\n");
        tip.push_str(&tr!(
            "Maximum volume {p} % (Settings > Sound)",
            p = format!("{:.0}", a.limit * 100.0)
        ));
    }
    if a.pickup {
        tip.push_str("\n\n");
        tip.push_str(t(
            "Reset: a MIDI fader only takes effect again once it reaches the current value.",
        ));
    }
    resp.on_hover_text(tip);

    // Speaker: click mutes, right-click: fade out / 100 %
    let icon = if a.mute {
        Icon::SpeakerMute
    } else if a.master <= 0.0 {
        Icon::SpeakerNone
    } else if a.master < 0.5 {
        Icon::SpeakerLow
    } else {
        Icon::SpeakerHigh
    };
    let mut b = IconButton::new(icon).size(28.0).flat();
    if a.mute || a.master < 0.995 {
        b = b.active(true).tint(color);
    }
    let r = ui.add(b);
    let overlay = crate::mapping::target(ui, r.rect, Control::AudioMute);
    let r = r.on_hover_text(t("Mute sound (right-click: fade out)"));
    if !overlay && r.clicked() {
        actions.push(MenuAction::AudioMute);
    }
    menu::context(&r, |ui| {
        menu::begin(ui);
        if menu::item(
            ui,
            MenuItem::new(&tr!(
                "Fade out sound ({s} s)",
                s = if a.fade_ms.is_multiple_of(1000) {
                    (a.fade_ms / 1000).to_string()
                } else {
                    format!("{:.1}", a.fade_ms as f32 / 1000.0)
                }
            ))
            .enabled(!a.fading && a.master > 0.0),
        ) {
            actions.push(MenuAction::AudioFade);
        }
        if menu::item(ui, MenuItem::new(t("Set to 100 %"))) {
            actions.push(MenuAction::AudioMaster(1.0));
        }
    });
}

/// Load display: status dot of the output and bar.
fn load_meter(ui: &mut egui::Ui, info: &TitleInfo<'_>) {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(96.0, 18.0), Sense::hover());
    let p = ui.painter();
    p.circle_filled(
        egui::pos2(rect.left() + 5.0, rect.center().y),
        4.0,
        info.status.1,
    );
    p.text(
        egui::pos2(rect.left() + 14.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        t("LOAD"),
        theme::heading_font(),
        theme::text_dim(),
    );
    let bar = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 52.0, rect.center().y - 4.0),
        egui::pos2(rect.right(), rect.center().y + 4.0),
    );
    p.rect_filled(bar, 2.0, theme::raised());
    let l = info.load.clamp(0.0, 1.0);
    let color = if info.load > 0.85 {
        theme::DANGER
    } else if info.load > 0.6 {
        theme::WARN
    } else {
        theme::OK
    };
    if l > 0.0 {
        let fill = egui::Rect::from_min_size(
            bar.min,
            egui::vec2((bar.width() * l).max(2.0), bar.height()),
        );
        p.rect_filled(fill, 2.0, color);
    }
    resp.on_hover_text(tr!(
        "{status}\n{output} · {fps} fps · load {load} %",
        status = info.status.0,
        output = t(info.output.label()),
        fps = format!("{:.0}", info.fps),
        load = format!("{:.0}", info.load * 100.0),
    ));
}

fn main_menu(ui: &mut egui::Ui, info: &TitleInfo<'_>, actions: &mut Vec<MenuAction>) {
    let free = !info.locked;
    let resp = ui
        .add(IconButton::new(Icon::Menu).size(28.0).flat())
        .on_hover_text(t("Menu"));
    menu::dropdown(&resp, |ui| {
        menu::begin(ui);
        let mut act = |ui: &mut egui::Ui, item: MenuItem<'_>, a: MenuAction| {
            if menu::item(ui, item) {
                actions.push(a);
            }
        };
        menu::sub(ui, MenuItem::new(t("File")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("New"))
                    .icon(Icon::FileNew)
                    .shortcut(&keys::new())
                    .enabled(free),
                MenuAction::NewShow,
            );
            act(
                ui,
                MenuItem::new(t("Open …"))
                    .icon(Icon::FileOpen)
                    .shortcut(&keys::open())
                    .enabled(free),
                MenuAction::OpenShow,
            );
            menu::sub(
                ui,
                MenuItem::new(t("Open recent"))
                    .icon(Icon::Recent)
                    .enabled(free && !info.recent.is_empty()),
                |ui| {
                    for path in info.recent {
                        let name = std::path::Path::new(path)
                            .file_stem()
                            .map(|n| n.to_string_lossy().into_owned())
                            .unwrap_or_else(|| path.clone());
                        act(
                            ui,
                            MenuItem::new(&name).tooltip(path),
                            MenuAction::OpenRecent(path.clone()),
                        );
                    }
                },
            );
            menu::separator(ui);
            act(
                ui,
                MenuItem::new(t("Save"))
                    .icon(Icon::Save)
                    .shortcut(&keys::save()),
                MenuAction::SaveShow,
            );
            act(
                ui,
                MenuItem::new(t("Save as …"))
                    .icon(Icon::SaveAs)
                    .shortcut(&keys::save_as())
                    .enabled(free),
                MenuAction::SaveShowAs,
            );
        });
        menu::sub(ui, MenuItem::new(t("Edit")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Undo"))
                    .icon(Icon::Undo)
                    .shortcut(&keys::undo())
                    .enabled(free && info.can_undo),
                MenuAction::Undo,
            );
            act(
                ui,
                MenuItem::new(t("Redo"))
                    .icon(Icon::Redo)
                    .shortcut(&keys::redo())
                    .enabled(free && info.can_redo),
                MenuAction::Redo,
            );
            menu::separator(ui);
            let (lock_label, lock_icon) = if info.locked {
                (t("Unlock …"), Icon::Unlocked)
            } else {
                (t("Lock"), Icon::Locked)
            };
            act(
                ui,
                MenuItem::new(lock_label)
                    .icon(lock_icon)
                    .shortcut(&keys::lock()),
                MenuAction::ToggleLock,
            );
            act(
                ui,
                MenuItem::new(t("Settings …"))
                    .icon(Icon::Settings)
                    .shortcut(&keys::settings())
                    .enabled(free),
                MenuAction::Settings,
            );
        });
        menu::sub(ui, MenuItem::new(t("View")).plain(), |ui| {
            menu::title(ui, t("Tabs"));
            for m in Mode::TABS {
                let shown = info.tabs.contains(&m);
                act(
                    ui,
                    MenuItem::new(t(m.label()))
                        .checked(shown)
                        .enabled(!(shown && info.tabs.len() <= 1)),
                    MenuAction::ShowTab(m, !shown),
                );
            }
            menu::separator(ui);
            act(
                ui,
                MenuItem::new(t("Full screen"))
                    .icon(Icon::Fullscreen)
                    .shortcut("F11"),
                MenuAction::ToggleFullscreen,
            );
            menu::separator(ui);
            act(
                ui,
                MenuItem::new(t("Larger"))
                    .icon(Icon::ZoomIn)
                    .shortcut(&keys::zoom_in()),
                MenuAction::Zoom(0.1),
            );
            act(
                ui,
                MenuItem::new(t("Smaller"))
                    .icon(Icon::ZoomOut)
                    .shortcut(&keys::zoom_out()),
                MenuAction::Zoom(-0.1),
            );
            act(
                ui,
                MenuItem::new(t("Reset size"))
                    .icon(Icon::ZoomReset)
                    .shortcut(&keys::zoom_reset()),
                MenuAction::Zoom(0.0),
            );
        });
        menu::sub(ui, MenuItem::new(t("Tools")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("3D visualizer")).icon(Icon::Cube),
                MenuAction::Visualizer,
            );
            act(
                ui,
                MenuItem::new(t("DMX levels"))
                    .icon(Icon::Levels)
                    .shortcut(&keys::levels())
                    .enabled(free),
                MenuAction::DmxLevels,
            );
            act(
                ui,
                MenuItem::new(t("Profile Builder …"))
                    .icon(Icon::Builder)
                    .enabled(free),
                MenuAction::ProfileBuilder,
            );
        });
        menu::sub(ui, MenuItem::new(t("Mappings")).plain(), |ui| {
            use dmxcraft_core::mapping::MapKind;
            for (label, icon, kind) in [
                (t("Keyboard mapping"), Icon::Keyboard, MapKind::Keyboard),
                (t("MIDI mapping"), Icon::Midi, MapKind::Midi),
            ] {
                act(
                    ui,
                    MenuItem::new(label).icon(icon).enabled(free),
                    MenuAction::Mapping(kind),
                );
            }
        });
        menu::sub(ui, MenuItem::new(t("Help")).plain(), |ui| {
            act(
                ui,
                MenuItem::new(t("Check for updates")).icon(Icon::Update),
                MenuAction::CheckUpdates,
            );
            act(
                ui,
                MenuItem::new(t("About DMXCraft")).icon(Icon::Info),
                MenuAction::About,
            );
        });
    });
}

/// Labels of the keyboard shortcuts (Ctrl on Windows/Linux, Cmd on macOS).
pub mod keys {
    use crate::i18n::t;

    fn modifier() -> &'static str {
        if cfg!(target_os = "macos") {
            "Cmd"
        } else {
            t("Ctrl")
        }
    }

    fn k(rest: &str) -> String {
        format!("{}+{rest}", modifier())
    }

    pub fn new() -> String {
        k("N")
    }
    pub fn open() -> String {
        k("O")
    }
    pub fn save() -> String {
        k("S")
    }
    pub fn save_as() -> String {
        format!("{}+{}+S", modifier(), t("Shift"))
    }
    /// Clipboard shortcuts: “Ctrl+X” etc.
    pub fn clip(key: &str) -> String {
        k(key)
    }
    pub fn undo() -> String {
        k("Z")
    }
    pub fn redo() -> String {
        k("Y")
    }
    pub fn lock() -> String {
        k("L")
    }
    pub fn settings() -> String {
        k(",")
    }
    pub fn zoom_in() -> String {
        k("+")
    }
    pub fn zoom_out() -> String {
        k("-")
    }
    pub fn zoom_reset() -> String {
        k("0")
    }
    pub fn levels() -> String {
        k("D")
    }
    pub fn delete() -> &'static str {
        t("Del")
    }
}

/// Own minimize / maximize / close buttons (borderless window).
fn window_buttons(ui: &mut egui::Ui, maximized: bool, actions: &mut Vec<MenuAction>) {
    if ui
        .add(IconButton::new(Icon::Close).size(HEIGHT).flat().danger())
        .on_hover_text(t("Close"))
        .clicked()
    {
        actions.push(MenuAction::Quit);
    }
    let (icon, tip) = if maximized {
        (Icon::Restore, t("Restore"))
    } else {
        (Icon::Maximize, t("Maximize"))
    };
    if ui
        .add(IconButton::new(icon).size(HEIGHT).flat())
        .on_hover_text(tip)
        .clicked()
    {
        ui.ctx()
            .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
    }
    if ui
        .add(IconButton::new(Icon::Minimize).size(HEIGHT).flat())
        .on_hover_text(t("Minimize"))
        .clicked()
    {
        ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
    }
}

/// Resize the window at the edges (borderless window).
pub fn resize_handles(ctx: &egui::Context, fullscreen: bool) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if maximized || fullscreen || NATIVE_FRAME {
        return;
    }
    let rect = ctx.content_rect();
    let Some(p) = ctx.input(|i| i.pointer.hover_pos()) else {
        return;
    };
    const EDGE: f32 = 5.0;
    let (l, r, t, b) = (
        p.x - rect.left() < EDGE,
        rect.right() - p.x < EDGE,
        p.y - rect.top() < EDGE,
        rect.bottom() - p.y < EDGE,
    );
    use egui::ResizeDirection as D;
    let dir = match (l, r, t, b) {
        (true, _, true, _) => D::NorthWest,
        (_, true, true, _) => D::NorthEast,
        (true, _, _, true) => D::SouthWest,
        (_, true, _, true) => D::SouthEast,
        (true, _, _, _) => D::West,
        (_, true, _, _) => D::East,
        (_, _, true, _) => D::North,
        (_, _, _, true) => D::South,
        _ => return,
    };
    let icon = match dir {
        D::North | D::South => egui::CursorIcon::ResizeVertical,
        D::East | D::West => egui::CursorIcon::ResizeHorizontal,
        D::NorthWest | D::SouthEast => egui::CursorIcon::ResizeNwSe,
        D::NorthEast | D::SouthWest => egui::CursorIcon::ResizeNeSw,
    };
    ctx.set_cursor_icon(icon);
    if ctx.input(|i| i.pointer.primary_pressed()) {
        ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
    }
}
