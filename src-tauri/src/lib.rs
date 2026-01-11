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
mod workspaces;
mod tabs;
mod calendar;
mod wayland;
mod accounts;
mod bookmarks;
mod history;
mod privacy;
mod quickcapture;
mod security;
mod sessions;
mod templates;
mod analytics;

// Re-export handlers for benchmarks
#[cfg(feature = "bench")]
pub use handlers::navigation;

// Re-export for tests
#[cfg(test)]
pub use bookmarks;
#[cfg(test)]
pub use history;

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

            // Global shortcuts will be registered in app::init with settings

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
            commands::downloads::handle_download,
            commands::downloads::download_with_dialog,
            commands::app::show_about,
            commands::app::check_updates,
            commands::settings::open_settings_window,
            workspaces::create_workspace,
            workspaces::list_workspaces,
            workspaces::switch_workspace,
            workspaces::close_workspace,
            tabs::open_tab,
            tabs::close_tab,
            tabs::switch_tab,
            tabs::list_tabs,
            calendar::sync_calendar,
            calendar::export_event,
            calendar::check_calendar_availability,
            wayland::is_wayland,
            wayland::get_wayland_info,
            accounts::start_oauth_flow,
            accounts::add_account,
            accounts::list_accounts,
            accounts::switch_account,
            accounts::remove_account,
            bookmarks::add_bookmark,
            bookmarks::remove_bookmark,
            bookmarks::list_bookmarks,
            bookmarks::search_bookmarks,
            bookmarks::get_bookmark,
            history::record_page_visit,
            history::get_recent_pages,
            history::clear_history,
            history::remove_history_entry,
            commands::sync::export_settings,
            commands::sync::import_settings,
            commands::sync::get_settings_json,
            commands::sync::restore_settings_json,
            search::global_search,
            search::get_search_history,
            search::clear_search_history,
            tabs::restore_tabs,
            privacy::get_privacy_settings,
            privacy::update_privacy_settings,
            privacy::clear_privacy_data,
            privacy::is_privacy_mode_enabled,
            quickcapture::open_quick_capture,
            quickcapture::list_capture_templates,
            quickcapture::add_capture_template,
            quickcapture::remove_capture_template,
            notifications::customization::get_notification_settings,
            notifications::customization::update_notification_settings,
            notifications::customization::should_show_notification,
            notifications::customization::get_notification_template,
            sessions::create_session,
            sessions::list_sessions,
            sessions::get_active_session,
            sessions::set_active_session,
            sessions::remove_session,
            sessions::cleanup_expired_sessions,
            templates::list_templates,
            templates::get_template,
            templates::create_template,
            templates::update_template,
            templates::delete_template,
            templates::use_template,
            templates::search_templates,
            analytics::record_page_view,
            analytics::record_page_edit,
            analytics::get_usage_stats,
            analytics::get_activity_timeline,
            analytics::clear_analytics,
            performance::get_resource_usage,
            performance::get_performance_metrics,
            performance::clear_performance_metrics,
            cache::cache_resource,
            cache::get_cached_resource,
            cache::clear_cache,
            cache::get_cache_stats,
            cache::preload_resources,
            monitor::get_monitors,
            monitor::get_primary_monitor,
            monitor::save_window_placement,
            monitor::restore_window_placement,
            monitor::move_window_to_monitor,
            notion_api::set_notion_api_key,
            notion_api::get_page_metadata,
            notion_api::has_notion_api_key,
            oauth::start_oauth_flow,
            oauth::handle_oauth_callback,
            oauth::refresh_oauth_token,
            offline::enable_offline_mode,
            offline::disable_offline_mode,
            offline::cache_page_for_offline,
            offline::get_cached_page,
            offline::sync_offline_changes,
            offline::get_offline_status,
            offline::clear_offline_cache,
            plugins::list_plugins,
            plugins::load_plugin,
            plugins::enable_plugin,
            plugins::disable_plugin,
            plugins::uninstall_plugin,
            ai::ai_search,
            ai::get_ai_suggestions,
            ai::get_ai_autocomplete,
            ai::generate_smart_templates,
        ])
        // Window events
        .on_window_event(handlers::window::handle_window_event)
        // Navigation events
        .on_navigation(|window, url| {
            handlers::handle_navigation(window.app_handle(), &url.to_string())
        })
        // Run
        .run(tauri::generate_context!())
        .expect("error while running notive");
}
