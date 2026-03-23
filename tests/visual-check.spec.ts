
import { test, expect } from '@playwright/test';

test('MacOS Layout Back Button visual check', async ({ page }) => {
  // Navigate to the app root (assuming dev server running on port 1420)
  await page.goto('http://localhost:1420');

  // Navigate deeper to trigger the back button appearance (e.g., to movies)
  // Assuming clicking sidebar '电影' goes to /movies and pushes history
  await page.click('.sidebar-item >> text=电影');
  
  // Wait for navigation and back button to appear
  // The back button shows when window.history.length > 1.
  // We might need to force a pushState or just navigate.
  await page.waitForTimeout(1000); 

  // Verify back button visibility
  const backButton = page.locator('.macos-titlebar-back-btn');
  await expect(backButton).toBeVisible();

  // Take screenshot of the top-left area including traffic lights and back button
  await page.screenshot({ 
    path: '.sisyphus/evidence/macos-back-tuned-default.png',
    clip: { x: 0, y: 0, width: 300, height: 100 }
  });

  // Hover state
  await backButton.hover();
  await page.waitForTimeout(300); // Wait for transition
  await page.screenshot({ 
    path: '.sisyphus/evidence/macos-back-tuned-hover.png',
    clip: { x: 0, y: 0, width: 300, height: 100 }
  });
});
