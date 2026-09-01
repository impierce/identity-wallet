<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { TopNavBar } from '$lib/components';
  import { resolveAcceptConnectionPrompt } from '$lib/dev/mocks/resolve';
  import { state as appState } from '$lib/stores';
  import { hostname } from '$lib/utils/url';

  import EcosystemCard from '../EcosystemCard.svelte';

  // From the store, not props: the parent page is destroyed when this route mounts.
  // See ../+layout.svelte. No latch needed — this page cannot accept the prompt.
  $: prompt = resolveAcceptConnectionPrompt(page.url, $appState);

  $: ecosystems = prompt?.ecosystems ?? [];

  $: domain = prompt?.redirect_uri ? hostname(prompt.redirect_uri) : undefined;

  // Joined here so the separator only appears with a domain, and so the spacing does not
  // depend on how the formatter wraps an inline `{#if}`.
  $: subtitle = [domain, $LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_COUNT({ count: ecosystems.length })]
    .filter(Boolean)
    .join(' · ');
</script>

<div class="safe-area-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEMS()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  <div class="p-4">
    <p class="px-1 pb-3 text-[13px]/[20px] font-normal text-text-alt">
      {subtitle}
    </p>

    <div class="space-y-2">
      <!-- Keyed by index: it is also the detail route's key, so card and page cannot disagree. -->
      {#each ecosystems as ecosystem, index (index)}
        <EcosystemCard {ecosystem} {index} />
      {/each}
    </div>
  </div>
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
