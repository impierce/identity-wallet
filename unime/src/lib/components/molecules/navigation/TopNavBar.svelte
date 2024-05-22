<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { twMerge } from 'tailwind-merge';

  import CaretLeft from '~icons/ph/caret-left-bold';

  const dispatch = createEventDispatcher();

  export let title: string;
  export let disabled = false;
</script>

<div
  class={twMerge(
    'relative flex h-[50px] items-center justify-between self-stretch bg-silver px-6 py-[13px] text-slate-800 dark:bg-navy dark:text-grey',
    $$props.class,
  )}
>
  <button class="z-30 -ml-2 rounded-full p-2 disabled:opacity-25" on:click={() => dispatch('back')} {disabled}>
    <CaretLeft class="h-5 w-5" />
  </button>
  {#if title}
    <p class="custom absolute left-0 top-0 z-10 flex h-[50px] w-full items-center justify-center font-medium">
      {title}
    </p>
  {/if}
  <div class="z-10">
    <slot />
  </div>
</div>

<style>
  .custom {
    font-size: 13px;
    font-style: normal;
    line-height: 24px;
  }
</style>
