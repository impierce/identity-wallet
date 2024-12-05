<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button, Checkbox, TopNavBar } from '$lib/components';

  let checked = false;
</script>

<TopNavBar title={$LL.ONBOARDING.TERMS.NAVBAR_TITLE()} on:back={() => history.back()} />

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.TERMS.TITLE_1()} <span class="text-primary">{$LL.ONBOARDING.TERMS.TITLE_2()}</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.TERMS.SUBTITLE()}
    </p>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    <!-- TODO: feature disabled: "terms and conditions" -->
    <!-- <div
      class="flex items-center justify-between space-x-4 rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <div class="grow">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Data</p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          I have read and accept the <span class="text-primary">terms and conditions</span>
        </p>
      </div>
      <div>
        <button
          use:melt={$root}
          class="flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white dark:border-slate-600 dark:text-dark
            {$isChecked ? 'border-none bg-primary' : 'bg-white dark:bg-dark'}"
          id="checkbox"
        >
          {#if $isChecked}
            <Check class="h-3 w-3" />
          {/if}
          <input use:melt={$input} />
        </button>
      </div>
    </div> -->

    <!-- Use button, not div, to enable focus. -->
    <button
      on:click={() => {
        checked = !checked;
      }}
      class="flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <div class="grow text-left">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {$LL.ONBOARDING.TERMS.OWNERSHIP.TITLE()}
        </p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          {$LL.ONBOARDING.TERMS.OWNERSHIP.DESCRIPTION()}
        </p>
      </div>
      <div class="pl-4">
        <!-- When toggling checkbox, click event bubbles up and is handled by button. -->
        <Checkbox {checked} />
      </div>
    </button>
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/customize/name')} disabled={!checked} />
</div>
