/**
 * Unit tests for notifications module.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { setupNotificationBridge } from './notifications';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Notifications Module', () => {
  let originalNotification: typeof Notification;

  beforeEach(() => {
    vi.clearAllMocks();
    // Store original Notification
    originalNotification = window.Notification;
    // Clear any existing Notification
    delete (window as any).Notification;
  });

  afterEach(() => {
    // Restore original Notification
    if (originalNotification) {
      (window as any).Notification = originalNotification;
    }
  });

  describe('setupNotificationBridge', () => {
    it('should replace window.Notification', () => {
      setupNotificationBridge();

      expect(window.Notification).toBeDefined();
      expect(window.Notification).not.toBe(originalNotification);
    });

    it('should set permission to granted', () => {
      setupNotificationBridge();

      expect(window.Notification.permission).toBe('granted');
    });

    it('should allow creating notifications', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      const notification = new window.Notification('Test Title', {
        body: 'Test body',
      });

      expect(notification).toBeDefined();
      expect(notification.title).toBe('Test Title');
      expect(notification.body).toBe('Test body');
    });

    it('should forward notification to native backend', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      new window.Notification('Test Title', {
        body: 'Test body',
        icon: 'https://example.com/icon.png',
      });

      // Wait for async invoke
      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(invoke).toHaveBeenCalledWith('show_notification', {
        payload: {
          title: 'Test Title',
          body: 'Test body',
          icon: 'https://example.com/icon.png',
        },
      });
    });

    it('should handle notification without options', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      new window.Notification('Simple Title');

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(invoke).toHaveBeenCalledWith('show_notification', {
        payload: {
          title: 'Simple Title',
          body: null,
          icon: null,
        },
      });
    });

    it('should handle requestPermission', async () => {
      setupNotificationBridge();

      const permission = await window.Notification.requestPermission();

      expect(permission).toBe('granted');
    });

    it('should allow closing notifications', () => {
      setupNotificationBridge();

      const notification = new window.Notification('Test');
      
      // Should not throw
      expect(() => notification.close()).not.toThrow();
    });

    it('should handle notification with all options', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      const notification = new window.Notification('Full Notification', {
        body: 'Full body',
        icon: 'https://example.com/icon.png',
        tag: 'test-tag',
        data: { key: 'value' },
        dir: 'ltr',
        lang: 'en',
        badge: 'https://example.com/badge.png',
        image: 'https://example.com/image.png',
        renotify: true,
        requireInteraction: true,
        silent: false,
        timestamp: Date.now(),
        vibrate: [200, 100, 200],
      });

      expect(notification.title).toBe('Full Notification');
      expect(notification.body).toBe('Full body');
      expect(notification.icon).toBe('https://example.com/icon.png');
      expect(notification.tag).toBe('test-tag');
      expect(notification.data).toEqual({ key: 'value' });

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(invoke).toHaveBeenCalledWith('show_notification', {
        payload: {
          title: 'Full Notification',
          body: 'Full body',
          icon: 'https://example.com/icon.png',
        },
      });
    });

    it('should handle notification event handlers', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      const notification = new window.Notification('Test', {
        body: 'Test body',
      });

      let onshowCalled = false;
      let oncloseCalled = false;

      notification.onshow = () => {
        onshowCalled = true;
      };

      notification.onclose = () => {
        oncloseCalled = true;
      };

      await new Promise((resolve) => setTimeout(resolve, 10));

      // onshow should be called after invoke succeeds
      expect(onshowCalled).toBe(true);

      // Test close
      notification.close();
      expect(oncloseCalled).toBe(true);
    });

    it('should handle notification with error in onshow handler', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockResolvedValue(undefined);

      setupNotificationBridge();

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      const notification = new window.Notification('Test', {
        body: 'Test body',
      });

      notification.onshow = () => {
        throw new Error('Test error');
      };

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(consoleErrorSpy).toHaveBeenCalled();

      consoleErrorSpy.mockRestore();
    });

    it('should handle notification with error in invoke', async () => {
      const { invoke } = await import('@tauri-apps/api/core');
      vi.mocked(invoke).mockRejectedValue(new Error('Invoke failed'));

      setupNotificationBridge();

      const notification = new window.Notification('Test', {
        body: 'Test body',
      });

      let onerrorCalled = false;
      notification.onerror = () => {
        onerrorCalled = true;
      };

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(onerrorCalled).toBe(true);
    });
  });
});
