import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html', 'lcov'],
      exclude: [
        'node_modules/',
        'src-tauri/',
        'dist/',
        '**/*.d.ts',
        '**/*.config.*',
        '**/settings.html',
        '**/index.html',
      ],
      thresholds: {
        lines: 95,
        functions: 95,
        branches: 95,
        statements: 95,
      },
    },
    include: ['src/**/*.{test,spec}.{js,ts}'],
    benchmark: {
      include: ['src/**/*.{bench,benchmark}.{js,ts}'],
    },
  },
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
});
