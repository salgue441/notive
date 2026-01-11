/**
 * Tests for tab bar component.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { TabBar } from './tabs';
import * as ipc from './ipc';

// Mock IPC functions
vi.mock('./ipc', () => ({
  listTabs: vi.fn(),
  openTab: vi.fn(),
  closeTab: vi.fn(),
  switchTab: vi.fn(),
}));

describe('TabBar', () => {
  let container: HTMLElement;

  beforeEach(() => {
    container = document.createElement('div');
    container.id = 'test-tab-container';
    document.body.appendChild(container);
    vi.clearAllMocks();
  });

  afterEach(() => {
    container.remove();
  });

  it('should initialize tab bar', async () => {
    vi.mocked(ipc.listTabs).mockResolvedValue([]);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    expect(container.querySelector('.tab-bar')).toBeTruthy();
    expect(container.querySelector('.tab-list')).toBeTruthy();
    expect(container.querySelector('.tab-new')).toBeTruthy();
  });

  it('should load and render tabs', async () => {
    const mockTabs: ipc.Tab[] = [
      {
        id: 'tab-1',
        title: 'Test Tab',
        url: 'https://example.com',
        window_label: 'main',
      },
    ];
    vi.mocked(ipc.listTabs).mockResolvedValue(mockTabs);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    const tabList = container.querySelector('.tab-list');
    expect(tabList?.children.length).toBe(1);
  });

  it('should open a new tab', async () => {
    vi.mocked(ipc.listTabs).mockResolvedValue([]);
    vi.mocked(ipc.openTab).mockResolvedValue('new-tab-id');
    vi.mocked(ipc.switchTab).mockResolvedValue(undefined);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    await tabBar.openTab('https://example.com', 'New Tab');

    expect(ipc.openTab).toHaveBeenCalledWith('main', 'https://example.com', 'New Tab');
    expect(ipc.switchTab).toHaveBeenCalledWith('main', 'new-tab-id');
  });

  it('should close a tab', async () => {
    const mockTabs: ipc.Tab[] = [
      {
        id: 'tab-1',
        title: 'Test Tab',
        url: 'https://example.com',
        window_label: 'main',
      },
    ];
    vi.mocked(ipc.listTabs).mockResolvedValue(mockTabs);
    vi.mocked(ipc.closeTab).mockResolvedValue(undefined);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    await tabBar.closeTab('tab-1');

    expect(ipc.closeTab).toHaveBeenCalledWith('main', 'tab-1');
  });

  it('should switch to a tab', async () => {
    const mockTabs: ipc.Tab[] = [
      {
        id: 'tab-1',
        title: 'Test Tab',
        url: 'https://example.com',
        window_label: 'main',
      },
    ];
    vi.mocked(ipc.listTabs).mockResolvedValue(mockTabs);
    vi.mocked(ipc.switchTab).mockResolvedValue(undefined);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    await tabBar.switchToTab('tab-1');

    expect(ipc.switchTab).toHaveBeenCalledWith('main', 'tab-1');
  });

  it('should handle errors when loading tabs', async () => {
    vi.mocked(ipc.listTabs).mockRejectedValue(new Error('Failed to load'));

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    // Should not throw, just log error
    expect(container.querySelector('.tab-bar')).toBeTruthy();
  });

  it('should escape HTML in tab titles', async () => {
    const mockTabs: ipc.Tab[] = [
      {
        id: 'tab-1',
        title: '<script>alert("xss")</script>',
        url: 'https://example.com',
        window_label: 'main',
      },
    ];
    vi.mocked(ipc.listTabs).mockResolvedValue(mockTabs);

    const tabBar = new TabBar('main');
    await tabBar.init('test-tab-container');

    const tabTitle = container.querySelector('.tab-title');
    expect(tabTitle?.textContent).not.toContain('<script>');
  });
});
