<script lang="ts">
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

  import EcosystemAvatar from './EcosystemAvatar.svelte';

  export let ecosystem: EcosystemProfile;
  // Position in the prompt's `ecosystems`, which is the detail route's key: `EcosystemProfile`
  // has no id, unlike a certification's `credential.id`. Safe because the array is fixed for
  // the life of a prompt and the route is not deep-linkable.
  export let index: number;

  // Carry `?mock=` across so DEV previews survive the navigation.
  $: href = `/prompt/accept-connection/ecosystems/${index}${page.url.search}`;
</script>

<!--
@component
A single ecosystem the connection belongs to. Links to the detail page.

### Props
- ecosystem
- index
-->
<a
  {href}
  class="flex w-full items-center rounded-xl border border-slate-200 bg-white p-3 dark:border-slate-600 dark:bg-dark"
>
  <EcosystemAvatar name={ecosystem.name} logoUri={ecosystem.logo_uri} class="mr-4 size-12 rounded-lg" />

  <div class="flex min-w-0 grow flex-col">
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
      {ecosystem.name}
    </p>
    {#if ecosystem.description}
      <p class="line-clamp-2 text-[12px]/[20px] font-normal text-slate-500 dark:text-slate-300">
        {ecosystem.description}
      </p>
    {/if}
  </div>

  <span
    class="ml-3 shrink-0 self-center rounded-md bg-slate-100 px-2 py-1 text-[11px]/[16px] font-medium whitespace-nowrap text-slate-600 dark:bg-slate-700 dark:text-slate-200"
  >
    {$LL.SCAN.CONNECTION_REQUEST.ECOSYSTEM_MEMBERS({ count: ecosystem.member_count })}
  </span>
</a>
