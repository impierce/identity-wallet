<script lang="ts">
  import type { LayoutData } from './$types';

  export let data: LayoutData;
</script>

{#key data.pathname}
  <div class="content-height flex flex-col bg-silver dark:bg-navy">
    <slot />
  </div>
{/key}

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
