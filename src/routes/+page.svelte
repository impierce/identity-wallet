<script lang="ts">
  import UniMeLogo from '$src/lib/components/logo/UniMeLogo.svelte';
</script>

<div class="content-height flex items-center justify-center bg-silver dark:bg-navy">
  <UniMeLogo />
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
