<script lang="ts">
  import { TopNavBar } from '$lib/components';
</script>

<TopNavBar on:back={() => history.back()} title="Backup and recovery" />

<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <!-- TODO Button with `KeyboardFillIcon` and `Your DID`. -->
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
