//! Eigene Titelleiste (das Fenster hat keine Betriebssystem-Leiste):
//! Menü und Modus links, Programm- und Showname in der Mitte, Statistik und
//! Fensterknöpfe rechts. Leere Fläche: Ziehen = Fenster bewegen,
//! Doppelklick = maximieren.

use dmxcraft_output::OutputKind;
use eframe::egui::{self, Color32, Sense, ViewportCommand};

use crate::state::Mode;
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, Segmented};

pub const HEIGHT: f32 = 36.0;

/// Was die Titelleiste zum Zeichnen braucht.
pub struct TitleInfo<'a> {
    pub logo: Option<egui::TextureId>,
    pub show_name: &'a str,
    /// Ausgangsstatus als Text und Farbe (für den Punkt und den Tooltip).
    pub status: (String, Color32),
    /// Auslastung des Ausgabe-Takts 0..1.
    pub load: f32,
    pub fps: f32,
    pub locked: bool,
    pub output: OutputKind,
    pub blackout: bool,
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
    DemoShow,
    OpenShow,
    SaveShow,
    SaveShowAs,
    Quit,
    SetMode(Mode),
    SceneEditor,
    Zones,
    ToggleFullscreen,
    Zoom(f32),
    ReleaseAll,
    ToggleBlackout,
    ClearProgrammer,
    Output(OutputKind),
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
        "DmxCraft".into(),
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
        if let Some(tex) = info.logo {
            ui.add(egui::Image::new(egui::load::SizedTexture::new(
                tex,
                [22.0, 22.0],
            )));
        }
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
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(220.0);
        let mut item = |ui: &mut egui::Ui, label: &str, hint: &str, a: MenuAction| {
            // Bei gesperrter Oberfläche nur Wiedergabe, Ansicht, Speichern, Entsperren und Hilfe
            let allowed = free
                || matches!(
                    a,
                    MenuAction::SaveShow
                        | MenuAction::Quit
                        | MenuAction::ReleaseAll
                        | MenuAction::ToggleBlackout
                        | MenuAction::ClearProgrammer
                        | MenuAction::ToggleFullscreen
                        | MenuAction::Zoom(_)
                        | MenuAction::SetMode(Mode::Control)
                        | MenuAction::ToggleLock
                        | MenuAction::CheckUpdates
                        | MenuAction::InstallUpdate
                        | MenuAction::About
                );
            let b = egui::Button::new(label).shortcut_text(hint);
            if ui.add_enabled(allowed, b).clicked() {
                actions.push(a);
                ui.close();
            }
        };
        ui.menu_button("Datei", |ui| {
            item(ui, "Neue Show", "", MenuAction::NewShow);
            item(ui, "Demo-Show laden", "", MenuAction::DemoShow);
            ui.separator();
            item(ui, "Öffnen …", "Strg+O", MenuAction::OpenShow);
            item(ui, "Speichern", "Strg+S", MenuAction::SaveShow);
            item(ui, "Speichern unter …", "", MenuAction::SaveShowAs);
            ui.separator();
            item(ui, "Beenden", "", MenuAction::Quit);
        });
        ui.menu_button("Bearbeiten", |ui| {
            item(
                ui,
                "Einrichten (Patch & Bühnenplan)",
                "",
                MenuAction::SetMode(Mode::Setup),
            );
            item(ui, "Steuern", "", MenuAction::SetMode(Mode::Control));
            ui.separator();
            item(ui, "Szenen bearbeiten …", "", MenuAction::SceneEditor);
            item(ui, "Zonen & Gruppen …", "", MenuAction::Zones);
        });
        ui.menu_button("Wiedergabe", |ui| {
            item(ui, "Alle Szenen aus", "", MenuAction::ReleaseAll);
            let bo = if info.blackout {
                "Blackout aus"
            } else {
                "Blackout an"
            };
            item(ui, bo, "B", MenuAction::ToggleBlackout);
            item(ui, "Programmer leeren", "", MenuAction::ClearProgrammer);
        });
        ui.menu_button("Ausgang", |ui| {
            for k in OutputKind::ALL {
                let label = if k == info.output {
                    format!("{}  (aktiv)", k.label())
                } else {
                    k.label().to_string()
                };
                item(ui, &label, "", MenuAction::Output(k));
            }
        });
        ui.menu_button("Ansicht", |ui| {
            item(ui, "Vollbild", "F11", MenuAction::ToggleFullscreen);
            ui.separator();
            item(ui, "Größer", "Strg++", MenuAction::Zoom(0.1));
            item(ui, "Kleiner", "Strg+-", MenuAction::Zoom(-0.1));
            item(ui, "Normale Größe", "Strg+0", MenuAction::Zoom(0.0));
        });
        ui.separator();
        let lock_label = if info.locked {
            "Entsperren …"
        } else {
            "Oberfläche sperren"
        };
        item(ui, lock_label, "", MenuAction::ToggleLock);
        item(ui, "Einstellungen …", "", MenuAction::Settings);
        ui.menu_button("Hilfe", |ui| {
            item(ui, "Nach Updates suchen", "", MenuAction::CheckUpdates);
            item(ui, "Über DmxCraft", "", MenuAction::About);
        });
    });
}

/// Fenstergröße an den Rändern ändern (randloses Fenster).
pub fn resize_handles(ctx: &egui::Context) {
    let maximized = ctx.input(|i| i.viewport().maximized.unwrap_or(false));
    let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
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
