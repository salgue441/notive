/**
 * IPC communication with Rust backend.
 */

import { invoke } from '@tauri-apps/api/core';

import { listen } from '@tauri-apps/api/event';

/**
 * Sets up IPC event listeners and handlers.
 */
export async function setupIPC(): Promise<void> {
  console.log('[IPC] Setting up...');

  // Listen for download events from the webview
  // The webview handles downloads natively, but we can listen for events if needed
  
  // Listen for update events (if we emit them from backend)
  // This would be set up when we add event emission to the updater
}

/**
 * Minimizes the window to the system tray.
 */
export async function minimizeToTray(): Promise<void> {
  await invoke('minimize_to_tray');
}

/**
 * Restores the window from the system tray.
 */
export async function restoreFromTray(): Promise<void> {
  await invoke('restore_from_tray');
}

/**
 * Toggles fullscreen mode.
 */
export async function toggleFullscreen(): Promise<void> {
  await invoke('toggle_fullscreen');
}

/**
 * Sets the page zoom level.
 */
export async function setZoom(level: number): Promise<void> {
  await invoke('set_zoom', { level });
}

/**
 * Reloads the current page.
 */
export async function reloadPage(): Promise<void> {
  await invoke('reload_page');
}

/**
 * Gets the current user settings.
 */
export async function getSettings(): Promise<UserSettings> {
  return await invoke('get_settings');
}

/**
 * Updates user settings.
 */
export async function updateSettings(settings: UserSettings): Promise<void> {
  await invoke('update_settings', { settings });
}

/**
 * Handles a file download.
 */
export async function handleDownload(url: string, filename?: string): Promise<void> {
  await invoke('handle_download', { url, filename });
}

/**
 * Shows the about dialog.
 */
export async function showAbout(): Promise<void> {
  await invoke('show_about');
}

/**
 * Checks for updates manually.
 */
export async function checkUpdates(): Promise<boolean> {
  return await invoke('check_updates');
}

/**
 * Opens the settings window.
 */
export async function openSettingsWindow(): Promise<void> {
  await invoke('open_settings_window');
}

/**
 * Downloads a file with a native save dialog.
 */
export async function downloadWithDialog(url: string, suggestedFilename?: string): Promise<void> {
  await invoke('download_with_dialog', { url, suggestedFilename });
}

// ============================================================================
// Workspaces
// ============================================================================

/**
 * Creates a new workspace window.
 */
export async function createWorkspace(name: string, url?: string): Promise<string> {
  return await invoke('create_workspace', { name, url });
}

/**
 * Lists all available workspaces.
 */
export async function listWorkspaces(): Promise<Workspace[]> {
  return await invoke('list_workspaces');
}

/**
 * Switches to a different workspace.
 */
export async function switchWorkspace(workspaceId: string): Promise<void> {
  await invoke('switch_workspace', { workspaceId });
}

/**
 * Closes a workspace window.
 */
export async function closeWorkspace(workspaceId: string): Promise<void> {
  await invoke('close_workspace', { workspaceId });
}

// ============================================================================
// Tabs
// ============================================================================

/**
 * Opens a new tab in the specified window.
 */
export async function openTab(windowLabel: string, url: string, title?: string): Promise<string> {
  return await invoke('open_tab', { windowLabel, url, title });
}

/**
 * Closes a tab.
 */
export async function closeTab(windowLabel: string, tabId: string): Promise<void> {
  await invoke('close_tab', { windowLabel, tabId });
}

/**
 * Switches to a different tab.
 */
export async function switchTab(windowLabel: string, tabId: string): Promise<void> {
  await invoke('switch_tab', { windowLabel, tabId });
}

/**
 * Lists all tabs for a window.
 */
export async function listTabs(windowLabel: string): Promise<Tab[]> {
  return await invoke('list_tabs', { windowLabel });
}

// ============================================================================
// Calendar
// ============================================================================

/**
 * Syncs Notion calendar events to system calendar.
 */
export async function syncCalendar(events: CalendarEvent[]): Promise<number> {
  return await invoke('sync_calendar', { events });
}

/**
 * Exports a calendar event to system calendar.
 */
export async function exportEvent(event: CalendarEvent): Promise<void> {
  await invoke('export_event', { event });
}

/**
 * Checks if calendar integration is available.
 */
export async function checkCalendarAvailability(): Promise<boolean> {
  return await invoke('check_calendar_availability');
}

// ============================================================================
// Wayland
// ============================================================================

/**
 * Checks if running under Wayland.
 */
export async function isWayland(): Promise<boolean> {
  return await invoke('is_wayland');
}

/**
 * Gets Wayland-specific information.
 */
export async function getWaylandInfo(): Promise<WaylandInfo> {
  return await invoke('get_wayland_info');
}

// ============================================================================
// Accounts
// ============================================================================

/**
 * Initiates OAuth flow for a provider.
 */
export async function startOAuthFlow(provider: AccountProvider): Promise<string> {
  return await invoke('start_oauth_flow', { provider });
}

/**
 * Adds a new account.
 */
export async function addAccount(
  provider: AccountProvider,
  email: string,
  name?: string
): Promise<string> {
  return await invoke('add_account', { provider, email, name });
}

/**
 * Lists all accounts.
 */
export async function listAccounts(): Promise<Account[]> {
  return await invoke('list_accounts');
}

/**
 * Switches to a different account.
 */
export async function switchAccount(accountId: string): Promise<void> {
  await invoke('switch_account', { accountId });
}

/**
 * Removes an account.
 */
export async function removeAccount(accountId: string): Promise<void> {
  await invoke('remove_account', { accountId });
}

// ============================================================================
// Bookmarks
// ============================================================================

/**
 * Adds a bookmark.
 */
export async function addBookmark(title: string, url: string): Promise<string> {
  return await invoke('add_bookmark', { title, url });
}

/**
 * Removes a bookmark.
 */
export async function removeBookmark(bookmarkId: string): Promise<void> {
  await invoke('remove_bookmark', { bookmarkId });
}

/**
 * Lists all bookmarks.
 */
export async function listBookmarks(): Promise<Bookmark[]> {
  return await invoke('list_bookmarks');
}

/**
 * Searches bookmarks.
 */
export async function searchBookmarks(query: string): Promise<Bookmark[]> {
  return await invoke('search_bookmarks', { query });
}

/**
 * Gets a bookmark by ID.
 */
export async function getBookmark(bookmarkId: string): Promise<Bookmark | null> {
  return await invoke('get_bookmark', { bookmarkId });
}

// ============================================================================
// History
// ============================================================================

/**
 * Records a page visit.
 */
export async function recordPageVisit(title: string, url: string): Promise<void> {
  await invoke('record_page_visit', { title, url });
}

/**
 * Gets recent pages.
 */
export async function getRecentPages(limit?: number): Promise<HistoryEntry[]> {
  return await invoke('get_recent_pages', { limit });
}

/**
 * Clears page history.
 */
export async function clearHistory(): Promise<void> {
  await invoke('clear_history');
}

/**
 * Removes a history entry.
 */
export async function removeHistoryEntry(entryId: string): Promise<void> {
  await invoke('remove_history_entry', { entryId });
}

/**
 * User settings interface.
 */
export interface UserSettings {
  start_minimized: boolean;
  minimize_to_tray: boolean;
  close_to_tray: boolean;
  zoom_level: number;
  custom_css_enabled: boolean;
  custom_css: string;
  notifications_enabled: boolean;
  notification_sound: boolean;
  shortcuts: ShortcutSettings;
  auto_update: boolean;
  update_channel: 'stable' | 'beta' | 'nightly';
  autostart_enabled: boolean;
  hardware_acceleration: boolean;
  spellcheck: boolean;
  theme: 'system' | 'light' | 'dark';
}

/**
 * Shortcut settings interface.
 */
export interface ShortcutSettings {
  toggle_window: string;
  quick_capture: string;
  reload: string;
  zoom_in: string;
  zoom_out: string;
  zoom_reset: string;
}

// ============================================================================
// Workspaces
// ============================================================================

/**
 * Workspace interface.
 */
export interface Workspace {
  id: string;
  name: string;
  url: string;
  zoom_level: number;
}

// ============================================================================
// Tabs
// ============================================================================

/**
 * Tab interface.
 */
export interface Tab {
  id: string;
  title: string;
  url: string;
  window_label: string;
}

// ============================================================================
// Calendar
// ============================================================================

/**
 * Calendar event interface.
 */
export interface CalendarEvent {
  id: string;
  title: string;
  description?: string;
  start_time: string; // ISO 8601 format
  end_time?: string; // ISO 8601 format
  location?: string;
}

// ============================================================================
// Wayland
// ============================================================================

/**
 * Wayland information interface.
 */
export interface WaylandInfo {
  is_wayland: boolean;
  wayland_display?: string;
  xdg_session_type?: string;
  gdk_backend?: string;
  message?: string;
}

// ============================================================================
// Accounts
// ============================================================================

/**
 * Account provider types.
 */
export type AccountProvider = 'Google' | 'Apple' | 'Microsoft' | 'Email';

/**
 * Account interface.
 */
export interface Account {
  id: string;
  provider: AccountProvider;
  email: string;
  name?: string;
  is_active: boolean;
}

// ============================================================================
// Bookmarks
// ============================================================================

/**
 * Bookmark interface.
 */
export interface Bookmark {
  id: string;
  title: string;
  url: string;
  icon?: string;
  created_at: string;
  tags: string[];
}

// ============================================================================
// History
// ============================================================================

/**
 * History entry interface.
 */
export interface HistoryEntry {
  id: string;
  title: string;
  url: string;
  visited_at: string;
  visit_count: number;
}

// ============================================================================
// Search
// ============================================================================

/**
 * Search result interface.
 */
export interface SearchResult {
  id: string;
  title: string;
  url: string;
  snippet?: string;
  source: 'Bookmark' | 'History' | 'Workspace' | 'Tab';
  relevance: number;
}

// ============================================================================
// Privacy
// ============================================================================

/**
 * Privacy settings interface.
 */
export interface PrivacySettings {
  privacy_mode_enabled: boolean;
  clear_history_on_close: boolean;
  clear_cookies_on_close: boolean;
  no_history_tracking: boolean;
}

/**
 * Gets privacy settings.
 */
export async function getPrivacySettings(): Promise<PrivacySettings> {
  return invoke('get_privacy_settings');
}

/**
 * Updates privacy settings.
 */
export async function updatePrivacySettings(settings: PrivacySettings): Promise<void> {
  return invoke('update_privacy_settings', { settings });
}

/**
 * Clears all privacy-sensitive data.
 */
export async function clearPrivacyData(): Promise<void> {
  return invoke('clear_privacy_data');
}

/**
 * Checks if privacy mode is enabled.
 */
export async function isPrivacyModeEnabled(): Promise<boolean> {
  return invoke('is_privacy_mode_enabled');
}

// ============================================================================
// Quick Capture
// ============================================================================

/**
 * Capture template interface.
 */
export interface CaptureTemplate {
  id: string;
  name: string;
  url: string;
  description?: string;
}

/**
 * Opens quick capture with optional template and tags.
 */
export async function openQuickCapture(
  templateId?: string,
  tags?: string[],
): Promise<string> {
  return invoke('open_quick_capture', { template_id: templateId, tags });
}

/**
 * Lists available capture templates.
 */
export async function listCaptureTemplates(): Promise<CaptureTemplate[]> {
  return invoke('list_capture_templates');
}

/**
 * Adds a new capture template.
 */
export async function addCaptureTemplate(
  name: string,
  url: string,
  description?: string,
): Promise<string> {
  return invoke('add_capture_template', { name, url, description });
}

/**
 * Removes a capture template.
 */
export async function removeCaptureTemplate(templateId: string): Promise<void> {
  return invoke('remove_capture_template', { template_id: templateId });
}

// ============================================================================
// Notification Customization
// ============================================================================

/**
 * Notification sound type.
 */
export type NotificationSound = 'None' | 'Default' | { Custom: string };

/**
 * Notification template interface.
 */
export interface NotificationTemplate {
  id: string;
  name: string;
  title: string;
  body: string;
  sound: NotificationSound;
}

/**
 * Notification settings interface.
 */
export interface NotificationSettings {
  default_sound: NotificationSound;
  templates: NotificationTemplate[];
  enable_scheduling: boolean;
  quiet_hours_start?: string; // HH:MM format
  quiet_hours_end?: string;   // HH:MM format
}

/**
 * Gets notification customization settings.
 */
export async function getNotificationSettings(): Promise<NotificationSettings> {
  return invoke('get_notification_settings');
}

/**
 * Updates notification customization settings.
 */
export async function updateNotificationSettings(
  settings: NotificationSettings,
): Promise<void> {
  return invoke('update_notification_settings', { settings });
}

/**
 * Checks if notifications should be shown (respects quiet hours).
 */
export async function shouldShowNotification(): Promise<boolean> {
  return invoke('should_show_notification');
}

/**
 * Gets notification template by ID.
 */
export async function getNotificationTemplate(
  templateId: string,
): Promise<NotificationTemplate | null> {
  return invoke('get_notification_template', { template_id: templateId });
}

// ============================================================================
// Tab Persistence
// ============================================================================

/**
 * Restores tabs for a window on startup.
 */
export async function restoreTabs(windowLabel: string): Promise<Tab[]> {
  return invoke('restore_tabs', { window_label: windowLabel });
}

// ============================================================================
// Settings Sync
// ============================================================================

/**
 * Exports settings to a JSON file.
 */
export async function exportSettings(): Promise<string> {
  return invoke('export_settings');
}

/**
 * Imports settings from a JSON file.
 */
export async function importSettings(): Promise<string> {
  return invoke('import_settings');
}

/**
 * Gets settings as JSON string.
 */
export async function getSettingsJson(): Promise<string> {
  return invoke('get_settings_json');
}

/**
 * Restores settings from JSON string.
 */
export async function restoreSettingsJson(json: string): Promise<void> {
  return invoke('restore_settings_json', { json });
}

// ============================================================================
// Sessions
// ============================================================================

export interface Session {
  id: string;
  account_id: string;
  account_email: string;
  created_at: number;
  last_used: number;
  expires_at?: number;
  is_active: boolean;
}

export async function createSession(
  accountId: string,
  accountEmail: string,
  expiresInSeconds?: number,
): Promise<string> {
  return invoke('create_session', { account_id: accountId, account_email: accountEmail, expires_in_seconds: expiresInSeconds });
}

export async function listSessions(): Promise<Session[]> {
  return invoke('list_sessions');
}

export async function getActiveSession(): Promise<Session | null> {
  return invoke('get_active_session');
}

export async function setActiveSession(sessionId: string): Promise<void> {
  return invoke('set_active_session', { session_id: sessionId });
}

export async function removeSession(sessionId: string): Promise<void> {
  return invoke('remove_session', { session_id: sessionId });
}

export async function cleanupExpiredSessions(): Promise<number> {
  return invoke('cleanup_expired_sessions');
}

// ============================================================================
// Templates
// ============================================================================

export interface PageTemplate {
  id: string;
  name: string;
  description?: string;
  url: string;
  category?: string;
  tags: string[];
  icon?: string;
  created_at: string;
  usage_count: number;
}

export async function listTemplates(): Promise<PageTemplate[]> {
  return invoke('list_templates');
}

export async function getTemplate(templateId: string): Promise<PageTemplate | null> {
  return invoke('get_template', { template_id: templateId });
}

export async function createTemplate(
  name: string,
  url: string,
  description?: string,
  category?: string,
): Promise<string> {
  return invoke('create_template', { name, url, description, category });
}

export async function updateTemplate(
  templateId: string,
  name?: string,
  url?: string,
  description?: string,
  category?: string,
): Promise<void> {
  return invoke('update_template', { template_id: templateId, name, url, description, category });
}

export async function deleteTemplate(templateId: string): Promise<void> {
  return invoke('delete_template', { template_id: templateId });
}

export async function useTemplate(templateId: string): Promise<string> {
  return invoke('use_template', { template_id: templateId });
}

export async function searchTemplates(query: string): Promise<PageTemplate[]> {
  return invoke('search_templates', { query });
}

// ============================================================================
// Analytics
// ============================================================================

export interface UsageStats {
  total_views: number;
  total_edits: number;
  total_pages: number;
  most_viewed_pages: Array<[string, number]>;
  recent_activity: ActivityEntry[];
}

export interface ActivityEntry {
  id: string;
  page_id: string;
  page_url: string;
  page_title: string;
  activity_type: string;
  timestamp: number;
  metadata?: any;
}

export async function recordPageView(
  pageId: string,
  pageUrl: string,
  pageTitle: string,
): Promise<void> {
  return invoke('record_page_view', { page_id: pageId, page_url: pageUrl, page_title: pageTitle });
}

export async function recordPageEdit(
  pageId: string,
  pageUrl: string,
  editType: string,
): Promise<void> {
  return invoke('record_page_edit', { page_id: pageId, page_url: pageUrl, edit_type: editType });
}

export async function getUsageStats(): Promise<UsageStats> {
  return invoke('get_usage_stats');
}

export async function getActivityTimeline(limit?: number): Promise<ActivityEntry[]> {
  return invoke('get_activity_timeline', { limit });
}

export async function clearAnalytics(): Promise<void> {
  return invoke('clear_analytics');
}

// ============================================================================
// Performance
// ============================================================================

export interface ResourceUsage {
  memory_mb: number;
  cpu_percent: number;
  timestamp: number;
}

export interface PerformanceMetrics {
  memory_usage: ResourceUsage[];
  cpu_usage: ResourceUsage[];
  average_memory_mb: number;
  average_cpu_percent: number;
  peak_memory_mb: number;
  peak_cpu_percent: number;
}

export async function getResourceUsage(): Promise<ResourceUsage> {
  return invoke('get_resource_usage');
}

export async function getPerformanceMetrics(): Promise<PerformanceMetrics> {
  return invoke('get_performance_metrics');
}

export async function clearPerformanceMetrics(): Promise<void> {
  return invoke('clear_performance_metrics');
}

// ============================================================================
// Cache
// ============================================================================

export interface CacheStats {
  total_entries: number;
  total_size_mb: number;
  entries_by_type: Record<string, number>;
}

export async function cacheResource(
  key: string,
  url: string,
  cacheType: string,
  expiresInSeconds?: number,
): Promise<void> {
  return invoke('cache_resource', { key, url, cache_type: cacheType, expires_in_seconds: expiresInSeconds });
}

export async function getCachedResource(key: string): Promise<string | null> {
  return invoke('get_cached_resource', { key });
}

export async function clearCache(cacheType?: string): Promise<number> {
  return invoke('clear_cache', { cache_type: cacheType });
}

export async function getCacheStats(): Promise<CacheStats> {
  return invoke('get_cache_stats');
}

export async function preloadResources(urls: string[]): Promise<number> {
  return invoke('preload_resources', { urls });
}

// ============================================================================
// Monitor
// ============================================================================

export interface MonitorInfo {
  id: string;
  name: string;
  position: [number, number];
  size: [number, number];
  scale_factor: number;
  is_primary: boolean;
}

export async function getMonitors(): Promise<MonitorInfo[]> {
  return invoke('get_monitors');
}

export async function getPrimaryMonitor(): Promise<MonitorInfo> {
  return invoke('get_primary_monitor');
}

export async function saveWindowPlacement(
  windowLabel: string,
  monitorId?: string,
  position?: [number, number],
  size?: [number, number],
): Promise<void> {
  return invoke('save_window_placement', { window_label: windowLabel, monitor_id: monitorId, position, size });
}

export async function restoreWindowPlacement(windowLabel: string): Promise<void> {
  return invoke('restore_window_placement', { window_label: windowLabel });
}

export async function moveWindowToMonitor(windowLabel: string, monitorId: string): Promise<void> {
  return invoke('move_window_to_monitor', { window_label: windowLabel, monitor_id: monitorId });
}

// ============================================================================
// Notion API
// ============================================================================

export interface NotionPageMetadata {
  id: string;
  url: string;
  title: string;
  created_time: string;
  last_edited_time: string;
  created_by?: string;
  last_edited_by?: string;
}

export async function setNotionApiKey(apiKey: string): Promise<void> {
  return invoke('set_notion_api_key', { api_key: apiKey });
}

export async function getPageMetadata(pageId: string): Promise<NotionPageMetadata> {
  return invoke('get_page_metadata', { page_id: pageId });
}

export async function hasNotionApiKey(): Promise<boolean> {
  return invoke('has_notion_api_key');
}

// ============================================================================
// OAuth
// ============================================================================

export interface OAuthToken {
  access_token: string;
  refresh_token?: string;
  expires_in?: number;
  token_type: string;
  scope?: string;
}

export async function startOAuthFlow(providerName: string): Promise<string> {
  return invoke('start_oauth_flow', { provider_name: providerName });
}

export async function handleOAuthCallback(
  providerName: string,
  code: string,
  state: string,
): Promise<OAuthToken> {
  return invoke('handle_oauth_callback', { provider_name: providerName, code, state });
}

export async function refreshOAuthToken(providerName: string): Promise<OAuthToken> {
  return invoke('refresh_oauth_token', { provider_name: providerName });
}

// ============================================================================
// Offline Mode
// ============================================================================

export interface OfflinePage {
  url: string;
  title: string;
  content: string;
  cached_at: number;
  last_synced?: number;
}

export interface OfflineStatus {
  is_offline: boolean;
  cached_pages: number;
  last_sync?: number;
}

export async function enableOfflineMode(): Promise<void> {
  return invoke('enable_offline_mode');
}

export async function disableOfflineMode(): Promise<void> {
  return invoke('disable_offline_mode');
}

export async function cachePageForOffline(
  url: string,
  title: string,
  content: string,
): Promise<void> {
  return invoke('cache_page_for_offline', { url, title, content });
}

export async function getCachedPage(url: string): Promise<OfflinePage | null> {
  return invoke('get_cached_page', { url });
}

export async function syncOfflineChanges(): Promise<number> {
  return invoke('sync_offline_changes');
}

export async function getOfflineStatus(): Promise<OfflineStatus> {
  return invoke('get_offline_status');
}

export async function clearOfflineCache(): Promise<void> {
  return invoke('clear_offline_cache');
}

// ============================================================================
// Plugins
// ============================================================================

export interface PluginMetadata {
  id: string;
  name: string;
  version: string;
  description?: string;
  author?: string;
  enabled: boolean;
}

export async function listPlugins(): Promise<PluginMetadata[]> {
  return invoke('list_plugins');
}

export async function loadPlugin(pluginPath: string): Promise<PluginMetadata> {
  return invoke('load_plugin', { plugin_path: pluginPath });
}

export async function enablePlugin(pluginId: string): Promise<void> {
  return invoke('enable_plugin', { plugin_id: pluginId });
}

export async function disablePlugin(pluginId: string): Promise<void> {
  return invoke('disable_plugin', { plugin_id: pluginId });
}

export async function uninstallPlugin(pluginId: string): Promise<void> {
  return invoke('uninstall_plugin', { plugin_id: pluginId });
}

// ============================================================================
// AI Integration
// ============================================================================

export interface AISearchResult {
  title: string;
  url: string;
  snippet: string;
  relevance_score: number;
  ai_summary?: string;
}

export async function aiSearch(query: string): Promise<AISearchResult[]> {
  return invoke('ai_search', { query });
}

export async function getAISuggestions(context: string): Promise<string[]> {
  return invoke('get_ai_suggestions', { context });
}

export async function getAIAutocomplete(partialText: string): Promise<string[]> {
  return invoke('get_ai_autocomplete', { partial_text: partialText });
}

export async function generateSmartTemplates(userContext: string): Promise<PageTemplate[]> {
  return invoke('generate_smart_templates', { user_context: userContext });
}
