//! Unit tests for configuration module.

use super::{ShortcutSettings, Theme, UpdateChannel, UserSettings};
use serde_json;

#[test]
fn test_user_settings_default() {
    let settings = UserSettings::default();
    
    assert!(!settings.start_minimized);
    assert!(settings.minimize_to_tray);
    assert!(settings.close_to_tray);
    assert_eq!(settings.zoom_level, 1.0);
    assert!(!settings.custom_css_enabled);
    assert!(settings.notifications_enabled);
    assert!(settings.auto_update);
    assert_eq!(settings.update_channel, UpdateChannel::Stable);
    assert!(!settings.autostart_enabled);
    assert!(settings.hardware_acceleration);
    assert!(settings.spellcheck);
}

#[test]
fn test_shortcut_settings_default() {
    let shortcuts = ShortcutSettings::default();
    
    assert_eq!(shortcuts.toggle_window, "CommandOrControl+Shift+N");
    assert_eq!(shortcuts.quick_capture, "CommandOrControl+Shift+C");
    assert_eq!(shortcuts.reload, "CommandOrControl+R");
    assert_eq!(shortcuts.zoom_in, "CommandOrControl+=");
    assert_eq!(shortcuts.zoom_out, "CommandOrControl+-");
    assert_eq!(shortcuts.zoom_reset, "CommandOrControl+0");
}

#[test]
fn test_shortcut_settings_equality() {
    let shortcuts1 = ShortcutSettings::default();
    let shortcuts2 = ShortcutSettings::default();
    
    assert_eq!(shortcuts1, shortcuts2);
    
    let mut shortcuts3 = ShortcutSettings::default();
    shortcuts3.toggle_window = "Alt+T".to_string();
    assert_ne!(shortcuts1, shortcuts3);
}

#[test]
fn test_update_channel_serialization() {
    assert_eq!(serde_json::to_string(&UpdateChannel::Stable).unwrap(), "\"stable\"");
    assert_eq!(serde_json::to_string(&UpdateChannel::Beta).unwrap(), "\"beta\"");
    assert_eq!(serde_json::to_string(&UpdateChannel::Nightly).unwrap(), "\"nightly\"");
}

#[test]
fn test_update_channel_deserialization() {
    assert_eq!(
        serde_json::from_str::<UpdateChannel>("\"stable\"").unwrap(),
        UpdateChannel::Stable
    );
    assert_eq!(
        serde_json::from_str::<UpdateChannel>("\"beta\"").unwrap(),
        UpdateChannel::Beta
    );
    assert_eq!(
        serde_json::from_str::<UpdateChannel>("\"nightly\"").unwrap(),
        UpdateChannel::Nightly
    );
}

#[test]
fn test_user_settings_serialization() {
    let settings = UserSettings::default();
    let json = serde_json::to_string(&settings).unwrap();
    
    // Should serialize without errors
    assert!(!json.is_empty());
    
    // Should deserialize back
    let deserialized: UserSettings = serde_json::from_str(&json).unwrap();
    assert_eq!(settings.zoom_level, deserialized.zoom_level);
    assert_eq!(settings.start_minimized, deserialized.start_minimized);
}

#[test]
fn test_theme_default() {
    let settings = UserSettings::default();
    assert_eq!(settings.theme, Theme::System);
}

#[test]
fn test_theme_serialization() {
    assert_eq!(serde_json::to_string(&Theme::System).unwrap(), "\"system\"");
    assert_eq!(serde_json::to_string(&Theme::Light).unwrap(), "\"light\"");
    assert_eq!(serde_json::to_string(&Theme::Dark).unwrap(), "\"dark\"");
}

#[test]
fn test_theme_deserialization() {
    assert_eq!(
        serde_json::from_str::<Theme>("\"system\"").unwrap(),
        Theme::System
    );
    assert_eq!(
        serde_json::from_str::<Theme>("\"light\"").unwrap(),
        Theme::Light
    );
    assert_eq!(
        serde_json::from_str::<Theme>("\"dark\"").unwrap(),
        Theme::Dark
    );
}

#[test]
fn test_theme_equality() {
    assert_eq!(Theme::System, Theme::System);
    assert_eq!(Theme::Light, Theme::Light);
    assert_eq!(Theme::Dark, Theme::Dark);
    assert_ne!(Theme::System, Theme::Light);
    assert_ne!(Theme::Light, Theme::Dark);
}

#[test]
fn test_user_settings_with_theme() {
    let mut settings = UserSettings::default();
    settings.theme = Theme::Dark;
    
    let json = serde_json::to_string(&settings).unwrap();
    let deserialized: UserSettings = serde_json::from_str(&json).unwrap();
    
    assert_eq!(deserialized.theme, Theme::Dark);
}
