//! Encrypted token storage using keyring.

#[cfg(test)]
mod tests;

use keyring::Entry;
use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Stored token with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub token: String,
    pub expires_at: Option<u64>, // Unix timestamp
    pub refresh_token: Option<String>,
    pub created_at: u64,
}

impl StoredToken {
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expires_at
        } else {
            false
        }
    }
}

/// Stores an encrypted token securely.
pub fn store_token(service: &str, account: &str, token: StoredToken) -> Result<(), String> {
    let entry = Entry::new(service, account).map_err(|e| format!("Keyring error: {}", e))?;
    
    let json = serde_json::to_string(&token)
        .map_err(|e| format!("Serialization error: {}", e))?;
    
    entry.set_password(&json).map_err(|e| format!("Failed to store token: {}", e))?;
    
    log::debug!("Token stored securely for {}:{}", service, account);
    Ok(())
}

/// Retrieves an encrypted token.
pub fn get_token(service: &str, account: &str) -> Result<Option<StoredToken>, String> {
    let entry = Entry::new(service, account).map_err(|e| format!("Keyring error: {}", e))?;
    
    match entry.get_password() {
        Ok(json) => {
            let token: StoredToken = serde_json::from_str(&json)
                .map_err(|e| format!("Deserialization error: {}", e))?;
            Ok(Some(token))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to retrieve token: {}", e)),
    }
}

/// Deletes a stored token.
pub fn delete_token(service: &str, account: &str) -> Result<(), String> {
    let entry = Entry::new(service, account).map_err(|e| format!("Keyring error: {}", e))?;
    
    entry.delete_password().map_err(|e| format!("Failed to delete token: {}", e))?;
    
    log::debug!("Token deleted for {}:{}", service, account);
    Ok(())
}

/// Lists all stored tokens for a service.
pub fn list_tokens(service: &str) -> Result<Vec<String>, String> {
    // Note: keyring doesn't support listing entries directly
    // This is a limitation - we'd need to maintain our own index
    // For now, return empty list
    log::warn!("Token listing not fully supported by keyring");
    Ok(vec![])
}

/// Refreshes a token if it's expired.
pub fn refresh_token_if_needed(
    service: &str,
    account: &str,
    refresh_fn: impl FnOnce(&str) -> Result<StoredToken, String>,
) -> Result<Option<StoredToken>, String> {
    let stored = get_token(service, account)?;
    
    if let Some(token) = stored {
        if token.is_expired() {
            if let Some(refresh_token) = &token.refresh_token {
                log::info!("Token expired, refreshing...");
                let new_token = refresh_fn(refresh_token)?;
                store_token(service, account, new_token.clone())?;
                return Ok(Some(new_token));
            } else {
                log::warn!("Token expired but no refresh token available");
                return Ok(None);
            }
        }
        Ok(Some(token))
    } else {
        Ok(None)
    }
}
