/**
 * Performance benchmarks for offline indicator.
 */

import { bench, describe, beforeEach } from 'vitest';
import { setupOfflineIndicator } from './offline';

describe('Offline Indicator Performance Benchmarks', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
    Object.defineProperty(navigator, 'onLine', {
      writable: true,
      value: true,
      configurable: true,
    });
    setupOfflineIndicator();
  });

  bench('show offline indicator', () => {
    Object.defineProperty(navigator, 'onLine', {
      writable: true,
      value: false,
      configurable: true,
    });
    const event = new Event('offline');
    window.dispatchEvent(event);
  });

  bench('hide offline indicator', () => {
    Object.defineProperty(navigator, 'onLine', {
      writable: true,
      value: true,
      configurable: true,
    });
    const event = new Event('online');
    window.dispatchEvent(event);
  });

  bench('toggle online/offline 100 times', () => {
    for (let i = 0; i < 100; i++) {
      Object.defineProperty(navigator, 'onLine', {
        writable: true,
        value: i % 2 === 0,
        configurable: true,
      });
      const event = new Event(i % 2 === 0 ? 'online' : 'offline');
      window.dispatchEvent(event);
    }
  });
});
