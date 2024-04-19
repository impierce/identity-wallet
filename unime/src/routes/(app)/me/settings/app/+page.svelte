<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { locales } from '$lib/app/locales';
  import SettingsEntry from '$lib/app/settings/SettingsEntry.svelte';
  import Switch from '$lib/components/atoms/Switch.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state } from '$lib/stores';

  import ChatCircleText from '~icons/ph/chat-circle-text-fill';
  import Code from '~icons/ph/code-bold';
  import Confetti from '~icons/ph/confetti-fill';
  import Password from '~icons/ph/password-fill';
  import Sun from '~icons/ph/sun-fill';
  import Translate from '~icons/ph/translate-fill';

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
      icon={Translate}
      title={$LL.SETTINGS.APP.LANGUAGE.TITLE()}
      hasCaretRight={false}
      textRight={locales.find((l) => l.locale === $state.profile_settings.locale)?.displayName ??
        $state.profile_settings.locale}
      on:click={() => goto('/me/settings/app/language')}
    />
    <SettingsEntry icon={Sun} title={$LL.SETTINGS.APP.THEME.LABEL()} on:click={() => goto('/me/settings/app/theme')} />
    <SettingsEntry icon={Password} title={$LL.SETTINGS.APP.PASSWORD.TITLE()} disabled />
    <SettingsEntry
      icon={Confetti}
      title={$LL.SETTINGS.APP.ONBOARDING_JOURNEY.TITLE()}
      hasCaretRight={false}
      textRight={$LL.SETTINGS.APP.ONBOARDING_JOURNEY.BUTTON_TEXT()}
      disabled
    />
    <SettingsEntry
      icon={ChatCircleText}
      title={$LL.SETTINGS.APP.HINTS_AND_TIPS.TITLE()}
      hasCaretRight={false}
      textRight={$LL.SETTINGS.APP.HINTS_AND_TIPS.BUTTON_TEXT()}
      disabled
    />
    <SettingsEntry icon={Code} title={$LL.SETTINGS.APP.DEVELOPER_MODE.TITLE()} hasCaretRight={false}>
      <Switch active={$state?.dev_mode !== 'Off'} on:change={toggleDevSettings} />
    </SettingsEntry>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
