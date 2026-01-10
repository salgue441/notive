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

    // Event handlers
    onclick: ((this: NativeNotification, ev: Event) => any) | null = null;
    onclose: ((this: NativeNotification, ev: Event) => any) | null = null;
    onshow: ((this: NativeNotification, ev: Event) => any) | null = null;
    onerror: ((this: NativeNotification, ev: Event) => any) | null = null;

    // Standard properties
    readonly title: string;
    readonly tag: string | undefined;
    readonly data: any;
    readonly dir: NotificationDirection;
    readonly lang: string;
    readonly body: string;
    readonly icon: string;
    readonly badge: string;
    readonly image: string;
    readonly renotify: boolean;
    readonly requireInteraction: boolean;
    readonly silent: boolean;
    readonly timestamp: number;
    readonly vibrate: readonly number[];

    constructor(title: string, options?: NotificationOptions) {
      // Store title
      this.title = title;
      // Store properties from options
      this.tag = options?.tag;
      this.data = options?.data;
      this.dir = options?.dir ?? 'auto';
      this.lang = options?.lang ?? '';
      this.body = options?.body ?? '';
      this.icon = options?.icon ?? '';
      this.badge = options?.badge ?? '';
      this.image = options?.image ?? '';
      this.renotify = options?.renotify ?? false;
      this.requireInteraction = options?.requireInteraction ?? false;
      this.silent = options?.silent ?? false;
      this.timestamp = options?.timestamp ?? Date.now();
      this.vibrate = options?.vibrate ? [...options.vibrate] : [];

      // Forward to native notification
      const payload: NotificationPayload = {
        title,
        body: options?.body ?? null,
        icon: options?.icon ?? null,
      };

      invoke('show_notification', { payload })
        .then(() => {
          // Trigger onshow event if handler exists
          if (this.onshow) {
            try {
              this.onshow.call(this, new Event('show'));
            } catch (error) {
              console.error('[Notifications] Error in onshow handler:', error);
            }
          }
        })
        .catch((error) => {
          console.error('[Notifications] Failed to show notification:', error);
          // Trigger onerror event if handler exists
          if (this.onerror) {
            try {
              this.onerror.call(this, new Event('error'));
            } catch (err) {
              console.error('[Notifications] Error in onerror handler:', err);
            }
          }
        });
    }

    /**
     * Closes the notification.
     * Note: Native notifications may not support programmatic closing,
     * but we implement this for API compatibility.
     */
    close(): void {
      // Native notifications typically auto-dismiss, but we trigger onclose
      // for API compatibility with web notification behavior
      if (this.onclose) {
        try {
          this.onclose.call(this, new Event('close'));
        } catch (error) {
          console.error('[Notifications] Error in onclose handler:', error);
        }
      }
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
