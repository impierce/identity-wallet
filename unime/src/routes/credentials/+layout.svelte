<script lang="ts">
  // This layout is temporary and should be further up in the hierachy.
  // TODO Consolidation of layouts: https://github.com/impierce/identity-wallet/issues/230
  import { dev } from '$app/environment';

  import { pageTitleStore } from '$lib/stores';

  import TopNavBar from './TopNavBar.svelte';
</script>

<div class="relative h-screen">
  <TopNavBar title={$pageTitleStore} on:back={() => history.back()} />
  <div class="rest overflow-y-auto">
    <slot />
  </div>
</div>

<style>
  .rest {
    height: calc(100% - 48px - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }

  .rest::-webkit-scrollbar {
    display: none;
  }
</style>
