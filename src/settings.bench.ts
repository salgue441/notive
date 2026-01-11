/**
 * Performance benchmarks for settings window.
 */

import { bench, describe, beforeEach, vi } from 'vitest';
import { getSettings, updateSettings } from './lib/ipc';

// Mock Tauri API
vi.mock('./lib/ipc', () => ({
  getSettings: vi.fn().mockResolvedValue({
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
  }),
  updateSettings: vi.fn().mockResolvedValue(undefined),
}));

describe('Settings Window Performance Benchmarks', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <input id="start-minimized" type="checkbox" />
      <input id="zoom-level" type="range" min="0.5" max="2.0" step="0.1" value="1.0" />
      <span id="zoom-value">100%</span>
      <textarea id="custom-css"></textarea>
    `;
  });

  bench('populate form', async () => {
    const settings = await getSettings();
    (document.getElementById('start-minimized') as HTMLInputElement).checked =
      settings.start_minimized;
    (document.getElementById('zoom-level') as HTMLInputElement).value =
      settings.zoom_level.toString();
  });

  bench('collect form data', () => {
    const startMinimized = (
      document.getElementById('start-minimized') as HTMLInputElement
    ).checked;
    const zoomLevel = parseFloat(
      (document.getElementById('zoom-level') as HTMLInputElement).value,
    );
    const customCss = (document.getElementById('custom-css') as HTMLTextAreaElement)
      .value;
    return { startMinimized, zoomLevel, customCss };
  });

  bench('update settings 10 times', async () => {
    const settings = await getSettings();
    for (let i = 0; i < 10; i++) {
      await updateSettings(settings);
    }
  });
});
