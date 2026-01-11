//! Privacy mode and data management.

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Privacy mode settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PrivacySettings {
    pub privacy_mode_enabled: bool,
    pub clear_history_on_close: bool,
    pub clear_cookies_on_close: bool,
    pub no_history_tracking: bool,
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self {
            privacy_mode_enabled: false,
            clear_history_on_close: false,
            clear_cookies_on_close: false,
            no_history_tracking: false,
        }
    }
}

/// Gets privacy settings.
#[tauri::command]
pub fn get_privacy_settings<R: Runtime>(app: AppHandle<R>) -> Result<PrivacySettings, String> {
    // Load from persistent storage
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        if let Some(value) = store.get("privacy_settings") {
            serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))
        } else {
            Ok(PrivacySettings::default())
        }
    })
    .map_err(|e| e.to_string())
}

/// Updates privacy settings.
#[tauri::command]
pub fn update_privacy_settings<R: Runtime>(
    app: AppHandle<R>,
    settings: PrivacySettings,
) -> Result<(), String> {
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        let value = serde_json::to_value(&settings)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("privacy_settings".to_string(), value)?;
        store.save()?;
        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// Clears all privacy-sensitive data.
#[tauri::command]
pub async fn clear_privacy_data<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::info!("Clearing privacy data...");
    
    // Clear history
    if let Err(e) = crate::history::clear_history(&app) {
        log::warn!("Failed to clear history: {}", e);
    }
    
    // Clear bookmarks (optional, based on settings)
    // Note: We don't clear bookmarks by default as they're user-created
    
    // Clear search history
    if let Err(e) = crate::search::clear_search_history(&app).await {
        log::warn!("Failed to clear search history: {}", e);
    }
    
    log::info!("Privacy data cleared");
    Ok(())
}

/// Checks if privacy mode is enabled.
#[tauri::command]
pub fn is_privacy_mode_enabled<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    let settings = get_privacy_settings(app)?;
    Ok(settings.privacy_mode_enabled)
}
