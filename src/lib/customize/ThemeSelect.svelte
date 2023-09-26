<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { createRadioGroup, melt } from '@melt-ui/svelte';

  import Checkbox from '$src/lib/components/Checkbox.svelte';

  import Check from '~icons/ph/check-bold';

  export let defaultValue = 'system';

  const {
    elements: { root, item },
    helpers: { isChecked },
    states: { value }
  } = createRadioGroup({
    defaultValue
  });

  const dispatch = createEventDispatcher();

  $: {
    console.log('prefers-color-scheme: dark?', window.matchMedia('(prefers-color-scheme: dark)').matches);
    if ($value === 'dark') {
      // dark
      document.documentElement.classList.add('dark');
      console.log('dark mode enabled');
    } else if ($value === 'light') {
      // light
      document.documentElement.classList.remove('dark');
      console.log('light mode enabled');
    } else {
      // system
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.classList.add('dark');
        console.log('dark mode enabled');
      } else {
        document.documentElement.classList.remove('dark');
        console.log('light mode enabled');
      }
    }
    dispatch('change', { value: $value });
  }
</script>

<!-- System -->
<div class="mt-4 flex flex-col space-y-4">
  <div
    class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-50 p-4 dark:border-slate-600
        {$isChecked('system') ? 'ring ring-primary' : ''}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-slate-800">System</p>
    </div>
    <!-- Checkbox -->
    <!-- <Checkbox root={$item('system')} input={undefined} isChecked={$isChecked('system')} /> -->

    <!-- Using a custom checkbox here, so dark mode is not applied automatically -->
    <button
      use:melt={$item('system')}
      class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked('system') ? 'border-none bg-primary' : 'bg-white'}"
      id="checkbox"
    >
      {#if $isChecked('system')}
        <Check class="h-3 w-3" />
      {/if}
    </button>
    <!-- <Checkbox root={$item('system')} input={undefined} isChecked={$isChecked('system')} /> -->
    <!-- <div> -->
    <!-- <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button> -->
    <!-- </div> -->
    <!-- Theme preview -->
    <!-- Light -->
    <div class="absolute bottom-0 left-1/4 h-14 w-1/4 rounded-t-xl border border-b-0 border-slate-200 bg-white">
      <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
    </div>
    <!-- Dark -->
    <div class="absolute bottom-0 left-1/2 h-14 w-1/4 rounded-t-xl bg-slate-500">
      <p class="p-2 text-sm font-medium text-white">Aa</p>
    </div>
  </div>
</div>
<!-- Light -->
<div class="mt-4 flex flex-col space-y-4">
  <div
    class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600
        {$isChecked('light') ? 'ring ring-primary' : ''}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-slate-800">Light</p>
    </div>
    <!-- Checkbox -->
    <!-- <Checkbox root={$item('light')} input={undefined} isChecked={$isChecked('light')} /> -->
    <button
      use:melt={$item('light')}
      class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked('light') ? 'border-none bg-primary' : 'bg-white'}"
      id="checkbox"
    >
      {#if $isChecked('light')}
        <Check class="h-3 w-3" />
      {/if}
    </button>
    <!-- <div>
        <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button>
      </div> -->
    <!-- Theme preview -->
    <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl border border-b-0 border-slate-200 bg-slate-100">
      <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
    </div>
  </div>
</div>
<!-- Dark -->
<div class="mt-4 flex flex-col space-y-4">
  <div
    class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-800 p-4 dark:border-slate-600
        {$isChecked('dark') ? 'ring ring-primary' : ''}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-white">Dark</p>
    </div>
    <!-- Checkbox -->
    <Checkbox root={$item('dark')} input={undefined} isChecked={$isChecked('dark')} />
    <!-- <div> -->
    <!-- <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button> -->
    <!-- </div> -->
    <!-- Theme preview -->
    <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl bg-slate-600">
      <p class="p-2 text-sm font-medium text-white">Aa</p>
    </div>
  </div>
</div>
<!-- </div> -->
