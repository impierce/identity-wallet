<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { CredentialValidityLine, ListItemImage } from '$lib/components';
  import { state } from '$lib/stores';
  import { formatDateWithWrittenMonth } from '$lib/utils';

  import { isBadge, typeLabel } from './utils';

  export let requested: RequestedCredential;
  // All options of the request, in the order they are shown in the picker.
  export let candidates: DisplayCredential[];
  export let selected: DisplayCredential[];

  const dispatch = createEventDispatcher<{ change: void }>();

  const claimText = (value: unknown) => (typeof value === 'string' || typeof value === 'number' ? String(value) : '');

  // The first claim whose value differs between the options, which tells otherwise identical credentials apart.
  function distinguishingClaim(credential: DisplayCredential): string | undefined {
    const others = candidates.filter((c) => c.id !== credential.id);
    if (others.length === 0) return undefined;
    const claim = credential.display_claims.find((claim) => {
      const value = claimText(claim.value);
      const samePath = (c: DisplayCredential) =>
        c.display_claims.find((other) => other.path.join('/') === claim.path.join('/'));
      return value && value !== credential.display_name && others.some((c) => claimText(samePath(c)?.value) !== value);
    });
    return claim ? claimText(claim.value) : undefined;
  }

  $: first = selected[0];
  $: detail = first && selected.length === 1 ? distinguishingClaim(first) : undefined;
  $: expired = first && requested.options.some((option) => option.id === first.id && option.state === 'expired');
</script>

<!--
@component
A requested credential type on the share screen: the chosen credential, and a "Change" button when there are more
options.
-->
<div class="flex flex-col rounded-xl border border-grey bg-white dark:border-slate-600 dark:bg-dark">
  <div class="flex items-center gap-3 p-3">
    <!-- The image loads its logo once, so remount it when another credential is chosen. -->
    {#key first?.id}
      <ListItemImage
        id={first?.id ?? requested.query_id}
        type={first && isBadge(first) ? 'badge' : 'data'}
        icon={first?.metadata.icon}
      />
    {/key}
    <div class="flex min-w-0 grow flex-col gap-0.5">
      <p class="truncate text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
        {first?.display_name ?? typeLabel(requested)}
      </p>
      {#if selected.length > 1}
        <p class="text-[12px]/[20px] font-medium text-slate-700 dark:text-slate-300">
          {$LL.SCAN.SHARE_CREDENTIALS.SELECTED_COUNT({ count: selected.length })}
        </p>
      {:else if detail}
        <p class="truncate text-[12px]/[20px] font-medium text-slate-700 dark:text-slate-300">{detail}</p>
      {/if}
      {#if first}
        <p class="truncate text-[12px]/[20px] font-medium text-slate-400">
          {first.issuer_name}{first.metadata.date_issued && selected.length === 1
            ? ` · ${$LL.SCAN.SHARE_CREDENTIALS.ISSUED({
                date: formatDateWithWrittenMonth(first.metadata.date_issued, $state.profile_settings.locale),
              })}`
            : ''}
        </p>
        {#if expired}
          <CredentialValidityLine credential={first} />
        {/if}
      {/if}
    </div>
  </div>
  {#if candidates.length > 1 || selected.length === 0}
    <div class="flex items-center gap-2 border-t border-grey py-2 pr-[10px] pl-[14px] dark:border-slate-600">
      <p class="grow text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SCAN.SHARE_CREDENTIALS.OPTIONS_AVAILABLE({ count: candidates.length })}
      </p>
      <button
        class="h-9 rounded-xl border border-slate-200 bg-white px-4 text-[13px]/[24px] font-medium text-slate-800 dark:border-slate-600 dark:bg-dark dark:text-grey"
        on:click={() => dispatch('change')}
      >
        {$LL.SCAN.SHARE_CREDENTIALS.CHANGE()}
      </button>
    </div>
  {/if}
</div>
