import sequence from 'svelte-sequential-preprocessor';

import { preprocessMeltUI } from '@melt-ui/pp';
import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://kit.svelte.dev/docs/integrations#preprocessors
  // for more information about preprocessors
  preprocess: sequence([vitePreprocess(), preprocessMeltUI()]),

  kit: {
    adapter: adapter(),
    alias: {
      '@bindings/*': '../identity-wallet/bindings/*',
      $i18n: 'src/i18n',
    },
    // SvelteKit prerenders all non-dynamic routes by default.
    // Dynamic routes cannot be prerendered because they depend on state that is unknown at build time.
  },
};

export default config;
