<script lang="ts">
  import { onDestroy } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
  import { createPopover, melt } from '@melt-ui/svelte';

  import { Button, Image, PaddedIcon, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    CheckBoldIcon,
    PlugsConnectedFillIcon,
    QuestionMarkBoldIcon,
    WarningCircleFillIcon,
    XRegularIcon,
  } from '$lib/icons';
  import { error, state } from '$lib/stores';
  import { hash } from '$lib/utils';

  const {
    elements: { trigger, content, arrow },
    states: { open },
  } = createPopover();

  let loading = false;

  // TypeScript does not know that the `current_user_prompt` is of type `accept-connection`.
  // Extract the type from `CurrentUserPrompt`.
  type IsAcceptConnectionPrompt<T> = T extends { type: 'accept-connection' } ? T : never;
  type AcceptConnectionPrompt = IsAcceptConnectionPrompt<CurrentUserPrompt>;

  const { client_name, domain_validation, logo_uri, previously_connected, redirect_uri } =
    $state.current_user_prompt as AcceptConnectionPrompt;

  $: ({ hostname } = new URL(redirect_uri));
  $: imageId = logo_uri ? hash(logo_uri) : '_';

  // When an error is received, cancel the flow and redirect to the "me" page
  error.subscribe((err) => {
    if (err) {
      loading = false;
      dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
    }
  });

  onDestroy(async () => {
    // TODO: is onDestroy also called when user accepts since the component itself is destroyed?
    dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={$LL.SCAN.CONNECTION_REQUEST.NAVBAR_TITLE()} on:back={() => history.back()} disabled={loading} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    {#if logo_uri}
      <div
        class="flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver"
      >
        <Image id={imageId} iconFallback="Bank" isTempAsset={true} />
      </div>
    {:else}
      <PaddedIcon icon={PlugsConnectedFillIcon} />
    {/if}
    <div class="text-center">
      <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
        {client_name}
      </p>
      <p class="pt-[10px] text-sm font-medium text-slate-500">
        {hostname}
      </p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-3 rounded-3xl bg-white p-3 dark:bg-dark">
      <!-- Warning -->
      {#if !previously_connected}
        <div class="flex w-full items-center rounded-xl bg-silver p-4 dark:bg-navy">
          <span class="mr-4 h-6 w-6">
            <WarningCircleFillIcon class="h-6 w-6 text-amber-500" />
          </span>
          <div class="flex flex-col">
            <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {$LL.SCAN.CONNECTION_REQUEST.TITLE()}
            </p>
            <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.CONNECTION_REQUEST.DESCRIPTION()}
            </p>
          </div>
        </div>
      {/if}
      <!-- Connected previously -->
      <div
        class="flex items-center justify-between rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
      >
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {$LL.SCAN.CONNECTION_REQUEST.CONNECTED_PREVIOUSLY()}
        </p>
        {#if previously_connected}
          <CheckBoldIcon class="text-emerald-500" />
        {:else}
          <XRegularIcon class="text-rose-500" />
        {/if}
      </div>
      <!-- Domain validation -->
      <div
        use:melt={$trigger}
        class="flex items-center justify-between rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
      >
        <div class="flex items-center">
          <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{$LL.DOMAIN_LINKAGE.TITLE()}</p>
          {#if $open}
            <div
              use:melt={$content}
              transition:fade={{ duration: 200 }}
              class="z-10 w-1/2 rounded-2xl bg-blue p-[20px] text-white shadow-md dark:border dark:border-slate-500"
            >
              <div use:melt={$arrow} class="dark:border-l dark:border-t dark:border-slate-500" />
              <div class="break-words text-[12px]/[20px]">
                {#if domain_validation.status === 'Success'}
                  <!-- TODO: add a better description of _what_ was validated -->
                  <p>{$LL.DOMAIN_LINKAGE.SUCCESS()}</p>
                {:else if domain_validation.status === 'Failure'}
                  <p>{$LL.DOMAIN_LINKAGE.FAILURE()}</p>
                  <!-- TODO: pick a different color (bad contrast) -->
                  <p class="font-semibold text-rose-500">{$LL.DOMAIN_LINKAGE.CAUTION()}</p>
                {:else}
                  <p>{$LL.DOMAIN_LINKAGE.UNKNOWN()}</p>
                  <!-- TODO: pick a different color (bad contrast) -->
                  <p class="font-semibold text-rose-500">{$LL.DOMAIN_LINKAGE.CAUTION()}</p>
                {/if}
                <!-- Dev Mode: Show additional message -->
                {#if $state.dev_mode !== 'Off' && domain_validation.message}
                  <!-- TODO: dev_mode only: pick a different color (bad contrast) -->
                  <p class="text-rose-500">{domain_validation.message}</p>
                {/if}
              </div>
            </div>
          {/if}
        </div>
        {#if domain_validation.status === 'Success'}
          <CheckBoldIcon class="text-emerald-500" />
        {:else if domain_validation.status === 'Failure'}
          <XRegularIcon class="text-rose-500" />
        {:else}
          <QuestionMarkBoldIcon class="text-slate-400 dark:text-slate-300" />
        {/if}
      </div>
    </div>
  </div>

  <!-- Controls -->
  <!-- TODO: on iOS subtract the --safe-area-inset-bottom from the bottom-padding -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.CONNECTION_REQUEST.ACCEPT()}
      on:click={() => {
        loading = true;
        dispatch({
          type: '[Authenticate] Connection accepted',
        });
      }}
      {loading}
    />
    <Button
      label={$LL.REJECT()}
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
        goto('/me');
      }}
      disabled={loading}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
