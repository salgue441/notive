//! Global search functionality.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub source: SearchSource,
    pub relevance: f64,
}

/// Search source type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SearchSource {
    Bookmark,
    History,
    Workspace,
    Tab,
}

/// Searches across all sources (bookmarks, history, workspaces, tabs).
#[tauri::command]
pub async fn global_search<R: Runtime>(
    app: AppHandle<R>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    log::debug!("Global search: {}", query);
    
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();
    
    // Search bookmarks
    match crate::bookmarks::list_bookmarks(&app) {
        Ok(bookmarks) => {
            for bookmark in bookmarks {
                let relevance = calculate_relevance(&bookmark.title, &bookmark.url, &query_lower);
                if relevance > 0.0 {
                    results.push(SearchResult {
                        id: bookmark.id.clone(),
                        title: bookmark.title.clone(),
                        url: bookmark.url.clone(),
                        snippet: None,
                        source: SearchSource::Bookmark,
                        relevance,
                    });
                }
            }
        }
        Err(e) => log::warn!("Failed to load bookmarks for search: {}", e),
    }
    
    // Search history
    match crate::history::get_recent_pages(&app, Some(100)) {
        Ok(history) => {
            for entry in history {
                let relevance = calculate_relevance(&entry.title, &entry.url, &query_lower);
                if relevance > 0.0 {
                    results.push(SearchResult {
                        id: entry.id.clone(),
                        title: entry.title.clone(),
                        url: entry.url.clone(),
                        snippet: None,
                        source: SearchSource::History,
                        relevance,
                    });
                }
            }
        }
        Err(e) => log::warn!("Failed to load history for search: {}", e),
    }
    
    // Search workspaces
    match crate::workspaces::list_workspaces(&app) {
        Ok(workspaces) => {
            for workspace in workspaces {
                let relevance = calculate_relevance(&workspace.name, &workspace.url, &query_lower);
                if relevance > 0.0 {
                    results.push(SearchResult {
                        id: workspace.id.clone(),
                        title: workspace.name.clone(),
                        url: workspace.url.clone(),
                        snippet: None,
                        source: SearchSource::Workspace,
                        relevance,
                    });
                }
            }
        }
        Err(e) => log::warn!("Failed to load workspaces for search: {}", e),
    }
    
    // Sort by relevance (highest first)
    results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap_or(std::cmp::Ordering::Equal));
    
    // Limit results
    results.truncate(50);
    
    log::debug!("Search returned {} results", results.len());
    Ok(results)
}

/// Calculates relevance score for a search result.
fn calculate_relevance(title: &str, url: &str, query: &str) -> f64 {
    let title_lower = title.to_lowercase();
    let url_lower = url.to_lowercase();
    
    let mut score = 0.0;
    
    // Exact title match
    if title_lower == query {
        score += 100.0;
    }
    // Title starts with query
    else if title_lower.starts_with(query) {
        score += 50.0;
    }
    // Title contains query
    else if title_lower.contains(query) {
        score += 25.0;
    }
    
    // URL contains query
    if url_lower.contains(query) {
        score += 10.0;
    }
    
    // Word boundary matches (bonus)
    let query_words: Vec<&str> = query.split_whitespace().collect();
    for word in &query_words {
        if title_lower.contains(word) {
            score += 5.0;
        }
    }
    
    score
}

// Make calculate_relevance available for tests
#[cfg(test)]
pub use calculate_relevance;

/// Gets search history.
#[tauri::command]
pub fn get_search_history<R: Runtime>(app: AppHandle<R>) -> Result<Vec<String>, String> {
    // TODO: Load from persistent storage
    Ok(vec![])
}

/// Clears search history.
#[tauri::command]
pub async fn clear_search_history<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Clearing search history");
    // TODO: Clear from persistent storage
    Ok(())
}
