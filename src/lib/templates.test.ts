/**
 * Tests for page templates.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Templates', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should list templates', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockTemplates: ipc.PageTemplate[] = [
      {
        id: '1',
        name: 'Test Template',
        url: 'https://example.com',
        description: 'Test',
        category: 'work',
        tags: [],
        icon: undefined,
        created_at: '2025-01-15T10:00:00Z',
        usage_count: 5,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockTemplates);

    const templates = await ipc.listTemplates();

    expect(templates).toEqual(mockTemplates);
    expect(invoke).toHaveBeenCalledWith('list_templates');
  });

  it('should create template', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('template-id-123');

    const id = await ipc.createTemplate('Test', 'https://example.com', 'Description', 'work');

    expect(id).toBe('template-id-123');
    expect(invoke).toHaveBeenCalledWith('create_template', {
      name: 'Test',
      url: 'https://example.com',
      description: 'Description',
      category: 'work',
    });
  });

  it('should use template', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('https://example.com');

    const url = await ipc.useTemplate('template-id');

    expect(url).toBe('https://example.com');
    expect(invoke).toHaveBeenCalledWith('use_template', {
      template_id: 'template-id',
    });
  });

  it('should search templates', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockResults: ipc.PageTemplate[] = [];
    vi.mocked(invoke).mockResolvedValue(mockResults);

    const results = await ipc.searchTemplates('test');

    expect(results).toEqual(mockResults);
    expect(invoke).toHaveBeenCalledWith('search_templates', { query: 'test' });
  });
});
