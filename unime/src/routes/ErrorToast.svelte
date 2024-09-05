<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  import { fade, fly } from 'svelte/transition';

  import { WarningRegularIcon, XRegularIcon } from '$lib/icons';

  const dispatch = createEventDispatcher();

  export let autoDismissAfterMs = 0; // A value of 0 means the toast will never auto-dismiss
  export let title;
  export let detail: string | undefined = undefined;

  onMount(() => {
    if (autoDismissAfterMs > 0) {
      setTimeout(() => {
        dispatch('dismissed');
      }, autoDismissAfterMs);
    }
  });
</script>

<div
  class="relative flex items-center overflow-hidden rounded-xl border border-rose-400 bg-rose-100 p-4"
  in:fly={{ x: 200 }}
  out:fade={{ duration: 200 }}
>
  <WarningRegularIcon class="h-6 w-6 min-w-6 text-rose-400" />
  <div class="ml-4 mr-2 flex w-full flex-col overflow-hidden text-ellipsis break-words">
    <p class="truncate text-[12px]/[20px] font-bold text-slate-800">
      {title}
    </p>
    {#if detail}
      <p class="line-clamp-2 text-[12px]/[16px] text-slate-800">{detail}</p>
    {/if}
  </div>
  <button class="-mr-2 rounded-full p-3" on:click={() => dispatch('dismissed')}>
    <XRegularIcon class="h-4 w-4 text-slate-800" />
  </button>
</div>
