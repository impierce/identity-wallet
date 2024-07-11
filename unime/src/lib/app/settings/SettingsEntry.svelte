<script lang="ts">
  import { createEventDispatcher, type SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import CaretRight from '~icons/ph/caret-right-bold';

  const dispatch = createEventDispatcher();

  export let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  export let title: string;
  export let hasCaretRight = true;
  export let textRight: string | undefined = undefined;
  export let disabled = false;
</script>

<button
  class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark {disabled ? 'opacity-30' : ''}"
  on:click={() => dispatch('click')}
>
  <svelte:component this={icon} class="h-5 w-5 text-primary" />
  <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
    {title}
  </p>
  {#if hasCaretRight}
    <svelte:component this={CaretRight} class="h-4 w-4 text-slate-500" />
  {:else if textRight}
    <p class="text-[13px]/[24px] font-medium text-primary">{textRight}</p>
  {/if}
  <!-- Slot for any content which is not text or caret, such as a Switch -->
  <slot />
</button>
