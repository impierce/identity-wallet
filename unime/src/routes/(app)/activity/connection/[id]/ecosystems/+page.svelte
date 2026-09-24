<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { warn } from '@tauri-apps/plugin-log';

  import { TopNavBar } from '$lib/components';
  import { state as appState } from '$lib/stores';
  import { hostname } from '$lib/utils/url';

  import EcosystemCard from '../../../../../prompt/accept-connection/EcosystemCard.svelte';

  $: connection = $appState.connections.find((item) => item.id === page.params.id);
  $: ecosystems = connection?.ecosystems ?? [];
  $: domain = connection ? (hostname(connection.url) ?? connection.url) : undefined;
  $: subtitle = [domain, $LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_COUNT({ count: ecosystems.length })]
    .filter(Boolean)
    .join(' · ');

  onMount(() => {
    if (!connection) {
      warn(`No connection found with id: \`${page.params.id}\``);
      goto('/activity');
    }
  });
</script>

<div class="content-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEMS()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  {#if connection}
    <div class="p-4">
      <p class="px-1 pb-3 text-[13px]/[20px] font-normal break-all text-text-alt">{subtitle}</p>
      <div class="space-y-2">
        {#each ecosystems as ecosystem, index (index)}
          <EcosystemCard
            {ecosystem}
            {index}
            href={`/activity/connection/${connection.id}/ecosystems/${index}`}
            isTempAsset={false}
          />
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
