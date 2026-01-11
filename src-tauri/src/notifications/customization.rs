//! Notification customization functionality.

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Notification sound configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NotificationSound {
    None,
    Default,
    Custom(String), // Path to sound file
}

impl Default for NotificationSound {
    fn default() -> Self {
        Self::Default
    }
}

/// Notification template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub title: String,
    pub body: String,
    pub sound: NotificationSound,
}

impl NotificationTemplate {
    pub fn new(name: String, title: String, body: String, sound: NotificationSound) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            title,
            body,
            sound,
        }
    }
}

/// Notification customization settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub default_sound: NotificationSound,
    pub templates: Vec<NotificationTemplate>,
    pub enable_scheduling: bool,
    pub quiet_hours_start: Option<String>, // HH:MM format
    pub quiet_hours_end: Option<String>,   // HH:MM format
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            default_sound: NotificationSound::Default,
            templates: vec![
                NotificationTemplate::new(
                    "Default".to_string(),
                    "Notion".to_string(),
                    "You have a new notification".to_string(),
                    NotificationSound::Default,
                ),
            ],
            enable_scheduling: false,
            quiet_hours_start: None,
            quiet_hours_end: None,
        }
    }
}

/// Gets notification customization settings.
#[tauri::command]
pub fn get_notification_settings<R: Runtime>(
    app: AppHandle<R>,
) -> Result<NotificationSettings, String> {
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        if let Some(value) = store.get("notification_settings") {
            serde_json::from_value(value.clone())
                .map_err(|e| tauri_plugin_store::Error::Deserialize(e.to_string()))
        } else {
            Ok(NotificationSettings::default())
        }
    })
    .map_err(|e| e.to_string())
}

/// Updates notification customization settings.
#[tauri::command]
pub fn update_notification_settings<R: Runtime>(
    app: AppHandle<R>,
    settings: NotificationSettings,
) -> Result<(), String> {
    let stores = app.state::<tauri_plugin_store::StoreCollection<R>>();
    tauri_plugin_store::with_store(app, stores, "settings.json", |store| {
        let value = serde_json::to_value(&settings)
            .map_err(|e| tauri_plugin_store::Error::Serialize(e.to_string()))?;
        store.insert("notification_settings".to_string(), value)?;
        store.save()?;
        Ok(())
    })
    .map_err(|e| e.to_string())
}

/// Checks if notifications should be shown (respects quiet hours).
#[tauri::command]
pub fn should_show_notification<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    let settings = get_notification_settings(app)?;
    
    if !settings.enable_scheduling {
        return Ok(true);
    }
    
    // Check quiet hours
    if let (Some(start), Some(end)) = (&settings.quiet_hours_start, &settings.quiet_hours_end) {
        let now = chrono::Local::now().time();
        let start_time = chrono::NaiveTime::parse_from_str(start, "%H:%M")
            .map_err(|e| format!("Invalid start time: {}", e))?;
        let end_time = chrono::NaiveTime::parse_from_str(end, "%H:%M")
            .map_err(|e| format!("Invalid end time: {}", e))?;
        
        // Check if current time is within quiet hours
        if start_time <= end_time {
            // Same day range
            if now >= start_time && now <= end_time {
                return Ok(false);
            }
        } else {
            // Overnight range
            if now >= start_time || now <= end_time {
                return Ok(false);
            }
        }
    }
    
    Ok(true)
}

/// Gets notification template by ID.
#[tauri::command]
pub fn get_notification_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
) -> Result<Option<NotificationTemplate>, String> {
    let settings = get_notification_settings(app)?;
    Ok(settings
        .templates
        .into_iter()
        .find(|t| t.id == template_id))
}
