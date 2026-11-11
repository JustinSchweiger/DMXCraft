//! Windows: keep the keyboard focus on the active window.
//!
//! After some native dialogs or windows close (file dialogs, other
//! viewports), Windows can leave our window active but without keyboard
//! focus. Key presses then arrive as system keys (as if Alt were held), and
//! Windows plays its error sound for every key until the user switches
//! windows. Giving the focus back to the active window fixes that.

/// Call once per frame.
pub fn keep() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{
            GetActiveWindow, GetFocus, SetFocus,
        };
        // SAFETY: plain window-manager queries on the UI thread; SetFocus only
        // gets the active window of this thread.
        unsafe {
            let active = GetActiveWindow();
            if !active.is_null() && GetFocus().is_null() {
                SetFocus(active);
            }
        }
    }
}
