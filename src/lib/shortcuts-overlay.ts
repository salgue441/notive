/**
 * Notion Shortcuts Overlay - Shows keyboard shortcuts help.
 * Wrapper-level feature to help users discover shortcuts.
 */

import * as ipc from './ipc';

interface ShortcutInfo {
  category: string;
  shortcuts: Array<{
    action: string;
    shortcut: string;
    description?: string;
  }>;
}

export class ShortcutsOverlay {
  private overlay: HTMLElement | null = null;
  private isVisible = false;
  private shortcuts: ShortcutInfo[] = [];

  init(): void {
    this.createOverlay();
    this.loadShortcuts();
    this.setupKeyboardShortcut();
  }

  private createOverlay(): void {
    let overlay = document.getElementById('shortcuts-overlay');
    if (!overlay) {
      overlay = document.createElement('div');
      overlay.id = 'shortcuts-overlay';
      overlay.className = 'shortcuts-overlay';
      document.body.appendChild(overlay);
    }
    this.overlay = overlay;
  }

  private async loadShortcuts(): Promise<void> {
    try {
      const settings = await ipc.getSettings();
      
      this.shortcuts = [
        {
          category: 'Window Management',
          shortcuts: [
            {
              action: 'Toggle Window',
              shortcut: settings.shortcuts.toggle_window,
              description: 'Show/hide the main window',
            },
            {
              action: 'Minimize to Tray',
              shortcut: 'Click minimize button',
              description: 'Minimize window to system tray',
            },
            {
              action: 'Toggle Fullscreen',
              shortcut: 'F11',
              description: 'Enter/exit fullscreen mode',
            },
          ],
        },
        {
          category: 'Navigation',
          shortcuts: [
            {
              action: 'Quick Capture',
              shortcut: settings.shortcuts.quick_capture,
              description: 'Open quick capture/new page',
            },
            {
              action: 'Reload Page',
              shortcut: settings.shortcuts.reload,
              description: 'Refresh the current page',
            },
            {
              action: 'Global Search',
              shortcut: 'Ctrl+K / Cmd+K',
              description: 'Search across bookmarks, history, workspaces',
            },
            {
              action: 'Workspace Switcher',
              shortcut: 'Ctrl+Shift+W / Cmd+Shift+W',
              description: 'Switch between workspaces',
            },
            {
              action: 'Shortcuts Help',
              shortcut: '? / Ctrl+? / Cmd+?',
              description: 'Show this shortcuts overlay',
            },
          ],
        },
        {
          category: 'Zoom',
          shortcuts: [
            {
              action: 'Zoom In',
              shortcut: settings.shortcuts.zoom_in,
              description: 'Increase zoom level',
            },
            {
              action: 'Zoom Out',
              shortcut: settings.shortcuts.zoom_out,
              description: 'Decrease zoom level',
            },
            {
              action: 'Reset Zoom',
              shortcut: settings.shortcuts.zoom_reset,
              description: 'Reset to 100% zoom',
            },
          ],
        },
        {
          category: 'Notion Shortcuts',
          shortcuts: [
            {
              action: 'New Page',
              shortcut: 'Ctrl+N / Cmd+N',
              description: 'Create a new page in Notion',
            },
            {
              action: 'Search',
              shortcut: 'Ctrl+P / Cmd+P',
              description: 'Search in Notion',
            },
            {
              action: 'Toggle Sidebar',
              shortcut: 'Ctrl+\\ / Cmd+\\',
              description: 'Show/hide Notion sidebar',
            },
            {
              action: 'Quick Switch',
              shortcut: 'Ctrl+Shift+P / Cmd+Shift+P',
              description: 'Quick switch between pages',
            },
          ],
        },
      ];
    } catch (error) {
      console.error('Failed to load shortcuts:', error);
    }
  }

  show(): void {
    if (!this.overlay) return;
    this.overlay.style.display = 'flex';
    this.isVisible = true;
  }

  hide(): void {
    if (this.overlay) {
      this.overlay.style.display = 'none';
      this.isVisible = false;
    }
  }

  toggle(): void {
    if (this.isVisible) {
      this.hide();
    } else {
      this.show();
    }
  }

  render(): void {
    if (!this.overlay) return;

    this.overlay.innerHTML = `
      <div class="shortcuts-modal">
        <div class="shortcuts-header">
          <h2>Keyboard Shortcuts</h2>
          <button class="shortcuts-close" id="shortcuts-close">×</button>
        </div>
        <div class="shortcuts-content">
          ${this.shortcuts
            .map(
              (category) => `
            <div class="shortcuts-category">
              <h3>${this.escapeHtml(category.category)}</h3>
              <div class="shortcuts-list">
                ${category.shortcuts
                  .map(
                    (shortcut) => `
                  <div class="shortcut-item">
                    <div class="shortcut-info">
                      <span class="shortcut-action">${this.escapeHtml(shortcut.action)}</span>
                      ${shortcut.description ? `<span class="shortcut-description">${this.escapeHtml(shortcut.description)}</span>` : ''}
                    </div>
                    <kbd class="shortcut-key">${this.formatShortcut(shortcut.shortcut)}</kbd>
                  </div>
                `,
                  )
                  .join('')}
              </div>
            </div>
          `,
            )
            .join('')}
        </div>
        <div class="shortcuts-footer">
          <p>Press <kbd>?</kbd> or <kbd>Esc</kbd> to close</p>
        </div>
      </div>
    `;

    this.setupEventListeners();
  }

  private formatShortcut(shortcut: string): string {
    // Format shortcut for display
    return shortcut
      .replace(/CommandOrControl/g, navigator.platform.includes('Mac') ? '⌘' : 'Ctrl')
      .replace(/Shift/g, '⇧')
      .replace(/Alt/g, '⌥')
      .replace(/\+/g, ' + ')
      .replace(/\s+/g, ' ')
      .trim();
  }

  private setupEventListeners(): void {
    if (!this.overlay) return;

    const closeBtn = this.overlay.querySelector('#shortcuts-close');
    if (closeBtn) {
      closeBtn.addEventListener('click', () => this.hide());
    }

    // Close on Escape
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isVisible) {
        this.hide();
      }
    });

    // Close on overlay click
    this.overlay.addEventListener('click', (e) => {
      if (e.target === this.overlay) {
        this.hide();
      }
    });
  }

  private setupKeyboardShortcut(): void {
    document.addEventListener('keydown', (e) => {
      // ? key or Ctrl+? / Cmd+?
      if (
        e.key === '?' ||
        ((e.ctrlKey || e.metaKey) && e.key === '?') ||
        ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === '/')
      ) {
        // Don't trigger if typing in an input
        if (
          e.target instanceof HTMLInputElement ||
          e.target instanceof HTMLTextAreaElement ||
          (e.target as HTMLElement).isContentEditable
        ) {
          return;
        }
        e.preventDefault();
        this.render();
        this.toggle();
      }
    });
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

let shortcutsOverlayInstance: ShortcutsOverlay | null = null;

export function getShortcutsOverlay(): ShortcutsOverlay {
  if (!shortcutsOverlayInstance) {
    shortcutsOverlayInstance = new ShortcutsOverlay();
  }
  return shortcutsOverlayInstance;
}
