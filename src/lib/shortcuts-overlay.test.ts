/**
 * Tests for shortcuts overlay component.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { getShortcutsOverlay, ShortcutsOverlay } from './shortcuts-overlay';
import * as ipc from './ipc';

// Mock IPC
vi.mock('./ipc', () => ({
  getSettings: vi.fn(),
}));

describe('ShortcutsOverlay', () => {
  let overlay: ShortcutsOverlay;

  beforeEach(() => {
    document.body.innerHTML = '';
    overlay = new ShortcutsOverlay();
    vi.clearAllMocks();
  });

  afterEach(() => {
    document.body.innerHTML = '';
  });

  it('should create overlay on init', () => {
    overlay.init();

    const overlayElement = document.getElementById('shortcuts-overlay');
    expect(overlayElement).toBeTruthy();
  });

  it('should show overlay', () => {
    overlay.init();
    overlay.show();

    const overlayElement = document.getElementById('shortcuts-overlay');
    expect(overlayElement?.style.display).toBe('flex');
  });

  it('should hide overlay', () => {
    overlay.init();
    overlay.show();
    overlay.hide();

    const overlayElement = document.getElementById('shortcuts-overlay');
    expect(overlayElement?.style.display).toBe('none');
  });

  it('should toggle overlay', () => {
    overlay.init();

    expect(overlay['isVisible']).toBe(false);
    overlay.toggle();
    expect(overlay['isVisible']).toBe(true);
    overlay.toggle();
    expect(overlay['isVisible']).toBe(false);
  });

  it('should render shortcuts', async () => {
    vi.mocked(ipc.getSettings).mockResolvedValue({
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
      theme: 'system',
      custom_css_enabled: false,
      custom_css: '',
      notifications_enabled: true,
      notification_sound: true,
      shortcuts: {
        toggle_window: 'Ctrl+Shift+N',
        quick_capture: 'Ctrl+Shift+C',
        reload: 'Ctrl+R',
        zoom_in: 'Ctrl+=',
        zoom_out: 'Ctrl+-',
        zoom_reset: 'Ctrl+0',
      },
      auto_update: true,
      update_channel: 'stable',
      autostart_enabled: false,
      hardware_acceleration: true,
      spellcheck: true,
    });

    overlay.init();
    await overlay['loadShortcuts']();
    overlay.render();
    overlay.show();

    const modal = document.querySelector('.shortcuts-modal');
    expect(modal).toBeTruthy();
  });

  it('should close on Escape key', () => {
    overlay.init();
    overlay.show();

    const escapeEvent = new KeyboardEvent('keydown', { key: 'Escape' });
    document.dispatchEvent(escapeEvent);

    expect(overlay['isVisible']).toBe(false);
  });

  it('should get singleton instance', () => {
    const instance1 = getShortcutsOverlay();
    const instance2 = getShortcutsOverlay();

    expect(instance1).toBe(instance2);
  });
});
