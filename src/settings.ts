/**
 * Settings window logic.
 */

import {
  getSettings,
  updateSettings,
  type UserSettings,
  getPrivacySettings,
  updatePrivacySettings,
  clearPrivacyData,
  listCaptureTemplates,
  addCaptureTemplate,
  removeCaptureTemplate,
  getNotificationSettings,
  updateNotificationSettings,
  exportSettings,
  importSettings,
  type CaptureTemplate,
  type NotificationSettings,
  type PrivacySettings,
} from './lib/ipc';

// Load settings on window load
window.addEventListener('DOMContentLoaded', async () => {
  try {
    const settings = await getSettings();
    populateForm(settings);
    
    // Load additional settings
    await loadPrivacySettings();
    await loadQuickCaptureSettings();
    await loadNotificationSettings();
    
    setupEventListeners();
  } catch (error) {
    console.error('Failed to load settings:', error);
    alert('Failed to load settings. Please try again.');
  }
});

function populateForm(settings: UserSettings): void {
  // Window behavior
  (document.getElementById('start-minimized') as HTMLInputElement).checked = settings.start_minimized;
  (document.getElementById('minimize-to-tray') as HTMLInputElement).checked = settings.minimize_to_tray;
  (document.getElementById('close-to-tray') as HTMLInputElement).checked = settings.close_to_tray;

  // Appearance
  (document.getElementById('theme') as HTMLSelectElement).value = settings.theme;
  
  const zoomSlider = document.getElementById('zoom-level') as HTMLInputElement;
  const zoomValue = document.getElementById('zoom-value') as HTMLSpanElement;
  zoomSlider.value = settings.zoom_level.toString();
  zoomValue.textContent = `${Math.round(settings.zoom_level * 100)}%`;
  zoomSlider.addEventListener('input', () => {
    zoomValue.textContent = `${Math.round(parseFloat(zoomSlider.value) * 100)}%`;
  });

  (document.getElementById('custom-css-enabled') as HTMLInputElement).checked = settings.custom_css_enabled;
  (document.getElementById('custom-css') as HTMLTextAreaElement).value = settings.custom_css;

  // Notifications
  (document.getElementById('notifications-enabled') as HTMLInputElement).checked = settings.notifications_enabled;
  (document.getElementById('notification-sound') as HTMLInputElement).checked = settings.notification_sound;

  // Updates
  (document.getElementById('auto-update') as HTMLInputElement).checked = settings.auto_update;
  (document.getElementById('update-channel') as HTMLSelectElement).value = settings.update_channel;

  // System
  (document.getElementById('autostart-enabled') as HTMLInputElement).checked = settings.autostart_enabled;
  (document.getElementById('hardware-acceleration') as HTMLInputElement).checked = settings.hardware_acceleration;
  (document.getElementById('spellcheck') as HTMLInputElement).checked = settings.spellcheck;

  // Shortcuts
  (document.getElementById('shortcut-toggle-window') as HTMLInputElement).value = settings.shortcuts.toggle_window;
  (document.getElementById('shortcut-quick-capture') as HTMLInputElement).value = settings.shortcuts.quick_capture;
  (document.getElementById('shortcut-reload') as HTMLInputElement).value = settings.shortcuts.reload;
  (document.getElementById('shortcut-zoom-in') as HTMLInputElement).value = settings.shortcuts.zoom_in;
  (document.getElementById('shortcut-zoom-out') as HTMLInputElement).value = settings.shortcuts.zoom_out;
  (document.getElementById('shortcut-zoom-reset') as HTMLInputElement).value = settings.shortcuts.zoom_reset;
}

function setupEventListeners(): void {
  const saveBtn = document.getElementById('save-btn') as HTMLButtonElement;
  const cancelBtn = document.getElementById('cancel-btn') as HTMLButtonElement;
  const exportBtn = document.getElementById('export-settings-btn') as HTMLButtonElement;
  const importBtn = document.getElementById('import-settings-btn') as HTMLButtonElement;
  const clearPrivacyBtn = document.getElementById('clear-privacy-data-btn') as HTMLButtonElement;
  const addTemplateBtn = document.getElementById('add-template-btn') as HTMLButtonElement;
  const notificationScheduling = document.getElementById('notification-enable-scheduling') as HTMLInputElement;
  const notificationSound = document.getElementById('notification-default-sound') as HTMLSelectElement;
  const sidebarWidth = document.getElementById('sidebar-width') as HTMLInputElement;
  const sidebarWidthValue = document.getElementById('sidebar-width-value') as HTMLSpanElement;

  saveBtn.addEventListener('click', async () => {
    try {
      const settings = collectFormData();
      await updateSettings(settings);
      alert('Settings saved successfully!');
      window.close();
    } catch (error) {
      console.error('Failed to save settings:', error);
      alert('Failed to save settings. Please try again.');
    }
  });

  cancelBtn.addEventListener('click', () => {
    window.close();
  });

  exportBtn.addEventListener('click', async () => {
    try {
      const message = await exportSettings();
      alert(message);
    } catch (error) {
      console.error('Failed to export settings:', error);
      alert('Failed to export settings. Please try again.');
    }
  });

  importBtn.addEventListener('click', async () => {
    try {
      const message = await importSettings();
      alert(message);
      // Reload settings after import
      const settings = await getSettings();
      populateForm(settings);
    } catch (error) {
      console.error('Failed to import settings:', error);
      alert('Failed to import settings. Please try again.');
    }
  });

  clearPrivacyBtn.addEventListener('click', async () => {
    if (confirm('Are you sure you want to clear all privacy data? This cannot be undone.')) {
      try {
        await clearPrivacyData();
        alert('Privacy data cleared successfully.');
      } catch (error) {
        console.error('Failed to clear privacy data:', error);
        alert('Failed to clear privacy data. Please try again.');
      }
    }
  });

  addTemplateBtn.addEventListener('click', () => {
    const name = prompt('Template name:');
    if (!name) return;
    
    const url = prompt('Template URL (e.g., https://www.notion.so/new):');
    if (!url) return;
    
    const description = prompt('Description (optional):');
    
    addCaptureTemplate(name, url, description || undefined)
      .then(() => {
        alert('Template added successfully!');
        loadQuickCaptureSettings();
      })
      .catch((error) => {
        console.error('Failed to add template:', error);
        alert('Failed to add template. Please try again.');
      });
  });

  // Remove template buttons
  document.addEventListener('click', async (e) => {
    const target = e.target as HTMLElement;
    if (target.classList.contains('remove-template-btn')) {
      const templateId = target.dataset.templateId;
      if (templateId && confirm('Remove this template?')) {
        try {
          await removeCaptureTemplate(templateId);
          await loadQuickCaptureSettings();
        } catch (error) {
          console.error('Failed to remove template:', error);
          alert('Failed to remove template. Please try again.');
        }
      }
    }
  });

  notificationScheduling.addEventListener('change', (e) => {
    updateQuietHoursVisibility((e.target as HTMLInputElement).checked);
  });

  notificationSound.addEventListener('change', (e) => {
    const value = (e.target as HTMLSelectElement).value;
    const customSoundItem = document.getElementById('notification-custom-sound-item');
    if (customSoundItem) {
      customSoundItem.style.display = value === 'Custom' ? 'block' : 'none';
    }
  });

  sidebarWidth.addEventListener('input', (e) => {
    const value = (e.target as HTMLInputElement).value;
    if (sidebarWidthValue) {
      sidebarWidthValue.textContent = `${value}px`;
    }
  });
}

function collectFormData(): UserSettings {
  return {
    start_minimized: (document.getElementById('start-minimized') as HTMLInputElement).checked,
    minimize_to_tray: (document.getElementById('minimize-to-tray') as HTMLInputElement).checked,
    close_to_tray: (document.getElementById('close-to-tray') as HTMLInputElement).checked,
    zoom_level: parseFloat((document.getElementById('zoom-level') as HTMLInputElement).value),
    theme: (document.getElementById('theme') as HTMLSelectElement).value as 'system' | 'light' | 'dark',
    custom_css_enabled: (document.getElementById('custom-css-enabled') as HTMLInputElement).checked,
    custom_css: (document.getElementById('custom-css') as HTMLTextAreaElement).value,
    notifications_enabled: (document.getElementById('notifications-enabled') as HTMLInputElement).checked,
    notification_sound: (document.getElementById('notification-sound') as HTMLInputElement).checked,
    shortcuts: {
      toggle_window: (document.getElementById('shortcut-toggle-window') as HTMLInputElement).value || 'CommandOrControl+Shift+N',
      quick_capture: (document.getElementById('shortcut-quick-capture') as HTMLInputElement).value || 'CommandOrControl+Shift+C',
      reload: (document.getElementById('shortcut-reload') as HTMLInputElement).value || 'CommandOrControl+R',
      zoom_in: (document.getElementById('shortcut-zoom-in') as HTMLInputElement).value || 'CommandOrControl+=',
      zoom_out: (document.getElementById('shortcut-zoom-out') as HTMLInputElement).value || 'CommandOrControl+-',
      zoom_reset: (document.getElementById('shortcut-zoom-reset') as HTMLInputElement).value || 'CommandOrControl+0',
    },
    auto_update: (document.getElementById('auto-update') as HTMLInputElement).checked,
    update_channel: (document.getElementById('update-channel') as HTMLSelectElement).value as 'stable' | 'beta' | 'nightly',
    autostart_enabled: (document.getElementById('autostart-enabled') as HTMLInputElement).checked,
    hardware_acceleration: (document.getElementById('hardware-acceleration') as HTMLInputElement).checked,
    spellcheck: (document.getElementById('spellcheck') as HTMLInputElement).checked,
  };
}
