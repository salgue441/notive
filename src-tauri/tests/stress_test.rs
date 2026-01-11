//! Stress tests for performance and memory.

use notive_lib::config::{ShortcutSettings, UpdateChannel, UserSettings};
use serde_json;

#[test]
fn test_settings_serialization_stress() {
    // Test serializing many settings objects
    let settings = UserSettings::default();
    
    for _ in 0..1000 {
        let json = serde_json::to_string(&settings).unwrap();
        let _deserialized: UserSettings = serde_json::from_str(&json).unwrap();
    }
}

#[test]
fn test_settings_with_many_shortcuts() {
    // Test with many shortcut variations
    let mut shortcuts = ShortcutSettings::default();
    
    for i in 0..100 {
        shortcuts.toggle_window = format!("Alt+{}", i);
        let json = serde_json::to_string(&shortcuts).unwrap();
        let _deserialized: ShortcutSettings = serde_json::from_str(&json).unwrap();
    }
}

#[test]
fn test_large_css_handling() {
    // Test with very large CSS (1MB)
    let mut settings = UserSettings::default();
    settings.custom_css = "body { color: red; }\n".repeat(50000);
    settings.custom_css_enabled = true;
    
    // Should serialize
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.len() > 1000000);
    
    // Should deserialize
    let deserialized: UserSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.custom_css.len(), settings.custom_css.len());
}

#[test]
fn test_concurrent_settings_operations() {
    use std::sync::Arc;
    use std::thread;
    
    let settings = Arc::new(UserSettings::default());
    let mut handles = vec![];
    
    for _ in 0..10 {
        let settings_clone = Arc::clone(&settings);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let json = serde_json::to_string(&*settings_clone).unwrap();
                let _: UserSettings = serde_json::from_str(&json).unwrap();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_memory_efficient_settings_clone() {
    // Test that cloning doesn't cause memory issues
    let settings = UserSettings::default();
    
    let mut clones = Vec::new();
    for _ in 0..1000 {
        clones.push(settings.clone());
    }
    
    // Verify all clones are correct
    for clone in clones {
        assert_eq!(clone.zoom_level, settings.zoom_level);
    }
}
