<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  interface Props {
    id: string;
    label: string;
    placeholder?: string;
    value?: string;
    required?: boolean;
    ref?: HTMLInputElement;
    type?: 'text' | 'date';
    // The latest date a `date` input accepts (YYYY-MM-DD).
    max?: string;
  }

  let {
    id,
    label,
    placeholder,
    value = $bindable(),
    required = false,
    ref = $bindable(),
    type = 'text',
    max,
  }: Props = $props();
</script>

<div class="flex flex-col gap-1">
  <div class="flex items-center justify-between">
    <label for={id} class="truncate text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">{label}</label>
    {#if required}
      <span class="text-[12px]/[14px] font-medium text-primary">
        {$LL.ADD_CREDENTIALS.VALUE_REQUIRED()}
      </span>
    {/if}
  </div>
  <input
    name={id}
    {id}
    {type}
    {max}
    class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-800 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300 dark:[color-scheme:dark]"
    {placeholder}
    bind:value
    bind:this={ref}
  />
</div>
