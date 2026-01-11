//! History persistence to settings store.

use crate::history::HistoryEntry;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const HISTORY_STORE_PATH: &str = "history.json";
const MAX_HISTORY_ENTRIES: usize = 1000;

/// Saves history to persistent storage.
pub fn save_history<R: Runtime>(
    app: &AppHandle<R>,
    history: &[HistoryEntry],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    // Limit history size
    let history_to_save: Vec<&HistoryEntry> = history.iter().take(MAX_HISTORY_ENTRIES).collect();
    
    with_store(app, stores, HISTORY_STORE_PATH, |store| {
        let data = serde_json::to_value(history_to_save)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("history".to_string(), data)?;
        store.save()?;
        log::debug!("History saved to store: {} entries", history_to_save.len());
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Loads history from persistent storage.
pub fn load_history<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<Vec<HistoryEntry>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, HISTORY_STORE_PATH, |store| {
        if let Some(data) = store.get("history") {
            if let Some(history_array) = data.as_array() {
                match serde_json::from_value::<Vec<HistoryEntry>>(serde_json::Value::Array(history_array.clone())) {
                    Ok(history) => {
                        log::debug!("History loaded from store: {} entries", history.len());
                        return Ok(history);
                    }
                    Err(e) => {
                        log::warn!("Failed to deserialize history: {}", e);
                    }
                }
            }
        }
        
        log::debug!("No history found, returning empty list");
        Ok(vec![])
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
