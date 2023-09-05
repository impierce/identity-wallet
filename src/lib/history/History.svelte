<script lang="ts">
  import exampleEvents from '$lib/example/data/events.json';
  import type { Event, EventType } from '$lib/example/model/event';
  import { state } from '$src/stores';

  import HistoryEntry from './HistoryEntry.svelte';

  const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));

  const data = {
    ...events[0],
    connection: {
      domain: 'example.com',
      id: '0',
      url: 'https://example.org',
      lastConnected: 'n/a'
    },
    title: 'Initial connection',
    credentials: [$state.credentials.at(0)]
  };
</script>

<div class="flex flex-col space-y-8">
  <HistoryEntry {...data} />
  <HistoryEntry {...data} />
</div>
