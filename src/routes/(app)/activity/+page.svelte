<script lang="ts">
  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';
  import { MeltUiActivityTabs, ActivityTabs, SearchInput } from '@impierce/ui-components';
  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z'
</script>

<div class="h-full bg-slate-100">
  <MeltUiActivityTabs>
    <div slot="connections" class="bg-white">
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
    <div slot="history">
      <div class="flex h-full items-center justify-center">No history yet</div>
    </div>
  </MeltUiActivityTabs>
</div>
