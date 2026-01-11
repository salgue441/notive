//! Navigation and URL handlers.

use crate::analytics;
use crate::history;
use tauri::{Manager, Runtime};
use tauri_plugin_shell::ShellExt;
use url::Url;

/// Domains allowed to load within the WebView.
const ALLOWED_HOSTS: &[&str] = &["notion.so", "www.notion.so", "notion-static.com"];

/// OAuth provider domains that should open in external browser.
const OAUTH_HOSTS: &[&str] = &[
    "accounts.google.com",
    "login.microsoftonline.com",
    "appleid.apple.com",
    "github.com",
];

/// Checks if a host matches a domain (exact match or subdomain).
///
/// Examples:
/// - `notion.so` matches `notion.so` ✓
/// - `www.notion.so` matches `notion.so` ✓ (subdomain)
/// - `evil-notion.so` does NOT match `notion.so` ✗
#[cfg(test)]
pub fn host_matches_domain(host: &str, domain: &str) -> bool {
    host == domain || host.ends_with(&format!(".{}", domain))
}

#[cfg(not(test))]
fn host_matches_domain(host: &str, domain: &str) -> bool {
    host == domain || host.ends_with(&format!(".{}", domain))
}

/// Checks if a URL should be opened externally.
pub fn should_open_externally(url: &str) -> bool {
    if let Ok(parsed) = Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            // Check if it's an allowed Notion domain
            let is_notion = ALLOWED_HOSTS.iter().any(|h| host_matches_domain(host, h));
            return !is_notion;
        }
    }
    true
}

/// Checks if a URL is an OAuth provider.
pub fn is_oauth_url(url: &str) -> bool {
    if let Ok(parsed) = Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return OAUTH_HOSTS.iter().any(|h| host_matches_domain(host, h));
        }
    }
    false
}

/// Handles navigation events from the webview.
/// Returns true to allow navigation, false to prevent it.
pub fn handle_navigation<R: Runtime>(
    app: &tauri::AppHandle<R>,
    url: &str,
) -> bool {
    // Check if URL should open externally
    if should_open_externally(url) {
        log::debug!("Opening URL externally: {}", url);
        
        // Open in external browser
        let _ = app.shell().open(url, None);
        
        // Prevent navigation in webview
        return false;
    }
    
    // Record page visit for history (only for Notion pages)
    if !should_open_externally(url) {
        let app_handle = app.clone();
        let url_clone = url.to_string();
        tauri::async_runtime::spawn(async move {
            // Extract title from URL or use URL as title
            let title = url_clone
                .split('/')
                .last()
                .unwrap_or("Notion Page")
                .to_string();
            
            // Record in history
            let title_clone = title.clone();
            let url_clone2 = url_clone.clone();
            if let Err(e) = history::record_page_visit(app_handle.clone(), title_clone, url_clone2).await {
                log::warn!("Failed to record page visit: {}", e);
            }
            
            // Record in analytics
            let page_id = url_clone.clone();
            if let Err(e) = analytics::record_page_view(app_handle, page_id, url_clone, title) {
                log::warn!("Failed to record page view: {}", e);
            }
        });
    }
    
    // Allow navigation within webview
    true
}
