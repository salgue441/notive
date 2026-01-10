//! Path utilities.

use std::path::PathBuf;

/// Gets the application data directory.
pub fn app_data_dir() -> Option<PathBuf> {
    dirs::data_dir().map(|p| p.join("notive"))
}

/// Gets the application config directory.
pub fn app_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("notive"))
}

/// Gets the application cache directory.
pub fn app_cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|p| p.join("notive"))
}

/// Gets the downloads directory.
pub fn downloads_dir() -> Option<PathBuf> {
    dirs::download_dir()
}
