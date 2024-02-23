<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import HistoryEntry from '$lib/events/HistoryEntry.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import exampleEvents from '$src/lib/events/mock-data.json';
  import { state } from '$src/stores';

  import DownloadSimple from '~icons/ph/download-simple';
  import PlugsConnected from '~icons/ph/plugs-connected';
  import ShareFat from '~icons/ph/share-fat';
  import type { HistoryCredential } from '@bindings/HistoryCredential';

  // const events: Event[] = exampleEvents.map((e) => ({ ...e, type: e.type as EventType }));

  interface DisplayEvent {
    title: string,
    date: string,
    icon: typeof SvelteComponent<SvelteHTMLElements['svg']>,
    credentials: Array<HistoryCredential>
  }

  const events: DisplayEvent[] = $state.history.map((history) => {
    let title: string;
    let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
    // TODO dynamic parsing Intl.
    let date = history.date;
    let credentials = history.credentials;

    switch (history.event_type) {
      case 'added_credentials': {
        title = 'Received credentials from ' + history.issuer_name;
        icon = DownloadSimple;
        break;
      }
      case 'shared_credentials': {
        title = 'Shared credentials with ' + history.issuer_name;
        icon = ShareFat;
        break;
      }
      case 'added_connection': {
        title = 'First connection with ' + history.issuer_name;
        icon = PlugsConnected;
        break;
      }
    }
    return {
      title,
      icon,
      date,
      credentials
    } as DisplayEvent;
  });
</script>

<div class="relative flex h-full flex-col">
  {#if $state.history.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.TIMELINE.EMPTY()}</p>
    </div>
  {:else}
    <div class="flex grow flex-col space-y-8 pr-4 pt-4">
      {#each events as event}
        <div class="flex justify-between">
          <div class="z-10 mr-3 h-6 w-6 overflow-hidden rounded-full bg-white p-0.5 ring-8 ring-silver">
            <svelte:component this={event.icon} class="h-full w-full object-contain" />
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
