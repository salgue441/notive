/**
 * Tests for account management.
 */

import { describe, it, expect, vi, beforeEach } from 'vitest';
import { AccountManager } from './accounts';
import * as ipc from './ipc';

// Mock IPC functions
vi.mock('./ipc', () => ({
  listAccounts: vi.fn(),
  addAccount: vi.fn(),
  removeAccount: vi.fn(),
  switchAccount: vi.fn(),
  startOAuthFlow: vi.fn(),
}));

describe('AccountManager', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it('should initialize account manager', async () => {
    vi.mocked(ipc.listAccounts).mockResolvedValue([]);

    const manager = new AccountManager();
    await manager.init();

    expect(ipc.listAccounts).toHaveBeenCalled();
  });

  it('should load accounts', async () => {
    const mockAccounts: ipc.Account[] = [
      {
        id: '1',
        provider: 'Google',
        email: 'test@example.com',
        is_active: true,
      },
    ];
    vi.mocked(ipc.listAccounts).mockResolvedValue(mockAccounts);

    const manager = new AccountManager();
    await manager.loadAccounts();

    expect(manager.getAllAccounts()).toEqual(mockAccounts);
  });

  it('should get active account', async () => {
    const mockAccounts: ipc.Account[] = [
      {
        id: '1',
        provider: 'Google',
        email: 'test@example.com',
        is_active: true,
      },
      {
        id: '2',
        provider: 'Microsoft',
        email: 'test2@example.com',
        is_active: false,
      },
    ];
    vi.mocked(ipc.listAccounts).mockResolvedValue(mockAccounts);

    const manager = new AccountManager();
    await manager.loadAccounts();

    const active = manager.getActiveAccount();
    expect(active).not.toBeNull();
    expect(active?.id).toBe('1');
    expect(active?.is_active).toBe(true);
  });

  it('should switch to an account', async () => {
    vi.mocked(ipc.listAccounts).mockResolvedValue([]);
    vi.mocked(ipc.switchAccount).mockResolvedValue(undefined);

    const manager = new AccountManager();
    await manager.switchToAccount('account-id');

    expect(ipc.switchAccount).toHaveBeenCalledWith('account-id');
  });

  it('should remove an account', async () => {
    vi.mocked(ipc.listAccounts).mockResolvedValue([]);
    vi.mocked(ipc.removeAccount).mockResolvedValue(undefined);

    // Mock confirm to return true
    global.confirm = vi.fn(() => true);

    const manager = new AccountManager();
    await manager.removeAccountById('account-id');

    expect(ipc.removeAccount).toHaveBeenCalledWith('account-id');
  });

  it('should not remove account if cancelled', async () => {
    vi.mocked(ipc.listAccounts).mockResolvedValue([]);

    // Mock confirm to return false
    global.confirm = vi.fn(() => false);

    const manager = new AccountManager();
    await manager.removeAccountById('account-id');

    expect(ipc.removeAccount).not.toHaveBeenCalled();
  });

  it('should handle errors when loading accounts', async () => {
    vi.mocked(ipc.listAccounts).mockRejectedValue(new Error('Failed to load'));

    const manager = new AccountManager();
    await manager.loadAccounts();

    // Should handle error gracefully
    expect(manager.getAllAccounts()).toEqual([]);
  });
});
