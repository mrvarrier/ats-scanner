import { test, expect } from '@playwright/test';

test.describe('User Preferences & Settings', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');
    await page.click('[data-testid="nav-settings"]');
    await expect(page.locator('[data-testid="settings-page"]')).toBeVisible();
  });

  test('should load settings page with all sections', async ({ page }) => {
    // Check that all settings sections are present
    await expect(page.locator('[data-testid="ollama-settings"]')).toBeVisible();
    await expect(
      page.locator('[data-testid="analysis-settings"]')
    ).toBeVisible();
    await expect(page.locator('[data-testid="ui-preferences"]')).toBeVisible();
    await expect(
      page.locator('[data-testid="performance-settings"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="privacy-settings"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="notification-settings"]')
    ).toBeVisible();
    await expect(page.locator('[data-testid="export-settings"]')).toBeVisible();
    await expect(
      page.locator('[data-testid="management-settings"]')
    ).toBeVisible();
  });

  test('should update Ollama connection settings', async ({ page }) => {
    // Mock preferences API
    await page.route('**/update_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: { message: 'Preferences updated successfully' },
        }),
      });
    });

    // Update Ollama host
    await page.fill(
      '[data-testid="ollama-host-input"]',
      'http://localhost:11435'
    );

    // Update port
    await page.fill('[data-testid="ollama-port-input"]', '11435');

    // Update timeout
    await page.fill('[data-testid="connection-timeout-input"]', '60');

    // Toggle auto-connect
    await page.click('[data-testid="auto-connect-toggle"]');

    // Save settings
    await page.click('[data-testid="save-ollama-settings"]');

    // Verify success toast
    await expect(page.locator('[data-testid="success-toast"]')).toBeVisible();
  });

  test('should test Ollama connection', async ({ page }) => {
    // Mock connection test
    await page.route('**/test_ollama_connection', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            connected: true,
            version: '0.1.17',
            models_count: 3,
          },
        }),
      });
    });

    await page.click('[data-testid="test-connection-button"]');

    // Should show loading state
    await expect(
      page.locator('[data-testid="connection-testing"]')
    ).toBeVisible();

    // Should show success result
    await expect(
      page.locator('[data-testid="connection-success"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="connection-result"]')
    ).toContainText('Connected');
  });

  test('should update theme preferences', async ({ page }) => {
    await page.route('**/update_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });

    // Change theme to Dark
    await page.selectOption('[data-testid="theme-select"]', 'Dark');

    // Toggle animations
    await page.click('[data-testid="animations-toggle"]');

    // Collapse sidebar setting
    await page.click('[data-testid="sidebar-collapsed-toggle"]');

    // Show advanced features
    await page.click('[data-testid="advanced-features-toggle"]');

    await page.click('[data-testid="save-ui-preferences"]');
    await expect(page.locator('[data-testid="success-toast"]')).toBeVisible();
  });

  test('should configure performance settings', async ({ page }) => {
    await page.route('**/update_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });

    // Update concurrent analyses
    await page.fill('[data-testid="max-concurrent-input"]', '5');

    // Update cache size
    await page.fill('[data-testid="cache-size-input"]', '512');

    // Enable GPU acceleration
    await page.click('[data-testid="gpu-acceleration-toggle"]');

    await page.click('[data-testid="save-performance-settings"]');
    await expect(page.locator('[data-testid="success-toast"]')).toBeVisible();
  });

  test('should configure notification preferences', async ({ page }) => {
    await page.route('**/update_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({ success: true }),
      });
    });

    // Enable desktop notifications
    await page.click('[data-testid="desktop-notifications-toggle"]');

    // Enable sound notifications
    await page.click('[data-testid="sound-notifications-toggle"]');

    // Configure email notifications
    await page.click('[data-testid="email-notifications-toggle"]');
    await page.fill(
      '[data-testid="notification-email-input"]',
      'user@example.com'
    );

    await page.click('[data-testid="save-notification-settings"]');
    await expect(page.locator('[data-testid="success-toast"]')).toBeVisible();
  });

  test('should export user preferences', async ({ page }) => {
    // Mock preferences data
    await page.route('**/export_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: JSON.stringify({
            ollama_host: 'http://localhost:11434',
            theme: 'Dark',
            auto_connect_on_startup: true,
          }),
        }),
      });
    });

    const downloadPromise = page.waitForEvent('download');
    await page.click('[data-testid="export-preferences-button"]');
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/preferences.*\.json$/);
  });

  test('should import user preferences', async ({ page }) => {
    await page.route('**/import_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            id: 'pref1',
            theme: 'Dark',
            ollama_host: 'http://localhost:11434',
          },
        }),
      });
    });

    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('[data-testid="import-preferences-button"]');
    const fileChooser = await fileChooserPromise;

    const preferencesData = {
      theme: 'Dark',
      ollama_host: 'http://localhost:11434',
      auto_connect_on_startup: true,
    };

    await fileChooser.setFiles({
      name: 'preferences.json',
      mimeType: 'application/json',
      buffer: Buffer.from(JSON.stringify(preferencesData)),
    });

    await expect(
      page.locator('[data-testid="import-success-toast"]')
    ).toBeVisible();
  });

  test('should reset preferences to defaults', async ({ page }) => {
    await page.route('**/reset_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            id: 'default',
            theme: 'System',
            ollama_host: 'http://localhost:11434',
            auto_connect_on_startup: true,
          },
        }),
      });
    });

    // Click reset button (should show confirmation)
    await page.click('[data-testid="reset-preferences-button"]');

    // Confirm reset in dialog
    await expect(
      page.locator('[data-testid="reset-confirmation-dialog"]')
    ).toBeVisible();
    await page.click('[data-testid="confirm-reset-button"]');

    // Should show success and reload settings
    await expect(
      page.locator('[data-testid="reset-success-toast"]')
    ).toBeVisible();
  });

  test('should validate form inputs', async ({ page }) => {
    // Test invalid port number
    await page.fill('[data-testid="ollama-port-input"]', 'invalid');
    await page.click('[data-testid="save-ollama-settings"]');

    await expect(
      page.locator('[data-testid="port-validation-error"]')
    ).toBeVisible();

    // Test invalid email format
    await page.click('[data-testid="email-notifications-toggle"]');
    await page.fill(
      '[data-testid="notification-email-input"]',
      'invalid-email'
    );
    await page.click('[data-testid="save-notification-settings"]');

    await expect(
      page.locator('[data-testid="email-validation-error"]')
    ).toBeVisible();

    // Test invalid cache size
    await page.fill('[data-testid="cache-size-input"]', '-100');
    await page.click('[data-testid="save-performance-settings"]');

    await expect(
      page.locator('[data-testid="cache-validation-error"]')
    ).toBeVisible();
  });

  test('should persist settings across page reloads', async ({ page }) => {
    // Mock getting saved preferences
    await page.route('**/get_user_preferences', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            theme: 'Dark',
            ollama_host: 'http://localhost:11435',
            auto_connect_on_startup: false,
            max_concurrent_analyses: 5,
          },
        }),
      });
    });

    // Reload the page
    await page.reload();
    await page.waitForSelector('[data-testid="settings-page"]');

    // Verify settings are loaded correctly
    await expect(page.locator('[data-testid="theme-select"]')).toHaveValue(
      'Dark'
    );
    await expect(page.locator('[data-testid="ollama-host-input"]')).toHaveValue(
      'http://localhost:11435'
    );
    await expect(
      page.locator('[data-testid="auto-connect-toggle"]')
    ).not.toBeChecked();
    await expect(
      page.locator('[data-testid="max-concurrent-input"]')
    ).toHaveValue('5');
  });
});
