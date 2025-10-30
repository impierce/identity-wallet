<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade, scale } from 'svelte/transition';

  import { createDialog, melt } from '@melt-ui/svelte';

  import { Button, Checkbox, TopNavBar } from '$lib/components';
  import { XBoldIcon } from '$lib/icons';

  const {
    elements: { trigger, overlay, content, title, description, close, portalled },
    states: { open },
  } = createDialog({
    // Portal to root layout inside the `div` that sets the `dark` class. This ensures that dark mode works.
    portal: '#portal',
  });

  let checked_terms = false;
  let checked_ownership = false;
</script>

<TopNavBar title={$LL.ONBOARDING.TERMS.NAVBAR_TITLE()} on:back={() => history.back()} />

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="dark:text-grey pb-4 text-3xl font-semibold text-slate-700">
      {$LL.ONBOARDING.TERMS.TITLE_1()} <span class="text-primary">{$LL.ONBOARDING.TERMS.TITLE_2()}</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.TERMS.SUBTITLE()}
    </p>
  </div>
  <div class="mt-4 flex flex-col space-y-4">
    {#if $open}
      <div use:melt={$portalled}>
        <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" transition:fade={{ duration: 150 }}></div>
        <div
          data-component="Modal"
          use:melt={$content}
          class="bg-background fixed left-1/2 top-1/2 z-50 max-h-[85vh]
            w-[90vw] -translate-x-1/2 -translate-y-1/2 overflow-y-auto
            rounded-xl p-6 shadow-lg"
          transition:scale={{
            duration: 150,
            start: 0.96,
          }}
        >
          <h2 use:melt={$title} class="mb-4 text-lg font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.DIALOG_TITLE()}
          </h2>
          <p class="mb-2 text-xs font-medium text-slate-500 dark:text-slate-300">
            {$LL.ONBOARDING.TERMS.T_AND_C.LAST_UPDATED()}
          </p>
          <p use:melt={$description} class="rounded-lg text-xs font-medium">
            {$LL.ONBOARDING.TERMS.T_AND_C.TL_DR()}
          </p>

          <!-- acts as <hr> -->
          <div class="bg-brand my-2 h-px w-full"></div>
          <!-- Terms of Use -->
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.AGREEMENT.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.AGREEMENT.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DEFINITIONS.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DEFINITIONS.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.USER_RESPONSIBILITIES.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.USER_RESPONSIBILITIES.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DATA_OWNERSHIP.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DATA_OWNERSHIP.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DATA_VISIBILITY.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.DATA_VISIBILITY.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.INTELLECTUAL_PROPERTY_RIGHTS.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.INTELLECTUAL_PROPERTY_RIGHTS.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.PROHIBITED_ACTIVITIES.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.PROHIBITED_ACTIVITIES.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.THIRD_PARTY_SERVICES.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.THIRD_PARTY_SERVICES.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LIABILITY.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LIABILITY.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.INDEMNIFICATION.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.INDEMNIFICATION.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.MODIFICATIONS.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.MODIFICATIONS.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LAW_AND_JURISDIFICATION.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LAW_AND_JURISDIFICATION.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.SEVERABILITY.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.SEVERABILITY.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LANGUAGE.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.LANGUAGE.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.ENTIRE_AGREEMENT.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.ENTIRE_AGREEMENT.DESCRIPTION()}
          </p>
          <h3 class="my-1 text-xs font-bold">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.CONTACT.TITLE()}
          </h3>
          <p class="mb-3 text-xs font-light">
            {$LL.ONBOARDING.TERMS.T_AND_C.FULL.CONTACT.DESCRIPTION()}
          </p>

          <!-- First focusable element gets the focus (close button). This forces user to scroll through terms. -->
          <button
            use:melt={$close}
            aria-label="close"
            class="dark:bg-dark absolute right-2 top-2 flex size-7 items-center justify-center rounded-full border border-slate-200 bg-white p-1 dark:border-slate-600"
          >
            <XBoldIcon class="size-5" />
          </button>
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
          </div>
        </div>
      </div>
    {/if}

    <button
      data-component="Terms & Conditions"
      use:melt={$trigger}
      class="dark:bg-dark flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600"
    >
      <div class="grow text-left">
        <p class="dark:text-grey text-[13px]/[24px] font-medium text-slate-800">
          {$LL.ONBOARDING.TERMS.T_AND_C.TITLE()}
        </p>
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          {$LL.ONBOARDING.TERMS.T_AND_C.DESCRIPTION()}
        </p>
      </div>
      <div class="pl-4">
        <!-- The state of the checkbox is altered by the modal's buttons. -->
        <Checkbox checked={checked_terms} readonly={true} />
      </div>
    </button>

    <!-- Use button, not div, to enable focus. -->
    <button
      data-component="Data Ownership"
      on:click={() => {
        checked_ownership = !checked_ownership;
      }}
      class="dark:bg-dark flex items-center justify-between rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600"
    >
      <div class="grow text-left">
        <p class="dark:text-grey text-[13px]/[24px] font-medium text-slate-800">
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

<div class="dark:bg-dark rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label={$LL.CONTINUE()}
    on:click={() => goto('/welcome/customize/name')}
    disabled={!checked_terms || !checked_ownership}
  />
</div>
