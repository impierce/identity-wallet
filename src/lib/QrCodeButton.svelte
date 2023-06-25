<script lang="ts">
  import { QrCode } from 'svelte-heros-v2';
  import { dispatch } from '$lib/dispatcher';
  import { LoadingSpinner } from '@impierce/ui-components';
  import { info } from '@tauri-apps/plugin-log';

  import {
    Format,
    scan,
    cancel,
    checkPermissions,
    openAppSettings
    // Scanned
  } from '@tauri-apps/plugin-barcode-scanner';
  import { goto } from '$app/navigation';

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
    // checkPermissions().then((res) => console.log(res)).catch((err) => console.log(err));
    const userAgent = navigator.userAgent.toLowerCase();
    const isMobile = userAgent.includes('android') || userAgent.includes('iphone');
    info(`userAgent: ${userAgent}, isMobile: ${isMobile}`);
    if (true) {
      await checkPermissions().then((res) =>
        info(`app has permissions to access the camera: ${res}`)
      );
      info(
        `starting scan with parameters: { cameraDirection: 'back', windowed: false, formats: [Format.QRCode] }`
      );
      goto('/scanner');
    } else {
      dispatch({ type: '[QR Code] Scanned', payload: { rawString: 'MOCK' } });
    }
    // dispatch({ type: '[DID] Create new', payload: { display_name: display_name, password } });
    // loading = false;
  }}
>
  <div class="flex rounded-full bg-violet-700 p-4 shadow-md shadow-violet-700">
    {#if loading}
      <LoadingSpinner class="text-white" size="38" />
    {:else}
      <QrCode class="text-white" size="38" />
    {/if}
  </div>
</button>
