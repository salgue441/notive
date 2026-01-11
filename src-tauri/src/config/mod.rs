//! Configuration and settings management.

mod settings;

#[cfg(test)]
mod tests;

pub use settings::{ShortcutSettings, UpdateChannel, UserSettings};

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const SETTINGS_STORE_PATH: &str = "settings.json";

/// Gets whether close-to-tray is enabled.
pub fn get_close_to_tray<R: Runtime>(app: &AppHandle<R>) -> bool {
    match load(app) {
        Ok(settings) => settings.close_to_tray,
        Err(_) => true, // Default to true if loading fails
    }
}

/// Loads settings from persistent storage.
pub fn load<R: Runtime>(app: &AppHandle<R>) -> Result<UserSettings, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, SETTINGS_STORE_PATH, |store| {
        // Try to load settings from store
        if let Some(settings_json) = store.get("settings") {
            if let Some(settings) = settings_json.as_object() {
                match serde_json::from_value::<UserSettings>(serde_json::Value::Object(settings.clone())) {
                    Ok(settings) => {
                        log::debug!("Settings loaded from store");
                        return Ok(settings);
                    }
                    Err(e) => {
                        log::warn!("Failed to deserialize settings: {}", e);
                    }
                }
            }
        }
        
        // If no settings found or deserialization failed, return defaults
        log::debug!("Using default settings");
        Ok(UserSettings::default())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Saves settings to persistent storage.
pub fn save<R: Runtime>(
    app: &AppHandle<R>,
    settings: &UserSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, SETTINGS_STORE_PATH, |store| {
        // Serialize settings to JSON
        let settings_json = serde_json::to_value(settings)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        
        // Save to store
        store.insert("settings".to_string(), settings_json)?;
        
        // Persist to disk
        store.save()?;
        
        log::debug!("Settings saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
