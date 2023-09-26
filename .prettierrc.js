/** @type {import("prettier").Config} */
const config = {
  useTabs: false,
  tabWidth: 2,
  singleQuote: true,
  semi: true,
  trailingComma: 'none',
  printWidth: 100,
  importOrder: ['^@(.*)$', '^\\$(src|lib)/(.*)$', '^~icons/(.*)$', '^[./]'],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
  plugins: [
    'prettier-plugin-svelte',
    'prettier-plugin-tailwindcss',
    '@trivago/prettier-plugin-sort-imports'
  ],
  overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }]
};

export default config;
