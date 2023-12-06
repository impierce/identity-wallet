<script lang="ts">
  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';
  import ActivityTabs from '$src/lib/components/molecules/tabs/ActivityTabs.svelte';
  import History from '$src/lib/history/History.svelte';

  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-bold';
  import SortAscending from '~icons/ph/sort-ascending-bold';
  import Search from '$src/lib/components/Search.svelte';
  import Sort from '$src/lib/components/Sort.svelte';
  import SortModal from '$src/lib/components/SortModal.svelte';
</script>

<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div
    class="relative flex h-[50px] min-h-[50px] w-full items-center justify-center bg-silver text-[13px]/[24px] font-medium text-neutral-900 dark:bg-navy dark:text-white"
  >
    <p>Connected</p>
    <!-- TODO: feature disabled: "Sort & Search" -->
    <!-- <div class="absolute right-4 -mr-3">
      <button class="rounded-full p-3 pr-2">
        <SortAscending class="h-5 w-5" />
      </button>
      <button class="rounded-full p-3 pl-2">
        <MagnifyingGlass class="h-5 w-5" />
      </button>
    </div> -->

    <div class="absolute right-5 py-4">
      <Sort></Sort>
    </div>
  </div>

  <div class="grow overflow-y-auto px-4 py-5">
    <ActivityTabs>
      <div slot="connections" class="h-full pt-5">
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
      <div slot="history" class="h-full pt-5">
        <History />
        <!-- <div class="font-medium text-slate-400">No history yet</div> -->
      </div>
    </ActivityTabs>
  </div>
  <SortModal></SortModal>
</div>

<!-- TODO: bug (styling): not applied due to parent layout -->
<div class="safe-area-top bg-silver dark:bg-navy" />

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
