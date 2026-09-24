<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import { ActionSheet, Button } from '$lib/components';
  import { dispatch as dispatchAction } from '$lib/dispatcher';
  import { DotsThreeVerticalBoldIcon, InfoRegularIcon, SignOutFillIcon } from '$lib/icons';

  const dispatch = createEventDispatcher<{ showDetails: void }>();

  export let id: string;

  const {
    elements: { trigger, menu, item, arrow },
    states: { open },
  } = createDropdownMenu({
    portal: '#portal',
    loop: true,
  });

  async function disconnect() {
    await goto('/activity', { replaceState: true });
    await dispatchAction({ type: '[Connection] Delete', payload: { id } });
  }
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

  <ActionSheet
    titleText={$LL.CONNECTION.ACTIONS.DISCONNECT_TITLE()}
    descriptionText={$LL.CONNECTION.ACTIONS.DISCONNECT_DESCRIPTION()}
  >
    <button
      slot="trigger"
      let:trigger
      type="button"
      class="flex items-center space-x-2 rounded-lg py-2 pr-4 pl-3 text-rose-500 hover:bg-background"
      use:melt={$item}
      use:melt={trigger}
    >
      <SignOutFillIcon class="size-5" />
      <span class="grow text-left text-[13px]/[24px] font-medium">
        {$LL.CONNECTION.ACTIONS.DISCONNECT()}
      </span>
    </button>

    <div slot="content" class="w-full pt-[20px] pb-[10px]">
      <button
        type="button"
        class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
        on:click={disconnect}
      >
        {$LL.CONNECTION.ACTIONS.DISCONNECT_CONFIRM_BUTTON()}
      </button>
    </div>

    <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.CANCEL()} />
  </ActionSheet>

  <div use:melt={$arrow} class="border border-r-0 border-b-0 border-slate-300 dark:border-slate-600"></div>
</div>
