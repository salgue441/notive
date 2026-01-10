//! Settings management commands.

use crate::config::UserSettings;
use tauri::{AppHandle, Runtime};

/// Retrieves the current user settings.
#[tauri::command]
pub fn get_settings<R: Runtime>(_app: AppHandle<R>) -> Result<UserSettings, String> {
    // TODO: Implement settings retrieval from store
    log::debug!("Getting settings...");
    Ok(UserSettings::default())
}

/// Updates the user settings.
#[tauri::command]
pub async fn update_settings<R: Runtime>(
    _app: AppHandle<R>,
    _settings: UserSettings,
) -> Result<(), String> {
    // TODO: Implement settings update
    // - Save to store
    // - Apply changes (shortcuts, autostart, etc.)
    log::debug!("Updating settings...");
    Ok(())
}
