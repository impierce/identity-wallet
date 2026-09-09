<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { TopNavBar } from '$lib/components';
  import { resolveAcceptConnectionPrompt } from '$lib/dev/mocks/resolve';
  import { state as appState } from '$lib/stores';

  import CertificationCard from '../CertificationCard.svelte';

  // Read from the store rather than taking props: the parent page is destroyed when
  // this route mounts, so there is nothing to pass down. See ../+layout.svelte.
  // No latch needed here — this page cannot accept the prompt, so it never sees the
  // backend clear it out from under itself.
  $: certifications = resolveAcceptConnectionPrompt(page.url, $appState)?.linked_verifiable_presentations ?? [];
</script>

<div class="safe-area-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATIONS()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  <div class="space-y-2 p-4">
    {#each certifications as certification}
      <CertificationCard {certification} />
    {/each}
  </div>
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
