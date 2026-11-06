//! Vollbild umschalten.
//!
//! Unter Windows übernimmt winit bei randlosen Fenstern (ohne Titelleiste des
//! Systems) die neue Größe im Vollbild nicht zuverlässig – es bleibt ein
//! schwarzer Streifen. Dort legen wir das Fenster daher selbst über die
//! Win32-API auf die Fläche des aktuellen Monitors und stellen es danach
//! wieder her. Auf den anderen Systemen genügt der normale egui-Befehl.

use eframe::egui;

#[derive(Default)]
pub struct Fullscreen {
    #[cfg(windows)]
    saved: Option<win::Saved>,
    #[cfg(not(windows))]
    on: bool,
}

impl Fullscreen {
    pub fn is_on(&self, ctx: &egui::Context) -> bool {
        #[cfg(windows)]
        {
            let _ = ctx;
            self.saved.is_some()
        }
        #[cfg(not(windows))]
        {
            self.on || ctx.input(|i| i.viewport().fullscreen.unwrap_or(false))
        }
    }

    pub fn toggle(&mut self, ctx: &egui::Context, frame: &eframe::Frame) {
        #[cfg(windows)]
        {
            use raw_window_handle::{HasWindowHandle, RawWindowHandle};
            let Ok(handle) = frame.window_handle() else {
                return;
            };
            let RawWindowHandle::Win32(h) = handle.as_raw() else {
                return;
            };
            let hwnd = h.hwnd.get() as win::Hwnd;
            match self.saved.take() {
                Some(saved) => win::leave(hwnd, saved),
                None => self.saved = win::enter(hwnd),
            }
            ctx.request_repaint();
        }
        #[cfg(not(windows))]
        {
            let _ = frame;
            self.on = !self.is_on(ctx);
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.on));
        }
    }
}

#[cfg(windows)]
mod win {
    use windows_sys::Win32::Foundation::{HWND, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST,
    };
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetWindowRect, IsZoomed, SetWindowPos, ShowWindow, HWND_TOP, SWP_FRAMECHANGED,
        SWP_NOOWNERZORDER, SW_MAXIMIZE, SW_RESTORE,
    };

    pub type Hwnd = HWND;

    pub struct Saved {
        rect: RECT,
        maximized: bool,
    }

    pub fn enter(hwnd: HWND) -> Option<Saved> {
        // SAFETY: hwnd stammt von eframe und ist das gültige Hauptfenster;
        // alle Strukturen sind korrekt initialisiert.
        unsafe {
            let maximized = IsZoomed(hwnd) != 0;
            if maximized {
                // Aus dem maximierten Zustand heraus, sonst begrenzt Windows
                // das Fenster auf den Arbeitsbereich ohne Taskleiste.
                ShowWindow(hwnd, SW_RESTORE);
            }
            let mut rect: RECT = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut rect) == 0 {
                return None;
            }
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut info: MONITORINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(monitor, &mut info) == 0 {
                return None;
            }
            let m = info.rcMonitor;
            SetWindowPos(
                hwnd,
                HWND_TOP,
                m.left,
                m.top,
                m.right - m.left,
                m.bottom - m.top,
                SWP_FRAMECHANGED | SWP_NOOWNERZORDER,
            );
            Some(Saved { rect, maximized })
        }
    }

    pub fn leave(hwnd: HWND, saved: Saved) {
        let r = saved.rect;
        // SAFETY: wie oben.
        unsafe {
            SetWindowPos(
                hwnd,
                HWND_TOP,
                r.left,
                r.top,
                r.right - r.left,
                r.bottom - r.top,
                SWP_FRAMECHANGED | SWP_NOOWNERZORDER,
            );
            if saved.maximized {
                ShowWindow(hwnd, SW_MAXIMIZE);
            }
        }
    }
}
