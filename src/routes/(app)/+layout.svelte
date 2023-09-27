<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import BottomNavBar from '$src/lib/components/molecules/navigation/BottomNavBar.svelte';

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
</div>

<div class="safe-area-top bg-white dark:bg-dark" />
<div class="safe-area-bottom" />

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
