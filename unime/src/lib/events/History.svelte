<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import type { HistoryCredential } from '@bindings/HistoryCredential';
  import type { HistoryEvent } from '@bindings/HistoryEvent';

  import HistoryEntry from '$lib/events/HistoryEntry.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import DownloadSimple from '~icons/ph/download-simple-fill';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import ShareFat from '~icons/ph/share-fat-fill';

  export let connectionId: string | undefined = undefined;

  interface DisplayEvent {
    title: string;
    date: string;
    icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
    credentials: Array<HistoryCredential>;
  }

  let filteredEvents: HistoryEvent[];

  if (connectionId) {
    filteredEvents = $state.history.filter((his) => his.connection_id === connectionId);
  } else {
    filteredEvents = $state.history;
  }

  const events: DisplayEvent[] = filteredEvents.map((history) => {
    let title: string;
    let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;

    let date = history.date;
    let credentials = history.credentials;

    switch (history.event_type) {
      case 'CredentialsAdded': {
        title = $LL.HISTORY.DATA_RECEIVED() + ' ' + history.connection_name;
        icon = DownloadSimple;
        break;
      }
      case 'CredentialsShared': {
        title = $LL.HISTORY.DATA_SHARED() + ' ' + history.connection_name;
        icon = ShareFat;
        break;
      }
      case 'ConnectionAdded': {
        title = $LL.HISTORY.CONNECTION_ADDED() + ' ' + history.connection_name;
        icon = PlugsConnected;
        break;
      }
    }

    return {
      title,
      icon,
      date,
      credentials,
    } as DisplayEvent;
  });

  function hasNextElement(i: number): boolean {
    return i + 1 < events.length;
  }
</script>

<div class="relative mt-6 flex h-full flex-col">
  {#if $state.history.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.HISTORY.EMPTY()}</p>
    </div>
  {:else}
    <div class="ml-2 mt-6">
      {#each events.reverse() as event, i}
        <div class="flex flex-row">
          <div class="mt-2 flex flex-col items-center">
            <div
              class="z-10 flex items-center justify-center rounded-full bg-grey text-slate-800 ring-8 ring-grey dark:bg-blue dark:text-grey dark:ring-blue"
            >
              <svelte:component this={event.icon} class="h-4 w-4 " />
            </div>
            {#if hasNextElement(i)}
              <!-- Vertical line on the left -->
              <div class="mb-2 mt-4 h-full w-[2px] bg-grey dark:bg-blue"></div>
            {/if}
          </div>
          <div class="ml-6 mt-[-5px] flex grow justify-between pb-10">
            <div class="grow">
              <HistoryEntry {...event} />
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
