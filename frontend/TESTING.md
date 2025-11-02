# Frontend Testing Guide

## Playwright E2E Tests

This project includes comprehensive end-to-end tests using Playwright.

### Test Setup

```bash
# Install dependencies (if not already done)
npm install

# Install Playwright browsers
npx playwright install chromium
```

### Running Tests

```bash
# Run all E2E tests with screenshots
npx playwright test

# Run tests in UI mode
npx playwright test --ui

# Run tests in headed mode (see browser)
npx playwright test --headed

# Run specific test file
npx playwright test e2e/frontend.spec.js
```

### Test Coverage

The E2E test suite covers:

1. **Homepage Test**
   - Verifies hero section loads
   - Takes full-page screenshot
   - Screenshot: `screenshots/homepage.png`

2. **Products Page Test**
   - Navigation from homepage
   - Product grid display
   - Product cards with details
   - Screenshots: `screenshots/products-page.png`, `screenshots/product-cards.png`

3. **Product Detail Page Test**
   - Navigation to individual product
   - Product details display
   - Screenshot: `screenshots/product-detail.png`

4. **Cart Page Test**
   - Empty cart state
   - Screenshot: `screenshots/empty-cart.png`

5. **Login Page Test**
   - Form elements display
   - Form validation
   - Screenshots: `screenshots/login-page.png`, `screenshots/login-validation.png`

6. **Register Page Test**
   - Form elements display
   - Form validation
   - Screenshots: `screenshots/register-page.png`, `screenshots/register-validation.png`

7. **Responsive Design Tests**
   - Mobile header with hamburger menu
   - Mobile menu functionality
   - Screenshots: `screenshots/mobile-header.png`, `screenshots/mobile-menu-open.png`

8. **Footer Test**
   - Footer visibility
   - Link display
   - Screenshot: `screenshots/footer.png`

## Screenshot Generation

Screenshots are automatically generated during test execution and saved to the `screenshots/` directory.

To generate screenshots:

```bash
# Run tests (screenshots are captured automatically)
npx playwright test

# View test report with screenshots
npx playwright show-report
```

## Manual Testing Checklist

### Desktop (1280x720)
- [ ] Homepage loads correctly
- [ ] Navigation to Products page works
- [ ] Product cards display in grid
- [ ] Product detail page loads
- [ ] Cart page shows empty state
- [ ] Login form displays correctly
- [ ] Register form displays correctly
- [ ] Footer is visible

### Mobile (375x667)
- [ ] Mobile header displays
- [ ] Hamburger menu icon visible
- [ ] Mobile menu opens/closes
- [ ] All pages are responsive
- [ ] Touch interactions work

### Form Validation
- [ ] Login form validates email
- [ ] Login form validates password
- [ ] Register form validates all fields
- [ ] Error messages display correctly

### Navigation
- [ ] All routes accessible
- [ ] Browser back/forward work
- [ ] Direct URL access works
- [ ] 404 handling (if implemented)

## CI/CD Integration

To run tests in CI/CD pipeline:

```bash
# Set CI environment variable
CI=true npx playwright test

# This will:
# - Run tests with 2 retries
# - Use single worker
# - Generate HTML report
# - Capture screenshots on failure
```

## Test Results

Expected output when all tests pass:

```
Running 12 tests using 1 worker

  ✓ should load homepage and take screenshot
  ✓ should navigate to products page and take screenshot
  ✓ should display product cards with details
  ✓ should navigate to product detail page
  ✓ should navigate to cart page and show empty state
  ✓ should navigate to login page and take screenshot
  ✓ should navigate to register page and take screenshot
  ✓ should validate login form
  ✓ should validate register form
  ✓ should have responsive header with mobile menu
  ✓ should open mobile menu on small screens
  ✓ should display footer with links

  12 passed (30s)
```

## Screenshot Examples

After running tests, you'll find screenshots in the `screenshots/` directory:

- `homepage.png` - Full homepage view
- `products-page.png` - Product listing
- `product-cards.png` - Close-up of product cards
- `product-detail.png` - Individual product page
- `empty-cart.png` - Empty cart state
- `login-page.png` - Login form
- `register-page.png` - Registration form
- `login-validation.png` - Login form validation errors
- `register-validation.png` - Register form validation errors
- `mobile-header.png` - Mobile responsive header
- `mobile-menu-open.png` - Mobile menu expanded
- `footer.png` - Footer section

## Notes

- Screenshots are generated at 1280x720 resolution by default
- Mobile screenshots use 375x667 (iPhone SE size)
- Full-page screenshots capture entire scrollable content
- Tests run against `http://localhost:3000` (dev server auto-starts)
- Browser: Chromium (Chrome/Edge)
