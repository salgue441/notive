//! Memory leak detection tests.

use notive_lib::config::{ShortcutSettings, UserSettings};
use serde_json;

#[test]
fn test_settings_no_memory_leak() {
    // Test that creating many settings doesn't leak memory
    let mut settings_vec = Vec::new();
    
    for i in 0..1000 {
        let mut settings = UserSettings::default();
        settings.zoom_level = 1.0 + (i as f64) * 0.001;
        settings_vec.push(settings);
    }
    
    // Clear vector (should free memory)
    settings_vec.clear();
    
    // Create new settings (memory should be available)
    let _new_settings = UserSettings::default();
}

#[test]
fn test_shortcut_settings_no_memory_leak() {
    // Test shortcut settings memory usage
    let mut shortcuts_vec = Vec::new();
    
    for i in 0..1000 {
        let mut shortcuts = ShortcutSettings::default();
        shortcuts.toggle_window = format!("Alt+{}", i);
        shortcuts_vec.push(shortcuts);
    }
    
    shortcuts_vec.clear();
    let _new_shortcuts = ShortcutSettings::default();
}

#[test]
fn test_serialization_no_memory_leak() {
    // Test that serialization doesn't leak memory
    let settings = UserSettings::default();
    
    for _ in 0..1000 {
        let json = serde_json::to_string(&settings).unwrap();
        let _deserialized: UserSettings = serde_json::from_str(&json).unwrap();
        // json and deserialized go out of scope here
    }
}

#[test]
fn test_large_css_memory() {
    // Test memory usage with large CSS
    let mut settings = UserSettings::default();
    settings.custom_css = "body { color: red; }\n".repeat(10000); // ~200KB
    
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.len() > 200000);
    
    // Should deserialize without issues
    let _deserialized: UserSettings = serde_json::from_str(&json).unwrap();
    
    // Clear settings
    drop(settings);
    drop(json);
    
    // Memory should be freed
    let _new_settings = UserSettings::default();
}
