<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { SettingsCaretLink, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { GearFillIcon, InfoFillIcon, UserCircleFillIcon, VaultFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  import EmojiAvatarSelect from './EmojiAvatarSelect.svelte';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col space-y-[15px] bg-background px-4 py-5" in:fly={{ x: 32, opacity: 1 }}>
  <!-- Avatar -->
  <div class="flex justify-center">
    <EmojiAvatarSelect
      selected={$state?.profile_settings.profile?.picture ?? undefined}
      showEditButton={true}
      on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { picture: e.detail } })}
    />
  </div>
  <!-- Account -->
  <div class="flex flex-col gap-3">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.SETTINGS.ACCOUNT()}</p>

    <SettingsCaretLink href="/me/settings/profile">
      {#snippet icon()}
        <UserCircleFillIcon class="h-5 w-5 text-primary"></UserCircleFillIcon>
      {/snippet}
      {$LL.SETTINGS.PROFILE.TITLE()}
    </SettingsCaretLink>

    <SettingsCaretLink href="/me/settings/app">
      {#snippet icon()}
        <GearFillIcon class="h-5 w-5 text-primary"></GearFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.TITLE()}
    </SettingsCaretLink>

    <SettingsCaretLink href="/me/settings/backup">
      {#snippet icon()}
        <VaultFillIcon class="h-5 w-5 text-primary"></VaultFillIcon>
      {/snippet}
      {$LL.SETTINGS.BACKUP_RECOVERY.TITLE()}
    </SettingsCaretLink>

    <!-- TODO Logout button with `SignOutFillIcon` and `LL.SETTINGS.LOG_OUT.TITLE()`.  -->
  </div>

  <!-- Support -->
  <div class="flex flex-col space-y-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.SETTINGS.SUPPORT.TITLE()}</p>

    <SettingsCaretLink href="/me/settings/about">
      {#snippet icon()}
        <InfoFillIcon class="h-5 w-5 text-primary"></InfoFillIcon>
      {/snippet}
      {$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}
    </SettingsCaretLink>

    <!-- TODO Feedback button with `EnvelopeFillIcon` and `$LL.SETTINGS.SUPPORT.FEEDBACK.TITLE()`. -->
  </div>
</div>
