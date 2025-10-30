<script lang="ts">
  import type { LayoutData } from './$types';

  export let data: LayoutData;
</script>

{#key data.pathname}
  <div class="content-height bg-silver dark:bg-navy flex flex-col">
    <slot />
  </div>
{/key}

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
