<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import CaretRight from '~icons/ph/caret-right-bold';
  import CheckCircle from '~icons/ph/check-circle';

  import { icons } from './utils';

  export let trigger: any;
  export let label: string;
  export let completed: boolean;
  export let icon: string = 'Trophy';
</script>

<button
  class="flex w-full items-center justify-between rounded-xl px-4 py-4
  {completed ? 'border border-slate-200 border-opacity-100 opacity-50' : 'bg-white dark:bg-dark'}"
  use:melt={trigger}
>
  <div class="flex items-center">
    <svelte:component this={icons[icon] || icons['Trophy']} class="mr-4 h-5 w-5 text-primary" />
    <p
      class="text-left text-[13px]/[24px] dark:text-white {completed
        ? 'font-normal text-slate-500 line-through'
        : 'font-medium text-slate-800'}"
    >
      {label}
    </p>
  </div>
  {#if completed}
    <CheckCircle class="h-6 w-6 text-primary" />
  {:else}
    <CaretRight class="h-5 w-5 text-neutral-400" />
  {/if}
</button>
