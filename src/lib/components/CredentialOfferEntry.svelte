<script lang="ts">
  import { goto } from '$app/navigation';
  import { writable } from 'svelte/store';

  import { createCheckbox, melt } from '@melt-ui/svelte';

  import Check from '~icons/ph/check-bold';

  const {
    elements: { root, input },
    helpers: { isChecked },
  } = createCheckbox({
    defaultChecked: true,
    disabled: true,
  });

  export let index: number | string;
  export let title = '';
  export let description = '';
  export let color: string | undefined = undefined;
</script>

<div class="flex h-[60px] items-center justify-start rounded-xl bg-white p-[5px] dark:bg-dark">
  <!-- Logo -->
  <div class="mr-[15px] {color} flex h-[50px] w-[50px] flex-col items-center justify-center overflow-hidden rounded-lg">
    <slot name="logo" />
  </div>
  <!-- Text -->
  <div class="flex grow flex-col items-start">
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{title}</p>
    <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
      {description}
    </p>
  </div>

  <!-- Checkbox -->
  <button
    use:melt={$root}
    class="mr-2 flex h-6 w-6 appearance-none items-center justify-center rounded-md border-[1.5px] border-slate-300 p-[6px] text-white data-[disabled]:opacity-50 dark:border-slate-600
    {isChecked ? 'border-none bg-primary' : 'bg-white dark:bg-dark'}"
    id="checkbox-{index}"
  >
    {#if $isChecked}
      <Check class="h-3 w-3" />
    {/if}
    <input use:melt={$input} />
  </button>
</div>
