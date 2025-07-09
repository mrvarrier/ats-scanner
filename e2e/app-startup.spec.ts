import { test, expect } from '@playwright/test';

test.describe('Application Startup', () => {
  test('should load the main application', async ({ page }) => {
    await page.goto('/');

    // Wait for the app to load
    await page.waitForSelector('[data-testid="main-layout"]', {
      timeout: 10000,
    });

    // Check that the main title is present
    await expect(page.locator('h1')).toContainText('ATS Scanner');

    // Verify sidebar navigation is present
    await expect(page.locator('[data-testid="sidebar"]')).toBeVisible();

    // Check that we're on the dashboard by default
    await expect(page.locator('[data-testid="dashboard-page"]')).toBeVisible();
  });

  test('should have working navigation', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');

    // Test navigation to Analysis page
    await page.click('[data-testid="nav-analysis"]');
    await expect(page.locator('[data-testid="analysis-page"]')).toBeVisible();

    // Test navigation to Optimization page
    await page.click('[data-testid="nav-optimization"]');
    await expect(
      page.locator('[data-testid="optimization-page"]')
    ).toBeVisible();

    // Test navigation to Batch Analysis page
    await page.click('[data-testid="nav-batch"]');
    await expect(
      page.locator('[data-testid="batch-analysis-page"]')
    ).toBeVisible();

    // Test navigation to Settings page
    await page.click('[data-testid="nav-settings"]');
    await expect(page.locator('[data-testid="settings-page"]')).toBeVisible();

    // Return to Dashboard
    await page.click('[data-testid="nav-dashboard"]');
    await expect(page.locator('[data-testid="dashboard-page"]')).toBeVisible();
  });

  test('should display connection status', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');

    // Check for Ollama connection status indicator
    const connectionStatus = page.locator('[data-testid="connection-status"]');
    await expect(connectionStatus).toBeVisible();

    // Should show either connected or disconnected state
    const statusText = await connectionStatus.textContent();
    expect(statusText).toMatch(/(Connected|Disconnected|Connecting)/);
  });

  test('should have dark mode toggle', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');

    // Find and click the theme toggle
    const themeToggle = page.locator('[data-testid="theme-toggle"]');
    await expect(themeToggle).toBeVisible();

    // Get initial theme state
    const initialTheme = await page.locator('html').getAttribute('class');

    // Toggle theme
    await themeToggle.click();

    // Wait a moment for theme to change
    await page.waitForTimeout(500);

    // Check that theme has changed
    const newTheme = await page.locator('html').getAttribute('class');
    expect(newTheme).not.toBe(initialTheme);
  });
});

test.describe('Error Handling', () => {
  test('should display error boundary when JS error occurs', async ({
    page,
  }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');

    // Inject an error to test error boundary
    await page.evaluate(() => {
      const errorButton = document.createElement('button');
      errorButton.onclick = () => {
        throw new Error('Test error for error boundary');
      };
      errorButton.click();
    });

    // Check if error boundary is displayed
    await expect(page.locator('[data-testid="error-boundary"]')).toBeVisible();
  });

  test('should handle network errors gracefully', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');

    // Mock network failure
    await page.route('**/test_ollama_connection', route => route.abort());

    // Try to trigger a connection test
    await page.click('[data-testid="nav-settings"]');
    const testConnectionButton = page.locator(
      '[data-testid="test-connection"]'
    );

    if (await testConnectionButton.isVisible()) {
      await testConnectionButton.click();

      // Should show an error message
      await expect(page.locator('[data-testid="error-toast"]')).toBeVisible();
    }
  });
});
