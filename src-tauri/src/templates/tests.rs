//! Tests for page templates.

use super::PageTemplate;

#[test]
fn test_template_creation() {
    let template = PageTemplate::new(
        "Test Template".to_string(),
        "https://www.notion.so/test".to_string(),
        Some("Test description".to_string()),
        Some("work".to_string()),
    );
    
    assert_eq!(template.name, "Test Template");
    assert_eq!(template.url, "https://www.notion.so/test");
    assert_eq!(template.usage_count, 0);
    assert!(!template.id.is_empty());
}

#[test]
fn test_template_increment_usage() {
    let mut template = PageTemplate::new(
        "Test".to_string(),
        "https://example.com".to_string(),
        None,
        None,
    );
    
    assert_eq!(template.usage_count, 0);
    template.increment_usage();
    assert_eq!(template.usage_count, 1);
    template.increment_usage();
    assert_eq!(template.usage_count, 2);
}

#[test]
fn test_template_serialization() {
    let template = PageTemplate::new(
        "Test".to_string(),
        "https://example.com".to_string(),
        None,
        None,
    );
    
    let json = serde_json::to_string(&template).unwrap();
    assert!(!json.is_empty());
    assert!(json.contains("Test"));
}

#[test]
fn test_template_deserialization() {
    let json = r#"{
        "id": "test-id",
        "name": "Test Template",
        "url": "https://example.com",
        "description": null,
        "category": null,
        "tags": [],
        "icon": null,
        "created_at": "2025-01-15T10:00:00Z",
        "usage_count": 5
    }"#;
    
    let template: PageTemplate = serde_json::from_str(json).unwrap();
    assert_eq!(template.name, "Test Template");
    assert_eq!(template.usage_count, 5);
}
