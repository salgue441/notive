//! Template persistence.

use super::PageTemplate;
use serde_json;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const TEMPLATES_STORE_PATH: &str = "templates.json";

/// Loads templates from persistent storage.
pub fn load_templates<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<PageTemplate>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, TEMPLATES_STORE_PATH, |store| {
        if let Some(value) = store.get("templates") {
            let templates: Vec<PageTemplate> = serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))?;
            Ok(templates)
        } else {
            Ok(vec![])
        }
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Saves templates to persistent storage.
pub fn save_templates<R: Runtime>(
    app: &AppHandle<R>,
    templates: &[PageTemplate],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, TEMPLATES_STORE_PATH, |store| {
        let data = serde_json::to_value(templates)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("templates".to_string(), data)?;
        store.save()?;
        log::debug!("Templates saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
