<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  const dispatch = createEventDispatcher();

  export let label: string;
  export let trigger: any = undefined; // TODO: add type
  export let disabled: boolean = false;
  export let variant: 'primary' | 'secondary' = 'primary';

  $: variant_classes =
    variant === 'primary'
      ? 'bg-primary text-white dark:text-slate-800'
      : 'bg-white dark:bg-dark text-slate-800 border border-slate-200 dark:border-slate-600 dark:text-grey';
</script>

<!-- TODO: does it make sense to pass in the trigger here? -->
{#if trigger}
  <button
    use:melt={trigger}
    class="custom h-[48px] w-full rounded-xl px-4 py-2 text-[13px]/[24px] font-medium disabled:opacity-50 {variant_classes}"
    {disabled}
    on:click={() => dispatch('click')}>{label}</button
  >
{:else}
  <button
    class="custom h-[48px] w-full rounded-xl px-4 py-2 text-[13px]/[24px] font-medium disabled:opacity-50 {variant_classes}"
    {disabled}
    on:click={() => dispatch('click')}>{label}</button
  >
{/if}
