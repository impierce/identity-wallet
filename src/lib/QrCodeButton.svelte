<script lang="ts">
  import { goto } from '$app/navigation';

  import {
    cancel,
    checkPermissions,
    Format,
    openAppSettings, // Scanned
    scan,
  } from '@tauri-apps/plugin-barcode-scanner';
  import { info, warn } from '@tauri-apps/plugin-log';

  import { dispatch } from '$lib/dispatcher';

  import QrCode from '~icons/heroicons/qr-code';
  import LoadingSpinner from '~icons/svg-spinners/3-dots-fade';

  let loading = false;

  const rawString =
    'siopv2://idtoken?' +
    'response_type=vp_token&' +
    'client_id=did%3Aexample%3A123&' +
    'redirect_uri=https%3A%2F%2Fexample.com&' +
    'nonce=nonce&' +
    'presentation_definition=%7B%22id%22%3A%22Verifiable+Presentation+request+for+sign-on%22%2C%22input_descriptors%22%3A%5B%7B%22id%22%3A%22Request+for+Ferris%27s+Verifiable+Credential%22%2C%22constraints%22%3A%7B%22fields%22%3A%5B%7B%22path%22%3A%5B%22%24.type%22%5D%2C%22filter%22%3A%7B%22type%22%3A%22array%22%2C%22contains%22%3A%7B%22const%22%3A%22PersonalInformation%22%7D%7D%7D%2C%7B%22path%22%3A%5B%22%24.credentialSubject.givenName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.credentialSubject.familyName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.credentialSubject.email%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.credentialSubject.birthdate%22%5D%7D%5D%7D%7D%5D%7D';
</script>

<button
  on:click={async () => {
    // loading = true;
    const userAgent = navigator.userAgent.toLowerCase();
    const isMobile = userAgent.includes('android') || userAgent.includes('iphone');
    info(`userAgent: ${userAgent}, isMobile: ${isMobile}`);
    if (true) {
      await checkPermissions()
        .then((res) => {
          info(`app has permissions to access the camera: ${res}`);
          // TODO: ask user to open settings (https://github.com/impierce/identity-wallet/issues/23)
          if (res === 'granted' || res === 'default') {
            goto('/scan');
          } else {
            warn('app does not have permissions to access the camera');
            openAppSettings();
          }
        })
        .catch((err) => {
          warn(`error checking permissions: ${err}`);
          const TEST_SIOP_REQUEST_URL =
            'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.1.234%3A4242%2Fsiop%2Frequest-uri';
          // const TEST_SIOP_REQUEST_URL = 'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.178.42%3A4242%2Fsiop%2Frequest-uri';
          dispatch({
            type: '[Authenticate] Read request',
            authorization_request: TEST_SIOP_REQUEST_URL,
          });
        });
    } else {
      dispatch({ type: '[QR Code] Scanned', form_urlencoded: 'MOCK' });
    }
    // loading = false;
  }}
>
  <div class="flex rounded-full bg-violet-700 p-4 shadow-md">
    {#if loading}
      <LoadingSpinner class="h-9 w-9 text-white" />
    {:else}
      <QrCode class="h-9 w-9 text-white" />
    {/if}
  </div>
</button>
