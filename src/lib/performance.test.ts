/**
 * Tests for performance monitoring.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Performance', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should get resource usage', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockUsage: ipc.ResourceUsage = {
      memory_mb: 150.5,
      cpu_percent: 25.0,
      timestamp: 1000,
    };
    vi.mocked(invoke).mockResolvedValue(mockUsage);

    const usage = await ipc.getResourceUsage();

    expect(usage).toEqual(mockUsage);
    expect(invoke).toHaveBeenCalledWith('get_resource_usage');
  });

  it('should get performance metrics', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockMetrics: ipc.PerformanceMetrics = {
      memory_usage: [],
      cpu_usage: [],
      average_memory_mb: 100.0,
      average_cpu_percent: 10.0,
      peak_memory_mb: 150.0,
      peak_cpu_percent: 25.0,
    };
    vi.mocked(invoke).mockResolvedValue(mockMetrics);

    const metrics = await ipc.getPerformanceMetrics();

    expect(metrics).toEqual(mockMetrics);
    expect(invoke).toHaveBeenCalledWith('get_performance_metrics');
  });
});
