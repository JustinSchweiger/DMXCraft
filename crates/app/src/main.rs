#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod demo;
mod desktop;
mod file_dialog;
mod history;
mod icon;
mod lock;
mod settings;
mod state;
mod titlebar;
mod ui;
mod updater;
mod views;

fn main() -> eframe::Result {
    desktop::ensure_linux_menu_entry();
    let settings = settings::Settings::load();
    let show_path = std::env::args().nth(1).or_else(|| {
        settings
            .open_last_show
            .then(|| settings.last_show.clone())
            .flatten()
    });
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DMXCraft")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([900.0, 600.0])
            .with_icon(icon::window_icon())
            // eigene Titelleiste statt der des Betriebssystems
            .with_decorations(false)
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "DMXCraft",
        options,
        Box::new(move |_cc| Ok(Box::new(app::DmxCraftApp::new(show_path, settings)))),
    )
}
