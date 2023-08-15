<script>
  import { TopNavigation } from '@impierce/ui-components';
  import { state } from '$src/stores';
  import { page } from '$app/stores';
  import { dispatch } from '$lib/dispatcher';
</script>

<div class="content-height flex flex-col items-stretch">
  <slot />

  <!-- DEBUG: Displays the current path at the bottom -->
  <div class="absolute bottom-2 left-2 rounded bg-orange-100 p-1 font-mono text-xs text-orange-600">
    {$page.url.pathname}
  </div>

  <div class="safe-area-top" />
  <div class="safe-area-bottom" />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
