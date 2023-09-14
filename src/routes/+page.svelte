<script lang="ts">
  import UniMeLogoDark from '$src/lib/components/logo/UniMeLogoDark.svelte';
  import UniMeLogoLight from '$src/lib/components/logo/UniMeLogoLight.svelte';

  // TODO: make reactive? necessary even on initial page load? (splashscreen is static)
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');
</script>

<div class="content-height flex items-center justify-center bg-silver dark:bg-navy">
  {#if isDarkModeEnabled}
    <UniMeLogoDark />
  {:else}
    <UniMeLogoLight />
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
