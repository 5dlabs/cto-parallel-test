import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import react from 'eslint-plugin-react'
import security from 'eslint-plugin-security'
import noUnsanitized from 'eslint-plugin-no-unsanitized'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist', '.next', 'build', 'out']),
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    plugins: {
      react,
      security,
      'no-unsanitized': noUnsanitized,
    },
    extends: [
      js.configs.recommended,
      react.configs.flat.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: { ...globals.browser, React: 'readonly', process: 'readonly', require: 'readonly' },
      // Use TS parser so rules also cover .ts/.tsx files
      parser: (await import('@typescript-eslint/parser')).default,
      parserOptions: {
        ecmaVersion: 'latest',
        ecmaFeatures: { jsx: true },
        sourceType: 'module',
        project: false,
      },
    },
    rules: {
      // General hygiene
      'no-unused-vars': ['error', { varsIgnorePattern: '^[A-Z_]' }],
      'react-refresh/only-export-components': 'off',
      'react/react-in-jsx-scope': 'off',
      'react/prop-types': 'off',

      // Security hardening
      'no-eval': 'error',
      'no-implied-eval': 'error',
      'no-new-func': 'error',
      'no-script-url': 'error',
      'react/no-danger': 'error',
      'no-unsanitized/method': 'error',
      'no-unsanitized/property': 'error',

      // Adjust noisy security rules for browser context
      'security/detect-object-injection': 'off',
      'security/detect-non-literal-fs-filename': 'off',
      'security/detect-unsafe-regex': 'warn',
    },
    settings: {
      react: { version: 'detect' },
    },
  },
  {
    files: ['**/*.config.js', '**/*.config.ts'],
    languageOptions: {
      globals: { ...globals.node },
    },
    rules: {
      // Config files may use Node globals or ESM helpers
      'no-undef': 'off',
    },
  },
])
