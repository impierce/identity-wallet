import path from 'path';

import { internalIpV4 } from 'internal-ip';
import Icons from 'unplugin-icons/vite';
import { defineConfig, type PluginOption } from 'vite';

import { sveltekit } from '@sveltejs/kit/vite';

const mobile = process.env.TAURI_PLATFORM === 'android' || process.env.TAURI_PLATFORM === 'ios';

const full_reload_always: PluginOption = {
  name: 'full-reload-always',
  handleHotUpdate({ server }) {
    server.ws.send({ type: 'full-reload' });
    return [];
  },
};

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
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    host: '0.0.0.0', // listen on all addresses
    port: 5173,
    strictPort: true,
    hmr: {
      protocol: 'ws',
      host: await internalIpV4(),
      port: 5183,
    },
  },
  // to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    // Tauri supports es2021
    target: process.env.TAURI_PLATFORM == 'windows' ? 'chrome105' : 'safari13',
    // don't minify for debug builds
    // TODO: "minify" breaks the type? ("No overload matches this call")
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    // produce sourcemaps for debug builds
    sourcemap: !!process.env.TAURI_DEBUG,
  },
  optimizeDeps: {
    exclude: ['~icons/*'],
  },
}));
