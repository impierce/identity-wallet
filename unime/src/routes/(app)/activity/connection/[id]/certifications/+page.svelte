<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { warn } from '@tauri-apps/plugin-log';

  import { TopNavBar } from '$lib/components';
  import { state as appState } from '$lib/stores';
  import { hostname } from '$lib/utils/url';

  import CertificationCard from '../../../../../prompt/accept-connection/CertificationCard.svelte';

  $: connection = $appState.connections.find((item) => item.id === page.params.id);
  $: certifications = connection?.linked_verifiable_presentations ?? [];
  $: domain = connection ? (hostname(connection.url) ?? connection.url) : undefined;

  onMount(() => {
    if (!connection) {
      warn(`No connection found with id: \`${page.params.id}\``);
      goto('/activity');
    }
  });
</script>

<div class="content-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATIONS()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  {#if connection}
    <div class="p-4">
      {#if domain}
        <p class="px-1 pb-3 text-[13px]/[20px] font-normal break-all text-text-alt">{domain}</p>
      {/if}
      <div class="space-y-2">
        {#each certifications as certification (certification.credential.id)}
          <CertificationCard
            {certification}
            href={`/activity/connection/${connection.id}/certifications/${certification.credential.id}`}
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
