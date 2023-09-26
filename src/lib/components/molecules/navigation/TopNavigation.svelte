<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { twMerge } from 'tailwind-merge';

  import ChevronLeft from '~icons/lucide/chevron-left';

  const dispatch = createEventDispatcher();

  export let title: string;
</script>

<div
  class={twMerge(
    'relative flex h-[50px] items-center justify-between self-stretch bg-white px-6 py-[13px] text-slate-800 dark:bg-dark dark:text-grey',
    $$props.class,
  )}
>
  <button class="z-10 -ml-2 rounded-full p-2" on:click={() => dispatch('back')}>
    <ChevronLeft class="h-6 w-6" />
  </button>
  {#if title}
    <p class="custom absolute left-0 top-0 flex h-[50px] w-full items-center justify-center font-medium">
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
