#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod i18n;
mod app;
mod audio;
mod audio_triggers;
mod demo;
mod desktop;
mod file_dialog;
mod fullscreen;
mod history;
mod icon;
mod lock;
mod mapping;
mod mcp;
mod midi;
mod midi_device;
mod missing_audio;
mod obs;
mod palettes;
mod profiles;
mod remote;
mod settings;
mod settings_window;
mod sound;
mod state;
mod titlebar;
mod ui;
mod updater;
mod views;
mod viz;

fn main() -> eframe::Result {
    // `dmxcraft mcp`: stdio-Brücke für KI-Assistenten (ohne Fenster)
    if std::env::args().nth(1).as_deref() == Some("mcp") {
        dmxcraft_mcp::bridge::run(settings::Settings::load().mcp_port);
        return Ok(());
    }
    desktop::ensure_linux_menu_entry();
    let settings = settings::Settings::load();
    i18n::set_lang(i18n::Lang::from_key(&settings.language));
    ui::theme::set_theme(ui::theme::Theme::from_key(&settings.theme));
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
        // Fenstergröße/-position und Größen der Bereiche (Panels) beim
        // nächsten Start wiederherstellen
        persist_window: true,
        ..Default::default()
    };
    let result = eframe::run_native(
        "DMXCraft",
        options,
        Box::new(move |_cc| Ok(Box::new(app::DmxCraftApp::new(show_path, settings)))),
    );
    // Fenster ist zu und alles gespeichert: Prozess sicher beenden, auch wenn
    // noch Hintergrund-Fäden (MCP-Verbindungen, Audio, Ausgabe) laufen
    if let Err(e) = &result {
        eprintln!("{e}");
        std::process::exit(1);
    }
    std::process::exit(0)
}
