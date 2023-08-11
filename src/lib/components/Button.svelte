<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { melt } from '@melt-ui/svelte';

  const dispatch = createEventDispatcher();

  export let label: string;
  export let trigger: any = undefined;
</script>

<!-- TODO: does it make sense to pass in the trigger here? -->
{#if trigger}
  <button
    use:melt={trigger}
    class="custom h-[48px] rounded-xl bg-indigo-500 px-4 py-2 text-white"
    on:click={() => dispatch('click')}>{label}</button
  >
{:else}
  <button
    class="custom h-[48px] rounded-xl bg-indigo-500 px-4 py-2 text-white"
    on:click={() => dispatch('click')}>{label}</button
  >
{/if}

<style>
  .custom {
    font-size: 14px;
    font-style: normal;
    font-weight: 500;
    line-height: 24px;
  }
</style>
