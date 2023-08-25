<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { MeltUiConnectionTabs, TopNavigation } from '@impierce/ui-components';

  import ConnectionData from '$lib/connections/ConnectionData.svelte';
  import ConnectionHistory from '$lib/connections/ConnectionHistory.svelte';
  import ConnectionSummary from '$lib/connections/ConnectionSummary.svelte';
  import type { Connection } from '$lib/connections/types';
  import exampleConnections from '$lib/example/data/connections.json';

  let connection: Connection = exampleConnections.find((c) => c.id === $page.params.id)!!;
</script>

<div class="flex h-full flex-col">
  <TopNavigation
    on:back={() => goto('/activity')}
    title={connection.displayName ?? connection.domain}
  />
  <div class="grow bg-neutral-100 px-4 pt-5">
    <MeltUiConnectionTabs>
      <!-- Summary -->
      <div slot="summary" class="h-full bg-neutral-100 py-5">
        <div class="flex flex-col items-center justify-center space-y-4">
          <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
            <div class="h-[75px] w-[75px] rounded-3xl border bg-white p-2">
              <img src={'/tauri.svg'} alt={connection.domain} />
            </div>

            <div class="text-center text-2xl font-semibold text-black">
              You are connected to <p class="text-primary">{connection.domain}</p>
            </div>
          </div>

          <ConnectionSummary {...connection} />
        </div>
      </div>

      <!-- Data -->
      <div slot="data" class="h-full bg-neutral-100 py-5">
        <ConnectionData />
      </div>

      <!-- History -->
      <div slot="history" class="h-full bg-neutral-100 py-5">
        <!-- TODO: If this turns out to be a costly operation (filtering in backend), consider lazy loading the component -->
        <ConnectionHistory />
      </div>
    </MeltUiConnectionTabs>
  </div>
</div>
