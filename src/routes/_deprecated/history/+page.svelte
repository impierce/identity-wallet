<script>
  import { goto } from '$app/navigation';
  // import { BottomNavigation } from '@impierce/ui-components';
  import { fly } from 'svelte/transition';
  import LL from '../../../i18n/i18n-svelte';
</script>

<div class="flex min-h-screen flex-col bg-slate-100 dark:bg-slate-700">
  <div
    class="flex w-screen grow flex-col justify-center space-y-8 p-8"
    in:fly={{ x: -32, opacity: 1 }}
  >
    <div class="select-none rounded-lg bg-slate-200 p-6 dark:bg-slate-800">
      <!-- <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p> -->
      <p class="text-center text-slate-400">{$LL.NO_HISTORY()}</p>
    </div>
  </div>
  <!-- Navigation -->
  <div class="safe-bottom fixed w-full">
    <!-- <BottomNavigation
      active="history"
      on:settings={() => goto('/settings')}
      on:profile={() => goto('/profile')}
    /> -->
  </div>
</div>

<style>
  .safe-bottom {
    bottom: env(safe-area-inset-bottom);
  }
</style>
