<script>
  import '../app.css';

  import { fly } from 'svelte/transition';
  import { ChevronUp, ChevronDown } from 'svelte-heros-v2';
  import { state } from '../stores';
  import { onMount } from 'svelte';
  import { loadAllLocales } from '../i18n/i18n-util.sync';
  import { dispatch } from '$lib/dispatcher';

  onMount(async () => {
    console.log('+layout.svelte: onMount');
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = false;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('state', $state);
  }
</script>

<main class="h-screen bg-slate-100">
  {#if showDevMode}
    <div
      class="hide-scrollbar fixed z-10 flex space-x-4 overflow-x-auto bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
      in:fly={{ y: -64, opacity: 1 }}
      out:fly={{ y: -64, opacity: 1 }}
    >
      <div class="flex-shrink-0 px-4 py-1 font-medium text-red-700">dev mode</div>
      <button
        class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        >back</button
      >
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[App] Reset' })}>reset</button
      >
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[DEV] Load profile' })}>load dev profile</button
      >
    </div>
  {/if}
  <button
    class="fixed right-3 top-3 z-10 rounded-full p-2 hover:bg-red-200 {showDevMode
      ? 'bg-red-200'
      : ''}"
    on:click={() => (showDevMode = !showDevMode)}
  >
    {#if showDevMode}
      <ChevronUp class="text-red-700" strokeWidth="2" />
    {:else}
      <ChevronDown class="text-red-700" strokeWidth="2" />
    {/if}
  </button>
  <!-- end: dev mode -->
  <div class="h-auto">
    <!-- <Route path="welcome" component={Welcome} /> -->
    <!-- <Route path="profile" component={Profile} primary={false} /> -->
    <slot />
  </div>
</main>
