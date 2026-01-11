//! Integration tests for theme functionality.

use notive::config::{Theme, UserSettings};

#[test]
fn test_theme_serialization() {
    let mut settings = UserSettings::default();
    settings.theme = Theme::Dark;
    
    let json = serde_json::to_string(&settings).unwrap();
    assert!(json.contains("\"theme\":\"dark\""));
}

#[test]
fn test_theme_deserialization() {
    let json = r#"{
        "start_minimized": false,
        "minimize_to_tray": true,
        "close_to_tray": true,
        "zoom_level": 1.0,
        "theme": "light",
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
    assert_eq!(settings.theme, Theme::Light);
}

#[test]
fn test_all_theme_variants() {
    let themes = vec![Theme::System, Theme::Light, Theme::Dark];
    
    for theme in themes {
        let json = serde_json::to_string(&theme).unwrap();
        let restored: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme, restored);
    }
}
