<script lang="ts">
import exampleEvents from '$lib/example/data/events.json';
  import type { Event, EventType } from '$lib/example/model/event';
  import InitialConnection from '$lib/history/events/InitialConnection.svelte';
  import CredentialOffer from '$lib/history/events/CredentialOffer.svelte';

  const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));
</script>

<div class="flex flex-col space-y-2">
  {#each events as event}
    {#if event.type === 'initial_connection'}
      <InitialConnection data={event.data} />
    {:else if event.type === 'credential_offer'}
      <CredentialOffer data={event.data} />
    {:else}
      <div class="flex flex-col bg-slate-200 px-4 py-2">
        <span class="text-right font-mono text-sm text-slate-400">{event.type}</span>
        <span class="break-all text-slate-600">{JSON.stringify(event.data)}</span>
        <span class="text-right font-mono text-sm text-slate-400">{event.timestamp}</span>
      </div>
    {/if}
  {/each}
</div>
