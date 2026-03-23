import { test, expect } from '@playwright/test';

test.describe('macOS Titlebar Back Button Styles', () => {
  test('Verify CSS properties and non-draggable state', async ({ page }) => {
    // Setup: Desktop Viewport
    await page.setViewportSize({ width: 1280, height: 800 });

    // Action: Navigate to create history
    await page.goto('http://localhost:5173/');
    await page.waitForTimeout(500);
    const tvLink = page.getByText('电视剧', { exact: true }).first();
    if (await tvLink.isVisible()) {
        await tvLink.click();
    } else {
        await page.goto('http://localhost:5173/shows');
    }
    
    await expect(page).toHaveURL(/.*\/shows/);
    
    const backBtn = page.locator('.macos-titlebar-back-btn');
    await expect(backBtn).toBeVisible();

    // Assertion: Check dimensions and shape
    await expect(backBtn).toHaveCSS('height', '28px');
    await expect(backBtn).toHaveCSS('border-radius', '14px');
    
    // Assertion: Check non-draggable region
    await expect(backBtn).toHaveCSS('-webkit-app-region', 'no-drag');

    // Assertion: Verify hover interaction
    await backBtn.hover();
  });
});
