<script lang="ts">
  import { ActivityTabs, MeltUiActivityTabs, SearchInput } from '@impierce/ui-components';

  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';

  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z';
</script>

<div class="h-full bg-slate-100">
  <MeltUiActivityTabs>
    <div slot="connections" class="h-full p-5">
      <!-- Search & Sort -->
      <div class="flex pb-4">
        <div class="grow">
          <SearchInput placeholder="Search connections" />
        </div>
        <button class="ml-2 rounded-full p-2 hover:bg-slate-100"
          ><ArrowDownAZ class="text-slate-500" /></button
        >
      </div>
      <ConnectionsList />
    </div>
    <div slot="history" class="flex h-full items-center justify-center p-5">
      <div class="font-medium text-slate-400">No history yet</div>
    </div>
  </MeltUiActivityTabs>
</div>
