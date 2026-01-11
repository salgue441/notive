//! Multi-account support with OAuth integration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime};

/// Account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub provider: AccountProvider,
    pub email: String,
    pub name: Option<String>,
    pub access_token: Option<String>, // Encrypted in production
    pub refresh_token: Option<String>, // Encrypted in production
    pub is_active: bool,
}

/// OAuth provider types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccountProvider {
    Google,
    Apple,
    Microsoft,
    Email, // Email/password login
}

/// Manages multiple accounts.
pub struct AccountManager {
    accounts: HashMap<String, Account>,
    active_account: Option<String>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            active_account: None,
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id.clone(), account);
    }

    pub fn remove_account(&mut self, id: &str) -> Option<Account> {
        self.accounts.remove(id)
    }

    pub fn get_account(&self, id: &str) -> Option<&Account> {
        self.accounts.get(id)
    }

    pub fn list_accounts(&self) -> Vec<&Account> {
        self.accounts.values().collect()
    }

    pub fn set_active(&mut self, id: Option<String>) {
        self.active_account = id;
    }

    pub fn get_active(&self) -> Option<&Account> {
        self.active_account
            .as_ref()
            .and_then(|id| self.accounts.get(id))
    }
}

/// Initiates OAuth flow for a provider.
#[tauri::command]
pub async fn start_oauth_flow<R: Runtime>(
    app: AppHandle<R>,
    provider: AccountProvider,
) -> Result<String, String> {
    log::debug!("Starting OAuth flow for provider: {:?}", provider);
    
    // OAuth URLs for different providers
    let oauth_url = match provider {
        AccountProvider::Google => {
            "https://accounts.google.com/o/oauth2/v2/auth?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI&response_type=code&scope=openid%20email%20profile"
        }
        AccountProvider::Apple => {
            "https://appleid.apple.com/auth/authorize?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI&response_type=code&scope=email%20name"
        }
        AccountProvider::Microsoft => {
            "https://login.microsoftonline.com/common/oauth2/v2.0/authorize?client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URI&response_type=code&scope=openid%20email%20profile"
        }
        AccountProvider::Email => {
            return Err("Email login does not use OAuth".to_string());
        }
    };
    
    // Open OAuth URL in webview
    // The navigation handler will detect OAuth URLs and handle them appropriately
    if let Some(window) = app.get_webview_window("main") {
        window
            .eval(&format!("window.location.href = '{}';", oauth_url))
            .map_err(|e| e.to_string())?;
    }
    
    Ok("OAuth flow initiated".to_string())
}

/// Adds a new account.
#[tauri::command]
pub async fn add_account<R: Runtime>(
    app: AppHandle<R>,
    provider: AccountProvider,
    email: String,
    name: Option<String>,
) -> Result<String, String> {
    log::debug!("Adding account: {} ({:?})", email, provider);
    
    let account = Account {
        id: uuid::Uuid::new_v4().to_string(),
        provider,
        email,
        name,
        access_token: None,
        refresh_token: None,
        is_active: false,
    };
    
    // TODO: Store account securely (encrypted)
    // TODO: Persist to settings/store
    
    log::info!("Account added: {} ({})", account.email, account.id);
    Ok(account.id)
}

/// Lists all accounts.
#[tauri::command]
pub fn list_accounts<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Account>, String> {
    // TODO: Load from persistent storage
    // For now, return empty list
    Ok(vec![])
}

/// Switches to a different account.
#[tauri::command]
pub async fn switch_account<R: Runtime>(
    app: AppHandle<R>,
    account_id: String,
) -> Result<(), String> {
    log::debug!("Switching to account: {}", account_id);
    
    // TODO: Implement account switching
    // This would involve:
    // 1. Logging out of current account
    // 2. Loading account credentials
    // 3. Logging in with new account
    // 4. Updating window URL if needed
    
    Ok(())
}

/// Removes an account.
#[tauri::command]
pub async fn remove_account<R: Runtime>(
    app: AppHandle<R>,
    account_id: String,
) -> Result<(), String> {
    log::debug!("Removing account: {}", account_id);
    
    // TODO: Remove from storage
    // TODO: Clear credentials
    
    Ok(())
}
