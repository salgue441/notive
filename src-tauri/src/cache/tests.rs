//! Tests for caching system.

use super::{CacheEntry, CacheType, CacheStats};
use std::collections::HashMap;

#[test]
fn test_cache_entry_creation() {
    let entry = CacheEntry {
        key: "cache-key".to_string(),
        url: "https://example.com".to_string(),
        cached_at: 1000,
        expires_at: Some(2000),
        size_bytes: 1024,
        cache_type: CacheType::Page,
    };
    
    assert_eq!(entry.key, "cache-key");
    assert_eq!(entry.cache_type, CacheType::Page);
}

#[test]
fn test_cache_type_serialization() {
    assert_eq!(
        serde_json::to_string(&CacheType::Page).unwrap(),
        "\"Page\""
    );
    assert_eq!(
        serde_json::to_string(&CacheType::Image).unwrap(),
        "\"Image\""
    );
}

#[test]
fn test_cache_stats_structure() {
    let mut entries = HashMap::new();
    entries.insert("page".to_string(), 10);
    entries.insert("image".to_string(), 5);
    
    let stats = CacheStats {
        total_entries: 15,
        total_size_mb: 50.0,
        entries_by_type: entries,
    };
    
    assert_eq!(stats.total_entries, 15);
    assert_eq!(stats.total_size_mb, 50.0);
}
