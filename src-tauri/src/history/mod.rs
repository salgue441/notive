//! Page history and recent pages management.

#[cfg(test)]
mod tests;

mod persistence;

use persistence::{load_history, save_history};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use tauri::{AppHandle, Manager, Runtime};

/// Page history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub title: String,
    pub url: String,
    pub visited_at: String, // ISO 8601 format
    pub visit_count: u32,
}

impl HistoryEntry {
    pub fn new(title: String, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            url,
            visited_at: chrono::Utc::now().to_rfc3339(),
            visit_count: 1,
        }
    }
}

/// Manages page history.
pub struct HistoryManager {
    history: VecDeque<HistoryEntry>,
    max_entries: usize,
}

impl HistoryManager {
    pub fn new(max_entries: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    pub fn add_entry(&mut self, mut entry: HistoryEntry) {
        // Check if URL already exists
        if let Some(existing) = self.history.iter_mut().find(|e| e.url == entry.url) {
            existing.visit_count += 1;
            existing.visited_at = chrono::Utc::now().to_rfc3339();
            existing.title = entry.title; // Update title in case it changed
        } else {
            // Add new entry
            if self.history.len() >= self.max_entries {
                self.history.pop_back();
            }
            self.history.push_front(entry);
        }
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&HistoryEntry> {
        self.history.iter().take(limit).collect()
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }

    pub fn remove_entry(&mut self, id: &str) -> Option<HistoryEntry> {
        if let Some(pos) = self.history.iter().position(|e| e.id == id) {
            self.history.remove(pos)
        } else {
            None
        }
    }
}

/// Records a page visit.
#[tauri::command]
pub async fn record_page_visit<R: Runtime>(
    app: AppHandle<R>,
    title: String,
    url: String,
) -> Result<(), String> {
    log::debug!("Recording page visit: {} -> {}", title, url);
    
    let mut history = load_history(&app).unwrap_or_default();
    
    // Check if URL already exists
    if let Some(existing) = history.iter_mut().find(|e| e.url == url) {
        existing.visit_count += 1;
        existing.visited_at = chrono::Utc::now().to_rfc3339();
        existing.title = title; // Update title in case it changed
    } else {
        // Add new entry
        let entry = HistoryEntry::new(title, url);
        history.insert(0, entry);
        
        // Limit history size
        if history.len() > 1000 {
            history.truncate(1000);
        }
    }
    
    // Persist history
    if let Err(e) = save_history(&app, &history) {
        log::warn!("Failed to save history: {}", e);
    }
    
    Ok(())
}

/// Gets recent pages.
#[tauri::command]
pub fn get_recent_pages<R: Runtime>(
    app: AppHandle<R>,
    limit: Option<usize>,
) -> Result<Vec<HistoryEntry>, String> {
    let limit = limit.unwrap_or(20);
    let history = load_history(&app).unwrap_or_default();
    Ok(history.into_iter().take(limit).collect())
}

/// Clears page history.
#[tauri::command]
pub async fn clear_history<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    log::debug!("Clearing page history");
    save_history(&app, &[]).map_err(|e| e.to_string())?;
    Ok(())
}

/// Removes a history entry.
#[tauri::command]
pub async fn remove_history_entry<R: Runtime>(
    app: AppHandle<R>,
    entry_id: String,
) -> Result<(), String> {
    log::debug!("Removing history entry: {}", entry_id);
    let mut history = load_history(&app).unwrap_or_default();
    history.retain(|e| e.id != entry_id);
    save_history(&app, &history).map_err(|e| e.to_string())?;
    Ok(())
}
