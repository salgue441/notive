//! Navigation and URL handlers.

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

/// Checks if a URL should be opened externally.
pub fn should_open_externally(url: &str) -> bool {
    if let Ok(parsed) = Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            // Check if it's an allowed Notion domain
            let is_notion = ALLOWED_HOSTS.iter().any(|h| host.ends_with(h));
            return !is_notion;
        }
    }
    true
}

/// Checks if a URL is an OAuth provider.
pub fn is_oauth_url(url: &str) -> bool {
    if let Ok(parsed) = Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return OAUTH_HOSTS.iter().any(|h| host.contains(h));
        }
    }
    false
}
