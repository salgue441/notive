/**
 * Tests for session management.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Sessions', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should create a session', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('session-id-123');

    const sessionId = await ipc.createSession('account-1', 'test@example.com', 3600);

    expect(sessionId).toBe('session-id-123');
    expect(invoke).toHaveBeenCalledWith('create_session', {
      account_id: 'account-1',
      account_email: 'test@example.com',
      expires_in_seconds: 3600,
    });
  });

  it('should list sessions', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockSessions: ipc.Session[] = [
      {
        id: '1',
        account_id: 'account-1',
        account_email: 'test@example.com',
        created_at: 1000,
        last_used: 2000,
        expires_at: 3000,
        is_active: true,
      },
    ];
    vi.mocked(invoke).mockResolvedValue(mockSessions);

    const sessions = await ipc.listSessions();

    expect(sessions).toEqual(mockSessions);
    expect(invoke).toHaveBeenCalledWith('list_sessions');
  });

  it('should get active session', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockSession: ipc.Session = {
      id: '1',
      account_id: 'account-1',
      account_email: 'test@example.com',
      created_at: 1000,
      last_used: 2000,
      expires_at: undefined,
      is_active: true,
    };
    vi.mocked(invoke).mockResolvedValue(mockSession);

    const session = await ipc.getActiveSession();

    expect(session).toEqual(mockSession);
    expect(invoke).toHaveBeenCalledWith('get_active_session');
  });

  it('should set active session', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.setActiveSession('session-id');

    expect(invoke).toHaveBeenCalledWith('set_active_session', {
      session_id: 'session-id',
    });
  });

  it('should remove session', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(undefined);

    await ipc.removeSession('session-id');

    expect(invoke).toHaveBeenCalledWith('remove_session', {
      session_id: 'session-id',
    });
  });

  it('should cleanup expired sessions', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue(5);

    const removed = await ipc.cleanupExpiredSessions();

    expect(removed).toBe(5);
    expect(invoke).toHaveBeenCalledWith('cleanup_expired_sessions');
  });
});
