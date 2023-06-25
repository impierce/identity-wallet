<script>
  import { goto } from '$app/navigation';
  import { BottomNavigation } from '@impierce/ui-components';
  import { fly } from 'svelte/transition';
  import LL from '../../i18n/i18n-svelte';
</script>

<div class="flex min-h-screen flex-col">
  <div
    class="flex w-screen grow flex-col justify-center space-y-8 p-8"
    in:fly={{ x: -32, opacity: 1 }}
  >
    <div class="rounded-lg bg-slate-200 p-6">
      <!-- <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p> -->
      <p class="text-center text-slate-400">{$LL.NO_HISTORY()}</p>
    </div>
  </div>
  <div class="absolute w-full bottom-[20px]">
    <BottomNavigation
      active="history"
      on:settings={() => goto('/settings')}
      on:profile={() => goto('/profile')}
    />
  </div>
</div>
