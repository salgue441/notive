//! System tray integration.

mod menu;

use tauri::{App, Runtime};

/// Sets up the system tray.
pub fn setup<R: Runtime>(app: &App<R>) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement system tray setup
    // - Create tray icon
    // - Setup menu
    // - Register event handlers
    log::debug!("Setting up system tray...");

    menu::build(app)?;

    Ok(())
}
