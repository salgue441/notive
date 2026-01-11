/**
 * Performance benchmarks for IPC module.
 */

import { bench, describe, vi } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));

describe('IPC Performance Benchmarks', () => {
  bench('getSettings', async () => {
    await ipc.getSettings();
  });

  bench('updateSettings', async () => {
    const settings: ipc.UserSettings = {
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
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
  });

  bench('setZoom', async () => {
    await ipc.setZoom(1.5);
  });
});
