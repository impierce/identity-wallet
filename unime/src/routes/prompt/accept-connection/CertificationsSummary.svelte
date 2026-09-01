<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';

  import CertificationAvatar from './CertificationAvatar.svelte';

  export let certifications: LinkedVerifiableCredentialData[];

  // How many badges the stack shows. Beyond three the overlap stops reading as distinct
  // marks and the label carries the rest.
  const STACK_COUNT = 3;

  // How many certifications the label names before falling back to "and N more". Two names
  // is the most that stays inside two lines at this width.
  const NAME_COUNT = 2;

  $: stack = certifications.slice(0, STACK_COUNT);

  $: names = certifications
    .slice(0, NAME_COUNT)
    .map((certification) => certification.credential.display_name)
    .join(', ');

  $: remaining = certifications.length - Math.min(certifications.length, NAME_COUNT);

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/certifications${page.url.search}`;
</script>

<!--
@component
Collapsed stand-in for the certification cards, shown on a known connection: a stack of
issuer logos and the count, linking to the full list. The whole card is the tap target —
"See all" is a label on it, not a separate one.

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
