<script lang="ts">
  import { melt } from '@melt-ui/svelte';
  import CheckCircle from '~icons/ph/check-circle';
  import Trophy from '~icons/ph/trophy-fill';
  import CaretRight from '~icons/ph/caret-right-bold';

  export let trigger: any;
  export let label: string;
  export let completed: boolean;
</script>

<button
  class={`flex w-full items-center justify-between rounded-lg px-4 py-4 ${
    completed ? 'border opacity-60' : 'bg-white'
  }`}
  use:melt={trigger}
>
  <div class="flex items-center">
    <Trophy class="mr-4 h-5 w-5 text-indigo-500" />
    <p class={`font-medium text-left ${completed ? 'line-through' : ''}`}>{label}</p>
  </div>
  {#if completed}
    <CheckCircle class="h-6 w-6 text-indigo-500" />
  {:else}
    <CaretRight class="h-5 w-5 text-neutral-500" />
  {/if}
</button>
