<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import { ActionSheet, Button } from '$lib/components';
  import { DotsThreeVerticalBoldIcon, TrashFillIcon } from '$lib/icons';

  const {
    elements: { trigger, menu, item, separator, arrow },
    // builders: { createSubmenu, createMenuRadioGroup, createCheckboxItem },
    states: { open },
  } = createDropdownMenu({
    // forceVisible: true,
    loop: true,
  });
</script>

<button type="button" class="trigger bg-fuchsia-500" use:melt={$trigger} aria-label="Open credential menu">
  <DotsThreeVerticalBoldIcon class="h-6 w-6 dark:text-white" />
  <!-- <DotsThreeOutlineVerticalFillIcon class="h-6 w-6 dark:text-white" /> -->
</button>

{#if open}
  <div class="menu" use:melt={$menu} transition:fly={{ duration: 150, y: -10 }}>
    <ActionSheet
      titleText={'Delete credential'}
      descriptionText={'Are you sure you want to remove the credential from your wallet?'}
    >
      <button
        slot="trigger"
        let:trigger
        class="flex items-center space-x-4 rounded-xl bg-navy px-4 py-2"
        use:melt={trigger}
        on:click={() => open.set(false)}
      >
        <svelte:component this={TrashFillIcon} class="h-5 w-5 text-rose-400" />
        <p class="grow text-left text-[13px]/[24px] font-medium text-white">
          {'Delete credential'}
        </p>
      </button>

      <!-- Yes, delete the credential -->
      <div slot="content" class="w-full pb-[10px] pt-[20px]">
        <button
          class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
          on:click={() => {}}>{'Yes, delete this credential'}</button
        >
      </div>

      <!-- No, keep the credential -->
      <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.CANCEL()} />
    </ActionSheet>
    <!-- <div class="item" use:melt={$item}>About Melt UI</div> -->
    <!-- Little arrow that visually links the menu box to the trigger button -->
    <div use:melt={$arrow} />
  </div>
{/if}

<style lang="postcss">
  .menu {
    @apply z-40 flex max-h-[300px] min-w-[220px] flex-col shadow-lg;
    /* @apply rounded-md bg-white p-1 shadow-neutral-900/30 lg:max-h-none; */
    @apply rounded-2xl bg-dark p-2 shadow-neutral-900/30 lg:max-h-none;
    @apply ring-0 !important;
  }

  .trigger {
    /* @apply inline-flex h-9 w-9 items-center justify-center rounded-full bg-white; */
    /* @apply text-blue-900 transition-colors hover:bg-white/90; */
    /* @apply data-[highlighted]:ring-blue-400 data-[highlighted]:ring-offset-2 !important; */
    /* @apply p-0 text-sm font-medium data-[highlighted]:outline-none; */
  }
</style>
