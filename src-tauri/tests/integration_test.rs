//! Integration tests for Notive.

use notive_lib::config::{load, save, UserSettings};
use std::sync::Once;
use tempfile::TempDir;

static INIT: Once = Once::new();

fn init_test() {
    INIT.call_once(|| {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
            .init();
    });
}

#[tokio::test]
async fn test_settings_persistence() {
    init_test();
    
    // This is a basic integration test structure
    // Full integration tests would require a Tauri app instance
    let settings = UserSettings::default();
    
    // Verify default settings
    assert!(!settings.start_minimized);
    assert!(settings.minimize_to_tray);
    assert_eq!(settings.zoom_level, 1.0);
    assert!(!settings.custom_css_enabled);
}

#[test]
fn test_shortcut_settings_default() {
    use notive_lib::config::ShortcutSettings;
    
    let shortcuts = ShortcutSettings::default();
    
    assert_eq!(shortcuts.toggle_window, "CommandOrControl+Shift+N");
    assert_eq!(shortcuts.quick_capture, "CommandOrControl+Shift+C");
    assert_eq!(shortcuts.reload, "CommandOrControl+R");
}

#[test]
fn test_update_channel_serialization() {
    use notive_lib::config::UpdateChannel;
    use serde_json;
    
    let stable = UpdateChannel::Stable;
    let beta = UpdateChannel::Beta;
    let nightly = UpdateChannel::Nightly;
    
    assert_eq!(serde_json::to_string(&stable).unwrap(), "\"stable\"");
    assert_eq!(serde_json::to_string(&beta).unwrap(), "\"beta\"");
    assert_eq!(serde_json::to_string(&nightly).unwrap(), "\"nightly\"");
}
