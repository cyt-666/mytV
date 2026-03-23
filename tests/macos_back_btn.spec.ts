import { test, expect } from '@playwright/test';

test.describe('macOS Titlebar Back Button', () => {
  test('Scenario A: Back button visibility & Hover on Desktop', async ({ page }) => {
    // Given: Viewport is set to Desktop size (1280x800)
    await page.setViewportSize({ width: 1280, height: 800 });

    // And: We are on the home page
    await page.goto('http://localhost:5173/');
    
    await page.waitForTimeout(1000); 

    // When: We navigate to "TV Shows" to create history
    const tvLink = page.getByText('电视剧', { exact: true }).first();
    if (await tvLink.isVisible()) {
        await tvLink.click();
    } else {
        await page.goto('http://localhost:5173/shows');
    }
    
    await expect(page).toHaveURL(/.*\/shows/);
    
    // Then: The back button should be visible
    const backBtn = page.locator('.macos-titlebar-back-btn');
    await expect(backBtn).toBeVisible();

    // And: We can hover over it
    await backBtn.hover();
    
    await page.screenshot({ path: '.sisyphus/evidence/macos-back-desktop.png' });
  });

  test('Scenario B: Responsive Hiding at < 700px', async ({ page }) => {
    // Given: Viewport is set to Mobile/Narrow size (600x800)
    await page.setViewportSize({ width: 600, height: 800 });

    // And: We have history (navigated to /shows)
    await page.goto('http://localhost:5173/');
    await page.waitForTimeout(500);
    const tvLink = page.getByText('电视剧', { exact: true }).first();
    if (await tvLink.isVisible()) {
        await tvLink.click();
    } else {
        await page.goto('http://localhost:5173/shows');
    }

    // Then: The back button should be HIDDEN
    const backBtn = page.locator('.macos-titlebar-back-btn');
    await expect(backBtn).toBeHidden();

    await page.screenshot({ path: '.sisyphus/evidence/macos-back-mobile-hidden.png' });
  });

  test('Scenario C: Keyboard Shortcut (Alt + ArrowLeft)', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    
    // Given: We have history (Home -> Shows)
    await page.goto('http://localhost:5173/');
    await page.waitForTimeout(500);
    
    const tvLink = page.getByText('电视剧', { exact: true }).first();
    if (await tvLink.isVisible()) {
        await tvLink.click();
    } else {
        await page.goto('http://localhost:5173/shows');
    }
    await expect(page).toHaveURL(/.*\/shows/);

    // When: We press Alt+ArrowLeft
    await page.keyboard.press('Alt+ArrowLeft');

    // Then: We should be back at the Home URL
    await page.waitForURL('http://localhost:5173/');
    await expect(page).toHaveURL('http://localhost:5173/');
  });
});
