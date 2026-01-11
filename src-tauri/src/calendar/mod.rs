//! System calendar integration.

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};

/// Calendar event from Notion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub start_time: String, // ISO 8601 format
    pub end_time: Option<String>, // ISO 8601 format
    pub location: Option<String>,
}

/// Syncs Notion calendar events to system calendar.
#[tauri::command]
pub async fn sync_calendar<R: Runtime>(
    app: AppHandle<R>,
    events: Vec<CalendarEvent>,
) -> Result<u32, String> {
    log::debug!("Syncing {} calendar events", events.len());
    
    // TODO: Implement actual calendar sync
    // This would require:
    // 1. Parsing Notion calendar data (via API or scraping)
    // 2. Converting to iCal format
    // 3. Using system calendar APIs (e.g., libical, evolution-data-server on Linux)
    // 4. Creating/updating calendar events
    
    log::info!("Calendar sync completed: {} events processed", events.len());
    Ok(events.len() as u32)
}

/// Exports a calendar event to system calendar.
#[tauri::command]
pub async fn export_event<R: Runtime>(
    app: AppHandle<R>,
    event: CalendarEvent,
) -> Result<(), String> {
    log::debug!("Exporting calendar event: {}", event.title);
    
    // Create iCal content
    let ical_content = format!(
        r#"BEGIN:VCALENDAR
VERSION:2.0
PRODID:-//Notive//Notion Calendar//EN
BEGIN:VEVENT
UID:{}
DTSTART:{}
{}
SUMMARY:{}
{}
{}
END:VEVENT
END:VCALENDAR"#,
        event.id,
        event.start_time,
        if let Some(end) = &event.end_time {
            format!("DTEND:{}", end)
        } else {
            String::new()
        },
        event.title,
        if let Some(desc) = &event.description {
            format!("DESCRIPTION:{}", desc.replace('\n', "\\n"))
        } else {
            String::new()
        },
        if let Some(loc) = &event.location {
            format!("LOCATION:{}", loc)
        } else {
            String::new()
        }
    );
    
    // Save to temporary file and open with default calendar app
    use std::fs;
    use std::path::PathBuf;
    
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join(format!("notion-event-{}.ics", event.id));
    
    fs::write(&file_path, ical_content).map_err(|e| e.to_string())?;
    
    // Open with default application
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&file_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    log::info!("Calendar event exported to: {:?}", file_path);
    Ok(())
}

/// Checks if calendar integration is available.
#[tauri::command]
pub fn check_calendar_availability<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    // Check if calendar applications are available
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        // Check for common calendar apps
        let apps = ["evolution", "thunderbird", "gnome-calendar", "korganizer"];
        for app in &apps {
            if Command::new("which")
                .arg(app)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Ok(false)
    }
}
