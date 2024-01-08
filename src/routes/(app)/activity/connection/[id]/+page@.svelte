<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import ConnectionData from '$lib/connections/ConnectionData.svelte';
  import ConnectionHistory from '$lib/connections/ConnectionHistory.svelte';
  import ConnectionSummary from '$lib/connections/ConnectionSummary.svelte';
  import type { Connection } from '$lib/connections/types';
  import ConnectionTabs from '$src/lib/components/molecules/navigation/tabs/ConnectionTabs.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import exampleConnections from '$src/lib/connections/mock-data.json';
  import { state } from '$src/stores';

  // let connection: Connection = exampleConnections.find((c) => c.id === $page.params.id)!!;
  console.log($page.params.id);
  console.log($state.connections);
  let connection: Connection = $state.connections.at($page.params.id)!!;
</script>

<div class="content-height flex flex-col">
  <TopNavigation on:back={() => goto('/activity')} title={connection.client_name} class="bg-silver dark:bg-navy" />
  <div class="grow bg-silver px-4 pt-5 dark:bg-navy">
    <ConnectionTabs>
      <!-- Summary -->
      <div slot="summary" class="h-full bg-silver py-5 dark:bg-navy">
        <div class="flex flex-col items-center justify-center space-y-4">
          <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
            <div class="flex h-[75px] w-[75px] rounded-3xl border bg-white p-2">
              <img src={connection.logo_uri} />
            </div>

            <div class="text-center text-2xl font-semibold text-black dark:text-white">
              Connected to <p class="text-primary">{connection.client_name}</p>
            </div>
          </div>

          <ConnectionSummary {...connection} />
        </div>
      </div>

      <!-- Data -->
      <div slot="data" class="h-full bg-silver py-5 dark:bg-navy">
        <ConnectionData />
      </div>

      <!-- History -->
      <div slot="activity" class="h-full bg-silver py-5 dark:bg-navy">
        <!-- TODO: If this turns out to be a costly operation (filtering in backend), consider lazy loading the component -->
        <ConnectionHistory />
      </div>
    </ConnectionTabs>
  </div>
</div>

<div class="safe-area-top bg-silver dark:bg-navy" />
<div class="safe-area-bottom bg-silver dark:bg-navy" />

<!-- 
  TODO: remove bottom bar. General rule: only show it in the top level, when navigation one level down, do not show bottom nav anymore
  Exception: in settings, we keep the bottom nav, because we have a lot of levels there
  -->
<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
