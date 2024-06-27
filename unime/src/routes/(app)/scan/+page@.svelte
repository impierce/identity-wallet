<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import {
    cancel,
    checkPermissions,
    Format,
    openAppSettings,
    requestPermissions,
    scan,
    type PermissionState,
    type Scanned,
  } from '@tauri-apps/plugin-barcode-scanner';
  import { debug, info, warn } from '@tauri-apps/plugin-log';

  import Button from '$lib/components/atoms/Button.svelte';
  import LoadingSpinner from '$lib/components/atoms/LoadingSpinner.svelte';
  import BottomNavBar from '$lib/components/molecules/navigation/BottomNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state } from '$lib/stores';

  import CameraSlash from '~icons/ph/camera-slash';

  let scanning = false;
  let loading = false;

  // We temporarily introduce this type that extends `PermissionState` to handle a possible error when checking for permissions.
  let permissions_nullable: PermissionState | null;

  let mockQrCodeValue = '';

  function onMessage(scanned: Scanned) {
    debug(`Scanned: ${scanned.content}`);
    loading = true;
    dispatch({ type: '[QR Code] Scanned', payload: { form_urlencoded: scanned.content } });
  }

  // from example in plugin-barcode-scanner repo
  async function startScan() {
    let permissions = await checkPermissions()
      .then((permissions) => {
        info(`Permissions to use the camera: ${permissions}`);
        return permissions;
      })
      .catch((error) => {
        warn(`Error checking for permissions to use the camera: ${error}`);
        return null; // possibly return "denied"? or does that imply that the check has been successful, but was actively denied?
      });

    // TODO: handle receiving "prompt-with-rationale" (issue: https://github.com/tauri-apps/plugins-workspace/issues/979)
    if (permissions === 'prompt') {
      info('Requesting camera permissions');
      permissions = await requestPermissions(); // handle in more detail?
      info(`Permissions to use the camera: ${permissions}`);
    }

    permissions_nullable = permissions;

    if (permissions === 'granted') {
      // Scanning parameters
      const formats = [Format.QRCode];
      const windowed = true;

      info(`Starting scan with parameters: { formats: ${formats}, windowed: ${windowed} }`);
      scanning = true;
      scan({ formats, windowed })
        .then((res) => {
          onMessage(res);
        })
        .catch((error) => {
          // TODO: display error to user
          warn(error);
        })
        .finally(() => {
          scanning = false;
        });
    }
  }

  const mockScanError = () => {
    loading = true;
    setTimeout(() => {
      loading = false;
      dispatch({ type: '[QR Code] Scanned', payload: { form_urlencoded: 'foobar' } });
    }, 1_000);
  };

  const mockSiopRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'accept-connection',
        client_name: 'Some other client',
        logo_uri: undefined,
        redirect_uri: 'https://demo.ngdil.com/auth/callback',
        previously_connected: false,
        // domain_validation: {
        //   status: 'Success',
        // },
        domain_validation: {
          status: 'Failure',
          message: 'DomainLinkageConfiguration could not be fetched',
        },
        thuiswinkel_waarborg_validation: {
          status: 'Success',
          name: 'Thuiswinkel Waarborg',
          logo_uri: 'https://www.thuiswinkel.org/Images/logo-thuiswinkel_waarborg.svg',
        },
        // thuiswinkel_waarborg_validation: {
        //   status: 'Failure',
        // },
      },
    });
  };

  const mockShareRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'share-credentials',
        client_name: 'My Client Name',
        logo_uri: undefined,
        options: [$state.credentials[0].id],
      },
    });
  };

  async function cancelScan() {
    await cancel();
    scanning = false;
    // TODO: non-scanning view is visible before redirecting to /me
    // goto('/me');
  }

  onDestroy(async () => {
    debug('/scan: onDestroy() called');
    document.documentElement.querySelector('body')!.classList.remove('transparent');
    await cancelScan();
  });

  onMount(async () => {
    debug('/scan: onMount() called');
    document.documentElement.querySelector('body')!.classList.add('transparent');

    // TODO find a good way to test if not dev_mode. This will have to be checked after $state is loaded.
    startScan();
  });
</script>

<div class="content-height flex flex-col items-stretch">
  <div class="hide-scrollbar grow overflow-x-hidden overflow-y-scroll">
    <div class="flex h-full w-full flex-col">
      {#if !scanning && !loading}
        <!-- This part is only visible when no scanning or loading is happening.
          Only visible when user has not granted permissions to the camera. -->
        <div class="relative flex h-full flex-col items-center justify-center space-y-4 bg-silver p-8 dark:bg-navy">
          <!-- Ask for permissions (only if not given) -->
          {#if permissions_nullable && permissions_nullable !== 'granted'}
            <div class="flex w-3/4 flex-col space-y-4">
              <div class="flex flex-col items-center rounded-lg bg-rose-100 px-8 py-4 text-rose-500">
                <CameraSlash class="m-2 h-8 w-8" />
                <p class="text-center text-[13px]/[24px] font-semibold">{$LL.SCAN.PERMISSION_DENIED()}</p>
              </div>
              <Button label={$LL.SCAN.OPEN_SETTINGS()} on:click={openAppSettings} />
            </div>
          {/if}

          <!-- Dev mode -->
          {#if $state?.dev_mode !== 'Off'}
            <div class="flex w-3/4 flex-col space-y-4">
              <!-- Mocks -->
              <div class="flex flex-col space-y-2">
                <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Mock scans</p>
                <Button variant="secondary" on:click={mockSiopRequest} label="New connection" />
                <Button variant="secondary" on:click={mockShareRequest} label="Share credentials" />
                <Button variant="secondary" on:click={mockScanError} label="Scan error" />
                <div class="flex flex-col space-y-2 rounded-[20px] border border-slate-200 p-2 dark:border-slate-600">
                  <input
                    bind:value={mockQrCodeValue}
                    class="h-12 w-full rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-teal dark:border-slate-600 dark:bg-dark"
                    placeholder="Paste QR code value"
                  />
                  <Button
                    variant="secondary"
                    on:click={() =>
                      dispatch({ type: '[QR Code] Scanned', payload: { form_urlencoded: mockQrCodeValue } })}
                    label="Process QR code"
                  />
                </div>
              </div>
              <!-- Divider -->
              <hr class="border-slate-300 dark:border-slate-500" />
              <Button variant="primary" on:click={startScan} label="Start new scan" />
            </div>
          {/if}
        </div>
      {:else}
        <!-- Scanning or loading/processing -->
        <div class="flex grow flex-col">
          <div class="bg-white p-5 dark:bg-dark">
            <p class="text-3xl font-semibold text-slate-700 dark:text-grey">
              {$LL.SCAN.TITLE_1()} <span class="text-primary">{$LL.SCAN.TITLE_2()}</span>
            </p>
            <p class="mt-4 text-sm font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.SUBTITLE()}
            </p>
          </div>
          <div class="my-container relative grow">
            {#if loading}
              <div class="absolute z-10 h-full w-full bg-silver dark:bg-navy" />
            {/if}
            <div class="barcode-scanner--area--container">
              <div class="square surround-cover">
                <div class="barcode-scanner--area--outer surround-cover">
                  <div
                    class="barcode-scanner--area--inner surround-cover flex items-center justify-center border-2 border-white"
                  >
                    {#if loading}
                      <LoadingSpinner class="z-20 h-12 w-12" />
                    {/if}
                  </div>
                </div>
              </div>
            </div>
            {#if $state?.dev_mode !== 'Off'}
              <div class="fixed bottom-[128px] z-10 flex w-full justify-center">
                <button class="rounded-lg bg-rose-100 px-4 py-3 font-medium text-rose-500" on:click={cancelScan}
                  >{$LL.CANCEL()}</button
                >
              </div>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
  <div class="z-10 shrink-0">
    {#if loading}
      <!-- Disable the BottomNavBar by overlaying a transparent element -->
      <div class="absolute z-10 h-full w-full bg-white opacity-60 dark:bg-dark" />
    {/if}
    <div class="fixed bottom-[var(--safe-area-inset-bottom)] w-full shadow-[0_-4px_20px_0px_rgba(0,0,0,0.03)]">
      <BottomNavBar
        active={'scan'}
        on:me={() => goto('/me')}
        on:scan={() => goto('/scan')}
        on:activity={() => goto('/activity')}
      />
    </div>
  </div>
</div>

<div class="safe-area-top {scanning || loading ? 'bg-white dark:bg-dark' : 'bg-silver dark:bg-navy'}" />
<div class="safe-area-bottom bg-white dark:bg-dark" />

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }

  .my-container {
    width: 100%;
    overflow: hidden;
  }
  .my-container {
    display: flex;
  }

  .square {
    width: 100%;
    position: relative;
    overflow: hidden;
    transition: 0.3s;
  }
  .square:after {
    content: '';
    top: 0;
    display: block;
    padding-bottom: 100%;
  }
  .square > div {
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
  }

  .surround-cover {
    box-shadow: 0 0 0 99999px rgba(0, 0, 0, 0.5);
  }

  .barcode-scanner--area--container {
    width: 75%;
    max-width: min(500px, 80vh);
    margin: auto;
  }
  .barcode-scanner--area--outer {
    display: flex;
  }
  .barcode-scanner--area--inner {
    width: 100%;
    border-radius: 20px;
  }
</style>
