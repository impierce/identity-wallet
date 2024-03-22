// @ts-check

import eslint from '@eslint/js';
import eslintConfigPrettier from 'eslint-config-prettier';
import eslintPluginSvelte from 'eslint-plugin-svelte';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.recommended,
  ...tseslint.configs.stylistic,
//   ...eslintPluginSvelte.configs['flat/recommended'],
  eslintConfigPrettier,
  {
    rules: {
        'no-console': 'warn'
    }
  },
//   {
//     files: ['*.svelte'],
//     languageOptions: {
//         parserOptions: {
            
//         },
//     }
//   }
);
