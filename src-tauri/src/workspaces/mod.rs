//! Workspace management for multiple Notion instances.

#[cfg(test)]
mod tests;

mod persistence;

use persistence::{load_workspaces, save_workspaces};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime, WindowBuilder, WindowUrl};

/// Workspace configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub url: String,
    pub zoom_level: f64,
}

impl Default for Workspace {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default Workspace".to_string(),
            url: "https://www.notion.so".to_string(),
            zoom_level: 1.0,
        }
    }
}

/// Manages multiple workspaces.
pub struct WorkspaceManager {
    workspaces: HashMap<String, Workspace>,
    active_workspace: Option<String>,
}

impl WorkspaceManager {
    pub fn new() -> Self {
        Self {
            workspaces: HashMap::new(),
            active_workspace: None,
        }
    }

    pub fn add_workspace(&mut self, workspace: Workspace) {
        self.workspaces.insert(workspace.id.clone(), workspace);
    }

    pub fn remove_workspace(&mut self, id: &str) -> Option<Workspace> {
        self.workspaces.remove(id)
    }

    pub fn get_workspace(&self, id: &str) -> Option<&Workspace> {
        self.workspaces.get(id)
    }

    pub fn list_workspaces(&self) -> Vec<&Workspace> {
        self.workspaces.values().collect()
    }

    pub fn set_active(&mut self, id: Option<String>) {
        self.active_workspace = id;
    }

    pub fn get_active(&self) -> Option<&Workspace> {
        self.active_workspace
            .as_ref()
            .and_then(|id| self.workspaces.get(id))
    }
}

/// Creates a new workspace window.
#[tauri::command]
pub async fn create_workspace<R: Runtime>(
    app: AppHandle<R>,
    name: String,
    url: Option<String>,
) -> Result<String, String> {
    log::debug!("Creating workspace: {} -> {:?}", name, url);
    
    let workspace = Workspace {
        id: uuid::Uuid::new_v4().to_string(),
        name: name.clone(),
        url: url.unwrap_or_else(|| "https://www.notion.so".to_string()),
        zoom_level: 1.0,
    };
    
    // Create window for workspace
    let window_label = format!("workspace-{}", workspace.id);
    let workspace_url = workspace.url.clone();
    let window = WindowBuilder::new(
        &app,
        &window_label,
        WindowUrl::External(
            workspace_url
                .parse::<url::Url>()
                .map_err(|e| format!("Invalid URL: {}", e))?,
        ),
    )
    .title(&format!("Notive - {}", name))
    .inner_size(1200.0, 800.0)
    .min_inner_size(800.0, 600.0)
    .resizable(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;
    
    // Persist workspace
    let mut workspaces = load_workspaces(&app).unwrap_or_default();
    workspaces.push(workspace.clone());
    if let Err(e) = save_workspaces(&app, &workspaces) {
        log::warn!("Failed to save workspace: {}", e);
    }
    
    log::info!("Workspace created: {} ({})", name, workspace.id);
    
    Ok(workspace.id)
}

/// Lists all available workspaces.
#[tauri::command]
pub fn list_workspaces<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Workspace>, String> {
    load_workspaces(&app).map_err(|e| e.to_string())
}

/// Switches to a different workspace.
#[tauri::command]
pub async fn switch_workspace<R: Runtime>(
    app: AppHandle<R>,
    workspace_id: String,
) -> Result<(), String> {
    log::debug!("Switching to workspace: {}", workspace_id);
    
    // Find workspace window
    let window_label = format!("workspace-{}", workspace_id);
    if let Some(window) = app.get_webview_window(&window_label) {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Workspace {} not found", workspace_id))
    }
}

/// Closes a workspace window.
#[tauri::command]
pub async fn close_workspace<R: Runtime>(
    app: AppHandle<R>,
    workspace_id: String,
) -> Result<(), String> {
    log::debug!("Closing workspace: {}", workspace_id);
    
    let window_label = format!("workspace-{}", workspace_id);
    if let Some(window) = app.get_webview_window(&window_label) {
        window.close().map_err(|e| e.to_string())?;
        
        // Remove from persistent storage
        let mut workspaces = load_workspaces(&app).unwrap_or_default();
        workspaces.retain(|w| w.id != workspace_id);
        if let Err(e) = save_workspaces(&app, &workspaces) {
            log::warn!("Failed to update workspaces: {}", e);
        }
        
        Ok(())
    } else {
        Err(format!("Workspace {} not found", workspace_id))
    }
}
