<script lang="ts">
  import { TopNavigation, MeltUiConnectionTabs } from '@impierce/ui-components';
  import ConnectionSummary from '$lib/connections/ConnectionSummary.svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import exampleConnections from '$lib/example/data/connections.json';
  import type { Connection } from '$lib/connections/types';
  import ConnectionData from '$lib/connections/ConnectionData.svelte';
  import ConnectionHistory from '$lib/connections/ConnectionHistory.svelte';

  let connection: Connection = exampleConnections.find((c) => c.id === $page.params.id)!!;
</script>

<div class="flex h-full flex-col">
  <TopNavigation on:back={() => goto('/activity')} title={connection.displayName ?? connection.domain} />
  <div class="grow">
    <MeltUiConnectionTabs>
      <!-- Summary -->
      <div slot="summary" class="h-full p-5">
        <div class="flex flex-col items-center justify-center p-4 space-y-4">
          <div class="text-xl font-semibold text-slate-700 text-center">
            You are connected to <p class="text-violet-700">{connection.domain}</p>
          </div>
          <div class="w-fit rounded-lg border">
            <img src={'/tauri.svg'} alt={connection.domain} class="h-32 w-32 p-4" />
          </div>
          <ConnectionSummary {...connection} />
        </div>
      </div>

      <!-- Data -->
      <div slot="data" class="h-full p-5">
        <ConnectionData />
      </div>

      <!-- History -->
      <div slot="history" class="h-full p-5">
        <!-- TODO: If this turns out to be a costly operation (filtering in backend), consider lazy loading the component -->
        <ConnectionHistory />
      </div>
    </MeltUiConnectionTabs>
  </div>
</div>
