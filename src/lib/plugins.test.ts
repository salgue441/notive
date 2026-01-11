/**
 * Tests for plugin system.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Plugins', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should list plugins', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockPlugins: ipc.PluginMetadata[] = [
      {
        id: 'plugin-1',
        name: 'Test Plugin',
        version: '1.0.0',
        description: 'Test',
        author: 'Author',
        enabled: true,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockPlugins);

    const plugins = await ipc.listPlugins();

    expect(plugins).toEqual(mockPlugins);
    expect(invoke).toHaveBeenCalledWith('list_plugins');
  });

  it('should enable plugin', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.enablePlugin('plugin-id');

    expect(invoke).toHaveBeenCalledWith('enable_plugin', {
      plugin_id: 'plugin-id',
    });
  });

  it('should disable plugin', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.disablePlugin('plugin-id');

    expect(invoke).toHaveBeenCalledWith('disable_plugin', {
      plugin_id: 'plugin-id',
    });
  });
});
