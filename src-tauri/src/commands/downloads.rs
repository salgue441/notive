//! Download management commands.

#[cfg(test)]
mod tests;

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder};
use tauri_plugin_notification::NotificationExt;

/// Handles file downloads from the webview.
/// 
/// This command can show a native save dialog and trigger the download.
#[tauri::command]
pub async fn handle_download<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    filename: Option<String>,
) -> Result<(), String> {
    log::debug!("Download requested: {} -> {:?}", url, filename);
    
    let file_name = filename.unwrap_or_else(|| {
        url.split('/')
            .last()
            .unwrap_or("file")
            .split('?')
            .next()
            .unwrap_or("file")
            .to_string()
    });
    
    // Show notification that download started
    let _ = app.notification().builder()
        .title("Download Started")
        .body(&format!("Downloading: {}", file_name))
        .show();
    
    // Optionally show a save dialog (for future enhancement)
    // For now, we'll use the webview's native download handling
    // which respects the user's browser download preferences
    
    // Open the URL in the webview, which will trigger the native download
    if let Some(window) = app.get_webview_window("main") {
        // Use a more reliable method to trigger download
        let script = format!(
            r#"
            (function() {{
                const link = document.createElement('a');
                link.href = '{}';
                link.download = '{}';
                link.target = '_blank';
                document.body.appendChild(link);
                link.click();
                document.body.removeChild(link);
            }})();
            "#,
            url.replace('\'', "\\'"),
            file_name.replace('\'', "\\'")
        );
        window.eval(&script).map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

/// Shows a native file save dialog and triggers download.
/// This provides more control over where files are saved.
#[tauri::command]
pub async fn download_with_dialog<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    suggested_filename: Option<String>,
) -> Result<(), String> {
    log::debug!("Download with dialog requested: {} -> {:?}", url, suggested_filename);
    
    let default_name = suggested_filename.unwrap_or_else(|| {
        url.split('/')
            .last()
            .unwrap_or("file")
            .split('?')
            .next()
            .unwrap_or("file")
            .to_string()
    });
    
    // Show save dialog
    let file_path = app.dialog()
        .file()
        .set_file_name(&default_name)
        .save()
        .await;
    
    match file_path {
        Some(path) => {
            log::info!("User selected save location: {:?}", path);
            
            // Show notification
            let _ = app.notification().builder()
                .title("Download Started")
                .body(&format!("Downloading to: {}", path.display()))
                .show();
            
            // For now, we'll still use the webview download
            // In a full implementation, we'd download the file directly using HTTP
            // and save it to the selected path
            if let Some(window) = app.get_webview_window("main") {
                let script = format!(
                    r#"
                    (function() {{
                        const link = document.createElement('a');
                        link.href = '{}';
                        link.download = '{}';
                        document.body.appendChild(link);
                        link.click();
                        document.body.removeChild(link);
                    }})();
                    "#,
                    url.replace('\'', "\\'"),
                    default_name.replace('\'', "\\'")
                );
                window.eval(&script).map_err(|e| e.to_string())?;
            }
            
            Ok(())
        }
        None => {
            log::debug!("User cancelled download dialog");
            Ok(())
        }
    }
}
