<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade, scale } from 'svelte/transition';

  import { createDialog, melt } from '@melt-ui/svelte';

  import { Button, Checkbox, TopNavBar } from '$lib/components';
  import { XBoldIcon } from '$lib/icons';

  const {
    elements: { trigger, overlay, content, title, description, close },
    states: { open },
  } = createDialog();

  let checked_terms = false;
  let checked_ownership = false;
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

    {#if $open}
      <!-- TODO: when using "portalled", dark mode is not applied correctly -->
      <!-- <div use:melt={$portalled}> -->
      <div>
        <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" transition:fade={{ duration: 150 }} />
        <div
          class="fixed left-1/2 top-1/2 z-50 max-h-[85vh] w-[90vw]
            -translate-x-1/2 -translate-y-1/2 overflow-y-auto rounded-xl
            bg-background p-6 shadow-lg"
          transition:scale={{
            duration: 150,
            start: 0.96,
          }}
          use:melt={$content}
        >
          <h2 use:melt={$title} class="mb-4 text-lg font-bold">{$LL.ONBOARDING.TERMS.T_AND_C.DIALOG_TITLE()}</h2>
          <p class="mb-2 text-xs font-medium text-slate-500 dark:text-slate-300">
            {$LL.ONBOARDING.TERMS.T_AND_C.LAST_UPDATED()}
          </p>
          <p use:melt={$description} class="rounded-lg text-xs font-medium">
            {$LL.ONBOARDING.TERMS.T_AND_C.TL_DR()}
          </p>
          <!-- acts as <hr> -->
          <div class="my-2 h-px w-full bg-brand" />
          <p class="text-xs font-light">{$LL.ONBOARDING.TERMS.T_AND_C.FULL()}</p>
          <div class="mt-6 flex justify-end gap-4">
            <Button
              on:click={() => {
                checked_terms = false;
              }}
              variant="secondary"
              trigger={$close}
              label={$LL.REJECT()}
            />

            <Button on:click={() => (checked_terms = true)} trigger={$close} label={$LL.ACCEPT()} />
            <!-- <button
              use:melt={$close}
              class="bg-magnum-100 text-magnum-900 inline-flex h-8 items-center
                    justify-center rounded-sm px-4 font-medium leading-none"
            >
              Save changes
            </button> -->
          </div>
          <button
            use:melt={$close}
            aria-label="close"
            class="text-magnum-800 hover:bg-magnum-100 focus:shadow-magnum-400 absolute right-4 top-4 inline-flex
                h-6 w-6 appearance-none items-center justify-center
                rounded-full p-1"
          >
            <XBoldIcon class="size-4" />
          </button>
        </div>
      </div>
    {/if}

    <!-- Use button, not div, to enable focus. -->
    <button
      use:melt={$trigger}
      on:click={() => {
        // checked_terms = !checked_terms;
      }}
      class="flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <div class="grow text-left">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {$LL.ONBOARDING.TERMS.T_AND_C.TITLE()}
        </p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          {$LL.ONBOARDING.TERMS.T_AND_C.DESCRIPTION()}
        </p>
      </div>
      <div class="pl-4">
        <!-- When toggling checkbox, click event bubbles up and is handled by button. -->
        <Checkbox checked={checked_terms} />
      </div>
    </button>

    <!-- Use button, not div, to enable focus. -->
    <button
      on:click={() => {
        checked_ownership = !checked_ownership;
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
        <Checkbox checked={checked_ownership} />
      </div>
    </button>
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label={$LL.CONTINUE()}
    on:click={() => goto('/welcome/customize/name')}
    disabled={!checked_terms || !checked_ownership}
  />
</div>
