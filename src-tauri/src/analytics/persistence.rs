//! Analytics persistence.

use serde_json;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_store::{with_store, StoreCollection};

const ANALYTICS_STORE_PATH: &str = "analytics.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyticsData {
    pub views: Vec<super::PageView>,
    pub edits: Vec<super::PageEdit>,
}

impl Default for AnalyticsData {
    fn default() -> Self {
        Self {
            views: vec![],
            edits: vec![],
        }
    }
}

pub use AnalyticsData;

/// Loads analytics data from persistent storage.
pub fn load_analytics<R: Runtime>(app: &AppHandle<R>) -> Result<AnalyticsData, Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, ANALYTICS_STORE_PATH, |store| {
        if let Some(value) = store.get("analytics") {
            let data: AnalyticsData = serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))?;
            Ok(data)
        } else {
            Ok(AnalyticsData::default())
        }
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}

/// Saves analytics data to persistent storage.
pub fn save_analytics<R: Runtime>(
    app: &AppHandle<R>,
    data: &AnalyticsData,
) -> Result<(), Box<dyn std::error::Error>> {
    let stores = app.state::<StoreCollection<R>>();
    
    with_store(app, stores, ANALYTICS_STORE_PATH, |store| {
        let json = serde_json::to_value(data)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("analytics".to_string(), json)?;
        store.save()?;
        log::debug!("Analytics saved to store");
        Ok(())
    })
    .map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) as Box<dyn std::error::Error>)
}
