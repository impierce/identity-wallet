<script lang="ts">
  import ConnectionsList from '$lib/connections/ConnectionsList.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import Tabs from '$src/lib/components/molecules/navigation/Tabs.svelte';
  import History from '$src/lib/events/History.svelte';
</script>

<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div
    class="relative flex h-[50px] min-h-[50px] w-full items-center justify-center bg-silver text-[13px]/[24px] font-medium text-neutral-900 dark:bg-navy dark:text-white"
  >
    <p>{$LL.ACTIVITY.NAVBAR_TITLE()}</p>
  </div>
  <div class="flex grow flex-col overflow-y-auto px-4 pt-5">
    <Tabs triggers={[$LL.ACTIVITY.TABS.CONNECTIONS(), $LL.ACTIVITY.TABS.HISTORY()]}>
      <div slot="0" class="h-full">
        <ConnectionsList />
      </div>
      <div slot="1" class="h-full">
        <History />
      </div>
    </Tabs>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
