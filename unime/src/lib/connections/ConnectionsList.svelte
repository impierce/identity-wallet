<script lang="ts">
  import { goto } from '$app/navigation';

  import Image from '$lib/components/atoms/Image.svelte';
  import exampleConnections from '$lib/connections/mock-data.json';
  import LL from '$src/i18n/i18n-svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';

  import type { Connection } from './types';
  import { groupConnectionsAlphabetically } from './utils';

  let connections: Map<string, Connection[]> = groupConnectionsAlphabetically($state.connections);
  console.log(connections);
</script>

<div class="flex h-full flex-col space-y-3">
  {#if connections.size === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.CONNECTION.SUMMARY.EMPTY()}</p>
    </div>
  {/if}
  {#each Object.entries(Object.fromEntries(connections)) as entry}
    <p class="w-full px-4 text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {entry[0]}
    </p>
    {#each entry[1] as connection}
      <!-- <button on:click={() => goto(`/activity/connection/${connection.id}`)}>
        <div class="flex h-[64px] items-center rounded-xl bg-white px-4 dark:bg-dark">
          <div
            class="mr-4 flex h-9 w-9 overflow-hidden rounded-full border-none border-slate-300 dark:border-slate-600"
          >
            <div class="flex h-full w-full items-center bg-white">
              <Image id={connection.id} imgClass="p-1">
                <div
                  slot="fallback"
                  class="h-full w-full rounded-full ring-1 ring-inset ring-slate-200 dark:bg-dark dark:ring-slate-600"
                />
              </Image>
            </div>
          </div>

          <div class="flex grow flex-col items-start">
            <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {connection.client_name}
            </div>
            <div class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
              {connection.url}
            </div>
          </div>
        </div>
      </button> -->

      <ListItemCard
        id={connection.id}
        title={connection.client_name}
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
              class="dark:bg-dark h-full w-full rounded-full ring-1 ring-inset ring-slate-200 dark:ring-slate-600"
            />
          </Image>
        </div>
        <div slot="right" class="h-full pr-2 pt-1 text-[12px]/[20px] font-medium text-slate-400">
          {new Date().toLocaleString($state.locale, {
            dateStyle: 'short',
            timeStyle: 'short',
          })}
        </div>
      </ListItemCard>
    {/each}
  {/each}
</div>
