//! Integration tests for bookmark functionality.

use notive::bookmarks::Bookmark;

#[test]
fn test_bookmark_serialization() {
    let bookmark = Bookmark::new("Test Page".to_string(), "https://www.notion.so/test".to_string());
    let json = serde_json::to_string(&bookmark).unwrap();
    
    assert!(!json.is_empty());
    assert!(json.contains("Test Page"));
    assert!(json.contains("https://www.notion.so/test"));
}

#[test]
fn test_bookmark_deserialization() {
    let json = r#"{
        "id": "test-id",
        "title": "Test Page",
        "url": "https://example.com",
        "icon": null,
        "created_at": "2025-01-15T10:00:00Z",
        "tags": []
    }"#;
    
    let bookmark: Bookmark = serde_json::from_str(json).unwrap();
    assert_eq!(bookmark.title, "Test Page");
    assert_eq!(bookmark.url, "https://example.com");
}

#[test]
fn test_bookmark_with_tags() {
    let mut bookmark = Bookmark::new("Test".to_string(), "https://example.com".to_string());
    bookmark.tags = vec!["work".to_string(), "important".to_string()];
    
    let json = serde_json::to_string(&bookmark).unwrap();
    let restored: Bookmark = serde_json::from_str(&json).unwrap();
    
    assert_eq!(restored.tags.len(), 2);
    assert!(restored.tags.contains(&"work".to_string()));
}
