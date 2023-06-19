<script>
  import { goto } from '$app/navigation';
  import { BottomNavigation } from '@impierce/ui-components';
  import { fly } from 'svelte/transition';
  import LL from '../../i18n/i18n-svelte';
</script>

<div class="flex flex-col min-h-screen">
  <div
    class="grow flex w-screen flex-col justify-center space-y-8 p-8"
    in:fly={{ x: -32, opacity: 1 }}
  >
    <div class="rounded-lg bg-slate-200 p-6">
      <!-- <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p> -->
      <p class="text-center text-slate-400">{$LL.NO_HISTORY()}</p>
    </div>
  </div>
  <div class="sticky top-[100vh]">
    <BottomNavigation
      active="history"
      on:settings={() => goto('/settings')}
      on:profile={() => goto('/profile')}
    />
  </div>
</div>
