/**
 * Notive - Main entry point
 *
 * This file initializes the frontend bridges for:
 * - Notification interception (web -> native)
 * - Download handling
 * - Keyboard shortcuts
 * - IPC communication with Rust backend
 */

import { setupNotificationBridge } from './lib/notifications';
import { setupShortcuts } from './lib/shortcuts';
import { setupIPC } from './lib/ipc';
import { setupOfflineIndicator } from './lib/offline';
import { getGlobalSearch } from './lib/search';
import { initSidebar } from './lib/sidebar';
import { getWorkspaceSwitcher } from './lib/workspace-switcher';
import { getShortcutsOverlay } from './lib/shortcuts-overlay';
import { getCSSEditor } from './lib/css-editor';

/**
 * Initialize all frontend bridges and handlers.
 */
async function init(): Promise<void> {
  console.log('[Notive] Initializing...');

  try {
    // Setup IPC communication
    await setupIPC();

    // Bridge web notifications to native
    setupNotificationBridge();

    // Setup keyboard shortcut handlers
    setupShortcuts();

    // Setup offline indicator
    setupOfflineIndicator();

    // Setup global search
    getGlobalSearch().init();

    // Setup sidebar
    await initSidebar();

    // Setup workspace switcher
    getWorkspaceSwitcher().init();

    // Setup shortcuts overlay
    getShortcutsOverlay().init();

    // Setup CSS editor
    getCSSEditor().init();

    console.log('[Notive] Initialization complete');
  } catch (error) {
    console.error('[Notive] Initialization failed:', error);
  }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
