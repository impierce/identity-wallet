<script lang="ts">
  import { onMount } from 'svelte';

  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { warn } from '@tauri-apps/plugin-log';

  import { Image, TopNavBar } from '$lib/components';
  import { resolveAcceptConnectionPrompt } from '$lib/dev/mocks/resolve';
  import { ShieldCheckFillIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { hash } from '$lib/utils';
  import { hostname } from '$lib/utils/url';

  import DefaultRenderer from '../../../../credentials/[id]/DefaultRenderer.svelte';
  import DomainPill from '../../DomainPill.svelte';
  import CertificationOverview from './CertificationOverview.svelte';

  // Read from the store rather than taking props, as the sibling list page does.
  // No latch needed — this page cannot accept the prompt, so it never sees the backend
  // clear it out from under itself.
  // `?? []` as well as `?.`: the optional chain covers a missing prompt, not a prompt that
  // arrives without the field while the data model is still in flight.
  $: certification = (resolveAcceptConnectionPrompt(page.url, $appState)?.linked_verifiable_presentations ?? []).find(
    (c) => c.credential.id === page.params.id,
  );

  // See `CertificationCard`: the first result stands in for all linked domains.
  $: validation = certification?.issuer_domain_validations.at(0);
  $: issuer = validation?.name;
  $: domain = validation ? hostname(validation.url) : undefined;
  // See `CertificationCard`: re-hash the certification logo URL to find what the backend downloaded.
  $: logoUri = certification?.credential.metadata.icon ?? certification?.credential.issuer_logo_uri;
  $: imageId = logoUri ? hash(logoUri) : undefined;

  // A tinted badge when there is no logo (or it
  // fails to load), a plain backdrop for a real one.
  let useFallback = false;
  $: showBadge = !imageId || useFallback;

  // `DefaultRenderer` dereferences `data.credentialSubject` unguarded, so a certification
  // that arrives without one would take the whole prompt screen down with it.
  $: subject = certification?.credential.data?.credentialSubject;
  $: hasClaims = !!subject && typeof subject === 'object';

  onMount(() => {
    if (!certification) {
      warn(`No certification found with id: \`${page.params.id}\``);
      // Stay inside the prompt subtree: leaving it cancels the flow. See ../../+layout.svelte.
      history.back();
    }
  });
</script>

<div class="safe-area-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-background-alt">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATION()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  {#if certification}
    <div class="flex min-h-full flex-col bg-background-alt px-4 pb-7">
      <div class="-mx-4 flex flex-col items-center gap-4 bg-background py-5">
        <div
          class="flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl {showBadge
            ? 'bg-primary'
            : 'bg-white p-2 dark:bg-silver'}"
        >
          {#if imageId}
            <Image id={imageId} isTempAsset={true} bind:useFallback imgClass="size-full object-contain">
              <ShieldCheckFillIcon slot="fallback" class="size-7 text-background-alt" />
            </Image>
          {:else}
            <ShieldCheckFillIcon class="size-7 text-background-alt" />
          {/if}
        </div>

        <div class="text-center">
          <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
            {certification.credential.display_name}
          </p>
          {#if issuer}
            <p class="pt-[10px] text-[13px]/[20px] font-normal text-text-alt">
              {$LL.CREDENTIAL.DETAILS.ISSUED_BY()}
              {issuer}
            </p>
          {/if}
          {#if validation && domain}
            <div class="flex flex-wrap items-center justify-center gap-x-2 gap-y-1 pt-[10px]">
              <p class="text-[13px]/[20px] font-normal text-text-alt">{domain}</p>
              <span class="text-[13px]/[20px] text-slate-300 dark:text-slate-500" aria-hidden="true">·</span>
              <DomainPill status={validation.status} />
            </div>
          {/if}
        </div>
      </div>

      <div class="mt-4">
        <CertificationOverview credential={certification.credential} />
      </div>

      {#if hasClaims}
        <div class="mt-4">
          <DefaultRenderer credential={certification.credential} />
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
