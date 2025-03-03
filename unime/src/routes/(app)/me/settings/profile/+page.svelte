<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { melt } from '@melt-ui/svelte';

  import { ActionSheet, Button, SettingsCaretLink, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { KeyboardFillIcon, TrashFillIcon } from '$lib/icons';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.PROFILE.TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col gap-3 bg-background px-4 py-5">
  <SettingsCaretLink href="/me/settings/profile/name">
    {#snippet icon()}
      <KeyboardFillIcon class="h-5 w-5 text-primary"></KeyboardFillIcon>
    {/snippet}
    {$LL.SETTINGS.PROFILE.PROFILE_NAME.TITLE()}
  </SettingsCaretLink>

  <!-- Delete profile -->
  <ActionSheet titleText={$LL.SETTINGS.RESET_APP.TITLE()} descriptionText={$LL.SETTINGS.RESET_APP.DESCRIPTION()}>
    <button
      slot="trigger"
      let:trigger
      class="flex items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
      use:melt={trigger}
    >
      <svelte:component this={TrashFillIcon} class="h-5 w-5 text-rose-400" />
      <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
        {$LL.SETTINGS.PROFILE.DELETE_PROFILE.TITLE()}
      </p>
    </button>

    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <button
        class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
        on:click={() => dispatch({ type: '[App] Reset' })}>{$LL.SETTINGS.RESET_APP.CONFIRM()}</button
      >
    </div>
    <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.SETTINGS.RESET_APP.CANCEL()} />
  </ActionSheet>
</div>
