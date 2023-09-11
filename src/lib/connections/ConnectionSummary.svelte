<script lang="ts">
  import BadgeCheck from '~icons/lucide/badge-check';
  import Pencil from '~icons/lucide/pencil';

  export let domain: string;
  export let url: string;
  // export let displayName: string | undefined = undefined;
  export let lastConnected: string;

  let summary = {
    URL: url,
    Domain: domain,
    // displayName,
    Verified: 'yes',
    'First connected': 'n/a',
    'Last connected': lastConnected
  };
</script>

<div
  class="w-full divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:bg-dark"
>
  {#each Object.entries(summary) as entry}
    <div class="flex flex-col items-start px-4 py-[10px]">
      <p class="text-[13px]/[24px] font-medium text-slate-400">{entry[0]}</p>
      <p class="break-all text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
        {entry[1]}
      </p>
    </div>
  {/each}
</div>
