//! Notification system.
//!
//! Bridges web notifications to native desktop notifications.

use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

/// Shows a native notification.
pub fn show<R: Runtime>(app: &AppHandle<R>, title: &str, body: Option<&str>) -> Result<(), String> {
    let mut builder = app.notification().builder().title(title);

    if let Some(body) = body {
        builder = builder.body(body);
    }

    builder.show().map_err(|e| e.to_string())
}
