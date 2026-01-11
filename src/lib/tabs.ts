/**
 * Tab bar UI component and management.
 */

import { listTabs, openTab, closeTab, switchTab, type Tab } from './ipc';

/**
 * Creates and manages a tab bar UI component.
 */
export class TabBar {
  private container: HTMLElement | null = null;
  private tabs: Tab[] = [];
  private activeTabId: string | null = null;
  private windowLabel: string;

  constructor(windowLabel: string = 'main') {
    this.windowLabel = windowLabel;
  }

  /**
   * Initializes the tab bar in the DOM.
   */
  async init(containerId: string): Promise<void> {
    this.container = document.getElementById(containerId);
    if (!this.container) {
      throw new Error(`Container element not found: ${containerId}`);
    }

    // Create tab bar HTML
    this.container.innerHTML = `
      <div class="tab-bar">
        <div class="tab-list" id="tab-list-${this.windowLabel}"></div>
        <button class="tab-new" id="tab-new-${this.windowLabel}" title="New Tab">+</button>
      </div>
    `;

    // Load existing tabs
    await this.loadTabs();

    // Setup event listeners
    this.setupEventListeners();
  }

  /**
   * Loads tabs from the backend.
   */
  async loadTabs(): Promise<void> {
    try {
      this.tabs = await listTabs(this.windowLabel);
      this.render();
    } catch (error) {
      console.error('Failed to load tabs:', error);
    }
  }

  /**
   * Renders the tab bar.
   */
  private render(): void {
    const tabList = document.getElementById(`tab-list-${this.windowLabel}`);
    if (!tabList) return;

    tabList.innerHTML = '';

    this.tabs.forEach((tab) => {
      const tabElement = document.createElement('div');
      tabElement.className = `tab ${tab.id === this.activeTabId ? 'active' : ''}`;
      tabElement.dataset.tabId = tab.id;
      tabElement.innerHTML = `
        <span class="tab-title">${this.escapeHtml(tab.title)}</span>
        <button class="tab-close" data-tab-id="${tab.id}" title="Close Tab">×</button>
      `;

      // Click to switch tab
      tabElement.addEventListener('click', (e) => {
        if (!(e.target as HTMLElement).classList.contains('tab-close')) {
          this.switchToTab(tab.id);
        }
      });

      // Close button
      const closeBtn = tabElement.querySelector('.tab-close');
      closeBtn?.addEventListener('click', (e) => {
        e.stopPropagation();
        this.closeTab(tab.id);
      });

      tabList.appendChild(tabElement);
    });
  }

  /**
   * Opens a new tab.
   */
  async openTab(url: string, title?: string): Promise<void> {
    try {
      const tabId = await openTab(this.windowLabel, url, title);
      await this.loadTabs();
      this.switchToTab(tabId);
    } catch (error) {
      console.error('Failed to open tab:', error);
    }
  }

  /**
   * Closes a tab.
   */
  async closeTab(tabId: string): Promise<void> {
    try {
      await closeTab(this.windowLabel, tabId);
      await this.loadTabs();
      
      // Switch to another tab if needed
      if (this.activeTabId === tabId && this.tabs.length > 0) {
        const remainingTabs = this.tabs.filter((t) => t.id !== tabId);
        if (remainingTabs.length > 0) {
          this.switchToTab(remainingTabs[0].id);
        }
      }
    } catch (error) {
      console.error('Failed to close tab:', error);
    }
  }

  /**
   * Switches to a different tab.
   */
  async switchToTab(tabId: string): Promise<void> {
    try {
      await switchTab(this.windowLabel, tabId);
      this.activeTabId = tabId;
      this.render();
    } catch (error) {
      console.error('Failed to switch tab:', error);
    }
  }

  /**
   * Sets up event listeners.
   */
  private setupEventListeners(): void {
    const newTabBtn = document.getElementById(`tab-new-${this.windowLabel}`);
    newTabBtn?.addEventListener('click', () => {
      this.openTab('https://www.notion.so');
    });
  }

  /**
   * Escapes HTML to prevent XSS.
   */
  private escapeHtml(text: string): string {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }
}

/**
 * Initializes tab bar for the main window.
 */
export async function initTabBar(containerId: string = 'tab-bar-container'): Promise<TabBar> {
  const tabBar = new TabBar('main');
  await tabBar.init(containerId);
  return tabBar;
}
