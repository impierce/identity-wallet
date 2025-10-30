<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { store } from '@impierce/tauri-plugin-keystore';
  import { authenticate, BiometryType, checkStatus, type Status } from '@tauri-apps/plugin-biometric';

  import { ActionSheet, Button, TopNavBar } from '$lib/components';
  import {
    EyeClosedRegularIcon,
    EyeRegularIcon,
    FingerprintLightIcon,
    ScanSmileyLightIcon,
    SmileyRegularIcon,
    SmileySadRegularIcon,
  } from '$lib/icons';
  import { error, onboarding_state, state } from '$lib/stores';
  import { localizedBiometricsTypeString } from '$lib/utils';

  // 3 states: true (match), false (mismatch), undefined (not checked yet).
  let passwordsMatch: boolean | undefined = undefined;

  let showPassword = false;
  let value = '';

  // Ref to input DOM element.
  let inputElement: HTMLInputElement;

  // Tracks whether the user has interacted with the input element. This allows to suppress the validation message only on first try.
  let touched = false;

  let biometricsStatus: Status;
  let biometricsName: string;

  const enableBiometrics = async () => {
    $onboarding_state.biometrics_enabled = true;

    if ($state.dev_mode !== 'Off') {
      goto('/welcome/completed');
    }

    const password = $onboarding_state.password;

    if (password) {
      // TODO: authenticate first, before storing the password => duplicate check?
      await authenticate('Enable biometrics').then(async () => {
        await store(password).then(() => {
          goto('/welcome/completed');
        });
      });
    } else {
      error.set('Biometrics enabled, but no password value provided');
    }
  };

  // TODO: This workaround capitalizes the first letter for a specific language, since typesafe-i18n formatters do not seem to support this.
  const capitalize = (value: string) => {
    if ($state.profile_settings.locale === 'nl-NL') {
      return value.charAt(0).toUpperCase() + value.slice(1);
    } else {
      return value;
    }
  };

  onMount(async () => {
    inputElement.focus();
    biometricsStatus = await checkStatus().catch(() => ({ isAvailable: false, biometryType: BiometryType.None }));
    biometricsName = localizedBiometricsTypeString(biometricsStatus.biometryType);
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.CONFIRM.NAVBAR_TITLE()} />

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="dark:text-grey pb-8 text-3xl font-semibold text-slate-700">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.SUBTITLE()}
    </p>
  </div>
  <div class="relative">
    <!-- Dynamic type attribute requires one-way binding instead of two-way bind:value. -->
    <input
      {value}
      bind:this={inputElement}
      type={showPassword ? 'text' : 'password'}
      placeholder={$LL.ONBOARDING.PASSWORD.CONFIRM.INPUT_PLACEHOLDER()}
      on:blur={() => {
        if (value.length > 0) {
          if (value === $onboarding_state.password) {
            passwordsMatch = true;
          } else {
            passwordsMatch = false;
          }
          // Mark the input element as "touched" after losing focus for the first time.
          touched = true;
        }
      }}
      on:input={(e) => {
        value = e.currentTarget.value;
        if (value === $onboarding_state.password) {
          passwordsMatch = true;
        } else {
          // Suppress validation message (only on first try).
          if (touched) {
            passwordsMatch = false;
          } else {
            passwordsMatch = undefined;
          }
        }
      }}
      class="dark:bg-dark h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:text-slate-300"
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button
        class="rounded-full p-2"
        on:click={() => {
          // Focus input element when toggling visibility.
          inputElement.focus();
          return (showPassword = !showPassword);
        }}
      >
        {#if showPassword}
          <EyeRegularIcon class="dark:text-grey text-slate-700" />
        {:else}
          <EyeClosedRegularIcon class="dark:text-grey text-slate-700" />
        {/if}
      </button>
    </div>
  </div>
  {#if passwordsMatch !== undefined}
    <div class="mt-8 flex items-center justify-center">
      {#if passwordsMatch}
        <SmileyRegularIcon class="text-primary mr-[10px] h-5 w-5" />
        <p class="text-primary text-[13px]/[24px] font-medium">{$LL.ONBOARDING.PASSWORD.CONFIRM.MATCH()}</p>
      {:else}
        <SmileySadRegularIcon class="mr-[10px] h-5 w-5 text-rose-500" />
        <p class="text-[13px]/[24px] font-medium text-rose-500">{$LL.ONBOARDING.PASSWORD.CONFIRM.NO_MATCH()}</p>
      {/if}
    </div>
  {/if}
</div>

<div class="dark:bg-dark rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  {#if biometricsStatus?.isAvailable}
    <ActionSheet
      titleText={capitalize($LL.ONBOARDING.PASSWORD.BIOMETRICS.TITLE({ type: biometricsName }))}
      descriptionText={$LL.ONBOARDING.PASSWORD.BIOMETRICS.DESCRIPTION({ type: biometricsName })}
    >
      <Button slot="trigger" let:trigger {trigger} label={$LL.CONTINUE()} disabled={!passwordsMatch} />
      <div slot="icon" class="mb-2">
        {#if biometricsStatus.biometryType === BiometryType.FaceID || biometricsStatus.biometryType === BiometryType.Iris}
          <ScanSmileyLightIcon class="text-primary size-14" />
        {:else}
          <FingerprintLightIcon class="text-primary size-12" />
        {/if}
      </div>
      <div slot="content" class="flex w-full flex-col space-y-[10px] pt-[20px]">
        <Button
          label={$LL.ONBOARDING.PASSWORD.BIOMETRICS.CONFIRM({ type: biometricsName })}
          on:click={enableBiometrics}
        />
        <Button
          variant="secondary"
          label={$LL.ONBOARDING.PASSWORD.BIOMETRICS.DECIDE_LATER()}
          on:click={() => goto('/welcome/completed')}
        />
      </div>
    </ActionSheet>
  {:else}
    <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/completed')} disabled={!passwordsMatch} />
  {/if}
</div>
