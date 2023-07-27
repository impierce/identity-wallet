<script lang="ts">
  import { BottomNavigation } from '@impierce/ui-components';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';

  $: active = $page.route.id?.split('/').at(2) ?? 'home';
</script>

<div class="h-[calc(100vh-100px)]">
  <slot />
  <div class="fixed bottom-[env(safe-area-inset-bottom)] w-full">
    <BottomNavigation
      {active}
      on:home={() => goto('/home')}
      on:data={() => goto('/data')}
      on:scan={() => goto('/scan')}
      on:connected={() => goto('/connected')}
      on:activity={() => goto('/activity')}
    />
  </div>
  <!-- safe-area -->
  <div
    class="fixed bottom-0 z-10 h-[env(safe-area-inset-bottom)] w-full bg-white dark:bg-slate-800"
  />
</div>
