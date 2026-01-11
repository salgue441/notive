//! Tests for offline mode.

use super::{OfflinePage, OfflineStatus};

#[test]
fn test_offline_page_structure() {
    let page = OfflinePage {
        url: "https://www.notion.so/page".to_string(),
        title: "Test Page".to_string(),
        content: "<html>...</html>".to_string(),
        cached_at: 1000,
        last_synced: Some(2000),
    };
    
    assert_eq!(page.title, "Test Page");
    assert!(page.last_synced.is_some());
}

#[test]
fn test_offline_status_structure() {
    let status = OfflineStatus {
        is_offline: true,
        cached_pages: 5,
        last_sync: Some(1000),
    };
    
    assert!(status.is_offline);
    assert_eq!(status.cached_pages, 5);
}
