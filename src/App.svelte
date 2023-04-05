<script lang="ts">
  import { Router, Link, Route, navigate } from 'svelte-navigator';
  import Greet from './lib/Greet.svelte';
  import Welcome from './routes/Welcome.svelte';
  import { state } from './stores';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import Profile from './routes/Profile.svelte';
  import { Button } from '@impierce/ui-components';

  onMount(async () => {
    await invoke('execute_command', {
      commandMessage: { command: '[INIT] Get initial state', payload: '' }
    });
  });

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

<div class="bg-gray-100 h-screen">
<Router>
  <button class="bg-gray-300 text-gray-700 py-2 px-4 rounded shadow" on:click={() => navigate(-1)}
    >back</button
  >
  <button class="bg-gray-300 text-gray-700 py-2 px-4 rounded shadow" on:click={reset}>reset</button>
  <!-- <Link to="">root</Link> -->
  <Route path="welcome" component={Welcome} />
  <Route path="profile" component={Profile} primary={false} />
</Router>

<div class="text-neutral-500 text-xs">
  <pre>{JSON.stringify($state, null, 2)}</pre>
</div>
</div>

<!-- style global breaks tailwind buttons-->
<!-- <style global>
  @tailwind base;
  @tailwind components;
  @tailwind utilities;
</style> -->
