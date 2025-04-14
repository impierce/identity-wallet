<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { remove as remove_inner, retrieve as retrieve_inner, store } from '@impierce/tauri-plugin-keystore';
  import { authenticate, BiometryType, checkStatus, type Status } from '@tauri-apps/plugin-biometric';

  import { Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { EyeClosedRegularIcon, EyeRegularIcon, FingerprintFillIcon, ScanSmileyFillIcon } from '$lib/icons';
  import { state as appState, error as errorState } from '$lib/stores';
  import { localizedBiometricsTypeString } from '$lib/utils';

  const SERVICE = 'com.impierce.identity-wallet';
  const USER = 'unime'; // TODO: rename to "ACCOUNT" to reflect Keychain Access item?

  let biometricsStatus: Status | undefined = $state();
  let biometryTypeString: string = $state('');

  let showPasswordInput = $state(false);
  let showPassword = $state(false); // The user can show or hide what they are typing
  let passwordValue: string = $state($appState.dev_mode !== 'Off' ? 'sup3rSecr3t' : '');

  let error: string | null = $state(null);
  let showError = $state(false);

  // TODO: remove (during development only)
  let retrieved: string | null = $state(null);

  const toggleBiometrics = async (curr: boolean) => {
    if (curr) {
      // This is only useful for local development (Desktop)
      if ($appState.dev_mode !== 'Off') {
        await remove();
      }
      authenticate('Disable biometrics').then(async () => {
        await remove();
      });
    } else {
      showPasswordInput = true;
    }
  };

  const checkPassword = async () => {
    await dispatch({ type: '[Storage] Unlock', payload: { password: passwordValue, check_password_only: true } });
    const lastDebugMessage = $appState.debug_messages.at(-1);
    if (lastDebugMessage === 'Wrong Stronghold password') {
      error = 'Incorrect password';
    } else if (lastDebugMessage === 'Stronghold password OK') {
      // Local development (Desktop)
      if ($appState.dev_mode !== 'Off') {
        await dispatch({ type: '[Biometrics] Enable', payload: { enable: true } });
        showPasswordInput = false;
      }

      authenticate('Enable biometrics')
        .then(async () => {
          await store(passwordValue)
            .then(async () => {
              await dispatch({ type: '[Biometrics] Enable', payload: { enable: true } });
              showPasswordInput = false;
            })
            .catch((error) => {
              console.warn(error);
              errorState.set(error);
            });
        })
        .catch((error) => {
          console.warn(error);
          errorState.set(error);
        });
    } else {
      error = null;
    }
    showError = true;
  };

  const retrieve = async () => {
    retrieved = await retrieve_inner(SERVICE, USER).catch((error) => {
      return 'retrieve: error';
    });
    console.log('retrieved', retrieved);
  };

  const remove = async () => {
    if ($appState.dev_mode !== 'Off') {
      await dispatch({ type: '[Biometrics] Enable', payload: { enable: false } });
    }
    await remove_inner(SERVICE, USER)
      .then(async () => {
        retrieved = null;
        await dispatch({ type: '[Biometrics] Enable', payload: { enable: false } });
      })
      .catch(async (error) => {
        retrieved = 'remove: error';
      });
  };

  onMount(async () => {
    biometricsStatus = await checkStatus().catch((error) => {
      console.warn(error);
      return {
        isAvailable: false,
        biometryType: BiometryType.None,
        error: error,
      };
    });
    // Determine human-readable name for biometrics type (with respect to the device platform)
    biometryTypeString = localizedBiometricsTypeString(biometricsStatus.biometryType);
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.SECURITY.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    {#if biometricsStatus}
      <SettingsSwitch
        checked={$appState.profile_settings.biometrics_enabled}
        onCheckedChange={({ curr }) => {
          toggleBiometrics(curr);
          return curr;
        }}
      >
        {#snippet icon()}
          {#if biometricsStatus?.biometryType === BiometryType.FaceID}
            <ScanSmileyFillIcon class="size-5 text-primary"></ScanSmileyFillIcon>
          {:else}
            <FingerprintFillIcon class="size-5 text-primary"></FingerprintFillIcon>
          {/if}
        {/snippet}
        {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({ type: localizedBiometricsTypeString(biometricsStatus.biometryType) })}
      </SettingsSwitch>

      {#if showPasswordInput}
        <div class="relative">
          <input
            type={showPassword ? 'text' : 'password'}
            class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
            placeholder={$LL.LOCK_SCREEN.PASSWORD_INPUT_PLACEHOLDER()}
            oninput={() => (showError = false)}
            bind:value={passwordValue}
          />
          <div class="absolute right-3 top-0 flex h-full items-center">
            <button class="rounded-full p-2" onclick={() => (showPassword = !showPassword)}>
              {#if showPassword}
                <EyeRegularIcon class="text-slate-700 dark:text-grey" />
              {:else}
                <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
              {/if}
            </button>
          </div>
        </div>

        <Button
          label={$LL.ONBOARDING.PASSWORD.BIOMETRICS.TITLE({ type: biometryTypeString })}
          on:click={() => checkPassword()}
        />

        {#if showError}
          {#if $appState.debug_messages.at(-1) === 'Wrong Stronghold password'}
            <div class="text-center text-xs font-medium text-rose-500">{error}</div>
          {/if}
        {/if}
      {/if}
    {/if}
  </div>

  <!-- TODO: dev -->
  {#if $appState.dev_mode !== 'Off'}
    <div class="m-8 flex flex-col space-y-4">
      <button
        class="rounded-lg border border-amber-300 bg-amber-100 py-4 text-xs font-medium text-amber-600 shadow"
        onclick={retrieve}>Retrieve from secure storage</button
      >
      <button
        class="rounded-lg border border-sky-300 bg-sky-100 py-4 text-xs font-medium text-sky-600 shadow"
        onclick={remove}>Clear secure storage</button
      >
      <pre class="text-sm">value: {retrieved}</pre>
    </div>
  {/if}
</div>
