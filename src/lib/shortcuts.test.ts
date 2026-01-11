/**
 * Unit tests for shortcuts module.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { setupShortcuts } from './shortcuts';

// Mock IPC
vi.mock('./ipc', () => ({
  toggleFullscreen: vi.fn(),
  setZoom: vi.fn(),
  reloadPage: vi.fn(),
}));

describe('Shortcuts Module', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    document.body.innerHTML = '';
  });

  describe('setupShortcuts', () => {
    it('should setup keyboard event listeners', () => {
      const addEventListenerSpy = vi.spyOn(document, 'addEventListener');

      setupShortcuts();

      expect(addEventListenerSpy).toHaveBeenCalledWith(
        'keydown',
        expect.any(Function),
      );
    });
  });

  describe('F11 fullscreen toggle', () => {
    it('should toggle fullscreen on F11', async () => {
      const { toggleFullscreen } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', { key: 'F11' });
      document.dispatchEvent(event);

      expect(toggleFullscreen).toHaveBeenCalled();
    });

    it('should prevent default on F11', () => {
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: 'F11',
        cancelable: true,
      });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');
      document.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
    });
  });

  describe('Ctrl+R reload', () => {
    it('should reload on Ctrl+R', async () => {
      const { reloadPage } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: 'r',
        ctrlKey: true,
        cancelable: true,
      });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');
      document.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
      expect(reloadPage).toHaveBeenCalled();
    });

    it('should not reload on Ctrl+Shift+R', async () => {
      const { reloadPage } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: 'r',
        ctrlKey: true,
        shiftKey: true,
      });
      document.dispatchEvent(event);

      expect(reloadPage).not.toHaveBeenCalled();
    });
  });

  describe('Zoom shortcuts', () => {
    it('should zoom in on Ctrl+=', async () => {
      const { setZoom } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: '=',
        ctrlKey: true,
        cancelable: true,
      });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');
      document.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
      // Zoom should be called (exact value depends on implementation)
      expect(setZoom).toHaveBeenCalled();
    });

    it('should zoom out on Ctrl+-', async () => {
      const { setZoom } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: '-',
        ctrlKey: true,
        cancelable: true,
      });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');
      document.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
      expect(setZoom).toHaveBeenCalled();
    });

    it('should reset zoom on Ctrl+0', async () => {
      const { setZoom } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', {
        key: '0',
        ctrlKey: true,
        cancelable: true,
      });
      const preventDefaultSpy = vi.spyOn(event, 'preventDefault');
      document.dispatchEvent(event);

      expect(preventDefaultSpy).toHaveBeenCalled();
      expect(setZoom).toHaveBeenCalledWith(1.0);
    });
  });

  describe('Non-shortcut keys', () => {
    it('should not trigger shortcuts for regular keys', async () => {
      const { toggleFullscreen, setZoom, reloadPage } = await import('./ipc');
      setupShortcuts();

      const event = new KeyboardEvent('keydown', { key: 'a' });
      document.dispatchEvent(event);

      expect(toggleFullscreen).not.toHaveBeenCalled();
      expect(setZoom).not.toHaveBeenCalled();
      expect(reloadPage).not.toHaveBeenCalled();
    });
  });
});
