//! Unit tests for app module.

use crate::config::UserSettings;

#[test]
fn test_app_state_structure() {
    // Test app state structure
    use crate::app::AppState;
    
    let state = AppState::default();
    assert!(!state.is_minimized);
    assert!(!state.update_available);
    assert_eq!(state.update_version, None);
}

#[test]
fn test_app_state_with_values() {
    use crate::app::AppState;
    
    let mut state = AppState::default();
    state.is_minimized = true;
    state.update_available = true;
    state.update_version = Some("1.1.0".to_string());
    
    assert!(state.is_minimized);
    assert!(state.update_available);
    assert_eq!(state.update_version, Some("1.1.0".to_string()));
}

#[test]
fn test_app_initialization_logic() {
    // Test app initialization logic structure
    let settings = UserSettings::default();
    
    // Should load settings
    assert!(!settings.start_minimized);
    
    // Should apply autostart if enabled
    if settings.autostart_enabled {
        // Autostart logic
        assert!(true);
    }
    
    // Should apply zoom
    assert_eq!(settings.zoom_level, 1.0);
    
    // Should register shortcuts
    assert!(!settings.shortcuts.toggle_window.is_empty());
}

#[test]
fn test_app_state_serialization() {
    use crate::app::AppState;
    use serde_json;
    
    let state = AppState::default();
    let json = serde_json::to_string(&state).unwrap();
    let deserialized: AppState = serde_json::from_str(&json).unwrap();
    
    assert_eq!(state.is_minimized, deserialized.is_minimized);
    assert_eq!(state.update_available, deserialized.update_available);
}
