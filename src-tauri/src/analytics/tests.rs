//! Tests for page analytics.

use super::{PageView, PageEdit, ActivityEntry, UsageStats};

#[test]
fn test_page_view_creation() {
    let view = PageView {
        page_id: "page-1".to_string(),
        page_url: "https://www.notion.so/page-1".to_string(),
        page_title: "Test Page".to_string(),
        viewed_at: 1000,
        duration: Some(60),
    };
    
    assert_eq!(view.page_id, "page-1");
    assert_eq!(view.page_title, "Test Page");
    assert_eq!(view.duration, Some(60));
}

#[test]
fn test_page_edit_creation() {
    let edit = PageEdit {
        page_id: "page-1".to_string(),
        page_url: "https://www.notion.so/page-1".to_string(),
        edited_at: 1000,
        edit_type: "modified".to_string(),
    };
    
    assert_eq!(edit.page_id, "page-1");
    assert_eq!(edit.edit_type, "modified");
}

#[test]
fn test_activity_entry_creation() {
    let entry = ActivityEntry {
        id: "entry-1".to_string(),
        page_id: "page-1".to_string(),
        page_url: "https://example.com".to_string(),
        page_title: "Test".to_string(),
        activity_type: "view".to_string(),
        timestamp: 1000,
        metadata: None,
    };
    
    assert_eq!(entry.activity_type, "view");
    assert_eq!(entry.timestamp, 1000);
}

#[test]
fn test_usage_stats_structure() {
    let stats = UsageStats {
        total_views: 100,
        total_edits: 50,
        total_pages: 10,
        most_viewed_pages: vec![("https://example.com".to_string(), 20)],
        recent_activity: vec![],
    };
    
    assert_eq!(stats.total_views, 100);
    assert_eq!(stats.total_edits, 50);
    assert_eq!(stats.total_pages, 10);
}
