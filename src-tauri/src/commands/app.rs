//! Application-level commands.

#[cfg(test)]
mod tests;

use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tauri_plugin_notification::NotificationExt;

/// Shows the about dialog.
#[tauri::command]
pub async fn show_about<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let version = app.package_info().version.to_string();
    let tauri_version = app.package_info().tauri_version.to_string();
    
    let about_text = format!(
        "Notive v{}\n\n\
        A high-performance Notion desktop wrapper for Linux\n\n\
        Built with Tauri v{}\n\n\
        Copyright (c) 2025 Notive Contributors\n\
        Licensed under the MIT License",
        version, tauri_version
    );

    app.dialog()
        .message(&about_text)
        .title("About Notive")
        .kind(MessageDialogKind::Info)
        .buttons(MessageDialogButtons::Ok)
        .show()
        .await;

    Ok(())
}

/// Manually checks for updates.
#[tauri::command]
pub async fn check_updates<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    log::debug!("Manual update check requested");
    
    match crate::updater::check(&app).await {
        Ok(has_update) => {
            if has_update {
                // Show notification
                let _ = app.notification().builder()
                    .title("Update Available")
                    .body("A new version of Notive is available. Click to install.")
                    .show();
                
                // Show dialog
                let should_install = app.dialog()
                    .message("A new version of Notive is available. Would you like to install it now?")
                    .title("Update Available")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::YesNo)
                    .show()
                    .await;
                
                if should_install {
                    if let Err(e) = crate::updater::install(&app).await {
                        log::error!("Failed to install update: {}", e);
                        let _ = app.dialog()
                            .message(&format!("Failed to install update: {}", e))
                            .title("Update Error")
                            .kind(MessageDialogKind::Error)
                            .buttons(MessageDialogButtons::Ok)
                            .show()
                            .await;
                        return Err(e);
                    }
                }
            } else {
                // Show no update available
                let _ = app.dialog()
                    .message("You are running the latest version of Notive.")
                    .title("No Updates")
                    .kind(MessageDialogKind::Info)
                    .buttons(MessageDialogButtons::Ok)
                    .show()
                    .await;
            }
            Ok(has_update)
        }
        Err(e) => {
            log::error!("Update check failed: {}", e);
            let _ = app.dialog()
                .message(&format!("Failed to check for updates: {}", e))
                .title("Update Error")
                .kind(MessageDialogKind::Error)
                .buttons(MessageDialogButtons::Ok)
                .show()
                .await;
            Err(e)
        }
    }
}
