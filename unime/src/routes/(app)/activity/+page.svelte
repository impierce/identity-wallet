<script lang="ts">
  import { beforeNavigate, goto, replaceState } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';
  import { writable, type Writable } from 'svelte/store';
  import { fly } from 'svelte/transition';

  import { History, Tabs } from '$lib/components';

  import ConnectionsList from './ConnectionsList.svelte';

  let triggers = [$LL.ACTIVITY.TABS.CONNECTIONS(), $LL.ACTIVITY.TABS.HISTORY()];
  let activeTab: Writable<string> = writable(page.state.tab || triggers[0]);

  beforeNavigate(async ({ type, cancel }) => {
    replaceState('', { tab: $activeTab });
    if (type === 'popstate') {
      cancel();
      goto('/me');
    }
  });
</script>

<div class="content-height flex flex-col bg-silver dark:bg-navy" in:fly={{ y: 18, duration: 200, opacity: 1 }}>
  <div
    class="relative flex h-[50px] min-h-[50px] w-full items-center justify-center bg-silver text-[13px]/[24px] font-medium text-neutral-900 dark:bg-navy dark:text-white"
  >
    <p>{$LL.ACTIVITY.NAVBAR_TITLE()}</p>
  </div>
  <div class="flex grow flex-col overflow-y-auto px-4 pt-5">
    <Tabs value={activeTab} {triggers}>
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
