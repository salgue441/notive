/**
 * Tests for history management.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('History', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should record a page visit', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.recordPageVisit('Test Page', 'https://www.notion.so/test');
    
    expect(invoke).toHaveBeenCalledWith('record_page_visit', {
      title: 'Test Page',
      url: 'https://www.notion.so/test',
    });
  });

  it('should get recent pages', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockHistory: ipc.HistoryEntry[] = [
      {
        id: '1',
        title: 'Test Page',
        url: 'https://example.com',
        visited_at: '2025-01-15T10:00:00Z',
        visit_count: 1,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockHistory);

    const history = await ipc.getRecentPages();
    
    expect(history).toEqual(mockHistory);
    expect(invoke).toHaveBeenCalledWith('get_recent_pages', { limit: undefined });
  });

  it('should get recent pages with limit', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockHistory: ipc.HistoryEntry[] = [];
    vi.mocked(invoke).mockResolvedValue(mockHistory);

    await ipc.getRecentPages(10);
    
    expect(invoke).toHaveBeenCalledWith('get_recent_pages', { limit: 10 });
  });

  it('should clear history', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.clearHistory();
    
    expect(invoke).toHaveBeenCalledWith('clear_history');
  });

  it('should remove a history entry', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.removeHistoryEntry('entry-id-123');
    
    expect(invoke).toHaveBeenCalledWith('remove_history_entry', {
      entryId: 'entry-id-123',
    });
  });

  it('should handle empty history', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue([]);

    const history = await ipc.getRecentPages();
    
    expect(history).toEqual([]);
  });

  it('should handle errors when recording visit', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Failed to record'));

    await expect(ipc.recordPageVisit('Test', 'https://example.com')).rejects.toThrow();
  });
});
