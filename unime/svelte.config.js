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
    // For dynamic routes we set `prerender = false`.
    // But adapter-static wants all routes to be prerendered by default.
    // `strict: false` makes adapter-static ignore pages that are not prerendered.
    adapter: adapter({ strict: false }),
    alias: {
      '@bindings/*': '../identity-wallet/bindings/*',
      $i18n: 'src/i18n',
    },
    // Only the development switches for the frontend (`UNIME_DEV_PUBLIC_*` in `unime/.env`) reach the client, not the
    // backend's `UNIME_DEV_*` variables.
    env: {
      publicPrefix: 'UNIME_DEV_PUBLIC_',
    },
  },
};

export default config;
