//! Tests for settings sync functionality.

use crate::config::UserSettings;

#[test]
fn test_settings_serialization_for_export() {
    let settings = UserSettings::default();
    let json = serde_json::to_string_pretty(&settings).unwrap();
    
    // Should serialize to valid JSON
    assert!(!json.is_empty());
    assert!(json.contains("\"zoom_level\""));
    assert!(json.contains("\"theme\""));
}

#[test]
fn test_settings_deserialization_for_import() {
    let json = r#"{
        "start_minimized": false,
        "minimize_to_tray": true,
        "close_to_tray": true,
        "zoom_level": 1.5,
        "theme": "dark",
        "custom_css_enabled": false,
        "custom_css": "",
        "notifications_enabled": true,
        "notification_sound": true,
        "shortcuts": {
            "toggle_window": "Ctrl+Shift+N",
            "quick_capture": "Ctrl+Shift+C",
            "reload": "Ctrl+R",
            "zoom_in": "Ctrl+=",
            "zoom_out": "Ctrl+-",
            "zoom_reset": "Ctrl+0"
        },
        "auto_update": true,
        "update_channel": "stable",
        "autostart_enabled": false,
        "hardware_acceleration": true,
        "spellcheck": true
    }"#;
    
    let settings: UserSettings = serde_json::from_str(json).unwrap();
    assert_eq!(settings.zoom_level, 1.5);
    assert_eq!(settings.theme, crate::config::Theme::Dark);
}

#[test]
fn test_settings_round_trip() {
    let original = UserSettings::default();
    let json = serde_json::to_string(&original).unwrap();
    let restored: UserSettings = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.zoom_level, restored.zoom_level);
    assert_eq!(original.theme, restored.theme);
    assert_eq!(original.start_minimized, restored.start_minimized);
}
