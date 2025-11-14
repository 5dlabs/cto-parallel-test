#!/usr/bin/env node
/**
 * audit-ci.cjs
 * Runs npm audit and fails if moderate/high/critical vulnerabilities are found in runtime dependencies.
 */

const { execSync } = require('child_process');

try {
  // Run npm audit with JSON output (omit dev deps; fail threshold handled below)
  const output = execSync('npm audit --omit=dev --json', { encoding: 'utf8' });
  const audit = JSON.parse(output);
  
  // Check for vulnerabilities
  const metadata = audit.metadata || {};
  const vulnerabilities = metadata.vulnerabilities || {};
  
  const moderate = vulnerabilities.moderate || 0;
  const high = vulnerabilities.high || 0;
  const critical = vulnerabilities.critical || 0;
  
  console.log('npm audit summary:');
  console.log(`  Low: ${vulnerabilities.low || 0}`);
  console.log(`  Moderate: ${moderate}`);
  console.log(`  High: ${high}`);
  console.log(`  Critical: ${critical}`);
  
  if (moderate > 0 || high > 0 || critical > 0) {
    console.error('\n❌ Found vulnerabilities in production dependencies!');
    console.error('Run `npm audit` for details.');
    process.exit(1);
  }
  
  console.log('\n✅ No moderate/high/critical vulnerabilities found in production dependencies.');
  process.exit(0);
} catch (error) {
  // npm audit returns non-zero exit code if vulnerabilities are found
  // Try to parse the output if available
  if (error.stdout) {
    try {
      const audit = JSON.parse(error.stdout);
      const metadata = audit.metadata || {};
      const vulnerabilities = metadata.vulnerabilities || {};
      
      const moderate = vulnerabilities.moderate || 0;
      const high = vulnerabilities.high || 0;
      const critical = vulnerabilities.critical || 0;
      
      console.log('npm audit summary:');
      console.log(`  Low: ${vulnerabilities.low || 0}`);
      console.log(`  Moderate: ${moderate}`);
      console.log(`  High: ${high}`);
      console.log(`  Critical: ${critical}`);
      
      if (moderate > 0 || high > 0 || critical > 0) {
        console.error('\n❌ Found vulnerabilities in production dependencies!');
        console.error('Run `npm audit` for details.');
        process.exit(1);
      }
    } catch (parseError) {
      console.error('Failed to parse audit output:', parseError.message);
      process.exit(1);
    }
  }
  
  console.error('npm audit failed:', error.message);
  process.exit(1);
}
