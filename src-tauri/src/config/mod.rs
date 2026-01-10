//! Configuration and settings management.

mod settings;

pub use settings::{ShortcutSettings, UpdateChannel, UserSettings};

use tauri::{AppHandle, Runtime};

/// Gets whether close-to-tray is enabled.
pub fn get_close_to_tray<R: Runtime>(_app: &AppHandle<R>) -> bool {
    // TODO: Implement settings check
    true
}

/// Loads settings from persistent storage.
pub fn load<R: Runtime>(_app: &AppHandle<R>) -> Result<UserSettings, Box<dyn std::error::Error>> {
    // TODO: Implement settings loading from tauri-plugin-store
    Ok(UserSettings::default())
}

/// Saves settings to persistent storage.
pub fn save<R: Runtime>(
    _app: &AppHandle<R>,
    _settings: &UserSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement settings saving
    Ok(())
}
