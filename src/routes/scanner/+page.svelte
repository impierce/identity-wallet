<script lang="ts">
  import { goto } from '$app/navigation';
  import Scanner from '$lib/Scanner.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { Button } from '@impierce/ui-components';

  let selected = {
    label: 'Scanner',
    component: Scanner
    // icon: 'i-ph-scan'
  };

  let isMobile = true;

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

  function onMessage(value) {
    console.log(value);
    dispatch({ type: '[QR Code] Scanned', payload: { rawString: value.content } });
    goto('/profile');
  }

  const html = document.querySelector('body');
  html.classList.add('transparent');

  import { scan, Format, cancel } from '@tauri-apps/plugin-barcode-scanner';
  import { onDestroy, onMount } from 'svelte';
  let scanning = false;

  function startScan() {
    scanning = true;
    scan({ windowed: true, formats: [Format.QRCode] })
      .then((res) => {
        scanning = false;
        onMessage(res);
      })
      .catch((error) => {
        scanning = false;
        onMessage(error);
      });
  }

  onDestroy(async () => {
    await cancel();
    scanning = false;
  });

  onMount(() => startScan());
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

<div class="fill-screen">
  <div class:invisible={scanning}>
    <Button on:click={startScan}>Scan</Button>
  </div>

  <div class="scanning full-height" class:invisible={!scanning}>
    <div class="scanner-background">
      <!-- this background simulates the camera view -->
    </div>
    <div class="full-height my-container">
      <div class="barcode-scanner--area--container">
        <div class="relative">
          <p>Aim your camera at a QR code</p>
        </div>
        <div class="square surround-cover">
          <div class="barcode-scanner--area--outer surround-cover">
            <div class="barcode-scanner--area--inner" />
          </div>
        </div>
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
    /* border: 1px solid red; */
  }

  .fill-screen {
    height: calc(100vh - env(safe-area-inset-top) - env(safe-area-inset-bottom));
    width: calc(100vw - env(safe-area-inset-left) - env(safe-area-inset-right));
  }

  p {
    color: #fff;
    font-family: sans-serif;
    text-align: center;
    font-weight: 600;
  }

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
