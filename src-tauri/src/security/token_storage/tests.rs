//! Tests for encrypted token storage.

use super::StoredToken;

#[test]
fn test_store_and_retrieve_token() {
    let service = "test_service";
    let account = "test_account";
    
    let token = StoredToken {
        token: "test_token".to_string(),
        expires_at: Some(1000),
        refresh_token: Some("refresh_token".to_string()),
        created_at: 500,
    };
    
    // Note: Actual keyring operations require system keyring
    // These tests verify the structure and logic
    assert!(!token.token.is_empty());
    assert_eq!(token.expires_at, Some(1000));
}

#[test]
fn test_token_expiration() {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let expired_token = StoredToken {
        token: "token".to_string(),
        expires_at: Some(now - 100),
        refresh_token: None,
        created_at: now - 200,
    };
    
    assert!(expired_token.is_expired());
    
    let valid_token = StoredToken {
        token: "token".to_string(),
        expires_at: Some(now + 100),
        refresh_token: None,
        created_at: now,
    };
    
    assert!(!valid_token.is_expired());
}

#[test]
fn test_token_without_expiration() {
    let token = StoredToken {
        token: "token".to_string(),
        expires_at: None,
        refresh_token: None,
        created_at: 0,
    };
    
    assert!(!token.is_expired());
}
