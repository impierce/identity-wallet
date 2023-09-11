<script lang="ts">
  import { goto } from '$app/navigation';

  import exampleConnections from '$lib/example/data/connections.json';

  import ChevronRight from '~icons/lucide/chevron-right';

  import type { Connection } from './types';
  import { groupConnectionsAlphabetically } from './utils';

  // let connections: Map<string, Connection[]> = groupConnectionsAlphabetically(exampleConnections);
  let connections: Map<string, Connection[]> = groupConnectionsAlphabetically([]);
  console.log(Object.fromEntries(connections));
</script>

<div class="flex h-full flex-col space-y-3">
  {#if connections.size === 0}
    <div class="flex h-full flex-col items-center justify-center">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-white">
        No connections yet.
      </p>
    </div>
  {/if}
  {#each Object.entries(Object.fromEntries(connections)) as entry}
    <p class="w-full px-4 text-[14px]/[22px] font-medium text-slate-600 dark:text-white">
      {entry[0]}
    </p>
    {#each entry[1] as connection}
      <button on:click={() => goto(`/activity/connection/${connection.id}`)}>
        <div class="flex h-16 items-center rounded-xl bg-white px-4 dark:bg-dark">
          <!-- Icon -->
          <div class="mr-4 h-8 w-8 rounded-full border border-slate-300 dark:border-slate-600" />
          <!-- Text -->
          <div class="flex grow flex-col items-start">
            <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {connection.displayName ?? connection.url}
            </div>
            <div class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
              {connection.lastConnected}
            </div>
          </div>
        </div>
      </button>
    {/each}
  {/each}
</div>
