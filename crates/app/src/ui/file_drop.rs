//! Dragging files from Explorer/Finder into the window: where is the mouse
//! pointer? On Windows the window reports no pointer movement during the
//! drag, so the position is asked from the system there.

use eframe::egui::{self, Pos2};

/// Pointer position (window coordinates) while files hover over the window
/// or are being dropped.
pub fn pointer(ctx: &egui::Context) -> Option<Pos2> {
    #[cfg(windows)]
    if let Some(p) = os_cursor(ctx) {
        return Some(p);
    }
    ctx.input(|i| i.pointer.hover_pos().or(i.pointer.latest_pos()))
}

/// Are files currently hovering over the window?
pub fn hovering(ctx: &egui::Context) -> bool {
    ctx.input(|i| !i.raw.hovered_files.is_empty())
}

#[cfg(windows)]
fn os_cursor(ctx: &egui::Context) -> Option<Pos2> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: GetCursorPos only writes to the given point.
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
