<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade, fly } from 'svelte/transition';

  import { SettingsCaretLink, SettingsSwitch, SettingsValueLink, TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import {
    CodeBoldIcon,
    FilesFillIcon,
    KeyFillIcon,
    ListStarFillIcon,
    ShieldCheckFillIcon,
    SunFillIcon,
    TranslateFillIcon,
  } from '$lib/icons';
  import { locales } from '$lib/locales';
  import { error, navigationDirection, state } from '$lib/stores';

  beforeNavigate(({ type, cancel }) => {
    if (type === 'popstate') {
      cancel();
      goto('/me/settings');
    }
  });

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="bg-silver dark:bg-navy flex flex-col" in:fly={{ x, duration, opacity: 1 }}>
  <div class="flex flex-col gap-3 px-4 py-5">
    <SettingsValueLink
      href="/me/settings/app/language"
      value={locales.find((l) => l.locale === $state.profile_settings.locale)?.displayName ??
        $state.profile_settings.locale}
    >
      {#snippet icon()}
        <TranslateFillIcon class="text-primary h-5 w-5"></TranslateFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.LANGUAGE.TITLE()}
    </SettingsValueLink>

    <SettingsCaretLink href="/me/settings/app/theme">
      {#snippet icon()}
        <SunFillIcon class="text-primary h-5 w-5"></SunFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.THEME.LABEL()}
    </SettingsCaretLink>

    <SettingsCaretLink href="/me/settings/app/security">
      {#snippet icon()}
        <ShieldCheckFillIcon class="text-primary h-5 w-5"></ShieldCheckFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.SECURITY.LABEL()}
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

    {#if $state.show_dev_mode_setting}
      <SettingsSwitch
        checked={$state?.dev_mode !== 'Off'}
        onCheckedChange={({ curr, next }) => {
          try {
            dispatch({
              type: '[DEV] Toggle DEV mode',
            });
            return next;
          } catch (e) {
            if (e instanceof Error) {
              $error = e.message;
            }
            return curr;
          }
        }}
      >
        {#snippet icon()}
          <CodeBoldIcon class="text-primary h-5 w-5"></CodeBoldIcon>
        {/snippet}
        {$LL.SETTINGS.APP.DEVELOPER_MODE.TITLE()}
      </SettingsSwitch>
    {/if}

    {#if $state.dev_mode !== 'Off'}
      <div in:fade={{ duration: 200 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/did">
          {#snippet icon()}
            <FilesFillIcon class="text-primary h-5 w-5"></FilesFillIcon>
          {/snippet}
          DID management
        </SettingsCaretLink>
      </div>

      <div in:fade={{ duration: 200, delay: 50 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/keys">
          {#snippet icon()}
            <KeyFillIcon class="text-primary h-5 w-5"></KeyFillIcon>
          {/snippet}
          Key management
        </SettingsCaretLink>
      </div>

      <div in:fade={{ duration: 200, delay: 100 }} out:fade={{ duration: 200 }}>
        <SettingsCaretLink href="/me/settings/app/trust-list">
          {#snippet icon()}
            <ListStarFillIcon class="text-primary h-5 w-5"></ListStarFillIcon>
          {/snippet}
          Trusted issuers
        </SettingsCaretLink>
      </div>
    {/if}
  </div>
</div>
