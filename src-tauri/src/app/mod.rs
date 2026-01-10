//! Application state management.

mod state;

pub use state::AppState;

use tauri::App;

/// Initializes the application state.
pub fn init(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Initialize application state
    // - Load settings from store
    // - Setup state management
    log::debug!("Initializing application state...");
    Ok(())
}
