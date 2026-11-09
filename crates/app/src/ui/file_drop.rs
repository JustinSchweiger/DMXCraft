//! Dateien aus Explorer/Finder ins Fenster ziehen: Wo befindet sich der
//! Mauszeiger? Unter Windows meldet das Fenster während des Ziehens keine
//! Zeigerbewegungen, daher wird dort die Position beim System erfragt.

use eframe::egui::{self, Pos2};

/// Zeigerposition (Fensterkoordinaten), solange Dateien über dem Fenster
/// schweben oder gerade abgelegt werden.
pub fn pointer(ctx: &egui::Context) -> Option<Pos2> {
    #[cfg(windows)]
    if let Some(p) = os_cursor(ctx) {
        return Some(p);
    }
    ctx.input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos()))
}

/// Schweben gerade Dateien über dem Fenster?
pub fn hovering(ctx: &egui::Context) -> bool {
    ctx.input(|i| !i.raw.hovered_files.is_empty())
}

#[cfg(windows)]
fn os_cursor(ctx: &egui::Context) -> Option<Pos2> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: GetCursorPos schreibt nur in den übergebenen Punkt.
    if unsafe { GetCursorPos(&mut p) } == 0 {
        return None;
    }
    let (inner, ppp) = ctx.input(|i| (i.viewport().inner_rect, i.pixels_per_point()));
    let inner = inner?;
    Some(egui::pos2(
        p.x as f32 / ppp - inner.min.x,
        p.y as f32 / ppp - inner.min.y,
    ))
}
