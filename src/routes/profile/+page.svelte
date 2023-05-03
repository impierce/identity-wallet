<script lang="ts">
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import { state } from '../stores';
  import LL from '../i18n/i18n-svelte';
  import { Button } from '@impierce/ui-components';
  import { dispatch } from '../lib/dispatcher';
  import { useFocus } from 'svelte-navigator';
  import { getPhoto, ResultType, Source } from "tauri-plugin-camera-api";
  import readQR from '@paulmillr/qr/decode';

  let request = "";

  function imageToUint8Array(image: HTMLImageElement): Uint8Array {
    const canvas = document.createElement('canvas');
    canvas.width = image.width;
    canvas.height = image.height;

    const ctx = canvas.getContext('2d');
    ctx.drawImage(image, 0, 0);

    const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const data = new Uint8Array(imageData.data.buffer);

    return data;
  }

  async function getDimensionsFromBase64(base64String: string): Promise<{ width: number; height: number }> {
      const img = new Image();
      img.src = base64String;
      await img.decode();

      return { width: img.width, height: img.height };
  }

  async function decodeQRCode() {
    try {
      const { data } = await getPhoto({
        resultType: ResultType.Base64,
        source: Source.Camera
      });
      const img = new Image();
      img.src = `data:image/png;base64,${data}`;

      const dimensions = await getDimensionsFromBase64(img.src);
      img.width = dimensions.width;
      img.height = dimensions.height;
      
      const data2 = imageToUint8Array(img);
      request = readQR({ height: img.height, width: img.width, data: data2 });

    } catch (e) {
      console.error(e);
    }

    await getRequest();
  }

  const getRequest = async () =>
    dispatch({ type: '[Authenticate] Get request', payload: { request_url: request } });

  const sendResponse = async () =>
    dispatch({ type: '[Authenticate] Send response', payload: { user_claims: claims } });

  const registerFocus = useFocus();

  let claims = new Map<string, string>();
  let values = {};

  function updateMap() {
    // Clear the map and add each key-value pair from the object
    claims.clear();
    Object.entries($state?.active_requested_claims).forEach(([key, value]) => {
      if (values[key]) {
        claims.set(key, values[key]);
      }
    });
  }
</script>

<div class="space-y-8 p-8">
  <h1 class="font-serif text-2xl font-semibold">
    {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
  </h1>
  <p>{$LL.CREATE_IDENTITY_SUCCESS()}!</p>
  <div
    class="truncate rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600"
    data-testid="primary-did"
  >
    {$state?.active_profile?.primary_did}
  </div>

  {#if $state?.active_requested_claims}
    <div
      class="truncate rounded-lg bg-gray-300 px-4 py-2 font-mono text-sm font-semibold text-gray-600"
      data-testid="claims"
    >
      <p data-testid="label-prompt-username" class="text-gray-600">{$LL.ENTER_CLAIMS()}</p>
  <!-- TODO: replace with ui-components/Input -->

      {#each Object.entries($state?.active_requested_claims) as [key, value]}
        <p>{key}: {value}</p>
        <div>
          <input
            type="text"
            data-testid="input-username"
            class="w-full rounded-lg border px-4 py-2 shadow focus:outline-none focus:ring-2 focus:ring-violet-600"
            placeholder=""
            bind:value={values[key]} on:input={updateMap}
            use:registerFocus
          />
        </div>
      {/each}
    </div>
    <Button label={$LL.AUTHENTICATE()} on:clicked={sendResponse} />
  {:else}
    <Button label={$LL.SCAN_QRCODE()} on:clicked={decodeQRCode} />
  {/if}

</div>
