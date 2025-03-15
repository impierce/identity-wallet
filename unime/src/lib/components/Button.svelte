<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import LoadingSpinner from './LoadingSpinner.svelte';

  const dispatch = createEventDispatcher();

  export let label: string;
  // Typing is a workaround to make svelte-check errors go away.
  // TODO: Melt UI triggers should not be passed as props but applied to HTML elements.
  export let trigger:
    | ({ readonly type: 'button' } & { [x: `data-melt-${string}`]: '' } & {
        action: (node: HTMLElement) => { destroy?: (() => void) | undefined };
      })
    | undefined = undefined;
  export let disabled = false;
  export let loading = false;
  export let variant: 'primary' | 'secondary' = 'primary';

  $: variant_classes =
    variant === 'primary'
      ? 'bg-primary text-white dark:text-slate-800'
      : 'bg-white dark:bg-dark text-slate-800 border border-slate-200 dark:border-slate-600 dark:text-grey';
</script>

<!-- TODO: does it make sense to pass in the trigger here? -->
{#if !loading}
  {#if trigger}
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
  {/if}
{:else}
  <!-- Loading -->
  <div class="relative">
    <div class="absolute z-10 flex h-full w-full items-center justify-center">
      <LoadingSpinner />
    </div>
    <button
      class="h-[48px] w-full rounded-xl px-4 py-2 text-[13px]/[24px] font-medium opacity-25 {variant_classes}"
      disabled={true}
      on:click={() => dispatch('click')}
    >
      <!-- No label -->
    </button>
  </div>
{/if}
