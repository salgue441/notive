//! Page analytics and usage tracking.

#[cfg(test)]
mod tests;

mod persistence;

use persistence::{load_analytics, save_analytics, AnalyticsData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime};

/// Page view analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageView {
    pub page_id: String,
    pub page_url: String,
    pub page_title: String,
    pub viewed_at: u64,
    pub duration: Option<u64>, // seconds
}

/// Page edit history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageEdit {
    pub page_id: String,
    pub page_url: String,
    pub edited_at: u64,
    pub edit_type: String, // "created", "modified", "deleted"
}

/// Activity timeline entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub id: String,
    pub page_id: String,
    pub page_url: String,
    pub page_title: String,
    pub activity_type: String, // "view", "edit", "bookmark", etc.
    pub timestamp: u64,
    pub metadata: Option<serde_json::Value>,
}

/// Usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub total_views: u64,
    pub total_edits: u64,
    pub total_pages: u64,
    pub most_viewed_pages: Vec<(String, u64)>, // (page_url, view_count)
    pub recent_activity: Vec<ActivityEntry>,
}

/// Records a page view.
#[tauri::command]
pub fn record_page_view<R: Runtime>(
    app: AppHandle<R>,
    page_id: String,
    page_url: String,
    page_title: String,
) -> Result<(), String> {
    let view = PageView {
        page_id,
        page_url,
        page_title,
        viewed_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        duration: None,
    };
    
    let mut analytics = load_analytics(&app).unwrap_or_default();
    analytics.views.push(view);
    
    // Keep only last 1000 views
    if analytics.views.len() > 1000 {
        analytics.views.drain(0..analytics.views.len() - 1000);
    }
    
    save_analytics(&app, &analytics).map_err(|e| e.to_string())?;
    Ok(())
}

/// Records a page edit.
#[tauri::command]
pub fn record_page_edit<R: Runtime>(
    app: AppHandle<R>,
    page_id: String,
    page_url: String,
    edit_type: String,
) -> Result<(), String> {
    let edit = PageEdit {
        page_id,
        page_url,
        edited_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        edit_type,
    };
    
    let mut analytics = load_analytics(&app).unwrap_or_default();
    analytics.edits.push(edit);
    
    // Keep only last 500 edits
    if analytics.edits.len() > 500 {
        analytics.edits.drain(0..analytics.edits.len() - 500);
    }
    
    save_analytics(&app, &analytics).map_err(|e| e.to_string())?;
    Ok(())
}

/// Gets usage statistics.
#[tauri::command]
pub fn get_usage_stats<R: Runtime>(app: AppHandle<R>) -> Result<UsageStats, String> {
    let analytics = load_analytics(&app).unwrap_or_default();
    
    // Count views per page
    let mut view_counts: HashMap<String, (String, u64)> = HashMap::new();
    for view in &analytics.views {
        let entry = view_counts.entry(view.page_url.clone()).or_insert_with(|| {
            (view.page_title.clone(), 0)
        });
        entry.1 += 1;
    }
    
    let mut most_viewed: Vec<(String, u64)> = view_counts
        .into_iter()
        .map(|(url, (_, count))| (url, count))
        .collect();
    most_viewed.sort_by(|a, b| b.1.cmp(&a.1));
    most_viewed.truncate(10);
    
    // Get unique pages count
    let unique_pages: HashSet<String> = analytics
        .views
        .iter()
        .map(|v| v.page_url.clone())
        .collect();
    
    // Get recent activity
    let mut recent_activity: Vec<ActivityEntry> = vec![];
    
    // Add recent views
    for view in analytics.views.iter().rev().take(20) {
        recent_activity.push(ActivityEntry {
            id: uuid::Uuid::new_v4().to_string(),
            page_id: view.page_id.clone(),
            page_url: view.page_url.clone(),
            page_title: view.page_title.clone(),
            activity_type: "view".to_string(),
            timestamp: view.viewed_at,
            metadata: None,
        });
    }
    
    // Add recent edits
    for edit in analytics.edits.iter().rev().take(20) {
        recent_activity.push(ActivityEntry {
            id: uuid::Uuid::new_v4().to_string(),
            page_id: edit.page_id.clone(),
            page_url: edit.page_url.clone(),
            page_title: String::new(),
            activity_type: format!("edit:{}", edit.edit_type),
            timestamp: edit.edited_at,
            metadata: None,
        });
    }
    
    recent_activity.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    recent_activity.truncate(50);
    
    // Get unique pages count
    let unique_pages: std::collections::HashSet<String> = analytics
        .views
        .iter()
        .map(|v| v.page_url.clone())
        .collect();
    
    Ok(UsageStats {
        total_views: analytics.views.len() as u64,
        total_edits: analytics.edits.len() as u64,
        total_pages: unique_pages.len() as u64,
        most_viewed_pages: most_viewed,
        recent_activity,
    })
}

/// Gets activity timeline.
#[tauri::command]
pub fn get_activity_timeline<R: Runtime>(
    app: AppHandle<R>,
    limit: Option<usize>,
) -> Result<Vec<ActivityEntry>, String> {
    let analytics = load_analytics(&app).unwrap_or_default();
    let limit = limit.unwrap_or(100);
    
    let mut timeline: Vec<ActivityEntry> = vec![];
    
    // Add views
    for view in analytics.views.iter().rev().take(limit) {
        timeline.push(ActivityEntry {
            id: uuid::Uuid::new_v4().to_string(),
            page_id: view.page_id.clone(),
            page_url: view.page_url.clone(),
            page_title: view.page_title.clone(),
            activity_type: "view".to_string(),
            timestamp: view.viewed_at,
            metadata: None,
        });
    }
    
    // Add edits
    for edit in analytics.edits.iter().rev().take(limit) {
        timeline.push(ActivityEntry {
            id: uuid::Uuid::new_v4().to_string(),
            page_id: edit.page_id.clone(),
            page_url: edit.page_url.clone(),
            page_title: String::new(),
            activity_type: format!("edit:{}", edit.edit_type),
            timestamp: edit.edited_at,
            metadata: None,
        });
    }
    
    timeline.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    timeline.truncate(limit);
    
    Ok(timeline)
}

/// Clears analytics data.
#[tauri::command]
pub fn clear_analytics<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let analytics = persistence::AnalyticsData::default();
    persistence::save_analytics(&app, &analytics).map_err(|e| e.to_string())?;
    Ok(())
}
