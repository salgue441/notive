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
