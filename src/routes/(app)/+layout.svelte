<script lang="ts">
  import { BottomNavigation } from '@impierce/ui-components';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  $: active = $page.route.id?.split('/').at(2) ?? 'me';
</script>

<!-- <div class="h-[calc(100vh-125px)]"> -->
<div class="flex h-screen flex-col items-stretch">
  <div class="hide-scrollbar grow overflow-auto">
    <slot />
  </div>
  <div class="shrink-0">
  <!-- <div class="fixed bottom-[env(safe-area-inset-bottom)] w-full"> -->
    <!-- TODO: testing only, rather use env() -->
    <!-- <div class="fixed bottom-[var(--safe-area-inset-bottom)] w-full"> -->
      <BottomNavigation
        {active}
        on:me={() => goto('/me')}
        on:scan={() => goto('/scan')}
        on:activity={() => goto('/activity')}
      />
    <!-- </div> -->
  </div>

  <!-- safe-area -->
  <!-- <div
    class="fixed bottom-0 z-10 h-[env(safe-area-inset-bottom)] w-full bg-blue-100 dark:bg-slate-800"
  /> -->
  <!-- TODO: testing only, rather use env() -->
  <!-- <div
    class="fixed bottom-0 z-10 h-[var(--safe-area-inset-bottom)] w-full bg-blue-100 dark:bg-slate-800"
  /> -->
</div>
