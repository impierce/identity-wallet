<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';

  import { ActionSheet, Button, SettingsCaretLink, TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import { KeyboardFillIcon, TrashFillIcon } from '$lib/icons';
  import { navigationDirection } from '$lib/stores';

  const parentRoute = '/me/settings';

  beforeNavigate(({ type, cancel }) => {
    if (type === 'popstate') {
      cancel();
      goto(parentRoute);
    }
  });

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar on:back={() => goto(parentRoute)} title={$LL.SETTINGS.PROFILE.TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col gap-3 bg-background px-4 py-5" in:fly={{ x, duration, opacity: 1 }}>
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
      <TrashFillIcon class="h-5 w-5 text-rose-400" />
      <p class="grow text-left text-sm font-medium text-slate-800 dark:text-white">
        {$LL.SETTINGS.PROFILE.DELETE_PROFILE.TITLE()}
      </p>
    </button>

    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <button
        class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
        onclick={() => dispatch({ type: '[App] Reset' })}
      >
        {$LL.SETTINGS.RESET_APP.CONFIRM()}
      </button>
    </div>
    <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.SETTINGS.RESET_APP.CANCEL()} />
  </ActionSheet>
</div>
