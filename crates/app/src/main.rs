#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod demo;
mod desktop;
mod icon;
mod theme;
mod updater;
mod views;

fn main() -> eframe::Result {
    desktop::ensure_linux_menu_entry();
    let show_path = std::env::args().nth(1);
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DmxCraft")
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([900.0, 600.0])
            .with_icon(icon::window_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "DmxCraft",
        options,
        Box::new(move |_cc| Ok(Box::new(app::DmxCraftApp::new(show_path)))),
    )
}
