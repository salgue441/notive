//! Notion API integration for direct API access.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Notion API client configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionApiConfig {
    pub api_key: Option<String>,
    pub base_url: String,
    pub rate_limit_per_minute: u32,
}

impl Default for NotionApiConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            base_url: "https://api.notion.com/v1".to_string(),
            rate_limit_per_minute: 3, // Notion API rate limit
        }
    }
}

/// Notion page metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotionPageMetadata {
    pub id: String,
    pub url: String,
    pub title: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub created_by: Option<String>,
    pub last_edited_by: Option<String>,
}

/// Sets the Notion API key.
#[tauri::command]
pub fn set_notion_api_key<R: Runtime>(
    _app: AppHandle<R>,
    api_key: String,
) -> Result<(), String> {
    // Store API key securely
    crate::security::token_storage::store_token(
        "notive",
        "notion_api_key",
        crate::security::token_storage::StoredToken {
            token: api_key,
            expires_at: None,
            refresh_token: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        },
    )
    .map_err(|e| e.to_string())?;
    
    log::info!("Notion API key stored securely");
    Ok(())
}

/// Gets page metadata from Notion API.
#[tauri::command]
pub async fn get_page_metadata<R: Runtime>(
    app: AppHandle<R>,
    page_id: String,
) -> Result<NotionPageMetadata, String> {
    // Get API key
    let api_key = crate::security::token_storage::get_token("notive", "notion_api_key")
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Notion API key not set".to_string())?;
    
    // Make API request
    let client = reqwest::Client::new();
    let url = format!("https://api.notion.com/v1/pages/{}", page_id);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", api_key.token))
        .header("Notion-Version", "2022-06-28")
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    let json: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;
    
    // Extract metadata
    Ok(NotionPageMetadata {
        id: json["id"].as_str().unwrap_or("").to_string(),
        url: json["url"].as_str().unwrap_or("").to_string(),
        title: json["properties"]
            .as_object()
            .and_then(|props| {
                props.values().next()
            })
            .and_then(|prop| prop["title"].as_array())
            .and_then(|title_array| title_array.first())
            .and_then(|title| title["plain_text"].as_str())
            .unwrap_or("Untitled")
            .to_string(),
        created_time: json["created_time"].as_str().unwrap_or("").to_string(),
        last_edited_time: json["last_edited_time"].as_str().unwrap_or("").to_string(),
        created_by: json["created_by"]["id"].as_str().map(|s| s.to_string()),
        last_edited_by: json["last_edited_by"]["id"].as_str().map(|s| s.to_string()),
    })
}

/// Checks if API key is set.
#[tauri::command]
pub fn has_notion_api_key<R: Runtime>(_app: AppHandle<R>) -> Result<bool, String> {
    match crate::security::token_storage::get_token("notive", "notion_api_key") {
        Ok(Some(_)) => Ok(true),
        Ok(None) => Ok(false),
        Err(e) => Err(e),
    }
}
