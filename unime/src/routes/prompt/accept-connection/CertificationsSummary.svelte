<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';

  import CertificationAvatar from './CertificationAvatar.svelte';

  export let certifications: LinkedVerifiableCredentialData[];

  // Past three the overlap stops reading as distinct marks. Matches `EcosystemsSummary`.
  const STACK_COUNT = 3;

  // Two names is the most that stays inside two lines at this width.
  const NAME_COUNT = 2;

  $: stack = certifications.slice(0, STACK_COUNT);

  // The issuing body, read from the first domain validation exactly as `CertificationCard`
  // does. `name` is optional there, so an unattributed certification falls back to the
  // credential's own title rather than contributing an empty entry to the list.
  $: names = certifications
    .slice(0, NAME_COUNT)
    .map(
      (certification) => certification.issuer_domain_validations.at(0)?.name ?? certification.credential.display_name,
    )
    .join(', ');

  $: remaining = certifications.length - Math.min(certifications.length, NAME_COUNT);

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/certifications${page.url.search}`;
</script>

<!--
@component
Collapsed stand-in for the certification cards, shown on a known connection. The whole card is
the tap target — "See all" is a label, not a separate link.

### Props
- certifications
-->
<a
  {href}
  class="flex w-full flex-col rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
>
  <div class="flex items-center justify-between">
    <p class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
      {$LL.SCAN.CONNECTION_REQUEST.CERTIFICATIONS()}
    </p>
    <span class="text-[13px]/[24px] font-normal text-primary">
      {$LL.SCAN.CONNECTION_REQUEST.SEE_ALL()}
    </span>
  </div>

  <div class="flex items-center gap-3 pt-3">
    <div class="flex shrink-0 items-center -space-x-2">
      <!-- Keyed by credential id: unlike an ecosystem, a certification has one. -->
      {#each stack as certification (certification.credential.id)}
        <CertificationAvatar
          {certification}
          class="size-8 rounded-full ring-2 ring-white dark:ring-dark"
          textClass="text-[11px]/[16px]"
        />
      {/each}
    </div>

    <p class="line-clamp-2 text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      {remaining > 0 ? $LL.SCAN.CONNECTION_REQUEST.AND_MORE({ names, count: remaining }) : names}
    </p>
  </div>
</a>
