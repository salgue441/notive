//! Tray menu construction and handling.

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager, Runtime,
};

/// Builds and registers the tray menu.
pub fn build<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items
    let show = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let separator1 = PredefinedMenuItem::separator(app)?;
    let settings = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let check_updates =
        MenuItem::with_id(app, "check_updates", "Check for Updates", true, None::<&str>)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let about = MenuItem::with_id(app, "about", "About Notive", true, None::<&str>)?;
    let separator3 = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Build menu
    let menu = Menu::with_items(
        app,
        &[
            &show,
            &hide,
            &separator1,
            &settings,
            &check_updates,
            &separator2,
            &about,
            &separator3,
            &quit,
        ],
    )?;

    // Build tray icon
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(false)
        .on_menu_event(|app, event| {
            handle_menu_event(app, event.id.as_ref());
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                // Show window on left click
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

/// Handles tray menu item clicks.
fn handle_menu_event<R: Runtime>(app: &tauri::AppHandle<R>, id: &str) {
    match id {
        "quit" => {
            log::info!("Quit requested from tray");
            app.exit(0);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "hide" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }
        }
        "settings" => {
            // TODO: Open settings window/dialog
            log::debug!("Settings requested");
        }
        "check_updates" => {
            // TODO: Trigger update check
            log::debug!("Update check requested");
        }
        "about" => {
            // TODO: Show about dialog
            log::debug!("About requested");
        }
        _ => {}
    }
}
