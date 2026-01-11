//! Bookmarks and favorites management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime};

/// Bookmark information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub created_at: String, // ISO 8601 format
    pub tags: Vec<String>,
}

impl Bookmark {
    pub fn new(title: String, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            url,
            icon: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            tags: Vec::new(),
        }
    }
}

/// Manages bookmarks.
pub struct BookmarkManager {
    bookmarks: HashMap<String, Bookmark>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: HashMap::new(),
        }
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.bookmarks.insert(bookmark.id.clone(), bookmark);
    }

    pub fn remove_bookmark(&mut self, id: &str) -> Option<Bookmark> {
        self.bookmarks.remove(id)
    }

    pub fn get_bookmark(&self, id: &str) -> Option<&Bookmark> {
        self.bookmarks.get(id)
    }

    pub fn list_bookmarks(&self) -> Vec<&Bookmark> {
        self.bookmarks.values().collect()
    }

    pub fn search_bookmarks(&self, query: &str) -> Vec<&Bookmark> {
        let query_lower = query.to_lowercase();
        self.bookmarks
            .values()
            .filter(|b| {
                b.title.to_lowercase().contains(&query_lower)
                    || b.url.to_lowercase().contains(&query_lower)
                    || b.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }
}

/// Adds a bookmark.
#[tauri::command]
pub async fn add_bookmark<R: Runtime>(
    app: AppHandle<R>,
    title: String,
    url: String,
) -> Result<String, String> {
    log::debug!("Adding bookmark: {} -> {}", title, url);
    
    let bookmark = Bookmark::new(title, url);
    let id = bookmark.id.clone();
    
    // Persist bookmark
    let mut bookmarks = load_bookmarks(&app).unwrap_or_default();
    bookmarks.push(bookmark.clone());
    if let Err(e) = save_bookmarks(&app, &bookmarks) {
        log::warn!("Failed to save bookmark: {}", e);
    }
    
    log::info!("Bookmark added: {} ({})", bookmark.title, id);
    
    Ok(id)
}

/// Removes a bookmark.
#[tauri::command]
pub async fn remove_bookmark<R: Runtime>(
    app: AppHandle<R>,
    bookmark_id: String,
) -> Result<(), String> {
    log::debug!("Removing bookmark: {}", bookmark_id);
    
    let mut bookmarks = load_bookmarks(&app).unwrap_or_default();
    bookmarks.retain(|b| b.id != bookmark_id);
    save_bookmarks(&app, &bookmarks).map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Lists all bookmarks.
#[tauri::command]
pub fn list_bookmarks<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Bookmark>, String> {
    load_bookmarks(&app).map_err(|e| e.to_string())
}

/// Searches bookmarks.
#[tauri::command]
pub fn search_bookmarks<R: Runtime>(
    app: AppHandle<R>,
    query: String,
) -> Result<Vec<Bookmark>, String> {
    let bookmarks = load_bookmarks(&app).unwrap_or_default();
    let query_lower = query.to_lowercase();
    let results: Vec<Bookmark> = bookmarks
        .into_iter()
        .filter(|b| {
            b.title.to_lowercase().contains(&query_lower)
                || b.url.to_lowercase().contains(&query_lower)
                || b.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
        })
        .collect();
    Ok(results)
}

/// Gets a bookmark by ID.
#[tauri::command]
pub fn get_bookmark<R: Runtime>(
    app: AppHandle<R>,
    bookmark_id: String,
) -> Result<Option<Bookmark>, String> {
    let bookmarks = load_bookmarks(&app).unwrap_or_default();
    Ok(bookmarks.into_iter().find(|b| b.id == bookmark_id))
}
