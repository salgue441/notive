/**
 * IPC communication with Rust backend.
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Sets up IPC event listeners and handlers.
 */
export async function setupIPC(): Promise<void> {
  console.log('[IPC] Setting up...');

  // TODO: Setup event listeners for backend events
  // - Update notifications
  // - Download progress
  // - Settings changes
}

/**
 * Minimizes the window to the system tray.
 */
export async function minimizeToTray(): Promise<void> {
  await invoke('minimize_to_tray');
}

/**
 * Restores the window from the system tray.
 */
export async function restoreFromTray(): Promise<void> {
  await invoke('restore_from_tray');
}

/**
 * Toggles fullscreen mode.
 */
export async function toggleFullscreen(): Promise<void> {
  await invoke('toggle_fullscreen');
}

/**
 * Sets the page zoom level.
 */
export async function setZoom(level: number): Promise<void> {
  await invoke('set_zoom', { level });
}

/**
 * Reloads the current page.
 */
export async function reloadPage(): Promise<void> {
  await invoke('reload_page');
}

/**
 * Gets the current user settings.
 */
export async function getSettings(): Promise<UserSettings> {
  return await invoke('get_settings');
}

/**
 * Updates user settings.
 */
export async function updateSettings(settings: UserSettings): Promise<void> {
  await invoke('update_settings', { settings });
}

/**
 * User settings interface.
 */
export interface UserSettings {
  start_minimized: boolean;
  minimize_to_tray: boolean;
  close_to_tray: boolean;
  zoom_level: number;
  custom_css_enabled: boolean;
  custom_css: string;
  notifications_enabled: boolean;
  notification_sound: boolean;
  shortcuts: ShortcutSettings;
  auto_update: boolean;
  update_channel: 'stable' | 'beta' | 'nightly';
  autostart_enabled: boolean;
  hardware_acceleration: boolean;
  spellcheck: boolean;
}

/**
 * Shortcut settings interface.
 */
export interface ShortcutSettings {
  toggle_window: string;
  quick_capture: string;
  reload: string;
  zoom_in: string;
  zoom_out: string;
  zoom_reset: string;
}
