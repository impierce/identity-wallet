<script lang="ts">
  import { onDestroy } from 'svelte';

  import { page } from '$app/state';
  import { get } from 'svelte/store';

  import { isMockPrompt } from '$lib/dev/mocks/resolve';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState, error } from '$lib/stores';

  // This lives in the layout so that  navigating to a child route
  // does not cancel the flow.
  $: isMock = isMockPrompt(page.url, $appState);

  const unsubscribe = error.subscribe((err) => {
    if (err && !isMock) {
      dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
    }
  });

  onDestroy(() => {
    unsubscribe();
    if (isMock) return;
    if (get(appState).current_user_prompt?.type === 'accept-connection') {
      dispatch({ type: '[User Flow] Cancel', payload: {} });
    }
  });
</script>

<slot />
