module.exports = {
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint'],
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/stylistic',
    'plugin:svelte/recommended',
    'prettier',
  ],
  root: true,
  parserOptions: {
    sourceType: 'module',
    ecmaVersion: 2020,
    extraFileExtensions: ['.svelte'],
  },
  overrides: [
    {
      files: ['*.svelte'],
      parser: 'svelte-eslint-parser',
      parserOptions: {
        parser: '@typescript-eslint/parser',
      },
    },
  ],
  env: {
    browser: true,
    es2017: true,
    node: true,
  },
  rules: {
    'no-undef': 'off', // only necessary for JS (https://typescript-eslint.io/troubleshooting/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors)
    'no-console': 'error', // disallow any logs to console
    'svelte/no-at-html-tags': 'warn', // TODO: security risk even applicable for context of Tauri app?
  },
};
