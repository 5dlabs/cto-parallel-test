const { chromium } = require('playwright');
const path = require('path');

(async () => {
  const browser = await chromium.launch();
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();

  // Serve the built app
  const buildPath = path.join(__dirname, 'build', 'index.html');
  
  console.log('Taking screenshots of the built application...');
  
  // HomePage
  await page.goto(`file://${buildPath}`);
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'screenshots/homepage.png', fullPage: true });
  console.log('✓ Homepage screenshot saved');

  // Mobile view
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto(`file://${buildPath}`);
  await page.waitForTimeout(1000);
  await page.screenshot({ path: 'screenshots/mobile-homepage.png', fullPage: true });
  console.log('✓ Mobile homepage screenshot saved');

  await browser.close();
  console.log('\nAll screenshots generated successfully!');
})();
