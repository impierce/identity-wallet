<script lang="ts">
  import { goto } from '$app/navigation';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import SettingsEntry from '$src/lib/app/settings/SettingsEntry.svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import ActionSheet from '$src/lib/components/molecules/dialogs/ActionSheet.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';

  import Keyboard from '~icons/ph/keyboard-fill';
  import Trash from '~icons/ph/trash-fill';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.PROFILE.TITLE()} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry
      icon={Keyboard}
      title={$LL.SETTINGS.PROFILE.PROFILE_NAME.TITLE()}
      on:click={() => goto('/me/settings/profile/name')}
    />

    <!-- Delete profile -->
    <ActionSheet titleText={$LL.SETTINGS.RESET_APP.TITLE()} descriptionText={$LL.SETTINGS.RESET_APP.DESCRIPTION()}>
      <button
        slot="trigger"
        let:trigger
        class="flex items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
        use:melt={trigger}
      >
        <svelte:component this={Trash} class="h-5 w-5 text-rose-400" />
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
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
