//! Window event handlers.

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
        }
        WindowEvent::Focused(focused) => {
            log::trace!("Window focused: {}", focused);
        }
        _ => {}
    }
}
