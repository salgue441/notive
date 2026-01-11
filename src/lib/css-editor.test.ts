/**
 * Tests for CSS editor component.
 */

import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { getCSSEditor, CSSEditor } from './css-editor';
import * as ipc from './ipc';

// Mock IPC
vi.mock('./ipc', () => ({
  getSettings: vi.fn(),
  updateSettings: vi.fn(),
}));

describe('CSSEditor', () => {
  let editor: CSSEditor;

  beforeEach(() => {
    document.body.innerHTML = '<div id="css-editor-container"></div>';
    editor = new CSSEditor('css-editor-container');
    vi.clearAllMocks();
  });

  afterEach(() => {
    document.body.innerHTML = '';
  });

  it('should create editor on init', async () => {
    vi.mocked(ipc.getSettings).mockResolvedValue({
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
      theme: 'system',
      custom_css_enabled: false,
      custom_css: 'body { color: red; }',
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

    await editor.init();

    const modal = document.getElementById('css-editor-modal');
    expect(modal).toBeTruthy();
  });

  it('should load CSS from settings', async () => {
    const testCSS = 'body { color: blue; }';
    vi.mocked(ipc.getSettings).mockResolvedValue({
      start_minimized: false,
      minimize_to_tray: true,
      close_to_tray: true,
      zoom_level: 1.0,
      theme: 'system',
      custom_css_enabled: true,
      custom_css: testCSS,
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

    await editor.init();

    const textarea = document.getElementById('css-editor-textarea') as HTMLTextAreaElement;
    expect(textarea?.value).toBe(testCSS);
  });

  it('should save CSS to settings', async () => {
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
    vi.mocked(ipc.updateSettings).mockResolvedValue(undefined);

    await editor.init();

    const textarea = document.getElementById('css-editor-textarea') as HTMLTextAreaElement;
    textarea.value = 'body { color: green; }';

    const saveBtn = document.getElementById('css-editor-save');
    saveBtn?.click();

    await new Promise((resolve) => setTimeout(resolve, 100));

    expect(ipc.updateSettings).toHaveBeenCalled();
  });

  it('should get singleton instance', () => {
    const instance1 = getCSSEditor();
    const instance2 = getCSSEditor();

    expect(instance1).toBe(instance2);
  });
});
