/**
 * Sidebar component for quick access to recent pages and bookmarks.
 */

import * as ipc from './ipc';

export interface SidebarItem {
  id: string;
  title: string;
  url: string;
  type: 'bookmark' | 'history' | 'workspace';
  icon?: string;
}

export class Sidebar {
  private container: HTMLElement | null = null;
  private isCollapsed = false;
  private recentPages: ipc.HistoryEntry[] = [];
  private bookmarks: ipc.Bookmark[] = [];

  constructor(containerId: string = 'sidebar-container') {
    this.container = document.getElementById(containerId);
    if (!this.container) {
      console.warn(`Sidebar container #${containerId} not found`);
    }
  }

  async init(): Promise<void> {
    if (!this.container) return;

    this.createSidebarHTML();
    await this.loadData();
    this.render();
    this.setupEventListeners();
  }

  private createSidebarHTML(): void {
    if (!this.container) return;

    this.container.innerHTML = `
      <div class="sidebar">
        <div class="sidebar-header">
          <button class="sidebar-toggle" id="sidebar-toggle" title="Toggle sidebar">
            <span class="toggle-icon">◀</span>
          </button>
          <h3 class="sidebar-title">Quick Access</h3>
        </div>
        <div class="sidebar-content" id="sidebar-content">
          <div class="sidebar-section">
            <h4 class="section-title">Recent Pages</h4>
            <ul class="sidebar-list" id="recent-pages-list"></ul>
          </div>
          <div class="sidebar-section">
            <h4 class="section-title">Bookmarks</h4>
            <ul class="sidebar-list" id="bookmarks-list"></ul>
          </div>
        </div>
      </div>
    `;
  }

  private async loadData(): Promise<void> {
    try {
      this.recentPages = await ipc.getRecentPages(10);
      this.bookmarks = await ipc.listBookmarks();
    } catch (error) {
      console.error('Failed to load sidebar data:', error);
    }
  }

  private render(): void {
    if (!this.container) return;

    const recentList = document.getElementById('recent-pages-list');
    const bookmarksList = document.getElementById('bookmarks-list');

    if (recentList) {
      recentList.innerHTML = this.recentPages
        .map(
          (entry) => `
        <li class="sidebar-item" data-url="${this.escapeHtml(entry.url)}">
          <a href="#" class="sidebar-link" data-url="${this.escapeHtml(entry.url)}">
            <span class="item-title">${this.escapeHtml(entry.title)}</span>
            <span class="item-url">${this.escapeHtml(entry.url)}</span>
          </a>
        </li>
      `,
        )
        .join('');
    }

    if (bookmarksList) {
      bookmarksList.innerHTML = this.bookmarks
        .map(
          (bookmark) => `
        <li class="sidebar-item" data-url="${this.escapeHtml(bookmark.url)}">
          <a href="#" class="sidebar-link" data-url="${this.escapeHtml(bookmark.url)}">
            <span class="item-title">${this.escapeHtml(bookmark.title)}</span>
            <span class="item-url">${this.escapeHtml(bookmark.url)}</span>
          </a>
        </li>
      `,
        )
        .join('');
    }
  }

  private setupEventListeners(): void {
    const toggle = document.getElementById('sidebar-toggle');
    if (toggle) {
      toggle.addEventListener('click', () => this.toggle());
    }

    // Handle clicks on sidebar items
    this.container?.addEventListener('click', (e) => {
      const target = e.target as HTMLElement;
      const link = target.closest('.sidebar-link') as HTMLElement;
      if (link) {
        e.preventDefault();
        const url = link.dataset.url;
        if (url) {
          window.location.href = url;
        }
      }
    });
  }

  toggle(): void {
    this.isCollapsed = !this.isCollapsed;
    const content = document.getElementById('sidebar-content');
    const toggleIcon = document.querySelector('.toggle-icon');
    const sidebar = this.container?.querySelector('.sidebar');

    if (content && sidebar) {
      if (this.isCollapsed) {
        content.style.display = 'none';
        sidebar.classList.add('collapsed');
        if (toggleIcon) {
          toggleIcon.textContent = '▶';
        }
      } else {
        content.style.display = 'block';
        sidebar.classList.remove('collapsed');
        if (toggleIcon) {
          toggleIcon.textContent = '◀';
        }
      }
    }
  }

  async refresh(): Promise<void> {
    await this.loadData();
    this.render();
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

/**
 * Initializes sidebar component.
 */
export async function initSidebar(containerId: string = 'sidebar-container'): Promise<Sidebar> {
  const sidebar = new Sidebar(containerId);
  await sidebar.init();
  return sidebar;
}
