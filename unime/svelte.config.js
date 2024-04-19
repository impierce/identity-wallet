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
    prerender: {
      entries: [
        '*',
        '/activity/connection/0',
        '/activity/connection/1',
        '/activity/connection/2',
        '/goals/2/faqs',
        '/goals/2/step/0',
        '/credentials/*',
        '/badges/*',
      ],
    },
  },
};

export default config;
