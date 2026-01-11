/**
 * Tests for AI integration.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('AI Integration', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should perform AI search', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockResults: ipc.AISearchResult[] = [
      {
        title: 'Test Result',
        url: 'https://example.com',
        snippet: 'Test snippet',
        relevance_score: 0.95,
        ai_summary: 'AI summary',
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockResults);

    const results = await ipc.aiSearch('test query');

    expect(results).toEqual(mockResults);
    expect(invoke).toHaveBeenCalledWith('ai_search', { query: 'test query' });
  });

  it('should get AI suggestions', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockSuggestions = ['Suggestion 1', 'Suggestion 2'];
    vi.mocked(invoke).mockResolvedValue(mockSuggestions);

    const suggestions = await ipc.getAISuggestions('context');

    expect(suggestions).toEqual(mockSuggestions);
    expect(invoke).toHaveBeenCalledWith('get_ai_suggestions', { context: 'context' });
  });

  it('should get AI autocomplete', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockCompletions = ['completion1', 'completion2'];
    vi.mocked(invoke).mockResolvedValue(mockCompletions);

    const completions = await ipc.getAIAutocomplete('partial');

    expect(completions).toEqual(mockCompletions);
    expect(invoke).toHaveBeenCalledWith('get_ai_autocomplete', {
      partial_text: 'partial',
    });
  });
});
