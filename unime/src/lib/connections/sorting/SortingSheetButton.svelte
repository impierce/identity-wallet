<script lang="ts">
  import { createEventDispatcher, SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  export let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  export let label: string;
  export let active: boolean = false;
  const dispatch = createEventDispatcher();
  export let reversed: boolean;
</script>

<button
  on:click={() => dispatch('click')}
  class={`dark:text-grey my-1 flex w-full gap-2 border p-[10px] text-slate-800 ${
    active ? 'border-grey bg-silver dark:bg-navy rounded-lg dark:border-slate-600' : 'border-transparent'
  }`}
>
  <svelte:component this={icon} />
  <p class="text-sm font-medium">{label}</p>
  {#if active}
    <div class="absolute right-2">
      <p class="text-primary text-sm font-medium">
        {label === 'Alphabetical'
          ? reversed
            ? 'Z to A'
            : 'A to Z'
          : label === 'Date Issued'
            ? reversed
              ? 'Newest'
              : 'Oldest'
            : label === 'Date Added'
              ? reversed
                ? 'Oldest'
                : 'Newest'
              : ''}
      </p>
    </div>
  {/if}
</button>
