<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let active = false;
  export let label: string;
</script>

<div class="flex flex-col items-center">
  <!-- Indicator -->
  {#if active}
    <div class="h-[4px] w-full rounded-full bg-primary" />
  {/if}

  <!-- Button -->
  <button class={`rounded-full px-6 ${active ? 'pt-2' : 'pt-3'}`} on:click={() => dispatch('click')}>
    <div class="flex h-12 w-10 flex-col items-center justify-center">
      <!-- Currently, there is no better way of applying conditional Tailwind classes to a named slot inside a component. -->
      <slot name="icon" />
      <div
        class={`pt-[6px] text-[12px]/[20px] font-medium ${
          active ? 'text-slate-800 dark:text-grey' : 'text-slate-300 dark:text-slate-500'
        }`}
      >
        {label}
      </div>
    </div>
  </button>
</div>
