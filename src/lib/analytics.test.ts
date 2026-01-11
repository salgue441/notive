/**
 * Tests for analytics functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Analytics', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should record page view', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.recordPageView('page-id', 'https://example.com', 'Test Page');

    expect(invoke).toHaveBeenCalledWith('record_page_view', {
      page_id: 'page-id',
      page_url: 'https://example.com',
      page_title: 'Test Page',
    });
  });

  it('should record page edit', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.recordPageEdit('page-id', 'https://example.com', 'modified');

    expect(invoke).toHaveBeenCalledWith('record_page_edit', {
      page_id: 'page-id',
      page_url: 'https://example.com',
      edit_type: 'modified',
    });
  });

  it('should get usage stats', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockStats: ipc.UsageStats = {
      total_views: 100,
      total_edits: 50,
      total_pages: 10,
      most_viewed_pages: [['https://example.com', 20]],
      recent_activity: [],
    };
    vi.mocked(invoke).mockResolvedValue(mockStats);

    const stats = await ipc.getUsageStats();

    expect(stats).toEqual(mockStats);
    expect(invoke).toHaveBeenCalledWith('get_usage_stats');
  });

  it('should get activity timeline', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockTimeline: ipc.ActivityEntry[] = [];
    vi.mocked(invoke).mockResolvedValue(mockTimeline);

    const timeline = await ipc.getActivityTimeline(50);

    expect(timeline).toEqual(mockTimeline);
    expect(invoke).toHaveBeenCalledWith('get_activity_timeline', { limit: 50 });
  });

  it('should clear analytics', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.clearAnalytics();

    expect(invoke).toHaveBeenCalledWith('clear_analytics');
  });
});
