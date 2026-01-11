/**
 * Advanced CSS Editor with syntax highlighting and preview.
 */

import * as ipc from './ipc';

export class CSSEditor {
  private container: HTMLElement | null = null;
  private editor: HTMLTextAreaElement | null = null;
  private preview: HTMLElement | null = null;
  private isVisible = false;

  constructor(containerId: string = 'css-editor-container') {
    this.container = document.getElementById(containerId);
    if (!this.container) {
      console.warn(`CSS Editor container #${containerId} not found`);
    }
  }

  async init(): Promise<void> {
    if (!this.container) return;

    this.createEditorHTML();
    await this.loadCSS();
    this.setupEventListeners();
  }

  private createEditorHTML(): void {
    if (!this.container) return;

    this.container.innerHTML = `
      <div class="css-editor-modal" id="css-editor-modal">
        <div class="css-editor-header">
          <h2>Advanced CSS Editor</h2>
          <button class="css-editor-close" id="css-editor-close">×</button>
        </div>
        <div class="css-editor-content">
          <div class="css-editor-toolbar">
            <button class="btn btn-small" id="css-editor-apply">Apply</button>
            <button class="btn btn-small" id="css-editor-reset">Reset</button>
            <button class="btn btn-small" id="css-editor-save">Save</button>
            <select id="css-theme-select">
              <option value="default">Default Theme</option>
              <option value="dark">Dark Theme</option>
              <option value="custom">Custom Theme</option>
            </select>
          </div>
          <div class="css-editor-split">
            <div class="css-editor-pane">
              <textarea
                id="css-editor-textarea"
                class="css-editor-textarea"
                placeholder="Enter your CSS here..."
                spellcheck="false"
              ></textarea>
            </div>
            <div class="css-preview-pane">
              <div class="css-preview-content" id="css-preview-content">
                <h3>Preview</h3>
                <p>Your CSS will be previewed here.</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    `;

    this.editor = document.getElementById('css-editor-textarea') as HTMLTextAreaElement;
    this.preview = document.getElementById('css-preview-content');
  }

  private async loadCSS(): Promise<void> {
    try {
      const settings = await ipc.getSettings();
      if (this.editor) {
        this.editor.value = settings.custom_css || '';
        this.updatePreview();
      }
    } catch (error) {
      console.error('Failed to load CSS:', error);
    }
  }

  private setupEventListeners(): void {
    const closeBtn = document.getElementById('css-editor-close');
    const applyBtn = document.getElementById('css-editor-apply');
    const resetBtn = document.getElementById('css-editor-reset');
    const saveBtn = document.getElementById('css-editor-save');

    if (closeBtn) {
      closeBtn.addEventListener('click', () => this.hide());
    }

    if (applyBtn && this.editor) {
      applyBtn.addEventListener('click', () => {
        this.applyCSS(this.editor!.value);
      });
    }

    if (resetBtn) {
      resetBtn.addEventListener('click', () => {
        if (this.editor) {
          this.editor.value = '';
          this.updatePreview();
        }
      });
    }

    if (saveBtn && this.editor) {
      saveBtn.addEventListener('click', async () => {
        await this.saveCSS(this.editor!.value);
      });
    }

    if (this.editor) {
      this.editor.addEventListener('input', () => {
        this.updatePreview();
      });
    }

    // Close on Escape
    document.addEventListener('keydown', (e) => {
      if (e.key === 'Escape' && this.isVisible) {
        this.hide();
      }
    });
  }

  private updatePreview(): void {
    if (!this.preview || !this.editor) return;

    const css = this.editor.value;
    const styleId = 'css-editor-preview-style';
    let style = document.getElementById(styleId);

    if (!style) {
      style = document.createElement('style');
      style.id = styleId;
      document.head.appendChild(style);
    }

    style.textContent = css;
  }

  private applyCSS(css: string): void {
    const styleId = 'notive-custom-css';
    let style = document.getElementById(styleId);

    if (!style) {
      style = document.createElement('style');
      style.id = styleId;
      document.head.appendChild(style);
    }

    style.textContent = css;
  }

  private async saveCSS(css: string): Promise<void> {
    try {
      const settings = await ipc.getSettings();
      settings.custom_css = css;
      settings.custom_css_enabled = true;
      await ipc.updateSettings(settings);
      alert('CSS saved successfully!');
    } catch (error) {
      console.error('Failed to save CSS:', error);
      alert('Failed to save CSS. Please try again.');
    }
  }

  show(): void {
    if (this.container) {
      const modal = document.getElementById('css-editor-modal');
      if (modal) {
        modal.style.display = 'flex';
        this.isVisible = true;
        if (this.editor) {
          this.editor.focus();
        }
      }
    }
  }

  hide(): void {
    if (this.container) {
      const modal = document.getElementById('css-editor-modal');
      if (modal) {
        modal.style.display = 'none';
        this.isVisible = false;
      }
    }
  }

  toggle(): void {
    if (this.isVisible) {
      this.hide();
    } else {
      this.show();
    }
  }
}

let cssEditorInstance: CSSEditor | null = null;

export function getCSSEditor(): CSSEditor {
  if (!cssEditorInstance) {
    cssEditorInstance = new CSSEditor();
  }
  return cssEditorInstance;
}
