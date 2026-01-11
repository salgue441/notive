//! Unit tests for utilities module.

use super::paths::*;

#[test]
fn test_app_data_dir() {
    let dir = app_data_dir();
    // Should return Some on most systems
    // On CI or restricted environments it might be None
    if let Some(path) = dir {
        assert!(path.to_string_lossy().contains("notive"));
    }
}

#[test]
fn test_app_config_dir() {
    let dir = app_config_dir();
    if let Some(path) = dir {
        assert!(path.to_string_lossy().contains("notive"));
    }
}

#[test]
fn test_app_cache_dir() {
    let dir = app_cache_dir();
    if let Some(path) = dir {
        assert!(path.to_string_lossy().contains("notive"));
    }
}

#[test]
fn test_downloads_dir() {
    let dir = downloads_dir();
    // Downloads dir might exist or not depending on system
    // Just verify it doesn't panic
    let _ = dir;
}
