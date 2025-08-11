<script lang="ts">
  import { onMount } from 'svelte';

  import { fade, fly } from 'svelte/transition';

  import { InfoRegularIcon, WarningRegularIcon, XRegularIcon } from '$lib/icons';

  interface Props {
    variant: 'error' | 'info';
    dismissible?: boolean;
    autoDismissAfterMs?: number; // A value of 0 means the toast will never auto-dismiss
    title: string;
    detail?: string;
    ondismissed?: () => void;
  }

  let { variant, dismissible = true, autoDismissAfterMs = 0, title, detail, ondismissed }: Props = $props();

  onMount(() => {
    if (autoDismissAfterMs > 0) {
      setTimeout(() => {
        ondismissed?.();
      }, autoDismissAfterMs);
    }
  });
</script>

<!-- teal-50: #f0fdfa -->
<div
  class={`relative flex items-center overflow-hidden rounded-xl border  p-4 ${variant === 'error' ? 'border-rose-400 bg-rose-100' : 'border-primary bg-[#f0fdfa]'}`}
  in:fly={{ x: 200 }}
  out:fade={{ duration: 200 }}
>
  {#if variant === 'error'}
    <WarningRegularIcon class="h-6 w-6 min-w-6 text-rose-400" />
  {:else}
    <InfoRegularIcon class="size-6 min-w-6 text-primary" />
  {/if}
  <div class="ml-4 mr-2 flex w-full flex-col overflow-hidden text-ellipsis break-words">
    <p class="truncate text-[12px]/[20px] font-bold text-slate-800">
      {title}
    </p>
    {#if detail}
      <p class="line-clamp-2 text-[12px]/[16px] text-slate-800">{detail}</p>
    {/if}
  </div>
  {#if dismissible}
    <button class="-mr-2 rounded-full p-3" onclick={() => ondismissed?.()}>
      <XRegularIcon class="h-4 w-4 text-slate-800" />
    </button>
  {/if}
</div>
