//! Eigene Titelleiste (das Fenster hat keine Betriebssystem-Leiste):
//! Menü und Modus links, Programm- und Showname in der Mitte, Statistik und
//! Fensterknöpfe rechts. Leere Fläche: Ziehen = Fenster bewegen,
//! Doppelklick = maximieren.

use dmxcraft_output::OutputKind;
use eframe::egui::{self, Color32, Sense, ViewportCommand};

use crate::state::{Mode, SetupTab};
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton, Segmented};

pub const HEIGHT: f32 = 36.0;

/// Was die Titelleiste zum Zeichnen braucht.
pub struct TitleInfo<'a> {
    pub logo: Option<egui::TextureId>,
    pub show_name: &'a str,
    pub status: (String, Color32),
    pub stats: String,
    pub output: OutputKind,
    pub blackout: bool,
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
    Setup(SetupTab),
    ToggleFullscreen,
    Zoom(f32),
    ReleaseAll,
    ToggleBlackout,
    ClearProgrammer,
    Output(OutputKind),
    Settings,
    CheckUpdates,
    About,
}

pub fn show(ui: &mut egui::Ui, info: &TitleInfo<'_>, mode: &mut Mode) -> Vec<MenuAction> {
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
        Segmented::new(&[(Mode::Setup, "Einrichten"), (Mode::Control, "Steuern")]).show(ui, mode);

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
            ui.label(egui::RichText::new(&info.stats).color(theme::TEXT_DIM));
            ui.label(egui::RichText::new(&info.status.0).color(info.status.1))
                .on_hover_text(format!("Ausgang: {}", info.output.label()));
        });
    });
    actions
}

fn main_menu(ui: &mut egui::Ui, info: &TitleInfo<'_>, actions: &mut Vec<MenuAction>) {
    let resp = ui
        .add(IconButton::new(Icon::Menu).size(28.0).flat())
        .on_hover_text("Menü");
    egui::Popup::menu(&resp).show(|ui| {
        ui.set_min_width(220.0);
        let mut item = |ui: &mut egui::Ui, label: &str, hint: &str, a: MenuAction| {
            let b = egui::Button::new(label).shortcut_text(hint);
            if ui.add(b).clicked() {
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
        ui.menu_button("Einrichten", |ui| {
            item(ui, "Patch", "", MenuAction::Setup(SetupTab::Patch));
            item(ui, "Bühnenplan", "", MenuAction::Setup(SetupTab::Stage));
            item(ui, "Szenen", "", MenuAction::Setup(SetupTab::Scenes));
            item(
                ui,
                "Zonen & Gruppen",
                "",
                MenuAction::Setup(SetupTab::Zones),
            );
            item(ui, "DMX-Adressen", "", MenuAction::Setup(SetupTab::Monitor));
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
