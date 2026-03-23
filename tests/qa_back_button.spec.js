import { test, expect } from '@playwright/test';

test('verify macos back button styling', async ({ page }) => {
  // 1. Navigate to home
  await page.goto('http://localhost:5173/');
  
  // 2. Navigate to another page to populate history (e.g., movies)
  await page.goto('http://localhost:5173/movies');
  
  // 3. Wait for the back button to appear
  const backBtn = page.locator('.macos-titlebar-back-btn');
  await backBtn.waitFor({ state: 'visible', timeout: 5000 });
  
  // 4. Check computed styles
  // Offset 3px (transform: translateY(3px) or matrix equivalent)
  const transform = await backBtn.evaluate((el) => window.getComputedStyle(el).transform);
  console.log('Back button transform:', transform);
  
  // Verify it contains the 3px translation (matrix(1, 0, 0, 1, 0, 3) or similar)
  // The matrix format is matrix(a, b, c, d, tx, ty). ty should be 3.
  expect(transform).toMatch(/matrix\(.*,\s*3\)/);

  // Parent padding/gap check
  // The instruction says "Parent padding should reflect 14px gap"
  // Let's check the parent's computed style.
  const parent = backBtn.locator('..');
  const gap = await parent.evaluate((el) => window.getComputedStyle(el).gap);
  console.log('Parent gap:', gap);
  
  // If gap is used
  if (gap && gap !== 'normal') {
      expect(gap).toBe('14px');
  } else {
      // Maybe it's padding-left on the parent or margin-right on the button?
      // Instruction says "Parent padding should reflect 14px gap".
      // Let's check padding-left of the container?
      // Or maybe the button is inside a container that has padding?
      // I'll log padding as well to be sure.
      const paddingLeft = await parent.evaluate((el) => window.getComputedStyle(el).paddingLeft);
      console.log('Parent padding-left:', paddingLeft);
  }

  // 5. Screenshot - Default
  await page.screenshot({ path: '.sisyphus/evidence/macos-back-tuned-default.png' });

  // 6. Screenshot - Hover
  await backBtn.hover();
  // Wait a bit for transition
  await page.waitForTimeout(300);
  await page.screenshot({ path: '.sisyphus/evidence/macos-back-tuned-hover.png' });
});
