//! Unit tests for notification commands.

use crate::commands::notifications::NotificationPayload;

#[test]
fn test_notification_payload_structure() {
    let payload = NotificationPayload {
        title: "Test Title".to_string(),
        body: Some("Test Body".to_string()),
        icon: Some("https://example.com/icon.png".to_string()),
    };
    
    assert_eq!(payload.title, "Test Title");
    assert_eq!(payload.body, Some("Test Body".to_string()));
    assert_eq!(payload.icon, Some("https://example.com/icon.png".to_string()));
}

#[test]
fn test_notification_payload_without_options() {
    let payload = NotificationPayload {
        title: "Simple Title".to_string(),
        body: None,
        icon: None,
    };
    
    assert_eq!(payload.title, "Simple Title");
    assert_eq!(payload.body, None);
    assert_eq!(payload.icon, None);
}

#[test]
fn test_notification_payload_serialization() {
    let payload = NotificationPayload {
        title: "Test".to_string(),
        body: Some("Body".to_string()),
        icon: None,
    };
    
    let json = serde_json::to_string(&payload).unwrap();
    assert!(json.contains("Test"));
    assert!(json.contains("Body"));
}
