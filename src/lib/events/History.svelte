<script lang="ts">
  import type { Event, EventType } from '$lib/events';
  import HistoryEntry from '$lib/events/HistoryEntry.svelte';
  import exampleEvents from '$src/lib/events/mock-data.json';
  import { state } from '$src/stores';

  const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));
  // const events: Event[] = [];

  const data_0 = {
    ...events[0],
    connection: {
      domain: 'example.com',
      id: '0',
      url: 'https://example.org',
      lastConnected: 'n/a',
    },
    title: 'Initial connection',
    timestamp: '2023-08-03T12:23:41.218Z',
    credentials: [],
  };

  const data_1 = {
    ...events[0],
    connection: {
      domain: 'example.com',
      id: '1',
      url: 'https://example.org',
      lastConnected: 'n/a',
    },
    title: 'Data shared',
    timestamp: '2023-08-03T12:23:42.749Z',
    credentials: [$state.credentials[0], $state.credentials[1]],
  };
</script>

<div class="flex h-full flex-col items-end pr-4">
  {#if events.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Coming soon ...</p>
    </div>
  {:else}
    <div class="flex w-3/4 flex-col space-y-8">
      <HistoryEntry {...data_1} />
      <HistoryEntry {...data_0} />
    </div>
  {/if}
</div>
