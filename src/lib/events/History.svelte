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

  let eventsList = [data_1, data_0];
</script>

<div class="relative flex h-full flex-col pr-4 pt-4">
  {#if events.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Coming soon ...</p>
    </div>
  {:else}
    <div class="flex grow flex-col space-y-8">
      {#each eventsList as event}
        <div class="flex justify-between">
          <div class="z-10 mr-3 h-6 w-6 overflow-hidden rounded-full bg-white ring-8 ring-silver">
            <img src="https://demo.ngdil.com/imgs/kw1c-white.png" alt="" class="h-full object-contain" />
          </div>
          <div class="grow">
            <HistoryEntry {...event} />
          </div>
        </div>
      {/each}
    </div>
    <!-- Timeline -->
    <div class="absolute left-3 top-4 h-full w-0.5 -translate-x-1/2 transform bg-slate-200"></div>
  {/if}
</div>
