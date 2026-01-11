/**
 * Tests for global search functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Global Search', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should perform global search', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: 'Test Page',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 50.0,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockResults);

    const results = await ipc.globalSearch('test');
    
    expect(results).toEqual(mockResults);
    expect(invoke).toHaveBeenCalledWith('global_search', { query: 'test' });
  });

  it('should get search history', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockHistory = ['test', 'notion', 'workspace'];
    vi.mocked(invoke).mockResolvedValue(mockHistory);

    const history = await ipc.getSearchHistory();
    
    expect(history).toEqual(mockHistory);
    expect(invoke).toHaveBeenCalledWith('get_search_history');
  });

  it('should clear search history', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.clearSearchHistory();
    
    expect(invoke).toHaveBeenCalledWith('clear_search_history');
  });

  it('should handle empty search results', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue([]);

    const results = await ipc.globalSearch('nonexistent');
    
    expect(results).toEqual([]);
  });

  it('should handle search with multiple sources', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: 'Bookmark',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 100.0,
      },
      {
        id: '2',
        title: 'History Entry',
        url: 'https://example.com/history',
        source: 'History',
        relevance: 50.0,
      },
      {
        id: '3',
        title: 'Workspace',
        url: 'https://example.com/workspace',
        source: 'Workspace',
        relevance: 25.0,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockResults);

    const results = await ipc.globalSearch('example');
    
    expect(results).toHaveLength(3);
    expect(results[0].relevance).toBeGreaterThanOrEqual(results[1].relevance);
  });

  it('should handle search errors', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Search failed'));

    await expect(ipc.globalSearch('test')).rejects.toThrow();
  });
});
