//! Unit tests for handlers module.

use super::navigation::*;

#[test]
fn test_host_matches_domain() {
    // Exact match
    assert!(host_matches_domain("notion.so", "notion.so"));
    
    // Subdomain match
    assert!(host_matches_domain("www.notion.so", "notion.so"));
    assert!(host_matches_domain("api.notion.so", "notion.so"));
    
    // No match
    assert!(!host_matches_domain("evil-notion.so", "notion.so"));
    assert!(!host_matches_domain("notion.com", "notion.so"));
    assert!(!host_matches_domain("example.com", "notion.so"));
}

#[test]
fn test_should_open_externally() {
    // Notion domains should stay in webview
    assert!(!should_open_externally("https://www.notion.so"));
    assert!(!should_open_externally("https://notion.so"));
    assert!(!should_open_externally("https://api.notion.so"));
    assert!(!should_open_externally("https://www.notion-static.com"));
    
    // External domains should open externally
    assert!(should_open_externally("https://example.com"));
    assert!(should_open_externally("https://google.com"));
    assert!(should_open_externally("https://github.com"));
}

#[test]
fn test_is_oauth_url() {
    // OAuth providers
    assert!(is_oauth_url("https://accounts.google.com"));
    assert!(is_oauth_url("https://login.microsoftonline.com"));
    assert!(is_oauth_url("https://appleid.apple.com"));
    assert!(is_oauth_url("https://github.com"));
    
    // Not OAuth
    assert!(!is_oauth_url("https://www.notion.so"));
    assert!(!is_oauth_url("https://example.com"));
}

#[test]
fn test_should_open_externally_with_paths() {
    assert!(!should_open_externally("https://www.notion.so/page"));
    assert!(!should_open_externally("https://www.notion.so/page?query=value"));
    assert!(should_open_externally("https://example.com/page"));
}

#[test]
fn test_should_open_externally_invalid_url() {
    // Invalid URLs should default to opening externally
    assert!(should_open_externally("not-a-url"));
    assert!(should_open_externally(""));
}
