//! Page templates library for quick page creation.

#[cfg(test)]
mod tests;

mod persistence;

use persistence::{load_templates, save_templates};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

/// Page template definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageTemplate {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub icon: Option<String>,
    pub created_at: String,
    pub usage_count: u64,
}

impl PageTemplate {
    pub fn new(
        name: String,
        url: String,
        description: Option<String>,
        category: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            url,
            category,
            tags: vec![],
            icon: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            usage_count: 0,
        }
    }

    pub fn increment_usage(&mut self) {
        self.usage_count += 1;
    }
}

/// Lists all available templates.
#[tauri::command]
pub fn list_templates<R: Runtime>(app: AppHandle<R>) -> Result<Vec<PageTemplate>, String> {
    load_templates(&app).map_err(|e| e.to_string())
}

/// Gets a template by ID.
#[tauri::command]
pub fn get_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
) -> Result<Option<PageTemplate>, String> {
    let templates = load_templates(&app).map_err(|e| e.to_string())?;
    Ok(templates.into_iter().find(|t| t.id == template_id))
}

/// Creates a new template.
#[tauri::command]
pub fn create_template<R: Runtime>(
    app: AppHandle<R>,
    name: String,
    url: String,
    description: Option<String>,
    category: Option<String>,
) -> Result<String, String> {
    let mut template = PageTemplate::new(name, url, description, category);
    let template_id = template.id.clone();
    
    let mut templates = load_templates(&app).unwrap_or_default();
    templates.push(template);
    save_templates(&app, &templates).map_err(|e| e.to_string())?;
    
    Ok(template_id)
}

/// Updates a template.
#[tauri::command]
pub fn update_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
    name: Option<String>,
    url: Option<String>,
    description: Option<String>,
    category: Option<String>,
) -> Result<(), String> {
    let mut templates = load_templates(&app).map_err(|e| e.to_string())?;
    
    if let Some(template) = templates.iter_mut().find(|t| t.id == template_id) {
        if let Some(name) = name {
            template.name = name;
        }
        if let Some(url) = url {
            template.url = url;
        }
        if let Some(description) = description {
            template.description = Some(description);
        }
        if let Some(category) = category {
            template.category = Some(category);
        }
        
        save_templates(&app, &templates).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Template {} not found", template_id))
    }
}

/// Deletes a template.
#[tauri::command]
pub fn delete_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
) -> Result<(), String> {
    let mut templates = load_templates(&app).map_err(|e| e.to_string())?;
    templates.retain(|t| t.id != template_id);
    save_templates(&app, &templates).map_err(|e| e.to_string())?;
    Ok(())
}

/// Uses a template (increments usage count and returns URL).
#[tauri::command]
pub fn use_template<R: Runtime>(
    app: AppHandle<R>,
    template_id: String,
) -> Result<String, String> {
    let mut templates = load_templates(&app).map_err(|e| e.to_string())?;
    
    if let Some(template) = templates.iter_mut().find(|t| t.id == template_id) {
        template.increment_usage();
        let url = template.url.clone();
        save_templates(&app, &templates).map_err(|e| e.to_string())?;
        Ok(url)
    } else {
        Err(format!("Template {} not found", template_id))
    }
}

/// Searches templates by name, description, or category.
#[tauri::command]
pub fn search_templates<R: Runtime>(
    app: AppHandle<R>,
    query: String,
) -> Result<Vec<PageTemplate>, String> {
    let templates = load_templates(&app).map_err(|e| e.to_string())?;
    let query_lower = query.to_lowercase();
    
    let results: Vec<PageTemplate> = templates
        .into_iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&query_lower)
                || t.description
                    .as_ref()
                    .map(|d| d.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
                || t.category
                    .as_ref()
                    .map(|c| c.to_lowercase().contains(&query_lower))
                    .unwrap_or(false)
                || t.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
        })
        .collect();
    
    Ok(results)
}
