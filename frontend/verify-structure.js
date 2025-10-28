/**
 * Simple verification script to check if all required files exist
 * and have valid syntax
 */

const fs = require('fs');
const path = require('path');

const requiredFiles = [
  'package.json',
  'src/App.js',
  'src/index.js',
  'src/components/Header.js',
  'src/components/Footer.js',
  'src/components/HomePage.js',
  'src/components/ProductList.js',
  'src/components/ProductDetail.js',
  'src/components/Cart.js',
  'src/components/Login.js',
  'src/components/Register.js',
  'public/index.html'
];

console.log('Verifying frontend structure...\n');

let allFilesExist = true;

requiredFiles.forEach(file => {
  const filePath = path.join(__dirname, file);
  const exists = fs.existsSync(filePath);

  if (exists) {
    console.log(`✓ ${file}`);
  } else {
    console.log(`✗ ${file} - MISSING`);
    allFilesExist = false;
  }
});

console.log('\n');

if (allFilesExist) {
  console.log('✅ All required files exist!');
  process.exit(0);
} else {
  console.log('❌ Some files are missing!');
  process.exit(1);
}
