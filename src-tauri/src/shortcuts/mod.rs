//! Global keyboard shortcuts.

use tauri::{App, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// Registers global keyboard shortcuts.
pub fn register<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Registering global shortcuts...");

    let handle = app.handle().clone();

    // Toggle window visibility: Ctrl+Shift+N
    app.global_shortcut().on_shortcut(
        "CommandOrControl+Shift+N",
        move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                toggle_window(&handle);
            }
        },
    )?;

    Ok(())
}

/// Toggles the main window visibility.
fn toggle_window<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
            log::debug!("Window hidden via shortcut");
        } else {
            let _ = window.show();
            let _ = window.set_focus();
            log::debug!("Window shown via shortcut");
        }
    }
}

/// Updates shortcuts based on user settings.
pub fn update<R: Runtime>(
    _app: &tauri::AppHandle<R>,
    _shortcuts: &crate::config::ShortcutSettings,
) -> Result<(), String> {
    // TODO: Implement shortcut updates
    // - Unregister old shortcuts
    // - Register new shortcuts from settings
    Ok(())
}
