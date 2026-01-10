//! Platform detection utilities.

/// Returns true if running on Wayland.
pub fn is_wayland() -> bool {
    std::env::var("WAYLAND_DISPLAY").is_ok()
}

/// Returns true if running on X11.
pub fn is_x11() -> bool {
    std::env::var("DISPLAY").is_ok() && !is_wayland()
}

/// Gets the current desktop environment.
pub fn desktop_environment() -> Option<String> {
    std::env::var("XDG_CURRENT_DESKTOP").ok()
}
