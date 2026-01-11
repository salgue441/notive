//! Window management commands.

#[cfg(test)]
mod tests;

use tauri::{AppHandle, Manager, Runtime};

/// Minimizes the main window to the system tray.
#[tauri::command]
pub fn minimize_to_tray<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Minimizing to tray...");
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Restores the main window from the system tray.
#[tauri::command]
pub fn restore_from_tray<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Restoring from tray...");
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Toggles fullscreen mode for the main window.
#[tauri::command]
pub fn toggle_fullscreen<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Toggling fullscreen...");
    if let Some(window) = app.get_webview_window("main") {
        let is_fullscreen = window.is_fullscreen().map_err(|e| e.to_string())?;
        window
            .set_fullscreen(!is_fullscreen)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Sets the zoom level for the main window.
#[tauri::command]
pub fn set_zoom<R: Runtime>(app: AppHandle<R>, level: f64) -> Result<(), String> {
    log::debug!("Setting zoom level to {}...", level);
    if let Some(window) = app.get_webview_window("main") {
        window
            .with_webview(|webview| {
                #[cfg(target_os = "linux")]
                {
                    use webkit2gtk::WebViewExt;
                    if let Some(webview) = webview.as_ref().and_then(|w| w.downcast_ref::<webkit2gtk::WebView>()) {
                        webview.set_zoom_level(level);
                    }
                }
            })
            .map_err(|e| e.to_string())?;
        
        // Also save zoom level to settings
        if let Ok(mut settings) = crate::config::load(&app) {
            settings.zoom_level = level;
            let _ = crate::config::save(&app, &settings);
        }
    }
    Ok(())
}

/// Reloads the current page.
#[tauri::command]
pub fn reload_page<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Reloading page...");
    if let Some(window) = app.get_webview_window("main") {
        window
            .eval("window.location.reload()")
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
