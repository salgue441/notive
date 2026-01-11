//! Tests for OAuth implementation.

use super::{OAuthProvider, OAuthToken};

#[test]
fn test_oauth_provider_structure() {
    let provider = OAuthProvider {
        name: "Google".to_string(),
        client_id: "client-id".to_string(),
        client_secret: Some("client-secret".to_string()),
        auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
        token_url: "https://oauth2.googleapis.com/token".to_string(),
        redirect_uri: "http://localhost:8080/callback".to_string(),
        scopes: vec!["openid".to_string(), "email".to_string()],
    };
    
    assert_eq!(provider.name, "Google");
    assert_eq!(provider.scopes.len(), 2);
}

#[test]
fn test_oauth_token_structure() {
    let token = OAuthToken {
        access_token: "access-token".to_string(),
        refresh_token: Some("refresh-token".to_string()),
        expires_in: Some(3600),
        token_type: "Bearer".to_string(),
        scope: Some("openid email".to_string()),
    };
    
    assert_eq!(token.token_type, "Bearer");
    assert!(token.refresh_token.is_some());
}
