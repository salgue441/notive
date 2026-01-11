//! Tab management for multiple Notion pages.

mod persistence;

use persistence::{load_tabs_for_window, save_tabs_for_window};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime};

/// Tab configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tab {
    pub id: String,
    pub title: String,
    pub url: String,
    pub window_label: String,
}

impl Tab {
    pub fn new(title: String, url: String, window_label: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            url,
            window_label,
        }
    }
}

/// Manages tabs for a window.
pub struct TabManager {
    tabs: HashMap<String, Vec<Tab>>, // window_label -> tabs
    active_tabs: HashMap<String, String>, // window_label -> active_tab_id
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tabs: HashMap::new(),
        }
    }

    pub fn add_tab(&mut self, window_label: &str, tab: Tab) {
        self.tabs
            .entry(window_label.to_string())
            .or_insert_with(Vec::new)
            .push(tab.clone());
        self.active_tabs
            .insert(window_label.to_string(), tab.id.clone());
    }

    pub fn remove_tab(&mut self, window_label: &str, tab_id: &str) -> Option<Tab> {
        if let Some(tabs) = self.tabs.get_mut(window_label) {
            if let Some(pos) = tabs.iter().position(|t| t.id == tab_id) {
                let tab = tabs.remove(pos);
                // Update active tab if needed
                if self.active_tabs.get(window_label) == Some(&tab_id.to_string()) {
                    if let Some(new_active) = tabs.first() {
                        self.active_tabs
                            .insert(window_label.to_string(), new_active.id.clone());
                    } else {
                        self.active_tabs.remove(window_label);
                    }
                }
                return Some(tab);
            }
        }
        None
    }

    pub fn get_tabs(&self, window_label: &str) -> Vec<&Tab> {
        self.tabs
            .get(window_label)
            .map(|tabs| tabs.iter().collect())
            .unwrap_or_default()
    }

    pub fn set_active(&mut self, window_label: &str, tab_id: &str) {
        self.active_tabs.insert(window_label.to_string(), tab_id.to_string());
    }

    pub fn get_active(&self, window_label: &str) -> Option<&Tab> {
        self.active_tabs
            .get(window_label)
            .and_then(|active_id| {
                self.tabs
                    .get(window_label)
                    .and_then(|tabs| tabs.iter().find(|t| t.id == *active_id))
            })
    }
}

/// Opens a new tab in the specified window.
#[tauri::command]
pub async fn open_tab<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    url: String,
    title: Option<String>,
) -> Result<String, String> {
    log::debug!("Opening tab in {}: {} -> {:?}", window_label, url, title);
    
    let tab_title = title.unwrap_or_else(|| {
        url.split('/')
            .last()
            .unwrap_or("New Tab")
            .to_string()
    });
    
    let tab = Tab::new(tab_title.clone(), url.clone(), window_label.clone());
    
    // Navigate the window to the new URL
    if let Some(window) = app.get_webview_window(&window_label) {
        window
            .eval(&format!("window.location.href = '{}';", url.replace('\'', "\\'")))
            .map_err(|e| e.to_string())?;
        
        // Update window title
        window
            .set_title(&format!("Notive - {}", tab_title))
            .map_err(|e| e.to_string())?;
    } else {
        return Err(format!("Window {} not found", window_label));
    }
    
    log::info!("Tab opened: {} ({})", tab_title, tab.id);
    Ok(tab.id)
}

/// Closes a tab.
#[tauri::command]
pub async fn close_tab<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    tab_id: String,
) -> Result<(), String> {
    log::debug!("Closing tab {} in {}", tab_id, window_label);
    
    // For now, we'll just navigate back to the main Notion page
    // In a full implementation, we'd manage multiple tabs with iframes or multiple windows
    if let Some(window) = app.get_webview_window(&window_label) {
        window
            .eval("window.location.href = 'https://www.notion.so';")
            .map_err(|e| e.to_string())?;
        window
            .set_title("Notive")
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// Switches to a different tab.
#[tauri::command]
pub async fn switch_tab<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    tab_id: String,
) -> Result<(), String> {
    log::debug!("Switching to tab {} in {}", tab_id, window_label);
    
    // In a full implementation, we'd navigate to the tab's URL
    // For now, this is a placeholder
    Ok(())
}

/// Lists all tabs for a window.
#[tauri::command]
pub fn list_tabs<R: Runtime>(app: AppHandle<R>, window_label: String) -> Result<Vec<Tab>, String> {
    load_tabs_for_window(&app, &window_label).map_err(|e| e.to_string())
}

/// Restores tabs for a window on startup.
#[tauri::command]
pub async fn restore_tabs<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
) -> Result<Vec<Tab>, String> {
    log::debug!("Restoring tabs for window: {}", window_label);
    load_tabs_for_window(&app, &window_label).map_err(|e| e.to_string())
}
