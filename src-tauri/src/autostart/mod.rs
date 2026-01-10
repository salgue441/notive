//! Autostart management.

use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;

/// Enables autostart.
pub fn enable<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    app.autolaunch()
        .enable()
        .map_err(|e| format!("Failed to enable autostart: {}", e))
}

/// Disables autostart.
pub fn disable<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    app.autolaunch()
        .disable()
        .map_err(|e| format!("Failed to disable autostart: {}", e))
}

/// Checks if autostart is enabled.
pub fn is_enabled<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|e| format!("Failed to check autostart: {}", e))
}
