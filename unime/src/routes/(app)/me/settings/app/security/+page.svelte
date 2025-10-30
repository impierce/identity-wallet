<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { writable } from 'svelte/store';
  import { fly } from 'svelte/transition';

  import { remove as remove_inner, store as store_inner } from '@impierce/tauri-plugin-keystore';
  import { authenticate, BiometryType, checkStatus, type Status } from '@tauri-apps/plugin-biometric';
  import { warn } from '@tauri-apps/plugin-log';

  import { ActionSheet, Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import { EyeClosedRegularIcon, EyeRegularIcon, FingerprintFillIcon, ScanSmileyFillIcon } from '$lib/icons';
  import { state as appState, error as errorState, navigationDirection } from '$lib/stores';
  import { localizedBiometricsTypeString } from '$lib/utils';

  const SERVICE = 'com.impierce.identity-wallet';
  const USER = 'unime'; // TODO: rename to "ACCOUNT" to reflect Keychain Access item?

  let biometricsStatus: Status | undefined = $state();
  let biometryTypeString: string = $state('');

  let openPasswordPrompt = writable(false);

  let showPasswordValue = $state(false);
  let passwordValue: string = $state($appState.dev_mode !== 'Off' ? 'sup3rSecr3t' : '');

  let error: string | null = $state(null);

  let action: 'enable' | 'disable' | undefined = $state();

  // Ref to input DOM element.
  let inputElement: HTMLInputElement;

  const toggleBiometrics = async (curr: boolean) => {
    if (curr) {
      action = 'disable';
    } else {
      action = 'enable';
    }
    $openPasswordPrompt = true;
  };

  const checkPassword = async () => {
    let enable = action === 'enable';
    // Check if the password is correct
    await dispatch({ type: '[Storage] Check password', payload: { password: passwordValue } });
    const lastDebugMessage = $appState.debug_messages.at(-1);
    if (lastDebugMessage === 'Wrong Stronghold password') {
      error = 'Incorrect password';
    } else if (lastDebugMessage === 'Stronghold password OK') {
      // (dev): In local development, biometrics plugin is not activated.
      if ($appState.dev_mode !== 'Off') {
        await dispatch({ type: '[Biometrics] Enable', payload: { enable } });
        $openPasswordPrompt = false;
        return;
      }

      // Authenticate with biometrics first, then update the state.
      if (enable) {
        authenticate('Enable biometrics').then(async () => {
          await store(passwordValue);
          $openPasswordPrompt = false;
        });
      } else {
        authenticate('Disable biometrics').then(async () => {
          await remove();
          $openPasswordPrompt = false;
        });
      }
    }
  };

  /**
   * Updates the app state when the value could be removed successfully.
   */
  const remove = async () => {
    await remove_inner(SERVICE, USER)
      .then(async () => {
        await dispatch({ type: '[Biometrics] Enable', payload: { enable: false } });
      })
      .catch((error) => {
        warn(error);
        errorState.set(error);
      });
  };

  /**
   * Updates the app state when the value could be stored successfully.
   * @param value
   */
  const store = async (value: string) => {
    await store_inner(value)
      .then(async () => {
        await dispatch({ type: '[Biometrics] Enable', payload: { enable: true } });
      })
      .catch((error) => {
        warn(error);
        errorState.set(error);
      });
  };

  onMount(async () => {
    biometricsStatus = await checkStatus().catch((error) => {
      warn(error);
      return {
        isAvailable: false,
        biometryType: BiometryType.None,
        error: error,
      };
    });
    // Determine human-readable name for biometrics type (with respect to the device platform)
    biometryTypeString = localizedBiometricsTypeString(biometricsStatus.biometryType);
  });

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.SECURITY.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy" in:fly={{ x, duration, opacity: 1 }}>
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
    {/if}
  </div>

  <ActionSheet
    titleText={action === 'enable'
      ? $LL.SETTINGS.APP.SECURITY.ENABLE.DIALOG_TITLE({ type: biometryTypeString })
      : $LL.SETTINGS.APP.SECURITY.DISABLE.DIALOG_TITLE({ type: biometryTypeString })}
    descriptionText={action === 'enable'
      ? $LL.SETTINGS.APP.SECURITY.ENABLE.DIALOG_CONTENT({ type: biometryTypeString })
      : $LL.SETTINGS.APP.SECURITY.DISABLE.DIALOG_CONTENT({ type: biometryTypeString })}
    open={openPasswordPrompt}
  >
    <div slot="content" class="flex w-full flex-col gap-3 pt-[20px]">
      <!-- Password input -->
      <div class="relative">
        <!-- Dynamic type attribute requires one-way binding instead of two-way bind:value. -->
        <input
          value={passwordValue}
          bind:this={inputElement}
          type={showPasswordValue ? 'text' : 'password'}
          placeholder={$LL.LOCK_SCREEN.PASSWORD_INPUT_PLACEHOLDER()}
          oninput={(e) => {
            error = null;
            passwordValue = e.currentTarget.value;
          }}
          class={`h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300 ${error ? 'border-rose-500 ring-3 ring-rose-100' : ''}`}
        />
        <div class="absolute top-0 right-3 flex h-full items-center">
          <button
            class="rounded-full p-2"
            onclick={() => {
              // Focus input element when toggling visibility.
              inputElement.focus();
              return (showPasswordValue = !showPasswordValue);
            }}
          >
            {#if showPasswordValue}
              <EyeRegularIcon class="text-slate-700 dark:text-grey" />
            {:else}
              <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
            {/if}
          </button>
        </div>
      </div>

      <Button
        label={$LL.CONTINUE()}
        on:click={() => {
          checkPassword();
        }}
      />
    </div>
  </ActionSheet>
</div>
