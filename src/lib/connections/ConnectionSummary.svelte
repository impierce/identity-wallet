<script lang="ts">
  import BadgeCheck from '~icons/lucide/badge-check';
  import Pencil from '~icons/lucide/pencil';

  export let id: string;
  export let domain: string;
  export let displayName: string | undefined = undefined;
  export let lastConnected: string;
</script>

<div
  class="grid w-full grid-flow-row auto-rows-max grid-cols-2 items-center gap-y-4 bg-slate-100 p-4"
>
  <!-- <div class="text-slate-400">displayName</div> -->
  <div class="col-span-2 flex items-center justify-between">
    <p class="text-lg font-medium">{displayName ?? domain}</p>
    <button class="rounded-full border p-4"><Pencil class="text-slate-400" /></button>
  </div>

  <div class="text-slate-400">domain</div>
  <div class="flex items-center">
    <p>{domain}</p>
    <BadgeCheck class="ml-2 h-5 w-5 text-blue-500" />
  </div>

  <div class="text-slate-400">first connected</div>
  <div>n/a</div>

  <div class="text-slate-400">lastConnected</div>
  <div>{lastConnected}</div>
</div>
