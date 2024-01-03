import type { PlaywrightTestConfig } from '@playwright/experimental-ct-svelte';

const config: PlaywrightTestConfig = {
  testDir: './tests',
  use: {
    trace: 'on-first-retry',
    ctViteConfig: {
      resolve: {
        alias: {
          $lib: 'src/lib',
          $app: './node_modules/@sveltejs/kit/src/runtime/app',
        },
      },
    },
  },
  webServer: {
    command: 'npm run dev',
    url: 'http://localhost:5173',
    reuseExistingServer: !process.env.CI,
    stdout: 'pipe',
  },
};

export default config;
