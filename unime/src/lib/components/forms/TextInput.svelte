<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  interface Props {
    id: string;
    label: string;
    placeholder?: string;
    value?: string;
    required?: boolean;
    ref?: HTMLInputElement;
  }

  let { id, label, placeholder, value = $bindable(), required = false, ref = $bindable() }: Props = $props();
</script>

<div class="flex flex-col gap-1">
  <div class="flex items-center justify-between">
    <label for={id} class="dark:text-grey truncate text-[14px]/[22px] font-medium text-slate-800">{label}</label>
    {#if required}
      <span class="text-primary text-[12px]/[14px] font-medium">
        {$LL.ADD_CREDENTIALS.VALUE_REQUIRED()}
      </span>
    {/if}
  </div>
  <input
    name={id}
    type="text"
    class="dark:bg-dark w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-800 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:text-slate-300 dark:caret-slate-300"
    {placeholder}
    bind:value
    bind:this={ref}
  />
</div>
