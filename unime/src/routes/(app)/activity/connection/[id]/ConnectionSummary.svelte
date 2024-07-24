<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { Connection } from '@bindings/connections/Connection';
  import { open } from '@tauri-apps/plugin-shell';

  import { Image } from '$lib/components';
  import { ArrowSquareOutBoldIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  import { buildIotaExplorerSearchLink } from '../../utils';

  export let connection: Connection;

  let summary = {
    URL: connection.url,
    [$LL.CONNECTION.SUMMARY.FIRST_CONNECTED()]: new Date(connection.first_interacted).toLocaleString(
      $state.profile_settings.locale,
      {
        dateStyle: 'medium',
        timeStyle: 'medium',
      },
    ),
    [$LL.CONNECTION.SUMMARY.LAST_CONNECTED()]: new Date(connection.last_interacted).toLocaleString(
      $state.profile_settings.locale,
      {
        dateStyle: 'medium',
        timeStyle: 'medium',
      },
    ),
  };

  // Holds the link to an external explorer (distributed ledger)
  let explorerLink: string | undefined = undefined;

  // Currently only IOTA DIDs are supported
  if (connection.did?.startsWith('did:iota')) {
    explorerLink = buildIotaExplorerSearchLink(connection.did);
  }

  async function openExplorerLink() {
    if (!explorerLink) return;
    await open(explorerLink);
  }
</script>

<div class="flex flex-col items-center justify-center space-y-4">
  <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
    <div class="flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl bg-white p-2">
      <Image
        id={connection.id}
        imgClass="h-full w-full rounded-2xl"
        iconFallback="Bank"
        iconClass="h-6 w-6 dark:text-slate-800"
      />
    </div>
    <div class="text-center text-2xl font-semibold text-slate-700 dark:text-grey">
      {$LL.CONNECTION.SUMMARY.TITLE()}
      <p class="text-primary">{connection.name}</p>
    </div>
  </div>
  <!-- Details -->
  <div
    class="w-full divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:divide-slate-600 dark:border-slate-600 dark:bg-dark"
  >
    {#each Object.entries(summary) as entry}
      <div class="flex flex-col items-start px-4 py-[10px]">
        <p class="text-[13px]/[24px] font-medium text-slate-400">{entry[0]}</p>
        <p class="break-all text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {entry[1]}
        </p>
      </div>
    {/each}
    <!-- Dev Mode: show DID (and explorer link, if available ) -->
    {#if $state.dev_mode !== 'Off'}
      <div class="flex flex-col items-start px-4 py-[10px]">
        <p class="text-[13px]/[24px] font-medium text-slate-400">DID</p>
        <p class="select-text break-all font-mono text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {connection.did ?? '-'}
        </p>
        {#if explorerLink}
          <div class="flex w-full justify-center pt-2">
            <button
              type="button"
              class="h-10 rounded-lg px-4 text-[13px]/[24px] font-medium text-primary"
              on:click={openExplorerLink}
            >
              <div class="flex items-center">
                <p>View on explorer</p>
                <ArrowSquareOutBoldIcon class="ml-2" />
              </div>
            </button>
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>
