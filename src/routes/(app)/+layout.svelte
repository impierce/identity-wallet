<script lang="ts">
  import { BottomNavigation } from '@impierce/ui-components';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  $: active = $page.route.id?.split('/').at(2) ?? 'me';

  const checkScanPrerequisites = () => {
    console.log('checkScanPrerequisites');
    // goto('/scan');
  };
</script>

<div class="h-[calc(100vh-125px)]">
  <slot />
  <div class="fixed bottom-[var(--safe-area-inset-bottom)] w-full">
    <BottomNavigation
      {active}
      on:me={() => goto('/me')}
      on:scan={() => checkScanPrerequisites()}
      on:activity={() => goto('/activity')}
    />
  </div>
  <!-- safe-area -->
  <div
    class="fixed bottom-0 z-10 h-[var(--safe-area-inset-bottom)] w-full bg-blue-100 dark:bg-slate-800"
  />
</div>
