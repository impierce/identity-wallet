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
    <!-- Currently, there is no better way of applying conditional Tailwind classes to a named slot inside a component. -->
    <!-- <User
        slot="icon"
        class={`h-6 w-6 ${
          active == 'me' ? 'text-black dark:text-primary' : 'text-slate-400 dark:text-white'
        }`}
      /> -->
    <Me
      slot="icon"
      class={`h-6 w-6 ${active == 'me' ? 'text-slate-800 dark:text-grey' : 'text-slate-300 dark:text-slate-500'}`}
    />
  </BottomNavItem>

  <!-- 2: Scan -->
  <BottomNavItem label={$LL.SCAN.BOTTOM_NAVIGATION_TITLE()} active={active == 'scan'} on:click={() => dispatch('scan')}>
    <Scan
      slot="icon"
      class={`h-6 w-6 ${active == 'scan' ? 'text-slate-800 dark:text-grey' : 'text-slate-300 dark:text-slate-500'}`}
    />
  </BottomNavItem>

  <!-- 3: Activity -->
  <BottomNavItem
    label={$LL.ACTIVITY.BOTTOM_NAVIGATION_TITLE()}
    active={active == 'activity'}
    on:click={() => dispatch('activity')}
  >
    <Activity
      slot="icon"
      class={`h-6 w-6 ${active == 'activity' ? 'text-slate-800 dark:text-grey' : 'text-slate-300 dark:text-slate-500'}`}
    />
  </BottomNavItem>
</div>
