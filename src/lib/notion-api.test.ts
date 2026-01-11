/**
 * Tests for Notion API integration.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Notion API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should set API key', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.setNotionApiKey('api-key-123');

    expect(invoke).toHaveBeenCalledWith('set_notion_api_key', {
      api_key: 'api-key-123',
    });
  });

  it('should check if API key exists', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(true);

    const hasKey = await ipc.hasNotionApiKey();

    expect(hasKey).toBe(true);
    expect(invoke).toHaveBeenCalledWith('has_notion_api_key');
  });

  it('should get page metadata', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockMetadata: ipc.NotionPageMetadata = {
      id: 'page-id',
      url: 'https://www.notion.so/page-id',
      title: 'Test Page',
      created_time: '2025-01-15T10:00:00Z',
      last_edited_time: '2025-01-15T11:00:00Z',
      created_by: 'user-id',
      last_edited_by: 'user-id',
    };
    vi.mocked(invoke).mockResolvedValue(mockMetadata);

    const metadata = await ipc.getPageMetadata('page-id');

    expect(metadata).toEqual(mockMetadata);
    expect(invoke).toHaveBeenCalledWith('get_page_metadata', {
      page_id: 'page-id',
    });
  });
});
