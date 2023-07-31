<script lang="ts">
  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';
  import { Input, MeltUiTabs, ActivityTabs } from '@impierce/ui-components';
</script>

<div class="h-full bg-violet-400">
  <MeltUiTabs>
    <div slot="connections" class="bg-white">
      <Input placeholder="Search connections" class="mb-4" />
      <ConnectionsList />
    </div>
    <div slot="history">
      <div class="flex h-full items-center justify-center">No history yet</div>
    </div>
  </MeltUiTabs>
</div>
