//! Notive - A high-performance Notion desktop wrapper for Linux
//!
//! This crate provides the core functionality for the Notive application,
//! including window management, system tray integration, notifications,
//! keyboard shortcuts, and auto-updates.

mod app;
mod commands;
mod config;
mod handlers;
mod notifications;
mod shortcuts;
mod tray;
mod updater;
mod utils;

use tauri::Manager;

/// Runs the Notive application.
///
/// This function initializes all plugins, sets up the application state,
/// and starts the Tauri runtime.
///
/// # Arguments
///
/// * `start_minimized` - If true, the window will be hidden to tray on startup.
pub fn run(start_minimized: bool) {
    env_logger::init();

    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_http::init())
        // Setup
        .setup(move |app| {
            log::info!("Setting up Notive...");

            // Initialize application state
            app::init(app)?;

            // Setup system tray
            tray::setup(app)?;

            // Register global shortcuts
            shortcuts::register(app)?;

            // Hide window to tray if --minimized flag was passed
            if start_minimized {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                    log::info!("Started minimized to tray");
                }
            }

            // Setup auto-updater (background check)
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                updater::setup(&handle).await;
            });

            log::info!("Notive started successfully");
            Ok(())
        })
        // IPC Commands
        .invoke_handler(tauri::generate_handler![
            commands::window::minimize_to_tray,
            commands::window::restore_from_tray,
            commands::window::toggle_fullscreen,
            commands::window::set_zoom,
            commands::window::reload_page,
            commands::settings::get_settings,
            commands::settings::update_settings,
            commands::notifications::show_notification,
        ])
        // Window events
        .on_window_event(handlers::window::handle_window_event)
        // Run
        .run(tauri::generate_context!())
        .expect("error while running notive");
}
