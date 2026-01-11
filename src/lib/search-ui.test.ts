/**
 * Tests for global search UI component.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { GlobalSearch, getGlobalSearch } from './search';
import * as ipc from './ipc';

// Mock IPC functions
vi.mock('./ipc', () => ({
  globalSearch: vi.fn(),
}));

describe('GlobalSearch', () => {
  let search: GlobalSearch;

  beforeEach(() => {
    document.body.innerHTML = '';
    search = new GlobalSearch();
    vi.clearAllMocks();
  });

  afterEach(() => {
    document.body.innerHTML = '';
  });

  it('should create search overlay on init', () => {
    search.init();

    const overlay = document.getElementById('global-search-overlay');
    expect(overlay).toBeTruthy();
    expect(overlay?.classList.contains('search-overlay')).toBe(true);
  });

  it('should show search overlay', () => {
    search.init();
    search.show();

    const overlay = document.getElementById('global-search-overlay');
    expect(overlay?.style.display).toBe('flex');
  });

  it('should hide search overlay', () => {
    search.init();
    search.show();
    search.hide();

    const overlay = document.getElementById('global-search-overlay');
    expect(overlay?.style.display).toBe('none');
  });

  it('should toggle search overlay', () => {
    search.init();

    expect(search['isVisible']).toBe(false);
    search.toggle();
    expect(search['isVisible']).toBe(true);
    search.toggle();
    expect(search['isVisible']).toBe(false);
  });

  it('should perform search on input', async () => {
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: 'Test Result',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 50.0,
      },
    ];
    vi.mocked(ipc.globalSearch).mockResolvedValue(mockResults);

    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 'test';
    input.dispatchEvent(new Event('input'));

    // Wait for async search
    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(ipc.globalSearch).toHaveBeenCalledWith('test');
  });

  it('should not search with less than 2 characters', async () => {
    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 't';
    input.dispatchEvent(new Event('input'));

    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(ipc.globalSearch).not.toHaveBeenCalled();
  });

  it('should close on Escape key', () => {
    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    const escapeEvent = new KeyboardEvent('keydown', { key: 'Escape' });
    input.dispatchEvent(escapeEvent);

    expect(search['isVisible']).toBe(false);
  });

  it('should navigate to first result on Enter', async () => {
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: 'Test Result',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 50.0,
      },
    ];
    vi.mocked(ipc.globalSearch).mockResolvedValue(mockResults);

    // Mock window.location
    const originalLocation = window.location;
    delete (window as any).location;
    window.location = { ...originalLocation, href: '' };

    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 'test';
    input.dispatchEvent(new Event('input'));

    await new Promise((resolve) => setTimeout(resolve, 100));

    const enterEvent = new KeyboardEvent('keydown', { key: 'Enter' });
    input.dispatchEvent(enterEvent);

    // Restore location
    window.location = originalLocation;
  });

  it('should render search results', async () => {
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: 'Test Result',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 50.0,
      },
    ];
    vi.mocked(ipc.globalSearch).mockResolvedValue(mockResults);

    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 'test';
    input.dispatchEvent(new Event('input'));

    await new Promise((resolve) => setTimeout(resolve, 100));

    const results = document.querySelectorAll('.search-result');
    expect(results.length).toBe(1);
  });

  it('should show empty message when no results', async () => {
    vi.mocked(ipc.globalSearch).mockResolvedValue([]);

    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 'nonexistent';
    input.dispatchEvent(new Event('input'));

    await new Promise((resolve) => setTimeout(resolve, 100));

    const empty = document.querySelector('.search-empty');
    expect(empty).toBeTruthy();
  });

  it('should escape HTML in search results', async () => {
    const mockResults: ipc.SearchResult[] = [
      {
        id: '1',
        title: '<script>alert("xss")</script>',
        url: 'https://example.com',
        source: 'Bookmark',
        relevance: 50.0,
      },
    ];
    vi.mocked(ipc.globalSearch).mockResolvedValue(mockResults);

    search.init();
    search.show();

    const input = document.getElementById('global-search-input') as HTMLInputElement;
    input.value = 'test';
    input.dispatchEvent(new Event('input'));

    await new Promise((resolve) => setTimeout(resolve, 100));

    const resultTitle = document.querySelector('.search-result-title');
    expect(resultTitle?.textContent).not.toContain('<script>');
  });

  it('should get singleton instance', () => {
    const instance1 = getGlobalSearch();
    const instance2 = getGlobalSearch();

    expect(instance1).toBe(instance2);
  });
});
