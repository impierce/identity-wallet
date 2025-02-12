<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { remove, retrieve, store } from '@impierce/tauri-plugin-keystore';
  import { BiometryType, checkStatus, type Status } from '@tauri-apps/plugin-biometric';
  import { platform } from '@tauri-apps/plugin-os';

  import { LoadingSpinner, SettingsEntry, Switch, TopNavBar } from '$lib/components';
  import { FingerprintFillIcon, PasswordFillIcon, ScanSmileyFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  const SERVICE = 'com.impierce.identity-wallet';
  const USER = 'tester';

  let biometricsStatus: Status;
  let biometryTypeString: string;
  let enabled: boolean = false;

  let retrieved: string | null;
  let loading: boolean = false;

  const checkBiometrics = async (): Promise<Status> => {
    return await checkStatus().catch((error) => {
      console.warn(error);
      return {
        isAvailable: false,
        biometryType: BiometryType.None,
        error: error,
      };
    });
  };

  const enableBiometrics = async () => {
    // TODO: ask for the password first
    loading = true;
    const password = 'sup3rSecr3t';
    await store(password).then(() => {
      // retrieved = password;
    });
    loading = false;
  };

  const _retrieve = async () => {
    loading = true;
    retrieved = await retrieve(SERVICE, USER).catch((error) => {
      return 'error';
    });
    console.log('retrieved', retrieved);
    loading = false;
  };

  const _remove = async () => {
    loading = true;
    await remove(SERVICE, USER).then(() => {
      retrieved = null;
    });
    loading = false;
  };

  onMount(async () => {
    biometricsStatus = await checkBiometrics();
    biometryTypeString = 'Biometrics';
    // On iOS, we distinguish between Face ID and Touch ID.
    if (platform() === 'ios') {
      switch (biometricsStatus.biometryType) {
        case BiometryType.TouchID:
          biometryTypeString = 'Touch ID';
          break;
        case BiometryType.FaceID:
          biometryTypeString = 'Face ID';
          break;
        default:
          biometryTypeString = 'Biometrics';
      }
      // On Android, we distinguish between fingerprint and face unlock.
    } else if (platform() === 'android') {
      switch (biometricsStatus.biometryType) {
        case BiometryType.TouchID:
          biometryTypeString = 'Fingerprint';
          break;
        case BiometryType.FaceID:
          biometryTypeString = 'Face Unlock';
          break;
        default:
          biometryTypeString = 'Biometrics';
      }
    }
  });
</script>

<TopNavBar on:back={() => history.back()} title={'Security'} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    {#if biometricsStatus}
      <SettingsEntry
        icon={biometricsStatus.biometryType === BiometryType.FaceID ? ScanSmileyFillIcon : FingerprintFillIcon}
        title={`Unlock with ${biometryTypeString}`}
        hasCaretRight={false}
      >
        <Switch active={enabled} on:change={enableBiometrics} />
      </SettingsEntry>
      {#if biometricsStatus.error && $state.dev_mode !== 'Off'}
        <div class="h-12 rounded-lg bg-rose-50 py-4 text-center text-xs font-medium text-rose-500">
          Biometrics are not available.
        </div>
      {/if}
    {/if}
    <SettingsEntry icon={PasswordFillIcon} title={'Change password'} disabled />
    <button
      class="rounded-lg border border-amber-300 bg-amber-100 py-4 text-xs font-medium text-amber-600 shadow"
      on:click={_retrieve}>Retrieve from secure storage</button
    >
  </div>
  <button
    class="mx-4 rounded-lg border border-sky-300 bg-sky-100 py-4 text-xs font-medium text-sky-600 shadow"
    on:click={_remove}>Clear secure storage</button
  >
  <div class="m-4">
    <pre>retrieved: {retrieved}</pre>
    {#if loading}
      <LoadingSpinner />
    {/if}
  </div>
</div>
