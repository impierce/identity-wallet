<script lang="ts">
  import { onDestroy } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';

  import { Button, Image, PaddedIcon, StatusIndicator, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { error, state } from '$lib/stores';
  import { formatDate, hash } from '$lib/utils';

  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import WarningCircle from '~icons/ph/warning-circle-fill';

  let loading = false;

  // TypeScript does not know that the `current_user_prompt` is of type `accept-connection`.
  // Extract the type rather than repeating the type definition.
  type IsAcceptConnectionPrompt<T> = T extends { type: 'accept-connection' } ? T : never;
  type AcceptConnectionPrompt = IsAcceptConnectionPrompt<CurrentUserPrompt>;

  // Use reactive statement to coerce the type only once.
  const { client_name, domain_validation, logo_uri, previously_connected, redirect_uri, thuiswinkel_validation } =
    $state.current_user_prompt as AcceptConnectionPrompt;

  const profile_settings = $state.profile_settings;

  $: ({ hostname } = new URL(redirect_uri));

  const imageId = logo_uri ? hash(logo_uri) : '_';

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
      <PaddedIcon icon={PlugsConnected} />
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
            <WarningCircle class="h-6 w-6 text-amber-500" />
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
      <StatusIndicator
        status={previously_connected ? 'Success' : 'Failure'}
        title={$LL.SCAN.CONNECTION_REQUEST.CONNECTED_PREVIOUSLY()}
      />

      <!-- Domain validation -->
      <StatusIndicator status={domain_validation.status} title={$LL.DOMAIN_LINKAGE.TITLE()}>
        <div class="break-words text-[12px]/[20px]" slot="popover">
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
      </StatusIndicator>

      <!-- Thuiswinkel validaton -->
      {#if thuiswinkel_validation.status === 'Success' && thuiswinkel_validation.name}
        {@const issuanceDate =
          thuiswinkel_validation.issuance_date && profile_settings.locale
            ? formatDate(thuiswinkel_validation.issuance_date, profile_settings.locale)
            : undefined}
        <StatusIndicator
          status="Success"
          title={thuiswinkel_validation.name}
          description={`${$LL.SORT.PREFERENCES.DATE_ISSUED()}: ${issuanceDate}`}
          logoUrl={thuiswinkel_validation.logo_uri}
        />
      {/if}
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
