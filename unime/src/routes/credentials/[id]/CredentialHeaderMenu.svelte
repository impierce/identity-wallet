<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import { ActionSheet, Button } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { DotsThreeVerticalBoldIcon, TrashFillIcon } from '$lib/icons';

  export let id: string;

  const {
    elements: { trigger, menu, item, arrow },
    // TODO: see comment below (Ln 28)
    // states: { open },
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
  class="flex min-w-[160px] flex-col rounded-xl bg-background-alt p-2 shadow-md"
  use:melt={$menu}
  transition:fly={{ duration: 150, y: -10 }}
>
  <ActionSheet
    titleText={$LL.CREDENTIAL.ACTIONS.DELETE.TITLE()}
    descriptionText={$LL.CREDENTIAL.ACTIONS.DELETE.DESCRIPTION()}
  >
    <button
      slot="trigger"
      let:trigger
      class="flex items-center space-x-4 rounded-lg bg-background-alt px-3 py-3"
      use:melt={$item}
      use:melt={trigger}
    >
      <svelte:component this={TrashFillIcon} class="h-5 w-5 text-rose-400" />
      <p class="grow text-left text-[13px]/[24px] font-medium dark:text-white">
        {$LL.CREDENTIAL.ACTIONS.DELETE.BUTTON_LABEL()}
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
  <div use:melt={$arrow} />
</div>
<!-- {/if} -->
