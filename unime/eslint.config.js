import { fileURLToPath } from 'node:url';

import prettier from 'eslint-config-prettier';
import svelte from 'eslint-plugin-svelte';
import globals from 'globals';
import ts from 'typescript-eslint';

import { includeIgnoreFile } from '@eslint/compat';
import js from '@eslint/js';

const gitignorePath = fileURLToPath(new URL('./.gitignore', import.meta.url));

export default ts.config(
  includeIgnoreFile(gitignorePath),
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs['flat/recommended'],
  prettier,
  ...svelte.configs['flat/prettier'],
  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
  {
    files: ['**/*.svelte'],
    languageOptions: {
      parserOptions: {
        parser: ts.parser,
      },
    },
  },
  {
    // i18n files are generated and have to be excluded besides everything from `.gitignore`.
    ignores: ['src/i18n/*.ts'],
  },
  {
    rules: {
      'no-console': 'error',
      'svelte/no-at-html-tags': 'warn', // TODO: security risk even applicable for context of Tauri app?
    },
  },
);
