<script lang="ts">
  import UniMeLogo from '$lib/static/svg/logo/UniMeLogo.svelte';
</script>

<div class="content-height bg-silver dark:bg-navy flex items-center justify-center">
  <UniMeLogo class="text-blue dark:text-silver" />
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
