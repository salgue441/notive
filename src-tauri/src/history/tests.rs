//! Tests for history management.

use super::{HistoryEntry, HistoryManager};

#[test]
fn test_history_entry_creation() {
    let entry = HistoryEntry::new("Test Page".to_string(), "https://www.notion.so/test".to_string());
    assert_eq!(entry.title, "Test Page");
    assert_eq!(entry.url, "https://www.notion.so/test");
    assert_eq!(entry.visit_count, 1);
    assert!(!entry.id.is_empty());
    assert!(!entry.visited_at.is_empty());
}

#[test]
fn test_history_manager_add() {
    let mut manager = HistoryManager::new(100);
    let entry = HistoryEntry::new("Test".to_string(), "https://example.com".to_string());
    
    manager.add_entry(entry);
    
    let recent = manager.get_recent(10);
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].title, "Test");
}

#[test]
fn test_history_manager_duplicate_url() {
    let mut manager = HistoryManager::new(100);
    let entry1 = HistoryEntry::new("First Visit".to_string(), "https://example.com".to_string());
    let entry2 = HistoryEntry::new("Second Visit".to_string(), "https://example.com".to_string());
    
    manager.add_entry(entry1);
    manager.add_entry(entry2);
    
    let recent = manager.get_recent(10);
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].visit_count, 2);
    assert_eq!(recent[0].title, "Second Visit"); // Title updated
}

#[test]
fn test_history_manager_limit() {
    let mut manager = HistoryManager::new(5);
    
    for i in 0..10 {
        let entry = HistoryEntry::new(
            format!("Page {}", i),
            format!("https://example.com/{}", i),
        );
        manager.add_entry(entry);
    }
    
    let recent = manager.get_recent(10);
    assert_eq!(recent.len(), 5); // Should be limited to max_entries
}

#[test]
fn test_history_manager_get_recent() {
    let mut manager = HistoryManager::new(100);
    
    for i in 0..20 {
        let entry = HistoryEntry::new(
            format!("Page {}", i),
            format!("https://example.com/{}", i),
        );
        manager.add_entry(entry);
    }
    
    let recent = manager.get_recent(5);
    assert_eq!(recent.len(), 5);
    
    // Most recent should be first
    assert_eq!(recent[0].title, "Page 19");
}

#[test]
fn test_history_manager_clear() {
    let mut manager = HistoryManager::new(100);
    
    for i in 0..5 {
        let entry = HistoryEntry::new(
            format!("Page {}", i),
            format!("https://example.com/{}", i),
        );
        manager.add_entry(entry);
    }
    
    assert_eq!(manager.get_recent(10).len(), 5);
    
    manager.clear();
    assert_eq!(manager.get_recent(10).len(), 0);
}

#[test]
fn test_history_manager_remove_entry() {
    let mut manager = HistoryManager::new(100);
    let entry = HistoryEntry::new("Test".to_string(), "https://example.com".to_string());
    let id = entry.id.clone();
    
    manager.add_entry(entry);
    assert_eq!(manager.get_recent(10).len(), 1);
    
    let removed = manager.remove_entry(&id);
    assert!(removed.is_some());
    assert_eq!(manager.get_recent(10).len(), 0);
}

#[test]
fn test_history_manager_remove_nonexistent() {
    let mut manager = HistoryManager::new(100);
    let entry = HistoryEntry::new("Test".to_string(), "https://example.com".to_string());
    manager.add_entry(entry);
    
    let removed = manager.remove_entry("nonexistent-id");
    assert!(removed.is_none());
    assert_eq!(manager.get_recent(10).len(), 1);
}
