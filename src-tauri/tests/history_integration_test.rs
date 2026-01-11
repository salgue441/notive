//! Integration tests for history functionality.

use notive::history::HistoryEntry;

#[test]
fn test_history_entry_serialization() {
    let entry = HistoryEntry::new("Test Page".to_string(), "https://www.notion.so/test".to_string());
    let json = serde_json::to_string(&entry).unwrap();
    
    assert!(!json.is_empty());
    assert!(json.contains("Test Page"));
    assert!(json.contains("visit_count"));
}

#[test]
fn test_history_entry_deserialization() {
    let json = r#"{
        "id": "test-id",
        "title": "Test Page",
        "url": "https://example.com",
        "visited_at": "2025-01-15T10:00:00Z",
        "visit_count": 5
    }"#;
    
    let entry: HistoryEntry = serde_json::from_str(json).unwrap();
    assert_eq!(entry.title, "Test Page");
    assert_eq!(entry.visit_count, 5);
}

#[test]
fn test_history_entry_round_trip() {
    let original = HistoryEntry::new("Test".to_string(), "https://example.com".to_string());
    let json = serde_json::to_string(&original).unwrap();
    let restored: HistoryEntry = serde_json::from_str(&json).unwrap();
    
    assert_eq!(original.title, restored.title);
    assert_eq!(original.url, restored.url);
    assert_eq!(original.visit_count, restored.visit_count);
}
