<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { Connection } from '@bindings/connections/Connection';
  import { open as openExternal } from '@tauri-apps/plugin-shell';

  import { Image } from '$lib/components';
  import { ArrowSquareOutBoldIcon, ShieldCheckRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatRelativeDateTime } from '$lib/utils';
  import { countInteractions } from '$lib/utils/history';
  import { connectionBrowserUrl, connectionHostname } from '$lib/utils/url';

  import CertificationsSummary from '../../../../prompt/accept-connection/CertificationsSummary.svelte';
  import DomainPill from '../../../../prompt/accept-connection/DomainPill.svelte';
  import EcosystemsSummary from '../../../../prompt/accept-connection/EcosystemsSummary.svelte';

  export let connection: Connection;

  $: domain = connectionHostname(connection.url);
  $: browserUrl = connectionBrowserUrl(connection.url);
  $: certifications = connection.linked_verifiable_presentations ?? [];
  $: ecosystems = connection.ecosystems ?? [];
  $: interactions = $state.history.filter((event) => event.connection_id === connection.id);
  $: interactionCount = countInteractions(interactions).total;

  async function openConnectionUrl() {
    if (browserUrl) await openExternal(browserUrl);
  }
</script>

<div class="flex flex-col items-center space-y-6">
  <div class="flex w-full flex-col items-center">
    <div class="flex size-[75px] items-center justify-center overflow-hidden rounded-3xl bg-white p-2">
      <Image
        id={connection.id}
        imgClass="size-full rounded-2xl"
        iconFallback="BankLight"
        iconClass="size-6 dark:text-slate-800"
      />
    </div>
    <p class="pt-4 text-center text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
      {connection.name}
    </p>
    {#if domain}
      {#if browserUrl}
        <button
          type="button"
          class="flex items-center gap-1 pt-[10px] text-center text-[13px]/[20px] font-normal break-all text-secondary"
          on:click={openConnectionUrl}
        >
          {domain}
          <ArrowSquareOutBoldIcon class="size-4 shrink-0" />
        </button>
      {:else}
        <p class="pt-[10px] text-center text-[13px]/[20px] font-normal break-all text-text-alt">
          {domain}
        </p>
      {/if}
    {/if}
    {#if connection.domain_validation}
      <div class="flex justify-center pt-[6px]">
        <DomainPill status={connection.domain_validation[0]} />
      </div>
    {/if}
  </div>

  <div
    class="flex w-full items-center rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
  >
    <span class="mr-4 size-6 shrink-0">
      <ShieldCheckRegularIcon class="size-6 text-green-500" />
    </span>
    <div class="flex min-w-0 grow flex-col">
      <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
        {$LL.SCAN.CONNECTION_REQUEST.KNOWN_CONNECTION()}
      </p>
      <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.CONNECTION.SUMMARY.FIRST_CONNECTED_TIME({
          duration: formatRelativeDateTime(connection.first_interacted, $state.profile_settings.locale, {
            capitalize: false,
          }),
        })}
      </p>
      <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.CONNECTION.SUMMARY.LAST_CONNECTED_TIME({
          duration: formatRelativeDateTime(connection.last_interacted, $state.profile_settings.locale, {
            capitalize: false,
          }),
        })}
      </p>
    </div>
    <div class="ml-4 flex shrink-0 flex-col items-center">
      <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SCAN.CONNECTION_REQUEST.INTERACTIONS()}
      </p>
      <p class="text-[22px]/[30px] font-semibold text-slate-800 dark:text-grey">
        {interactionCount}
      </p>
    </div>
  </div>

  {#if certifications.length > 0}
    <section class="w-full">
      <CertificationsSummary
        {certifications}
        href={`/activity/connection/${connection.id}/certifications`}
        isTempAsset={false}
      />
    </section>
  {/if}

  {#if ecosystems.length > 0}
    <section class="w-full">
      <EcosystemsSummary {ecosystems} href={`/activity/connection/${connection.id}/ecosystems`} isTempAsset={false} />
    </section>
  {/if}
</div>
