/** @type {import("prettier").Config} */
export default {
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
  // prettier-plugin-tailwindcss must be loaded last:
  // https://github.com/tailwindlabs/prettier-plugin-tailwindcss?tab=readme-ov-file#compatibility-with-other-prettier-plugins
  plugins: ['prettier-plugin-svelte', '@ianvs/prettier-plugin-sort-imports', 'prettier-plugin-tailwindcss'],
  // Link to config so Tailwind can sort custom classes like `bg-primary` correctly.
  tailwindConfig: './unime/tailwind.config.js',
};
