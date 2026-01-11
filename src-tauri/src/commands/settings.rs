//! Settings management commands.

#[cfg(test)]
mod tests;

use crate::autostart;
use crate::config::{self, UserSettings};
use crate::shortcuts;
use tauri::{AppHandle, Manager, Runtime, WindowBuilder, WindowUrl};

/// Retrieves the current user settings.
#[tauri::command]
pub fn get_settings<R: Runtime>(app: AppHandle<R>) -> Result<UserSettings, String> {
    log::debug!("Getting settings...");
    config::load(&app).map_err(|e| e.to_string())
}

/// Updates the user settings.
#[tauri::command]
pub async fn update_settings<R: Runtime>(
    app: AppHandle<R>,
    settings: UserSettings,
) -> Result<(), String> {
    log::debug!("Updating settings...");
    
    // Load current settings to compare changes
    let current_settings = config::load(&app).unwrap_or_default();
    
    // Save new settings
    config::save(&app, &settings).map_err(|e| e.to_string())?;
    
    // Apply settings changes
    apply_settings(&app, &settings, &current_settings).await?;
    
    Ok(())
}

/// Applies settings changes to the application.
async fn apply_settings<R: Runtime>(
    app: &AppHandle<R>,
    new_settings: &UserSettings,
    old_settings: &UserSettings,
) -> Result<(), String> {
    // Apply autostart changes
    if new_settings.autostart_enabled != old_settings.autostart_enabled {
        if new_settings.autostart_enabled {
            autostart::enable(app).map_err(|e| e.to_string())?;
            log::info!("Autostart enabled");
        } else {
            autostart::disable(app).map_err(|e| e.to_string())?;
            log::info!("Autostart disabled");
        }
    }
    
    // Apply zoom level if changed
    if new_settings.zoom_level != old_settings.zoom_level {
        if let Some(window) = app.get_webview_window("main") {
            window
                .with_webview(|webview| {
                    #[cfg(target_os = "linux")]
                    {
                        use webkit2gtk::WebViewExt;
                        if let Some(webview) = webview.as_ref().and_then(|w| w.downcast_ref::<webkit2gtk::WebView>()) {
                            webview.set_zoom_level(new_settings.zoom_level);
                        }
                    }
                })
                .map_err(|e| e.to_string())?;
            log::debug!("Zoom level set to {}", new_settings.zoom_level);
        }
    }
    
    // Apply shortcut changes
    if new_settings.shortcuts != old_settings.shortcuts {
        shortcuts::update(app, &new_settings.shortcuts).map_err(|e| e.to_string())?;
        log::info!("Shortcuts updated");
    }
    
    // Apply custom CSS if changed
    if new_settings.custom_css_enabled != old_settings.custom_css_enabled
        || new_settings.custom_css != old_settings.custom_css
    {
        apply_custom_css(app, new_settings).await?;
    }
    
    // Apply spellcheck if changed
    if new_settings.spellcheck != old_settings.spellcheck {
        apply_spellcheck(app, new_settings.spellcheck)?;
    }
    
    // Apply hardware acceleration if changed
    if new_settings.hardware_acceleration != old_settings.hardware_acceleration {
        apply_hardware_acceleration(app, new_settings.hardware_acceleration)?;
    }
    
    // Apply theme if changed
    if new_settings.theme != old_settings.theme {
        apply_theme(app, &new_settings.theme)?;
    }
    
    Ok(())
}

/// Applies custom CSS to the webview.
async fn apply_custom_css<R: Runtime>(
    app: &AppHandle<R>,
    settings: &UserSettings,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if settings.custom_css_enabled && !settings.custom_css.is_empty() {
            let css = format!(
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
                settings.custom_css.replace('`', r"\`").replace('$', r"\$")
            );
            
            window.eval(&css).map_err(|e| e.to_string())?;
            log::debug!("Custom CSS applied");
        } else {
            // Remove custom CSS
            let remove_script = r#"
                (function() {
                    const style = document.getElementById('notive-custom-css');
                    if (style) {
                        style.remove();
                    }
                })();
            "#;
            window.eval(remove_script).map_err(|e| e.to_string())?;
            log::debug!("Custom CSS removed");
        }
    }
    Ok(())
}

/// Applies spellcheck setting to the webview.
fn apply_spellcheck<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .with_webview(|webview| {
                #[cfg(target_os = "linux")]
                {
                    use webkit2gtk::SettingsExt;
                    if let Some(webview) = webview.as_ref().and_then(|w| w.downcast_ref::<webkit2gtk::WebView>()) {
                        if let Some(settings) = webview.settings() {
                            settings.set_enable_spell_checking(enabled);
                            log::debug!("Spellcheck set to {}", enabled);
                        }
                    }
                }
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Applies hardware acceleration setting.
/// Note: Hardware acceleration is typically controlled at the process level
/// and may require application restart to take full effect.
fn apply_hardware_acceleration<R: Runtime>(app: &AppHandle<R>, enabled: bool) -> Result<(), String> {
    // Hardware acceleration is a process-level setting that typically requires
    // application restart. The setting is saved and will be respected on next launch.
    log::info!("Hardware acceleration setting changed to {} (restart may be required)", enabled);
    Ok(())
}

/// Applies theme setting to the application (internal, can be called from app::init).
pub fn apply_theme_internal<R: Runtime>(
    window: &tauri::WebviewWindow<R>,
    theme: &crate::config::Theme,
) -> Result<(), String> {
    let theme_css = match theme {
        crate::config::Theme::Dark => {
            r#"
            (function() {
                const styleId = 'notive-theme-dark';
                let style = document.getElementById(styleId);
                if (!style) {
                    style = document.createElement('style');
                    style.id = styleId;
                    document.head.appendChild(style);
                }
                style.textContent = `
                    :root {
                        color-scheme: dark;
                    }
                    body {
                        background-color: #1e1e1e !important;
                        color: #ffffff !important;
                    }
                `;
            })();
            "#
        }
        crate::config::Theme::Light => {
            r#"
            (function() {
                const styleId = 'notive-theme-dark';
                const style = document.getElementById(styleId);
                if (style) {
                    style.remove();
                }
            })();
            "#
        }
        crate::config::Theme::System => {
            r#"
            (function() {
                const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                const styleId = 'notive-theme-dark';
                let style = document.getElementById(styleId);
                if (prefersDark) {
                    if (!style) {
                        style = document.createElement('style');
                        style.id = styleId;
                        document.head.appendChild(style);
                    }
                    style.textContent = `
                        :root {
                            color-scheme: dark;
                        }
                        body {
                            background-color: #1e1e1e !important;
                            color: #ffffff !important;
                        }
                    `;
                } else {
                    if (style) {
                        style.remove();
                    }
                }
            })();
            "#
        }
    };
    
    window.eval(theme_css).map_err(|e| e.to_string())?;
    log::debug!("Theme applied: {:?}", theme);
    Ok(())
}

/// Applies theme setting to the application.
fn apply_theme<R: Runtime>(app: &AppHandle<R>, theme: &crate::config::Theme) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let theme_css = match theme {
            crate::config::Theme::Dark => {
                r#"
                (function() {
                    const styleId = 'notive-theme-dark';
                    let style = document.getElementById(styleId);
                    if (!style) {
                        style = document.createElement('style');
                        style.id = styleId;
                        document.head.appendChild(style);
                    }
                    style.textContent = `
                        :root {
                            color-scheme: dark;
                        }
                        body {
                            background-color: #1e1e1e !important;
                            color: #ffffff !important;
                        }
                    `;
                })();
                "#
            }
            crate::config::Theme::Light => {
                r#"
                (function() {
                    const styleId = 'notive-theme-dark';
                    const style = document.getElementById(styleId);
                    if (style) {
                        style.remove();
                    }
                })();
                "#
            }
            crate::config::Theme::System => {
                // System theme - detect and apply
                r#"
                (function() {
                    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
                    const styleId = 'notive-theme-dark';
                    let style = document.getElementById(styleId);
                    if (prefersDark) {
                        if (!style) {
                            style = document.createElement('style');
                            style.id = styleId;
                            document.head.appendChild(style);
                        }
                        style.textContent = `
                            :root {
                                color-scheme: dark;
                            }
                            body {
                                background-color: #1e1e1e !important;
                                color: #ffffff !important;
                            }
                        `;
                    } else {
                        if (style) {
                            style.remove();
                        }
                    }
                })();
                "#
            }
        };
        
        window.eval(theme_css).map_err(|e| e.to_string())?;
        log::debug!("Theme applied: {:?}", theme);
    }
    Ok(())
}

/// Opens the settings window.
#[tauri::command]
pub async fn open_settings_window<R: Runtime>(
    app: AppHandle<R>,
) -> Result<(), String> {
    // Check if settings window already exists
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    // Create settings window
    let _window = WindowBuilder::new(
        &app,
        "settings",
        WindowUrl::App("settings.html".into()),
    )
    .title("Notive Settings")
    .inner_size(700.0, 600.0)
    .min_inner_size(600.0, 500.0)
    .resizable(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
