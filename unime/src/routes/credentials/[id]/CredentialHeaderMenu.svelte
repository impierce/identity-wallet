<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import Icon from '@iconify/svelte';
  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import { ActionSheet, Button } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { DotsThreeVerticalBoldIcon, TrashFillIcon } from '$lib/icons';

  const dispatchEvent = createEventDispatcher();

  export let id: string;

  const {
    elements: { trigger, menu, item, arrow },
    // TODO: see comment below
    states: { open },
  } = createDropdownMenu({
    portal: '#portal',
    // forceVisible: true,
    loop: true,
  });
</script>

<button type="button" class="trigger p-0.5" use:melt={$trigger} aria-label="Open credential menu">
  <DotsThreeVerticalBoldIcon class="h-8 w-8 dark:text-white" />
</button>

<!-- TODO: Problem when using $open: when the dropdown menu is closed, it closes the contained ActionSheet as well. -->
<!-- {#if $open} -->
<div
  class="flex min-w-[160px] flex-col rounded-xl border border-slate-300 bg-background-alt p-1 shadow-md dark:border-slate-600"
  use:melt={$menu}
  transition:fly={{ duration: 150, y: -10 }}
>
  <!-- Edit title -->
  <button
    on:click={() => {
      dispatchEvent('edit');
      $open = false;
    }}
    class="flex items-center space-x-2 rounded-lg py-2 pl-3 pr-4 hover:bg-background dark:text-grey"
  >
    <Icon class="size-5" icon="ph:pencil-fill" />
    <p class="grow text-left text-[13px]/[24px] font-medium">
      {$LL.CREDENTIAL.ACTIONS.EDIT.MENU_BUTTON()}
    </p>
  </button>

  <!-- Delete credential -->
  <ActionSheet
    titleText={$LL.CREDENTIAL.ACTIONS.DELETE.TITLE()}
    descriptionText={$LL.CREDENTIAL.ACTIONS.DELETE.DESCRIPTION()}
  >
    <button
      slot="trigger"
      let:trigger
      class="flex items-center space-x-2 rounded-lg py-2 pl-3 pr-4 hover:bg-background"
      use:melt={$item}
      use:melt={trigger}
    >
      <svelte:component this={TrashFillIcon} class="h-5 w-5 text-rose-500" />
      <p class="grow text-left text-[13px]/[24px] font-medium dark:text-grey">
        {$LL.CREDENTIAL.ACTIONS.DELETE.MENU_BUTTON()}
      </p>
    </button>

    <!-- Yes, delete the credential -->
    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <button
        class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
        on:click={() => dispatch({ type: '[Credential] Delete', payload: { id } })}
      >
        {$LL.CREDENTIAL.ACTIONS.DELETE.CONFIRM_BUTTON()}
      </button>
    </div>

    <!-- No, keep the credential -->
    <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.CANCEL()} />
  </ActionSheet>

  <!-- Little arrow that visually links the menu box to the trigger button -->
  <div use:melt={$arrow} class="border border-b-0 border-r-0 border-slate-300 dark:border-slate-600"></div>
</div>
<!-- {/if} -->
