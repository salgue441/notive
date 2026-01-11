//! Window event handlers.

#[cfg(test)]
mod tests;

use crate::privacy;
use crate::tray;
use tauri::{Runtime, Window, WindowEvent};

/// Handles window events.
pub fn handle_window_event<R: Runtime>(window: &Window<R>, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            // Check if close-to-tray is enabled
            let app = window.app_handle();
            if crate::config::get_close_to_tray(&app) {
                api.prevent_close();
                let _ = window.hide();
                log::debug!("Window hidden to tray");
            }
            
            // Check privacy mode settings
            if let Ok(privacy_settings) = privacy::get_privacy_settings(app.clone()) {
                if privacy_settings.clear_history_on_close {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        if let Err(e) = privacy::clear_privacy_data(app_handle).await {
                            log::warn!("Failed to clear privacy data: {}", e);
                        }
                    });
                }
            }
        }
        WindowEvent::Focused(focused) => {
            log::trace!("Window focused: {}", focused);
            if *focused {
                // Reset unread count when window gains focus
                tray::reset_unread_count(window.app_handle());
            }
        }
        _ => {}
    }
}
