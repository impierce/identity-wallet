<script lang="ts">
  import { page } from '$app/state';

  import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';

  import { ShieldCheckRegularIcon, WarningRegularIcon } from '$lib/icons';
  import { hostname } from '$lib/utils/url';

  import CertificationAvatar from './CertificationAvatar.svelte';

  export let certification: LinkedVerifiableCredentialData;

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/certifications/${certification.credential.id}${page.url.search}`;

  // Showing the first domain.
  $: validation = certification.issuer_domain_validations.at(0);

  // The issuing body, e.g. "Intl. Organization for Standardization".
  $: issuer = validation?.name;

  $: domain = validation ? hostname(validation.url) : undefined;

  $: verified = validation?.status === 'Success';
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
  <CertificationAvatar {certification} class="mr-4 size-12 rounded-lg" />

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
