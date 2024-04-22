import path from 'path';

import { internalIpV4 } from 'internal-ip';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';

import { sveltekit } from '@sveltejs/kit/vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [sveltekit(), Icons({ compiler: 'svelte' })],
  resolve: {
    alias: {
      $src: path.resolve('./src'),
      $lib: path.resolve('./src/lib'),
    },
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    globals: true,
    environment: 'jsdom',
    coverage: {
      include: ['src/**'],
      exclude: ['src/i18n/**'],
    },
  },
  clearScreen: false,
  server: {
    host: '0.0.0.0',
    port: 5173,
    strictPort: true,
    hmr: {
      protocol: 'ws',
      host: await internalIpV4(),
      port: 5183,
    },
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // TODO: "minify" breaks the type? ("No overload matches this call")
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
      'date-fns',
      'date-fns/locale',
      '@tauri-apps/plugin-barcode-scanner',
    ],
  },
}));
