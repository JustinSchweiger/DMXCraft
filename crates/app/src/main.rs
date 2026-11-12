#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
mod i18n;
mod app;
mod audio;
mod audio_triggers;
mod autosave;
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
mod video_out;
mod views;
mod viz;

fn main() -> eframe::Result {
    // `dmxcraft mcp`: stdio bridge for AI assistants (without a window)
    if std::env::args().nth(1).as_deref() == Some("mcp") {
        dmxcraft_mcp::bridge::run(settings::Settings::load().mcp_port);
        return Ok(());
    }
    desktop::ensure_linux_menu_entry();
    let settings = settings::Settings::load();
    i18n::set_lang(i18n::Lang::from_key(&settings.language));
    ui::theme::set_theme(ui::theme::Theme::from_key(&settings.theme));
    ui::theme::set_overrides(settings.theme_overrides());
    let show_path = std::env::args().nth(1).or_else(|| {
        settings
            .open_last_show
            .then(|| settings.last_show.clone())
            .flatten()
    });
    let viewport = eframe::egui::ViewportBuilder::default()
        .with_title("DMXCraft")
        .with_inner_size([1440.0, 900.0])
        .with_min_inner_size([900.0, 600.0])
        .with_icon(icon::window_icon())
        .with_resizable(true);
    // Own title bar instead of the operating system's. On macOS the window
    // keeps its native frame (traffic lights, native fullscreen, resizing)
    // with the content drawn behind a transparent title bar.
    #[cfg(target_os = "macos")]
    let viewport = viewport
        .with_fullsize_content_view(true)
        .with_titlebar_shown(false)
        .with_title_shown(false);
    #[cfg(not(target_os = "macos"))]
    let viewport = viewport.with_decorations(false);
    let options = eframe::NativeOptions {
        viewport,
        // restore window size/position and the sizes of the areas (panels) at the
        // next start
        persist_window: true,
        ..Default::default()
    };
    let result = eframe::run_native(
        "DMXCraft",
        options,
        Box::new(move |cc| {
            ui::system_fonts::install(&cc.egui_ctx);
            Ok(Box::new(app::DmxCraftApp::new(show_path, settings)))
        }),
    );
    // Window is closed and everything is saved: end the process safely, even
    // if background threads (MCP connections, audio, output) are still running
    if let Err(e) = &result {
        eprintln!("{e}");
        std::process::exit(1);
    }
    std::process::exit(0)
}
