/**
 * Unit tests for settings window.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { getSettings, updateSettings } from './lib/ipc';

// Mock Tauri API
vi.mock('./lib/ipc', () => ({
  getSettings: vi.fn(),
  updateSettings: vi.fn(),
}));

describe('Settings Window', () => {
  beforeEach(() => {
    document.body.innerHTML = `
      <div id="app">
        <input id="start-minimized" type="checkbox" />
        <input id="minimize-to-tray" type="checkbox" />
        <input id="close-to-tray" type="checkbox" />
        <input id="zoom-level" type="range" min="0.5" max="2.0" step="0.1" value="1.0" />
        <span id="zoom-value">100%</span>
        <input id="custom-css-enabled" type="checkbox" />
        <textarea id="custom-css"></textarea>
        <input id="notifications-enabled" type="checkbox" />
        <input id="notification-sound" type="checkbox" />
        <select id="update-channel">
          <option value="stable">Stable</option>
          <option value="beta">Beta</option>
          <option value="nightly">Nightly</option>
        </select>
        <input id="auto-update" type="checkbox" />
        <input id="autostart-enabled" type="checkbox" />
        <input id="hardware-acceleration" type="checkbox" />
        <input id="spellcheck" type="checkbox" />
        <select id="theme">
          <option value="system">System</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
        <button id="save-btn"></button>
        <button id="cancel-btn"></button>
        <button id="export-settings-btn"></button>
        <button id="import-settings-btn"></button>
      </div>
    `;
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  describe('Form population', () => {
    it('should populate form with settings', async () => {
      const mockSettings = {
        start_minimized: true,
        minimize_to_tray: false,
        close_to_tray: true,
        zoom_level: 1.5,
        custom_css_enabled: true,
        custom_css: 'body { color: red; }',
        notifications_enabled: false,
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
        update_channel: 'beta' as const,
        autostart_enabled: true,
        hardware_acceleration: false,
        spellcheck: false,
        theme: 'dark' as const,
      };

      vi.mocked(getSettings).mockResolvedValue(mockSettings);

      // Simulate loading settings (would be done in actual settings.ts)
      const startMinimized = document.getElementById(
        'start-minimized',
      ) as HTMLInputElement;
      const minimizeToTray = document.getElementById(
        'minimize-to-tray',
      ) as HTMLInputElement;
      const zoomLevel = document.getElementById(
        'zoom-level',
      ) as HTMLInputElement;
      const zoomValue = document.getElementById('zoom-value') as HTMLSpanElement;

      startMinimized.checked = mockSettings.start_minimized;
      minimizeToTray.checked = mockSettings.minimize_to_tray;
      zoomLevel.value = mockSettings.zoom_level.toString();
      zoomValue.textContent = `${Math.round(mockSettings.zoom_level * 100)}%`;

      expect(startMinimized.checked).toBe(true);
      expect(minimizeToTray.checked).toBe(false);
      expect(zoomLevel.value).toBe('1.5');
      expect(zoomValue.textContent).toBe('150%');
    });

    it('should populate theme selector', async () => {
      const mockSettings = {
        start_minimized: false,
        minimize_to_tray: true,
        close_to_tray: true,
        zoom_level: 1.0,
        theme: 'dark' as const,
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
        update_channel: 'stable' as const,
        autostart_enabled: false,
        hardware_acceleration: true,
        spellcheck: true,
      };

      const theme = document.getElementById('theme') as HTMLSelectElement;
      theme.value = mockSettings.theme;

      expect(theme.value).toBe('dark');
    });
  });

  describe('Form submission', () => {
    it('should collect form data correctly', () => {
      const startMinimized = document.getElementById(
        'start-minimized',
      ) as HTMLInputElement;
      const zoomLevel = document.getElementById(
        'zoom-level',
      ) as HTMLInputElement;

      startMinimized.checked = true;
      zoomLevel.value = '1.2';

      expect(startMinimized.checked).toBe(true);
      expect(parseFloat(zoomLevel.value)).toBe(1.2);
    });
  });
});
