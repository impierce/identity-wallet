<script lang="ts">
  import { writable, type Writable } from 'svelte/store';
  import { fade } from 'svelte/transition';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import { createDialog, melt } from '@melt-ui/svelte';

  import { XBoldIcon } from '$lib/icons';

  export let credential: DisplayCredential;
  export let open: Writable<boolean> = writable(false);

  // Instead of default portal in `<body>`, we create the portal at ID `#portal` in root layout.
  // This way the modal opens within the safe area.
  const {
    elements: { overlay, content, title, close, portalled },
  } = createDialog({ open, portal: '#portal' });

  $: rawData = JSON.stringify(credential, null, 2);
</script>

<!--
  @component
  Dev Mode: a full screen modal showing the raw JSON data of a credential.

  @prop credential - The credential to display.
  @prop open - A writable store to control the visibility of the modal.
-->

<div use:melt={$portalled}>
  {#if $open}
    <!-- Overlay -->
    <div use:melt={$overlay} class="fixed inset-0 z-40 bg-black/50" transition:fade={{ duration: 150 }}></div>

    <div
      use:melt={$content}
      class="fixed inset-0 z-40 flex flex-col bg-background-alt pt-(--safe-area-inset-top) pb-(--safe-area-inset-bottom) focus:outline-hidden"
      transition:fade={{ duration: 150 }}
    >
      <!-- Header -->
      <div class="flex shrink-0 items-center justify-between gap-4 px-4 py-3">
        <p use:melt={$title} class="truncate text-[16px]/[24px] font-semibold text-slate-800 dark:text-white">
          Raw data
        </p>
        <button use:melt={$close} type="button" aria-label="Close raw data" class="p-1.5">
          <XBoldIcon class="size-5 dark:text-white" />
        </button>
      </div>

      <!-- Scrollable in both axes -->
      <div class="grow overflow-auto px-4 pb-4">
        <pre
          class="font-mono text-[11px]/[16px] whitespace-pre text-slate-800 select-text dark:text-grey">{rawData}</pre>
      </div>
    </div>
  {/if}
</div>
