<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { SettingsEntry, Switch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    ChatCircleTextFillIcon,
    CodeBoldIcon,
    ConfettiFillIcon,
    FilesFillIcon,
    KeyFillIcon,
    ListStarFillIcon,
    PasswordFillIcon,
    SunFillIcon,
    TranslateFillIcon,
  } from '$lib/icons';
  import { locales } from '$lib/locales';
  import { state } from '$lib/stores';

  async function toggleDevSettings() {
    await dispatch({
      type: '[DEV] Toggle DEV mode',
    });
  }
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.NAVBAR_TITLE()} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry
      icon={TranslateFillIcon}
      title={$LL.SETTINGS.APP.LANGUAGE.TITLE()}
      hasCaretRight={false}
      textRight={locales.find((l) => l.locale === $state.profile_settings.locale)?.displayName ??
        $state.profile_settings.locale}
      on:click={() => goto('/me/settings/app/language')}
    />
    <SettingsEntry
      icon={SunFillIcon}
      title={$LL.SETTINGS.APP.THEME.LABEL()}
      on:click={() => goto('/me/settings/app/theme')}
    />
    <SettingsEntry icon={PasswordFillIcon} title={$LL.SETTINGS.APP.PASSWORD.TITLE()} disabled />
    <SettingsEntry
      icon={ConfettiFillIcon}
      title={$LL.SETTINGS.APP.ONBOARDING_JOURNEY.TITLE()}
      hasCaretRight={false}
      textRight={$LL.SETTINGS.APP.ONBOARDING_JOURNEY.BUTTON_TEXT()}
      disabled
    />
    <SettingsEntry
      icon={ChatCircleTextFillIcon}
      title={$LL.SETTINGS.APP.HINTS_AND_TIPS.TITLE()}
      hasCaretRight={false}
      textRight={$LL.SETTINGS.APP.HINTS_AND_TIPS.BUTTON_TEXT()}
      disabled
    />
    <SettingsEntry icon={CodeBoldIcon} title={$LL.SETTINGS.APP.DEVELOPER_MODE.TITLE()} hasCaretRight={false}>
      <Switch active={$state?.dev_mode !== 'Off'} on:change={toggleDevSettings} />
    </SettingsEntry>
    {#if $state.dev_mode !== 'Off'}
      <div in:fade={{ duration: 200 }} out:fade={{ duration: 200 }}>
        <SettingsEntry
          icon={FilesFillIcon}
          title={'DID management'}
          hasCaretRight={true}
          on:click={() => goto('/me/settings/app/did')}
        />
      </div>
      <div in:fade={{ duration: 200 }} out:fade={{ duration: 200 }}>
        <SettingsEntry
          icon={KeyFillIcon}
          title={'Key management'}
          hasCaretRight={true}
          on:click={() => goto('/me/settings/app/keys')}
        />
      </div>
      <div in:fade={{ duration: 200 }} out:fade={{ duration: 200 }}>
        <SettingsEntry
          icon={ListStarFillIcon}
          title={'Trust lists'}
          hasCaretRight={true}
          on:click={() => goto('/me/settings/app/trust-list')}
        />
      </div>
    {/if}
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
