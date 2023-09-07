<script lang="ts">
  import { ActivityTabs, MeltUiActivityTabs, SearchInput } from '@impierce/ui-components';

  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';
  import History from '$src/lib/history/History.svelte';

  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import SortAscending from '~icons/ph/sort-ascending-bold';
</script>

<div class="flex flex-col bg-silver dark:bg-navy">
  <div
    class="relative flex h-[50px] w-full items-center justify-center bg-silver text-[13px]/[24px] font-medium text-neutral-900 dark:bg-navy dark:text-white"
  >
    <p>Connected</p>
    <!-- TODO: comment out, if no functionality given -->
    <!-- Sort & Search -->
    <div class="absolute right-4 -mr-3">
      <button class="rounded-full p-3 pr-2">
        <SortAscending class="h-5 w-5" />
      </button>
      <button class="rounded-full p-3 pl-2">
        <MagnifyingGlass class="h-5 w-5" />
      </button>
    </div>
  </div>
  <div class="grow px-4 py-5">
    <MeltUiActivityTabs>
      <div slot="connections" class="h-full bg-silver pt-5 dark:bg-navy">
        <!-- Search & Sort -->
        <!-- <div class="flex pb-4">
        <div class="grow">
          <SearchInput placeholder="Search connections" />
        </div>
        <button class="ml-2 rounded-full p-2 hover:bg-slate-100"
          ><ArrowDownAZ class="text-slate-500" /></button
        >
      </div> -->
        <ConnectionsList />
      </div>
      <div slot="history" class="flex h-full items-center justify-center p-4">
        <History />
        <!-- <div class="font-medium text-slate-400">No history yet</div> -->
      </div>
    </MeltUiActivityTabs>
  </div>
</div>
