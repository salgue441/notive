//! Tests for session management.

use super::{Session, SessionManager};

#[test]
fn test_session_creation() {
    let session = Session::new(
        "account-1".to_string(),
        "test@example.com".to_string(),
        Some(3600),
    );
    
    assert_eq!(session.account_id, "account-1");
    assert_eq!(session.account_email, "test@example.com");
    assert!(session.is_active);
    assert!(session.expires_at.is_some());
}

#[test]
fn test_session_expiration() {
    let session = Session::new(
        "account-1".to_string(),
        "test@example.com".to_string(),
        Some(0), // Expires immediately
    );
    
    // Wait a moment and check expiration
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(session.is_expired());
}

#[test]
fn test_session_manager() {
    let mut manager = SessionManager::new();
    
    let session1 = Session::new(
        "account-1".to_string(),
        "test1@example.com".to_string(),
        None,
    );
    
    manager.add_session(session1.clone());
    assert_eq!(manager.list_sessions().len(), 1);
    assert!(manager.get_active_session().is_some());
}

#[test]
fn test_session_manager_multiple_sessions() {
    let mut manager = SessionManager::new();
    
    let session1 = Session::new(
        "account-1".to_string(),
        "test1@example.com".to_string(),
        None,
    );
    let session2 = Session::new(
        "account-2".to_string(),
        "test2@example.com".to_string(),
        None,
    );
    
    manager.add_session(session1);
    manager.add_session(session2);
    
    assert_eq!(manager.list_sessions().len(), 2);
}

#[test]
fn test_session_manager_remove() {
    let mut manager = SessionManager::new();
    
    let session = Session::new(
        "account-1".to_string(),
        "test@example.com".to_string(),
        None,
    );
    let session_id = session.id.clone();
    
    manager.add_session(session);
    assert_eq!(manager.list_sessions().len(), 1);
    
    let removed = manager.remove_session(&session_id);
    assert!(removed.is_some());
    assert_eq!(manager.list_sessions().len(), 0);
}

#[test]
fn test_session_manager_cleanup_expired() {
    let mut manager = SessionManager::new();
    
    let expired_session = Session::new(
        "account-1".to_string(),
        "test@example.com".to_string(),
        Some(0), // Expires immediately
    );
    
    manager.add_session(expired_session);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    manager.cleanup_expired();
    assert_eq!(manager.list_sessions().len(), 0);
}
