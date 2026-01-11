//! Settings sync and backup functionality.

#[cfg(test)]
mod tests;

use crate::config;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
use tauri_plugin_fs::FsExt;

/// Exports settings to a JSON file.
#[tauri::command]
pub async fn export_settings<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    log::debug!("Exporting settings...");
    
    // Load current settings
    let settings = config::load(&app).map_err(|e| e.to_string())?;
    
    // Serialize to JSON
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    // Show save dialog
    let file_path = app
        .dialog()
        .file()
        .set_file_name("notive-settings.json")
        .add_filter("JSON", &["json"])
        .save()
        .await;
    
    match file_path {
        Some(path) => {
            // Write to file
            app.fs()
                .write_file(&path, json.as_bytes())
                .await
                .map_err(|e| format!("Failed to write file: {}", e))?;
            
            log::info!("Settings exported to: {:?}", path);
            Ok(format!("Settings exported to: {}", path.display()))
        }
        None => Err("Export cancelled".to_string()),
    }
}

/// Imports settings from a JSON file.
#[tauri::command]
pub async fn import_settings<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    log::debug!("Importing settings...");
    
    // Show open dialog
    let file_path = app
        .dialog()
        .file()
        .add_filter("JSON", &["json"])
        .pick_file()
        .await;
    
    match file_path {
        Some(path) => {
            // Read file
            let contents = app
                .fs()
                .read_text_file(&path)
                .await
                .map_err(|e| format!("Failed to read file: {}", e))?;
            
            // Deserialize settings
            let settings: config::UserSettings = serde_json::from_str(&contents)
                .map_err(|e| format!("Invalid settings file: {}", e))?;
            
            // Save settings
            config::save(&app, &settings).map_err(|e| e.to_string())?;
            
            log::info!("Settings imported from: {:?}", path);
            Ok("Settings imported successfully. Please restart the application for all changes to take effect.".to_string())
        }
        None => Err("Import cancelled".to_string()),
    }
}

/// Gets settings as JSON string (for sync).
#[tauri::command]
pub fn get_settings_json<R: Runtime>(app: AppHandle<R>) -> Result<String, String> {
    let settings = config::load(&app).map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())
}

/// Restores settings from JSON string (for sync).
#[tauri::command]
pub async fn restore_settings_json<R: Runtime>(
    app: AppHandle<R>,
    json: String,
) -> Result<(), String> {
    let settings: config::UserSettings = serde_json::from_str(&json)
        .map_err(|e| format!("Invalid settings JSON: {}", e))?;
    
    config::save(&app, &settings).map_err(|e| e.to_string())?;
    
    log::info!("Settings restored from JSON");
    Ok(())
}
