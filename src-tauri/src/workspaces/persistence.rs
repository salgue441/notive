//! Workspace persistence to settings store.

use crate::workspaces::Workspace;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const WORKSPACES_STORE_PATH: &str = "workspaces.json";

/// Saves workspaces to persistent storage.
pub fn save_workspaces<R: Runtime>(
    app: &AppHandle<R>,
    workspaces: &[Workspace],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, WORKSPACES_STORE_PATH, |store| {
        let data = serde_json::to_value(workspaces)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("workspaces".to_string(), data)?;
        store.save()?;
        log::debug!("Workspaces saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Loads workspaces from persistent storage.
pub fn load_workspaces<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<Vec<Workspace>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, WORKSPACES_STORE_PATH, |store| {
        if let Some(data) = store.get("workspaces") {
            if let Some(workspaces_array) = data.as_array() {
                match serde_json::from_value::<Vec<Workspace>>(serde_json::Value::Array(workspaces_array.clone())) {
                    Ok(workspaces) => {
                        log::debug!("Workspaces loaded from store: {} workspaces", workspaces.len());
                        return Ok(workspaces);
                    }
                    Err(e) => {
                        log::warn!("Failed to deserialize workspaces: {}", e);
                    }
                }
            }
        }
        
        log::debug!("No workspaces found, returning empty list");
        Ok(vec![])
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
