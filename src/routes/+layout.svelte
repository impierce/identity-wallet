<script>
  import '../app.css';

  import { fly } from 'svelte/transition';
  import { ChevronUp, ChevronDown } from 'svelte-heros-v2';
  import { state } from '../stores';
  import { onMount } from 'svelte';
  import { loadAllLocales } from '../i18n/i18n-util.sync';
  import { dispatch } from '$lib/dispatcher';

  onMount(async () => {
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = true;

  $: {
    console.log('state', $state);
  }
</script>

<main>
  {#if showDevMode}
    <div
      class="flex space-x-4 bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
      in:fly={{ y: -64 }}
      out:fly={{ y: -64 }}
    >
      <div class="px-4 py-1 font-medium text-red-700">dev mode</div>
      <button
        class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        >back</button
      >
      <button
        class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[App] Reset' })}>reset</button
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
  <div class="h-full">
    <!-- <Route path="welcome" component={Welcome} /> -->
    <!-- <Route path="profile" component={Profile} primary={false} /> -->
    <slot />
  </div>
</main>
