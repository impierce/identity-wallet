<script lang="ts">
  import { goto } from '$app/navigation';
  import { onDestroy, onMount } from 'svelte';

  import { Button as ButtonDeprecated } from '@impierce/ui-components';
  import {
    Format,
    type Scanned,
    cancel,
    openAppSettings,
    scan
  } from '@tauri-apps/plugin-barcode-scanner';
  import { debug, info } from '@tauri-apps/plugin-log';

  // import Scanner from '$lib/Scanner.svelte';
  import { dispatch } from '$lib/dispatcher';
  import Button from '$src/lib/components/Button.svelte';
  import { state } from '$src/stores';

  import { checkScanPrerequisites } from './utils';

  // let selected = {
  //   label: 'Scanner',
  //   component: Scanner
  //   // icon: 'i-ph-scan'
  // };

  // let isMobile = true;

  //   function insecureRenderHtml(html) {
  //     messages.update((r) => [
  //       {
  //         html:
  //           `<pre><strong class="text-accent dark:text-darkAccent">[${new Date().toLocaleTimeString()}]:</strong> ` +
  //           html +
  //           "</pre>",
  //       },
  //       ...r,
  //     ]);
  //   }

  // variables
  let scanning = false;
  let permissionsGiven = false;

  // functions
  function onMessage(scanned: Scanned) {
    debug(`scanned: ${scanned.content}`);
    dispatch({ type: '[QR Code] Scanned', payload: { form_urlencoded: scanned.content } });
    goto('/me');
  }

  function startScan() {
    info(
      `starting scan with parameters: { cameraDirection: 'back', windowed: false, formats: [Format.QRCode] }`
    );
    scanning = true;
    scan({ windowed: true, formats: [Format.QRCode] })
      .then((scanned) => {
        scanning = false;
        onMessage(scanned);
      })
      .catch((error) => {
        scanning = false;
        onMessage(error);
      });
  }

  const mockScanSiopRequest = () => {
    const TEST_SIOP_REQUEST_URL_BY_REFERENCE =
      'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.1.234%3A4243%2Fsiop%2Frequest-uri';
    const TEST_SIOP_REQUEST_URL_BY_VALUE =
      'siopv2://idtoken?response_type=id_token+vp_token&response_mode=post&client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&scope=openid&presentation_definition=%7B%22id%22%3A%22Verifiable+Presentation+request+for+sign-on%22%2C%22input_descriptors%22%3A%5B%7B%22id%22%3A%22Request+for+Ferris%27s+Verifiable+Credential%22%2C%22constraints%22%3A%7B%22fields%22%3A%5B%7B%22path%22%3A%5B%22%24.vc.type%22%5D%2C%22filter%22%3A%7B%22type%22%3A%22array%22%2C%22contains%22%3A%7B%22const%22%3A%22PersonalInformation%22%7D%7D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.givenName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.familyName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.email%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.birthdate%22%5D%7D%5D%7D%7D%5D%7D&redirect_uri=http%3A%2F%2Ftest%3A4243%2Fsiop%2Fresponse&nonce=n-0S6_WzA2Mj&client_metadata=%7B%22subject_syntax_types_supported%22%3A%5B%22did%3Akey%22%5D%7D&state=50f04e4d-632a-48c8-bfe5-1ffa71fc88e5';
    dispatch({
      type: '[QR Code] Scanned',
      payload: { form_urlencoded: TEST_SIOP_REQUEST_URL_BY_VALUE }
    });
  };

  const mockSiopRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'accept-connection'
        // options: ['c798fc11-e78e-432c-85f2-790be603a581']
      }
    });
  };

  const mockShareRequest = () => {
    state.set({
      ...$state,
      current_user_prompt: {
        type: 'share-credentials',
        options: [$state.credentials[0].id]
      }
    });
  };

  const mockScanCredentialOffer = (amount: number) => {
    if (amount == 1) {
      dispatch({
        type: '[QR Code] Scanned',
        payload: {
          form_urlencoded:
            'openid-credential-offer://?credential_offer=%7B%22credential_issuer%22%3A%22http%3A%2F%2F192.168.1.127%3A9090%2F%22%2C%22credentials%22%3A%5B%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22PersonalInformation%22%5D%7D%7D%5D%2C%22grants%22%3A%7B%22urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code%22%3A%7B%22pre-authorized_code%22%3A%220YI5DXtuCltKyNa5%22%2C%22user_pin_required%22%3Afalse%7D%7D%7D'
        }
      });
    } else if (amount > 1) {
      // dispatch({
      //   type: '[QR Code] Scanned',
      //   payload: {
      //     form_urlencoded:
      //       'openid-credential-offer://?credential_offer=%7B%22credential_issuer%22%3A%22http%3A%2F%2F10.15.185.12%3A9090%2F%22%2C%22credentials%22%3A%5B%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22PersonalInformation%22%5D%7D%7D%2C%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22DriverLicenseCredential%22%5D%7D%7D%5D%2C%22grants%22%3A%7B%22urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code%22%3A%7B%22pre-authorized_code%22%3A%22crzhlepEdqjsXD3I%22%2C%22user_pin_required%22%3Afalse%7D%7D%7D'
      //   }
      // });
      state.set({
        ...$state,
        current_user_prompt: {
          type: 'credential-offer',
          credential_offer: {
            credential_issuer: 'http://10.15.185.12:9090/',
            credentials: [
              {
                format: 'jwt_vc_json',
                credential_definition: { type: ['VerifiableCredential', 'PersonalInformation'] }
              },
              {
                format: 'jwt_vc_json',
                credential_definition: {
                  type: ['VerifiableCredential', 'DriverLicenseCredential']
                }
              }
            ],
            grants: {
              'urn:ietf:params:oauth:grant-type:pre-authorized_code': {
                'pre-authorized_code': 'crzhlepEdqjsXD3I',
                user_pin_required: false
              }
            }
          }
        }
      });
    }
  };

  // lifecycle functions
  onDestroy(async () => {
    document.documentElement.querySelector('body')!!.classList.remove('transparent');
    scanning = false;
    await cancel();
  });

  onMount(async () => {
    document.documentElement.querySelector('body')!!.classList.add('transparent');
    permissionsGiven = await checkScanPrerequisites();
    if (permissionsGiven) {
      startScan();
    }
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

<div class="h-full w-full">
  <!-- visible when NOT scanning -->
  <div
    class:invisible={scanning}
    class="relative flex h-full flex-col items-center justify-center bg-neutral-100 p-8"
  >
    {#if !permissionsGiven}
      <div class="flex flex-col items-center space-y-2">
        <div class="rounded-lg bg-red-100 px-8 py-4 text-red-600">Permissions not given</div>
        <Button label="Open settings" on:click={openAppSettings} />
      </div>
      <p class="my-4 h-[1px] w-full bg-slate-200" />
    {/if}
    <div class="flex flex-col space-y-2">
      <Button variant="secondary" on:click={mockSiopRequest} label="Connection request (SIOPv2)" />
      <Button variant="secondary" on:click={mockShareRequest} label="Share request (VP)" />
      <Button
        variant="secondary"
        on:click={() => mockScanCredentialOffer(1)}
        disabled
        label="Mock Credential Offer (single)"
      />
      <Button
        variant="secondary"
        on:click={() => mockScanCredentialOffer(2)}
        label="Mock Credential Offer (multi)"
      />
      <Button
        variant="secondary"
        on:click={() =>
          dispatch({
            type: '[QR Code] Scanned',
            payload: {
              form_urlencoded:
                'openid-credential-offer://?credential_offer_uri=https://api.ngdil-demo.tanglelabs.io/api/offers/creds/u08LmjU8lAcTwx7pLMpy0'
            }
          })}
        label="Dominique (student)"
      />
      <Button variant="primary" on:click={startScan} label="Start new scan" />
    </div>
  </div>

  <!-- visible during scanning -->
  <div class="fill-screen" class:invisible={!scanning}>
    <div class="mx-5 mb-6 pt-5">
      <p class="text-3xl font-semibold text-slate-700">
        Scan a <span class="text-primary">QR Code</span>
      </p>
      <p class="mt-4 text-sm font-medium text-slate-500">
        Bring a QR Code into view of this screen to start an interaction.
      </p>
    </div>
    <div class="scanner-background">
      <!-- this background simulates the camera view -->
    </div>
    <div class=" my-container">
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
      <div class="fixed bottom-12 left-[calc(50%_-_42px)]">
        <ButtonDeprecated
          class="bg-red-100 font-semibold text-red-500 shadow"
          on:click={() => goto('/me')}>Cancel</ButtonDeprecated
        >
      </div>
    </div>
  </div>
</div>

<style>
  .invisible {
    display: none;
  }

  .full-height {
    height: 100%;
    border: 1px solid red;
  }

  .fill-screen {
    height: calc(
      100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 136px
    ); /* 64px: Bottom Nav Bar, 136px: Top section */
    width: calc(100vw - var(--safe-area-inset-left) - var(--safe-area-inset-right));
  }

  /* p {
    font-family: sans-serif;
    text-align: center;
    font-weight: 600;
  } */

  .my-container {
    width: 100%;
    height: 100%;
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
    box-shadow: 0px 0px 2px 1px rgb(0 0 0 / 0.5), inset 0px 0px 2px 1px rgb(0 0 0 / 0.5);
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
