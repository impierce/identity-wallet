<script lang="ts">
  import { slide } from 'svelte/transition';

  import { createCollapsible, melt } from '@melt-ui/svelte';

  import { CaretDownBoldIcon } from '$lib/icons';

  export let defaultOpen = false;

  const {
    elements: { root, content, trigger },
    states: { open },
  } = createCollapsible({
    defaultOpen,
  });
</script>

<div use:melt={$root} class="overflow-hidden rounded-xl bg-background p-4">
  <button use:melt={$trigger} class="flex w-full items-center justify-between text-left">
    <slot name="title" />
    <div class="transition-transform duration-300 {$open ? 'rotate-180' : ''}">
      <CaretDownBoldIcon class="size-4" />
    </div>
  </button>

  {#if $open}
    <div use:melt={$content} transition:slide={{ duration: 300 }}>
      <div class="prose-xs prose py-2 text-xs dark:prose-invert">
        <slot />
      </div>
    </div>
  {/if}
</div>
