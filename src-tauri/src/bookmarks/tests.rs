//! Tests for bookmark management.

use super::{Bookmark, BookmarkManager};

#[test]
fn test_bookmark_creation() {
    let bookmark = Bookmark::new("Test Page".to_string(), "https://www.notion.so/test".to_string());
    assert_eq!(bookmark.title, "Test Page");
    assert_eq!(bookmark.url, "https://www.notion.so/test");
    assert!(!bookmark.id.is_empty());
    assert!(!bookmark.created_at.is_empty());
    assert!(bookmark.tags.is_empty());
}

#[test]
fn test_bookmark_manager_add() {
    let mut manager = BookmarkManager::new();
    let bookmark = Bookmark::new("Test".to_string(), "https://example.com".to_string());
    let id = bookmark.id.clone();
    
    manager.add_bookmark(bookmark);
    
    assert_eq!(manager.list_bookmarks().len(), 1);
    assert!(manager.get_bookmark(&id).is_some());
}

#[test]
fn test_bookmark_manager_remove() {
    let mut manager = BookmarkManager::new();
    let bookmark = Bookmark::new("Test".to_string(), "https://example.com".to_string());
    let id = bookmark.id.clone();
    
    manager.add_bookmark(bookmark);
    assert_eq!(manager.list_bookmarks().len(), 1);
    
    let removed = manager.remove_bookmark(&id);
    assert!(removed.is_some());
    assert_eq!(manager.list_bookmarks().len(), 0);
}

#[test]
fn test_bookmark_manager_search() {
    let mut manager = BookmarkManager::new();
    
    manager.add_bookmark(Bookmark::new(
        "Notion Workspace".to_string(),
        "https://www.notion.so/workspace".to_string(),
    ));
    manager.add_bookmark(Bookmark::new(
        "Test Page".to_string(),
        "https://www.notion.so/test".to_string(),
    ));
    manager.add_bookmark(Bookmark::new(
        "Another Page".to_string(),
        "https://www.notion.so/another".to_string(),
    ));
    
    let results = manager.search_bookmarks("Notion");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Notion Workspace");
    
    let results = manager.search_bookmarks("test");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Test Page");
}

#[test]
fn test_bookmark_search_case_insensitive() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark(Bookmark::new(
        "Test Page".to_string(),
        "https://example.com".to_string(),
    ));
    
    let results = manager.search_bookmarks("TEST");
    assert_eq!(results.len(), 1);
    
    let results = manager.search_bookmarks("page");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_bookmark_search_by_url() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark(Bookmark::new(
        "Test".to_string(),
        "https://www.notion.so/my-page".to_string(),
    ));
    
    let results = manager.search_bookmarks("notion");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_bookmark_search_by_tags() {
    let mut manager = BookmarkManager::new();
    let mut bookmark = Bookmark::new("Test".to_string(), "https://example.com".to_string());
    bookmark.tags = vec!["work".to_string(), "important".to_string()];
    manager.add_bookmark(bookmark);
    
    let results = manager.search_bookmarks("work");
    assert_eq!(results.len(), 1);
    
    let results = manager.search_bookmarks("important");
    assert_eq!(results.len(), 1);
}

#[test]
fn test_bookmark_empty_search() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark(Bookmark::new("Test".to_string(), "https://example.com".to_string()));
    
    let results = manager.search_bookmarks("");
    assert_eq!(results.len(), 1); // Empty query matches all
}

#[test]
fn test_bookmark_no_results() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark(Bookmark::new("Test".to_string(), "https://example.com".to_string()));
    
    let results = manager.search_bookmarks("nonexistent");
    assert_eq!(results.len(), 0);
}
