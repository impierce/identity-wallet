<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import { state } from '$src/stores';

  import BadgeCheck from '~icons/lucide/badge-check';
  import Pencil from '~icons/lucide/pencil';

  import type { Connection } from './types';

  export let connection: Connection;

  let summary = {
    URL: connection.url,
    // Verified: 'no',
    [$LL.CONNECTION.SUMMARY.FIRST_CONNECTED()]: new Date(connection.first_interacted).toLocaleString($state.locale, {
      dateStyle: 'medium',
      timeStyle: 'medium',
    }),
    [$LL.CONNECTION.SUMMARY.LAST_CONNECTED()]: new Date(connection.last_interacted).toLocaleString($state.locale, {
      dateStyle: 'medium',
      timeStyle: 'medium',
    }),
  };
</script>

<div class="flex flex-col items-center justify-center space-y-4">
  <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
    <div class="flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl bg-white p-4">
      <Image
        id={connection.id}
        imgClass="h-full w-full rounded-2xl"
        iconFallback="Bank"
        iconClass="h-6 w-6 dark:text-slate-800"
      />
    </div>
    <div class="text-center text-2xl font-semibold text-slate-700 dark:text-grey">
      {$LL.CONNECTION.SUMMARY.TITLE()}
      <p class="text-primary">{connection.client_name}</p>
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
  </div>
</div>
