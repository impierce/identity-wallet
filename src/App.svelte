<script lang="ts">
  import { Router, Route, navigate } from 'svelte-navigator';
  import Welcome from './routes/Welcome.svelte';
  import { state } from './stores';
  import { onMount } from 'svelte';
  import Profile from './routes/Profile.svelte';
  import { loadAllLocales } from './i18n/i18n-util.sync';
  import { ChevronUp, ChevronDown } from 'svelte-heros-v2';
  import { fly } from 'svelte/transition';
  import { dispatch } from './lib/dispatcher';

  onMount(async () => {
    loadAllLocales(); //TODO: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  const reset = async () => dispatch({ type: '[App] Reset' });

  let showDevMode = false;

  $state; // TODO: needs to be called at least once to trigger subscribers --> move somewhere else
</script>

<main class="h-screen bg-gray-100">
  <Router>
    <!-- dev mode -->
    {#if showDevMode}
      <div
        class="flex space-x-4 bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
        in:fly={{ y: -64 }}
        out:fly={{ y: -64 }}
      >
        <div class="px-4 py-1 font-medium text-red-700">dev mode</div>
        <button
          class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
          on:click={() => navigate(-1)}>navigate back</button
        >
        <button
          class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
          on:click={reset}>reset app</button
        >
      </div>
    {/if}
    <button
      class="fixed right-3 top-3 rounded-full p-2 hover:bg-red-200"
      on:click={() => (showDevMode = !showDevMode)}
    >
      {#if showDevMode}
        <ChevronUp class="text-red-700" strokeWidth="2" />
      {:else}
        <ChevronDown class="text-red-700" strokeWidth="2" />
      {/if}
    </button>
    <!-- end: dev mode -->
    <div class="">
      <Route path="welcome" component={Welcome} />
      <Route path="profile" component={Profile} primary={false} />
    </div>
  </Router>
</main>
