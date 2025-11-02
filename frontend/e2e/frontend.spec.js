const { test, expect } = require('@playwright/test');
const path = require('path');

test.describe('E-commerce Frontend Tests', () => {
  test('should load homepage and take screenshot', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Welcome to E-Shop');
    await page.screenshot({ path: 'screenshots/homepage.png', fullPage: true });
  });

  test('should navigate to products page and take screenshot', async ({ page }) => {
    await page.goto('/');
    await page.click('text=Shop Now');
    await expect(page).toHaveURL('/products');
    await expect(page.locator('h1')).toContainText('Our Products');
    await page.screenshot({ path: 'screenshots/products-page.png', fullPage: true });
  });

  test('should display product cards with details', async ({ page }) => {
    await page.goto('/products');
    const productCards = await page.locator('[class*="Card"]').count();
    expect(productCards).toBeGreaterThan(0);
    await page.screenshot({ path: 'screenshots/product-cards.png', fullPage: true });
  });

  test('should navigate to product detail page', async ({ page }) => {
    await page.goto('/products');
    await page.click('text=View Details >> nth=0');
    await expect(page).toHaveURL(/\/products\/\d+/);
    await page.screenshot({ path: 'screenshots/product-detail.png', fullPage: true });
  });

  test('should navigate to cart page and show empty state', async ({ page }) => {
    await page.goto('/cart');
    await expect(page.locator('text=Your cart is empty')).toBeVisible();
    await page.screenshot({ path: 'screenshots/empty-cart.png', fullPage: true });
  });

  test('should navigate to login page and take screenshot', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h2')).toContainText('Welcome Back');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    await page.screenshot({ path: 'screenshots/login-page.png', fullPage: true });
  });

  test('should navigate to register page and take screenshot', async ({ page }) => {
    await page.goto('/register');
    await expect(page.locator('h2')).toContainText('Create Account');
    await expect(page.locator('input#username')).toBeVisible();
    await expect(page.locator('input#email')).toBeVisible();
    await page.screenshot({ path: 'screenshots/register-page.png', fullPage: true });
  });

  test('should validate login form', async ({ page }) => {
    await page.goto('/login');
    await page.click('button[type="submit"]');
    await expect(page.locator('text=Email is required')).toBeVisible();
    await page.screenshot({ path: 'screenshots/login-validation.png', fullPage: true });
  });

  test('should validate register form', async ({ page }) => {
    await page.goto('/register');
    await page.fill('input#username', 'ab');
    await page.fill('input#email', 'invalid-email');
    await page.fill('input#password', '123');
    await page.fill('input#confirmPassword', '456');
    await page.click('button[type="submit"]');
    await page.screenshot({ path: 'screenshots/register-validation.png', fullPage: true });
  });

  test('should have responsive header with mobile menu', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    await expect(page.locator('button[aria-label="Toggle menu"]')).toBeVisible();
    await page.screenshot({ path: 'screenshots/mobile-header.png', fullPage: true });
  });

  test('should open mobile menu on small screens', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    await page.click('button[aria-label="Toggle menu"]');
    await expect(page.locator('text=Products')).toBeVisible();
    await page.screenshot({ path: 'screenshots/mobile-menu-open.png', fullPage: true });
  });

  test('should display footer with links', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('footer')).toBeVisible();
    await expect(page.locator('text=Privacy Policy')).toBeVisible();
    await page.screenshot({ path: 'screenshots/footer.png' });
  });
});
