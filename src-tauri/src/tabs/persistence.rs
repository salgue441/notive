//! Tab persistence functionality.

use super::Tab;
use serde_json;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const TABS_STORE_PATH: &str = "tabs.json";

/// Loads tabs from persistent storage.
pub fn load_tabs<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<Tab>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, TABS_STORE_PATH, |store| {
        if let Some(value) = store.get("tabs") {
            let tabs: Vec<Tab> = serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))?;
            Ok(tabs)
        } else {
            Ok(vec![])
        }
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Saves tabs to persistent storage.
pub fn save_tabs<R: Runtime>(
    app: &AppHandle<R>,
    tabs: &[Tab],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, TABS_STORE_PATH, |store| {
        let data = serde_json::to_value(tabs)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("tabs".to_string(), data)?;
        store.save()?;
        log::debug!("Tabs saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Loads tabs for a specific window.
pub fn load_tabs_for_window<R: Runtime>(
    app: &AppHandle<R>,
    window_label: &str,
) -> Result<Vec<Tab>, Box<dyn std::error::Error>> {
    let all_tabs = load_tabs(app)?;
    Ok(all_tabs
        .into_iter()
        .filter(|tab| tab.window_label == window_label)
        .collect())
}

/// Saves tabs for a specific window.
pub fn save_tabs_for_window<R: Runtime>(
    app: &AppHandle<R>,
    window_label: &str,
    tabs: &[Tab],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut all_tabs = load_tabs(app).unwrap_or_default();
    
    // Remove existing tabs for this window
    all_tabs.retain(|tab| tab.window_label != window_label);
    
    // Add new tabs
    all_tabs.extend_from_slice(tabs);
    
    save_tabs(app, &all_tabs)
}
