//! System tray integration.

mod menu;

use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{App, Runtime};

/// Global unread notification count
static UNREAD_COUNT: AtomicU32 = AtomicU32::new(0);

/// Sets up the system tray.
pub fn setup<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    log::debug!("Setting up system tray...");

    menu::build(app)?;

    Ok(())
}

/// Increments the unread notification count and updates the tray badge.
pub fn increment_unread_count<R: Runtime>(app: &tauri::AppHandle<R>) {
    let count = UNREAD_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    update_tray_badge(app, Some(count));
    log::debug!("Unread notification count: {}", count);
}

/// Resets the unread notification count and clears the tray badge.
pub fn reset_unread_count<R: Runtime>(app: &tauri::AppHandle<R>) {
    UNREAD_COUNT.store(0, Ordering::SeqCst);
    update_tray_badge(app, None);
    log::debug!("Unread notification count reset");
}

/// Updates the tray icon badge with the unread count.
fn update_tray_badge<R: Runtime>(app: &tauri::AppHandle<R>, count: Option<u32>) {
    // Note: Tauri 2.0 tray icons don't have built-in badge support on Linux
    // This is a placeholder for future implementation or platform-specific code
    // On platforms that support it (macOS, Windows), we could update the tray icon
    // For Linux, we could potentially overlay a badge on the icon image
    
    if let Some(count) = count {
        if count > 0 {
            log::debug!("Tray badge updated: {} unread notifications", count);
            // TODO: Implement platform-specific badge rendering
            // For now, we'll just log the count
        }
    } else {
        log::debug!("Tray badge cleared");
    }
}

/// Gets the current unread notification count.
pub fn get_unread_count() -> u32 {
    UNREAD_COUNT.load(Ordering::SeqCst)
}
