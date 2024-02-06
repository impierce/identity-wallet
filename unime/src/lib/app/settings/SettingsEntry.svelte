<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import CaretRight from '~icons/ph/caret-right-bold';

  const dispatch = createEventDispatcher();
  export let icon: any;
  export let title: string;
  export let hasCaretRight: boolean = true;
  export let textRight: string | undefined = undefined;

  // TODO: remove this dev flag
  export let todo: boolean = false;
</script>

<button
  class="flex h-14 items-center space-x-4 w-full rounded-xl bg-white p-4 dark:bg-dark {todo ? 'opacity-30' : ''}"
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
