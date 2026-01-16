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

<div use:melt={$root} class="overflow-hidden rounded-xl bg-background">
  <button use:melt={$trigger} class="flex w-full items-center justify-between p-4 text-left">
    <slot name="title" />
    <div class="transition-transform duration-300 {$open ? 'rotate-180' : ''}">
      <CaretDownBoldIcon class="size-4" />
    </div>
  </button>

  {#if $open}
    <div use:melt={$content} transition:slide={{ duration: 300 }}>
      <div class="prose prose-sm p-4 pt-0 text-sm dark:prose-invert">
        <slot />
      </div>
    </div>
  {/if}
</div>
