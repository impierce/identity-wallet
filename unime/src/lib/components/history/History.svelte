<script lang="ts">
  import type { Component } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import type { SVGAttributes } from 'svelte/elements';

  import type { HistoryCredential } from '@bindings/history/HistoryCredential';
  import type { HistoryEvent } from '@bindings/history/HistoryEvent';

  import { DownloadSimpleFillIcon, PlugsConnectedFillIcon, ShareFatFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { currentConnectionName } from '$lib/utils/issuer';

  import HistoryEntry from './HistoryEntry.svelte';

  export let connectionId: string | undefined = undefined;

  interface DisplayEvent {
    title: string;
    date: string;
    icon: Component<SVGAttributes<SVGSVGElement>>;
    credentials: HistoryCredential[];
  }

  let filteredEvents: HistoryEvent[];

  if (connectionId) {
    filteredEvents = $state.history.filter((his) => his.connection_id === connectionId);
  } else {
    filteredEvents = $state.history;
  }

  const events: DisplayEvent[] = filteredEvents.map((history) => {
    let title: string;
    const connectionName = currentConnectionName(history.connection_id, history.connection_name, $state.connections);
    let icon: Component<SVGAttributes<SVGSVGElement>>;

    let date = history.date;
    let credentials = history.credentials;

    switch (history.event_type) {
      case 'CredentialsAdded': {
        title = $LL.HISTORY.DATA_RECEIVED() + ' ' + connectionName;
        icon = DownloadSimpleFillIcon;
        break;
      }
      case 'CredentialsShared': {
        title = $LL.HISTORY.DATA_SHARED() + ' ' + connectionName;
        icon = ShareFatFillIcon;
        break;
      }
      case 'ConnectionAdded': {
        title = $LL.HISTORY.CONNECTION_ADDED() + ' ' + connectionName;
        icon = PlugsConnectedFillIcon;
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

<div class="relative flex h-full flex-col">
  {#if $state.history.length === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.HISTORY.EMPTY()}</p>
    </div>
  {:else}
    <div class="ml-2 pt-10">
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
              <div class="mt-4 mb-2 h-full w-[2px] bg-grey dark:bg-blue"></div>
            {/if}
          </div>
          <div class="mt-[-5px] ml-6 flex grow justify-between pb-10">
            <div class="grow">
              <HistoryEntry {...event} />
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
