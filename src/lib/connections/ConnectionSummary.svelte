<script lang="ts">
  import BadgeCheck from '~icons/lucide/badge-check';
  import Pencil from '~icons/lucide/pencil';

  export let domain: string;
  export let displayName: string | undefined = undefined;
  export let lastConnected: string;

  let summary = {
    Domain: domain,
    // displayName,
    Verified: 'yes',
    'First connected': 'n/a',
    'Last connected': lastConnected
  };
</script>

<div
  class="w-full divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white"
>
  {#each Object.entries(summary) as entry}
    <div class="flex flex-col items-start px-4 py-[10px]">
      <p class="text-[13px]/[24px] font-medium text-slate-400">{entry[0]}</p>
      <p class="break-all text-[13px]/[24px] font-medium text-slate-800">{entry[1]}</p>
    </div>
  {/each}
</div>
