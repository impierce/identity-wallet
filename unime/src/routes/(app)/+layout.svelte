<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import { BottomNavBar } from '$lib/components';

  $: active = ($page.route.id?.split('/').at(2) ?? 'me') as 'me' | 'scan' | 'activity';
</script>

<div class="content-height">
  <div class="hide-scrollbar h-full overflow-y-auto pb-[64px]">
    <slot />
  </div>
  <div class="sticky bottom-[var(--safe-area-inset-bottom)] w-full shadow-[0_-4px_20px_0px_rgba(0,0,0,0.03)]">
    <BottomNavBar
      {active}
      on:me={() => goto('/me')}
      on:scan={() => goto('/scan')}
      on:activity={() => goto('/activity')}
    />
  </div>
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
