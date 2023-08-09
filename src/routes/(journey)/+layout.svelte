<script>
  import { TopNavigation } from '@impierce/ui-components';
  import { state } from '../../stores';
  import { page } from '$app/stores';
  import { dispatch } from '$lib/dispatcher';
</script>

<div class="content-height flex flex-col items-stretch">
  <slot />

  <!-- DEBUG: Displays the current path at the bottom -->
  <div class="absolute bottom-2 left-2 rounded bg-orange-100 p-1 font-mono text-xs text-orange-600">
    {$page.url.pathname}
  </div>

  <!-- safe-area -->
  <div class="fixed top-0 z-50 h-[var(--safe-area-inset-top)] w-full bg-white opacity-80" />
  <div
    class="fixed bottom-0 z-10 h-[var(--safe-area-inset-bottom)] w-full bg-neutral-100 opacity-80 dark:bg-slate-800"
  />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
