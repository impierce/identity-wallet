<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { warn } from '@tauri-apps/plugin-log';

  import { Image, TopNavBar } from '$lib/components';
  import { ShieldCheckFillIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { hash } from '$lib/utils';
  import { hostname } from '$lib/utils/url';

  import CredentialClaimsRenderer from '../../../../../../credentials/[id]/CredentialClaimsRenderer.svelte';
  import CertificationOverview from '../../../../../../prompt/accept-connection/certifications/[id]/CertificationOverview.svelte';
  import DomainPill from '../../../../../../prompt/accept-connection/DomainPill.svelte';

  $: connection = $appState.connections.find((item) => item.id === page.params.id);
  $: certification = (connection?.linked_verifiable_presentations ?? []).find(
    (item) => item.credential.id === page.params.certificationId,
  );
  $: validation = certification?.issuer_domain_validations.at(0);
  $: issuer = validation?.name;
  $: domain = validation ? hostname(validation.url) : undefined;
  $: logoUri = certification?.credential.metadata.icon ?? certification?.credential.issuer_logo_uri;
  $: imageId = logoUri ? hash(logoUri) : undefined;
  $: subject = certification?.credential.data?.credentialSubject;
  $: hasClaims = !!subject && typeof subject === 'object';

  let useFallback = false;
  $: showBadge = !imageId || useFallback;

  onMount(() => {
    if (!connection || !certification) {
      warn(`No certification found with id: \`${page.params.certificationId}\``);
      goto(connection ? `/activity/connection/${connection.id}/certifications` : '/activity');
    }
  });
</script>

<div class="content-height flex hide-scrollbar flex-col items-stretch overflow-y-auto bg-background-alt">
  <TopNavBar
    title={$LL.SCAN.CONNECTION_REQUEST.CERTIFICATION()}
    on:back={() => history.back()}
    class="sticky top-0 z-10"
  />

  {#if certification}
    <div class="flex min-h-full flex-col bg-background-alt px-4 pb-7">
      <div class="-mx-4 flex flex-col items-center gap-4 bg-background py-5">
        <div
          class="flex size-[75px] items-center justify-center overflow-hidden rounded-3xl {showBadge
            ? 'bg-primary'
            : 'bg-white p-2 dark:bg-silver'}"
        >
          {#if imageId}
            <Image id={imageId} isTempAsset={false} bind:useFallback imgClass="size-full object-contain">
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
            <div class="flex flex-col items-center gap-1.5 pt-[10px]">
              <p class="text-[13px]/[20px] font-normal text-text-alt">{domain}</p>
              <DomainPill status={validation.status} />
            </div>
          {/if}
        </div>
      </div>

      <div class="mt-4">
        <CertificationOverview credential={certification.credential} isTempAsset={false} />
      </div>

      {#if hasClaims}
        <div class="mt-4">
          <CredentialClaimsRenderer credential={certification.credential} />
        </div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
