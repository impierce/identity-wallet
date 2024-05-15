<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { createRadioGroup, melt } from '@melt-ui/svelte';
  import { debug } from '@tauri-apps/plugin-log';

  export let defaultValue = 'system';

  const {
    elements: { root, item },
    helpers: { isChecked },
    states: { value },
  } = createRadioGroup({
    defaultValue,
  });

  const dispatch = createEventDispatcher();

  $: {
    debug(`prefers-color-scheme: dark? ${window.matchMedia('(prefers-color-scheme: dark)').matches}`);
    if ($value === 'dark') {
      // dark
      document.documentElement.classList.add('dark');
      debug('Dark mode enabled');
    } else if ($value === 'light') {
      // light
      document.documentElement.classList.remove('dark');
      debug('Light mode enabled');
    } else {
      // system
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.documentElement.classList.add('dark');
        debug('Dark mode enabled');
      } else {
        document.documentElement.classList.remove('dark');
        debug('Light mode enabled');
      }
    }
    dispatch('change', { value: $value });
  }
</script>

<div use:melt={$root}>
  <!-- System theme radio button -->
  <div class="mt-4 flex flex-col space-y-4">
    <div
      use:melt={$item('system')}
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-50 p-4 dark:border-slate-600
        {$isChecked('system') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">{$LL.SETTINGS.THEME.SYSTEM()}</p>
      </div>
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

  <!-- Light theme radio button -->
  <div class="mt-4 flex flex-col space-y-4">
    <div
      use:melt={$item('light')}
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600
        {$isChecked('light') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800">{$LL.SETTINGS.THEME.LIGHT()}</p>
      </div>
      <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl border border-b-0 border-slate-200 bg-slate-100">
        <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
      </div>
    </div>
  </div>

  <!-- Dark theme radio button -->
  <div class="mt-4 flex flex-col space-y-4">
    <div
      use:melt={$item('dark')}
      class="relative flex items-center justify-between rounded-xl border border-slate-200 bg-slate-800 p-4 dark:border-slate-600
        {$isChecked('dark') ? 'ring ring-primary' : ''}"
    >
      <div class="h-12 grow">
        <p class="text-[13px]/[24px] font-medium text-white">{$LL.SETTINGS.THEME.DARK()}</p>
      </div>
      <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl bg-slate-600">
        <p class="p-2 text-sm font-medium text-white">Aa</p>
      </div>
    </div>
  </div>
</div>
