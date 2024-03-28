<script lang="ts">
  import { createEventDispatcher, SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import ArrowDown from '~icons/ph/arrow-down-bold';
  import ArrowUp from '~icons/ph/arrow-up-bold';

  export let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  export let label: string;
  export let active: boolean = false;
  const dispatch = createEventDispatcher();
  export let sortingOrder: 'ascending' | 'descending' | undefined = undefined;
</script>

<button
  on:click={() => dispatch('click')}
  class={`my-1 flex w-full gap-2 border p-[10px] text-slate-800 dark:text-grey ${
    active ? 'rounded-lg border-grey bg-silver dark:border-slate-600 dark:bg-navy' : 'border-transparent'
  }`}
>
  <svelte:component this={icon} />
  <p class="text-sm font-medium">{label}</p>
  {#if active}
    <div class="absolute right-2">
      {#if sortingOrder == 'descending'}
        <ArrowDown class="text-primary" />
      {:else if sortingOrder == 'ascending'}
        <ArrowUp class="text-primary" />
      {/if}
    </div>
  {/if}
</button>
