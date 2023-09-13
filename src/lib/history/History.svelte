<script lang="ts">
  import exampleEvents from '$lib/example/data/events.json';
  import type { Event, EventType } from '$lib/example/model/event';
  import { state } from '$src/stores';

  import HistoryEntry from './HistoryEntry.svelte';

  // const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));
  const events: Event[] = [];

  const data_0 = {
    ...events[0],
    connection: {
      domain: 'example.com',
      id: '0',
      url: 'https://example.org',
      lastConnected: 'n/a'
    },
    title: 'Initial connection',
    credentials: []
  };

  const data_1 = {
    ...events[0],
    connection: {
      domain: 'example.com',
      id: '1',
      url: 'https://example.org',
      lastConnected: 'n/a'
    },
    title: 'Data shared',
    credentials: [$state.credentials[0]]
  };
</script>

<div class="flex h-full flex-col">
  {#if events.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        Coming soon ...
      </p>
    </div>
  {:else}
    <div class="flex w-3/4 flex-col space-y-8">
      <HistoryEntry {...data_1} />
      <HistoryEntry {...data_0} />
    </div>
  {/if}
</div>
