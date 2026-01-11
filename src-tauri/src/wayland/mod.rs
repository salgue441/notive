//! Wayland-specific optimizations and improvements.

use tauri::{AppHandle, Runtime};

/// Checks if running under Wayland.
#[tauri::command]
pub fn is_wayland<R: Runtime>(app: AppHandle<R>) -> Result<bool, String> {
    #[cfg(target_os = "linux")]
    {
        Ok(std::env::var("WAYLAND_DISPLAY").is_ok() || std::env::var("XDG_SESSION_TYPE") == Ok("wayland".to_string()))
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Ok(false)
    }
}

/// Applies Wayland-specific optimizations.
pub fn apply_wayland_optimizations<R: Runtime>(app: &tauri::AppHandle<R>) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        if is_wayland(app.clone()).unwrap_or(false) {
            log::info!("Applying Wayland optimizations...");
            
            // Set Wayland-specific environment variables for better compatibility
            std::env::set_var("GDK_BACKEND", "wayland");
            
            // Note: Additional optimizations could include:
            // - Native Wayland clipboard integration
            // - Wayland-specific window management
            // - Fractional scaling support
            // - Better input method handling
            
            log::info!("Wayland optimizations applied");
        }
    }
    
    Ok(())
}

/// Gets Wayland-specific information.
#[tauri::command]
pub fn get_wayland_info<R: Runtime>(app: AppHandle<R>) -> Result<serde_json::Value, String> {
    #[cfg(target_os = "linux")]
    {
        let mut info = serde_json::json!({
            "is_wayland": is_wayland(app.clone()).unwrap_or(false),
            "wayland_display": std::env::var("WAYLAND_DISPLAY").ok(),
            "xdg_session_type": std::env::var("XDG_SESSION_TYPE").ok(),
            "gdk_backend": std::env::var("GDK_BACKEND").ok(),
        });
        
        Ok(info)
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        Ok(serde_json::json!({
            "is_wayland": false,
            "message": "Wayland is only available on Linux"
        }))
    }
}
