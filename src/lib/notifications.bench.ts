/**
 * Performance benchmarks for notifications module.
 */

import { bench, describe } from 'vitest';
import { setupNotificationBridge } from './notifications';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn().mockResolvedValue(undefined),
}));

describe('Notifications Performance Benchmarks', () => {
  beforeEach(() => {
    delete (window as any).Notification;
    setupNotificationBridge();
  });

  bench('create notification', () => {
    new window.Notification('Test', { body: 'Body' });
  });

  bench('create notification with all options', () => {
    new window.Notification('Test', {
      body: 'Body',
      icon: 'https://example.com/icon.png',
      tag: 'test-tag',
      data: { key: 'value' },
    });
  });

  bench('request permission', async () => {
    await window.Notification.requestPermission();
  });

  bench('create 100 notifications', () => {
    for (let i = 0; i < 100; i++) {
      new window.Notification(`Notification ${i}`, { body: `Body ${i}` });
    }
  });
});
