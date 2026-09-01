<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

  import EcosystemAvatar from './EcosystemAvatar.svelte';

  export let ecosystems: EcosystemProfile[];

  // Past three the overlap stops reading as distinct marks. Matches `CertificationsSummary`.
  const STACK_COUNT = 3;

  // Two names is the most that stays inside two lines at this width.
  const NAME_COUNT = 2;

  $: stack = ecosystems.slice(0, STACK_COUNT);

  $: names = ecosystems
    .slice(0, NAME_COUNT)
    .map((ecosystem) => ecosystem.name)
    .join(', ');

  $: remaining = ecosystems.length - Math.min(ecosystems.length, NAME_COUNT);

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/ecosystems${page.url.search}`;
</script>

<!--
@component
Collapsed stand-in for the ecosystem cards, shown on a known connection. The whole card is the
tap target — "See all" is a label, not a separate link.

### Props
- ecosystems
-->
<a
  {href}
  class="flex w-full flex-col rounded-xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
>
  <div class="flex items-center justify-between">
    <p class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
      {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEMS()}
    </p>
    <span class="text-[13px]/[24px] font-normal text-primary">
      {$LL.SCAN.CONNECTION_REQUEST.SEE_ALL()}
    </span>
  </div>

  <div class="flex items-center gap-3 pt-3">
    <div class="flex shrink-0 items-center -space-x-2">
      <!-- Keyed by name: no id, and the badge derives from the name anyway. -->
      {#each stack as ecosystem (ecosystem.name)}
        <EcosystemAvatar
          name={ecosystem.name}
          logoUri={ecosystem.logo_uri}
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
