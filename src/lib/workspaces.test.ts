/**
 * Tests for workspace management.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Workspaces', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should create a workspace', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('workspace-id-123');

    const id = await ipc.createWorkspace('My Workspace', 'https://www.notion.so/my-workspace');
    
    expect(id).toBe('workspace-id-123');
    expect(invoke).toHaveBeenCalledWith('create_workspace', {
      name: 'My Workspace',
      url: 'https://www.notion.so/my-workspace',
    });
  });

  it('should create workspace with default URL', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('workspace-id-123');

    await ipc.createWorkspace('My Workspace');
    
    expect(invoke).toHaveBeenCalledWith('create_workspace', {
      name: 'My Workspace',
      url: undefined,
    });
  });

  it('should list workspaces', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockWorkspaces: ipc.Workspace[] = [
      {
        id: '1',
        name: 'Workspace 1',
        url: 'https://www.notion.so/workspace1',
        zoom_level: 1.0,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockWorkspaces);

    const workspaces = await ipc.listWorkspaces();
    
    expect(workspaces).toEqual(mockWorkspaces);
    expect(invoke).toHaveBeenCalledWith('list_workspaces');
  });

  it('should switch workspace', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.switchWorkspace('workspace-id');
    
    expect(invoke).toHaveBeenCalledWith('switch_workspace', {
      workspaceId: 'workspace-id',
    });
  });

  it('should close workspace', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.closeWorkspace('workspace-id');
    
    expect(invoke).toHaveBeenCalledWith('close_workspace', {
      workspaceId: 'workspace-id',
    });
  });

  it('should handle empty workspace list', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue([]);

    const workspaces = await ipc.listWorkspaces();
    
    expect(workspaces).toEqual([]);
  });

  it('should handle errors when creating workspace', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockRejectedValue(new Error('Failed to create'));

    await expect(ipc.createWorkspace('Test', 'https://example.com')).rejects.toThrow();
  });
});
