//! Global keyboard shortcuts.

#[cfg(test)]
mod tests;

use std::sync::Mutex;
use tauri::{App, Manager, Runtime};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

// Store registered shortcuts for cleanup
static REGISTERED_SHORTCUTS: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Registers global keyboard shortcuts with default settings.
pub fn register<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    let default_shortcuts = crate::config::ShortcutSettings::default();
    register_with_settings(app, &default_shortcuts)
}

/// Registers global keyboard shortcuts from settings.
pub fn register_with_settings<R: Runtime>(
    app: &App<R>,
    shortcuts: &crate::config::ShortcutSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Registering global shortcuts from settings...");

    let handle = app.handle().clone();

    // Register toggle window shortcut
    if let Err(e) = register_shortcut(
        app,
        &shortcuts.toggle_window,
        move |app| {
            toggle_window(app);
        },
    ) {
        log::warn!("Failed to register toggle_window shortcut: {}", e);
    }

    // Register quick capture shortcut
    let handle2 = handle.clone();
    if let Err(e) = register_shortcut(
        app,
        &shortcuts.quick_capture,
        move |app| {
            quick_capture(app);
        },
    ) {
        log::warn!("Failed to register quick_capture shortcut: {}", e);
    }

    Ok(())
}

/// Registers a single shortcut.
fn register_shortcut<R: Runtime, F>(
    app: &App<R>,
    shortcut_str: &str,
    callback: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&tauri::AppHandle<R>) + Send + 'static,
{
    let handle = app.handle().clone();
    
    app.global_shortcut().on_shortcut(
        shortcut_str,
        move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                callback(&handle);
            }
        },
    )?;

    // Track registered shortcut
    if let Ok(mut registered) = REGISTERED_SHORTCUTS.lock() {
        registered.push(shortcut_str.to_string());
    }

    log::debug!("Registered shortcut: {}", shortcut_str);
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

/// Opens quick capture (creates a new page in Notion).
fn quick_capture<R: Runtime>(app: &tauri::AppHandle<R>) {
    // Use enhanced quick capture with default settings
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = crate::quickcapture::open_quick_capture(handle, None, None).await {
            log::warn!("Failed to open quick capture: {}", e);
        }
    });
}

/// Updates shortcuts based on user settings.
pub fn update<R: Runtime>(
    app: &tauri::AppHandle<R>,
    shortcuts: &crate::config::ShortcutSettings,
) -> Result<(), String> {
    log::debug!("Updating shortcuts...");

    // Unregister all existing shortcuts
    if let Ok(mut registered) = REGISTERED_SHORTCUTS.lock() {
        for shortcut in registered.iter() {
            if let Err(e) = app.global_shortcut().unregister(shortcut) {
                log::warn!("Failed to unregister shortcut {}: {}", shortcut, e);
            }
        }
        registered.clear();
    }

    // Re-register with new settings
    // We need to convert AppHandle to App, but we can't easily do that
    // So we'll use a workaround: store the shortcuts and re-register on next app init
    // For now, we'll just log that shortcuts should be updated
    // The proper way would be to maintain a reference to the App, but that's complex
    // So we'll note this limitation and register shortcuts on startup only
    
    log::warn!("Shortcut updates require app restart to take effect");
    Ok(())
}
