<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { createRadioGroup, melt } from '@melt-ui/svelte';

  export let defaultValue = 'system';

  const {
    elements: { root, item },
    helpers: { isChecked },
    states: { value },
  } = createRadioGroup({
    defaultValue,
  });

  const dispatch = createEventDispatcher();

  // `$value` contains the value of the selected radio button.
  // Dispatch custom event whenever that value changes.
  // ESLint does not understand the reactive statement.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  $: $value, dispatch('change', { value: $value });
</script>

<div use:melt={$root} class="flex flex-col gap-4">
  <!-- System theme radio button -->
  <div
    use:melt={$item('system')}
    class="relative flex items-center justify-between rounded-xl border bg-slate-50 p-4
        {$isChecked('system') ? 'border-primary ring-1 ring-primary' : 'border-slate-200 dark:border-slate-600'}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-slate-800">{$LL.SETTINGS.THEME.SYSTEM()}</p>
    </div>
    <!-- Light -->
    <div class="absolute bottom-0 left-1/4 h-14 w-1/4 rounded-t-xl border border-b-0 border-slate-200 bg-white">
      <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
    </div>
    <!-- Dark -->
    <div class="absolute bottom-0 left-1/2 h-14 w-1/4 rounded-t-xl border-b-0 border-slate-200 bg-blue">
      <p class="p-2 text-sm font-medium text-white">Aa</p>
    </div>
  </div>

  <!-- Light theme radio button -->
  <div
    use:melt={$item('light')}
    class="relative flex items-center justify-between rounded-xl border bg-white p-4
        {$isChecked('light') ? 'border-primary ring-1 ring-primary' : 'border-slate-200 dark:border-slate-600'}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-slate-800">{$LL.SETTINGS.THEME.LIGHT()}</p>
    </div>
    <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl border border-b-0 border-slate-200 bg-slate-100">
      <p class="p-2 text-sm font-medium text-slate-800">Aa</p>
    </div>
  </div>

  <!-- Dark theme radio button -->
  <div
    use:melt={$item('dark')}
    class="relative flex items-center justify-between rounded-xl border bg-blue p-4
        {$isChecked('dark') ? 'border-primary ring-1 ring-primary' : 'border-slate-200 dark:border-slate-600'}"
  >
    <div class="h-12 grow">
      <p class="text-[13px]/[24px] font-medium text-white">{$LL.SETTINGS.THEME.DARK()}</p>
    </div>
    <div class="absolute bottom-0 left-1/4 h-14 w-1/2 rounded-t-xl bg-slate-600">
      <p class="p-2 text-sm font-medium text-white">Aa</p>
    </div>
  </div>
</div>
