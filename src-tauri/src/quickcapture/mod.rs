//! Enhanced quick capture functionality.

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Quick capture template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureTemplate {
    pub id: String,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
}

impl CaptureTemplate {
    pub fn new(name: String, url: String, description: Option<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            url,
            description,
        }
    }
}

/// Quick capture configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickCaptureConfig {
    pub default_template: Option<String>,
    pub templates: Vec<CaptureTemplate>,
    pub enable_tags: bool,
    pub default_tags: Vec<String>,
}

impl Default for QuickCaptureConfig {
    fn default() -> Self {
        Self {
            default_template: None,
            templates: vec![
                CaptureTemplate::new(
                    "New Page".to_string(),
                    "https://www.notion.so/new".to_string(),
                    Some("Create a new blank page".to_string()),
                ),
                CaptureTemplate::new(
                    "Quick Note".to_string(),
                    "https://www.notion.so/new?template=quick-note".to_string(),
                    Some("Quick note template".to_string()),
                ),
            ],
            enable_tags: true,
            default_tags: vec![],
        }
    }
}

/// Opens quick capture with optional template and tags.
#[tauri::command]
pub async fn open_quick_capture<R: Runtime>(
    app: AppHandle<R>,
    template_id: Option<String>,
    tags: Option<Vec<String>>,
) -> Result<String, String> {
    log::debug!("Opening quick capture: template={:?}, tags={:?}", template_id, tags);
    
    // Load quick capture config
    let config = load_quick_capture_config(&app).unwrap_or_default();
    
    // Determine URL
    let url = if let Some(tid) = template_id {
        config
            .templates
            .iter()
            .find(|t| t.id == tid)
            .map(|t| t.url.clone())
            .unwrap_or_else(|| "https://www.notion.so/new".to_string())
    } else if let Some(default) = &config.default_template {
        config
            .templates
            .iter()
            .find(|t| t.id == *default)
            .map(|t| t.url.clone())
            .unwrap_or_else(|| "https://www.notion.so/new".to_string())
    } else {
        "https://www.notion.so/new".to_string()
    };
    
    // Build URL with tags if enabled
    let final_url = if config.enable_tags && tags.is_some() {
        let mut url_obj = url::Url::parse(&url).map_err(|e| e.to_string())?;
        for tag in tags.unwrap_or_default() {
            url_obj.query_pairs_mut().append_pair("tag", &tag);
        }
        url_obj.to_string()
    } else {
        url
    };
    
    // Open in main window
    if let Some(window) = app.get_webview_window("main") {
        if !window.is_visible().unwrap_or(false) {
            let _ = window.show();
            let _ = window.set_focus();
        }
        
        window
            .eval(&format!("window.location.href = '{}';", final_url.replace('\'', "\\'")))
            .map_err(|e| e.to_string())?;
        
        log::info!("Quick capture opened: {}", final_url);
        Ok(final_url)
    } else {
        Err("Main window not found".to_string())
    }
}

/// Lists available capture templates.
#[tauri::command]
pub fn list_capture_templates<R: Runtime>(app: AppHandle<R>) -> Result<Vec<CaptureTemplate>, String> {
    let config = load_quick_capture_config(&app).unwrap_or_default();
    Ok(config.templates)
}

/// Adds a new capture template.
#[tauri::command]
pub fn add_capture_template<R: Runtime>(
    app: AppHandle<R>,
    name: String,
    url: String,
    description: Option<String>,
) -> Result<String, String> {
    let mut config = load_quick_capture_config(&app).unwrap_or_default();
    let template = CaptureTemplate::new(name, url, description);
    let id = template.id.clone();
    config.templates.push(template);
    save_quick_capture_config(&app, &config)?;
    Ok(id)
}

/// Removes a capture template.
#[tauri::command]
pub fn remove_capture_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
) -> Result<(), String> {
    let mut config = load_quick_capture_config(&app).unwrap_or_default();
    config.templates.retain(|t| t.id != template_id);
    save_quick_capture_config(&app, &config)?;
    Ok(())
}

/// Loads quick capture configuration.
fn load_quick_capture_config<R: Runtime>(app: &AppHandle<R>) -> Result<QuickCaptureConfig, String> {
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        if let Some(value) = store.get("quick_capture_config") {
            serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))
        } else {
            Ok(QuickCaptureConfig::default())
        }
    })
    .map_err(|e| e.to_string())
}

/// Saves quick capture configuration.
fn save_quick_capture_config<R: Runtime>(
    app: &AppHandle<R>,
    config: &QuickCaptureConfig,
) -> Result<(), String> {
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        let value = serde_json::to_value(config)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("quick_capture_config".to_string(), value)?;
        store.save()?;
        Ok(())
    })
    .map_err(|e| e.to_string())
}
