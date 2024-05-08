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

  <!-- Active button gets 4px less padding to accommodate indicator. -->
  <button class={`rounded-full px-6 ${active ? 'pt-2' : 'pt-3'}`} on:click={() => dispatch('click')}>
    <div
      class={`flex h-12 w-10 flex-col items-center justify-center ${
        active ? 'text-primary' : 'text-slate-800 dark:text-grey'
      }`}
    >
      <!-- Slot accepts SVG icons, which bring their own dimensions. -->
      <slot name="icon" />
      <div class="pt-[6px] text-[12px]/[20px] font-medium">
        {label}
      </div>
    </div>
  </button>
</div>
