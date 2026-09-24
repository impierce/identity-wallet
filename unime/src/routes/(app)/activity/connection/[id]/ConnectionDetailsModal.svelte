<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { writable, type Writable } from 'svelte/store';
  import { fade, scale } from 'svelte/transition';

  import type { Connection } from '@bindings/connections/Connection';
  import { createDialog, melt } from '@melt-ui/svelte';
  import { open as openExternal } from '@tauri-apps/plugin-shell';

  import { ArrowSquareOutBoldIcon, XBoldIcon } from '$lib/icons';

  import { buildIotaExplorerSearchLink } from '../../utils';

  export let connection: Connection;
  export let open: Writable<boolean> = writable(false);

  const {
    elements: { overlay, content, title, close, portalled },
  } = createDialog({ open, portal: '#portal' });

  $: explorerLink = connection.did.startsWith('did:iota') ? buildIotaExplorerSearchLink(connection.did) : undefined;

  async function openExplorer() {
    if (explorerLink) await openExternal(explorerLink);
  }
</script>

<div use:melt={$portalled}>
  {#if $open}
    <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" transition:fade={{ duration: 150 }}></div>
    <div
      use:melt={$content}
      class="fixed top-1/2 left-1/2 z-50 max-h-[85vh] w-[90vw] max-w-md -translate-x-1/2 -translate-y-1/2 overflow-y-auto rounded-xl bg-background p-6 shadow-lg focus:outline-hidden"
      transition:scale={{ duration: 150, start: 0.96 }}
    >
      <h2 use:melt={$title} class="pr-8 text-[18px]/[26px] font-semibold text-slate-800 dark:text-white">
        {$LL.CONNECTION.DETAILS.TITLE()}
      </h2>

      <div class="mt-5 rounded-xl border border-slate-200 bg-background-alt p-4 dark:border-slate-600">
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          {$LL.CONNECTION.DETAILS.DID()}
        </p>
        <p class="pt-1 font-mono text-[12px]/[19px] font-medium break-all text-slate-800 select-text dark:text-grey">
          {connection.did || '-'}
        </p>
      </div>

      {#if explorerLink}
        <button
          type="button"
          class="mt-4 flex w-full items-center justify-center gap-2 rounded-xl border border-slate-200 px-4 py-3 text-[13px]/[20px] font-medium text-primary dark:border-slate-600"
          on:click={openExplorer}
        >
          {$LL.CONNECTION.DETAILS.VIEW_ON_EXPLORER()}
          <ArrowSquareOutBoldIcon class="size-4" />
        </button>
      {/if}

      <button
        use:melt={$close}
        type="button"
        aria-label={$LL.CLOSE()}
        class="absolute top-4 right-4 flex size-8 items-center justify-center rounded-full border border-slate-200 bg-white dark:border-slate-600 dark:bg-dark"
      >
        <XBoldIcon class="size-4" />
      </button>
    </div>
  {/if}
</div>
