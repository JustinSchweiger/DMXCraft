//! Eigene Titelleiste (das Fenster hat keine Betriebssystem-Leiste):
//! Menü und Modus links, Programm- und Showname in der Mitte, Statistik und
//! Fensterknöpfe rechts. Leere Fläche: Ziehen = Fenster bewegen,
//! Doppelklick = maximieren.

use eframe::egui::{self, Color32, Sense, ViewportCommand};

use crate::state::Mode;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton, Segmented};

pub const HEIGHT: f32 = 36.0;

/// Was die Titelleiste zum Zeichnen braucht.
pub struct TitleInfo<'a> {
    pub show_name: &'a str,
    /// Ausgangsstatus als Text und Farbe (für den Punkt und den Tooltip).
    pub status: (String, Color32),
    /// Auslastung des Ausgabe-Takts 0..1.
    pub load: f32,
    pub fps: f32,
    pub locked: bool,
    pub output: dmxcraft_output::OutputKind,
    pub blackout: bool,
    /// Zuletzt geöffnete Shows (neueste zuerst).
    pub recent: &'a [String],
    pub can_undo: bool,
    pub can_redo: bool,
    /// Kurzzeitige Meldung (Text, ist Fehler).
    pub message: Option<(&'a str, bool)>,
    pub update: UpdateBadge,
}

/// Update-Hinweis in der Titelleiste.
#[derive(Clone, Debug, PartialEq)]
pub enum UpdateBadge {
    None,
    Available,
    Busy(&'static str),
    Installed(String),
    Error(String),
}

/// Menübefehle, die der App-Rahmen ausführt.
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
    Settings,
    CheckUpdates,
    InstallUpdate,
    ToggleLock,
    About,
}

pub fn show(ui: &mut egui::Ui, info: &TitleInfo<'_>, mode: &mut Mode) -> Vec<MenuAction> {
    if info.locked {
        *mode = Mode::Control;
    }
    let mut actions = Vec::new();
    let full = ui.max_rect();
    // Hintergrundfläche zum Ziehen (zuerst anlegen, damit Knöpfe darüber liegen)
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

    // Mitte: Programm- und Showname
    let painter = ui.painter();
    let center = full.center();
    let name = egui::text::LayoutJob::simple_singleline(
        "DMXCraft".into(),
        egui::FontId::proportional(theme::FS_TITLE),
        theme::TEXT,
    );
    let g1 = painter.layout_job(name);
    let g2 = painter.layout_no_wrap(
        format!("  ·  {}", info.show_name),
        theme::body(),
        theme::TEXT_DIM,
    );
    let w = g1.size().x + g2.size().x;
    let x0 = center.x - w / 2.0;
    painter.galley(
        egui::pos2(x0, center.y - g1.size().y / 2.0),
        g1.clone(),
        theme::TEXT,
    );
    painter.galley(
        egui::pos2(x0 + g1.size().x, center.y - g2.size().y / 2.0),
        g2,
        theme::TEXT_DIM,
    );

    ui.horizontal_centered(|ui| {
        ui.add_space(6.0);
        main_menu(ui, info, &mut actions);
        ui.add_space(theme::SP_XS);
        if info.locked {
            Segmented::new(&[(Mode::Control, "Steuern")]).show(ui, mode);
        } else {
            Segmented::new(&[(Mode::Setup, "Einrichten"), (Mode::Control, "Steuern")])
                .show(ui, mode);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            if ui
                .add(IconButton::new(Icon::Close).size(HEIGHT).flat().danger())
                .on_hover_text("Schließen")
                .clicked()
            {
                actions.push(MenuAction::Quit);
            }
            let (icon, tip) = if maximized {
                (Icon::Restore, "Wiederherstellen")
            } else {
                (Icon::Maximize, "Maximieren")
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
                .on_hover_text("Minimieren")
                .clicked()
            {
                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
            }
            ui.spacing_mut().item_spacing.x = theme::SP_S;
            ui.add_space(theme::SP_M);
            if info.blackout {
                ui.label(
                    egui::RichText::new("BLACKOUT")
                        .strong()
                        .color(theme::DANGER),
                );
            }
            match &info.update {
                UpdateBadge::None => {}
                UpdateBadge::Available => {
                    let b = egui::Button::new("Update installieren").fill(theme::ACCENT_DIM);
                    if ui
                        .add(b)
                        .on_hover_text("Neue Version von GitHub laden")
                        .clicked()
                    {
                        actions.push(MenuAction::InstallUpdate);
                    }
                }
                UpdateBadge::Busy(t) => {
                    ui.label(egui::RichText::new(*t).color(theme::TEXT_DIM));
                }
                UpdateBadge::Installed(v) => {
                    ui.label(
                        egui::RichText::new(format!("v{v} installiert – neu starten"))
                            .color(theme::OK),
                    );
                }
                UpdateBadge::Error(e) => {
                    if ui
                        .small_button("Update-Fehler")
                        .on_hover_text(format!("{e}\nKlicken für neuen Versuch"))
                        .clicked()
                    {
                        actions.push(MenuAction::CheckUpdates);
                    }
                }
            }
            let (icon, tip) = if info.locked {
                (Icon::Locked, "Oberfläche entsperren")
            } else {
                (Icon::Unlocked, "Oberfläche sperren (Passwort)")
            };
            if ui
                .add(IconButton::new(icon).size(28.0).flat().active(info.locked))
                .on_hover_text(tip)
                .clicked()
            {
                actions.push(MenuAction::ToggleLock);
            }
            ui.add_space(theme::SP_XS);
            load_meter(ui, info);
            if let Some((msg, err)) = info.message {
                ui.separator();
                let color = if err { theme::DANGER } else { theme::TEXT };
                ui.label(egui::RichText::new(msg).color(color));
            }
        });
    });
    actions
}

/// Auslastungsanzeige: Statuspunkt des Ausgangs und Balken.
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
        "LAST",
        theme::heading_font(),
        theme::TEXT_DIM,
    );
    let bar = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 52.0, rect.center().y - 4.0),
        egui::pos2(rect.right(), rect.center().y + 4.0),
    );
    p.rect_filled(bar, 2.0, theme::RAISED);
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
    resp.on_hover_text(format!(
        "{}\n{} · {:.0} fps · Auslastung {:.0} %",
        info.status.0,
        info.output.label(),
        info.fps,
        info.load * 100.0
    ));
}

fn main_menu(ui: &mut egui::Ui, info: &TitleInfo<'_>, actions: &mut Vec<MenuAction>) {
    let free = !info.locked;
    let resp = ui
        .add(IconButton::new(Icon::Menu).size(28.0).flat())
        .on_hover_text("Menü");
    menu::dropdown(&resp, |ui| {
        menu::begin(ui);
        let soon = "Noch nicht verfügbar";
        let mut act = |ui: &mut egui::Ui, item: MenuItem<'_>, a: MenuAction| {
            if menu::item(ui, item) {
                actions.push(a);
            }
        };
        menu::sub(ui, MenuItem::new("Datei").plain(), |ui| {
            act(
                ui,
                MenuItem::new("Neu")
                    .icon(Icon::FileNew)
                    .shortcut(&keys::NEW)
                    .enabled(free),
                MenuAction::NewShow,
            );
            act(
                ui,
                MenuItem::new("Öffnen …")
                    .icon(Icon::FileOpen)
                    .shortcut(&keys::OPEN)
                    .enabled(free),
                MenuAction::OpenShow,
            );
            menu::sub(
                ui,
                MenuItem::new("Zuletzt geöffnet")
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
                MenuItem::new("Speichern")
                    .icon(Icon::Save)
                    .shortcut(&keys::SAVE),
                MenuAction::SaveShow,
            );
            act(
                ui,
                MenuItem::new("Speichern unter …")
                    .icon(Icon::SaveAs)
                    .shortcut(&keys::SAVE_AS)
                    .enabled(free),
                MenuAction::SaveShowAs,
            );
            menu::separator(ui);
            ui.add(
                MenuItem::new("Importieren von Daslight …")
                    .icon(Icon::Import)
                    .enabled(false)
                    .tooltip(soon),
            );
        });
        menu::sub(ui, MenuItem::new("Bearbeiten").plain(), |ui| {
            act(
                ui,
                MenuItem::new("Rückgängig")
                    .icon(Icon::Undo)
                    .shortcut(&keys::UNDO)
                    .enabled(free && info.can_undo),
                MenuAction::Undo,
            );
            act(
                ui,
                MenuItem::new("Wiederherstellen")
                    .icon(Icon::Redo)
                    .shortcut(&keys::REDO)
                    .enabled(free && info.can_redo),
                MenuAction::Redo,
            );
            menu::separator(ui);
            let (lock_label, lock_icon) = if info.locked {
                ("Entsperren …", Icon::Unlocked)
            } else {
                ("Sperren", Icon::Locked)
            };
            act(
                ui,
                MenuItem::new(lock_label)
                    .icon(lock_icon)
                    .shortcut(&keys::LOCK),
                MenuAction::ToggleLock,
            );
            act(
                ui,
                MenuItem::new("Einstellungen …")
                    .icon(Icon::Settings)
                    .shortcut(&keys::SETTINGS)
                    .enabled(free),
                MenuAction::Settings,
            );
        });
        menu::sub(ui, MenuItem::new("Ansicht").plain(), |ui| {
            act(
                ui,
                MenuItem::new("Vollbild")
                    .icon(Icon::Fullscreen)
                    .shortcut("F11"),
                MenuAction::ToggleFullscreen,
            );
            menu::separator(ui);
            act(
                ui,
                MenuItem::new("Größer")
                    .icon(Icon::ZoomIn)
                    .shortcut(&keys::ZOOM_IN),
                MenuAction::Zoom(0.1),
            );
            act(
                ui,
                MenuItem::new("Kleiner")
                    .icon(Icon::ZoomOut)
                    .shortcut(&keys::ZOOM_OUT),
                MenuAction::Zoom(-0.1),
            );
            act(
                ui,
                MenuItem::new("Größe wiederherstellen")
                    .icon(Icon::ZoomReset)
                    .shortcut(&keys::ZOOM_RESET),
                MenuAction::Zoom(0.0),
            );
        });
        menu::sub(ui, MenuItem::new("Tools").plain(), |ui| {
            ui.add(
                MenuItem::new("3D-Visualisierer")
                    .icon(Icon::Cube)
                    .enabled(false)
                    .tooltip(soon),
            );
            act(
                ui,
                MenuItem::new("DMX-Werte")
                    .icon(Icon::Levels)
                    .shortcut(&keys::LEVELS)
                    .enabled(free),
                MenuAction::DmxLevels,
            );
            ui.add(
                MenuItem::new("Profile Builder")
                    .icon(Icon::Builder)
                    .enabled(false)
                    .tooltip(soon),
            );
        });
        menu::sub(ui, MenuItem::new("Mappings").plain(), |ui| {
            for (label, icon) in [
                ("Tastatur-Mapping", Icon::Keyboard),
                ("MIDI-Mapping", Icon::Midi),
                ("Touch-Mapping", Icon::Touch),
            ] {
                ui.add(MenuItem::new(label).icon(icon).enabled(false).tooltip(soon));
            }
        });
        menu::sub(ui, MenuItem::new("Hilfe").plain(), |ui| {
            act(
                ui,
                MenuItem::new("Nach Updates suchen").icon(Icon::Update),
                MenuAction::CheckUpdates,
            );
            act(
                ui,
                MenuItem::new("Über DMXCraft").icon(Icon::Info),
                MenuAction::About,
            );
        });
    });
}

/// Beschriftung der Tastenkürzel (Strg unter Windows/Linux, Cmd unter macOS).
pub mod keys {
    use std::sync::LazyLock;

    const MOD: &str = if cfg!(target_os = "macos") {
        "Cmd"
    } else {
        "Strg"
    };

    fn k(rest: &str) -> String {
        format!("{MOD}+{rest}")
    }

    pub static NEW: LazyLock<String> = LazyLock::new(|| k("N"));
    pub static OPEN: LazyLock<String> = LazyLock::new(|| k("O"));
    pub static SAVE: LazyLock<String> = LazyLock::new(|| k("S"));
    pub static SAVE_AS: LazyLock<String> = LazyLock::new(|| k("Umschalt+S"));
    pub static UNDO: LazyLock<String> = LazyLock::new(|| k("Z"));
    pub static REDO: LazyLock<String> = LazyLock::new(|| k("Y"));
    pub static LOCK: LazyLock<String> = LazyLock::new(|| k("L"));
    pub static SETTINGS: LazyLock<String> = LazyLock::new(|| k(","));
    pub static ZOOM_IN: LazyLock<String> = LazyLock::new(|| k("+"));
    pub static ZOOM_OUT: LazyLock<String> = LazyLock::new(|| k("-"));
    pub static ZOOM_RESET: LazyLock<String> = LazyLock::new(|| k("0"));
    pub static LEVELS: LazyLock<String> = LazyLock::new(|| k("D"));
}

/// Fenstergröße an den Rändern ändern (randloses Fenster).
pub fn resize_handles(ctx: &egui::Context, fullscreen: bool) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    if maximized || fullscreen {
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
