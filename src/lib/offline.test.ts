/**
 * Tests for offline mode functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Offline Mode', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should enable offline mode', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.enableOfflineMode();

    expect(invoke).toHaveBeenCalledWith('enable_offline_mode');
  });

  it('should cache page for offline', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.cachePageForOffline('https://example.com', 'Test Page', '<html>...</html>');

    expect(invoke).toHaveBeenCalledWith('cache_page_for_offline', {
      url: 'https://example.com',
      title: 'Test Page',
      content: '<html>...</html>',
    });
  });

  it('should get cached page', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockPage: ipc.OfflinePage = {
      url: 'https://example.com',
      title: 'Test Page',
      content: '<html>...</html>',
      cached_at: 1000,
      last_synced: 2000,
    };
    vi.mocked(invoke).mockResolvedValue(mockPage);

    const page = await ipc.getCachedPage('https://example.com');

    expect(page).toEqual(mockPage);
    expect(invoke).toHaveBeenCalledWith('get_cached_page', {
      url: 'https://example.com',
    });
  });

  it('should sync offline changes', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(5);

    const synced = await ipc.syncOfflineChanges();

    expect(synced).toBe(5);
    expect(invoke).toHaveBeenCalledWith('sync_offline_changes');
  });

  it('should get offline status', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockStatus: ipc.OfflineStatus = {
      is_offline: true,
      cached_pages: 5,
      last_sync: 1000,
    };
    vi.mocked(invoke).mockResolvedValue(mockStatus);

    const status = await ipc.getOfflineStatus();

    expect(status).toEqual(mockStatus);
    expect(invoke).toHaveBeenCalledWith('get_offline_status');
  });
});
