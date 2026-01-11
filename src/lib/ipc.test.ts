/**
 * Unit tests for IPC module.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn(),
}));

describe('IPC Module', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('setupIPC', () => {
    it('should setup IPC without errors', async () => {
      await expect(ipc.setupIPC()).resolves.not.toThrow();
    });
  });

  describe('minimizeToTray', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.minimizeToTray();

      expect(invoke).toHaveBeenCalledWith('minimize_to_tray');
    });
  });

  describe('restoreFromTray', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.restoreFromTray();

      expect(invoke).toHaveBeenCalledWith('restore_from_tray');
    });
  });

  describe('toggleFullscreen', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.toggleFullscreen();

      expect(invoke).toHaveBeenCalledWith('toggle_fullscreen');
    });
  });

  describe('setZoom', () => {
    it('should call invoke with correct parameters', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.setZoom(1.5);

      expect(invoke).toHaveBeenCalledWith('set_zoom', { level: 1.5 });
    });
  });

  describe('reloadPage', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.reloadPage();

      expect(invoke).toHaveBeenCalledWith('reload_page');
    });
  });

  describe('getSettings', () => {
    it('should return settings from backend', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const mockSettings: ipc.UserSettings = {
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
      vi.mocked(invoke).mockResolvedValue(mockSettings);

      const result = await ipc.getSettings();

      expect(invoke).toHaveBeenCalledWith('get_settings');
      expect(result).toEqual(mockSettings);
    });
  });

  describe('updateSettings', () => {
    it('should call invoke with settings', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      const settings: ipc.UserSettings = {
        start_minimized: true,
        minimize_to_tray: true,
        close_to_tray: true,
        zoom_level: 1.2,
        custom_css_enabled: false,
        custom_css: '',
        notifications_enabled: true,
        notification_sound: false,
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
        autostart_enabled: true,
        hardware_acceleration: true,
        spellcheck: true,
      };
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.updateSettings(settings);

      expect(invoke).toHaveBeenCalledWith('update_settings', { settings });
    });
  });

  describe('handleDownload', () => {
    it('should call invoke with url and optional filename', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.handleDownload('https://example.com/file.pdf', 'file.pdf');

      expect(invoke).toHaveBeenCalledWith('handle_download', {
        url: 'https://example.com/file.pdf',
        filename: 'file.pdf',
      });
    });

    it('should call invoke without filename', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.handleDownload('https://example.com/file.pdf');

      expect(invoke).toHaveBeenCalledWith('handle_download', {
        url: 'https://example.com/file.pdf',
        filename: undefined,
      });
    });
  });

  describe('showAbout', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.showAbout();

      expect(invoke).toHaveBeenCalledWith('show_about');
    });
  });

  describe('checkUpdates', () => {
    it('should return update status', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(true);

      const result = await ipc.checkUpdates();

      expect(invoke).toHaveBeenCalledWith('check_updates');
      expect(result).toBe(true);
    });
  });

  describe('openSettingsWindow', () => {
    it('should call invoke with correct command', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      await ipc.openSettingsWindow();

      expect(invoke).toHaveBeenCalledWith('open_settings_window');
    });
  });
});
