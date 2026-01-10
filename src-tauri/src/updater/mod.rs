//! Auto-update functionality.

use tauri::{AppHandle, Runtime};
use tauri_plugin_updater::UpdaterExt;

/// Sets up the auto-updater.
pub async fn setup<R: Runtime>(app: &AppHandle<R>) {
    log::debug!("Setting up auto-updater...");

    // Delay initial check to allow app to fully load
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Check for updates
    if let Err(e) = check(app).await {
        log::warn!("Failed to check for updates: {}", e);
    }

    // TODO: Setup periodic update checks
}

/// Checks for available updates.
pub async fn check<R: Runtime>(app: &AppHandle<R>) -> Result<bool, String> {
    log::debug!("Checking for updates...");

    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            log::info!("Update available: {}", update.version);
            // TODO: Emit event to frontend
            // TODO: Show notification
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
                    |_downloaded, _total| {
                        // TODO: Emit progress events
                    },
                    || {
                        log::info!("Update ready, preparing to restart...");
                    },
                )
                .await
                .map_err(|e| e.to_string())
        }
        Ok(None) => Err("No update available".to_string()),
        Err(e) => Err(e.to_string()),
    }
}
