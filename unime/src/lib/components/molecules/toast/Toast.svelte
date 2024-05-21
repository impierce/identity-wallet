<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';

  import { fade, fly } from 'svelte/transition';

  import Warning from '~icons/ph/warning';
  import X from '~icons/ph/x';

  const dispatch = createEventDispatcher();

  let dismissible = true; // TODO: should this even be configured?
  export let duration = 0; // 0 equals infinite
  export let title;
  export let detail: string | undefined = undefined;

  let remaining_percent = 100; // initialize to 100%

  onMount(() => {
    if (duration > 0) {
      const interval = setInterval(() => {
        remaining_percent -= 1;
        if (remaining_percent <= 0) {
          clearInterval(interval);
          dispatch('dismissed');
        }
      }, duration / 100);
    }
  });
</script>

<div
  class="relative flex items-center overflow-hidden rounded-xl border border-rose-400 bg-rose-100 p-4"
  in:fly={{ x: 200 }}
  out:fade={{ duration: 200 }}
>
  <Warning class="h-6 w-6 min-w-6 text-rose-400" />
  <div class="ml-4 flex flex-col pr-12">
    <p class="truncate text-[12px]/[20px] font-bold text-slate-800">
      {title}
    </p>
    {#if detail}
      <p class="line-clamp-2 text-[12px]/[16px] text-slate-800">{detail}</p>
    {/if}
  </div>
  {#if dismissible}
    <button class="absolute right-0 top-0 rounded-full p-3" on:click={() => dispatch('dismissed')}>
      <X class="h-4 w-4 text-slate-800" />
    </button>
  {/if}
  {#if duration > 0}
    <div class="absolute bottom-0 left-0 h-[4px] bg-rose-400" style={`width: ${remaining_percent}%`}></div>
  {/if}
</div>
