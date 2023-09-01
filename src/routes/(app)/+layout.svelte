<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { BottomNavBar } from '@impierce/ui-components';

  $: active = $page.route.id?.split('/').at(2) ?? 'me';
</script>

<!--
  safe-area-inset-top: 47px
  
  safe-area-inset-bottom: 34px
  ----------------------------
  total: 162px
-->
<div class="content-height flex flex-col items-stretch">
  <!-- <div class="flex h-screen flex-col items-stretch"> -->
  <div class="hide-scrollbar grow overflow-x-hidden overflow-y-scroll">
    <slot />
  </div>
  <div class="shrink-0">
    <div
      class="fixed bottom-[var(--safe-area-inset-bottom)] w-full shadow-[0_-4px_20px_0px_rgba(0,0,0,0.03)]"
    >
      <BottomNavBar
        {active}
        on:me={() => goto('/me')}
        on:scan={() => goto('/scan')}
        on:activity={() => goto('/activity')}
      />
    </div>
  </div>

  <div class="safe-area-top" />
  <div class="safe-area-bottom" />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 56px);
  }
</style>
