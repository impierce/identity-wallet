<script lang="ts">
  import { fade } from 'svelte/transition';

  import { createTooltip, melt } from '@melt-ui/svelte';

  import InfoFillIcon from '~icons/ph/info-fill';

  export let description: string;

  const {
    elements: { trigger, content, arrow },
    states: { open },
  } = createTooltip({
    positioning: { placement: 'top' },
    openDelay: 0,
    closeDelay: 0,
    forceVisible: true,
  });
</script>

<div class="relative">
  <button
    type="button"
    use:melt={$trigger}
    class="flex size-8 items-center justify-center rounded-full transition-colors hover:bg-slate-100 dark:hover:bg-slate-800"
  >
    <InfoFillIcon class="size-5 text-slate-500" />
  </button>

  {#if $open}
    <div
      use:melt={$content}
      transition:fade={{ duration: 100 }}
      class="z-50 rounded-lg border border-slate-200 bg-white p-2 shadow-lg"
    >
      <div use:melt={$arrow} />
      <p class="m-0 text-xs font-normal text-slate-600">
        {description}
      </p>
    </div>
  {/if}
</div>
