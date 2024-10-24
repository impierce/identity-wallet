<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { TopNavBar } from '$lib/components';
  import SettingsEntry from '$lib/components/SettingsEntry.svelte';
  import { dispatch } from '$lib/dispatcher';
  import {
    EnvelopeFillIcon,
    GearFillIcon,
    InfoFillIcon,
    SignOutFillIcon,
    UserCircleFillIcon,
    VaultFillIcon,
  } from '$lib/icons';
  import { state } from '$lib/stores';

  import EmojiAvatarSelect from './EmojiAvatarSelect.svelte';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.NAVBAR_TITLE()} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="hide-scrollbar flex flex-col space-y-[15px] overflow-y-scroll px-4 py-5" in:fly={{ x: 32, opacity: 1 }}>
    <!-- Avatar -->
    <div class="flex justify-center">
      <EmojiAvatarSelect
        selected={$state?.profile_settings.profile?.picture ?? undefined}
        showEditButton={true}
        on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { picture: e.detail } })}
      />
    </div>
    <!-- Account -->
    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.SETTINGS.ACCOUNT()}</p>
      <SettingsEntry
        icon={UserCircleFillIcon}
        title={$LL.SETTINGS.PROFILE.TITLE()}
        on:click={() => goto('/me/settings/profile')}
      />
      <SettingsEntry icon={GearFillIcon} title={$LL.SETTINGS.APP.TITLE()} on:click={() => goto('/me/settings/app')} />
      <SettingsEntry
        icon={VaultFillIcon}
        title={$LL.SETTINGS.BACKUP_RECOVERY.TITLE()}
        on:click={() => goto('/me/settings/backup')}
        disabled
      />
      <SettingsEntry icon={SignOutFillIcon} title={$LL.SETTINGS.LOG_OUT.TITLE()} hasCaretRight={false} disabled />
    </div>

    <!-- Support -->
    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.SETTINGS.SUPPORT.TITLE()}</p>
      <SettingsEntry
        icon={InfoFillIcon}
        title={$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}
        on:click={() => goto('/me/settings/about')}
      />
      <SettingsEntry
        icon={EnvelopeFillIcon}
        title={$LL.SETTINGS.SUPPORT.FEEDBACK.TITLE()}
        hasCaretRight={false}
        disabled
      />
    </div>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
