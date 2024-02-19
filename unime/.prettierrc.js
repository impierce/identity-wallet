/** @type {import("prettier").Config} */
const config = {
  useTabs: false,
  tabWidth: 2,
  singleQuote: true,
  semi: true,
  trailingComma: 'all',
  printWidth: 120,
  importOrder: [
    '^(svelte)$',
    '',
    '<THIRD_PARTY_MODULES>',
    '',
    '^@(.*)$',
    '',
    '^\\$(src|lib)/(.*)$',
    '',
    '^~icons/(.*)$',
    '',
    '^[./]',
  ],
  importOrderTypeScriptVersion: '5.0.0',
  plugins: ['prettier-plugin-svelte', 'prettier-plugin-tailwindcss', '@ianvs/prettier-plugin-sort-imports'],
  overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }],
};

export default config;
