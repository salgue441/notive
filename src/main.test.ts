/**
 * Unit tests for main initialization.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock all dependencies
vi.mock('./lib/ipc', () => ({
  setupIPC: vi.fn().mockResolvedValue(undefined),
}));

vi.mock('./lib/notifications', () => ({
  setupNotificationBridge: vi.fn(),
}));

vi.mock('./lib/shortcuts', () => ({
  setupShortcuts: vi.fn(),
}));

vi.mock('./lib/offline', () => ({
  setupOfflineIndicator: vi.fn(),
}));

describe('Main Initialization', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    // Reset DOM
    document.body.innerHTML = '';
    Object.defineProperty(document, 'readyState', {
      writable: true,
      value: 'loading',
      configurable: true,
    });
  });

  describe('DOMContentLoaded initialization', () => {
    it('should initialize when DOM is loading', async () => {
      Object.defineProperty(document, 'readyState', {
        writable: true,
        value: 'loading',
        configurable: true,
      });

      // Import after setting up mocks
      await import('./main');

      // Simulate DOMContentLoaded
      const event = new Event('DOMContentLoaded');
      document.dispatchEvent(event);

      // Wait for async operations
      await new Promise((resolve) => setTimeout(resolve, 10));

      const { setupIPC } = await import('./lib/ipc');
      const { setupNotificationBridge } = await import('./lib/notifications');
      const { setupShortcuts } = await import('./lib/shortcuts');
      const { setupOfflineIndicator } = await import('./lib/offline');

      expect(setupIPC).toHaveBeenCalled();
      expect(setupNotificationBridge).toHaveBeenCalled();
      expect(setupShortcuts).toHaveBeenCalled();
      expect(setupOfflineIndicator).toHaveBeenCalled();
    });

    it('should initialize immediately if DOM is already loaded', async () => {
      Object.defineProperty(document, 'readyState', {
        writable: true,
        value: 'complete',
        configurable: true,
      });

      await import('./main');

      // Wait for async operations
      await new Promise((resolve) => setTimeout(resolve, 10));

      const { setupIPC } = await import('./lib/ipc');
      expect(setupIPC).toHaveBeenCalled();
    });
  });

  describe('Error handling', () => {
    it('should handle initialization errors gracefully', async () => {
      const { setupIPC } = await import('./lib/ipc');
      vi.mocked(setupIPC).mockRejectedValue(new Error('Test error'));

      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});

      Object.defineProperty(document, 'readyState', {
        writable: true,
        value: 'complete',
        configurable: true,
      });

      await import('./main');

      await new Promise((resolve) => setTimeout(resolve, 10));

      expect(consoleErrorSpy).toHaveBeenCalledWith(
        expect.stringContaining('Initialization failed'),
        expect.any(Error),
      );

      consoleErrorSpy.mockRestore();
    });
  });
});
