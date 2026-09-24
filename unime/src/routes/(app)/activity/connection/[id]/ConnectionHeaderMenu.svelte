<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import { DotsThreeVerticalBoldIcon, InfoRegularIcon, SignOutFillIcon } from '$lib/icons';

  const dispatch = createEventDispatcher<{ showDetails: void }>();

  const {
    elements: { trigger, menu, item, arrow },
    states: { open },
  } = createDropdownMenu({
    portal: '#portal',
    loop: true,
  });
</script>

<button
  type="button"
  class="-mr-4 rounded-full p-2"
  use:melt={$trigger}
  aria-label={$LL.CONNECTION.ACTIONS.MENU_LABEL()}
>
  <DotsThreeVerticalBoldIcon class="size-5" />
</button>

<div
  class="flex min-w-[160px] flex-col rounded-xl border border-slate-300 bg-background-alt p-1 shadow-md dark:border-slate-600"
  use:melt={$menu}
  transition:fly={{ duration: 150, y: -10 }}
>
  <button
    type="button"
    class="flex items-center space-x-2 rounded-lg py-2 pr-4 pl-3 text-slate-800 hover:bg-background dark:text-grey"
    use:melt={$item}
    on:click={() => {
      $open = false;
      dispatch('showDetails');
    }}
  >
    <InfoRegularIcon class="size-5" />
    <span class="grow text-left text-[13px]/[24px] font-medium">
      {$LL.CONNECTION.ACTIONS.SHOW_DETAILS()}
    </span>
  </button>

  <button
    type="button"
    class="flex items-center space-x-2 rounded-lg py-2 pr-4 pl-3 text-rose-500 disabled:cursor-not-allowed disabled:opacity-50"
    use:melt={$item}
    disabled
  >
    <SignOutFillIcon class="size-5" />
    <span class="grow text-left text-[13px]/[24px] font-medium">
      {$LL.CONNECTION.ACTIONS.DISCONNECT()}
    </span>
  </button>

  <div use:melt={$arrow} class="border border-r-0 border-b-0 border-slate-300 dark:border-slate-600"></div>
</div>
