/**
 * Workspace Switcher UI component.
 * Manages multiple Notion window instances (wrapper-level feature).
 */

import * as ipc from './ipc';

export class WorkspaceSwitcher {
  private overlay: HTMLElement | null = null;
  private isVisible = false;
  private workspaces: ipc.Workspace[] = [];

  init(): void {
    this.createOverlay();
    this.setupKeyboardShortcut();
  }

  private createOverlay(): void {
    // Create overlay if it doesn't exist
    let overlay = document.getElementById('workspace-switcher-overlay');
    if (!overlay) {
      overlay = document.createElement('div');
      overlay.id = 'workspace-switcher-overlay';
      overlay.className = 'workspace-switcher-overlay';
      document.body.appendChild(overlay);
    }
    this.overlay = overlay;
  }

  async show(): Promise<void> {
    if (!this.overlay) return;

    try {
      this.workspaces = await ipc.listWorkspaces();
      this.render();
      this.overlay.style.display = 'flex';
      this.isVisible = true;

      // Focus search input if it exists
      const searchInput = this.overlay.querySelector('.workspace-search') as HTMLInputElement;
      if (searchInput) {
        searchInput.focus();
      }
    } catch (error) {
      console.error('Failed to load workspaces:', error);
    }
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

  private render(): void {
    if (!this.overlay) return;

    this.overlay.innerHTML = `
      <div class="workspace-switcher-modal">
        <div class="workspace-switcher-header">
          <h2>Switch Workspace</h2>
          <button class="workspace-switcher-close" id="workspace-switcher-close">×</button>
        </div>
        <div class="workspace-switcher-search">
          <input
            type="text"
            class="workspace-search"
            placeholder="Search workspaces..."
            id="workspace-search-input"
          />
        </div>
        <div class="workspace-switcher-list" id="workspace-list">
          ${this.renderWorkspaces(this.workspaces)}
        </div>
        <div class="workspace-switcher-actions">
          <button class="btn btn-primary" id="workspace-create-btn">Create New Workspace</button>
        </div>
      </div>
    `;

    this.setupEventListeners();
  }

  private renderWorkspaces(workspaces: ipc.Workspace[]): string {
    if (workspaces.length === 0) {
      return '<div class="workspace-empty">No workspaces yet. Create one to get started.</div>';
    }

    return workspaces
      .map(
        (workspace) => `
      <div class="workspace-item" data-workspace-id="${this.escapeHtml(workspace.id)}">
        <div class="workspace-info">
          <div class="workspace-name">${this.escapeHtml(workspace.name)}</div>
          <div class="workspace-url">${this.escapeHtml(workspace.url)}</div>
        </div>
        <div class="workspace-actions">
          <button class="btn btn-small btn-primary switch-workspace-btn" data-workspace-id="${this.escapeHtml(workspace.id)}">
            Switch
          </button>
          <button class="btn btn-small btn-danger remove-workspace-btn" data-workspace-id="${this.escapeHtml(workspace.id)}">
            Remove
          </button>
        </div>
      </div>
    `,
      )
      .join('');
  }

  private setupEventListeners(): void {
    if (!this.overlay) return;

    // Close button
    const closeBtn = this.overlay.querySelector('#workspace-switcher-close');
    if (closeBtn) {
      closeBtn.addEventListener('click', () => this.hide());
    }

    // Create workspace button
    const createBtn = this.overlay.querySelector('#workspace-create-btn');
    if (createBtn) {
      createBtn.addEventListener('click', () => this.showCreateWorkspaceDialog());
    }

    // Switch workspace buttons
    this.overlay.addEventListener('click', async (e) => {
      const target = e.target as HTMLElement;
      
      if (target.classList.contains('switch-workspace-btn')) {
        const workspaceId = target.dataset.workspaceId;
        if (workspaceId) {
          try {
            await ipc.switchWorkspace(workspaceId);
            this.hide();
          } catch (error) {
            console.error('Failed to switch workspace:', error);
            alert('Failed to switch workspace. Please try again.');
          }
        }
      }
      
      if (target.classList.contains('remove-workspace-btn')) {
        const workspaceId = target.dataset.workspaceId;
        if (workspaceId && confirm('Remove this workspace? The window will be closed.')) {
          try {
            await ipc.closeWorkspace(workspaceId);
            await this.show(); // Refresh list
          } catch (error) {
            console.error('Failed to remove workspace:', error);
            alert('Failed to remove workspace. Please try again.');
          }
        }
      }
    });

    // Search functionality
    const searchInput = this.overlay.querySelector('#workspace-search-input') as HTMLInputElement;
    if (searchInput) {
      searchInput.addEventListener('input', (e) => {
        const query = (e.target as HTMLInputElement).value.toLowerCase();
        this.filterWorkspaces(query);
      });
    }

    // Close on Escape
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isVisible) {
        this.hide();
      }
    });

    // Close on overlay click (outside modal)
    this.overlay.addEventListener('click', (e) => {
      if (e.target === this.overlay) {
        this.hide();
      }
    });
  }

  private filterWorkspaces(query: string): void {
    const list = this.overlay?.querySelector('#workspace-list');
    if (!list) return;

    const items = list.querySelectorAll('.workspace-item');
    items.forEach((item) => {
      const name = item.querySelector('.workspace-name')?.textContent?.toLowerCase() || '';
      const url = item.querySelector('.workspace-url')?.textContent?.toLowerCase() || '';
      const matches = name.includes(query) || url.includes(query);
      (item as HTMLElement).style.display = matches ? 'flex' : 'none';
    });
  }

  private async showCreateWorkspaceDialog(): Promise<void> {
    const name = prompt('Workspace name:');
    if (!name) return;

    const url = prompt('Notion URL (leave empty for default):') || undefined;

    try {
      await ipc.createWorkspace(name, url);
      alert('Workspace created successfully!');
      await this.show(); // Refresh list
    } catch (error) {
      console.error('Failed to create workspace:', error);
      alert('Failed to create workspace. Please try again.');
    }
  }

  private setupKeyboardShortcut(): void {
    document.addEventListener('keydown', (e) => {
      // Ctrl+Shift+W or Cmd+Shift+W to open workspace switcher
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'w') {
        e.preventDefault();
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

/**
 * Gets the global workspace switcher instance.
 */
let workspaceSwitcherInstance: WorkspaceSwitcher | null = null;

export function getWorkspaceSwitcher(): WorkspaceSwitcher {
  if (!workspaceSwitcherInstance) {
    workspaceSwitcherInstance = new WorkspaceSwitcher();
  }
  return workspaceSwitcherInstance;
}
