<script lang="ts">
  import { goto } from '$app/navigation';

  import exampleConnections from '$lib/example/data/connections.json';

  import ChevronRight from '~icons/lucide/chevron-right';

  import type { Connection } from './types';

  let connections: Connection[] = exampleConnections;
</script>

<div class="flex flex-col space-y-3">
  <p class="w-full px-4 text-[14px]/[22px] font-medium text-slate-600">A</p>
  {#each connections as connection}
    <button on:click={() => goto(`/activity/connection/${connection.id}`)}>
      <div class="flex h-16 items-center rounded-xl bg-white px-4">
        <!-- Icon -->
        <div class="mr-4 h-8 w-8 rounded-full border" />
        <!-- Text -->
        <div class="flex grow flex-col items-start">
          <div class="text-[13px]/[24px] font-medium text-[#1D1B20]">
            {connection.displayName ?? connection.domain}
          </div>
          <div class="text-[12px]/[20px] font-medium text-[#94A3B8]">
            {connection.lastConnected}
          </div>
        </div>
        <!-- <ChevronRight /> -->
      </div>
    </button>
  {/each}
</div>
