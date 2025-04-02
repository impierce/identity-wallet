<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { SettingsCaretLink, SettingsSwitch, SettingsValueLink, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    CodeBoldIcon,
    FilesFillIcon,
    KeyFillIcon,
    ListStarFillIcon,
    SunFillIcon,
    TranslateFillIcon,
  } from '$lib/icons';
  import { locales } from '$lib/locales';
  import { state } from '$lib/stores';

  async function handleDevModeSwitch() {
    await dispatch({
      type: '[DEV] Toggle DEV mode',
    });
  }
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col gap-3 px-4 py-5">
    <SettingsValueLink
      href="/me/settings/app/language"
      value={locales.find((l) => l.locale === $state.profile_settings.locale)?.displayName ??
        $state.profile_settings.locale}
    >
      {#snippet icon()}
        <TranslateFillIcon class="h-5 w-5 text-primary"></TranslateFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.LANGUAGE.TITLE()}
    </SettingsValueLink>

    <SettingsCaretLink href="/me/settings/app/theme">
      {#snippet icon()}
        <SunFillIcon class="h-5 w-5 text-primary"></SunFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.THEME.LABEL()}
    </SettingsCaretLink>

    <!-- <SettingsCaretLink href="#" disabled>
      {#snippet icon()}
        <PasswordFillIcon class="h-5 w-5 text-primary"></PasswordFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.PASSWORD.TITLE()}
    </SettingsCaretLink> -->

    <!-- <SettingsValueLink href="#" value={$LL.SETTINGS.APP.ONBOARDING_JOURNEY.BUTTON_TEXT()} disabled>
      {#snippet icon()}
        <ConfettiFillIcon class="h-5 w-5 text-primary"></ConfettiFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.ONBOARDING_JOURNEY.TITLE()}
    </SettingsValueLink> -->

    <!-- <SettingsValueLink href="#" value={$LL.SETTINGS.APP.HINTS_AND_TIPS.BUTTON_TEXT()} disabled>
      {#snippet icon()}
        <ChatCircleTextFillIcon class="h-5 w-5 text-primary"></ChatCircleTextFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.HINTS_AND_TIPS.TITLE()}
    </SettingsValueLink> -->

    <SettingsSwitch checked={$state?.dev_mode !== 'Off'} onchange={handleDevModeSwitch}>
      {#snippet icon()}
        <CodeBoldIcon class="h-5 w-5 text-primary"></CodeBoldIcon>
      {/snippet}
      {$LL.SETTINGS.APP.DEVELOPER_MODE.TITLE()}
    </SettingsSwitch>

    {#if $state.dev_mode !== 'Off'}
      <div in:fade={{ duration: 200 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/did">
          {#snippet icon()}
            <FilesFillIcon class="h-5 w-5 text-primary"></FilesFillIcon>
          {/snippet}
          DID management
        </SettingsCaretLink>
      </div>

      <div in:fade={{ duration: 200, delay: 50 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/keys">
          {#snippet icon()}
            <KeyFillIcon class="h-5 w-5 text-primary"></KeyFillIcon>
          {/snippet}
          Key management
        </SettingsCaretLink>
      </div>

      <div in:fade={{ duration: 200, delay: 100 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/trust-list">
          {#snippet icon()}
            <ListStarFillIcon class="h-5 w-5 text-primary"></ListStarFillIcon>
          {/snippet}
          Trusted issuers
        </SettingsCaretLink>
      </div>
    {/if}
  </div>
</div>
