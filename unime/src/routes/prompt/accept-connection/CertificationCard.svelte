<script lang="ts">
  import { page } from '$app/state';

  import { Image } from '$lib/components';
  import type { Certification } from '$lib/dev/accept-connection.types';
  import { ShieldCheckFillIcon, ShieldCheckRegularIcon, WarningRegularIcon } from '$lib/icons';
  import { hostname } from '$lib/utils/url';

  import { certificationLogoId } from './logo.js';

  export let certification: Certification;

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/certifications/${certification.credential.id}${page.url.search}`;

  $: imageId = certificationLogoId(certification);

  // The design shows a single domain; an issuer may link several, each with its own result.
  // Showing the first is deliberate.
  $: validation = certification.issuer_domain_validations.at(0);

  // The issuing body, e.g. "Intl. Organization for Standardization".
  $: issuer = validation?.name;

  $: domain = validation ? hostname(validation.url) : undefined;

  $: verified = validation?.status === 'Success';

  // <Image> reports whether it fell back to an icon, so the tile can switch between a
  // tinted badge and a plain backdrop for a real logo.
  let useFallback = false;

  // Without a logo there is nothing on disk to find, so skip <Image> entirely rather
  // than have it probe for a missing asset on every card.
  $: showBadge = !imageId || useFallback;
</script>

<!--
@component
A single certification from the prompt's linked verifiable presentations: what it is,
who issued it, and whether that issuer's domain checked out. Links to the detail page.

### Props
- certification
-->
<a
  {href}
  class="flex w-full items-center rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-600 dark:bg-dark"
>
  <div
    class="mr-4 flex size-12 shrink-0 items-center justify-center overflow-hidden rounded-lg {showBadge
      ? 'bg-primary'
      : 'bg-white'}"
  >
    {#if imageId}
      <Image id={imageId} isTempAsset={true} bind:useFallback imgClass="size-full object-contain">
        <ShieldCheckFillIcon slot="fallback" class="size-6 text-white" />
      </Image>
    {:else}
      <ShieldCheckFillIcon class="size-6 text-white" />
    {/if}
  </div>

  <div class="flex min-w-0 grow flex-col">
    <p class="truncate text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
      {certification.credential.display_name}
    </p>
    {#if issuer}
      <p class="truncate text-[12px]/[20px] font-normal text-slate-500 dark:text-slate-300">
        {issuer}
      </p>
    {/if}
    {#if domain}
      <div class="flex items-center gap-1">
        {#if verified}
          <ShieldCheckRegularIcon class="size-4 shrink-0 text-green-500" />
        {:else}
          <WarningRegularIcon class="size-4 shrink-0 text-amber-500" />
        {/if}
        <p class="truncate text-[12px]/[20px] font-normal text-primary">{domain}</p>
      </div>
    {/if}
  </div>
</a>
