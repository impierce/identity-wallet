<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import ConnectionData from '$lib/connections/ConnectionData.svelte';
  import ConnectionSummary from '$lib/connections/ConnectionSummary.svelte';
  import type { Connection } from '$lib/connections/types';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import ConnectionTabs from '$src/lib/components/molecules/navigation/tabs/ConnectionTabs.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import exampleConnections from '$src/lib/connections/mock-data.json';
  import History from '$src/lib/events/History.svelte';
  import { state } from '$src/stores';

  // let connection: Connection = exampleConnections.find((c) => c.id === $page.params.id)!!;
  console.log($page.params.id);
  console.log($state.connections);
  let connection: Connection = $state.connections.find((c) => c.id === $page.params.id)!!;
</script>

<div class="content-height flex flex-col">
  <TopNavBar on:back={() => goto('/activity')} title={connection.client_name} class="bg-silver dark:bg-navy" />
  <div class="grow bg-silver px-4 pt-5 dark:bg-navy">
    <ConnectionTabs>
      <!-- Summary -->
      <div slot="summary" class="h-full bg-silver py-5 dark:bg-navy">
        <div class="flex flex-col items-center justify-center space-y-4">
          <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
            <div class="flex h-[75px] w-[75px] rounded-3xl border bg-white p-2">
              <Image id={connection.id} iconClass="hidden" />
            </div>

            <div class="text-center text-2xl font-semibold text-slate-700 dark:text-grey">
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
        <History />
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
