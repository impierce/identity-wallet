<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import ConnectionData from '$lib/connections/ConnectionData.svelte';
  import ConnectionSummary from '$lib/connections/ConnectionSummary.svelte';
  import type { Connection } from '@bindings/Connection';
  import LL from '$src/i18n/i18n-svelte';
  import Tabs from '$src/lib/components/molecules/navigation/Tabs.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import History from '$src/lib/events/History.svelte';
  import { state } from '$src/stores';

  console.log($page.params.id);
  console.log($state.connections);
  let connection: Connection = $state.connections.find((c) => c.id === $page.params.id)!!;
</script>

<div class="content-height flex flex-col">
  <TopNavBar on:back={() => goto('/activity')} title={connection.name} class="bg-silver dark:bg-navy" />
  <div class="flex grow flex-col overflow-y-auto bg-silver px-4 py-5 dark:bg-navy">
    <Tabs triggers={[$LL.CONNECTION.TABS.SUMMARY(), $LL.CONNECTION.TABS.DATA(), $LL.CONNECTION.TABS.ACTIVITY()]}>
      <div slot="0" class="h-full pt-5">
        <ConnectionSummary {connection} />
      </div>
      <div slot="1" class="h-full bg-silver py-5 dark:bg-navy">
        <ConnectionData id={connection.id} />
      </div>

      <div slot="2" class="bg-silver pt-5 dark:bg-navy">
        <!-- TODO: If this turns out to be a costly operation (filtering in backend), consider lazy loading the component -->
        <History connectionId={connection.id} />
      </div>
    </Tabs>
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
