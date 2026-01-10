/**
 * Keyboard shortcut handlers for the frontend.
 */

import { toggleFullscreen, setZoom, reloadPage } from './ipc';

// Current zoom level
let currentZoom = 1.0;
const ZOOM_STEP = 0.1;
const ZOOM_MIN = 0.5;
const ZOOM_MAX = 2.0;

/**
 * Sets up keyboard shortcut listeners.
 */
export function setupShortcuts(): void {
  console.log('[Shortcuts] Setting up...');

  document.addEventListener('keydown', handleKeyDown);
}

/**
 * Handles keydown events for shortcuts.
 */
function handleKeyDown(event: KeyboardEvent): void {
  const isCtrl = event.ctrlKey || event.metaKey;

  // F11 - Toggle fullscreen
  if (event.key === 'F11') {
    event.preventDefault();
    toggleFullscreen();
    return;
  }

  if (!isCtrl) return;

  switch (event.key) {
    // Ctrl+R - Reload
    case 'r':
    case 'R':
      if (!event.shiftKey) {
        event.preventDefault();
        reloadPage();
      }
      break;

    // Ctrl+= or Ctrl++ - Zoom in
    case '=':
    case '+':
      event.preventDefault();
      zoomIn();
      break;

    // Ctrl+- - Zoom out
    case '-':
      event.preventDefault();
      zoomOut();
      break;

    // Ctrl+0 - Reset zoom
    case '0':
      event.preventDefault();
      resetZoom();
      break;
  }
}

/**
 * Increases the zoom level.
 */
async function zoomIn(): Promise<void> {
  currentZoom = Math.min(currentZoom + ZOOM_STEP, ZOOM_MAX);
  await setZoom(currentZoom);
  console.log(`[Shortcuts] Zoom: ${(currentZoom * 100).toFixed(0)}%`);
}

/**
 * Decreases the zoom level.
 */
async function zoomOut(): Promise<void> {
  currentZoom = Math.max(currentZoom - ZOOM_STEP, ZOOM_MIN);
  await setZoom(currentZoom);
  console.log(`[Shortcuts] Zoom: ${(currentZoom * 100).toFixed(0)}%`);
}

/**
 * Resets the zoom level to 100%.
 */
async function resetZoom(): Promise<void> {
  currentZoom = 1.0;
  await setZoom(currentZoom);
  console.log('[Shortcuts] Zoom reset to 100%');
}
