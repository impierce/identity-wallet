<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { debug } from '@tauri-apps/plugin-log';

  import { Button, Image, PaddedIcon, TopNavBar } from '$lib/components';
  import { resolveAcceptConnectionPrompt } from '$lib/dev/mocks/resolve';
  import { dispatch } from '$lib/dispatcher';
  import { PlugsConnectedFillIcon, ShieldCheckRegularIcon, WarningCircleFillIcon } from '$lib/icons';
  import { state as appState, error } from '$lib/stores';
  import { formatDate, formatRelativeDateTime, hash } from '$lib/utils';
  import { hostname } from '$lib/utils/url';

  import CertificationCard from './CertificationCard.svelte';
  import CertificationsSummary from './CertificationsSummary.svelte';
  import DomainPill from './DomainPill.svelte';
  import InteractionTiles from './InteractionTiles.svelte';
  import SectionHeader from './SectionHeader.svelte';

  // How many certifications to show before linking to the full list.
  const PREVIEW_COUNT = 3;

  let loading = false;

  // A known connection already shows the "Connected" panel and the interaction tiles, which
  // push Accept below the fold. Fold the certification cards away behind a count until asked;
  // a new connection has the room, so it shows them outright.
  let certificationsExpanded = false;

  // Latch the prompt. After the user accepts, the backend clears `current_user_prompt`
  // and pushes new state; without this, the destructure below would run against `null`
  // before we have navigated away. The page is only ever reached with an active
  // prompt, so the initial value is non-null.
  let prompt = resolveAcceptConnectionPrompt(page.url, $appState)!;
  $: {
    const next = resolveAcceptConnectionPrompt(page.url, $appState);
    if (next) prompt = next;
  }

  $: ({ client_name, logo_uri, redirect_uri, connection_data, domain_validation } = prompt);

  $: certifications = prompt.linked_verifiable_presentations ?? [];

  $: collapsible = !!connection_data;

  $: profile_settings = $appState.profile_settings;
  // `redirect_uri` is optional on the prompt, and the helper swallows a malformed one. A raw
  // `new URL()` here would throw and take the whole page down.
  $: domain = redirect_uri ? hostname(redirect_uri) : undefined;
  $: imageId = logo_uri ? hash(logo_uri) : '_';

  // For DEV previews only: `?mock=` renders a fixture instead of a real prompt.
  $: isMock = $appState.dev_mode !== 'Off' && page.url.searchParams.has('mock');

  onMount(() => {
    if ($appState.dev_mode !== 'Off' && domain_validation.message) {
      debug(`Domain validation (${domain_validation.status}): ${domain_validation.message}`);
    }
  });

  // Release the buttons on error. Cancelling the flow is the layout's job.
  const unsubscribe = error.subscribe((err) => {
    if (err) loading = false;
  });

  onDestroy(unsubscribe);
</script>

<div class="safe-area-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.NAVBAR_TITLE()}
    on:back={() => history.back()}
    disabled={loading}
    class="sticky top-0 z-10"
  />

  <div class="flex grow flex-col items-center space-y-6 p-4">
    {#if logo_uri}
      <div
        class="flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver"
      >
        <Image id={imageId} iconFallback="BankLight" isTempAsset={true} />
      </div>
    {:else}
      <PaddedIcon icon={PlugsConnectedFillIcon} />
    {/if}
    <div class="text-center">
      <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
        {client_name}
      </p>
      <div class="flex flex-wrap items-center justify-center gap-x-2 gap-y-1 pt-[10px]">
        {#if domain}
          <p class="text-[13px]/[20px] font-normal text-slate-500">
            {domain}
          </p>
          <span class="text-[13px]/[20px] text-slate-300 dark:text-slate-600" aria-hidden="true">·</span>
        {/if}
        <DomainPill status={domain_validation.status} />
      </div>
    </div>

    <div class="w-full space-y-3">
      {#if !connection_data}
        <div
          class="flex w-full items-center rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
        >
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

      <!-- Connected previously: how long we have known this party, when was the last interaction -->
      {#if connection_data}
        <div
          class="flex w-full items-center rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
        >
          <span class="mr-4 h-6 w-6 shrink-0">
            <ShieldCheckRegularIcon class="h-6 w-6 text-green-500" />
          </span>
          <div class="flex flex-col">
            <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {$LL.SCAN.CONNECTION_REQUEST.KNOWN_CONNECTION()}
            </p>
            <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.CONNECTION_REQUEST.FIRST_INTERACTION({
                duration: formatRelativeDateTime(connection_data.first_interacted_at, profile_settings.locale),
              })}
            </p>
            <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.CONNECTION_REQUEST.LAST_INTERACTION({
                date: formatDate(connection_data.last_interacted_at, profile_settings.locale),
              })}
            </p>
          </div>
        </div>

        <InteractionTiles interactions={connection_data.interactions} />
      {/if}
    </div>

    <!-- Certifications, sourced from the linked verifiable presentations. -->
    {#if certifications.length > 0}
      <section class="w-full">
        {#if collapsible}
          <!-- Collapsible: the header text toggles between the count and the cards. -->
          <SectionHeader
            title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATIONS()}
            action={certificationsExpanded
              ? $LL.SCAN.CONNECTION_REQUEST.SHOW_LESS()
              : $LL.SCAN.CONNECTION_REQUEST.SHOW_MORE()}
            on:action={() => (certificationsExpanded = !certificationsExpanded)}
          />
        {:else}
          <SectionHeader
            title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATIONS()}
            href={certifications.length > PREVIEW_COUNT
              ? `/prompt/accept-connection/certifications${page.url.search}`
              : undefined}
          />
        {/if}

        {#if collapsible && !certificationsExpanded}
          <CertificationsSummary count={certifications.length} />
        {:else}
          <div class="space-y-2">
            <!-- Expanding shows every certification: with "Show less" occupying the header,
                 there is no link left to reach the full list with. -->
            {#each collapsible ? certifications : certifications.slice(0, PREVIEW_COUNT) as certification}
              <CertificationCard {certification} />
            {/each}
          </div>
        {/if}
      </section>
    {/if}
  </div>

  <div class="sticky bottom-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.CONNECTION_REQUEST.ACCEPT()}
      on:click={() => {
        loading = true;
        if (!isMock) {
          dispatch({
            type: '[Authenticate] Connection accepted',
          });
        }
      }}
      {loading}
    />
    <Button
      label={$LL.REJECT()}
      variant="secondary"
      on:click={() => {
        if (!isMock) dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
        goto('/me');
      }}
      disabled={loading}
    />
  </div>
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
