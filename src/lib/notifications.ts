/**
 * Notification bridge - intercepts web notifications and forwards to native.
 */

import { invoke } from '@tauri-apps/api/core';

/**
 * Notification payload sent to the backend.
 */
interface NotificationPayload {
  title: string;
  body: string | null;
  icon: string | null;
}

/**
 * Sets up the notification bridge.
 *
 * Replaces the browser's Notification API with a custom implementation
 * that forwards notifications to the native system.
 */
export function setupNotificationBridge(): void {
  console.log('[Notifications] Setting up bridge...');

  // Store original Notification for reference
  const OriginalNotification = window.Notification;

  // Create a custom Notification class
  class NativeNotification {
    static permission: NotificationPermission = 'granted';

    static async requestPermission(): Promise<NotificationPermission> {
      // Always granted - we handle permissions natively
      return 'granted';
    }

    constructor(title: string, options?: NotificationOptions) {
      // Forward to native notification
      const payload: NotificationPayload = {
        title,
        body: options?.body ?? null,
        icon: options?.icon ?? null,
      };

      invoke('show_notification', { payload }).catch((error) => {
        console.error('[Notifications] Failed to show notification:', error);
      });
    }
  }

  // Replace browser Notification with our bridge
  try {
    Object.defineProperty(window, 'Notification', {
      value: NativeNotification,
      writable: false,
      configurable: false,
    });
    console.log('[Notifications] Bridge installed');
  } catch (error) {
    console.warn('[Notifications] Could not replace Notification API:', error);
  }
}
