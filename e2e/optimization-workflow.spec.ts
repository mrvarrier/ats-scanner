import { test, expect } from '@playwright/test';

test.describe('Resume Optimization Workflow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');
    await page.click('[data-testid="nav-optimization"]');
    await expect(
      page.locator('[data-testid="optimization-page"]')
    ).toBeVisible();
  });

  test('should load optimization workspace', async ({ page }) => {
    // Check that all main components are present
    await expect(
      page.locator('[data-testid="original-resume-editor"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="optimized-resume-editor"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="optimization-controls"]')
    ).toBeVisible();
  });

  test('should allow resume content input', async ({ page }) => {
    const resumeContent = `
John Doe
Software Engineer

EXPERIENCE
Software Developer at ABC Corp (2020-2024)
- Built web applications
- Worked with team

SKILLS
- JavaScript
- React
- Python
    `;

    const originalEditor = page.locator(
      '[data-testid="original-resume-editor"]'
    );
    await originalEditor.fill(resumeContent);

    // Verify content was entered
    await expect(originalEditor).toHaveValue(resumeContent);
  });

  test('should allow job description input', async ({ page }) => {
    const jobDescription = `
Senior Full-Stack Developer position requiring:
- 5+ years JavaScript experience
- React and TypeScript proficiency
- Node.js backend development
- AWS cloud experience
- Strong communication skills
    `;

    const jobInput = page.locator('[data-testid="job-description-input"]');
    await jobInput.fill(jobDescription);

    await expect(jobInput).toHaveValue(jobDescription);
  });

  test('should perform optimization with different levels', async ({
    page,
  }) => {
    // Mock optimization API
    await page.route('**/optimize_resume', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            optimized_content: 'Optimized resume content with better keywords',
            changes_made: [
              {
                section: 'Skills',
                change_type: 'Addition',
                original: 'JavaScript',
                optimized: 'JavaScript, TypeScript',
                impact_score: 0.8,
              },
            ],
            before_score: 75,
            after_score: 88,
            improvement_percentage: 17.3,
          },
        }),
      });
    });

    // Fill in required fields
    await page.fill(
      '[data-testid="original-resume-editor"]',
      'Original resume content'
    );
    await page.fill(
      '[data-testid="job-description-input"]',
      'Job requirements'
    );

    // Test Conservative optimization
    await page.selectOption(
      '[data-testid="optimization-level"]',
      'Conservative'
    );
    await page.click('[data-testid="optimize-button"]');

    // Wait for optimization to complete
    await expect(
      page.locator('[data-testid="optimization-loading"]')
    ).toBeVisible();
    await expect(page.locator('[data-testid="optimized-content"]')).toBeVisible(
      { timeout: 15000 }
    );

    // Verify results
    await expect(page.locator('[data-testid="before-score"]')).toContainText(
      '75'
    );
    await expect(page.locator('[data-testid="after-score"]')).toContainText(
      '88'
    );
    await expect(
      page.locator('[data-testid="improvement-percentage"]')
    ).toContainText('17.3%');
  });

  test('should show real-time analysis when enabled', async ({ page }) => {
    // Mock real-time analysis
    await page.route('**/analyze_resume', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            overall_score: 82,
            category_scores: {
              skills: 85,
              experience: 78,
              education: 80,
              keywords: 85,
              format: 90,
            },
            detailed_feedback: 'Good resume structure',
            missing_keywords: ['TypeScript', 'AWS'],
            recommendations: ['Add cloud experience'],
            processing_time_ms: 800,
          },
        }),
      });
    });

    // Enable real-time analysis
    await page.check('[data-testid="real-time-toggle"]');

    // Type in the resume editor
    await page.fill(
      '[data-testid="original-resume-editor"]',
      'John Doe\nSoftware Engineer'
    );
    await page.fill(
      '[data-testid="job-description-input"]',
      'Looking for a developer'
    );

    // Wait for real-time analysis to trigger
    await expect(page.locator('[data-testid="live-score"]')).toBeVisible({
      timeout: 5000,
    });
    await expect(page.locator('[data-testid="live-score"]')).toContainText(
      '82'
    );
  });

  test('should display change tracking', async ({ page }) => {
    // Mock optimization with detailed changes
    await page.route('**/optimize_resume', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            optimized_content: 'Enhanced resume content',
            changes_made: [
              {
                section: 'Skills',
                change_type: 'Addition',
                original: 'JavaScript',
                optimized: 'JavaScript, TypeScript, React',
                impact_score: 0.9,
              },
              {
                section: 'Experience',
                change_type: 'Enhancement',
                original: 'Built applications',
                optimized:
                  'Architected and developed scalable web applications',
                impact_score: 0.7,
              },
            ],
            before_score: 70,
            after_score: 89,
            improvement_percentage: 27.1,
          },
        }),
      });
    });

    await page.fill('[data-testid="original-resume-editor"]', 'Basic resume');
    await page.fill('[data-testid="job-description-input"]', 'Tech job');
    await page.click('[data-testid="optimize-button"]');

    await expect(page.locator('[data-testid="changes-tracking"]')).toBeVisible({
      timeout: 15000,
    });

    // Check that changes are displayed
    await expect(page.locator('[data-testid="change-item"]')).toHaveCount(2);
    await expect(
      page.locator('[data-testid="change-impact"]').first()
    ).toContainText('0.9');
  });

  test('should allow exporting optimized resume', async ({ page }) => {
    // Setup optimization result
    await page.route('**/optimize_resume', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            optimized_content: 'Optimized resume ready for export',
            changes_made: [],
            before_score: 75,
            after_score: 90,
            improvement_percentage: 20,
          },
        }),
      });
    });

    // Complete optimization
    await page.fill('[data-testid="original-resume-editor"]', 'Resume content');
    await page.fill('[data-testid="job-description-input"]', 'Job description');
    await page.click('[data-testid="optimize-button"]');

    await expect(
      page.locator('[data-testid="optimized-content"]')
    ).toBeVisible();

    // Test export
    const downloadPromise = page.waitForEvent('download');
    await page.click('[data-testid="export-optimized"]');
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(
      /optimized.*resume.*\.(txt|docx|pdf)$/
    );
  });

  test('should handle optimization errors gracefully', async ({ page }) => {
    // Mock API error
    await page.route('**/optimize_resume', route => {
      route.fulfill({
        status: 500,
        contentType: 'application/json',
        body: JSON.stringify({
          success: false,
          error: 'Optimization service temporarily unavailable',
        }),
      });
    });

    await page.fill('[data-testid="original-resume-editor"]', 'Resume content');
    await page.fill('[data-testid="job-description-input"]', 'Job description');
    await page.click('[data-testid="optimize-button"]');

    // Should show error message
    await expect(page.locator('[data-testid="error-message"]')).toBeVisible();
    await expect(page.locator('[data-testid="error-message"]')).toContainText(
      'temporarily unavailable'
    );

    // Button should be re-enabled
    await expect(page.locator('[data-testid="optimize-button"]')).toBeEnabled();
  });

  test('should preserve user input during optimization', async ({ page }) => {
    const originalContent = 'My original resume content';
    const jobContent = 'Job description content';

    await page.fill('[data-testid="original-resume-editor"]', originalContent);
    await page.fill('[data-testid="job-description-input"]', jobContent);

    // Mock slow optimization to ensure content persists
    await page.route('**/optimize_resume', async route => {
      await new Promise(resolve => setTimeout(resolve, 1000));
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            optimized_content: 'Optimized version',
            changes_made: [],
            before_score: 70,
            after_score: 85,
            improvement_percentage: 21.4,
          },
        }),
      });
    });

    await page.click('[data-testid="optimize-button"]');

    // Original content should still be there during optimization
    await expect(
      page.locator('[data-testid="original-resume-editor"]')
    ).toHaveValue(originalContent);
    await expect(
      page.locator('[data-testid="job-description-input"]')
    ).toHaveValue(jobContent);

    // After optimization completes
    await expect(
      page.locator('[data-testid="optimized-content"]')
    ).toBeVisible();
    await expect(
      page.locator('[data-testid="original-resume-editor"]')
    ).toHaveValue(originalContent);
  });
});
