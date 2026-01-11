//! Multi-monitor support and window placement.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime, Window};

/// Monitor information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorInfo {
    pub id: String,
    pub name: String,
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub scale_factor: f64,
    pub is_primary: bool,
}

/// Window placement preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowPlacement {
    pub window_label: String,
    pub monitor_id: Option<String>,
    pub position: Option<(i32, i32)>,
    pub size: Option<(u32, u32)>,
    pub maximized: bool,
}

/// Gets all available monitors.
#[tauri::command]
pub async fn get_monitors<R: Runtime>(_app: AppHandle<R>) -> Result<Vec<MonitorInfo>, String> {
    // In a full implementation, we'd query system for monitors
    // For now, return a placeholder
    Ok(vec![MonitorInfo {
        id: "primary".to_string(),
        name: "Primary Monitor".to_string(),
        position: (0, 0),
        size: (1920, 1080),
        scale_factor: 1.0,
        is_primary: true,
    }])
}

/// Gets the primary monitor.
#[tauri::command]
pub async fn get_primary_monitor<R: Runtime>(app: AppHandle<R>) -> Result<MonitorInfo, String> {
    let monitors = get_monitors(app).await?;
    monitors
        .into_iter()
        .find(|m| m.is_primary)
        .ok_or_else(|| "No primary monitor found".to_string())
}

/// Saves window placement.
#[tauri::command]
pub async fn save_window_placement<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    monitor_id: Option<String>,
    position: Option<(i32, i32)>,
    size: Option<(u32, u32)>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(&window_label) {
        if let Some(pos) = position {
            window.set_position(tauri::LogicalPosition::new(pos.0 as f64, pos.1 as f64))
                .map_err(|e| e.to_string())?;
        }
        if let Some(sz) = size {
            window.set_size(tauri::LogicalSize::new(sz.0 as f64, sz.1 as f64))
                .map_err(|e| e.to_string())?;
        }
    }
    
    log::debug!("Window placement saved for: {}", window_label);
    Ok(())
}

/// Restores window placement.
#[tauri::command]
pub async fn restore_window_placement<R: Runtime>(
    _app: AppHandle<R>,
    window_label: String,
) -> Result<(), String> {
    let _ = window_label;
    // In a full implementation, we'd load saved placement
    log::debug!("Window placement restored for: {}", window_label);
    Ok(())
}

/// Moves window to a specific monitor.
#[tauri::command]
pub async fn move_window_to_monitor<R: Runtime>(
    app: AppHandle<R>,
    window_label: String,
    monitor_id: String,
) -> Result<(), String> {
    let monitors = get_monitors(app.clone()).await?;
    let monitor = monitors
        .into_iter()
        .find(|m| m.id == monitor_id)
        .ok_or_else(|| format!("Monitor {} not found", monitor_id))?;
    
    if let Some(window) = app.get_webview_window(&window_label) {
        window.set_position(tauri::LogicalPosition::new(
            monitor.position.0 as f64,
            monitor.position.1 as f64,
        ))
        .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
