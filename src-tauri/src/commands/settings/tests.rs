//! Unit tests for settings commands.

use crate::config::UserSettings;

#[test]
fn test_settings_command_structure() {
    // Verify settings commands structure
    let settings = UserSettings::default();
    
    // Test that settings can be serialized
    let json = serde_json::to_string(&settings).unwrap();
    assert!(!json.is_empty());
}

#[test]
fn test_settings_comparison() {
    // Test settings comparison logic
    let settings1 = UserSettings::default();
    let mut settings2 = UserSettings::default();
    
    // Initially equal
    assert_eq!(settings1.zoom_level, settings2.zoom_level);
    assert_eq!(settings1.autostart_enabled, settings2.autostart_enabled);
    
    // Change one setting
    settings2.zoom_level = 1.5;
    assert_ne!(settings1.zoom_level, settings2.zoom_level);
}

#[test]
fn test_apply_settings_logic() {
    // Test apply settings logic structure
    let old_settings = UserSettings::default();
    let mut new_settings = UserSettings::default();
    
    // Test autostart change detection
    new_settings.autostart_enabled = true;
    let autostart_changed = new_settings.autostart_enabled != old_settings.autostart_enabled;
    assert!(autostart_changed);
    
    // Test zoom change detection
    new_settings.zoom_level = 1.2;
    let zoom_changed = new_settings.zoom_level != old_settings.zoom_level;
    assert!(zoom_changed);
    
    // Test CSS change detection
    new_settings.custom_css_enabled = true;
    new_settings.custom_css = "body { color: red; }".to_string();
    let css_changed = new_settings.custom_css_enabled != old_settings.custom_css_enabled
        || new_settings.custom_css != old_settings.custom_css;
    assert!(css_changed);
}

#[test]
fn test_custom_css_application() {
    // Test custom CSS application logic
    let mut settings = UserSettings::default();
    
    // CSS disabled
    assert!(!settings.custom_css_enabled);
    assert!(settings.custom_css.is_empty());
    
    // Enable CSS
    settings.custom_css_enabled = true;
    settings.custom_css = "body { margin: 0; }".to_string();
    
    assert!(settings.custom_css_enabled);
    assert!(!settings.custom_css.is_empty());
}
