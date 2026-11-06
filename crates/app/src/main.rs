#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod demo;
mod views;

fn main() -> eframe::Result {
    let show_path = std::env::args().nth(1);
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("OpenLicht")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([900.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "OpenLicht",
        options,
        Box::new(move |_cc| Ok(Box::new(app::OpenLichtApp::new(show_path)))),
    )
}
