<script lang="ts">
  import { onDestroy } from 'svelte';

  import { page } from '$app/state';

  import { dispatch } from '$lib/dispatcher';
  import { state as appState, error } from '$lib/stores';

  // This lives in the layout so that  navigating to a child route
  // does not cancel the flow.
  $: isMock = $appState.dev_mode !== 'Off' && page.url.searchParams.has('mock');

  const unsubscribe = error.subscribe((err) => {
    if (err && !isMock) {
      dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
    }
  });

  onDestroy(() => {
    unsubscribe();
    // TODO: is onDestroy also called when user accepts since the component itself is destroyed?
    if (!isMock) dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

<slot />
