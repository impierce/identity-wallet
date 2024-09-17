<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { CaretLeftBoldIcon } from '$lib/icons';

  const dispatch = createEventDispatcher();

  export let title: string;
</script>

<div class="relative grid h-[48px] place-items-center px-4 text-sm font-medium">
  <button class="absolute left-4 top-1/2 -translate-y-1/2 transform" on:click={() => dispatch('back')}>
    <CaretLeftBoldIcon class="h-5 w-5" />
  </button>
  <span>{title}</span>
</div>
