/**
 * Tests for OAuth functionality.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as ipc from './ipc';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('OAuth', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should start OAuth flow', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    vi.mocked(invoke).mockResolvedValue('https://oauth.example.com/auth');

    const url = await ipc.startOAuthFlow('google');

    expect(url).toBe('https://oauth.example.com/auth');
    expect(invoke).toHaveBeenCalledWith('start_oauth_flow', {
      provider_name: 'google',
    });
  });

  it('should handle OAuth callback', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockToken: ipc.OAuthToken = {
      access_token: 'access-token',
      refresh_token: 'refresh-token',
      expires_in: 3600,
      token_type: 'Bearer',
      scope: 'openid email',
    };
    vi.mocked(invoke).mockResolvedValue(mockToken);

    const token = await ipc.handleOAuthCallback('google', 'code-123', 'state-456');

    expect(token).toEqual(mockToken);
    expect(invoke).toHaveBeenCalledWith('handle_oauth_callback', {
      provider_name: 'google',
      code: 'code-123',
      state: 'state-456',
    });
  });

  it('should refresh OAuth token', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    const mockToken: ipc.OAuthToken = {
      access_token: 'new-access-token',
      refresh_token: 'new-refresh-token',
      expires_in: 3600,
      token_type: 'Bearer',
      scope: 'openid email',
    };
    vi.mocked(invoke).mockResolvedValue(mockToken);

    const token = await ipc.refreshOAuthToken('google');

    expect(token).toEqual(mockToken);
    expect(invoke).toHaveBeenCalledWith('refresh_oauth_token', {
      provider_name: 'google',
    });
  });
});
