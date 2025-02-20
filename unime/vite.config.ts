import Icons from 'unplugin-icons/vite';
import { defineConfig, mergeConfig } from 'vite';
import { defineConfig as defineVitestConfig } from 'vitest/config';

import { sveltekit } from '@sveltejs/kit/vite';

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
const viteConfig = defineConfig({
  plugins: [sveltekit(), Icons({ compiler: 'svelte' })],
  clearScreen: false,
  server: {
    host: host || false,
    port: 4173,
    strictPort: true,
    hmr: host
      ? {
          protocol: 'ws',
          host: host,
          port: 5183,
        }
      : undefined,
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  optimizeDeps: {
    exclude: ['~icons/*'],
    // #188: List of dependecies that Vite frequently optimizes.
    // Use `include`, not `exclude`: https://github.com/sveltejs/kit/issues/11793#issuecomment-1965850225.
    // TODO Check if this list can be removed after upgrading to Vite 5.
    include: [
      '@lottiefiles/lottie-player',
      '@tauri-apps/api/path',
      '@tauri-apps/plugin-fs',
      'markdown-it',
      'tailwind-merge',
      'qrcode',
      '@melt-ui/svelte',
      '@tauri-apps/api/event',
      '@tauri-apps/api/core',
      'typesafe-i18n/svelte',
      'typesafe-i18n/utils',
      'typesafe-i18n',
      'typesafe-i18n/detectors',
      '@tauri-apps/plugin-barcode-scanner',
      '@tauri-apps/plugin-shell',
      '@tauri-apps/plugin-log',
      '@aws-crypto/sha256-js',
    ],
  },
});

// TODO: Refactored by AI to fix TS errors (Claude 3.5 Sonnet).
const vitestConfig = defineVitestConfig({
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    globals: true,
    environment: 'jsdom',
    coverage: {
      include: ['src/**'],
      exclude: ['src/i18n/**'],
    },
  },
});

export default mergeConfig(viteConfig, vitestConfig);
