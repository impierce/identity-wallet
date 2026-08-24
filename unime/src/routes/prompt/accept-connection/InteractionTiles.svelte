<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { HistoryEvent } from '@bindings/history/HistoryEvent';

  import { countInteractions } from '$lib/utils/history';

  // Only rendered for a connection we have interacted with before, so `interactions` comes from a
  // non-null `connection_data`.
  export let interactions: HistoryEvent[];

  $: counts = countInteractions(interactions);

  $: tiles = [
    { label: $LL.SCAN.CONNECTION_REQUEST.INTERACTIONS(), value: counts.total },
    { label: $LL.SCAN.CONNECTION_REQUEST.SHARED_DATA(), value: counts.shared },
    { label: $LL.SCAN.CONNECTION_REQUEST.RECEIVED_DATA(), value: counts.received },
  ];
</script>

<div class="grid w-full grid-cols-3 gap-2">
  {#each tiles as tile}
    <div
      class="flex flex-col items-center justify-center rounded-xl bg-white px-2 py-4 dark:bg-dark"
    >
      <p class="text-center text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {tile.label}
      </p>
      <p class="pt-1 text-[22px]/[30px] font-semibold text-slate-800 dark:text-grey">
        {tile.value}
      </p>
    </div>
  {/each}
</div>
