//! Tests for Notion API integration.

use super::{NotionApiConfig, NotionPageMetadata};

#[test]
fn test_notion_api_config_default() {
    let config = NotionApiConfig::default();
    
    assert_eq!(config.base_url, "https://api.notion.com/v1");
    assert_eq!(config.rate_limit_per_minute, 3);
    assert!(config.api_key.is_none());
}

#[test]
fn test_notion_page_metadata_structure() {
    let metadata = NotionPageMetadata {
        id: "page-id".to_string(),
        url: "https://www.notion.so/page-id".to_string(),
        title: "Test Page".to_string(),
        created_time: "2025-01-15T10:00:00Z".to_string(),
        last_edited_time: "2025-01-15T11:00:00Z".to_string(),
        created_by: Some("user-id".to_string()),
        last_edited_by: Some("user-id".to_string()),
    };
    
    assert_eq!(metadata.title, "Test Page");
    assert!(metadata.created_by.is_some());
}
