<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { Switch } from '$lib/components';
  import { LockSimpleRegularIcon } from '$lib/icons';

  export let label: string;
  export let value: string;
  export let required: boolean;
  export let shared: boolean;

  const dispatch = createEventDispatcher<{ toggle: boolean }>();
</script>

<!--
@component
One requested field of a credential on the share screen. A required field cannot be turned off; an optional one has a
switch, and its value is greyed out while it is not shared.
-->
<div class="flex min-h-14 items-center gap-3 px-[14px] py-[10px]">
  {#if required}
    <div class="flex min-w-0 grow flex-col">
      <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{label}</p>
      <p class="text-[13px]/[24px] font-medium break-words text-slate-800 dark:text-grey">{value}</p>
    </div>
    <span
      class="flex shrink-0 items-center gap-1 rounded-full bg-slate-100 py-0.5 pr-[10px] pl-2 text-[12px]/[20px] font-medium text-slate-500 dark:bg-slate-700 dark:text-slate-300"
    >
      <LockSimpleRegularIcon class="size-3" />
      {$LL.SCAN.SHARE_CREDENTIALS.CLAIMS.REQUIRED()}
    </span>
  {:else}
    <div class="min-w-0 grow">
      <Switch
        checked={shared}
        onCheckedChange={({ next }) => {
          dispatch('toggle', next);
          return next;
        }}
      >
        <span class="flex items-center gap-2">
          <span class="flex min-w-0 grow flex-col">
            <span
              class="text-[12px]/[20px] font-medium {shared ? 'text-slate-500 dark:text-slate-300' : 'text-slate-400'}"
            >
              {label}
            </span>
            <span
              class="text-[13px]/[24px] font-medium break-words {shared
                ? 'text-slate-800 dark:text-grey'
                : 'text-slate-300 dark:text-slate-500'}"
            >
              {value}
            </span>
          </span>
          <span class="shrink-0 text-[12px]/[20px] font-medium text-slate-400">
            {$LL.SCAN.SHARE_CREDENTIALS.CLAIMS.OPTIONAL()}
          </span>
        </span>
      </Switch>
    </div>
  {/if}
</div>
