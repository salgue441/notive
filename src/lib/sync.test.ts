/**
 * Tests for settings sync functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Settings Sync', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should export settings', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('Settings exported to: /path/to/file.json');

    const message = await ipc.exportSettings();
    
    expect(message).toBe('Settings exported to: /path/to/file.json');
    expect(invoke).toHaveBeenCalledWith('export_settings');
  });

  it('should import settings', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('Settings imported successfully.');

    const message = await ipc.importSettings();
    
    expect(message).toBe('Settings imported successfully.');
    expect(invoke).toHaveBeenCalledWith('import_settings');
  });

  it('should get settings as JSON', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockJson = '{"zoom_level":1.0,"theme":"dark"}';
    vi.mocked(invoke).mockResolvedValue(mockJson);

    const json = await ipc.getSettingsJson();
    
    expect(json).toBe(mockJson);
    expect(invoke).toHaveBeenCalledWith('get_settings_json');
  });

  it('should restore settings from JSON', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    const json = '{"zoom_level":1.5,"theme":"dark"}';
    await ipc.restoreSettingsJson(json);
    
    expect(invoke).toHaveBeenCalledWith('restore_settings_json', { json });
  });

  it('should handle export cancellation', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Export cancelled'));

    await expect(ipc.exportSettings()).rejects.toThrow();
  });

  it('should handle import errors', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Invalid settings file'));

    await expect(ipc.importSettings()).rejects.toThrow();
  });

  it('should handle invalid JSON when restoring', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Invalid JSON'));

    await expect(ipc.restoreSettingsJson('invalid json')).rejects.toThrow();
  });
});
