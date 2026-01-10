//! Notification commands.

use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

/// Notification payload from the frontend.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct NotificationPayload {
    pub title: String,
    pub body: Option<String>,
    pub icon: Option<String>,
}

/// Shows a native notification.
#[tauri::command]
pub async fn show_notification<R: Runtime>(
    app: AppHandle<R>,
    payload: NotificationPayload,
) -> Result<(), String> {
    // TODO: Implement native notification
    log::debug!("Showing notification: {}", payload.title);

    let mut builder = app.notification().builder().title(&payload.title);

    if let Some(body) = &payload.body {
        builder = builder.body(body);
    }

    builder.show().map_err(|e| e.to_string())
}
