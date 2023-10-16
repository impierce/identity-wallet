<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import Button from '$src/lib/components/Button.svelte';
  import ThemeSelect from '$src/lib/customize/ThemeSelect.svelte';
  import { determineTheme } from '$src/routes/utils';
  import { onboarding_state } from '$src/stores';

  let value: 'system' | 'light' | 'dark' = 'system';

  $: {
    $onboarding_state.theme = value;
    determineTheme(window.matchMedia('(prefers-color-scheme: dark)').matches, value);
  }
</script>

<!-- <TopNavigation title="Appearance" on:back={() => history.back()} /> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      Choose your app <span class="text-primary">appearance</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Are you more of a night owl?</p>
  </div>

  <ThemeSelect on:change={(e) => (value = e.detail.value)} />

  <!-- <div class="mt-4 flex flex-col space-y-4">
    <div
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-50 p-4
        {$isChecked('system') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">System</p>
      </div>
      <Checkbox root={$item('system')} input={undefined} isChecked={$isChecked('system')} />
      <div
        class="absolute bottom-0 left-1/4 h-14 w-1/4 rounded-t-xl border border-b-0 border-slate-200 bg-white"
      >
        <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
      </div>
      <div class="absolute bottom-0 left-1/2 h-14 w-1/4 rounded-t-xl bg-slate-500">
        <p class="p-2 text-sm font-medium text-white">Aa</p>
      </div>
    </div>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    <div
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4
        {$isChecked('light') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">Light</p>
      </div>
      <Checkbox root={$item('light')} input={undefined} isChecked={$isChecked('light')} />
      <div
        class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl border border-b-0 border-slate-200 bg-slate-100"
      >
        <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
      </div>
    </div>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    <div
      class="relative flex items-center justify-between rounded-xl border border-slate-800 bg-slate-800 p-4
        {$isChecked('dark') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-white">Dark</p>
      </div>
      <Checkbox root={$item('dark')} input={undefined} isChecked={$isChecked('dark')} />
      <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl bg-slate-600">
        <p class="p-2 text-sm font-medium text-white">Aa</p>
      </div>
    </div>
  </div> -->
</div>
<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label="Continue" on:click={() => goto('/welcome/customize/avatar')} />
</div>
