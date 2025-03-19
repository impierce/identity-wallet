<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { remove, retrieve, store } from '@impierce/tauri-plugin-keystore';
  import { authenticate, BiometryType, checkStatus, type Status } from '@tauri-apps/plugin-biometric';

  import { ActionSheet, Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { EyeClosedRegularIcon, EyeRegularIcon, FingerprintFillIcon, ScanSmileyFillIcon } from '$lib/icons';
  import { error as errorState, state } from '$lib/stores';
  import { biometricsTypeString } from '$lib/utils';

  const SERVICE = 'com.impierce.identity-wallet';
  const USER = 'tester'; // TODO: rename to "ACCOUNT" to reflect Keychain Access item?

  let biometricsStatus: Status;
  let biometryTypeString: string = 'biometrics';
  let enabled: boolean = $state.profile_settings.biometrics_enabled;

  let enterPassword: boolean = false;
  let showPassword = false;
  let passwordValue: string;

  // Ref to input DOM element.
  let inputElement: HTMLInputElement;

  // TODO: remove (during development only)
  let retrieved: string | null;

  const toggleBiometrics = async () => {
    console.log('toggleBiometrics, current: ', enabled);
    if (enabled) {
      await _remove();
    } else {
      enterPassword = true;
      // TODO: ask for the password first
      const password = 'sup3rSecr3t';
      // Check biometrics first, before storing the password => duplicate check on Android?
      authenticate('Enable biometrics').then(async () => {
        await store(password)
          .then(async () => {
            await dispatch({ type: '[Biometrics] Enable', payload: { enable: true } });
          })
          .catch((error) => {
            console.warn(error);
            errorState.set(error);
          });
      });
    }
  };

  const _retrieve = async () => {
    retrieved = await retrieve(SERVICE, USER).catch((error) => {
      return 'retrieve: error';
    });
    console.log('retrieved', retrieved);
  };

  const _remove = async () => {
    await remove(SERVICE, USER)
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
    biometryTypeString = biometricsTypeString(biometricsStatus.biometryType);
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.SECURITY.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    {#if biometricsStatus}
      <SettingsSwitch
        initialChecked={enabled}
        onchange={(checked) => {
          toggleBiometrics();
        }}
      >
        {#snippet icon()}
          {#if biometricsStatus.biometryType === BiometryType.FaceID}
            <ScanSmileyFillIcon class="size-5 text-primary"></ScanSmileyFillIcon>
          {:else}
            <FingerprintFillIcon class="size-5 text-primary"></FingerprintFillIcon>
          {/if}
          <!-- <CodeBoldIcon class="h-5 w-5 text-primary"></CodeBoldIcon> -->
        {/snippet}
        {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({ type: biometryTypeString })}
      </SettingsSwitch>

      <!-- <SettingsEntry
        icon={biometricsStatus.biometryType === BiometryType.FaceID ? ScanSmileyFillIcon : FingerprintFillIcon}
        title={`Unlock with ${biometryTypeString}`}
        hasCaretRight={false}
      >
        <Switch active={enabled} onchange={toggleBiometrics} />
      </SettingsEntry>
      {#if biometricsStatus.error && $state.dev_mode !== 'Off'}
        <div class="h-12 rounded-lg bg-rose-50 py-4 text-center text-xs font-medium text-rose-500">
          Biometrics are not available.
        </div>
      {/if}
    {/if}
    <SettingsEntry icon={PasswordFillIcon} title={'Change password'} disabled /> -->
    {/if}
  </div>
  <!-- TODO: dev -->
  {#if $state.dev_mode !== 'Off'}
    <div class="m-8 flex flex-col space-y-4">
      <button
        class="rounded-lg border border-amber-300 bg-amber-100 py-4 text-xs font-medium text-amber-600 shadow"
        on:click={_retrieve}>Retrieve from secure storage</button
      >
      <button
        class="rounded-lg border border-sky-300 bg-sky-100 py-4 text-xs font-medium text-sky-600 shadow"
        on:click={_remove}>Clear secure storage</button
      >
      <pre class="text-sm">value: {retrieved}</pre>
    </div>
  {/if}

  <ActionSheet titleText={'titleText'} descriptionText={'descriptionText'} isOpen={enterPassword}>
    <div slot="content" class="flex w-full flex-col gap-3 pt-[20px]">
      <!-- Password input -->
      <div class="relative">
        <!-- Dynamic type attribute requires one-way binding instead of two-way bind:value. -->
        <input
          value={passwordValue}
          bind:this={inputElement}
          type={showPassword ? 'text' : 'password'}
          placeholder={$LL.LOCK_SCREEN.PASSWORD_INPUT_PLACEHOLDER()}
          on:blur={() => {
            if (passwordValue.length > 0) {
              // if (value === $onboarding_state.password) {
              //   passwordsMatch = true;
              // } else {
              //   passwordsMatch = false;
              // }
              // Mark the input element as "touched" after losing focus for the first time.
              // touched = true;
            }
          }}
          on:input={(e) => {
            // value = e.currentTarget.value;
            // if (value === $onboarding_state.password) {
            //   passwordsMatch = true;
            // } else {
            //   // Suppress validation message (only on first try).
            //   if (touched) {
            //     passwordsMatch = false;
            //   } else {
            //     passwordsMatch = undefined;
            //   }
            // }
          }}
          class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
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
              <EyeRegularIcon class="text-slate-700 dark:text-grey" />
            {:else}
              <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
            {/if}
          </button>
        </div>
      </div>

      <!-- <div slot="content" class="w-full pb-[10px] pt-[20px]">
        <Button label="Yes" on:click={() => dispatch({ type: '[User Journey] Cancel' })} />
      </div>
      <Button variant="secondary" slot="close" let:close trigger={close} label="No, let's continue" /> -->

      <!-- Confirm -->
      <Button label={$LL.CONTINUE()} />
      <!-- <button
      class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
      on:click={() => dispatch({ type: '[App] Reset' })}>{$LL.CONTINUE()}</button
    > -->
    </div>

    <!-- <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.SETTINGS.RESET_APP.CANCEL()} /> -->
  </ActionSheet>
</div>
