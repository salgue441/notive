//! Full OAuth implementation for secure authentication.

#[cfg(test)]
mod tests;

use crate::security::token_storage;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// OAuth provider configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: Option<String>, // Stored securely
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

/// OAuth token response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: String,
    pub scope: Option<String>,
}

/// Starts OAuth flow for a provider.
#[tauri::command]
pub async fn start_oauth_flow<R: Runtime>(
    app: AppHandle<R>,
    provider_name: String,
) -> Result<String, String> {
    log::debug!("Starting OAuth flow for: {}", provider_name);
    
    // Get provider config
    let provider = get_provider_config(&provider_name)?;
    
    // Generate state for CSRF protection
    let state = uuid::Uuid::new_v4().to_string();
    
    // Build authorization URL
    let mut auth_url = url::Url::parse(&provider.auth_url)
        .map_err(|e| format!("Invalid auth URL: {}", e))?;
    
    auth_url.query_pairs_mut()
        .append_pair("client_id", &provider.client_id)
        .append_pair("redirect_uri", &provider.redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("state", &state)
        .append_pair("scope", &provider.scopes.join(" "));
    
    // Store state for verification
    token_storage::store_token(
        "notive",
        &format!("oauth_state_{}", provider_name),
        token_storage::StoredToken {
            token: state,
            expires_at: Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() + 600, // 10 minutes
            ),
            refresh_token: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .map_err(|e| format!("Failed to store state: {}", e))?;
    
    Ok(auth_url.to_string())
}

/// Handles OAuth callback.
#[tauri::command]
pub async fn handle_oauth_callback<R: Runtime>(
    app: AppHandle<R>,
    provider_name: String,
    code: String,
    state: String,
) -> Result<OAuthToken, String> {
    log::debug!("Handling OAuth callback for: {}", provider_name);
    
    // Verify state
    let stored_state = token_storage::get_token("notive", &format!("oauth_state_{}", provider_name))
        .map_err(|e| format!("Failed to get state: {}", e))?
        .ok_or_else(|| "State not found".to_string())?;
    
    if stored_state.token != state {
        return Err("Invalid state".to_string());
    }
    
    // Get provider config
    let provider = get_provider_config(&provider_name)?;
    
    // Exchange code for token
    let client = reqwest::Client::new();
    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "authorization_code");
    params.insert("code", &code);
    params.insert("redirect_uri", &provider.redirect_uri);
    params.insert("client_id", &provider.client_id);
    
    if let Some(ref secret) = provider.client_secret {
        params.insert("client_secret", secret);
    }
    
    let response = client
        .post(&provider.token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Token request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Token request failed: {}", response.status()));
    }
    
    let token: OAuthToken = response.json().await
        .map_err(|e| format!("Failed to parse token: {}", e))?;
    
    // Store token securely
    let expires_at = token.expires_in.map(|expires_in| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + expires_in
    });
    
    token_storage::store_token(
        "notive",
        &format!("oauth_token_{}", provider_name),
        token_storage::StoredToken {
            token: token.access_token.clone(),
            expires_at,
            refresh_token: token.refresh_token.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .map_err(|e| format!("Failed to store token: {}", e))?;
    
    log::info!("OAuth token stored securely for: {}", provider_name);
    Ok(token)
}

/// Refreshes OAuth token.
#[tauri::command]
pub async fn refresh_oauth_token<R: Runtime>(
    app: AppHandle<R>,
    provider_name: String,
) -> Result<OAuthToken, String> {
    log::debug!("Refreshing OAuth token for: {}", provider_name);
    
    // Get stored token
    let stored = token_storage::get_token("notive", &format!("oauth_token_{}", provider_name))
        .map_err(|e| format!("Failed to get token: {}", e))?
        .ok_or_else(|| "Token not found".to_string())?;
    
    let refresh_token = stored.refresh_token
        .ok_or_else(|| "No refresh token available".to_string())?;
    
    // Get provider config
    let provider = get_provider_config(&provider_name)?;
    
    // Refresh token
    let client = reqwest::Client::new();
    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "refresh_token");
    params.insert("refresh_token", &refresh_token);
    params.insert("client_id", &provider.client_id);
    
    if let Some(ref secret) = provider.client_secret {
        params.insert("client_secret", secret);
    }
    
    let response = client
        .post(&provider.token_url)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Token refresh failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Token refresh failed: {}", response.status()));
    }
    
    let token: OAuthToken = response.json().await
        .map_err(|e| format!("Failed to parse token: {}", e))?;
    
    // Store new token
    let expires_at = token.expires_in.map(|expires_in| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() + expires_in
    });
    
    token_storage::store_token(
        "notive",
        &format!("oauth_token_{}", provider_name),
        token_storage::StoredToken {
            token: token.access_token.clone(),
            expires_at,
            refresh_token: token.refresh_token.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .map_err(|e| format!("Failed to store token: {}", e))?;
    
    Ok(token)
}

/// Gets provider configuration.
fn get_provider_config(provider_name: &str) -> Result<OAuthProvider, String> {
    // In a full implementation, we'd load from config
    // For now, return placeholder configs
    match provider_name {
        "google" => Ok(OAuthProvider {
            name: "Google".to_string(),
            client_id: "".to_string(), // Would be from config
            client_secret: None,
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uri: "http://localhost:8080/oauth/callback".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
        }),
        _ => Err(format!("Unknown provider: {}", provider_name)),
    }
}
