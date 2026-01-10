//! Application state definitions.

use serde::{Deserialize, Serialize};

/// Global application state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppState {
    /// Whether the window is currently minimized to tray.
    pub is_minimized: bool,

    /// Whether an update is available.
    pub update_available: bool,

    /// Version of available update, if any.
    pub update_version: Option<String>,
}
