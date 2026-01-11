/**
 * Account management UI.
 */

import {
  listAccounts,
  addAccount,
  removeAccount,
  switchAccount,
  startOAuthFlow,
  type Account,
  type AccountProvider,
} from './ipc';

/**
 * Account manager UI component.
 */
export class AccountManager {
  private accounts: Account[] = [];

  /**
   * Initializes account management.
   */
  async init(): Promise<void> {
    await this.loadAccounts();
  }

  /**
   * Loads accounts from backend.
   */
  async loadAccounts(): Promise<void> {
    try {
      this.accounts = await listAccounts();
    } catch (error) {
      console.error('Failed to load accounts:', error);
    }
  }

  /**
   * Shows account switcher menu.
   */
  async showAccountSwitcher(): Promise<Account | null> {
    // Create a simple account switcher
    const accounts = await listAccounts();
    
    if (accounts.length === 0) {
      const add = confirm('No accounts found. Would you like to add one?');
      if (add) {
        await this.showAddAccountDialog();
      }
      return null;
    }

    // Simple selection (in a full implementation, this would be a proper UI)
    const accountList = accounts.map((a, i) => `${i + 1}. ${a.email} (${a.provider})`).join('\n');
    const selection = prompt(`Select account:\n${accountList}\n\nEnter number or 'add' for new account:`);
    
    if (selection === 'add') {
      await this.showAddAccountDialog();
      return null;
    }

    const index = parseInt(selection || '0', 10) - 1;
    if (index >= 0 && index < accounts.length) {
      return accounts[index];
    }

    return null;
  }

  /**
   * Shows add account dialog.
   */
  async showAddAccountDialog(): Promise<void> {
    const provider = prompt('Enter provider (Google, Apple, Microsoft, Email):') as AccountProvider | null;
    if (!provider) return;

    const email = prompt('Enter email:');
    if (!email) return;

    const name = prompt('Enter name (optional):') || undefined;

    try {
      if (provider !== 'Email') {
        // Start OAuth flow
        await startOAuthFlow(provider);
      } else {
        // Add email account
        await addAccount(provider, email, name);
        await this.loadAccounts();
      }
    } catch (error) {
      console.error('Failed to add account:', error);
      alert(`Failed to add account: ${error}`);
    }
  }

  /**
   * Switches to an account.
   */
  async switchToAccount(accountId: string): Promise<void> {
    try {
      await switchAccount(accountId);
      await this.loadAccounts();
    } catch (error) {
      console.error('Failed to switch account:', error);
      alert(`Failed to switch account: ${error}`);
    }
  }

  /**
   * Removes an account.
   */
  async removeAccountById(accountId: string): Promise<void> {
    if (!confirm('Are you sure you want to remove this account?')) {
      return;
    }

    try {
      await removeAccount(accountId);
      await this.loadAccounts();
    } catch (error) {
      console.error('Failed to remove account:', error);
      alert(`Failed to remove account: ${error}`);
    }
  }

  /**
   * Gets current active account.
   */
  getActiveAccount(): Account | null {
    return this.accounts.find((a) => a.is_active) || null;
  }

  /**
   * Gets all accounts.
   */
  getAllAccounts(): Account[] {
    return this.accounts;
  }
}

/**
 * Global account manager instance.
 */
let accountManagerInstance: AccountManager | null = null;

/**
 * Gets or creates the account manager instance.
 */
export function getAccountManager(): AccountManager {
  if (!accountManagerInstance) {
    accountManagerInstance = new AccountManager();
  }
  return accountManagerInstance;
}
