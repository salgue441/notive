/**
 * Tests for bookmark management.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Bookmarks', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should add a bookmark', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('bookmark-id-123');

    const id = await ipc.addBookmark('Test Page', 'https://www.notion.so/test');
    
    expect(id).toBe('bookmark-id-123');
    expect(invoke).toHaveBeenCalledWith('add_bookmark', {
      title: 'Test Page',
      url: 'https://www.notion.so/test',
    });
  });

  it('should remove a bookmark', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.removeBookmark('bookmark-id-123');
    
    expect(invoke).toHaveBeenCalledWith('remove_bookmark', {
      bookmarkId: 'bookmark-id-123',
    });
  });

  it('should list bookmarks', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockBookmarks: ipc.Bookmark[] = [
      {
        id: '1',
        title: 'Test Page',
        url: 'https://example.com',
        created_at: '2025-01-15T10:00:00Z',
        tags: [],
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockBookmarks);

    const bookmarks = await ipc.listBookmarks();
    
    expect(bookmarks).toEqual(mockBookmarks);
    expect(invoke).toHaveBeenCalledWith('list_bookmarks');
  });

  it('should search bookmarks', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockResults: ipc.Bookmark[] = [
      {
        id: '1',
        title: 'Test Page',
        url: 'https://example.com',
        created_at: '2025-01-15T10:00:00Z',
        tags: ['test'],
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockResults);

    const results = await ipc.searchBookmarks('test');
    
    expect(results).toEqual(mockResults);
    expect(invoke).toHaveBeenCalledWith('search_bookmarks', { query: 'test' });
  });

  it('should get a bookmark by ID', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockBookmark: ipc.Bookmark = {
      id: '1',
      title: 'Test Page',
      url: 'https://example.com',
      created_at: '2025-01-15T10:00:00Z',
      tags: [],
    };
    vi.mocked(invoke).mockResolvedValue(mockBookmark);

    const bookmark = await ipc.getBookmark('1');
    
    expect(bookmark).toEqual(mockBookmark);
    expect(invoke).toHaveBeenCalledWith('get_bookmark', { bookmarkId: '1' });
  });

  it('should handle empty bookmark list', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue([]);

    const bookmarks = await ipc.listBookmarks();
    
    expect(bookmarks).toEqual([]);
  });

  it('should handle search with no results', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue([]);

    const results = await ipc.searchBookmarks('nonexistent');
    
    expect(results).toEqual([]);
  });

  it('should handle errors when adding bookmark', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Failed to add bookmark'));

    await expect(ipc.addBookmark('Test', 'https://example.com')).rejects.toThrow();
  });
});
