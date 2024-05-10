<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import BottomNavItem from '$lib/components/molecules/navigation/BottomNavItem.svelte';
  import Activity from '$lib/static/svg/navbar-icons/Activity.svelte';
  import Me from '$lib/static/svg/navbar-icons/Me.svelte';
  import Scan from '$lib/static/svg/navbar-icons/Scan.svelte';

  const dispatch = createEventDispatcher();

  export let active: 'me' | 'scan' | 'activity' = 'me';
</script>

<div class="flex h-[64px] justify-evenly bg-white py-0 dark:bg-dark sm:justify-center sm:space-x-24">
  <!-- 1: Me -->
  <BottomNavItem label={$LL.ME.BOTTOM_NAVIGATION_TITLE()} active={active == 'me'} on:click={() => dispatch('me')}>
    <!-- Icons bring their own dimensions (height and width).  -->
    <Me slot="icon" class="h-6 w-6" />
  </BottomNavItem>

  <!-- 2: Scan -->
  <BottomNavItem label={$LL.SCAN.BOTTOM_NAVIGATION_TITLE()} active={active == 'scan'} on:click={() => dispatch('scan')}>
    <Scan slot="icon" class="h-6 w-6" />
  </BottomNavItem>

  <!-- 3: Activity -->
  <BottomNavItem
    label={$LL.ACTIVITY.BOTTOM_NAVIGATION_TITLE()}
    active={active == 'activity'}
    on:click={() => dispatch('activity')}
  >
    <Activity slot="icon" class="h-6 w-6" />
  </BottomNavItem>
</div>
