/**
 * Global search UI component.
 */

import { globalSearch, type SearchResult } from './ipc';

/**
 * Global search overlay component.
 */
export class GlobalSearch {
  private overlay: HTMLElement | null = null;
  private input: HTMLInputElement | null = null;
  private results: HTMLElement | null = null;
  private isVisible = false;

  /**
   * Initializes the global search.
   */
  init(): void {
    this.createOverlay();
    this.setupKeyboardShortcut();
  }

  /**
   * Creates the search overlay.
   */
  private createOverlay(): void {
    // Create overlay
    this.overlay = document.createElement('div');
    this.overlay.id = 'global-search-overlay';
    this.overlay.className = 'search-overlay';
    this.overlay.innerHTML = `
      <div class="search-container">
        <div class="search-header">
          <input
            type="text"
            id="global-search-input"
            class="search-input"
            placeholder="Search bookmarks, history, workspaces..."
            autocomplete="off"
          />
          <button class="search-close" id="search-close-btn">×</button>
        </div>
        <div class="search-results" id="search-results"></div>
      </div>
    `;

    document.body.appendChild(this.overlay);

    // Get elements
    this.input = document.getElementById('global-search-input') as HTMLInputElement;
    this.results = document.getElementById('search-results') as HTMLElement;
    const closeBtn = document.getElementById('search-close-btn') as HTMLButtonElement;

    // Setup event listeners
    this.input.addEventListener('input', () => this.performSearch());
    this.input.addEventListener('keydown', (e) => this.handleKeyDown(e));
    closeBtn.addEventListener('click', () => this.hide());

    // Close on overlay click
    this.overlay.addEventListener('click', (e) => {
      if (e.target === this.overlay) {
        this.hide();
      }
    });
  }

  /**
   * Shows the search overlay.
   */
  show(): void {
    if (!this.overlay || !this.input) return;

    this.overlay.style.display = 'flex';
    this.input.focus();
    this.isVisible = true;
  }

  /**
   * Hides the search overlay.
   */
  hide(): void {
    if (!this.overlay || !this.input) return;

    this.overlay.style.display = 'none';
    this.input.value = '';
    this.results!.innerHTML = '';
    this.isVisible = false;
  }

  /**
   * Toggles the search overlay.
   */
  toggle(): void {
    if (this.isVisible) {
      this.hide();
    } else {
      this.show();
    }
  }

  /**
   * Performs a search.
   */
  private async performSearch(): Promise<void> {
    if (!this.input || !this.results) return;

    const query = this.input.value.trim();
    if (query.length < 2) {
      this.results.innerHTML = '<div class="search-empty">Type at least 2 characters to search</div>';
      return;
    }

    try {
      const searchResults = await globalSearch(query);
      this.renderResults(searchResults);
    } catch (error) {
      console.error('Search failed:', error);
      this.results.innerHTML = '<div class="search-error">Search failed. Please try again.</div>';
    }
  }

  /**
   * Renders search results.
   */
  private renderResults(results: SearchResult[]): void {
    if (!this.results) return;

    if (results.length === 0) {
      this.results.innerHTML = '<div class="search-empty">No results found</div>';
      return;
    }

    const html = results
      .map(
        (result) => `
      <div class="search-result" data-url="${this.escapeHtml(result.url)}">
        <div class="search-result-title">${this.escapeHtml(result.title)}</div>
        <div class="search-result-url">${this.escapeHtml(result.url)}</div>
        <div class="search-result-source">${result.source}</div>
      </div>
    `
      )
      .join('');

    this.results.innerHTML = html;

    // Add click handlers
    this.results.querySelectorAll('.search-result').forEach((el) => {
      el.addEventListener('click', () => {
        const url = (el as HTMLElement).dataset.url;
        if (url) {
          window.location.href = url;
          this.hide();
        }
      });
    });
  }

  /**
   * Handles keyboard events.
   */
  private handleKeyDown(e: KeyboardEvent): void {
    if (e.key === 'Escape') {
      this.hide();
    } else if (e.key === 'Enter') {
      const firstResult = this.results?.querySelector('.search-result') as HTMLElement;
      if (firstResult) {
        firstResult.click();
      }
    }
  }

  /**
   * Sets up keyboard shortcut (Ctrl+K or Cmd+K).
   */
  private setupKeyboardShortcut(): void {
    document.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
        e.preventDefault();
        this.toggle();
      }
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
 * Global search instance.
 */
let globalSearchInstance: GlobalSearch | null = null;

/**
 * Gets or creates the global search instance.
 */
export function getGlobalSearch(): GlobalSearch {
  if (!globalSearchInstance) {
    globalSearchInstance = new GlobalSearch();
  }
  return globalSearchInstance;
}
