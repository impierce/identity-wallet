<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';
  import { createCheckbox, createRadioGroup, melt } from '@melt-ui/svelte';

  import Button from '$src/lib/components/Button.svelte';
  import Checkbox from '$src/lib/components/Checkbox.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { onboarding_state } from '$src/stores';

  import Check from '~icons/ph/check-bold';

  const {
    elements: { root, item },
    helpers: { isChecked },
    states: { value }
  } = createRadioGroup({
    defaultValue: 'system'
  });

  $: {
    $onboarding_state.theme = $value;
  }

  $: {
    // system dark or explicitly chosen, then set dark
    if ($value === 'dark' || window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark');
      console.log('dark mode enabled');
    } else {
      document.documentElement.classList.remove('dark');
      console.log('light mode enabled');
    }
  }
</script>

<!-- <TopNavigation title="Appearance" on:back={() => history.back()} /> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      Let's go! Choose your <span class="text-primary">appearance</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">Are you more of a night owl?</p>
  </div>
  <!-- <div use:melt={$root} class="flex flex-col"> -->
  <!-- System -->
  <div class="mt-4 flex flex-col space-y-4">
    <div
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-50 p-4
        {$isChecked('system') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">System</p>
      </div>
      <!-- Checkbox -->
      <Checkbox root={$item('system')} input={undefined} isChecked={$isChecked('system')} />
      <!-- <Checkbox root={$item('system')} input={undefined} isChecked={$isChecked('system')} /> -->
      <!-- <div> -->
      <!-- <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
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
      <div
        class="absolute bottom-0 left-1/4 h-14 w-1/4 rounded-t-xl border border-b-0 border-slate-200 bg-white"
      >
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
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4
        {$isChecked('light') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">Light</p>
      </div>
      <!-- Checkbox -->
      <Checkbox root={$item('light')} input={undefined} isChecked={$isChecked('light')} />
      <!-- <div>
        <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
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
      <div
        class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl border border-b-0 border-slate-200 bg-slate-100"
      >
        <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
      </div>
    </div>
  </div>
  <!-- Dark -->
  <div class="mt-4 flex flex-col space-y-4">
    <div
      class="relative flex items-center justify-between rounded-xl border border-slate-800 bg-slate-800 p-4
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
            rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
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
</div>
<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label="Continue" on:click={() => goto('/welcome/customize/avatar')} />
</div>
