//! Bookmark persistence to settings store.

use crate::bookmarks::Bookmark;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const BOOKMARKS_STORE_PATH: &str = "bookmarks.json";

/// Saves bookmarks to persistent storage.
pub fn save_bookmarks<R: Runtime>(
    app: &AppHandle<R>,
    bookmarks: &[Bookmark],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, BOOKMARKS_STORE_PATH, |store| {
        let data = serde_json::to_value(bookmarks)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("bookmarks".to_string(), data)?;
        store.save()?;
        log::debug!("Bookmarks saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Loads bookmarks from persistent storage.
pub fn load_bookmarks<R: Runtime>(
    app: &AppHandle<R>,
) -> Result<Vec<Bookmark>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, BOOKMARKS_STORE_PATH, |store| {
        if let Some(data) = store.get("bookmarks") {
            if let Some(bookmarks_array) = data.as_array() {
                match serde_json::from_value::<Vec<Bookmark>>(serde_json::Value::Array(bookmarks_array.clone())) {
                    Ok(bookmarks) => {
                        log::debug!("Bookmarks loaded from store: {} bookmarks", bookmarks.len());
                        return Ok(bookmarks);
                    }
                    Err(e) => {
                        log::warn!("Failed to deserialize bookmarks: {}", e);
                    }
                }
            }
        }
        
        log::debug!("No bookmarks found, returning empty list");
        Ok(vec![])
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
