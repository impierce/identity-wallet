<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fade, scale } from 'svelte/transition';

  import { createDialog, melt } from '@melt-ui/svelte';

  import { Button, Toast, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { HeartFillIcon, XBoldIcon } from '$lib/icons';
  import UniMeLogo from '$lib/static/svg/logo/UniMeLogo.svelte';
  import { state as appState } from '$lib/stores';

  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  // In order to show the developer mode in the app settings,
  // the user has to tap the UniMe logo 7 times in a row.
  const REQUIRED_CLICKS = 7;

  let counter = $state(0);

  function handleClick() {
    counter++;
    if (counter === REQUIRED_CLICKS) {
      dispatch({ type: '[DEV] Show DEV mode setting', payload: { show: !$appState.show_dev_mode_setting } });
      counter = 0;
    }
  }

  let title = $appState.show_dev_mode_setting ? 'Hide Developer Mode' : 'Unlock Developer Mode';
  let detail = $derived(
    `Tap the logo ${REQUIRED_CLICKS - counter} more time${REQUIRED_CLICKS - counter > 1 ? 's' : ''} to ${$appState.show_dev_mode_setting ? 'hide' : 'unlock'}.`,
  );

  let showMessage = $derived(() => {
    if (counter >= 5 && counter < REQUIRED_CLICKS) {
      return true;
    } else {
      return false;
    }
  });

  const {
    elements: { trigger, overlay, content, description, close, portalled },
    states: { open },
  } = createDialog({
    // Portal to root layout inside the `div` that sets the `dark` class. This ensures that dark mode works.
    portal: '#portal',
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.SUPPORT.ABOUT.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <h1 class="sr-only">{$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}</h1>
  <div class="flex w-full scale-75 justify-center">
    <button onclick={handleClick}>
      <UniMeLogo class="text-blue dark:text-silver" />
    </button>
  </div>
  <div
    class="flex flex-col items-center gap-6 pt-4 text-[13px]/[24px] font-normal text-slate-500 opacity-50 dark:text-slate-300"
  >
    {#if $appState.dev_mode !== 'Off'}
      <section class="flex flex-col items-center">
        <h2 class="mb-3 font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.SPECIFICATIONS()}</h2>
        <dl class="flex flex-col items-center gap-3">
          {#each data.specifications as spec (spec.id)}
            <div class="flex flex-col items-center">
              <dt class="font-semibold">{`${spec.description} (${spec.id}):`}</dt>
              <!-- `target="_blank"` opens the spec link in the default browser. -->
              <dd><a href={spec.url} target="_blank" class="underline">{spec.version}</a></dd>
            </div>
          {/each}
        </dl>
      </section>
    {/if}
    <section class="flex flex-col items-center">
      <h2 class="font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.VERSION()}</h2>
      <div class="mb-3">0.10.0</div>
      <div class="flex items-center">
        <p>{$LL.SETTINGS.SUPPORT.ABOUT.BUILT_WITH()}</p>
        <HeartFillIcon class="pl-1" />
      </div>
    </section>

    <section class="flex flex-col items-center">
      <button
        data-component="Terms & Conditions"
        use:melt={$trigger}
        class="font-bold text-slate-500 underline dark:text-slate-300"
      >
        {$LL.ONBOARDING.TERMS.T_AND_C.TITLE()}
      </button>
    </section>

    <section class="mb-4 flex flex-col items-center">
      <h2 class="font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.LICENSE()}</h2>
      <div class="mb-3">Apache License 2.0</div>
      <div>{`${new Date().getFullYear()} Impierce Technologies B.V.`}</div>
    </section>
  </div>
</div>

{#if $open}
  <div use:melt={$portalled}>
    <div use:melt={$overlay} class="fixed inset-0 z-50 bg-black/50" transition:fade={{ duration: 150 }}></div>
    <div
      data-component="Modal"
      use:melt={$content}
      class="fixed left-1/2 top-1/2 z-50 max-h-[85vh] w-[90vw]
            -translate-x-1/2 -translate-y-1/2 overflow-y-auto rounded-xl
            bg-background p-6 shadow-lg"
      transition:scale={{ duration: 150, start: 0.96 }}
    >
      <h2 class="mb-4 text-lg font-bold">
        {$LL.ONBOARDING.TERMS.T_AND_C.DIALOG_TITLE()}
      </h2>

      <p class="mb-2 text-xs font-medium text-slate-500 dark:text-slate-300">
        {$LL.ONBOARDING.TERMS.T_AND_C.LAST_UPDATED()}
      </p>
      <p use:melt={$description} class="rounded-lg text-xs font-medium">
        {$LL.ONBOARDING.TERMS.T_AND_C.TL_DR()}
      </p>
      <!-- acts as <hr> -->
      <div class="my-2 h-px w-full bg-brand"></div>

      <!-- The modal content-->
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

      <div class="mt-6 flex justify-end gap-4">
        <Button variant="secondary" trigger={$close} label={$LL.CLOSE()} />
      </div>
      <button
        use:melt={$close}
        aria-label="close"
        class="absolute right-2 top-2 flex size-7 items-center justify-center rounded-full border border-slate-200 bg-white p-1 dark:border-slate-600 dark:bg-dark"
      >
        <XBoldIcon class="size-5" />
      </button>
    </div>
  </div>
{/if}

{#if showMessage()}
  <div class="absolute bottom-[calc(64px_+_16px_+_var(--safe-area-inset-bottom))] left-4 w-[calc(100%_-_32px)]">
    <Toast variant="info" {title} {detail} dismissible={false} />
  </div>
{/if}
