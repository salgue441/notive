/**
 * Offline indicator - monitors network connectivity.
 */

/**
 * Sets up the offline indicator.
 */
export function setupOfflineIndicator(): void {
  console.log('[Offline] Setting up indicator...');

  // Monitor online/offline events
  window.addEventListener('online', handleOnline);
  window.addEventListener('offline', handleOffline);

  // Initial state
  if (!navigator.onLine) {
    handleOffline();
  }
}

/**
 * Handles when the app comes online.
 */
function handleOnline(): void {
  console.log('[Offline] Connection restored');
  showOfflineIndicator(false);
}

/**
 * Handles when the app goes offline.
 */
function handleOffline(): void {
  console.log('[Offline] Connection lost');
  showOfflineIndicator(true);
}

/**
 * Shows or hides the offline indicator.
 */
function showOfflineIndicator(show: boolean): void {
  // Check if indicator already exists
  let indicator = document.getElementById('notive-offline-indicator');
  
  if (show && !indicator) {
    // Create indicator
    indicator = document.createElement('div');
    indicator.id = 'notive-offline-indicator';
    indicator.textContent = 'Offline - No internet connection';
    indicator.style.cssText = `
      position: fixed;
      top: 0;
      left: 0;
      right: 0;
      background: #e74c3c;
      color: white;
      padding: 0.75rem 1rem;
      text-align: center;
      font-size: 0.9rem;
      z-index: 10000;
      box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    `;
    document.body.appendChild(indicator);
  } else if (!show && indicator) {
    // Remove indicator
    indicator.remove();
  }
}
