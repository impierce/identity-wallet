<script lang="ts">
  import exampleEvents from '$lib/example/data/events.json';
  import type { Event, EventType } from '$lib/example/model/event';
  import CredentialOffer from '$lib/history/events/CredentialOffer.svelte';
  import InitialConnection from '$lib/history/events/InitialConnection.svelte';
  import Login from '$lib/history/events/Login.svelte';

  import Download from '~icons/lucide/download';
  import HelpCircle from '~icons/lucide/help-circle';
  import KeyRound from '~icons/lucide/key-round';
  import Sprout from '~icons/lucide/sprout';

  // const events: Event[] = exampleEvents
  //   .map((e) => ({ ...e, type: e.type as EventType })) // explicit type cast
  //   .map((e) => ({
  //     ...e,
  //     timestamp: new Date(e.timestamp).toLocaleString('en-US', {
  //       dateStyle: 'medium',
  //       timeStyle: 'medium'
  //     })
  //   }))
  //   .reverse(); // newest on top

  const events: Event[] = [];
</script>

<div class="flex h-full flex-col space-y-8">
  {#if events.length > 0}
    {#each events as event}
      <div class="flex items-start">
        <!-- Dot (type) -->
        <div class="z-10 mr-5 rounded-full bg-slate-200 p-3 ring-4 ring-silver dark:ring-navy">
          {#if event.type === 'initial_connection'}
            <Sprout class="h-5 w-5 text-slate-500" />
          {:else if event.type === 'credential_offer'}
            <Download class="h-5 w-5 text-slate-500" />
          {:else if event.type === 'login'}
            <KeyRound class="h-5 w-5 text-slate-500" />
          {:else}
            <HelpCircle class="h-5 w-5 text-slate-500" />
          {/if}
        </div>

        <!-- Line -->
        <!-- TODO: refactor! calculated top (line start): Navbar (56px) + Tabs (44px) -->
        <div class="top-[calc(56px + 44px)] fixed left-[36px] h-full w-[4px] bg-slate-200" />

        <!-- Event -->
        <div class="w-full">
          {#if event.type === 'initial_connection'}
            <InitialConnection data={event.data} />
          {:else if event.type === 'credential_offer'}
            <CredentialOffer data={event.data} />
          {:else if event.type === 'login'}
            <Login data={event.data} />
          {:else}
            <div class="flex flex-col bg-slate-200 px-4 py-2">
              <span class="text-right font-mono text-sm text-slate-400">{event.type}</span>
              <span class="break-all text-slate-600">{JSON.stringify(event.data)}</span>
              <span class="text-right font-mono text-sm text-slate-400">{event.timestamp}</span>
            </div>
          {/if}

          <!-- Timestamp -->
          <div class="text-right text-sm font-medium text-slate-400">{event.timestamp}</div>
        </div>
      </div>
    {/each}
  {:else}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Coming soon ...</p>
    </div>
  {/if}
</div>
