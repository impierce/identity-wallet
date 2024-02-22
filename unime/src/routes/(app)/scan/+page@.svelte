<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { goto } from '$app/navigation';

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

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import BottomNavBar from '$src/lib/components/molecules/navigation/BottomNavBar.svelte';
  import { state } from '$src/stores';

  import CameraSlash from '~icons/ph/camera-slash';

  let scanning = false;
  // We temporarily introduce this type that extends `PermissionState` to handle a possible error when checking for permissions.
  let permissions_nullable: PermissionState | null;

  function onMessage(scanned: Scanned) {
    debug(`scanned: ${scanned.content}`);
    dispatch({ type: '[QR Code] Scanned', payload: { form_urlencoded: scanned.content } });
    goto('/me');
  }

  function _startScan() {
    info(`starting scan with parameters: { cameraDirection: 'back', windowed: false, formats: [Format.QRCode] }`);
    scanning = true;
    scan({ windowed: true, formats: [Format.QRCode] })
      .then((scanned) => {
        scanning = false;
        onMessage(scanned);
      })
      .catch((error) => {
        scanning = false;
        // TODO: display error
        console.warn(error);
      });
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

    if (permissions === 'prompt') {
      info('Requesting camera permissions');
      permissions = await requestPermissions(); // handle in more detail?
    }

    permissions_nullable = permissions;

    if (permissions === 'granted') {
      // Scanning parameters
      const formats = [Format.QRCode];
      const windowed = true;

      info(`starting scan with parameters: { formats: ${formats}, windowed: ${windowed} }`);
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

  const mockScanSiopRequest = () => {
    const TEST_SIOP_REQUEST_URL_BY_REFERENCE =
      'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.1.234%3A4243%2Fsiop%2Frequest-uri';
    const TEST_SIOP_REQUEST_URL_BY_VALUE =
      'siopv2://idtoken?response_type=id_token+vp_token&response_mode=post&client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&scope=openid&presentation_definition=%7B%22id%22%3A%22Verifiable+Presentation+request+for+sign-on%22%2C%22input_descriptors%22%3A%5B%7B%22id%22%3A%22Request+for+Ferris%27s+Verifiable+Credential%22%2C%22constraints%22%3A%7B%22fields%22%3A%5B%7B%22path%22%3A%5B%22%24.vc.type%22%5D%2C%22filter%22%3A%7B%22type%22%3A%22array%22%2C%22contains%22%3A%7B%22const%22%3A%22PersonalInformation%22%7D%7D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.givenName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.familyName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.email%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.birthdate%22%5D%7D%5D%7D%7D%5D%7D&redirect_uri=http%3A%2F%2Ftest%3A4243%2Fsiop%2Fresponse&nonce=n-0S6_WzA2Mj&client_metadata=%7B%22subject_syntax_types_supported%22%3A%5B%22did%3Akey%22%5D%7D&state=50f04e4d-632a-48c8-bfe5-1ffa71fc88e5';
    dispatch({
      type: '[QR Code] Scanned',
      payload: {
        form_urlencoded: TEST_SIOP_REQUEST_URL_BY_VALUE,
      },
    });
  };

  const mockSiopRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'accept-connection',
        client_name: 'Some other client',
        logo_uri: 'https://demo.ngdil.com/imgs/ngdil.svg',
        redirect_uri: 'https://demo.ngdil.com/auth/callback',
        previously_connected: false,
        // logo_uri: 'https://picsum.photos/200'
      },
    });
  };

  const mockShareRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'share-credentials',
        client_name: 'My Client Name',
        logo_uri: 'https://demo.ngdil.com/imgs/ngdil.svg',
        // logo_uri: 'https://picsum.photos/200',
        options: [$state.credentials[0].id],
      },
    });
  };

  const mockScanCredentialOffer = (amount: number) => {
    if (amount == 1) {
      dispatch({
        type: '[QR Code] Scanned',
        payload: {
          form_urlencoded:
            'openid-credential-offer://?credential_offer=%7B%22credential_issuer%22%3A%22http%3A%2F%2F192.168.1.127%3A9090%2F%22%2C%22credentials%22%3A%5B%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22PersonalInformation%22%5D%7D%7D%5D%2C%22grants%22%3A%7B%22urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code%22%3A%7B%22pre-authorized_code%22%3A%220YI5DXtuCltKyNa5%22%2C%22user_pin_required%22%3Afalse%7D%7D%7D',
        },
      });
    } else if (amount > 1) {
      // dispatch({
      //   type: '[QR Code] Scanned',
      //   payload: {
      //     'openid-credential-offer://?credential_offer=%7B%22credential_issuer%22%3A%22http%3A%2F%2F10.15.185.12%3A9090%2F%22%2C%22credentials%22%3A%5B%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22PersonalInformation%22%5D%7D%7D%2C%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22DriverLicenseCredential%22%5D%7D%7D%5D%2C%22grants%22%3A%7B%22urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code%22%3A%7B%22pre-authorized_code%22%3A%22crzhlepEdqjsXD3I%22%2C%22user_pin_required%22%3Afalse%7D%7D%7D'
      //   }
      // });
      state.set({
        ...$state,
        current_user_prompt: {
          type: 'credential-offer',
          issuer_name: 'Some issuer',
          // logo_uri: 'https://picsum.photos/200',
          logo_uri: 'https://demo.ngdil.com/imgs/ngdil.svg',
          credential_offer: {
            credential_issuer: 'http://10.15.185.12:9090/',
            credentials: [
              {
                format: 'jwt_vc_json',
                credential_definition: { type: ['VerifiableCredential', 'PersonalInformation'] },
              },
              {
                format: 'jwt_vc_json',
                credential_definition: {
                  type: ['VerifiableCredential', 'DriverLicenseCredential'],
                },
              },
            ],
            grants: {
              'urn:ietf:params:oauth:grant-type:pre-authorized_code': {
                'pre-authorized_code': 'crzhlepEdqjsXD3I',
                user_pin_required: false,
              },
            },
          },
          display: [
            {
              name: 'My first Credential',
              logo: { url: 'https://picsum.photos/200.svg' },
            },
            {
              name: 'My second Credential',
            },
          ],
        },
      });
    }
  };

  async function cancelScan() {
    await cancel();
    scanning = false;
    // TODO: non-scanning view is visible before redirecting to /me
    // goto('/me');
  }

  // lifecycle functions
  onDestroy(async () => {
    console.log('onDestroy: /scan');
    document.documentElement.querySelector('body')!!.classList.remove('transparent');
    await cancelScan();
  });

  onMount(async () => {
    console.log('onMount: /scan');
    document.documentElement.querySelector('body')!!.classList.add('transparent');
    // permissionsGiven = await checkScanPrerequisites();

    // TODO find a good way to test if not dev_mode. This will have to be checked after $state is loaded.
    startScan();
  });
</script>

<!-- <main
  class="transition-colors-250 grid flex-1 grid-rows-[2fr_auto] bg-orange-300 transition-transform"
  class:transparent={isMobile}
>
  <div class="flex flex-col items-center p-6">
    <h1 class="p-6 text-3xl font-medium">{selected.label}</h1>
    <div class="overflow-y-auto">
      <div class="mr-2">
        <svelte:component this={selected.component} {onMessage} />
      </div>
    </div>
  </div>
</main> -->

<!-- ############ -->
<!-- <div class="h-screen">
  <p class="absolute m-4 w-fit rounded-full bg-slate-600 px-4 py-2 text-slate-200">
    scanning: {scanning}
  </p>

  {#if !scanning}
    <div class="h-screen bg-orange-300" class:invisible={scanning}>
      <Button variant="secondary" on:click={startScan}>SCAN</Button>
    </div>
  {:else}

  <div class="h-screen bg-gradient-to-r from-violet-600 via-green-400 to-violet-500">
        <div class="">
            <p>Aim your camera at a QR code</p>
        </div>
        <div class="absolute top-1/2 left-1/2">
            <div class="w-20 h-20 bg-slate-300 rounded overflow-hidden"></div>
        </div>
    </div>
  {/if}
</div> -->

<!-- {#if scanning}
    scanning
    <div class="h-screen bg-gradient-to-br from-violet-600 to-transparent" />
  {/if} -->

<!-- scanning in progress -->
<!-- <div
    class="h-screen bg-gradient-to-br from-violet-600 to-transparent"
    class:invisible={!scanning}
  />
  <div class="h-full w-full">test123</div> -->

<!-- ############ -->

<div class="content-height flex flex-col items-stretch">
  <!-- <div class="flex h-screen flex-col items-stretch"> -->
  <div class="hide-scrollbar grow overflow-x-hidden overflow-y-scroll">
    <div class="flex h-full w-full flex-col">
      <!-- visible when NOT scanning -->
      <div
        class:invisible={scanning}
        class="bg-silver dark:bg-navy relative flex h-full flex-col items-center justify-center space-y-4 p-8"
      >
        {#if permissions_nullable !== 'granted'}
          <div class="flex w-3/4 flex-col items-center space-y-4">
            <div class="flex flex-col items-center rounded-lg bg-rose-100 px-8 py-4 text-rose-500">
              <CameraSlash class="m-2 h-8 w-8" />
              <p class="text-center text-[13px]/[24px] font-semibold">{$LL.SCAN.PERMISSION_DENIED()}</p>
            </div>
            <Button label={$LL.SCAN.OPEN_SETTINGS()} on:click={openAppSettings} />
          </div>
        {/if}

        {#if $state?.dev_mode !== 'Off'}
          <div class="flex flex-col space-y-4">
            <!-- Mock -->
            <div class="flex flex-col space-y-2">
              <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Mock</p>
              <Button variant="secondary" on:click={mockSiopRequest} label="New connection" />
              <Button variant="secondary" on:click={mockShareRequest} label="Share credentials" />
            </div>
            <!-- UniCore (local) -->
            <div class="flex flex-col space-y-2">
              <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">UniCore (local)</p>
              <Button
                variant="secondary"
                on:click={() => mockScanCredentialOffer(1)}
                label="Credential Offer (single)"
              />
              <Button
                variant="secondary"
                on:click={() => mockScanCredentialOffer(2)}
                label="Credential Offer (multi)"
              />
            </div>
            <!-- Divider -->
            <hr />
            <Button variant="primary" on:click={startScan} label="Start new scan" />
          </div>
        {/if}
      </div>

      <!-- visible during scanning -->
      <div class="flex grow flex-col" class:invisible={!scanning}>
        <div class="dark:bg-dark bg-white p-5">
          <p class="dark:text-grey text-3xl font-semibold text-slate-700">
            {$LL.SCAN.TITLE_1()} <span class="text-primary">{$LL.SCAN.TITLE_2()}</span>
          </p>
          <p class="mt-4 text-sm font-medium text-slate-500 dark:text-slate-300">
            {$LL.SCAN.SUBTITLE()}
          </p>
        </div>
        <div class="scanner-background">
          <!-- this background simulates the camera view -->
        </div>
        <div class="my-container grow">
          <div class="barcode-scanner--area--container">
            <div class="relative">
              <!-- <p>Aim your camera at a QR code</p> -->
            </div>
            <div class="square surround-cover">
              <div class="barcode-scanner--area--outer surround-cover">
                <div class="barcode-scanner--area--inner" />
              </div>
            </div>
          </div>
          {#if $state?.dev_mode !== 'Off'}
            <div class="fixed bottom-[128px] left-[calc(50%_-_42px)]">
              <button class="rounded-lg bg-rose-100 px-4 py-3 font-medium text-rose-500" on:click={cancelScan}
                >{$LL.CANCEL()}</button
              >
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
  <div class="shrink-0">
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

<div class="safe-area-top {scanning ? 'dark:bg-dark bg-white' : 'bg-silver dark:bg-navy'}" />
<div class="safe-area-bottom dark:bg-dark bg-white" />

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }

  .invisible {
    display: none;
  }

  .full-height {
    height: 100%;
    border: 1px solid red;
  }

  /* p {
    font-family: sans-serif;
    text-align: center;
    font-weight: 600;
  } */

  .my-container {
    width: 100%;
    /* height: 100%; */
    overflow: hidden;
    /* border: 1px solid green; */
  }
  .my-container {
    display: flex;
  }

  .relative {
    position: relative;
    z-index: 1;
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
    width: 80%;
    max-width: min(500px, 80vh);
    margin: auto;
  }
  .barcode-scanner--area--outer {
    display: flex;
    border-radius: 1em;
  }
  .barcode-scanner--area--inner {
    width: 100%;
    margin: 1rem;
    border: 2px solid #fff;
    box-shadow:
      0px 0px 2px 1px rgb(0 0 0 / 0.5),
      inset 0px 0px 2px 1px rgb(0 0 0 / 0.5);
    border-radius: 1rem;
  }

  .scanner-background {
    background: linear-gradient(45deg, #673ab7, transparent);
    background-position: 45% 50%;
    background-size: cover;
    background-repeat: no-repeat;
    /* border: 1px solid blue; */
  }
</style>
