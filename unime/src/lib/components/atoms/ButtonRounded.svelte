<script lang="ts">
  import { createEventDispatcher, type SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';
  import { twMerge } from 'tailwind-merge';

  import { melt } from '@melt-ui/svelte';

  const dispatch = createEventDispatcher();

  export let label: string;
  export let icon: typeof SvelteComponent<SvelteHTMLElements['svg']>;
  export let trigger = undefined; // TODO: add type
</script>

<!-- TODO: does it make sense to pass in the trigger here? -->
{#if trigger}
  <button
    use:melt={trigger}
    class={twMerge(
      'flex h-12 items-center justify-center rounded-full bg-primary px-4 shadow-[0_2px_20px_0px_rgba(0,0,0,0.05)]',
      $$props.class,
    )}
    on:click={() => dispatch('click')}
  >
    <svelte:component this={icon} class="h-6 w-6 text-white" />
    <p class="pl-2 text-[13px]/[24px] font-medium text-white">{label}</p>
  </button>
{:else}
  <button
    class={twMerge(
      'flex h-12 items-center justify-center rounded-full bg-primary px-4 shadow-[0_2px_20px_0px_rgba(0,0,0,0.05)]',
      $$props.class,
    )}
    on:click={() => dispatch('click')}
  >
    <svelte:component this={icon} class="h-6 w-6 text-white" />
    <p class="pl-2 text-[13px]/[24px] font-medium text-white">{label}</p>
  </button>
{/if}

<!-- {#if trigger}
  <button
    use:melt={trigger}
    class="h-[48px] w-full rounded-xl px-4 py-2 text-[13px]/[24px] font-medium disabled:opacity-50 {variant_classes}"
    {disabled}
    on:click={() => dispatch('click')}>{label}</button
  >
{:else}
  <button
    class="h-[48px] w-full rounded-xl px-4 py-2 text-[13px]/[24px] font-medium disabled:opacity-50 {variant_classes}"
    {disabled}
    on:click={() => dispatch('click')}>{label}</button
  >
{/if} -->
