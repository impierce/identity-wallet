<script lang="ts">
  import { goto } from '$app/navigation';
  import ChevronRight from '~icons/lucide/chevron-right';
  import type { Connection } from './types';
  import exampleConnections from '$lib/example/data/connections.json';

  let connections: Connection[] = exampleConnections;
</script>

<div class="flex flex-col space-y-4">
  {#each connections as connection}
    <button on:click={() => goto(`/activity/connection/${connection.id}`)}>
      <div class="flex items-center justify-between rounded bg-slate-100 px-4 py-2">
        <div class="flex flex-col items-start">
          <div class="font-semibold text-slate-600">{connection.displayName ?? connection.domain}</div>
          <div class="text-sm font-light italic text-slate-400">
            {connection.lastConnected}
          </div>
        </div>
        <ChevronRight />
      </div>
    </button>
  {/each}
</div>
