/**
 * Performance benchmarks for shortcuts module.
 */

import { bench, describe } from 'vitest';
import { setupShortcuts } from './shortcuts';

describe('Shortcuts Performance Benchmarks', () => {
  beforeEach(() => {
    document.body.innerHTML = '';
    setupShortcuts();
  });

  bench('handle keydown event', () => {
    const event = new KeyboardEvent('keydown', {
      key: 'r',
      ctrlKey: true,
    });
    document.dispatchEvent(event);
  });

  bench('handle F11 event', () => {
    const event = new KeyboardEvent('keydown', { key: 'F11' });
    document.dispatchEvent(event);
  });

  bench('handle zoom in event', () => {
    const event = new KeyboardEvent('keydown', {
      key: '=',
      ctrlKey: true,
    });
    document.dispatchEvent(event);
  });

  bench('handle 1000 keydown events', () => {
    for (let i = 0; i < 1000; i++) {
      const event = new KeyboardEvent('keydown', {
        key: 'a',
        ctrlKey: i % 2 === 0,
      });
      document.dispatchEvent(event);
    }
  });
});
