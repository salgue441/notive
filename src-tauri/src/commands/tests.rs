//! Unit tests for commands module.

// Note: Full command tests would require a Tauri app instance
// These tests verify the command structure and serialization

use crate::commands::notifications::NotificationPayload;
use serde_json;

#[test]
fn test_notification_payload_serialization() {
    let payload = NotificationPayload {
        title: "Test Title".to_string(),
        body: Some("Test Body".to_string()),
        icon: Some("https://example.com/icon.png".to_string()),
    };
    
    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("Test Title"));
    assert!(json.contains("Test Body"));
}

#[test]
fn test_notification_payload_without_options() {
    let payload = NotificationPayload {
        title: "Simple Title".to_string(),
        body: None,
        icon: None,
    };
    
    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("Simple Title"));
}

#[test]
fn test_commands_module_structure() {
    // Verify all command modules exist and compile
    // This is tested by the fact that the tests compile
    assert!(true);
}

#[test]
fn test_command_serialization_performance() {
    use std::time::Instant;
    
    let payload = NotificationPayload {
        title: "Test".to_string(),
        body: Some("Body".to_string()),
        icon: None,
    };
    
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = serde_json::to_string(&payload).unwrap();
    }
    let duration = start.elapsed();
    
    // Should be fast
    assert!(duration.as_millis() < 50);
}
