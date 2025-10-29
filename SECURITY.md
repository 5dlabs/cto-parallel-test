# Security Documentation

## Known Vulnerabilities

### React-Scripts 5.0.1 Transitive Dependencies

The following vulnerabilities exist in react-scripts 5.0.1 development dependencies:

#### High Severity
1. **nth-check < 2.0.1** - Inefficient Regular Expression Complexity
   - Affects: svgo (dev dependency only)
   - Impact: Development environment only, not in production build
   - Status: Accepted risk - requires react-scripts upgrade

2. **webpack-dev-server <= 5.2.0** - Source code exposure
   - Impact: Development server only
   - Mitigation: Do not expose dev server publicly
   - Status: Accepted risk - fixed in react-scripts 6.x (breaking changes)

#### Moderate Severity
1. **postcss < 8.4.31** - Line return parsing error
   - Affects: resolve-url-loader
   - Impact: Build-time only
   - Status: Accepted risk - requires react-scripts upgrade

### Risk Assessment

All identified vulnerabilities affect **development dependencies only** and do not impact:
- Production builds
- Runtime security
- Deployed applications

### Mitigation Strategy

1. **Short-term**: Accept risk for current implementation
   - Vulnerabilities are in dev dependencies
   - Production build uses only compiled artifacts
   - No exposure to end users

2. **Long-term**: Upgrade to react-scripts 6.x or migrate to Vite
   - Breaking changes require significant refactoring
   - Planned for future sprint

### Security Best Practices

- Never expose webpack-dev-server publicly
- Use production builds for deployment
- Regularly update dependencies
- Monitor security advisories

## Production Dependencies

All production dependencies (runtime) are **free of known vulnerabilities**:
- axios: 1.13.1 (updated from 1.4.0 to fix SSRF/CSRF vulnerabilities)
- react: 18.2.0
- @mui/material: 5.14.0
- All other runtime dependencies: No known vulnerabilities

## Last Updated

2025-10-29
