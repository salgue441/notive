//! User settings definitions.

use serde::{Deserialize, Serialize};

/// User preferences and settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserSettings {
    // Window behavior
    pub start_minimized: bool,
    pub minimize_to_tray: bool,
    pub close_to_tray: bool,

    // Appearance
    pub zoom_level: f64,
    pub custom_css_enabled: bool,
    pub custom_css: String,

    // Notifications
    pub notifications_enabled: bool,
    pub notification_sound: bool,

    // Shortcuts
    pub shortcuts: ShortcutSettings,

    // Updates
    pub auto_update: bool,
    pub update_channel: UpdateChannel,

    // Autostart
    pub autostart_enabled: bool,

    // Performance
    pub hardware_acceleration: bool,
    pub spellcheck: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            start_minimized: false,
            minimize_to_tray: true,
            close_to_tray: true,
            zoom_level: 1.0,
            custom_css_enabled: false,
            custom_css: String::new(),
            notifications_enabled: true,
            notification_sound: true,
            shortcuts: ShortcutSettings::default(),
            auto_update: true,
            update_channel: UpdateChannel::Stable,
            autostart_enabled: false,
            hardware_acceleration: true,
            spellcheck: true,
        }
    }
}

/// Keyboard shortcut settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShortcutSettings {
    pub toggle_window: String,
    pub quick_capture: String,
    pub reload: String,
    pub zoom_in: String,
    pub zoom_out: String,
    pub zoom_reset: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            toggle_window: "CommandOrControl+Shift+N".to_string(),
            quick_capture: "CommandOrControl+Shift+C".to_string(),
            reload: "CommandOrControl+R".to_string(),
            zoom_in: "CommandOrControl+=".to_string(),
            zoom_out: "CommandOrControl+-".to_string(),
            zoom_reset: "CommandOrControl+0".to_string(),
        }
    }
}

/// Update channel selection.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UpdateChannel {
    #[default]
    Stable,
    Beta,
    Nightly,
}
