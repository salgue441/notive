//! Auto-update functionality.

#[cfg(test)]
mod tests;

use tauri::{AppHandle, Runtime};
use tauri_plugin_updater::UpdaterExt;

/// Sets up the auto-updater with periodic checks.
pub async fn setup<R: Runtime>(app: &AppHandle<R>) {
    log::debug!("Setting up auto-updater...");

    // Delay initial check to allow app to fully load
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Check for updates initially
    if let Err(e) = check(app).await {
        log::warn!("Failed to check for updates: {}", e);
    }

    // Setup periodic update checks (every 6 hours)
    let app_handle = app.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(6 * 60 * 60));
        interval.tick().await; // Skip the first tick (immediate check already done)
        
        loop {
            interval.tick().await;
            
            // Check if auto-update is enabled
            let settings = crate::config::load(&app_handle).unwrap_or_default();
            if !settings.auto_update {
                log::debug!("Auto-update disabled, skipping periodic check");
                continue;
            }
            
            log::debug!("Running periodic update check...");
            if let Err(e) = check(&app_handle).await {
                log::warn!("Periodic update check failed: {}", e);
            }
        }
    });
}

/// Checks for available updates.
pub async fn check<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    log::debug!("Checking for updates...");

    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            log::info!("Update available: {}", update.version);
            
            // Check if notifications are enabled
            let settings = crate::config::load(app).unwrap_or_default();
            if settings.notifications_enabled && settings.auto_update {
                // Show notification
                let _ = app.notification().builder()
                    .title("Update Available")
                    .body(&format!("Notive {} is available. Click to install.", update.version))
                    .show();
            }
            
            Ok(true)
        }
        Ok(None) => {
            log::debug!("No updates available");
            Ok(false)
        }
        Err(e) => {
            log::error!("Update check failed: {}", e);
            Err(e.to_string())
        }
    }
}

/// Downloads and installs an update.
pub async fn install<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    log::info!("Installing update...");

    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            update
                .download_and_install(
                    |downloaded, total| {
                        // Emit progress events
                        let progress = if total > 0 {
                            (downloaded as f64 / total as f64 * 100.0) as u32
                        } else {
                            0
                        };
                        log::debug!("Update download progress: {}% ({}/{} bytes)", progress, downloaded, total);
                        
                        // Optionally show progress notification for large updates
                        if total > 10_000_000 && progress % 25 == 0 {
                            // Show progress every 25% for large downloads
                            let _ = app.notification().builder()
                                .title("Updating Notive")
                                .body(&format!("Downloading update: {}%", progress))
                                .show();
                        }
                    },
                    || {
                        log::info!("Update ready, preparing to restart...");
                        let _ = app.notification().builder()
                            .title("Update Ready")
                            .body("Update downloaded. The application will restart shortly.")
                            .show();
                    },
                )
                .await
                .map_err(|e| e.to_string())
        }
        Ok(None) => Err("No update available".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
