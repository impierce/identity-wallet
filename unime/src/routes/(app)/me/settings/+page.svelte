<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { SettingsCaretLink, TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import { GearFillIcon, InfoFillIcon, UserCircleFillIcon } from '$lib/icons';
  import { navigationDirection, state } from '$lib/stores';

  import EmojiAvatarSelect from './EmojiAvatarSelect.svelte';

  const parentRoute = '/me';

  beforeNavigate(({ type, cancel }) => {
    if (type === 'popstate') {
      cancel();
      goto(parentRoute);
    }
  });

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar on:back={() => goto(parentRoute)} title={$LL.SETTINGS.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="bg-background flex flex-col space-y-[15px] px-4 py-5" in:fly={{ x, duration, opacity: 1 }}>
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
        <UserCircleFillIcon class="text-primary h-5 w-5"></UserCircleFillIcon>
      {/snippet}
      {$LL.SETTINGS.PROFILE.TITLE()}
    </SettingsCaretLink>

    <SettingsCaretLink href="/me/settings/app">
      {#snippet icon()}
        <GearFillIcon class="text-primary h-5 w-5"></GearFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.TITLE()}
    </SettingsCaretLink>

    <!-- <SettingsCaretLink href="/me/settings/backup">
      {#snippet icon()}
        <VaultFillIcon class="h-5 w-5 text-primary"></VaultFillIcon>
      {/snippet}
      {$LL.SETTINGS.BACKUP_RECOVERY.TITLE()}
    </SettingsCaretLink> -->

    <!-- TODO Logout button with `SignOutFillIcon` and `LL.SETTINGS.LOG_OUT.TITLE()`.  -->
  </div>

  <!-- Support -->
  <div class="flex flex-col space-y-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.SETTINGS.SUPPORT.TITLE()}</p>

    <SettingsCaretLink href="/me/settings/about">
      {#snippet icon()}
        <InfoFillIcon class="text-primary h-5 w-5"></InfoFillIcon>
      {/snippet}
      {$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}
    </SettingsCaretLink>

    <!-- TODO Feedback button with `EnvelopeFillIcon` and `$LL.SETTINGS.SUPPORT.FEEDBACK.TITLE()`. -->
  </div>
</div>
