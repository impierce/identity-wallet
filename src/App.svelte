<script lang="ts">
  import { Router, Link, Route, navigate } from 'svelte-navigator';
  import Greet from './lib/Greet.svelte';
  import Welcome from './routes/Welcome.svelte';
  import { state } from './stores';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import Profile from './routes/Profile.svelte';
  import { Button } from '@impierce/ui-components';
  // import { initI18n } from './i18n/i18n-svelte';
  // import { initI18nSvelte } from 'typesafe-i18n/svelte';
  // import { setLocale } from './i18n/i18n-svelte';
  import { loadAllLocales } from './i18n/i18n-util.sync';

  onMount(async () => {
    // initI18n('en');

    loadAllLocales();

    await invoke('execute_command', {
      commandMessage: { command: '[INIT] Get initial state', payload: '' }
    });

    // setLocale($state.locale);
  });

  let selected_locale;

  const setLocale = async () => {
    await invoke('execute_command', {
      commandMessage: { command: '[SETTINGS] Set locale', payload: selected_locale }
    });
  };

  const reset = async () => {
    await invoke('execute_command', {
      commandMessage: { command: '[INIT] Reset', payload: '' }
    });
  };
</script>

<!-- <main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://tauri.app" target="_blank" rel="noreferrer">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
  </div>

  <p class="font-semibold text-gray-400 underline">Click on the Tauri logo to learn more.</p>

  <div class="row">
    <Greet />
  </div>
  {JSON.stringify($state)}
</main> -->

<main class="h-screen bg-gray-100">
  <Router>
    dev mode:
    <button class="rounded bg-gray-300 px-4 py-2 text-gray-700 shadow" on:click={() => navigate(-1)}
      >navigate back</button
    >
    <button class="rounded bg-gray-300 px-4 py-2 text-gray-700 shadow" on:click={reset}
      >reset app</button
    >
    <hr class="border border-violet-600" />
    <!-- <Link to="">root</Link> -->
    <Route path="welcome" component={Welcome} />
    <Route path="profile" component={Profile} primary={false} />
  </Router>

  <select bind:value={selected_locale} on:change={() => setLocale()}>
    <option value="en">en</option>
    <option value="de">de</option>
    <option value="nl">nl</option>
  </select>

  <hr class="border border-violet-600" />

  state:
  <div class="text-xs text-neutral-500">
    <pre>{JSON.stringify($state, null, 2)}</pre>
  </div>
</main>

<!-- style global breaks tailwind buttons-->
<!-- <style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style> -->
