<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { Connection } from '@bindings/connections/Connection';
  import { info } from '@tauri-apps/plugin-log';

  import { Image, ListItemCard } from '$lib/components';
  import { state } from '$lib/stores';
  import { formatDate } from '$lib/utils';

  import { groupConnectionsAlphabetically } from './utils';

  let connections: Map<string, Connection[]> = groupConnectionsAlphabetically($state.connections);
  info(`Connections: ${JSON.stringify(Array.from(connections.entries()))}`);
</script>

<div class="flex h-full flex-col">
  {#if connections.size === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.CONNECTION.SUMMARY.EMPTY()}</p>
    </div>
  {:else}
    <div class="space-y-3 py-5">
      {#each Object.entries(Object.fromEntries(connections)) as entry}
        <p class="w-full px-4 text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
          {entry[0]}
        </p>
        {#each entry[1] as connection}
          <ListItemCard
            id={connection.id}
            title={connection.name}
            description={connection.url}
            on:click={() => goto(`/activity/connection/${connection.id}`)}
          >
            <div
              slot="image"
              class="ml-2 mr-4 flex h-9 w-9 overflow-hidden rounded-full border-none border-slate-300 dark:border-slate-600"
            >
              <Image id={connection.id} imgClass="p-1">
                <div
                  slot="fallback"
                  class="h-full w-full rounded-full ring-1 ring-inset ring-slate-200 dark:bg-dark dark:ring-slate-600"
                />
              </Image>
            </div>
            <div slot="right" class="h-full pr-2 pt-1 text-[12px]/[20px] font-medium text-slate-400">
              {formatDate(connection.last_interacted, $state.profile_settings.locale)}
            </div>
          </ListItemCard>
        {/each}
      {/each}
    </div>
  {/if}
</div>
