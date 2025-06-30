import { test, expect } from '@playwright/test';
import path from 'path';

test.describe('Resume Analysis Workflow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('[data-testid="main-layout"]');
    await page.click('[data-testid="nav-analysis"]');
    await expect(page.locator('[data-testid="analysis-page"]')).toBeVisible();
  });

  test('should allow file upload via drag and drop', async ({ page }) => {
    const fileChooserPromise = page.waitForEvent('filechooser');
    
    // Create a test file
    const testResume = `
John Doe
Software Engineer
Email: john.doe@example.com
Phone: (555) 123-4567

EXPERIENCE
Senior Software Engineer at Tech Corp (2020-2024)
- Developed React applications using TypeScript
- Led team of 5 developers
- Implemented CI/CD pipelines

SKILLS
- JavaScript, TypeScript, React, Node.js
- Python, Java, SQL
- AWS, Docker, Kubernetes
- Git, Jenkins, CI/CD

EDUCATION
Bachelor of Science in Computer Science
University of Technology (2016-2020)
    `;

    // Click the upload area
    await page.click('[data-testid="file-upload-area"]');
    const fileChooser = await fileChooserPromise;
    
    // Create a temporary file and upload it
    await fileChooser.setFiles({
      name: 'test-resume.txt',
      mimeType: 'text/plain',
      buffer: Buffer.from(testResume)
    });

    // Wait for file to be processed
    await expect(page.locator('[data-testid="uploaded-file"]')).toBeVisible();
    await expect(page.locator('[data-testid="file-name"]')).toContainText('test-resume.txt');
  });

  test('should allow job description input', async ({ page }) => {
    const jobDescription = `
We are seeking a Senior Full-Stack Developer to join our team.

Requirements:
- 5+ years of experience in software development
- Proficiency in React, TypeScript, and Node.js
- Experience with cloud platforms (AWS, Azure)
- Strong knowledge of databases (SQL, NoSQL)
- Experience with DevOps practices and CI/CD
- Bachelor's degree in Computer Science or related field

Responsibilities:
- Develop and maintain web applications
- Collaborate with cross-functional teams
- Mentor junior developers
- Participate in code reviews
- Design scalable architecture
    `;

    const textarea = page.locator('[data-testid="job-description-input"]');
    await expect(textarea).toBeVisible();
    
    await textarea.fill(jobDescription);
    
    // Check character count if present
    const charCount = page.locator('[data-testid="char-count"]');
    if (await charCount.isVisible()) {
      await expect(charCount).toContainText(jobDescription.length.toString());
    }
  });

  test('should perform complete analysis workflow', async ({ page }) => {
    // Upload resume file
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('[data-testid="file-upload-area"]');
    const fileChooser = await fileChooserPromise;
    
    await fileChooser.setFiles({
      name: 'test-resume.txt',
      mimeType: 'text/plain',
      buffer: Buffer.from('John Doe\nSoftware Engineer\nSkills: React, TypeScript, Python')
    });

    await expect(page.locator('[data-testid="uploaded-file"]')).toBeVisible();

    // Add job description
    await page.fill('[data-testid="job-description-input"]', 
      'Looking for a React developer with TypeScript experience.');

    // Select model (if dropdown is available)
    const modelSelect = page.locator('[data-testid="model-select"]');
    if (await modelSelect.isVisible()) {
      await modelSelect.click();
      await page.click('[data-testid="model-option"]:first-child');
    }

    // Start analysis
    await page.click('[data-testid="analyze-button"]');

    // Wait for analysis to complete
    await expect(page.locator('[data-testid="analysis-loading"]')).toBeVisible();
    await expect(page.locator('[data-testid="analysis-results"]')).toBeVisible({ timeout: 30000 });

    // Verify results display
    await expect(page.locator('[data-testid="overall-score"]')).toBeVisible();
    await expect(page.locator('[data-testid="category-scores"]')).toBeVisible();
    await expect(page.locator('[data-testid="recommendations"]')).toBeVisible();
  });

  test('should show loading state during analysis', async ({ page }) => {
    // Mock a slow analysis response
    await page.route('**/analyze_resume', async route => {
      await new Promise(resolve => setTimeout(resolve, 2000));
      await route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            overall_score: 85,
            category_scores: {
              skills: 90,
              experience: 80,
              education: 85,
              keywords: 88,
              format: 92
            },
            detailed_feedback: 'Great resume!',
            missing_keywords: ['Python', 'AWS'],
            recommendations: ['Add more Python experience'],
            processing_time_ms: 2000
          }
        })
      });
    });

    // Upload file and start analysis
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('[data-testid="file-upload-area"]');
    const fileChooser = await fileChooserPromise;
    
    await fileChooser.setFiles({
      name: 'test.txt',
      mimeType: 'text/plain',
      buffer: Buffer.from('Test resume content')
    });

    await page.fill('[data-testid="job-description-input"]', 'Test job description');
    await page.click('[data-testid="analyze-button"]');

    // Check loading state
    await expect(page.locator('[data-testid="analysis-loading"]')).toBeVisible();
    await expect(page.locator('[data-testid="analyze-button"]')).toBeDisabled();

    // Wait for completion
    await expect(page.locator('[data-testid="analysis-results"]')).toBeVisible();
    await expect(page.locator('[data-testid="analyze-button"]')).toBeEnabled();
  });

  test('should export analysis results', async ({ page }) => {
    // First complete an analysis (mock the response)
    await page.route('**/analyze_resume', route => {
      route.fulfill({
        status: 200,
        contentType: 'application/json',
        body: JSON.stringify({
          success: true,
          data: {
            overall_score: 85,
            category_scores: { skills: 90, experience: 80, education: 85, keywords: 88, format: 92 },
            detailed_feedback: 'Great resume!',
            missing_keywords: [],
            recommendations: ['Add more experience'],
            processing_time_ms: 1500
          }
        })
      });
    });

    // Quick analysis setup
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.click('[data-testid="file-upload-area"]');
    const fileChooser = await fileChooserPromise;
    await fileChooser.setFiles({
      name: 'test.txt',
      mimeType: 'text/plain',
      buffer: Buffer.from('Test content')
    });

    await page.fill('[data-testid="job-description-input"]', 'Test job');
    await page.click('[data-testid="analyze-button"]');
    await expect(page.locator('[data-testid="analysis-results"]')).toBeVisible();

    // Test export functionality
    const downloadPromise = page.waitForEvent('download');
    await page.click('[data-testid="export-results"]');
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/analysis.*\.(json|csv|txt)$/);
  });
});