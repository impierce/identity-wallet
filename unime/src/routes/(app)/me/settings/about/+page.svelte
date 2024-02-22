<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';

  import Heart from '~icons/ph/heart-fill';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.SUPPORT.ABOUT.NAVBAR_TITLE()} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col p-8">
    <!-- Footer -->
    <div
      class="flex flex-col items-center pt-4 text-[13px]/[24px] font-normal text-slate-500 opacity-50 dark:text-slate-300"
    >
      <!-- <div class="pb-4" /> -->
      <div>0.4.11</div>
      <div class="flex items-center pb-4">
        <p>{$LL.SETTINGS.SUPPORT.ABOUT.BUILT_WITH()}</p>
        <Heart class="pl-1" />
      </div>
      <div>Apache-2.0</div>
      <div>2023 Impierce Technologies</div>
    </div>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
