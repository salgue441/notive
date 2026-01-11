//! Session persistence.

use super::Session;
use serde_json;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const SESSIONS_STORE_PATH: &str = "sessions.json";

/// Loads sessions from persistent storage.
pub fn load_sessions<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, SESSIONS_STORE_PATH, |store| {
        if let Some(value) = store.get("sessions") {
            let sessions: Vec<Session> = serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))?;
            Ok(sessions)
        } else {
            Ok(vec![])
        }
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Saves sessions to persistent storage.
pub fn save_sessions<R: Runtime>(
    app: &AppHandle<R>,
    sessions: &[Session],
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, SESSIONS_STORE_PATH, |store| {
        let data = serde_json::to_value(sessions)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("sessions".to_string(), data)?;
        store.save()?;
        log::debug!("Sessions saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
