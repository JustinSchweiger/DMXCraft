#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod demo;
mod desktop;
mod updater;
mod views;

fn main() -> eframe::Result {
    desktop::ensure_linux_menu_entry();
    let show_path = std::env::args().nth(1);
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("DmxCraft")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "DmxCraft",
        options,
        Box::new(move |_cc| Ok(Box::new(app::DmxCraftApp::new(show_path)))),
    )
}
