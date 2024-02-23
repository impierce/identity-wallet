<script lang="ts">
  import Image from '$lib/components/atoms/Image.svelte';
  import type { Event, EventType } from '$lib/events';
  import HistoryEntry from '$lib/events/HistoryEntry.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import exampleEvents from '$src/lib/events/mock-data.json';
  import { state } from '$src/stores';

  // const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));
  const events: Event[] = [];

  const data_0 = {
    ...events[0],
    connection: {
      domain: 'kw1c.nl',
      id: 'kw1c',
      url: 'https://kw1c.nl',
      lastConnected: 'n/a',
    },
    title: 'Initial connection',
    timestamp: '2023-08-03T12:23:41.218Z',
    credentials: [],
  };

  const data_1 = {
    ...events[0],
    connection: {
      domain: 'impierce.com',
      id: 'impierce',
      url: 'https://impierce.com',
      lastConnected: 'n/a',
    },
    title: 'Data shared',
    timestamp: '2023-08-03T12:23:42.749Z',
    credentials: [$state.credentials[0], $state.credentials[1]],
  };

  const data_2 = {
    connection: {
      id: 'iota',
    },
    title: 'Initial connection',
    timestamp: '2024-01-09T11:53:53.937+00:00',
    credentials: [$state.credentials[2]],
  };

  let eventsList = [data_1, data_0, data_2];
</script>

<div class="relative flex h-full flex-col">
  {#if events.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.TIMELINE.EMPTY()}</p>
    </div>
  {:else}
    <div class="flex grow flex-col space-y-8 pr-4 pt-4">
      {#each eventsList as event}
        <div class="flex justify-between">
          <div class="z-10 mr-3 h-6 w-6 overflow-hidden rounded-full bg-white p-0.5 ring-8 ring-silver">
            <Image id={event.connection.id} imgClass="h-full w-full" />
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
