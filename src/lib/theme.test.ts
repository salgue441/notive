/**
 * Tests for theme functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Theme', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should include theme in settings', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockSettings: ipc.UserSettings = {
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
      theme: 'dark',
      custom_css_enabled: false,
      custom_css: '',
      notifications_enabled: true,
      notification_sound: true,
      shortcuts: {
        toggle_window: 'Ctrl+Shift+N',
        quick_capture: 'Ctrl+Shift+C',
        reload: 'Ctrl+R',
        zoom_in: 'Ctrl+=',
        zoom_out: 'Ctrl+-',
        zoom_reset: 'Ctrl+0',
      },
      auto_update: true,
      update_channel: 'stable',
      autostart_enabled: false,
      hardware_acceleration: true,
      spellcheck: true,
    };
    vi.mocked(invoke).mockResolvedValue(mockSettings);

    const settings = await ipc.getSettings();
    
    expect(settings.theme).toBe('dark');
  });

  it('should support all theme values', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    
    const themes: Array<'system' | 'light' | 'dark'> = ['system', 'light', 'dark'];
    
    for (const theme of themes) {
      const mockSettings: ipc.UserSettings = {
        start_minimized: false,
        minimize_to_tray: true,
        close_to_tray: true,
        zoom_level: 1.0,
        theme,
        custom_css_enabled: false,
        custom_css: '',
        notifications_enabled: true,
        notification_sound: true,
        shortcuts: {
          toggle_window: 'Ctrl+Shift+N',
          quick_capture: 'Ctrl+Shift+C',
          reload: 'Ctrl+R',
          zoom_in: 'Ctrl+=',
          zoom_out: 'Ctrl+-',
          zoom_reset: 'Ctrl+0',
        },
        auto_update: true,
        update_channel: 'stable',
        autostart_enabled: false,
        hardware_acceleration: true,
        spellcheck: true,
      };
      vi.mocked(invoke).mockResolvedValue(mockSettings);

      const settings = await ipc.getSettings();
      expect(settings.theme).toBe(theme);
    }
  });

  it('should update theme in settings', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    const settings: ipc.UserSettings = {
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
      theme: 'light',
      custom_css_enabled: false,
      custom_css: '',
      notifications_enabled: true,
      notification_sound: true,
      shortcuts: {
        toggle_window: 'Ctrl+Shift+N',
        quick_capture: 'Ctrl+Shift+C',
        reload: 'Ctrl+R',
        zoom_in: 'Ctrl+=',
        zoom_out: 'Ctrl+-',
        zoom_reset: 'Ctrl+0',
      },
      auto_update: true,
      update_channel: 'stable',
      autostart_enabled: false,
      hardware_acceleration: true,
      spellcheck: true,
    };

    await ipc.updateSettings(settings);
    
    expect(invoke).toHaveBeenCalledWith('update_settings', { settings });
    expect(settings.theme).toBe('light');
  });
});
