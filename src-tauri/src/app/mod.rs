//! Application state management.

mod state;

#[cfg(test)]
mod tests;

pub use state::AppState;

use crate::autostart;
use crate::config;
use crate::shortcuts;
use crate::wayland;
use crate::commands::settings;
use tauri::{App, Manager};

/// Initializes the application state.
pub fn init(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Initializing application state...");
    
    // Load settings
    let settings = config::load(app.handle())?;
    
    // Apply autostart setting
    if settings.autostart_enabled {
        if let Err(e) = autostart::enable(app.handle()) {
            log::warn!("Failed to enable autostart: {}", e);
        }
    }
    
    // Apply zoom level and webview settings
    if let Some(window) = app.get_webview_window("main") {
        window
            .with_webview(|webview| {
                #[cfg(target_os = "linux")]
                {
                    use webkit2gtk::{SettingsExt, WebViewExt};
                    if let Some(webview) = webview.as_ref().and_then(|w| w.downcast_ref::<webkit2gtk::WebView>()) {
                        webview.set_zoom_level(settings.zoom_level);
                        
                        // Apply spellcheck setting
                        if let Some(webview_settings) = webview.settings() {
                            webview_settings.set_enable_spell_checking(settings.spellcheck);
                            log::debug!("Spellcheck set to {}", settings.spellcheck);
                        }
                        
                        // Note: Hardware acceleration is typically controlled at the process level
                        // and may require application restart to take effect
                        // The setting is saved and will be applied on next launch
                        log::debug!("Hardware acceleration setting: {} (may require restart)", settings.hardware_acceleration);
                    }
                }
            })
            .ok();
    }
    
    // Register shortcuts from settings
    shortcuts::register_with_settings(app, &settings.shortcuts)?;
    
    // Apply theme
    if let Some(window) = app.get_webview_window("main") {
        let theme = settings.theme.clone();
        let window_handle = window.clone();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            let _ = settings::apply_theme_internal(&window_handle, &theme);
        });
    }
    
    // Apply Wayland optimizations if running under Wayland
    if let Err(e) = wayland::apply_wayland_optimizations(app.handle()) {
        log::warn!("Failed to apply Wayland optimizations: {}", e);
    }
    
    // Apply custom CSS if enabled
    if settings.custom_css_enabled && !settings.custom_css.is_empty() {
        if let Some(window) = app.get_webview_window("main") {
            // Wait a bit for the page to load before injecting CSS
            let css = settings.custom_css.clone();
            let window_handle = window.clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                let css_script = format!(
                    r#"
                    (function() {{
                        const styleId = 'notive-custom-css';
                        let style = document.getElementById(styleId);
                        if (!style) {{
                            style = document.createElement('style');
                            style.id = styleId;
                            document.head.appendChild(style);
                        }}
                        style.textContent = `{}`;
                    }})();
                    "#,
                    css.replace('`', r"\`").replace('$', r"\$")
                );
                let _ = window_handle.eval(&css_script);
            });
        }
    }
    
    // Restore tabs on startup
    if let Some(window) = app.get_webview_window("main") {
        let app_handle = app.handle().clone();
        let window_label = "main".to_string();
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            match crate::tabs::restore_tabs(app_handle, window_label).await {
                Ok(tabs) => {
                    if !tabs.is_empty() {
                        log::info!("Restored {} tabs", tabs.len());
                        // Optionally navigate to the first tab
                        if let Some(first_tab) = tabs.first() {
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.eval(&format!(
                                    "window.location.href = '{}';",
                                    first_tab.url.replace('\'', "\\'")
                                ));
                            }
                        }
                    }
                }
                Err(e) => log::warn!("Failed to restore tabs: {}", e),
            }
        });
    }
    
    log::debug!("Application state initialized");
    Ok(())
}
