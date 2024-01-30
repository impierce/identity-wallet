<script lang="ts">
  import { page } from '$app/stores';

  import { dispatch } from '$lib/dispatcher';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';
</script>

<!-- TODO: refactor routes/(journey) to routes/journey/[id]/goals/[id] as there could be multiple journeys -->
<div class="content-height flex flex-col items-stretch">
  <slot />

  <!-- DEBUG: Displays the current path at the bottom -->
  <div class="absolute bottom-2 left-2 rounded bg-orange-100 p-1 font-mono text-xs text-orange-600">
    {$page.url.pathname}
  </div>

  <div class="safe-area-top bg-silver dark:bg-navy" />
  <div class="safe-area-bottom" />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
